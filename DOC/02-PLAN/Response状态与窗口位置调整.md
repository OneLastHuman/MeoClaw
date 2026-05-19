# Response 状态与窗口位置调整

> 更新（2026-04-17）：
> 本文档中的实现细节基于旧版本 `DesktopPet.vue`，包含固定 `2x Retina`、独立 `adjustWindowForResponse()`、以及旧常量布局。
> 当前实现已经改为基于 `petSettings.ts` 的统一窗口模式计算，并与缩放功能合并处理。
>
> 当前真实实现请优先参考：
> `DOC/功能/桌宠大小调整功能实现-2026-04-17.md`

## 功能概述

Response 是桌宠的一个扩展状态，此时窗口会放大以容纳气泡内容。气泡作为 CSS 层在窗口内渲染。

**核心特点：**
- 窗口尺寸由前端根据气泡 CSS 自动计算，无需硬编码
- 切换状态时猫咪在屏幕上的 Y 坐标保持不变
- 全部逻辑由前端 JavaScript 处理，无需 Rust 参与窗口尺寸调整

## 坐标系说明

### 物理像素 vs 逻辑像素

- macOS Retina 屏幕使用 2x 缩放
- Tauri 的 `PhysicalPosition` / `PhysicalSize` 返回物理像素（实际像素值）
- `outer_position()` 和 `outer_size()` 返回的是物理像素
- CSS 和配置文件中的尺寸是逻辑像素，实际渲染时会乘以 2

### 窗口位置与猫咪位置

```
屏幕坐标系统
─────────────────────────→ X
│
│    ┌─────────────────┐
│    │                 │
│    │    窗口         │
│    │                 │
│    │  ┌───────────┐  │
│    │  │  气泡      │  │
│    │  └───────────┘  │
│    │                 │
│    └─────[猫咪]──────┘
│              ↑
│         bottom: 0
│
↓ Y

猫咪顶部 Y = window_y + window_height - CAT_HEIGHT
```

## 窗口尺寸计算

### 原理

窗口尺寸由前端 JavaScript 根据气泡 CSS 属性自动计算。

### 计算公式

```
窗口最小宽度 = bubble_left + bubble_width
窗口最小高度 = bubble_height + bubble_bottom
```

### 当前配置（与 CSS 保持一致）

| 常量 | 值 | 说明 |
|------|-----|------|
| `BUBBLE_LEFT` | 120 | 气泡左边距离窗口左边 |
| `BUBBLE_BOTTOM` | 150 | 气泡底部距离窗口底部 |
| `BUBBLE_WIDTH` | 360 | 气泡宽度 |
| `BUBBLE_HEIGHT` | 500 | 气泡高度 |
| `CAT_HEIGHT` | 180 | 猫咪高度 |

### 计算结果

```
RESPONSE_WIDTH = BUBBLE_LEFT + BUBBLE_WIDTH = 120 + 360 = 480px
RESPONSE_HEIGHT = BUBBLE_HEIGHT + BUBBLE_BOTTOM = 500 + 150 = 650px

普通窗口：180x180
Response 窗口：480x650
```

## 前端实现

### 配置文件位置

`src/components/DesktopPet.vue` 中的常量定义：

```typescript
// ============== Response 气泡尺寸配置（与 CSS 保持一致） ==============
const BUBBLE_LEFT = 120;
const BUBBLE_BOTTOM = 150;
const BUBBLE_WIDTH = 360;
const BUBBLE_HEIGHT = 500;
const CAT_HEIGHT = 180;

// 根据气泡尺寸计算窗口刚好匹配内容的尺寸
const RESPONSE_WIDTH = BUBBLE_LEFT + BUBBLE_WIDTH;  // 480
const RESPONSE_HEIGHT = BUBBLE_HEIGHT + BUBBLE_BOTTOM;  // 650

// 普通窗口尺寸
const NORMAL_WIDTH = 180;
const NORMAL_HEIGHT = 180;
```

### 窗口调整函数

```typescript
async function adjustWindowForResponse(enter: boolean) {
  const win = getCurrentWindow();
  const pos = await win.outerPosition();
  const size = await win.outerSize();

  // 计算当前猫咪顶部的 Y（物理像素）
  const catY = pos.y + size.height - CAT_HEIGHT * 2;

  if (enter) {
    // 进入 Response：放大窗口，保持猫咪位置
    const newHeight = RESPONSE_HEIGHT * 2;
    const newWidth = RESPONSE_WIDTH * 2;

    // 目标猫咪Y不变
    // catY = new_y + new_height - CAT_HEIGHT*2
    // new_y = catY - new_height + CAT_HEIGHT*2
    const targetY = catY - newHeight + CAT_HEIGHT * 2;

    console.log(`[WINDOW] enter_response: pos=(${pos.x},${pos.y}) size=${size.width}x${size.height} catY=${catY}`);
    console.log(`[WINDOW] enter_response: targetY=${targetY} newSize=${newWidth}x${newHeight}`);

    await win.setSize(new PhysicalSize(newWidth, newHeight));
    await win.setPosition(new PhysicalPosition(pos.x, targetY));
  } else {
    // 退出 Response：缩小窗口，保持猫咪位置
    const newHeight = NORMAL_HEIGHT * 2;
    const newWidth = NORMAL_WIDTH * 2;

    const targetY = catY - newHeight + CAT_HEIGHT * 2;

    console.log(`[WINDOW] exit_response: pos=(${pos.x},${pos.y}) size=${size.width}x${size.height} catY=${catY}`);
    console.log(`[WINDOW] exit_response: targetY=${targetY} newSize=${newWidth}x${newWidth}`);

    await win.setSize(new PhysicalSize(newWidth, newHeight));
    await win.setPosition(new PhysicalPosition(pos.x, targetY));
  }
}
```

### 状态监听

```typescript
watch(() => animState.value, async (state) => {
  if (state === 'Response') {
    try {
      await adjustWindowForResponse(true);
    } catch (err) {
      console.error('[WINDOW] adjustWindow failed:', err);
    }
  } else {
    try {
      await adjustWindowForResponse(false);
    } catch (err) {
      console.error('[WINDOW] adjustWindow failed:', err);
    }
  }
});
```

## CSS 气泡定位

### 气泡容器样式

```css
.response-bubble-container {
  position: absolute;
  /* 气泡在窗口内左对齐，底部与窗口底部保持距离 */
  left: 120px;      /* 与 BUBBLE_LEFT 一致 */
  bottom: 150px;     /* 与 BUBBLE_BOTTOM 一致 */
  width: 360px;      /* 与 BUBBLE_WIDTH 一致 */
  height: 500px;      /* 与 BUBBLE_HEIGHT 一致 */
  pointer-events: none;
}

.response-bubble {
  width: 100%;
  height: 100%;
  background: rgba(255, 255, 255, 0.95);
  border-radius: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 16px;
  color: #333;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
}
```

### 定位关系图

```
窗口 (480x650)
┌─────────────────────────────────┐
│                                 │
│                                 │  ← top
│   ┌─────────────────────────┐   │
│   │                         │   │
│   │      气泡 (360x500)      │   │
│   │                         │   │
│   │                         │   │
│   └─────────────────────────┘   │   ↑ bottom: 150
│                                 │
│   ┌───────────┐                │
│   │   猫咪     │                │  ← 猫咪在底部
│   │  (180x180) │                │
│   └───────────┘                │
└─────────────────────────────────┘
     ↑ left: 120
```

## 窗口位置保持算法

### 目标

无论窗口尺寸如何变化，猫咪在屏幕上的 Y 坐标保持不变。

### 公式推导

**猫咪顶部 Y 坐标计算（物理像素）：**
```
catY = window_y + window_height - CAT_HEIGHT * 2
```

**进入 Response 时保持 catY 不变：**
```
catY = new_y + new_height - CAT_HEIGHT * 2
new_y = catY - new_height + CAT_HEIGHT * 2
```

**退出 Response 时保持 catY 不变（同样公式）：**
```
new_y = catY - new_height + CAT_HEIGHT * 2
```

## 适配气泡尺寸变化

如果修改了气泡 CSS，只需要同步修改 `DesktopPet.vue` 中的对应常量即可：

```typescript
// 假设修改 CSS 为：
// left: 100px; bottom: 200px; width: 400px; height: 600px;

// 则修改常量为：
const BUBBLE_LEFT = 100;
const BUBBLE_BOTTOM = 200;
const BUBBLE_WIDTH = 400;
const BUBBLE_HEIGHT = 600;

// 计算结果自动更新：
// RESPONSE_WIDTH = 100 + 400 = 500
// RESPONSE_HEIGHT = 600 + 200 = 800
```

**无需修改任何其他代码！**

## 日志追踪

### 前端日志

```bash
# 在浏览器控制台或终端可以看到：
[WINDOW] enter_response: pos=(1000,500) size=360x360 catY=680
[WINDOW] enter_response: targetY=-620 newSize=960x1300
[WINDOW] exit_response: pos=(1000,-620) size=960x1300 catY=680
[WINDOW] exit_response: targetY=500 newSize=360x360
```

### Rust 日志

如果 Rust 端仍有调试日志：
```bash
tail -f ~/Library/Logs/com.meoclaw.app/MeoClaw.log
```

## 验证方法

1. 启动应用，确认猫咪在正常状态下的位置
2. 切换到 Response 状态：
   ```bash
   echo "Response" > /tmp/meo_anim_state.txt
   ```
   - 窗口应该放大，气泡应该完整显示
   - 猫咪位置应该不变

3. 切换回 idle 状态：
   ```bash
   echo "idle" > /tmp/meo_anim_state.txt
   ```
   - 窗口应该缩小回原大小
   - 猫咪位置应该恢复到原位置

4. 多次切换，确认 catY 保持一致

## 滚动条样式

气泡内容区 `.response-bubble` 默认会显示系统滚动条。如需隐藏滚动条但保留滚动功能（鼠标滚轮/触控板仍可滚动），可添加以下 CSS：

```css
.response-bubble {
  /* 隐藏滚动条但保留滚动功能 */
  scrollbar-width: none; /* Firefox */
  -ms-overflow-style: none; /* IE/Edge */
}

.response-bubble::-webkit-scrollbar {
  display: none; /* Chrome/Safari/Opera */
}
```

## 关键文件

| 文件 | 说明 |
|------|------|
| `src/components/DesktopPet.vue` | 前端窗口调整逻辑、气泡 CSS |
| `src/config/animations.ts` | 动画配置 |
| `src-tauri/src/lib.rs` | Rust 端命令（仅保留事件通知） |
