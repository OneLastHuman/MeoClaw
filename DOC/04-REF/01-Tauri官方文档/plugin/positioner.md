# 定位器

_Source: https://v2.tauri.org.cn/plugin/positioner/_

[ GitHub](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/positioner) [ npm ](https://npmjs.net.cn/package/@tauri-apps/plugin-positioner) [ crates.io ](https://crates.io/crates/tauri-plugin-positioner)

API 参考 [ ](/reference/javascript/positioner/) [ ](https://docs.rs/tauri-plugin-positioner)

将窗口定位到预定位置。

此插件是 Tauri 对 [electron-positioner](https://github.com/jenslind/electron-positioner) 的移植版本。

## 支持的平台

标题为“支持的平台”的章节

_此插件需要 Rust 版本至少为 **1.77.2**_

平台 | 级别 | 备注
---|---|---
windows |  |
linux |  |
macos |  |
android |  |
ios |  |

## 设置

标题为“设置”的章节

安装 positioner 插件以开始使用。

注意

如果你只打算通过 Rust 代码移动窗口，则只需在 `src-tauri/Cargo.toml` 中添加依赖，如果选择自动设置，则可以移除 `lib.rs` 中的插件注册。

  * 自动
  * 手动

使用你的项目包管理器添加依赖项

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri add positioner

    yarn run tauri add positioner

    pnpm tauri add positioner

    deno task tauri add positioner

    bun tauri add positioner

    cargo tauri add positioner

  1. 在 `src-tauri` 文件夹中运行以下命令，将插件添加到项目的 `Cargo.toml` 依赖项中

         cargo add tauri-plugin-positioner --target 'cfg(any(target_os = "macos", windows, target_os = "linux"))'

  2. 修改 `lib.rs` 以初始化插件

src-tauri/src/lib.rs

         #[cfg_attr(mobile, tauri::mobile_entry_point)]

         pub fn run() {

             tauri::Builder::default()

                 .setup(|app| {

                     #[cfg(desktop)]

                     app.handle().plugin(tauri_plugin_positioner::init());

                     Ok(())

                 })

                 .run(tauri::generate_context!())

                 .expect("error while running tauri application");

         }

  3. 使用您首选的 JavaScript 包管理器安装 JavaScript 访客绑定

     * npm
     * yarn
     * pnpm
     * deno
     * bun

    npm install @tauri-apps/plugin-positioner

    yarn add @tauri-apps/plugin-positioner

    pnpm add @tauri-apps/plugin-positioner

    deno add npm:@tauri-apps/plugin-positioner

    bun add @tauri-apps/plugin-positioner

若要使任务栏图标相关的定位功能生效，需要进行额外设置。

  1. 在你的 `Cargo.toml` 文件中添加 `tray-icon` 功能特性

src-tauri/Cargo.toml

         [dependencies]

         tauri-plugin-positioner = { version = "2.0.0", features = ["tray-icon"] }

  2. 为 positioner 插件设置 `on_tray_event`

src-tauri/src/lib.rs

         pub fn run() {

           tauri::Builder::default()

             // This is required to get tray-relative positions to work

             .setup(|app| {

                 #[cfg(desktop)]

                 {

                   app.handle().plugin(tauri_plugin_positioner::init());

                     tauri::tray::TrayIconBuilder::new()

                       .on_tray_icon_event(|tray_handle, event| {

                         tauri_plugin_positioner::on_tray_event(tray_handle.app_handle(), &event);

                       })

                       .build(app)?;

                 }

               Ok(())

             })

             .run(tauri::generate_context!())

             .expect("error while running tauri application");

         }

## 用法

名为“用法”的章节

该插件的 API 可通过 JavaScript 客端绑定使用

    import { moveWindow, Position } from '@tauri-apps/plugin-positioner';

    // when using `"withGlobalTauri": true`, you may use

    // const { moveWindow, Position } = window.__TAURI__.positioner;

    moveWindow(Position.TopRight);

你可以直接通过 Rust 使用 Window 特性扩展（trait extension）

    use tauri_plugin_positioner::{WindowExt, Position};

    let mut win = app.get_webview_window("main").unwrap();

    let _ = win.as_ref().window().move_window(Position::TopRight);

## Permissions

名为“权限”的章节

默认情况下，所有潜在危险的插件命令和范围都被阻止，无法访问。您必须修改 `capabilities` 配置中的权限才能启用这些功能。

有关更多信息，请参阅[功能概述](/security/capabilities/)，并参阅[分步指南](/learn/security/using-plugin-permissions/)以使用插件权限。

src-tauri/capabilities/default.json

    {

      "permissions": [

        ...,

        "positioner:default",

      ]

    }

## 默认权限

允许使用 moveWindow 和 handleIconState API

#### 此默认权限集包括以下内容

  * `allow-move-window`
  * `allow-move-window-constrained`
  * `allow-set-tray-icon-state`

## 权限表

标识符 | 描述
---|---
`positioner:allow-move-window` |  在没有任何预配置范围的情况下启用 move_window 命令。
`positioner:deny-move-window` |  在没有任何预配置范围的情况下禁用 move_window 命令。
`positioner:allow-move-window-constrained` |  在没有任何预配置范围的情况下启用 move_window_constrained 命令。
`positioner:deny-move-window-constrained` |  在没有任何预配置范围的情况下禁用 move_window_constrained 命令。
`positioner:allow-set-tray-icon-state` |  在没有任何预配置范围的情况下启用 set_tray_icon_state 命令。
`positioner:deny-set-tray-icon-state` |  在没有任何预配置范围的情况下禁用 set_tray_icon_state 命令。