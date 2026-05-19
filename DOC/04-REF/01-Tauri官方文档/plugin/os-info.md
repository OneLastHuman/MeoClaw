# 操作系统信息

_Source: https://v2.tauri.org.cn/plugin/os-info/_

[ GitHub](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/os) [ npm ](https://npmjs.net.cn/package/@tauri-apps/plugin-os) [ crates.io ](https://crates.io/crates/tauri-plugin-os)

API 参考 [ ](/reference/javascript/os/) [ ](https://docs.rs/tauri-plugin-os)

使用操作系统信息插件读取关于操作系统的信息。

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

安装操作系统信息插件以开始使用。

  * 自动
  * 手动

使用你的项目包管理器添加依赖项

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri add os

    yarn run tauri add os

    pnpm tauri add os

    deno task tauri add os

    bun tauri add os

    cargo tauri add os

  1. 在 `src-tauri` 文件夹中运行以下命令，将插件添加到项目的 `Cargo.toml` 依赖项中

         cargo add tauri-plugin-os

  2. 修改 `lib.rs` 以初始化插件

src-tauri/src/lib.rs

         #[cfg_attr(mobile, tauri::mobile_entry_point)]

         pub fn run() {

             tauri::Builder::default()

                 .plugin(tauri_plugin_os::init())

                 .run(tauri::generate_context!())

                 .expect("error while running tauri application");

         }

  3. 如果你想在 JavaScript 中使用它，也请安装 npm 包：

     * npm
     * yarn
     * pnpm
     * deno
     * bun

    npm install @tauri-apps/plugin-os

    yarn add @tauri-apps/plugin-os

    pnpm add @tauri-apps/plugin-os

    deno add npm:@tauri-apps/plugin-os

    bun add @tauri-apps/plugin-os

## 用法

名为“用法”的章节

通过此插件，您可以查询当前操作系统中的多种信息。请参阅 [JavaScript API](/reference/javascript/os/) 或 [Rust API](https://docs.rs/tauri-plugin-os/) 参考资料以查看所有可用函数。

#### 示例：操作系统平台

章节标题“示例：操作系统平台”

`platform` 返回一个描述当前所用操作系统的字符串。该值在编译时设定。可能的值包括 `linux`、`macos`、`ios`、`freebsd`、`dragonfly`、`netbsd`、`openbsd`、`solaris`、`android`、`windows`。

  * JavaScript
  * Rust

    import { platform } from '@tauri-apps/plugin-os';

    // when using `"withGlobalTauri": true`, you may use

    // const { platform } = window.__TAURI__.os;

    const currentPlatform = platform();

    console.log(currentPlatform);

    // Prints "windows" to the console

    let platform = tauri_plugin_os::platform();

    println!("Platform: {}", platform);

    // Prints "windows" to the terminal

## Permissions

名为“权限”的章节

默认情况下，所有潜在危险的插件命令和范围都被阻止，无法访问。您必须修改 `capabilities` 配置中的权限才能启用这些功能。

有关更多信息，请参阅[功能概述](/security/capabilities/)，并参阅[分步指南](/learn/security/using-plugin-permissions/)以使用插件权限。

src-tauri/capabilities/default.json

    {

      "permissions": [

        ...,

        "os:default"

      ]

    }

## 默认权限

此权限集配置了前端可以获取哪些操作系统信息。

#### 已授予权限

除主机名外，所有信息均可获取。

#### 此默认权限集包括以下内容

  * `allow-arch`
  * `allow-exe-extension`
  * `allow-family`
  * `allow-locale`
  * `allow-os-type`
  * `allow-platform`
  * `allow-version`

## 权限表

标识符 | 描述
---|---
`os:allow-arch` |  启用 arch 命令，无需任何预先配置的作用域。
`os:deny-arch` |  禁用 arch 命令，无需任何预先配置的作用域。
`os:allow-exe-extension` |  启用 exe_extension 命令，无需任何预先配置的作用域。
`os:deny-exe-extension` |  禁用 exe_extension 命令，无需任何预先配置的作用域。
`os:allow-family` |  启用 family 命令，无需任何预先配置的作用域。
`os:deny-family` |  禁用 family 命令，无需任何预先配置的作用域。
`os:allow-hostname` |  启用 hostname 命令，无需任何预先配置的作用域。
`os:deny-hostname` |  禁用 hostname 命令，无需任何预先配置的作用域。
`os:allow-locale` |  启用 locale 命令，无需任何预先配置的作用域。
`os:deny-locale` |  禁用 locale 命令，无需任何预先配置的作用域。
`os:allow-os-type` |  启用 os_type 命令，无需任何预先配置的作用域。
`os:deny-os-type` |  禁用 os_type 命令，无需任何预先配置的作用域。
`os:allow-platform` |  启用 platform 命令，无需任何预先配置的作用域。
`os:deny-platform` |  禁用 platform 命令，无需任何预先配置的作用域。
`os:allow-version` |  启用 version 命令，无需任何预先配置的作用域。
`os:deny-version` |  禁用 version 命令，无需任何预先配置的作用域。