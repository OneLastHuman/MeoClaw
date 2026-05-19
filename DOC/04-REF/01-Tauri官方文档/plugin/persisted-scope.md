# 持久作用域

_Source: https://v2.tauri.org.cn/plugin/persisted-scope/_

[ GitHub](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/persisted-scope) [ crates.io ](https://crates.io/crates/tauri-plugin-persisted-scope)

API 参考 [ ](https://docs.rs/tauri-plugin-persisted-scope)

保存文件系统和资源作用域，并在应用重新打开时恢复它们。

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

安装 `persisted-scope` 插件即可开始使用。

  * 自动
  * 手动

使用你的项目包管理器添加依赖项

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri add persisted-scope

    yarn run tauri add persisted-scope

    pnpm tauri add persisted-scope

    deno task tauri add persisted-scope

    bun tauri add persisted-scope

    cargo tauri add persisted-scope

  1. 在 `src-tauri` 文件夹中运行以下命令，将插件添加到项目的 `Cargo.toml` 依赖项中

         cargo add tauri-plugin-persisted-scope

  2. 修改 `lib.rs` 以初始化插件

src-tauri/src/lib.rs

         #[cfg_attr(mobile, tauri::mobile_entry_point)]

         pub fn run() {

             tauri::Builder::default()

                 .plugin(tauri_plugin_persisted_scope::init())

                 .run(tauri::generate_context!())

                 .expect("error while running tauri application");

         }

注意

`persisted-scope` 插件 _必须_ 在 `fs` 插件之后进行注册和初始化，如下图所示

    #[cfg_attr(mobile, tauri::mobile_entry_point)]

    pub fn run() {

        tauri::Builder::default()

            .plugin(tauri_plugin_fs::init()) // fs MUST BE before persisted scope!

            .plugin(tauri_plugin_persisted_scope::init())

            .run(tauri::generate_context!())

            .expect("error while running tauri application");

    }

**如果不这样做，将导致持久化作用域无法工作！** 在开发模式下启动应用时，您还应该会看到类似以下的警告信息

    Please make sure to register the `fs` plugin before the `persisted-scope` plugin!

## 用法

名为“用法”的章节

设置完成后，插件将自动保存并恢复文件系统和资源作用域。