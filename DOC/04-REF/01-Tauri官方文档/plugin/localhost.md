# 本地主机

_Source: https://v2.tauri.org.cn/plugin/localhost/_

[ GitHub](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/localhost) [ crates.io ](https://crates.io/crates/tauri-plugin-localhost)

API 参考 [ ](https://docs.rs/tauri-plugin-localhost)

通过 localhost 服务器而非默认的自定义协议来公开应用的资源。

注意

此插件会带来相当大的安全风险，因此只有在明确自己在做什么的情况下才应使用它。如有疑问，请使用默认的自定义协议实现。

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

安装 localhost 插件即可开始使用。

  * 自动
  * 手动

使用你的项目包管理器添加依赖项

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri add localhost

    yarn run tauri add localhost

    pnpm tauri add localhost

    deno task tauri add localhost

    bun tauri add localhost

    cargo tauri add localhost

  1. 在 `src-tauri` 文件夹中运行以下命令，将插件添加到项目的 `Cargo.toml` 依赖项中

         cargo add tauri-plugin-localhost

  2. 修改 `lib.rs` 以初始化插件

src-tauri/src/lib.rs

         #[cfg_attr(mobile, tauri::mobile_entry_point)]

         pub fn run() {

             tauri::Builder::default()

                 .plugin(tauri_plugin_localhost::Builder::new().build())

                 .run(tauri::generate_context!())

                 .expect("error while running tauri application");

         }

## 用法

名为“用法”的章节

localhost 插件可在 Rust 中使用。

src-tauri/src/lib.rs

    use tauri::{webview::WebviewWindowBuilder, WebviewUrl};

    pub fn run() {

      let port: u16 = 9527;

      tauri::Builder::default()

          .plugin(tauri_plugin_localhost::Builder::new(port).build())

          .setup(move |app| {

              let url = format!("https://:{}", port).parse().unwrap();

              WebviewWindowBuilder::new(app, "main".to_string(), WebviewUrl::External(url))

                  .title("Localhost Example")

                  .build()?;

              Ok(())

          })

          .run(tauri::generate_context!())

          .expect("error while running tauri application");

    }