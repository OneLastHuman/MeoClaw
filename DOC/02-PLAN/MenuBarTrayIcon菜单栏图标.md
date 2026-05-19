# Menu Bar Tray Icon（菜单栏图标）

## 功能说明

在 Mac 屏幕顶部的菜单栏添加一个小图标，提供快捷操作。

## 交互逻辑

| 操作 | 功能 |
|------|------|
| 左键单击 | 显示/隐藏桌宠主窗口 |
| 右键单击 | 显示右键菜单 |

## 右键菜单

- **选项** - 打开独立设置窗口（当前用于桌宠缩放）
- **重启** - 重启应用
- **退出** - 退出应用程序

> 历史：后端切换菜单（选择后端）已从托盘右键菜单中移除，后端切换功能保留在设置窗口（Options）中。

## 实现方案

### 1. 依赖配置

**src-tauri/Cargo.toml**
```toml
tauri = { version = "2", features = ["macos-private-api", "tray-icon"] }
```

### 2. 核心代码

**src-tauri/src/lib.rs**

```rust
use tauri::menu::{MenuBuilder, MenuItemBuilder};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};

// 创建菜单项
let quit_item = MenuItemBuilder::with_id("quit", "退出").build(app)?;
let options_item = MenuItemBuilder::with_id("options", "选项").build(app)?;

let menu = MenuBuilder::new(app)
    .item(&options_item)
    .separator()
    .item(&quit_item)
    .build()?;

// 创建 TrayIcon
let _tray = TrayIconBuilder::new()
    .icon(icon)                              // 使用 app 默认图标
    .tooltip("MeoClaw")
    .menu(&menu)
    .show_menu_on_left_click(false)          // 禁止左键显示菜单
    .on_menu_event(move |app, event| {
        match event.id().as_ref() {
            "quit" => app.exit(0),
            "restart" => { /* 重启应用 */ }
            "options" => { /* 打开 options 窗口 */ }
            _ => {}
        }
    })
    .on_tray_icon_event(move |_tray, event| {
        if let TrayIconEvent::Click { button: MouseButton::Left, button_state: MouseButtonState::Up, .. } = event {
            // 左键单击 - 切换窗口显示/隐藏
            if let Some(window) = main_win.get_webview_window("main") {
                if window.is_visible().unwrap_or(false) {
                    let _ = window.hide();
                } else {
                    let _ = window.show();
                }
            }
        }
    })
    .build(app)?;
```

## 关键 API

### TrayIconBuilder

| 方法 | 说明 |
|------|------|
| `.icon(Image)` | 设置图标 |
| `.tooltip(&str)` | 设置鼠标悬停提示 |
| `.menu(&Menu)` | 设置右键菜单 |
| `.show_menu_on_left_click(false)` | 禁止左键单击弹出菜单 |
| `.on_menu_event(callback)` | 菜单项点击事件 |
| `.on_tray_icon_event(callback)` | 图标点击事件 |

### 事件类型

`TrayIconEvent::Click` 包含：
- `button`: `MouseButton::Left` / `MouseButton::Right`
- `button_state`: `MouseButtonState::Up` / `MouseButtonState::Down`

### MenuItemBuilder

```rust
MenuItemBuilder::with_id("quit", "退出").build(app)?
```

## 注意事项

1. **左键双击问题**：macOS 系统 tray 图标不支持双击事件，只能通过单击模拟双击（300ms 内两次单击），但容易与单次单击混淆，故采用单击直接触发
2. **图标来源**：使用 `app.default_window_icon()` 获取 tauri.conf.json 中配置的图标
3. **菜单显示**：通过 `.show_menu_on_left_click(false)` 禁用左键菜单，使左键专用于切换窗口显示
4. **选项窗口**：`options` 菜单项当前已经落地，打开的是 `options.html` 对应的独立 Tauri 窗口
