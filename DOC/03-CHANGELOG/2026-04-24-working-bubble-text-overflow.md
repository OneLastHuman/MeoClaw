# Working 气泡文字省略

## 日期

2026-04-24

## 问题

Working 状态下右上角工具提示气泡（`.bubble-overlay`）显示工具调用信息时，如果文字过长会超出气泡边界被截断。

## 解决方案

修改 `DesktopPet.vue` 中 CSS 样式：

- `.bubble-overlay` 添加 `overflow: hidden`
- `.bubble-text` 添加 `max-width: 100%`、`text-overflow: ellipsis`、`overflow: hidden`

## 验证

- `workingBubbleLeft = 100 * scale`，`workingBubbleWidth = 220 * scale`
- 气泡右边缘位置 = 320 * scale，不超出窗口宽度 360 * scale
- 动态缩放时 `max-width: 100%` 自动跟随父元素宽度

## 涉及文件

- `src/components/DesktopPet.vue` — CSS 样式修改
