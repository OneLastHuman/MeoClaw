# 全局快捷键

_Source: https://v2.tauri.org.cn/plugin/global-shortcut/_

[ GitHub](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/global-shortcut) [ npm ](https://npmjs.net.cn/package/@tauri-apps/plugin-global-shortcut) [ crates.io ](https://crates.io/crates/tauri-plugin-global-shortcut)

API 参考 [ ](/reference/javascript/global-shortcut/) [ ](https://docs.rs/tauri-plugin-global-shortcut)

注册全局快捷键。

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

安装 global-shortcut 插件即可开始使用。

  * 自动
  * 手动

使用你的项目包管理器添加依赖项

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri add global-shortcut

    yarn run tauri add global-shortcut

    pnpm tauri add global-shortcut

    deno task tauri add global-shortcut

    bun tauri add global-shortcut

    cargo tauri add global-shortcut

  1. 在 `src-tauri` 文件夹中运行以下命令，将插件添加到项目的 `Cargo.toml` 依赖项中

         cargo add tauri-plugin-global-shortcut --target 'cfg(any(target_os = "macos", windows, target_os = "linux"))'

  2. 修改 `lib.rs` 以初始化插件

src-tauri/src/lib.rs

         pub fn run() {

             tauri::Builder::default()

                 .setup(|app| {

                     #[cfg(desktop)]

                     app.handle().plugin(tauri_plugin_global_shortcut::Builder::new().build());

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

    npm install @tauri-apps/plugin-global-shortcut

    yarn add @tauri-apps/plugin-global-shortcut

    pnpm add @tauri-apps/plugin-global-shortcut

    deno add npm:@tauri-apps/plugin-global-shortcut

    bun add @tauri-apps/plugin-global-shortcut

## 用法

名为“用法”的章节

global-shortcut 插件同时支持 JavaScript 和 Rust。

  * JavaScript
  * Rust

    import { register } from '@tauri-apps/plugin-global-shortcut';

    // when using `"withGlobalTauri": true`, you may use

    // const { register } = window.__TAURI__.globalShortcut;

    await register('CommandOrControl+Shift+C', () => {

      console.log('Shortcut triggered');

    });

src-tauri/src/lib.rs

    pub fn run() {

        tauri::Builder::default()

            .setup(|app| {

                #[cfg(desktop)]

                {

                    use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

                    let ctrl_n_shortcut = Shortcut::new(Some(Modifiers::CONTROL), Code::KeyN);

                    app.handle().plugin(

                        tauri_plugin_global_shortcut::Builder::new().with_handler(move |_app, shortcut, event| {

                            println!("{:?}", shortcut);

                            if shortcut == &ctrl_n_shortcut {

                                match event.state() {

                                  ShortcutState::Pressed => {

                                    println!("Ctrl-N Pressed!");

                                  }

                                  ShortcutState::Released => {

                                    println!("Ctrl-N Released!");

                                  }

                                }

                            }

                        })

                        .build(),

                    )?;

                    app.global_shortcut().register(ctrl_n_shortcut)?;

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

      "$schema": "../gen/schemas/desktop-schema.json",

      "identifier": "main-capability",

      "description": "Capability for the main window",

      "windows": ["main"],

      "permissions": [

        "global-shortcut:allow-is-registered",

        "global-shortcut:allow-register",

        "global-shortcut:allow-unregister"

      ]

    }

## 默认权限

默认不启用任何功能，因为我们认为快捷键本身可能具有危险性，是否注册或注销特定快捷键应由具体的应用程序决定。

## 权限表

标识符 | 描述
---|---
`global-shortcut:allow-is-registered` |  在没有预配置作用域的情况下启用 is_registered 命令。
`global-shortcut:deny-is-registered` |  在没有预配置作用域的情况下禁用 is_registered 命令。
`global-shortcut:allow-register` |  在没有预配置作用域的情况下启用 register 命令。
`global-shortcut:deny-register` |  在没有预配置作用域的情况下禁用 register 命令。
`global-shortcut:allow-register-all` |  在没有预配置作用域的情况下启用 register_all 命令。
`global-shortcut:deny-register-all` |  在没有预配置作用域的情况下禁用 register_all 命令。
`global-shortcut:allow-unregister` |  在没有预配置作用域的情况下启用 unregister 命令。
`global-shortcut:deny-unregister` |  在没有预配置作用域的情况下禁用 unregister 命令。
`global-shortcut:allow-unregister-all` |  在没有预配置作用域的情况下启用 unregister_all 命令。
`global-shortcut:deny-unregister-all` |  在没有预配置作用域的情况下禁用 unregister_all 命令。