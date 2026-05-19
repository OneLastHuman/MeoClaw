# 选项窗口无边框化

## 变更内容

### Rust 端 (`src-tauri/src/lib.rs`)
- 选项窗口添加 `.decorations(false)` — 移除系统标题栏和边框
- 添加 `.transparent(true)` — 窗口背景透明，支持 CSS 圆角显示

### 前端 (`src/options.ts`)
- 移除 title bar 中 3 个假信号灯（traffic light）占位元素
- 替换为功能性红色圆形关闭按钮（`&times;`），绑定 `getCurrentWindow().close()`
- title bar 添加 `data-tauri-drag-region` 支持窗口拖拽
- 布局适配无边框窗口：
  - `body` 背景改为透明，渐变背景移至 `.options-shell`
  - `.options-shell` 承载 `border-radius: 12px` 和 `border: 3px`
  - 移除外层 `padding: 20px`，窗口内容填满整个区域
  - `.options-body` 改为 `flex: 1` 弹性高度替代固定 `440px`
- 标题文字居中显示（`margin: 0 auto`）

## 设计说明

无边框设计去掉了系统原生窗口装饰，保持 retro 像素风格的一致性。关闭按钮使用红色圆形 + `&times;`，与整体像素复古主题匹配。用户可通过 title bar 空白区域拖拽窗口。
