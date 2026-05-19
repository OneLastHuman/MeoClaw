import { describe, expect, it } from 'vitest';
import {
  computeSnapSide,
  getEdgeSnapThreshold,
  getBehaviorTargetX,
  getCompactWindowMetrics,
  getHiddenRevealPx,
  getPeekBufferPx,
  getPeekRevealPx,
  isCompactBehavior,
} from '../../windowBehavior';

describe('windowBehavior', () => {
  it('紧凑模式仅 normal 以外为 true', () => {
    expect(isCompactBehavior('normal')).toBe(false);
    expect(isCompactBehavior('dragging')).toBe(true);
    expect(isCompactBehavior('edgeHiddenLeft')).toBe(true);
  });

  it('计算紧凑窗口尺寸时使用猫本体大小', () => {
    expect(getCompactWindowMetrics(1)).toEqual({
      scale: 1,
      windowWidth: 180,
      windowHeight: 180,
    });

    expect(getCompactWindowMetrics(1.5)).toEqual({
      scale: 1.5,
      windowWidth: 270,
      windowHeight: 270,
    });
  });

  it('吸附阈值等于当前窗口宽度', () => {
    expect(getEdgeSnapThreshold(180)).toBe(180);
    expect(getEdgeSnapThreshold(270)).toBe(270);
  });

  it('半隐藏露出宽度为窗口宽度的65%', () => {
    expect(getHiddenRevealPx(180)).toBe(117);
    expect(getHiddenRevealPx(270)).toBe(176);
  });

  it('探出露出宽度与缓冲区按身位比例计算', () => {
    expect(getPeekRevealPx(180)).toBe(90);
    expect(getPeekRevealPx(270)).toBe(135);
    expect(getPeekBufferPx(180)).toBe(45);
    expect(getPeekBufferPx(270)).toBe(68);
  });

  it('鼠标靠近左边缘时吸附到左边', () => {
    expect(computeSnapSide(120, 180, { x: 0, width: 1440 })).toBe('left');
  });

  it('鼠标靠近右边缘时吸附到右边', () => {
    expect(computeSnapSide(1440 - 120, 180, { x: 0, width: 1440 })).toBe('right');
  });

  it('不在吸附区时不吸附', () => {
    expect(computeSnapSide(300, 180, { x: 0, width: 1440 })).toBeNull();
  });

  it('能计算左右隐藏和探出位置', () => {
    expect(
      getBehaviorTargetX('edgeHiddenLeft', { x: 0, width: 1440 }, 180, getHiddenRevealPx(180), getPeekRevealPx(180)),
    ).toBe(-63);

    expect(
      getBehaviorTargetX('edgeHiddenRight', { x: 0, width: 1440 }, 180, getHiddenRevealPx(180), getPeekRevealPx(180)),
    ).toBe(1323);

    expect(
      getBehaviorTargetX('edgePeekLeft', { x: 0, width: 1440 }, 180, getHiddenRevealPx(180), getPeekRevealPx(180)),
    ).toBe(-90);

    expect(
      getBehaviorTargetX('edgePeekRight', { x: 0, width: 1440 }, 180, getHiddenRevealPx(180), getPeekRevealPx(180)),
    ).toBe(1350);
  });
});
