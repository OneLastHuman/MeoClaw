# 双击进入 Response（带输入框）复用上次回复气泡

## 目标

用户在输入新问题时，能看到上一次 AI 回复的气泡作为参考。

## 设计思路

`Response` 状态本身已经同时包含响应气泡和输入框（`showInput` 在 `Response` 下为 `true`）。所以不需要新增气泡逻辑，只需在双击时判断：

- 有上次回复 → 进入 `Response` 状态（气泡 + 输入框都有了）
- 无上次回复 → 进入 `EnterInput` 状态（纯输入框）

## 变更

仅 `src/components/DesktopPet.vue` 双击逻辑一处：

```typescript
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
```

## 不变

- `showResponseBubble`、`animState` watcher、窗口自适应：全部不变
- 位置、样式、大小：完全复用 `Response` 状态自带的 CSS
- 消息存储：不引入历史记录

## 边界情况

| 场景 | 行为 |
|---|---|
| 首次启动，无历史回复 | 双击 → `EnterInput`（无气泡） |
| 有上次回复，双击 | 双击 → `Response`（气泡 + 输入框） |
| Response 中双击 | 切回 `idle`（原有 toggle 行为） |
