<script setup lang="ts">
import { computed, nextTick, onMounted, onUnmounted, ref, watch } from 'vue';
import {
  cursorPosition,
  currentMonitor,
  getCurrentWindow,
  monitorFromPoint,
  PhysicalPosition,
  PhysicalSize,
} from '@tauri-apps/api/window';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import { marked } from 'marked';
import {
  ANIMATIONS,
  WINDOW_BEHAVIOR_ANIMATIONS,
  WINDOW_SIZE,
  type AnimState,
} from '../config/animations';
import { showContextMenu, onSwitchAnim } from '../menu/contextMenu';
import { getFileIcon } from '../config/fileIcons';
import {
  computeSnapSide,
  EDGE_PEEK_LEAVE_DELAY,
  getBehaviorTargetX,
  getCompactWindowMetrics,
  getEdgeSnapThreshold,
  getHiddenRevealPx,
  getPeekRevealPx,
  isCompactBehavior,
  isHiddenBehavior,
  isPeekBehavior,
  type WindowBehaviorState,
} from '../windowBehavior';
import FileDropOverlay from './FileDropOverlay.vue';
import {
  getWindowMetrics,
  petScale,
  responseSound,
  scaledCSSVars,
  setPetScale,
  setResponseSound,
  getResponseSoundFile,
  RESPONSE_SOUND_OFF,
  windowModeForState,
  type ResponseSoundId,
} from '../stores/petSettings';

const WINDOW_WIDTH = WINDOW_SIZE.width;
const WINDOW_HEIGHT = WINDOW_SIZE.height;
const CSS_FRAME_SIZE = WINDOW_WIDTH;

const spriteRef = ref<HTMLDivElement | null>(null);
const responseBubbleRef = ref<HTMLElement | null>(null);
const edgeEnterCanvasRef = ref<HTMLCanvasElement | null>(null);
defineExpose({ spriteRef });

const responseWindowHeightOverride = ref<number | null>(null);

const animState = ref<AnimState>('idle');
const currentFrame = ref(0);
let animationTimer: number | null = null;
let pingpongDir = 1;
let snapAnimationFrame: number | null = null;
let snapAnimationToken = 0;
const PEEK_CURSOR_POLL_INTERVAL_MS = 50;
const PEEK_CURSOR_QUERY_TIMEOUT_MS = 120;

const isCompactWindow = computed(() => isCompactBehavior(windowBehaviorState.value));
const isWindowHidden = computed(() => isHiddenBehavior(windowBehaviorState.value));
const isWindowPeek = computed(() => isPeekBehavior(windowBehaviorState.value));
const showInput = computed(
  () =>
    !isCompactWindow.value &&
    (animState.value === 'EnterInput' ||
      animState.value === 'received' ||
      animState.value === 'Response'),
);
const inputText = ref('');
const inputRef = ref<HTMLInputElement | null>(null);

interface ToolCall {
  name: string;
  status: string;
  detail: string;
}

const bubbleTools = ref<ToolCall[]>([]);
const showBubble = computed(
  () =>
    !isCompactWindow.value &&
    (animState.value === 'working' || animState.value === 'workingPreview') &&
    bubbleTools.value.length > 0,
);

const bubbleDisplayText = computed(() => {
  if (bubbleTools.value.length === 0) return '';
  return bubbleTools.value
    .map((tool) => (tool.detail ? `${tool.name}: ${tool.detail}` : tool.name))
    .join(' | ');
});

const showResponseBubble = computed(() => !isCompactWindow.value && animState.value === 'Response');
const responseBubbleText = ref('思考中...');

const ENVELOPE_REGEX = /^\[([^\]]+)\]\s*/;

interface BubbleContent {
  envelope: string | null;
  html: string;
}

function parseBubbleContent(text: string): BubbleContent {
  const match = text.match(ENVELOPE_REGEX);
  let envelope: string | null = null;
  let content = text;

  if (match) {
    envelope = match[1];
    content = text.slice(match[0].length);
  }

  return {
    envelope,
    html: marked.parse(content, { breaks: true, gfm: true }) as string,
  };
}

const responseContent = computed(() => parseBubbleContent(responseBubbleText.value));

const showQueueBubble = ref(false);
const queueBubbleText = ref('');

interface DroppedFile {
  name: string;
  icon: string;
  path: string;
}

const droppedFiles = ref<DroppedFile[]>([]);

// 播放 Response 音效
function playResponseSound() {
  const soundId = responseSound.value;
  if (soundId === RESPONSE_SOUND_OFF) {
    return;
  }

  const soundFile = getResponseSoundFile(soundId);
  if (!soundFile) {
    return;
  }

  try {
    const audio = new Audio(soundFile);
    audio.volume = 1;
    audio.play().catch((err) => {
      console.warn('[Sound] Failed to play response sound:', err);
    });
  } catch (err) {
    console.warn('[Sound] Failed to create audio:', err);
  }
}

function onRemoveFile(displayIndex: number) {
  droppedFiles.value = droppedFiles.value.filter((_, index) => index !== displayIndex);
  if (droppedFiles.value.length === 0) {
    switchAnim('EnterInput');
  }
}

let dragActive = false;

async function setupDragDrop() {
  const win = getCurrentWindow();
  return win.onDragDropEvent((event) => {
    const payload = event.payload;
    const curState = animState.value;

    if (payload.type === 'enter') {
      dragActive = true;
      console.log('[DRAG] enter, curState=', curState);
      switchAnim('EnterReceiving');
      return;
    }

    if (payload.type === 'drop') {
      dragActive = false;
      const paths: string[] = payload.paths ?? [];
      console.log('[DRAG] drop, files=', paths.length, 'curState=', curState);

      const newFiles: DroppedFile[] = paths.map((path) => {
        const name = path.split('/').pop() ?? path;
        return { name, icon: getFileIcon(name), path };
      });

      const existingNames = new Set(droppedFiles.value.map((file) => file.name));
      const uniqueNewFiles = newFiles.filter((file) => !existingNames.has(file.name));
      droppedFiles.value = [...droppedFiles.value, ...uniqueNewFiles];
      switchAnim('received');
      return;
    }

    if (payload.type === 'leave') {
      console.log('[DRAG] leave, dragActive=', dragActive, 'curState=', curState);
      if (!dragActive) {
        return;
      }

      dragActive = false;
      if (droppedFiles.value.length === 0) {
        switchAnim('idle');
      } else {
        switchAnim('received');
      }
    }
  });
}

let cancelFromEnterInput = false;
let savedAnimState: AnimState | null = null;
const windowBehaviorState = ref<WindowBehaviorState>('normal');
const isWindowDragging = computed(() => windowBehaviorState.value === 'dragging');
const DRAG_START_THRESHOLD = 4;

interface WindowDragSession {
  pointerId: number;
  startScreenX: number;
  startScreenY: number;
  windowHalfWidth: number;
  windowHalfHeight: number;
  scaleFactor: number;
  initialBehavior: WindowBehaviorState;
  started: boolean;
}

let windowDragSession: WindowDragSession | null = null;
let pendingDragPosition: PhysicalPosition | null = null;
let dragPositionFlushInFlight = false;
const edgeMonitorBounds = ref<{ x: number; width: number } | null>(null);

const currentWindowMode = computed(() => windowModeForState(animState.value));
const currentWindowMetrics = computed(() => {
  if (isCompactWindow.value) {
    return getCompactWindowMetrics(petScale.value);
  }

  const base = getWindowMetrics(currentWindowMode.value, petScale.value);

  if (currentWindowMode.value === 'response' && responseWindowHeightOverride.value !== null) {
    return {
      ...base,
      windowHeight: responseWindowHeightOverride.value,
    };
  }

  return base;
});

const rootCSSVars = computed(() => {
  const scaled = scaledCSSVars.value;
  const windowMetrics = currentWindowMetrics.value;

  return {
    '--window-width': `${windowMetrics.windowWidth}px`,
    '--window-height': `${windowMetrics.windowHeight}px`,
    '--cat-width': `${scaled.catWidth}px`,
    '--cat-height': `${scaled.catHeight}px`,
    '--input-bottom': `${scaled.inputBottom}px`,
    '--input-width': `${scaled.inputWidth}px`,
    '--input-height': `${scaled.inputHeight}px`,
    '--input-font-size': `${scaled.inputFontSize}px`,
    '--input-step': `${scaled.inputStep}px`,
    '--file-drop-top': `${scaled.fileDropTop}px`,
    '--file-drop-gap': `${scaled.fileDropGap}px`,
    '--file-icon-size': `${scaled.fileIconSize}px`,
    '--file-name-font-size': `${scaled.fileNameFontSize}px`,
    '--file-name-max-width': `${scaled.fileNameMaxWidth}px`,
    '--close-button-size': `${scaled.closeButtonSize}px`,
    '--close-button-offset': `${scaled.closeButtonOffset}px`,
    '--working-bubble-left': `${scaled.workingBubbleLeft}px`,
    '--working-bubble-top': `${scaled.workingBubbleTop}px`,
    '--working-bubble-width': `${scaled.workingBubbleWidth}px`,
    '--working-bubble-min-height': `${scaled.workingBubbleMinHeight}px`,
    '--working-bubble-font-size': `${scaled.workingBubbleFontSize}px`,
    '--queue-bubble-top': `${scaled.queueBubbleTop}px`,
    '--response-bubble-left': `${scaled.responseBubbleLeft}px`,
    '--response-bubble-bottom': `${scaled.responseBubbleBottom}px`,
    '--response-bubble-width': `${scaled.responseBubbleWidth}px`,
    '--response-bubble-height': `${scaled.responseBubbleHeight}px`,
    '--response-bubble-font-size': `${scaled.responseBubbleFontSize}px`,
    '--pet-scale': String(scaled.scale),
  };
});

let windowLayoutSignature = '';
let syncSequence = 0;

async function resizeWindowPreservingBottomLeft(targetWidth: number, targetHeight: number) {
  const win = getCurrentWindow();
  const [outerPosition, outerSize] = await Promise.all([win.outerPosition(), win.outerSize()]);
  const bottomY = outerPosition.y + outerSize.height;
  const targetY = bottomY - targetHeight;
  const shouldResize = outerSize.width !== targetWidth || outerSize.height !== targetHeight;

  if (!shouldResize) {
    if (outerPosition.y !== targetY) {
      await win.setPosition(new PhysicalPosition(outerPosition.x, targetY));
    }

    return {
      x: outerPosition.x,
      y: targetY,
      width: targetWidth,
      height: targetHeight,
    };
  }

  await win.setSize(new PhysicalSize(targetWidth, targetHeight));

  const resizedPosition = await win.outerPosition();
  const dx = outerPosition.x - resizedPosition.x;
  const dy = targetY - resizedPosition.y;

  if (dx !== 0 || dy !== 0) {
    await win.setPosition(new PhysicalPosition(resizedPosition.x + dx, resizedPosition.y + dy));
  }

  return {
    x: outerPosition.x,
    y: targetY,
    width: targetWidth,
    height: targetHeight,
  };
}

async function syncWindowBounds() {
  const signature = `${currentWindowMode.value}:${windowBehaviorState.value}:${petScale.value.toFixed(1)}`;
  const requestId = ++syncSequence;

  const win = getCurrentWindow();
  const [scaleFactor, outerSize] = await Promise.all([win.scaleFactor(), win.outerSize()]);

  if (requestId !== syncSequence) {
    return;
  }

  const metrics = currentWindowMetrics.value;
  const targetWidth = Math.round(metrics.windowWidth * scaleFactor);
  const targetHeight = Math.round(metrics.windowHeight * scaleFactor);

  if (
    signature === windowLayoutSignature &&
    outerSize.width === targetWidth &&
    outerSize.height === targetHeight
  ) {
    return;
  }

  await resizeWindowPreservingBottomLeft(targetWidth, targetHeight);
  windowLayoutSignature = signature;
}

watch([currentWindowMode, petScale, windowBehaviorState], () => {
  void syncWindowBounds();
});

async function recalcResponseWindowHeight() {
  const bubble = responseBubbleRef.value;
  if (!bubble) {
    return;
  }

  const contentHeight = bubble.offsetHeight;
  const maxHeight = scaledCSSVars.value.responseBubbleHeight;
  const scaledBottom = scaledCSSVars.value.responseBubbleBottom;
  const effectiveHeight = Math.min(contentHeight, maxHeight);
  const newWindowHeight = scaledBottom + effectiveHeight;

  if (responseWindowHeightOverride.value !== newWindowHeight) {
    responseWindowHeightOverride.value = newWindowHeight;
    void syncWindowBounds();
  }
}

watch(animState, async (state, previous) => {
  if (state === 'Response') {
    responseWindowHeightOverride.value = null;

    if (responseBubbleText.value) {
      await nextTick();
      void recalcResponseWindowHeight();
    }
    return;
  }

  if (previous === 'Response') {
    responseWindowHeightOverride.value = null;
    void syncWindowBounds();
  }
});

watch(responseBubbleText, async () => {
  if (animState.value !== 'Response') {
    return;
  }

  await nextTick();
  void recalcResponseWindowHeight();
});

watch(petScale, async () => {
  if (animState.value !== 'Response') {
    return;
  }

  await nextTick();
  void recalcResponseWindowHeight();
});

watch(isWindowDragging, () => {
  currentFrame.value = currentAnim.value.startFrame;
  startAnimation();
});

async function onEnterPress() {
  if (!inputText.value.trim()) {
    cancelFromEnterInput = true;
    switchAnim('working');
    return;
  }

  const text = inputText.value;
  console.log('[INPUT] submit:', text);

  try {
    let fullMessage = text;
    if (droppedFiles.value.length > 0) {
      const attachLines = ['\n\n[附件]'];
      for (const file of droppedFiles.value) {
        const ext = file.name.split('.').pop()?.toLowerCase() ?? '';
        attachLines.push(`文件: ${file.path}`);
        attachLines.push(`类型: ${getMimeType(ext)}`);
      }
      fullMessage += attachLines.join('\n');
    }

    await invoke('send_message', { message: fullMessage, attachments: [] });
    inputText.value = '';
    droppedFiles.value = [];
    switchAnim('working');
  } catch (error) {
    console.error('[INPUT] send failed:', error);
    queueBubbleText.value = `发送失败：${error}`;
    showQueueBubble.value = true;
  }
}

function getMimeType(ext: string): string {
  const mimeMap: Record<string, string> = {
    jpg: 'image/jpeg',
    jpeg: 'image/jpeg',
    png: 'image/png',
    gif: 'image/gif',
    bmp: 'image/bmp',
    svg: 'image/svg+xml',
    webp: 'image/webp',
    ico: 'image/x-icon',
    tiff: 'image/tiff',
    psd: 'image/vnd.adobe.photoshop',
    raw: 'image/raw',
    mp4: 'video/mp4',
    avi: 'video/x-msvideo',
    mkv: 'video/x-matroska',
    mov: 'video/quicktime',
    wmv: 'video/x-ms-wmv',
    flv: 'video/x-flv',
    webm: 'video/webm',
    m4v: 'video/x-m4v',
    mpg: 'video/mpeg',
    mpeg: 'video/mpeg',
    mp3: 'audio/mpeg',
    wav: 'audio/wav',
    flac: 'audio/flac',
    aac: 'audio/aac',
    ogg: 'audio/ogg',
    m4a: 'audio/mp4',
    wma: 'audio/x-ms-wma',
    ape: 'audio/ape',
    pdf: 'application/pdf',
    doc: 'application/msword',
    docx: 'application/vnd.openxmlformats-officedocument.wordprocessingml.document',
    xls: 'application/vnd.ms-excel',
    xlsx: 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
    ppt: 'application/vnd.ms-powerpoint',
    pptx: 'application/vnd.openxmlformats-officedocument.presentationml.presentation',
    txt: 'text/plain',
    html: 'text/html',
    htm: 'text/html',
    css: 'text/css',
    js: 'application/javascript',
    ts: 'application/typescript',
    json: 'application/json',
    xml: 'application/xml',
    md: 'text/markdown',
    zip: 'application/zip',
    rar: 'application/vnd.rar',
    '7z': 'application/x-7z-compressed',
    tar: 'application/x-tar',
    gz: 'application/gzip',
  };

  return mimeMap[ext] ?? 'application/octet-stream';
}

function focusInput() {
  window.setTimeout(() => {
    inputRef.value?.focus();
  }, 50);
}

function onInputBlur() {
  if (animState.value !== 'EnterInput') {
    return;
  }

  cancelFromEnterInput = true;
  switchAnim('idle');
}

const idleImage = new Image();
idleImage.src = '/anim/ildesanimation.png';
const draggingImage = new Image();
draggingImage.src = '/anim/edge-placeholders/dragging.png';
const hideLeftImage = new Image();
hideLeftImage.src = '/anim/edge-placeholders/hide_left.png';
const hideRightImage = new Image();
hideRightImage.src = '/anim/edge-placeholders/hide_right.png';
const peekLeftImage = new Image();
peekLeftImage.src = '/anim/edge-placeholders/peek_left.png';
const peekRightImage = new Image();
peekRightImage.src = '/anim/edge-placeholders/peek_right.png';
const enterImage = new Image();
enterImage.src = '/anim/talkmode.png';
const startworkingImage = new Image();
startworkingImage.src = '/anim/startworking.png';
const receivingImage = new Image();
receivingImage.src = '/anim/receiving.png';
const receivedImage = new Image();
receivedImage.src = '/anim/received.png';
const shockImage = new Image();
shockImage.src = '/anim/shock.png';
const edgeEnterImage = new Image();
edgeEnterImage.src = '/anim/edge-placeholders/edgeEnter.png';
const edgePeekLoopImage = new Image();
edgePeekLoopImage.src = '/anim/edge-placeholders/edgePeekLoop.png';

const currentAnim = computed(() => {
  if (windowBehaviorState.value === 'dragging') {
    return WINDOW_BEHAVIOR_ANIMATIONS.dragging;
  }

  if (windowBehaviorState.value === 'edgeHiddenLeft') {
    return WINDOW_BEHAVIOR_ANIMATIONS.edgeHiddenLeft;
  }

  if (windowBehaviorState.value === 'edgeHiddenRight') {
    return WINDOW_BEHAVIOR_ANIMATIONS.edgeHiddenRight;
  }

  if (windowBehaviorState.value === 'edgePeekLeft') {
    return WINDOW_BEHAVIOR_ANIMATIONS.edgePeekLeft;
  }

  if (windowBehaviorState.value === 'edgePeekRight') {
    return WINDOW_BEHAVIOR_ANIMATIONS.edgePeekRight;
  }

  return ANIMATIONS[animState.value];
});
const cols = computed(() => currentAnim.value.cols);
const rows = computed(() => currentAnim.value.rows);
const totalFrames = computed(() => currentAnim.value.totalFrames);
const interval = computed(() => currentAnim.value.interval);
const currentImage = computed(() => currentAnim.value.imagePath);
const animLoop = computed(() => currentAnim.value.loop);
const animPingpong = computed(() => currentAnim.value.pingpong);
const startFrame = computed(() => currentAnim.value.startFrame);

function getFramePosition(frame: number, columns: number) {
  const column = frame % columns;
  const row = Math.floor(frame / columns);
  return {
    bgX: column * CSS_FRAME_SIZE,
    bgY: row * CSS_FRAME_SIZE,
  };
}

const backgroundStyle = computed(() => {
  const { bgX, bgY } = getFramePosition(currentFrame.value, cols.value);
  const spriteWidth = cols.value * CSS_FRAME_SIZE * petScale.value;
  const spriteHeight = rows.value * CSS_FRAME_SIZE * petScale.value;
  const isMirrored = windowBehaviorState.value === 'edgeHiddenLeft';

  return {
    backgroundImage: `url(${currentImage.value})`,
    backgroundRepeat: 'no-repeat',
    backgroundPosition: `-${bgX * petScale.value}px -${bgY * petScale.value}px`,
    backgroundSize: `${spriteWidth}px ${spriteHeight}px`,
    width: `${WINDOW_WIDTH * petScale.value}px`,
    height: `${WINDOW_HEIGHT * petScale.value}px`,
    transform: `translateZ(0)${isMirrored ? ' scaleX(-1)' : ''}`,
    willChange: 'background-position',
  };
});

function stopAnimation() {
  if (animationTimer === null) {
    return;
  }

  clearInterval(animationTimer);
  animationTimer = null;
}

function startAnimation() {
  stopAnimation();
  const anim = currentAnim.value;
  const firstFrame = startFrame.value;
  pingpongDir = 1;

  animationTimer = window.setInterval(() => {
    if (animLoop.value) {
      if (animPingpong.value) {
        const lastFrame = firstFrame + totalFrames.value - 1;
        let next = currentFrame.value + pingpongDir;
        if (next >= lastFrame) {
          next = lastFrame - 1;
          pingpongDir = -1;
        } else if (next <= firstFrame) {
          next = firstFrame + 1;
          pingpongDir = 1;
        }
        currentFrame.value = next;
        return;
      }

      const relativeFrame = currentFrame.value - firstFrame;
      currentFrame.value = firstFrame + ((relativeFrame + 1) % totalFrames.value);
      return;
    }

    const relativeFrame = currentFrame.value - firstFrame;
    const nextRelative = (relativeFrame + 1) % totalFrames.value;

    if (nextRelative === 0) {
      stopAnimation();
      if (animState.value === 'shock' && savedAnimState) {
        const restored = savedAnimState;
        savedAnimState = null;
        switchAnim(restored);
      } else if (anim.nextState) {
        switchAnim(anim.nextState);
      }
      return;
    }

    currentFrame.value = firstFrame + nextRelative;
  }, interval.value);
}

function setWindowBehavior(next: WindowBehaviorState) {
  if (windowBehaviorState.value === next) {
    return;
  }

  console.log('[WINDOW] behavior', windowBehaviorState.value, '->', next);
  windowBehaviorState.value = next;
}

async function startWindowDragging() {
  cancelSnapAnimation();
  setWindowBehavior('dragging');
}

function cancelSnapAnimation() {
  snapAnimationToken += 1;

  if (snapAnimationFrame !== null) {
    window.clearTimeout(snapAnimationFrame);
    snapAnimationFrame = null;
  }
}

function getMonitorBounds(monitor: Awaited<ReturnType<typeof currentMonitor>>) {
  if (!monitor) {
    return null;
  }

  return {
    x: monitor.workArea.position.x,
    width: monitor.workArea.size.width,
  };
}

async function resolveSnapContext(session: WindowDragSession, event: PointerEvent) {
  const cursorX = Math.round(event.screenX * session.scaleFactor);
  const cursorY = Math.round(event.screenY * session.scaleFactor);
  const compactWindowWidth = Math.round(getCompactWindowMetrics(petScale.value).windowWidth * session.scaleFactor);
  const snapThreshold = getEdgeSnapThreshold(compactWindowWidth);
  const monitor =
    (await monitorFromPoint(cursorX, cursorY)) ??
    (await currentMonitor());
  const monitorBounds = getMonitorBounds(monitor);

  if (!monitorBounds) {
    return null;
  }

  const snapSide = computeSnapSide(cursorX, compactWindowWidth, monitorBounds, snapThreshold);

  return {
    compactWindowWidth,
    monitorBounds,
    snapSide,
  };
}

async function applyHiddenWindowBounds(scaleFactor: number) {
  const metrics = getCompactWindowMetrics(petScale.value);
  const targetWidth = Math.round(metrics.windowWidth * scaleFactor);
  const targetHeight = Math.round(metrics.windowHeight * scaleFactor);
  const bounds = await resizeWindowPreservingBottomLeft(targetWidth, targetHeight);

  return {
    targetWidth: bounds.width,
    targetHeight: bounds.height,
    targetY: bounds.y,
  };
}

interface EdgeBehaviorContext {
  behavior: Extract<
    WindowBehaviorState,
    'edgeHiddenLeft' | 'edgeHiddenRight' | 'edgePeekLeft' | 'edgePeekRight'
  >;
  monitorBounds: { x: number; width: number };
  windowWidth: number;
  windowHeight: number;
  windowY: number;
  hiddenRevealPx: number;
  peekRevealPx: number;
}

function isCursorInsideHiddenTriggerZone(cursor: PhysicalPosition, context: EdgeBehaviorContext) {
  const monitorRight = context.monitorBounds.x + context.monitorBounds.width;
  const visibleStartX =
    context.behavior === 'edgeHiddenLeft'
      ? context.monitorBounds.x
      : monitorRight - context.hiddenRevealPx;
  const visibleEndX =
    context.behavior === 'edgeHiddenLeft'
      ? context.monitorBounds.x + context.hiddenRevealPx
      : monitorRight;

  return (
    cursor.x >= visibleStartX &&
    cursor.x <= visibleEndX &&
    cursor.y >= context.windowY &&
    cursor.y <= context.windowY + context.windowHeight
  );
}

function isCursorInsidePeekWindow(cursor: PhysicalPosition, context: EdgeBehaviorContext) {
  const windowLeft =
    context.behavior === 'edgePeekLeft'
      ? context.monitorBounds.x - context.windowWidth + context.peekRevealPx
      : context.monitorBounds.x + context.monitorBounds.width - context.peekRevealPx;
  const windowRight = windowLeft + context.windowWidth;

  return (
    cursor.x >= windowLeft &&
    cursor.x <= windowRight &&
    cursor.y >= context.windowY &&
    cursor.y <= context.windowY + context.windowHeight
  );
}

async function getEdgeBehaviorContext(): Promise<EdgeBehaviorContext | null> {
  const behavior = windowBehaviorState.value;
  if (!isHiddenBehavior(behavior) && !isPeekBehavior(behavior)) {
    return null;
  }

  const win = getCurrentWindow();
  const [outerPosition, outerSize] = await Promise.all([win.outerPosition(), win.outerSize()]);
  const monitorBounds = edgeMonitorBounds.value;

  if (!monitorBounds) {
    return null;
  }

  return {
    behavior: behavior as EdgeBehaviorContext['behavior'],
    monitorBounds,
    windowWidth: outerSize.width,
    windowHeight: outerSize.height,
    windowY: outerPosition.y,
    hiddenRevealPx: getHiddenRevealPx(outerSize.width),
    peekRevealPx: getPeekRevealPx(outerSize.width),
  };
}

async function withTimeout<T>(promise: Promise<T>, timeoutMs: number): Promise<T> {
  let timer: number | null = null;

  try {
    return await Promise.race([
      promise,
      new Promise<T>((_, reject) => {
        timer = window.setTimeout(() => reject(new Error(`timeout:${timeoutMs}`)), timeoutMs);
      }),
    ]);
  } finally {
    if (timer !== null) {
      window.clearTimeout(timer);
    }
  }
}

let peekLeaveTimer: number | null = null;
let peekTransitionInFlight = false;
let peekCursorPollTimer: number | null = null;

const isEdgeEnterPlaying = ref(false);
const edgeEnterFrame = ref(0);
const edgeEnterDirection = ref<'left' | 'right'>('right');
const edgeEnterMode = ref<'snap' | 'peek' | null>(null);
let edgeEnterAnimationTimer: number | null = null;

function cancelEdgeEnterAnimation() {
  if (edgeEnterAnimationTimer !== null) {
    clearTimeout(edgeEnterAnimationTimer);
    edgeEnterAnimationTimer = null;
  }
  isEdgeEnterPlaying.value = false;
  edgeEnterMode.value = null;
}

function renderEdgeEnterFrame() {
  const canvas = edgeEnterCanvasRef.value;
  if (!canvas) return;

  const ctx = canvas.getContext('2d');
  if (!ctx) return;

  const frame = edgeEnterFrame.value;
  const cols = 4;
  const frameSize = 960;
  const col = frame % cols;
  const row = Math.floor(frame / cols);
  const srcX = col * frameSize;
  const srcY = row * frameSize;

  const scale = petScale.value;
  const displaySize = 180 * scale;
  canvas.width = displaySize;
  canvas.height = displaySize;

  ctx.clearRect(0, 0, displaySize, displaySize);

  if (edgeEnterDirection.value === 'left') {
    ctx.save();
    ctx.translate(displaySize, 0);
    ctx.scale(-1, 1);
    ctx.drawImage(
      edgeEnterImage,
      srcX, srcY, frameSize, frameSize,
      0, 0, displaySize, displaySize,
    );
    ctx.restore();
  } else {
    ctx.drawImage(
      edgeEnterImage,
      srcX, srcY, frameSize, frameSize,
      0, 0, displaySize, displaySize,
    );
  }
}

function playEdgeEnterAnimation(
  direction: 'left' | 'right',
  mode: 'snap' | 'peek',
  reverse: boolean,
  endFrame: number,
  onComplete?: () => void,
) {
  if (isEdgeEnterPlaying.value) return;

  cancelEdgeEnterAnimation();
  isEdgeEnterPlaying.value = true;
  edgeEnterDirection.value = direction;
  edgeEnterMode.value = mode;

  const totalFrames = 13;
  const interval = 23;
  let frameIndex = reverse ? totalFrames - 1 : 0;

  function tick() {
    edgeEnterFrame.value = frameIndex;
    renderEdgeEnterFrame();
    frameIndex = reverse ? frameIndex - 1 : frameIndex + 1;

    if (reverse ? frameIndex >= 0 : frameIndex < totalFrames) {
      edgeEnterAnimationTimer = window.setTimeout(tick, interval);
    } else {
      edgeEnterFrame.value = endFrame;
      renderEdgeEnterFrame();
      isEdgeEnterPlaying.value = false;
      edgeEnterMode.value = null;
      edgeEnterAnimationTimer = null;
      onComplete?.();
    }
  }

  tick();
}

// EdgePeekLoop animation state (looping while hovering in peek)
const isEdgePeekLoopPlaying = ref(false);
const edgePeekLoopFrame = ref(0);
const edgePeekLoopDirection = ref<'left' | 'right'>('right');
let edgePeekLoopTimer: number | null = null;

function cancelEdgePeekLoopAnimation() {
  if (edgePeekLoopTimer !== null) {
    clearTimeout(edgePeekLoopTimer);
    edgePeekLoopTimer = null;
  }
  isEdgePeekLoopPlaying.value = false;
}

function renderEdgePeekLoopFrame() {
  const canvas = edgeEnterCanvasRef.value;
  if (!canvas) return;

  const ctx = canvas.getContext('2d');
  if (!ctx) return;

  const frame = edgePeekLoopFrame.value;
  const cols = 3;
  const frameSize = 960;
  const col = frame % cols;
  const row = Math.floor(frame / cols);
  const srcX = col * frameSize;
  const srcY = row * frameSize;

  const scale = petScale.value;
  const displaySize = 180 * scale;
  canvas.width = displaySize;
  canvas.height = displaySize;

  ctx.clearRect(0, 0, displaySize, displaySize);

  if (edgePeekLoopDirection.value === 'left') {
    ctx.save();
    ctx.translate(displaySize, 0);
    ctx.scale(-1, 1);
    ctx.drawImage(
      edgePeekLoopImage,
      srcX, srcY, frameSize, frameSize,
      0, 0, displaySize, displaySize,
    );
    ctx.restore();
  } else {
    ctx.drawImage(
      edgePeekLoopImage,
      srcX, srcY, frameSize, frameSize,
      0, 0, displaySize, displaySize,
    );
  }
}

function playEdgePeekLoopAnimation(direction: 'left' | 'right') {
  cancelEdgePeekLoopAnimation();
  isEdgePeekLoopPlaying.value = true;
  edgePeekLoopDirection.value = direction;

  const totalFrames = 6;
  const interval = 250; // 1.5s / 6 frames
  let frameIndex = 0;

  function tick() {
    edgePeekLoopFrame.value = frameIndex;
    renderEdgePeekLoopFrame();
    frameIndex = (frameIndex + 1) % totalFrames;
    edgePeekLoopTimer = window.setTimeout(tick, interval);
  }

  tick();
}

function cancelPeekLeaveTimer() {
  if (peekLeaveTimer !== null) {
    window.clearTimeout(peekLeaveTimer);
    peekLeaveTimer = null;
  }
}

function stopPeekCursorTracking() {
  if (peekCursorPollTimer !== null) {
    window.clearInterval(peekCursorPollTimer);
    peekCursorPollTimer = null;
  }
}

function startPeekCursorTracking() {
  if (peekCursorPollTimer !== null) {
    return;
  }

  peekCursorPollTimer = window.setInterval(() => {
    void pollPeekCursor();
  }, PEEK_CURSOR_POLL_INTERVAL_MS);
}

function transitionToEdgeBehavior(next: Extract<WindowBehaviorState, 'edgePeekLeft' | 'edgePeekRight'>) {
  if (peekTransitionInFlight || windowBehaviorState.value === next) {
    return;
  }

  console.log('[PEEK] transition to', next);
  peekTransitionInFlight = true;
  cancelPeekLeaveTimer();
  setWindowBehavior(next);

  // Play edgeEnter animation forwards (0→12) for peek entry, then start edgePeekLoop
  const direction = next === 'edgePeekLeft' ? 'left' : 'right';
  playEdgeEnterAnimation(direction, 'peek', false, 12, () => {
    // After edgeEnter completes, start edgePeekLoop animation
    playEdgePeekLoopAnimation(direction);
  });

  peekTransitionInFlight = false;
}

function schedulePeekHide(next: 'edgeHiddenLeft' | 'edgeHiddenRight') {
  if (peekLeaveTimer !== null || isEdgeEnterPlaying.value) {
    return;
  }

  peekLeaveTimer = window.setTimeout(() => {
    peekLeaveTimer = null;
    if (windowBehaviorState.value !== (next === 'edgeHiddenLeft' ? 'edgePeekLeft' : 'edgePeekRight')) {
      return;
    }

    // Cancel edgePeekLoop first, then play edgeEnter animation backwards (12→0)
    cancelEdgePeekLoopAnimation();
    const direction = next === 'edgeHiddenLeft' ? 'left' : 'right';
    playEdgeEnterAnimation(direction, 'peek', true, 0, () => {
      setWindowBehavior(next);
    });
  }, EDGE_PEEK_LEAVE_DELAY);
}

async function pollPeekCursor() {
  if (peekTransitionInFlight || (!isWindowHidden.value && !isWindowPeek.value)) {
    return;
  }

  const context = await getEdgeBehaviorContext();
  if (!context) {
    return;
  }

  let cursor: PhysicalPosition;

  try {
    cursor = await withTimeout(cursorPosition(), PEEK_CURSOR_QUERY_TIMEOUT_MS);
  } catch (error) {
    console.error('[PEEK] cursor polling failed:', error);
    return;
  }

  if (isHiddenBehavior(context.behavior)) {
    if (isCursorInsideHiddenTriggerZone(cursor, context)) {
      void transitionToEdgeBehavior(
        context.behavior === 'edgeHiddenLeft' ? 'edgePeekLeft' : 'edgePeekRight',
      );
    }
    return;
  }

  if (isCursorInsidePeekWindow(cursor, context)) {
    cancelPeekLeaveTimer();
    return;
  }

  schedulePeekHide(context.behavior === 'edgePeekLeft' ? 'edgeHiddenLeft' : 'edgeHiddenRight');
}

async function flushPendingDragPosition() {
  if (dragPositionFlushInFlight || pendingDragPosition === null) {
    return;
  }

  dragPositionFlushInFlight = true;

  try {
    while (pendingDragPosition !== null) {
      const nextPosition = pendingDragPosition;
      pendingDragPosition = null;
      await getCurrentWindow().setPosition(nextPosition);
    }
  } catch (error) {
    console.error('[WINDOW] setPosition during drag failed:', error);
  } finally {
    dragPositionFlushInFlight = false;
  }
}

function queueDragWindowPosition(position: PhysicalPosition) {
  pendingDragPosition = position;
  void flushPendingDragPosition();
}

async function beginWindowDrag(event: PointerEvent) {
  cancelSnapAnimation();

  const win = getCurrentWindow();
  const scaleFactor = await win.scaleFactor();
  const metrics = getCompactWindowMetrics(petScale.value);

  windowDragSession = {
    pointerId: event.pointerId,
    startScreenX: event.screenX,
    startScreenY: event.screenY,
    windowHalfWidth: Math.round((metrics.windowWidth * scaleFactor) / 2),
    windowHalfHeight: Math.round((metrics.windowHeight * scaleFactor) / 2),
    scaleFactor,
    initialBehavior: windowBehaviorState.value,
    started: false,
  };
}

async function finishWindowDrag(event?: PointerEvent) {
  if (!windowDragSession) {
    return;
  }

  const session = windowDragSession;
  windowDragSession = null;
  pendingDragPosition = null;

  if (!session.started || !event) {
    setWindowBehavior(session.initialBehavior === 'dragging' ? 'normal' : session.initialBehavior);
    return;
  }

  const snapContext = await resolveSnapContext(session, event);

  if (!snapContext?.snapSide) {
    setWindowBehavior('normal');
    return;
  }

  const targetBehavior =
    snapContext.snapSide === 'left' ? 'edgeHiddenLeft' : 'edgeHiddenRight';
  const hiddenRevealPx = getHiddenRevealPx(snapContext.compactWindowWidth);
  edgeMonitorBounds.value = snapContext.monitorBounds;

  setWindowBehavior(targetBehavior);
  await nextTick();

  const { targetWidth, targetY } = await applyHiddenWindowBounds(session.scaleFactor);
  const targetX = getBehaviorTargetX(
    targetBehavior,
    snapContext.monitorBounds,
    targetWidth,
    hiddenRevealPx,
    getPeekRevealPx(targetWidth),
  );

  if (targetX === null) {
    return;
  }

  // Instantly move to target position
  await getCurrentWindow().setPosition(new PhysicalPosition(targetX, targetY));
  // Play edgeEnter animation (snap mode: backwards 12→0, blocking)
  playEdgeEnterAnimation(snapContext.snapSide, 'snap', true, 0);
}

function switchAnim(newState: AnimState) {
  const previous = animState.value;
  animState.value = newState;
  currentFrame.value = ANIMATIONS[newState].startFrame;
  startAnimation();

  if (newState === 'EnterInput') {
    focusInput();
  }

  if (newState === 'shock') {
    savedAnimState = previous;
  }

  if (newState === 'workingPreview') {
    bubbleTools.value = [{ name: 'tool_call', status: '', detail: 'preview mode' }];
  }
}

let lastClickTime = 0;

async function onPointerDown(event: PointerEvent) {
  if (event.button !== 0) {
    return;
  }

  // Cancel any ongoing edgeEnter/edgePeekLoop animation when starting a drag from hidden/peek
  if ((isEdgeEnterPlaying.value || isEdgePeekLoopPlaying.value) && (isWindowHidden.value || isWindowPeek.value)) {
    cancelEdgeEnterAnimation();
    cancelEdgePeekLoopAnimation();
    cancelPeekLeaveTimer();
  }

  const now = Date.now();
  const clickX = event.offsetX;
  const clickY = event.offsetY;
  const dragWidth = scaledCSSVars.value.catWidth;
  const dragHeight = scaledCSSVars.value.catHeight;
  const isInDragArea = clickX >= 0 && clickX <= dragWidth && clickY >= 0 && clickY <= dragHeight;

  if ((isWindowHidden.value || isWindowPeek.value) && isInDragArea) {
    spriteRef.value?.setPointerCapture(event.pointerId);
    await beginWindowDrag(event);
  } else if (now - lastClickTime < 300) {
    if (animState.value === 'idle' || animState.value === 'working') {
      if (cancelFromEnterInput && animState.value === 'working') {
        cancelFromEnterInput = false;
      } else if (responseBubbleText.value !== '思考中...') {
        switchAnim('Response');
      } else {
        switchAnim('EnterInput');
      }
    } else {
      switchAnim('idle');
    }
  } else if (isInDragArea) {
    spriteRef.value?.setPointerCapture(event.pointerId);
    await beginWindowDrag(event);
  }

  lastClickTime = now;
}

function onPointerMove(event: PointerEvent) {
  if (isEdgeEnterPlaying.value) {
    return;
  }

  if (!windowDragSession) {
    if (peekTransitionInFlight) {
      return;
    }

    if (windowBehaviorState.value === 'edgeHiddenLeft') {
      void transitionToEdgeBehavior('edgePeekLeft');
      return;
    }

    if (windowBehaviorState.value === 'edgeHiddenRight') {
      void transitionToEdgeBehavior('edgePeekRight');
      return;
    }

    if (windowBehaviorState.value === 'edgePeekLeft' || windowBehaviorState.value === 'edgePeekRight') {
      cancelPeekLeaveTimer();
    }

    return;
  }

  if (event.pointerId !== windowDragSession.pointerId) {
    return;
  }

  const screenX = Math.round(event.screenX * windowDragSession.scaleFactor);
  const screenY = Math.round(event.screenY * windowDragSession.scaleFactor);

  if (!windowDragSession.started) {
    const movedEnough =
      Math.abs(event.screenX - windowDragSession.startScreenX) >= DRAG_START_THRESHOLD ||
      Math.abs(event.screenY - windowDragSession.startScreenY) >= DRAG_START_THRESHOLD;

    if (!movedEnough) {
      return;
    }

    windowDragSession.started = true;
    void startWindowDragging();
  }

  const targetX = screenX - windowDragSession.windowHalfWidth;
  const targetY = screenY - windowDragSession.windowHalfHeight;
  queueDragWindowPosition(new PhysicalPosition(targetX, targetY));
}

function onPointerUp(event: PointerEvent) {
  if (isEdgeEnterPlaying.value) {
    return;
  }

  if (!windowDragSession || event.pointerId !== windowDragSession.pointerId) {
    return;
  }

  if (spriteRef.value?.hasPointerCapture(event.pointerId)) {
    spriteRef.value.releasePointerCapture(event.pointerId);
  }

  void finishWindowDrag(event);
}

function onPointerCancel(event: PointerEvent) {
  if (isEdgeEnterPlaying.value) {
    return;
  }

  if (!windowDragSession || event.pointerId !== windowDragSession.pointerId) {
    return;
  }

  void finishWindowDrag();
}

function onSpritePointerEnter() {
  cancelPeekLeaveTimer();

  if (peekTransitionInFlight || windowDragSession) {
    return;
  }

  if (windowBehaviorState.value === 'edgeHiddenLeft') {
    void transitionToEdgeBehavior('edgePeekLeft');
    return;
  }

  if (windowBehaviorState.value === 'edgeHiddenRight') {
    void transitionToEdgeBehavior('edgePeekRight');
  }
}

function onSpritePointerLeave() {
  if (peekTransitionInFlight || windowDragSession) {
    return;
  }

  if (windowBehaviorState.value === 'edgePeekLeft') {
    schedulePeekHide('edgeHiddenLeft');
    return;
  }

  if (windowBehaviorState.value === 'edgePeekRight') {
    schedulePeekHide('edgeHiddenRight');
  }
}

function onContextMenu(event: MouseEvent) {
  event.preventDefault();
  void showContextMenu(event.shiftKey);
}

interface BubblePayload {
  tools: ToolCall[];
}

interface ResponseBubblePayload {
  text: string;
}

interface ScaleChangedPayload {
  scale?: number;
}

interface ResponseSoundChangedPayload {
  soundId?: string;
}

let unlistenSwitch: (() => void) | null = null;
let unlistenBubble: (() => void) | null = null;
let unlistenResponseBubble: (() => void) | null = null;
let unlistenDrag: (() => void) | null = null;
let unlistenScale: (() => void) | null = null;
let unlistenSound: (() => void) | null = null;
let removeStorageListener: (() => void) | null = null;

watch(windowBehaviorState, (state) => {
  cancelPeekLeaveTimer();

  if (isHiddenBehavior(state) || isPeekBehavior(state)) {
    startPeekCursorTracking();
    return;
  }

  if (state === 'normal' || state === 'dragging') {
    edgeMonitorBounds.value = null;
  }

  stopPeekCursorTracking();
});

onMounted(async () => {
  onSwitchAnim((state) => {
    switchAnim(state);
  });

  unlistenSwitch = await listen<string>('switch-animation', (event) => {
    const state = event.payload as AnimState;
    if (state && ANIMATIONS[state]) {
      switchAnim(state);
    }
  });

  unlistenBubble = await listen<BubblePayload>('update-bubble', (event) => {
    if (event.payload.tools) {
      bubbleTools.value = event.payload.tools;
    }
  });

  let lastResponseText = '';
  unlistenResponseBubble = await listen<ResponseBubblePayload>(
    'update-response-bubble',
    (event) => {
      if (event.payload.text && event.payload.text !== lastResponseText) {
        lastResponseText = event.payload.text;
        responseBubbleText.value = event.payload.text;
        playResponseSound();
      }
    },
  );

  unlistenScale = await listen<ScaleChangedPayload>('pet-scale-changed', (event) => {
    if (typeof event.payload?.scale === 'number') {
      setPetScale(event.payload.scale);
    }
  });

  unlistenSound = await listen<ResponseSoundChangedPayload>('response-sound-changed', (event) => {
    if (typeof event.payload?.soundId === 'string') {
      setResponseSound(event.payload.soundId as ResponseSoundId);
    }
  });

  const onStorage = (event: StorageEvent) => {
    if (event.key !== 'meoclaw.petScale' || event.newValue == null) {
      return;
    }

    setPetScale(Number(event.newValue));
  };

  window.addEventListener('storage', onStorage);
  removeStorageListener = () => window.removeEventListener('storage', onStorage);

  unlistenDrag = await setupDragDrop();

  await Promise.all(
    [
      idleImage,
      draggingImage,
      hideLeftImage,
      hideRightImage,
      peekLeftImage,
      peekRightImage,
      enterImage,
      startworkingImage,
      receivingImage,
      receivedImage,
      shockImage,
      edgeEnterImage,
      edgePeekLoopImage,
    ].map(
      (image) =>
        new Promise<void>((resolve) => {
          if (image.complete && image.naturalWidth > 0) {
            resolve();
            return;
          }

          image.onload = () => resolve();
        }),
    ),
  );

  startAnimation();
  await syncWindowBounds();
});

onUnmounted(() => {
  stopAnimation();
  unlistenSwitch?.();
  unlistenBubble?.();
  unlistenResponseBubble?.();
  unlistenDrag?.();
  unlistenScale?.();
  unlistenSound?.();
  removeStorageListener?.();
  cancelSnapAnimation();
  cancelPeekLeaveTimer();
  stopPeekCursorTracking();
  void finishWindowDrag();
});
</script>

<template>
  <div
    class="pet-root"
    :class="{ 'is-window-dragging': isWindowDragging }"
    :data-window-behavior="windowBehaviorState"
    :style="rootCSSVars"
  >
    <div v-if="showInput" class="pixel-bar">
      <input
        ref="inputRef"
        v-model="inputText"
        class="pixel-input"
        type="text"
        placeholder="ask me anything..."
        @keydown.enter="onEnterPress"
        @blur="onInputBlur"
      />
      <div class="mic-btn" aria-hidden="true">
        <svg
          class="mic-icon"
          viewBox="0 0 24 24"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
        >
          <rect x="9" y="3" width="6" height="10" rx="3" fill="currentColor" />
          <path
            d="M7 10.5C7 13.2614 9.23858 15.5 12 15.5C14.7614 15.5 17 13.2614 17 10.5"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="square"
          />
          <path d="M12 15.5V20" stroke="currentColor" stroke-width="2" stroke-linecap="square" />
          <path d="M9 20H15" stroke="currentColor" stroke-width="2" stroke-linecap="square" />
        </svg>
      </div>
    </div>

    <FileDropOverlay
      v-if="animState === 'received' && !isCompactWindow"
      :files="droppedFiles"
      @remove="onRemoveFile"
    />

    <div v-if="showBubble" class="bubble-overlay">
      <span class="bubble-text">{{ bubbleDisplayText }}</span>
    </div>

    <div v-if="showBubble && showQueueBubble" class="queue-bubble-overlay">
      <span class="bubble-text">{{ queueBubbleText }}</span>
    </div>

    <div v-if="showResponseBubble" class="response-bubble-container">
      <div ref="responseBubbleRef" class="response-bubble">
        <div class="response-bubble-text">
          <span v-if="responseContent.envelope" class="bubble-envelope">
            {{ responseContent.envelope }}
          </span>
          <div class="bubble-markdown" v-html="responseContent.html"></div>
        </div>
      </div>
    </div>

    <div
      ref="spriteRef"
      class="desktop-pet"
      :class="{ 'is-window-dragging': isWindowDragging, 'is-edge-enter-hidden': isEdgeEnterPlaying || isEdgePeekLoopPlaying }"
      :style="backgroundStyle"
      @pointerdown="onPointerDown"
      @pointerenter="onSpritePointerEnter"
      @pointerleave="onSpritePointerLeave"
      @pointermove="onPointerMove"
      @pointerup="onPointerUp"
      @pointercancel="onPointerCancel"
      @contextmenu="onContextMenu"
    ></div>

    <canvas
      ref="edgeEnterCanvasRef"
      class="edge-enter-canvas"
      :class="{ 'is-edge-enter-playing': isEdgeEnterPlaying || isEdgePeekLoopPlaying }"
    ></canvas>
  </div>
</template>

<style scoped>
.pet-root {
  position: relative;
  width: var(--window-width);
  height: var(--window-height);
  overflow: hidden;
}

.desktop-pet {
  position: absolute;
  bottom: 0;
  left: 0;
  cursor: grab;
  image-rendering: pixelated;
  user-select: none;
  -webkit-user-select: none;
}

.pet-root.is-window-dragging {
  cursor: grabbing;
}

.pet-root.is-window-dragging * {
  cursor: grabbing !important;
}

.edge-enter-canvas {
  position: absolute;
  bottom: 0;
  left: 0;
  display: none;
  pointer-events: none;
  image-rendering: pixelated;
}

.edge-enter-canvas.is-edge-enter-playing {
  display: block;
  pointer-events: none;
}

.desktop-pet.is-edge-enter-hidden {
  opacity: 0;
}

.desktop-pet.is-window-dragging {
  filter: brightness(1.04) saturate(0.96);
}

.pixel-bar {
  --p-black: #000;
  --p-white: #fff;
  --p-gray-light: #bcbcbc;
  --p-gray-dark: #818181;
  --p-bg: #e7e7e7;
  --step: var(--input-step);
  position: absolute;
  bottom: var(--input-bottom);
  left: calc(var(--cat-width) / 2 - var(--input-width) / 2);
  width: var(--input-width);
  height: var(--input-height);
  display: flex;
  z-index: 10;
  background-color: var(--p-white);
  box-shadow:
    0 calc(-1 * var(--step)) 0 0 var(--p-black),
    0 var(--step) 0 0 var(--p-black),
    calc(-1 * var(--step)) 0 0 0 var(--p-black),
    var(--step) 0 0 0 var(--p-black),
    inset calc(2 * var(--step)) calc(2 * var(--step)) 0 0 var(--p-gray-light),
    inset calc(-2 * var(--step)) calc(-2 * var(--step)) 0 0 var(--p-gray-light),
    inset calc(3 * var(--step)) calc(3 * var(--step)) 0 0 var(--p-white),
    inset calc(-3 * var(--step)) calc(-3 * var(--step)) 0 0 var(--p-gray-dark);
}

.pixel-bar::before {
  content: '';
  position: absolute;
  top: calc(-1 * var(--step));
  left: calc(-1 * var(--step));
  right: calc(-1 * var(--step));
  bottom: calc(-1 * var(--step));
  background-repeat: no-repeat;
  pointer-events: none;
  z-index: 10;
  background-image:
    linear-gradient(var(--p-bg), var(--p-bg)),
    linear-gradient(var(--p-bg), var(--p-bg)),
    linear-gradient(var(--p-bg), var(--p-bg)),
    linear-gradient(var(--p-bg), var(--p-bg)),
    linear-gradient(var(--p-bg), var(--p-bg)),
    linear-gradient(var(--p-bg), var(--p-bg)),
    linear-gradient(var(--p-bg), var(--p-bg)),
    linear-gradient(var(--p-bg), var(--p-bg));
  background-size:
    calc(2 * var(--step)) var(--step),
    var(--step) calc(2 * var(--step)),
    calc(2 * var(--step)) var(--step),
    var(--step) calc(2 * var(--step)),
    calc(2 * var(--step)) var(--step),
    var(--step) calc(2 * var(--step)),
    calc(2 * var(--step)) var(--step),
    var(--step) calc(2 * var(--step));
  background-position:
    0 0,
    0 0,
    100% 0,
    100% 0,
    0 100%,
    0 100%,
    100% 100%,
    100% 100%;
}

.pixel-input {
  flex: 1;
  border: none;
  outline: none;
  padding: 0;
  margin: 0 0 0 calc(1 * var(--step));
  font-size: var(--input-font-size);
  font-family: monospace;
  color: #333;
  background: transparent;
  cursor: text;
  min-width: 0;
  z-index: 5;
}

.pixel-input::placeholder {
  color: #999;
}

.mic-btn {
  display: none;
  width: calc(26px * var(--pet-scale));
  min-width: calc(26px * var(--pet-scale));
  height: calc(var(--input-height) - 2px);
  margin: 1px;
  align-items: center;
  justify-content: center;
  background-color: var(--p-gray-light);
  box-shadow:
    inset calc(-1 * var(--step)) calc(-1 * var(--step)) 0 0 var(--p-gray-dark),
    inset var(--step) var(--step) 0 0 rgba(255, 255, 255, 0.3);
  cursor: pointer;
  z-index: 5;
}

.mic-btn:hover {
  background-color: #a8a8a8;
}

.mic-btn:active {
  box-shadow: inset var(--step) var(--step) 0 0 var(--p-gray-dark);
}

.mic-icon {
  width: calc(18px * var(--pet-scale));
  height: calc(18px * var(--pet-scale));
  color: #333;
}

.bubble-overlay {
  position: absolute;
  left: var(--working-bubble-left);
  top: var(--working-bubble-top);
  width: var(--working-bubble-width);
  min-height: var(--working-bubble-min-height);
  z-index: 20;
  pointer-events: none;
  overflow: hidden;
}

.bubble-text {
  display: inline-block;
  max-width: 100%;
  padding: calc(6px * var(--pet-scale)) calc(12px * var(--pet-scale));
  background: rgba(255, 255, 255, 0.9);
  border-radius: calc(12px * var(--pet-scale));
  font-size: var(--working-bubble-font-size);
  font-family: sans-serif;
  color: #333;
  box-shadow: 0 calc(2px * var(--pet-scale)) calc(8px * var(--pet-scale)) rgba(0, 0, 0, 0.15);
  white-space: nowrap;
  text-overflow: ellipsis;
  overflow: hidden;
}

.queue-bubble-overlay {
  position: absolute;
  top: var(--queue-bubble-top);
  left: 50%;
  transform: translateX(-50%);
  z-index: 20;
  pointer-events: none;
}

.response-bubble-container {
  --bubble-bg: #f7f2ec;
  --bubble-radius: calc(20px * var(--pet-scale));
  --bubble-shadow: 0 calc(4px * var(--pet-scale)) calc(12px * var(--pet-scale)) rgba(60, 40, 20, 0.08);
  --bubble-padding: calc(12px * var(--pet-scale)) calc(16px * var(--pet-scale));
  --bubble-font-size: var(--response-bubble-font-size);
  --bubble-font: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
  --bubble-text-color: #4a3828;
  --envelope-color: #6e4828;
  --envelope-radius: calc(4px * var(--pet-scale));
  --envelope-bg: rgba(110, 72, 40, 0.12);
  --border: #ddd0c2;
  position: absolute;
  left: var(--response-bubble-left);
  bottom: var(--response-bubble-bottom);
  width: var(--response-bubble-width);
  height: auto;
  max-height: var(--response-bubble-height);
  z-index: 9999;
}

.response-bubble {
  width: 100%;
  height: auto;
  max-height: var(--response-bubble-height);
  background: var(--bubble-bg);
  border: 1px solid var(--border);
  border-radius: var(--bubble-radius);
  box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.5), var(--bubble-shadow);
  overflow-y: auto;
  overflow-x: hidden;
  padding: var(--bubble-padding);
  -webkit-overflow-scrolling: touch;
  scrollbar-width: none;
  -ms-overflow-style: none;
}

.response-bubble::-webkit-scrollbar {
  display: none;
}

.response-bubble-text {
  display: block;
  font-size: var(--bubble-font-size);
  font-family: var(--bubble-font);
  color: var(--bubble-text-color);
  line-height: 1.5;
  word-wrap: break-word;
  overflow-wrap: break-word;
}

.bubble-envelope {
  display: inline-block;
  background: var(--envelope-bg);
  color: var(--envelope-color);
  border-radius: var(--envelope-radius);
  padding: calc(1px * var(--pet-scale)) calc(6px * var(--pet-scale));
  font-size: calc(11px * var(--pet-scale));
  font-weight: 600;
  letter-spacing: 0.04em;
  text-transform: uppercase;
  vertical-align: baseline;
  line-height: 1.4;
  font-style: normal;
  margin-right: calc(8px * var(--pet-scale));
  margin-bottom: calc(4px * var(--pet-scale));
}

.bubble-markdown {
  word-wrap: break-word;
  overflow-wrap: break-word;
}

.bubble-markdown :deep(h1),
.bubble-markdown :deep(h2),
.bubble-markdown :deep(h3),
.bubble-markdown :deep(h4) {
  color: var(--bubble-text-color, #d8c8b8);
  margin-top: 1em;
  margin-bottom: 0.5em;
  font-weight: 600;
}

.bubble-markdown :deep(p) {
  margin: 0.5em 0;
}

.bubble-markdown :deep(table) {
  border-collapse: collapse;
  width: 100%;
  margin: 0.8em 0;
  font-size: calc(13px * var(--pet-scale));
}

.bubble-markdown :deep(th),
.bubble-markdown :deep(td) {
  border: 1px solid var(--border, #302418);
  padding: calc(6px * var(--pet-scale)) calc(10px * var(--pet-scale));
  text-align: left;
}

.bubble-markdown :deep(th) {
  background: rgba(110, 72, 40, 0.08);
  font-weight: 600;
}

.bubble-markdown :deep(tr:nth-child(even)) {
  background: rgba(0, 0, 0, 0.02);
}

.bubble-markdown :deep(code) {
  background: rgba(110, 72, 40, 0.1);
  border-radius: calc(3px * var(--pet-scale));
  padding: calc(1px * var(--pet-scale)) calc(5px * var(--pet-scale));
  font-family: 'JetBrains Mono', ui-monospace, monospace;
  font-size: calc(12px * var(--pet-scale));
}

.bubble-markdown :deep(pre) {
  background: rgba(0, 0, 0, 0.05);
  border-radius: calc(6px * var(--pet-scale));
  padding: calc(10px * var(--pet-scale)) calc(12px * var(--pet-scale));
  overflow-x: auto;
  margin: 0.8em 0;
}

.bubble-markdown :deep(pre code) {
  background: none;
  padding: 0;
  font-size: calc(12px * var(--pet-scale));
  line-height: 1.4;
}

.bubble-markdown :deep(blockquote) {
  border-left: calc(3px * var(--pet-scale)) solid var(--envelope-color, #6e4828);
  margin: 0.8em 0;
  padding-left: calc(12px * var(--pet-scale));
  color: var(--muted, #756050);
}

.bubble-markdown :deep(ul),
.bubble-markdown :deep(ol) {
  margin: 0.5em 0;
  padding-left: 1.5em;
}

.bubble-markdown :deep(li) {
  margin: 0.25em 0;
}

.bubble-markdown :deep(hr) {
  border: none;
  border-top: 1px solid var(--border, #302418);
  margin: 1em 0;
}

.bubble-markdown :deep(strong) {
  font-weight: 600;
  color: var(--bubble-text-color, #1a1a1e);
}
</style>
