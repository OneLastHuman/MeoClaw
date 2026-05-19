# Menu Bar 菜单栏重启功能

## 需求
- 在 Menu Bar 图标的右键菜单中新增"重启"选项
- 点击"重启"后重启整个应用程序

---

## 实现

### Menu Bar Tray Icon 代码 (`src-tauri/src/lib.rs`)

#### 1. 创建菜单项

```rust
// === Menu Bar Tray Icon ===
let quit_item = MenuItemBuilder::with_id("quit", "退出").build(app)?;
let restart_item = MenuItemBuilder::with_id("restart", "重启").build(app)?;
let options_item = MenuItemBuilder::with_id("options", "选项").build(app)?;

let menu = MenuBuilder::new(app)
    .item(&options_item)
    .separator()
    .item(&restart_item)
    .item(&quit_item)
    .build()?;
```

#### 2. 处理菜单事件

```rust
.on_menu_event(move |app, event| {
    match event.id().as_ref() {
        "quit" => {
            log::info!("[Tray] Quit clicked");
            app.exit(0);
        }
        "restart" => {
            log::info!("[Tray] Restart clicked");
            let app_handle = app.clone();
            std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_millis(100));
                app_handle.restart();
            });
        }
        "options" => {
            log::info!("[Tray] Options clicked");
            if let Err(err) = show_options_window(app) {
                log::error!("[Tray] Failed to open options window: {}", err);
            }
        }
        _ => {}
    }
})
```

---

## 技术细节

### 重启方式
使用 `AppHandle.restart()` 在新线程中异步调用，避免阻塞菜单事件回调。

### 选项窗口现状
`options` 入口已经启用，不再是占位菜单。

- 窗口页面：`options.html`
- 前端逻辑：`src/options.ts`
- Rust 创建入口：`show_options_window(app)`

### dev vs release 模式
- **dev 模式**：`current_exe()` 返回 `cargo`，可能导致重启后窗口不显示
- **release 模式**：正常工作，重启后应用完整恢复

### 注意事项
- `app.restart()` 是异步的，需要在独立线程中调用
- 重启前添加短暂延迟（100ms）确保进程清理完成

---

## 相关文件

```
src-tauri/src/lib.rs    # Menu Bar Tray Icon 实现
```

---

## Menu Bar 菜单结构

```
[图标]
├── 选项
├── ─────────
├── 重启        ← 新增
└── 退出
```
