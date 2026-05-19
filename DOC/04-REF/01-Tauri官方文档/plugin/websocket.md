# Websocket

_Source: https://v2.tauri.org.cn/plugin/websocket/_

[ GitHub](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/websocket) [ npm ](https://npmjs.net.cn/package/@tauri-apps/plugin-websocket) [ crates.io ](https://crates.io/crates/tauri-plugin-websocket)

API 参考 [ ](/reference/javascript/websocket/) [ ](https://docs.rs/tauri-plugin-websocket)

在 JavaScript 中使用 Rust 客户端打开 WebSocket 连接。

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

安装 websocket 插件以开始使用。

  * 自动
  * 手动

使用你的项目包管理器添加依赖项

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri add websocket

    yarn run tauri add websocket

    pnpm tauri add websocket

    deno task tauri add websocket

    bun tauri add websocket

    cargo tauri add websocket

  1. 在 `src-tauri` 文件夹中运行以下命令，将插件添加到项目的 `Cargo.toml` 依赖项中

         cargo add tauri-plugin-websocket

  2. 修改 `lib.rs` 以初始化插件

src-tauri/src/lib.rs

         #[cfg_attr(mobile, tauri::mobile_entry_point)]

         pub fn run() {

             tauri::Builder::default()

                 .plugin(tauri_plugin_websocket::init())

                 .run(tauri::generate_context!())

                 .expect("error while running tauri application");

         }

  3. 使用您首选的 JavaScript 包管理器安装 JavaScript 访客绑定

     * npm
     * yarn
     * pnpm
     * deno
     * bun

    npm install @tauri-apps/plugin-websocket

    yarn add @tauri-apps/plugin-websocket

    pnpm add @tauri-apps/plugin-websocket

    deno add npm:@tauri-apps/plugin-websocket

    bun add @tauri-apps/plugin-websocket

## 用法

名为“用法”的章节

websocket 插件可在 JavaScript 中使用。

    import WebSocket from '@tauri-apps/plugin-websocket';

    // when using `"withGlobalTauri": true`, you may use

    // const WebSocket = window.__TAURI__.websocket;

    const ws = await WebSocket.connect('ws://127.0.0.1:8080');

    const removeListener = ws.addListener((msg) => {

      console.log('Received Message:', msg);

    });

    await ws.send('Hello World!');

    // optionally remove the listener

    removeListener();

    await ws.disconnect();

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

      "permissions": ["websocket:default"]

    }

## 默认权限

允许连接到 WebSocket 服务器并发送数据

#### 此默认权限集包括以下内容

  * `allow-connect`
  * `allow-send`

## 权限表

标识符 | 描述
---|---
`websocket:allow-connect` |  在没有任何预配置作用域的情况下启用 connect 命令。
`websocket:deny-connect` |  在没有任何预配置作用域的情况下拒绝 connect 命令。
`websocket:allow-send` |  在没有任何预配置作用域的情况下启用 send 命令。
`websocket:deny-send` |  在没有任何预配置作用域的情况下拒绝 send 命令。