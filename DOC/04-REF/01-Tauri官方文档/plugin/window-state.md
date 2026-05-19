# 窗口状态

_Source: https://v2.tauri.org.cn/plugin/window-state/_

[ GitHub](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/window-state) [ npm ](https://npmjs.net.cn/package/@tauri-apps/plugin-window-state) [ crates.io ](https://crates.io/crates/tauri-plugin-window-state)

API 参考 [ ](/reference/javascript/window-state/) [ ](https://docs.rs/tauri-plugin-window-state)

保存窗口位置和大小，并在应用重新打开时恢复。

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

安装 window-state 插件以开始使用。

  * 自动
  * 手动

使用你的项目包管理器添加依赖项

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri add window-state

    yarn run tauri add window-state

    pnpm tauri add window-state

    deno task tauri add window-state

    bun tauri add window-state

    cargo tauri add window-state

  1. 在 `src-tauri` 文件夹中运行以下命令，将插件添加到项目的 `Cargo.toml` 依赖项中

         cargo add tauri-plugin-window-state --target 'cfg(any(target_os = "macos", windows, target_os = "linux"))'

  2. 修改 `lib.rs` 以初始化插件

src-tauri/src/lib.rs

         #[cfg_attr(mobile, tauri::mobile_entry_point)]

         pub fn run() {

             tauri::Builder::default()

                 .setup(|app| {

                     #[cfg(desktop)]

                     app.handle().plugin(tauri_plugin_window_state::Builder::default().build());

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

    npm install @tauri-apps/plugin-window-state

    yarn add @tauri-apps/plugin-window-state

    pnpm add @tauri-apps/plugin-window-state

    deno add npm:@tauri-apps/plugin-window-state

    bun add @tauri-apps/plugin-window-state

## 用法

名为“用法”的章节

添加 window-state 插件后，所有窗口在关闭时都会记住其状态，并会在下次启动时恢复到之前的状态。

你也可以在 JavaScript 和 Rust 中访问 window-state 插件。

提示

状态恢复发生在窗口创建之后，因此为了防止窗口闪烁，你可以在创建窗口时将 `visible` 设置为 `false`，插件会在恢复状态时显示该窗口。

### JavaScript

名为“JavaScript”的章节

你可以使用 `saveWindowState` 手动保存窗口状态。

    import { saveWindowState, StateFlags } from '@tauri-apps/plugin-window-state';

    // when using `"withGlobalTauri": true`, you may use

    // const { saveWindowState, StateFlags } = window.__TAURI__.windowState;

    saveWindowState(StateFlags.ALL);

同样，你可以从磁盘手动恢复窗口的状态。

    import {

      restoreStateCurrent,

      StateFlags,

    } from '@tauri-apps/plugin-window-state';

    // when using `"withGlobalTauri": true`, you may use

    // const { restoreStateCurrent, StateFlags } = window.__TAURI__.windowState;

    restoreStateCurrent(StateFlags.ALL);

### Rust

名为“Rust”的章节

你可以使用 `AppHandleExt` 特性（trait）公开的 `save_window_state()` 方法。

    use tauri_plugin_window_state::{AppHandleExt, StateFlags};

    // `tauri::AppHandle` now has the following additional method

    app.save_window_state(StateFlags::all()); // will save the state of all open windows to disk

同样，你可以使用 `WindowExt` 特性公开的 `restore_state()` 方法从磁盘手动恢复窗口的状态。

    use tauri_plugin_window_state::{WindowExt, StateFlags};

    // all `Window` types now have the following additional method

    window.restore_state(StateFlags::all()); // will restore the window's state from disk

## Permissions

名为“权限”的章节

默认情况下，所有潜在危险的插件命令和范围都被阻止，无法访问。您必须修改 `capabilities` 配置中的权限才能启用这些功能。

有关更多信息，请参阅[功能概述](/security/capabilities/)，并参阅[分步指南](/learn/security/using-plugin-permissions/)以使用插件权限。

src-tauri/capabilities/default.json

    {

      "permissions": [

        ...,

        "window-state:default",

      ]

    }

## 默认权限

此权限集配置了 window-state 插件可以执行的操作类型。

#### 已授予权限

默认情况下，所有操作均已启用。

#### 此默认权限集包括以下内容

  * `allow-filename`
  * `allow-restore-state`
  * `allow-save-window-state`

## 权限表

标识符 | 描述
---|---
`window-state:allow-filename` |  启用 filename 命令，无需任何预先配置的范围。
`window-state:deny-filename` |  禁用 filename 命令，无需任何预先配置的范围。
`window-state:allow-restore-state` |  启用 restore_state 命令，无需任何预先配置的范围。
`window-state:deny-restore-state` |  禁用 restore_state 命令，无需任何预先配置的范围。
`window-state:allow-save-window-state` |  启用 save_window_state 命令，无需任何预先配置的范围。
`window-state:deny-save-window-state` |  禁用 save_window_state 命令，无需任何预先配置的范围。