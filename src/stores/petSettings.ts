import { computed, ref, watch } from 'vue';
import type { AnimState } from '../config/animations';

export const PET_SCALE_STORAGE_KEY = 'meoclaw.petScale';
export const PET_SCALE_MIN = 0.5;
export const PET_SCALE_MAX = 2;
export const PET_SCALE_STEP = 0.1;
export const PET_SCALE_DEFAULT = 1;

// Response 音效配置
export const RESPONSE_SOUND_STORAGE_KEY = 'meoclaw.responseSound';
export const RESPONSE_SOUND_OFF = 'off';
export const RESPONSE_SOUND_DEFAULT = 'chime.mp3';

export const RESPONSE_SOUNDS = [
  { id: 'off', label: 'Off', file: null },
  { id: 'chime.mp3', label: 'Chime', file: '/sound/response/chime.mp3' },
  { id: 'soft.mp3', label: 'Soft', file: '/sound/response/soft.mp3' },
  { id: 'notification.ogg', label: 'Notification', file: '/sound/response/notification.ogg' },
  { id: 'classic.wav', label: 'Classic', file: '/sound/response/classic.wav' },
  { id: 'crisp.wav', label: 'Crisp', file: '/sound/response/crisp.wav' },
] as const;

export type ResponseSoundId = typeof RESPONSE_SOUNDS[number]['id'];

export function loadResponseSound(): ResponseSoundId {
  if (typeof window === 'undefined') {
    return RESPONSE_SOUND_DEFAULT;
  }

  const stored = window.localStorage.getItem(RESPONSE_SOUND_STORAGE_KEY);
  if (!stored) {
    return RESPONSE_SOUND_DEFAULT;
  }

  // 验证是否是有效的音效 ID
  const validIds = RESPONSE_SOUNDS.map((s) => s.id);
  if (validIds.includes(stored as ResponseSoundId)) {
    return stored as ResponseSoundId;
  }

  return RESPONSE_SOUND_DEFAULT;
}

export function persistResponseSound(soundId: ResponseSoundId) {
  if (typeof window === 'undefined') {
    return;
  }

  window.localStorage.setItem(RESPONSE_SOUND_STORAGE_KEY, soundId);
}

export const responseSound = ref<ResponseSoundId>(loadResponseSound());

watch(
  responseSound,
  (soundId) => {
    persistResponseSound(soundId);
  },
  { flush: 'post' },
);

export function setResponseSound(soundId: ResponseSoundId) {
  responseSound.value = soundId;
}

export function getResponseSoundFile(soundId: ResponseSoundId): string | null {
  const sound = RESPONSE_SOUNDS.find((s) => s.id === soundId);
  return sound?.file ?? null;
}

// 音量配置
export const VOLUME_STORAGE_KEY = 'meoclaw.volume';
export const VOLUME_MIN = 0;
export const VOLUME_MAX = 1;
export const VOLUME_STEP = 0.1;
export const VOLUME_DEFAULT = 0.5;

export function loadVolume(): number {
  if (typeof window === 'undefined') {
    return VOLUME_DEFAULT;
  }

  const stored = window.localStorage.getItem(VOLUME_STORAGE_KEY);
  if (stored === null) {
    return VOLUME_DEFAULT;
  }

  const parsed = Number(stored);
  if (!Number.isFinite(parsed)) {
    return VOLUME_DEFAULT;
  }

  return Math.min(VOLUME_MAX, Math.max(VOLUME_MIN, parsed));
}

export function persistVolume(volume: number) {
  if (typeof window === 'undefined') return;

  window.localStorage.setItem(VOLUME_STORAGE_KEY, String(volume));
}

export const responseVolume = ref<number>(loadVolume());

watch(
  responseVolume,
  (volume) => {
    persistVolume(volume);
  },
  { flush: 'post' },
);

export function setResponseVolume(volume: number) {
  responseVolume.value = Math.min(VOLUME_MAX, Math.max(VOLUME_MIN, volume));
}

export type WindowMode = 'normal' | 'working' | 'response';

export const BASE_LAYOUT = {
  catWidth: 180,
  catHeight: 180,
  inputBottom: 35,
  inputWidth: 160,
  inputHeight: 44,
  inputFontSize: 18,
  inputStep: 3,
  fileDropTop: 70,
  fileDropGap: 8,
  fileIconSize: 40,
  fileNameFontSize: 10,
  fileNameMaxWidth: 56,
  closeButtonSize: 16,
  closeButtonOffset: -4,
  workingBubbleLeft: 100,
  workingBubbleTop: 30,
  workingBubbleWidth: 220,
  workingBubbleMinHeight: 32,
  workingBubbleFontSize: 13,
  queueBubbleTop: 150,
  responseBubbleLeft: 120,
  responseBubbleBottom: 150,
  responseBubbleWidth: 360,
  responseBubbleHeight: 500,
  responseBubbleFontSize: 14,
} as const;

const BASE_WINDOW_SIZES: Record<WindowMode, { width: number; height: number }> = {
  normal: {
    width: BASE_LAYOUT.catWidth,
    height: BASE_LAYOUT.catHeight,
  },
  working: {
    width: BASE_LAYOUT.workingBubbleLeft + BASE_LAYOUT.workingBubbleWidth,
    height: BASE_LAYOUT.catHeight + BASE_LAYOUT.workingBubbleTop + BASE_LAYOUT.workingBubbleMinHeight,
  },
  response: {
    width: BASE_LAYOUT.responseBubbleLeft + BASE_LAYOUT.responseBubbleWidth,
    height: BASE_LAYOUT.responseBubbleBottom + BASE_LAYOUT.responseBubbleHeight,
  },
};

export function normalizePetScale(raw: unknown): number {
  const parsed = typeof raw === 'number' ? raw : Number(raw);
  if (!Number.isFinite(parsed)) {
    return PET_SCALE_DEFAULT;
  }

  const clamped = Math.min(PET_SCALE_MAX, Math.max(PET_SCALE_MIN, parsed));
  return Math.round(clamped / PET_SCALE_STEP) * PET_SCALE_STEP;
}

export function loadPetScale(): number {
  if (typeof window === 'undefined') {
    return PET_SCALE_DEFAULT;
  }

  return normalizePetScale(window.localStorage.getItem(PET_SCALE_STORAGE_KEY));
}

export function persistPetScale(scale: number) {
  if (typeof window === 'undefined') {
    return;
  }

  window.localStorage.setItem(PET_SCALE_STORAGE_KEY, String(normalizePetScale(scale)));
}

export const petScale = ref(loadPetScale());

watch(
  petScale,
  (scale) => {
    persistPetScale(scale);
  },
  { flush: 'post' },
);

export function setPetScale(scale: number) {
  petScale.value = normalizePetScale(scale);
}

export function windowModeForState(state: AnimState): WindowMode {
  if (state === 'Response') {
    return 'response';
  }

  if (state === 'working' || state === 'workingPreview') {
    return 'working';
  }

  return 'normal';
}

export function getWindowMetrics(mode: WindowMode, scale: number) {
  const normalizedScale = normalizePetScale(scale);
  const baseWindow = BASE_WINDOW_SIZES[mode];

  return {
    scale: normalizedScale,
    windowWidth: baseWindow.width * normalizedScale,
    windowHeight: baseWindow.height * normalizedScale,
  };
}

export const scaledCSSVars = computed(() => {
  const scale = petScale.value;

  return {
    scale,
    catWidth: BASE_LAYOUT.catWidth * scale,
    catHeight: BASE_LAYOUT.catHeight * scale,
    inputBottom: BASE_LAYOUT.inputBottom * scale,
    inputWidth: BASE_LAYOUT.inputWidth * scale,
    inputHeight: BASE_LAYOUT.inputHeight * scale,
    inputFontSize: BASE_LAYOUT.inputFontSize * scale,
    inputStep: BASE_LAYOUT.inputStep * scale,
    fileDropTop: BASE_LAYOUT.fileDropTop * scale,
    fileDropGap: BASE_LAYOUT.fileDropGap * scale,
    fileIconSize: BASE_LAYOUT.fileIconSize * scale,
    fileNameFontSize: BASE_LAYOUT.fileNameFontSize * scale,
    fileNameMaxWidth: BASE_LAYOUT.fileNameMaxWidth * scale,
    closeButtonSize: BASE_LAYOUT.closeButtonSize * scale,
    closeButtonOffset: BASE_LAYOUT.closeButtonOffset * scale,
    workingBubbleLeft: BASE_LAYOUT.workingBubbleLeft * scale,
    workingBubbleTop: BASE_LAYOUT.workingBubbleTop * scale,
    workingBubbleWidth: BASE_LAYOUT.workingBubbleWidth * scale,
    workingBubbleMinHeight: BASE_LAYOUT.workingBubbleMinHeight * scale,
    workingBubbleFontSize: BASE_LAYOUT.workingBubbleFontSize * scale,
    queueBubbleTop: BASE_LAYOUT.queueBubbleTop * scale,
    responseBubbleLeft: BASE_LAYOUT.responseBubbleLeft * scale,
    responseBubbleBottom: BASE_LAYOUT.responseBubbleBottom * scale,
    responseBubbleWidth: BASE_LAYOUT.responseBubbleWidth * scale,
    responseBubbleHeight: BASE_LAYOUT.responseBubbleHeight * scale,
    responseBubbleFontSize: BASE_LAYOUT.responseBubbleFontSize * scale,
  };
});
