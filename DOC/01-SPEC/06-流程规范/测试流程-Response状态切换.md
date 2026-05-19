# 测试流程：Response 状态切换

## 测试目标

验证切换 Response 状态时：
- 猫咪在屏幕上的位置（catY）保持不变
- 窗口尺寸能根据气泡内容自动调整

---

## 前置条件

- 应用已启动（`npm run tauri dev`）
- 可查看前端日志（浏览器 DevTools Console）

---

## 测试步骤

### 1. 观察初始状态

确认 idle 状态下：
- 窗口大小：360x360
- 猫咪位置已记录

### 2. 切换到 Response 状态

```bash
echo "Response" > /tmp/meo_anim_state.txt
```

验证：
- 窗口放大到正确尺寸（约为 480x650）
- 气泡完整显示
- 猫咪位置保持不变

### 3. 切换回 idle 状态

```bash
echo "idle" > /tmp/meo_anim_state.txt
```

验证：
- 窗口缩小回 360x360
- 猫咪位置恢复原位

### 4. 视觉对比

进入 Response 前和退出 Response 后，猫咪的屏幕位置应完全一致。

---

## 日志格式

### 前端日志

```
[WINDOW] enter_response: pos=(x,y) size=WxH catY=n
[WINDOW] enter_response: targetY=n newSize=WxH
[WINDOW] exit_response: pos=(x,y) size=WxH catY=n
[WINDOW] exit_response: targetY=n newSize=WxH
```

### catY 计算公式（物理像素）

```
catY = window_y + window_height - 360
```

### 成功标志

- `catY` 进入 Response 前和退出后保持一致
- 窗口尺寸根据气泡 CSS 自动计算

---

## 状态切换命令

```bash
# Response 状态
echo "Response" > /tmp/meo_anim_state.txt

# idle 状态
echo "idle" > /tmp/meo_anim_state.txt

# working 状态
echo "working" > /tmp/meo_anim_state.txt

# EnterReceiving 状态
echo "EnterReceiving" > /tmp/meo_anim_state.txt
```

---

## 调试技巧

### 查看前端日志

浏览器 DevTools Console 面板，筛选 `[WINDOW]` 日志。

### 相关代码

`DesktopPet.vue` 中的 `adjustWindowForResponse` 函数已包含 console.log 输出。

---

## 已知行为

1. **macOS 锚点行为** - macOS resize 时从当前位置扩展/收缩，前端通过先 setSize 再 setPosition 正确处理。

2. **Retina 缩放** - 前端日志中的尺寸是物理像素（CSS 值的 2 倍）。

3. **负数 Y 值** - 计算出的 targetY 为负数时，macOS 会 clamp 到可见区域，不影响猫咪定位的最终正确性。
