# Trunk

_Source: https://v2.tauri.org.cn/start/frontend/trunk/_

Trunk 是一个用于 Rust 的 WASM Web 应用打包器。访问 <https://trunk-rs.github.io/trunk/> 了解更多关于 Trunk 的信息。本指南适用于 Trunk 0.17.5 版本。

## 清单

名为“检查清单”的章节

  * 请使用 SSG（静态站点生成），Tauri 官方不支持基于服务器的解决方案。
  * 使用 `serve.ws_protocol = "ws"`，以便热重载 WebSocket 能够正确连接，用于移动端开发。
  * 启用 `withGlobalTauri` 以确保 Tauri API 在 `window.__TAURI__` 变量中可用，并且可以通过 `wasm-bindgen` 进行导入。

## 示例配置

名为“示例配置”的章节

  1. ##### 更新 Tauri 配置

标题为“更新 Tauri 配置”的章节

tauri.conf.json

         {

           "build": {

             "beforeDevCommand": "trunk serve",

             "beforeBuildCommand": "trunk build",

             "devUrl": "https://:8080",

             "frontendDist": "../dist"

           },

           "app": {

             "withGlobalTauri": true

           }

         }

  2. ##### 更新 Trunk 配置

标题为“更新 Trunk 配置”的章节

Trunk.toml

         [watch]

         ignore = ["./src-tauri"]

         [serve]

         ws_protocol = "ws"