# 精灵动画系统

## 概述

桌宠使用 CSS sprite（精灵图）方式实现动画。通过定时切换背景图片的位置，显示精灵图中的某一帧，从而形成动画效果。

## 核心原理

### CSS 单位一致性

精灵动画的关键是 `background-size` 和 `background-position` 必须使用相同的 CSS 单位。精灵图的实际像素尺寸与显示尺寸按同一比例缩放。

```
窗口尺寸：180×180px
每帧在 CSS 中的大小 = 180px
精灵图 4×4 排列 → background-size = 720×720px
```

精灵图按帧索引切分：
```typescript
const col = frameIndex % cols;
const row = Math.floor(frameIndex / cols);
const bgX = col * CSS_FRAME_SIZE;
const bgY = row * CSS_FRAME_SIZE;
```

### 镜像支持

`edgeHiddenLeft` 状态需要对精灵图进行水平镜像翻转：
```typescript
transform: `translateZ(0)${isMirrored ? ' scaleX(-1)' : ''}`,
```

## 动画配置

所有动画配置集中定义在 `src/config/animations.ts` 的 `ANIMATIONS` 对象中。

```typescript
export interface AnimationConfig {
  name: string;           // 动画名称
  cols: number;           // 精灵图列数
  rows: number;           // 精灵图行数
  totalFrames: number;    // 总帧数
  interval: number;       // 每帧间隔(ms)
  loop: boolean;          // 是否循环播放
  pingpong?: boolean;     // 是否 ping-pong 来回播放（默认 false）
  startFrame: number;     // 起始帧索引
  imagePath: string;      // 精灵图路径
  nextState?: AnimState;  // 非循环动画播放完成后的目标状态
}
```

## 动画状态

```typescript
export type AnimState =
  | 'idle'
  | 'EnterInput'
  | 'shock'
  | 'startworking'
  | 'working'
  | 'workingPreview'
  | 'EnterReceiving'
  | 'Receiving'
  | 'received'
  | 'Response';
```

### 动画清单

| 状态 | 精灵图 | 排列 | 帧范围 | 循环方式 | 间隔 |
|------|--------|------|--------|----------|------|
| `idle` | `ildesanimation.png` | 4×4 | 0~15 | ping-pong 往返 | 100ms |
| `EnterInput` | `talkmode.png` | 3×1 | 0~2 | 单向循环 | 667ms |
| `shock` | `shock.png` | 3×4 | 0~9 | 单次播放 | 150ms |
| `startworking` | `startworking.png` | 7×7 | 0~13 | 单次播放 → `working` | 100ms |
| `working` | `startworking.png` | 7×7 | 13~47 | 正向循环 | 100ms |
| `workingPreview` | `startworking.png` | 7×7 | 13~47 | 正向循环 | 100ms |
| `EnterReceiving` | `receiving.png` | 4×3 | 0~5 | 单次播放 → `Receiving` | 100ms |
| `Receiving` | `receiving.png` | 4×3 | 6~11 | 循环 | 100ms |
| `received` | `received.png` | 1×1 | 0 | 单帧静态 | — |
| `Response` | `ildesanimation.png` | 4×4 | 0~15 | ping-pong 往返 | 100ms |

### ping-pong 往返逻辑

ping-pong 模式在到达终点帧后反向播放，到达起始帧后再正向播放：

```typescript
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
```

### 非循环动画自动切换

`loop: false` 的动画播放完一次后，通过 `nextState` 自动切换到目标状态：

```typescript
if (nextRelative === 0) {
  stopAnimation();
  if (anim.nextState) {
    switchAnim(anim.nextState);
  }
  return;
}
```

`shock` 动画有特殊处理：播放完成后恢复进入 `shock` 之前的状态（`savedAnimState`）。

## 帧位置计算

```typescript
function getFramePosition(frame: number, columns: number) {
  const column = frame % columns;
  const row = Math.floor(frame / columns);
  return {
    bgX: column * CSS_FRAME_SIZE,
    bgY: row * CSS_FRAME_SIZE,
  };
}
```

## 窗口行为动画

窗口行为（贴边、peek 等）使用专用的占位符精灵图，位于 `src/config/animations.ts` 的 `WINDOW_BEHAVIOR_ANIMATIONS` 对象中：

| 状态 | 精灵图 | 说明 |
|------|--------|------|
| `dragging` | `edge-placeholders/dragging.png` | 拖拽中，2帧循环 |
| `edgeHiddenLeft` | `edge-placeholders/hide_right.png` | 贴住左边缘（镜像显示） |
| `edgeHiddenRight` | `edge-placeholders/hide_right.png` | 贴住右边缘 |
| `edgePeekLeft` | `edge-placeholders/hide_right.png` | 左边缘 peek（镜像显示） |
| `edgePeekRight` | `edge-placeholders/hide_right.png` | 右边缘 peek |

`edgeHiddenLeft` 和 `edgePeekLeft` 在渲染时会进行 `scaleX(-1)` 水平镜像，使右边缘占位符显示为左边缘效果。

## 关键文件

| 文件路径 | 说明 |
|----------|------|
| `src/config/animations.ts` | 动画配置定义 |
| `src/components/DesktopPet.vue` | 动画播放逻辑、帧计算、状态切换 |
| `public/anim/*.png` | 精灵图资源 |
