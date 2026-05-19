# 自动启动

_Source: https://v2.tauri.org.cn/plugin/autostart/_

[ GitHub](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/autostart) [ npm ](https://npmjs.net.cn/package/@tauri-apps/plugin-autostart) [ crates.io ](https://crates.io/crates/tauri-plugin-autostart)

API 参考 [ ](/reference/javascript/autostart/) [ ](https://docs.rs/tauri-plugin-autostart)

在系统启动时自动运行您的应用程序。

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

安装 autostart 插件以开始使用。

  * 自动
  * 手动

使用你的项目包管理器添加依赖项

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri add autostart

    yarn run tauri add autostart

    pnpm tauri add autostart

    deno task tauri add autostart

    bun tauri add autostart

    cargo tauri add autostart

  1. 在 `src-tauri` 文件夹中运行以下命令，将插件添加到项目的 `Cargo.toml` 依赖项中

         cargo add tauri-plugin-autostart --target 'cfg(any(target_os = "macos", windows, target_os = "linux"))'

  2. 修改 `lib.rs` 以初始化插件

src-tauri/src/lib.rs

         #[cfg_attr(mobile, tauri::mobile_entry_point)]

         pub fn run() {

             tauri::Builder::default()

                 .setup(|app| {

                     #[cfg(desktop)]

                     app.handle().plugin(tauri_plugin_autostart::init(tauri_plugin_autostart::MacosLauncher::LaunchAgent, Some(vec!["--flag1", "--flag2"]) /* arbitrary number of args to pass to your app */));

                     Ok(())

                 })

                 .run(tauri::generate_context!())

                 .expect("error while running tauri application");

         }

  3. 您可以使用首选的 JavaScript 包管理器安装 JavaScript 客户端绑定。

     * npm
     * yarn
     * pnpm
     * deno
     * bun

    npm install @tauri-apps/plugin-autostart

    yarn add @tauri-apps/plugin-autostart

    pnpm add @tauri-apps/plugin-autostart

    deno add npm:@tauri-apps/plugin-autostart

    bun add @tauri-apps/plugin-autostart

## 用法

名为“用法”的章节

autostart 插件同时提供 JavaScript 和 Rust 版本。

  * JavaScript
  * Rust

    import { enable, isEnabled, disable } from '@tauri-apps/plugin-autostart';

    // when using `"withGlobalTauri": true`, you may use

    // const { enable, isEnabled, disable } = window.__TAURI__.autostart;

    // Enable autostart

    await enable();

    // Check enable state

    console.log(`registered for autostart? ${await isEnabled()}`);

    // Disable autostart

    disable();

    #[cfg_attr(mobile, tauri::mobile_entry_point)]

    pub fn run() {

        tauri::Builder::default()

            .setup(|app| {

                #[cfg(desktop)]

                {

                    use tauri_plugin_autostart::MacosLauncher;

                    use tauri_plugin_autostart::ManagerExt;

                    app.handle().plugin(tauri_plugin_autostart::init(

                        MacosLauncher::LaunchAgent,

                        Some(vec!["--flag1", "--flag2"]),

                    ));

                    // Get the autostart manager

                    let autostart_manager = app.autolaunch();

                    // Enable autostart

                    let _ = autostart_manager.enable();

                    // Check enable state

                    println!("registered for autostart? {}", autostart_manager.is_enabled().unwrap());

                    // Disable autostart

                    let _ = autostart_manager.disable();

                }

                Ok(())

            })

            .run(tauri::generate_context!())

            .expect("error while running tauri application");

    }

## Permissions

名为“权限”的章节

默认情况下，所有潜在危险的插件命令和范围都被阻止，无法访问。您必须修改 `capabilities` 配置中的权限才能启用这些功能。

有关更多信息，请参阅[功能概述](/security/capabilities/)，并参阅[分步指南](/learn/security/using-plugin-permissions/)以使用插件权限。

src-tauri/capabilities/default.json

    {

      "permissions": [

        ...,

        "autostart:allow-enable",

        "autostart:allow-disable",

        "autostart:allow-is-enabled"

      ]

    }

## 默认权限

此权限设置用于配置您的应用程序是否可以启用或禁用开机自动启动。

#### 已授予权限

它允许所有人检查、启用和禁用开机自动启动。

#### 此默认权限集包括以下内容

  * `allow-enable`
  * `allow-disable`
  * `allow-is-enabled`

## 权限表

标识符 | 描述
---|---
`autostart:allow-disable` |  在没有任何预配置范围的情况下启用 disable 命令。
`autostart:deny-disable` |  在没有任何预配置范围的情况下拒绝 disable 命令。
`autostart:allow-enable` |  在没有任何预配置范围的情况下启用 enable 命令。
`autostart:deny-enable` |  在没有任何预配置范围的情况下拒绝 enable 命令。
`autostart:allow-is-enabled` |  在没有任何预配置范围的情况下启用 is_enabled 命令。
`autostart:deny-is-enabled` |  在没有任何预配置范围的情况下拒绝 is_enabled 命令。