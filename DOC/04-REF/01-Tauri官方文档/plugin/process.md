# 进程

_Source: https://v2.tauri.org.cn/plugin/process/_

[ GitHub](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/process) [ npm ](https://npmjs.net.cn/package/@tauri-apps/plugin-process) [ crates.io ](https://crates.io/crates/tauri-plugin-process)

API 参考 [ ](/reference/javascript/process/) [ ](https://docs.rs/tauri-plugin-process)

此插件提供了访问当前进程的 API。若要生成子进程，请参阅 [shell](/plugin/shell/) 插件。

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

安装 process 插件以开始使用。

  * 自动
  * 手动

使用你的项目包管理器添加依赖项

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri add process

    yarn run tauri add process

    pnpm tauri add process

    deno task tauri add process

    bun tauri add process

    cargo tauri add process

  1. 在 `src-tauri` 文件夹中运行以下命令，将插件添加到项目的 `Cargo.toml` 依赖项中

         cargo add tauri-plugin-process

  2. 修改 `lib.rs` 以初始化插件

src-tauri/src/lib.rs

         #[cfg_attr(mobile, tauri::mobile_entry_point)]

         pub fn run() {

             tauri::Builder::default()

                 .plugin(tauri_plugin_process::init())

                 .run(tauri::generate_context!())

                 .expect("error while running tauri application");

         }

  3. 如果你想在 JavaScript 中使用该插件，请同时安装 npm 包

     * npm
     * yarn
     * pnpm
     * deno
     * bun

    npm install @tauri-apps/plugin-process

    yarn add @tauri-apps/plugin-process

    pnpm add @tauri-apps/plugin-process

    deno add npm:@tauri-apps/plugin-process

    bun add @tauri-apps/plugin-process

## 用法

名为“用法”的章节

process 插件同时支持 JavaScript 和 Rust。

  * JavaScript
  * Rust

    import { exit, relaunch } from '@tauri-apps/plugin-process';

    // when using `"withGlobalTauri": true`, you may use

    // const { exit, relaunch } = window.__TAURI__.process;

    // exits the app with the given status code

    await exit(0);

    // restarts the app

    await relaunch();

请注意，`app` 是 [`AppHandle`](https://docs.rs/tauri/2.0.0/tauri/struct.AppHandle.html) 的一个实例。

    // exits the app with the given status code

    app.exit(0);

    // restarts the app

    app.restart();

## Permissions

名为“权限”的章节

默认情况下，所有潜在危险的插件命令和范围都被阻止，无法访问。您必须修改 `capabilities` 配置中的权限才能启用这些功能。

有关更多信息，请参阅[功能概述](/security/capabilities/)，并参阅[分步指南](/learn/security/using-plugin-permissions/)以使用插件权限。

src-tauri/capabilities/default.json

    {

      "permissions": [

        ...,

        "process:default",

      ]

    }

## 默认权限

此权限集配置了默认公开的进程功能。

#### 已授予权限

这允许通过 `allow-exit` 退出应用程序，以及通过 `allow-restart` 重启应用程序。

#### 此默认权限集包括以下内容

  * `allow-exit`
  * `allow-restart`

## 权限表

标识符 | 描述
---|---
`process:allow-exit` |  启用 exit 命令，无需任何预先配置的作用域。
`process:deny-exit` |  禁用 exit 命令，无需任何预先配置的作用域。
`process:allow-restart` |  启用 restart 命令，无需任何预先配置的作用域。
`process:deny-restart` |  禁用 restart 命令，无需任何预先配置的作用域。