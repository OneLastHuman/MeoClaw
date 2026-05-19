import { BASE_LAYOUT } from './stores/petSettings';

export type WindowBehaviorState =
  | 'normal'
  | 'dragging'
  | 'edgeHiddenLeft'
  | 'edgeHiddenRight'
  | 'edgePeekLeft'
  | 'edgePeekRight';

export type ActivityLevel = 'none' | 'working' | 'response' | 'error';
export type SnapSide = 'left' | 'right' | null;

export interface MonitorBounds {
  x: number;
  width: number;
}

export const EDGE_PEEK_LEAVE_DELAY = 0;
export const EDGE_HIDDEN_REVEAL_RATIO = 0.65;
export const EDGE_PEEK_REVEAL_RATIO = 0.5;
export const EDGE_PEEK_BUFFER_RATIO = 0.25;

export function isCompactBehavior(state: WindowBehaviorState): boolean {
  return state !== 'normal';
}

export function isHiddenBehavior(state: WindowBehaviorState): boolean {
  return state === 'edgeHiddenLeft' || state === 'edgeHiddenRight';
}

export function isPeekBehavior(state: WindowBehaviorState): boolean {
  return state === 'edgePeekLeft' || state === 'edgePeekRight';
}

export function getEdgeSnapThreshold(windowWidth: number) {
  return Math.max(0, Math.round(windowWidth));
}

export function getHiddenRevealPx(windowWidth: number) {
  return Math.max(0, Math.round(windowWidth * EDGE_HIDDEN_REVEAL_RATIO));
}

export function getPeekRevealPx(windowWidth: number) {
  return Math.max(0, Math.round(windowWidth * EDGE_PEEK_REVEAL_RATIO));
}

export function getPeekBufferPx(windowWidth: number) {
  return Math.max(0, Math.round(windowWidth * EDGE_PEEK_BUFFER_RATIO));
}

export function getCompactWindowMetrics(scale: number) {
  return {
    scale,
    windowWidth: BASE_LAYOUT.catWidth * scale,
    windowHeight: BASE_LAYOUT.catHeight * scale,
  };
}

export function computeSnapSide(
  cursorX: number,
  windowWidth: number,
  monitor: MonitorBounds,
  thresholdPx = getEdgeSnapThreshold(windowWidth),
): SnapSide {
  const leftGap = Math.abs(cursorX - monitor.x);
  const rightGap = Math.abs(monitor.x + monitor.width - cursorX);

  if (leftGap <= thresholdPx && leftGap <= rightGap) {
    return 'left';
  }

  if (rightGap <= thresholdPx) {
    return 'right';
  }

  return null;
}

export function getBehaviorTargetX(
  state: WindowBehaviorState,
  monitor: MonitorBounds,
  windowWidth: number,
  hiddenRevealPx: number,
  peekRevealPx: number,
): number | null {
  switch (state) {
    case 'edgeHiddenLeft':
      return monitor.x - windowWidth + hiddenRevealPx;
    case 'edgeHiddenRight':
      return monitor.x + monitor.width - hiddenRevealPx;
    case 'edgePeekLeft':
      return monitor.x - windowWidth + peekRevealPx;
    case 'edgePeekRight':
      return monitor.x + monitor.width - peekRevealPx;
    default:
      return null;
  }
}
