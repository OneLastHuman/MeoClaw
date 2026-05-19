# 贴边动画重构 - edgeEnter 与 edgePeekLoop（2026-04-21）

## 本次实现范围

本次重构了贴边隐藏状态的动画系统，将原有的 peek 动画替换为新的 `edgeEnter` + `edgePeekLoop` 组合动画，并修复了拖动交互问题。

---

## 动画系统重构

### 旧方案问题
- 使用 `peek_left.png` / `peek_right.png` 作为 peek 状态动画
- 动画与 Canvas 动画重叠播放
- 从 peek/hover 状态无法拖动（`display: none` 阻止了交互）

### 新方案

#### 1. edgeEnter 动画（吸附进入）
- 素材：`edgeEnter.png`（3840×3840，4×4排列，13帧）
- 用途：拖动吸附成功后的入口动画
- 播放方式：
  - 右侧：第13帧 → 第1帧（倒序）
  - 左侧：第13帧 → 第1帧（逐帧镜像）
- 播放时间：~0.3秒（每帧23ms）

#### 2. edgePeekLoop 动画（hover 循环）
- 素材：`edgePeekLoop.png`（2880×1920，3×2排列，6帧）
- 用途：鼠标进入隐藏区域后的 hover 循环动画
- 播放方式：
  - 右侧：第1帧 → 第6帧（正序循环）
  - 左侧：第1帧 → 第6帧（逐帧镜像）
- 播放时间：1.5秒/循环（每帧250ms）

#### 3. 素材命名规范
| 功能 | 旧命名 | 新命名 |
|------|--------|--------|
| 吸附入口动画 | goingout.png | edgeEnter.png |
| hover 循环动画 | sayhi.png | edgePeekLoop.png |

---

## 行为流程

### 拖动吸附流程
```
用户拖动猫咪 → 松手在边缘吸附区
    ↓
瞬间移动到贴边位置（65% 露出）
    ↓
播放 edgeEnter 倒序动画（阻塞交互）
    ↓
停在第1帧
```

### Peek 入口流程（鼠标进入隐藏区）
```
edgeHidden → 鼠标进入热区
    ↓
切换到 edgePeek 状态
    ↓
播放 edgeEnter 正序动画（0→12帧，~0.3秒）
    ↓
播放 edgePeekLoop 循环动画（1.5秒/循环）
    ↓
鼠标离开 → 取消 loop → 播放 edgeEnter 倒序（12→0帧）→ 回到 hide 静态
```

### 拖动交互
- 贴边任何状态（hidden / edgePeek / edgePeekLoop）都可以随时拖动
- 拖动时取消所有动画，直接进入 dragging 状态

---

## 实现细节

### 变量/函数命名
所有相关变量和函数已重命名以符合规范：

| 旧命名 | 新命名 |
|--------|--------|
| `goingoutImage` | `edgeEnterImage` |
| `sayhiImage` | `edgePeekLoopImage` |
| `goingoutFrame` | `edgeEnterFrame` |
| `goingoutDirection` | `edgeEnterDirection` |
| `goingoutMode` | `edgeEnterMode` |
| `goingoutAnimationTimer` | `edgeEnterAnimationTimer` |
| `isGoingoutPlaying` | `isEdgeEnterPlaying` |
| `sayhiFrame` | `edgePeekLoopFrame` |
| `sayhiDirection` | `edgePeekLoopDirection` |
| `sayhiAnimationTimer` | `edgePeekLoopTimer` |
| `isSayhiPlaying` | `isEdgePeekLoopPlaying` |
| `goingoutCanvasRef` | `edgeEnterCanvasRef` |

### Canvas 渲染
- `edgeEnter` 和 `edgePeekLoop` 共用同一个 Canvas 元素
- 左侧动画通过 `ctx.scale(-1, 1)` 实现逐帧镜像（非整图镜像）

### CSS 修复
- `is-goingout-hidden` → `is-edge-enter-hidden`
- `display: none` → `opacity: 0`（保持精灵图可交互，用于拖动检测）

### 交互阻塞
- `snap` 模式（吸附进入）：阻塞所有交互直到动画播完
- `peek` 模式（hover 进入/离开）：不阻塞交互

---

## 涉及文件

- `src/components/DesktopPet.vue` - 主要逻辑修改
- `src/config/__tests__/animations.test.ts` - 测试用例修复
- `public/anim/edge-placeholders/edgeEnter.png` - 新增素材
- `public/anim/edge-placeholders/edgePeekLoop.png` - 新增素材

---

## 验证方式

### 自动验证
```bash
npm test
npm run build
```

### 手动验证
```bash
npm run tauri dev
```

1. 拖动猫咪到左右边缘松手，验证吸附动画
2. 鼠标进入隐藏区，验证 edgeEnter + edgePeekLoop 动画
3. 在 peek/hover 状态时长按拖动，验证可以正常拖走

---

## 下一步建议

1. 整理 edge placeholders 目录，移除不再使用的旧素材（`peek_left.png`, `peek_right.png`）
2. 考虑将 hide 静态图也纳入 Canvas 渲染体系，统一管理
3. 添加自动化视觉测试验证动画帧序
