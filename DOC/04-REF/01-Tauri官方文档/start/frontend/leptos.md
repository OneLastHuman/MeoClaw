# Leptos

_Source: https://v2.tauri.org.cn/start/frontend/leptos/_

Leptos 是一个基于 Rust 的 Web 框架。你可以在其[官方网站](https://leptos.dev/)上阅读更多关于 Leptos 的信息。本指南适用于 Leptos 0.6 版本。

## 清单

名为“检查清单”的章节

  * 请使用 SSG（静态站点生成），Tauri 官方不支持基于服务器的解决方案。
  * 请使用 `serve.ws_protocol = "ws"`，以便热重载 websocket 能够正确连接，用于移动端开发。
  * 启用 `withGlobalTauri` 以确保 Tauri API 在 `window.__TAURI__` 变量中可用，并可以使用 `wasm-bindgen` 进行导入。

## 示例配置

名为“示例配置”的章节

  1. ##### 更新 Tauri 配置

名为“更新 Tauri 配置”的章节

src-tauri/tauri.conf.json

         {

           "build": {

             "beforeDevCommand": "trunk serve",

             "devUrl": "https://:1420",

             "beforeBuildCommand": "trunk build",

             "frontendDist": "../dist"

           },

           "app": {

             "withGlobalTauri": true

           }

         }

  2. ##### 更新 Trunk 配置

名为“更新 Trunk 配置”的章节

Trunk.toml

         [build]

         target = "./index.html"

         [watch]

         ignore = ["./src-tauri"]

         [serve]

         port = 1420

         open = false

         ws_protocol = "ws"