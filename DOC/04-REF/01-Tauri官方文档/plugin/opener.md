# 开启器

_Source: https://v2.tauri.org.cn/plugin/opener/_

[ GitHub](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/opener) [ npm ](https://npmjs.net.cn/package/@tauri-apps/plugin-opener) [ crates.io ](https://crates.io/crates/tauri-plugin-opener)

API 参考 [ ](/reference/javascript/opener/) [ ](https://docs.rs/tauri-plugin-opener)

此插件允许你在指定的或默认的应用程序中打开文件和 URL。它还支持在系统的文件资源管理器中“显示”文件。

## 支持的平台

标题为“支持的平台”的章节

_此插件需要 Rust 版本至少为 **1.77.2**_

平台 | 级别 | 备注
---|---|---
windows |  |
linux |  |
macos |  |
android |  |  仅允许通过 `open` 打开 URL
ios |  |  仅允许通过 `open` 打开 URL

## 设置

标题为“设置”的章节

安装 opener 插件以开始使用。

  * 自动
  * 手动

使用你的项目包管理器添加依赖项

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri add opener

    yarn run tauri add opener

    pnpm tauri add opener

    deno task tauri add opener

    bun tauri add opener

    cargo tauri add opener

  1. 在 `src-tauri` 文件夹中运行以下命令，将插件添加到项目的 `Cargo.toml` 依赖项中

         cargo add tauri-plugin-opener

  2. 修改 `lib.rs` 以初始化插件

src-tauri/src/lib.rs

         #[cfg_attr(mobile, tauri::mobile_entry_point)]

         pub fn run() {

             tauri::Builder::default()

                 .plugin(tauri_plugin_opener::init())

                 .run(tauri::generate_context!())

                 .expect("error while running tauri application");

         }

  3. 使用您首选的 JavaScript 包管理器安装 JavaScript 访客绑定

     * npm
     * yarn
     * pnpm
     * deno
     * bun

    npm install @tauri-apps/plugin-opener

    yarn add @tauri-apps/plugin-opener

    pnpm add @tauri-apps/plugin-opener

    deno add npm:@tauri-apps/plugin-opener

    bun add @tauri-apps/plugin-opener

## 用法

名为“用法”的章节

opener 插件同时提供 JavaScript 和 Rust 版本。

  * JavaScript
  * Rust

    import { openPath, openUrl } from '@tauri-apps/plugin-opener';

    // when using `"withGlobalTauri": true`, you may use

    // const { openPath } = window.__TAURI__.opener;

    // opens a file using the default program:

    await openPath('/path/to/file');

    // opens a file using `vlc` command on Windows:

    await openPath('C:/path/to/file', 'vlc');

    // opens a URL using the default program:

    await openUrl('https://tauri.org.cn');

注意，`app` 是 `App` 或 [`AppHandle`](https://docs.rs/tauri/2.0.0/tauri/struct.AppHandle.html) 的实例。

    use tauri_plugin_opener::OpenerExt;

    // opens a file using the default program:

    app.opener().open_path("/path/to/file", None::<&str>);

    // opens a file using `vlc` command on Windows:

    app.opener().open_path("C:/path/to/file", Some("vlc"));

    // opens a URL using the default program:

    app.opener().open_url("https://tauri.org.cn", None::<&str>);

## Permissions

名为“权限”的章节

默认情况下，所有潜在危险的插件命令和范围都被阻止，无法访问。您必须修改 `capabilities` 配置中的权限才能启用这些功能。

有关更多信息，请参阅[功能概述](/security/capabilities/)，并参阅[分步指南](/learn/security/using-plugin-permissions/)以使用插件权限。

以下是两种作用域配置示例。`path` 和 `url` 都使用 [glob 模式语法](https://docs.rs/glob/latest/glob/struct.Pattern.html) 来定义允许的文件路径和 URL。

首先，一个关于如何为 `openPath()` 函数添加特定路径权限的示例

src-tauri/capabilities/default.json

    {

      "$schema": "../gen/schemas/desktop-schema.json",

      "identifier": "main-capability",

      "description": "Capability for the main window",

      "windows": ["main"],

      "permissions": [

        {

          "identifier": "opener:allow-open-path",

          "allow": [

            {

              "path": "/path/to/file"

            },

            {

              "path": "$APPDATA/file"

            }

          ]

        }

      ]

    }

最后，一个关于如何为 `openUrl()` 函数添加特定 `https://tauri.org.cn` URL 以及自定义协议（必须为操作系统所知）下的所有 URL 的权限示例

src-tauri/capabilities/default.json

    {

      "$schema": "../gen/schemas/desktop-schema.json",

      "identifier": "main-capability",

      "description": "Capability for the main window",

      "windows": ["main"],

      "permissions": [

        {

          "identifier": "opener:allow-open-url",

          "allow": [

            {

              "url": "https://tauri.org.cn"

            },

            {

              "url": "custom:*"

            }

          ]

        }

      ]

    }

## 默认权限

此权限集允许使用默认应用程序打开 `mailto:`、`tel:`、`https://` 和 `http://` URL，并支持使用默认文件管理器在目录中显示文件

#### 此默认权限集包括以下内容

  * `allow-open-url`
  * `allow-reveal-item-in-dir`
  * `allow-default-urls`

## 权限表

标识符 | 描述
---|---
`opener:allow-default-urls` |  这使得可以使用默认应用程序打开 `mailto:`、`tel:`、`https://` 和 `http://` URL。
`opener:allow-open-path` |  启用 `open_path` 命令，且无需预配置作用域。
`opener:deny-open-path` |  禁用 `open_path` 命令，且无需预配置作用域。
`opener:allow-open-url` |  启用 `open_url` 命令，且无需预配置作用域。
`opener:deny-open-url` |  禁用 `open_url` 命令，且无需预配置作用域。
`opener:allow-reveal-item-in-dir` |  启用 `reveal_item_in_dir` 命令，且无需预配置作用域。
`opener:deny-reveal-item-in-dir` |  禁用 `reveal_item_in_dir` 命令，且无需预配置作用域。