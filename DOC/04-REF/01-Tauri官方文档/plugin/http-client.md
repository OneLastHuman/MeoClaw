# HTTP 客户端

_Source: https://v2.tauri.org.cn/plugin/http-client/_

[ GitHub](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/http) [ npm ](https://npmjs.net.cn/package/@tauri-apps/plugin-http) [ crates.io ](https://crates.io/crates/tauri-plugin-http)

API 参考 [ ](/reference/javascript/http/) [ ](https://docs.rs/tauri-plugin-http)

使用 http 插件发起 HTTP 请求。

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

安装 http 插件以开始使用。

  * 自动
  * 手动

使用你的项目包管理器添加依赖项

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri add http

    yarn run tauri add http

    pnpm tauri add http

    deno task tauri add http

    bun tauri add http

    cargo tauri add http

  1. 在 `src-tauri` 文件夹中运行以下命令，将插件添加到项目的 `Cargo.toml` 依赖项中

         cargo add tauri-plugin-http

  2. 修改 `lib.rs` 以初始化插件

src-tauri/src/lib.rs

         #[cfg_attr(mobile, tauri::mobile_entry_point)]

         pub fn run() {

             tauri::Builder::default()

                 .plugin(tauri_plugin_http::init())

                 .run(tauri::generate_context!())

                 .expect("error while running tauri application");

         }

  3. 如果你想在 JavaScript 中发起 http 请求，请同时安装 npm 包

     * npm
     * yarn
     * pnpm
     * deno
     * bun

    npm install @tauri-apps/plugin-http

    yarn add @tauri-apps/plugin-http

    pnpm add @tauri-apps/plugin-http

    deno add npm:@tauri-apps/plugin-http

    bun add @tauri-apps/plugin-http

## 用法

名为“用法”的章节

HTTP 插件既可以在 Rust 中作为 [reqwest](https://docs.rs/reqwest/) 的重新导出使用，也可以在 JavaScript 中使用。

### JavaScript

名为“JavaScript”的章节

  1. 配置允许的 URL

src-tauri/capabilities/default.json

         {

           "permissions": [

             {

               "identifier": "http:default",

               "allow": [{ "url": "https://*.tauri.app" }],

               "deny": [{ "url": "https://private.tauri.app" }]

             }

           ]

         }

欲了解更多信息，请参阅 [权限概述](/security/permissions/) 文档

  2. 发送请求

`fetch` 方法旨在尽可能兼容 [`fetch` Web API](https://mdn.org.cn/en-US/docs/Web/API/Fetch_API)。

         import { fetch } from '@tauri-apps/plugin-http';

         // Send a GET request

         const response = await fetch('http://test.tauri.app/data.json', {

           method: 'GET',

         });

         console.log(response.status); // e.g. 200

         console.log(response.statusText); // e.g. "OK"

注意

[禁止的请求头](https://fetch.spec.whatwg.org/#terminology-headers)默认会被忽略。要使用它们，必须启用 `unsafe-headers` 功能标志

src-tauri/Cargo.toml

         [dependencies]

         tauri-plugin-http = { version = "2", features = ["unsafe-headers"] }

### Rust

名为“Rust”的章节

在 Rust 中，你可以使用插件重新导出的 `reqwest` crate。有关详细信息，请参考 [reqwest 文档](https://docs.rs/reqwest/)。

    use tauri_plugin_http::reqwest;

    let res = reqwest::get("http://my.api.host/data.json").await;

    println!("{:?}", res.status()); // e.g. 200

    println!("{:?}", res.text().await); // e.g Ok("{ Content }")

## 默认权限

此权限集配置了 http 插件可用的 fetch 操作类型。

这启用了所有 fetch 操作，但不允许明确获取任何源。使用前需要手动配置。

#### 已授予权限

启用了所有 fetch 操作。

#### 此默认权限集包括以下内容

  * `allow-fetch`
  * `allow-fetch-cancel`
  * `allow-fetch-send`
  * `allow-fetch-read-body`
  * `allow-fetch-cancel-body`

## 权限表

标识符 | 描述
---|---
`http:allow-fetch` |  启用不带任何预配置作用域的 fetch 命令。
`http:deny-fetch` |  禁用不带任何预配置作用域的 fetch 命令。
`http:allow-fetch-cancel` |  启用不带任何预配置作用域的 fetch_cancel 命令。
`http:deny-fetch-cancel` |  禁用不带任何预配置作用域的 fetch_cancel 命令。
`http:allow-fetch-cancel-body` |  启用不带任何预配置作用域的 fetch_cancel_body 命令。
`http:deny-fetch-cancel-body` |  禁用不带任何预配置作用域的 fetch_cancel_body 命令。
`http:allow-fetch-read-body` |  启用不带任何预配置作用域的 fetch_read_body 命令。
`http:deny-fetch-read-body` |  禁用不带任何预配置作用域的 fetch_read_body 命令。
`http:allow-fetch-send` |  启用不带任何预配置作用域的 fetch_send 命令。
`http:deny-fetch-send` |  禁用不带任何预配置作用域的 fetch_send 命令。