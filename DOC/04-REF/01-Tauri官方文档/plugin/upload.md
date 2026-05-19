# 上传

_Source: https://v2.tauri.org.cn/plugin/upload/_

[ GitHub](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/upload) [ npm ](https://npmjs.net.cn/package/@tauri-apps/plugin-upload) [ crates.io ](https://crates.io/crates/tauri-plugin-upload)

API 参考 [ ](/reference/javascript/upload/) [ ](https://docs.rs/tauri-plugin-upload)

通过 HTTP 将文件从磁盘上传到远程服务器。将文件从远程 HTTP 服务器下载到磁盘。

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

  * 自动
  * 手动

使用你的项目包管理器添加依赖项

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri add upload

    yarn run tauri add upload

    pnpm tauri add upload

    deno task tauri add upload

    bun tauri add upload

    cargo tauri add upload

  1. 在 `src-tauri` 文件夹中运行以下命令，将插件添加到项目的 `Cargo.toml` 依赖项中

         cargo add tauri-plugin-upload

  2. 修改 `lib.rs` 以初始化插件

src-tauri/src/lib.rs

         #[cfg_attr(mobile, tauri::mobile_entry_point)]

         pub fn run() {

           tauri::Builder::default()

             .plugin(tauri_plugin_upload::init())

               .run(tauri::generate_context!())

               .expect("error while running tauri application");

         }

  3. 使用您首选的 JavaScript 包管理器安装 JavaScript 访客绑定

     * npm
     * yarn
     * pnpm
     * deno
     * bun

    npm install @tauri-apps/plugin-upload

    yarn add @tauri-apps/plugin-upload

    pnpm add @tauri-apps/plugin-upload

    deno add npm:@tauri-apps/plugin-upload

    bun add @tauri-apps/plugin-upload

## 用法

名为“用法”的章节

一旦完成了插件的注册和设置过程，你就可以通过 JavaScript 客端绑定访问其所有 API。

以下是如何使用该插件上传和下载文件的示例

    import { upload } from '@tauri-apps/plugin-upload';

    // when using `"withGlobalTauri": true`, you may use

    // const { upload } = window.__TAURI__.upload;

    upload(

      'https://example.com/file-upload',

      './path/to/my/file.txt',

      ({ progress, total }) =>

        console.log(`Uploaded ${progress} of ${total} bytes`), // a callback that will be called with the upload progress

      { 'Content-Type': 'text/plain' } // optional headers to send with the request

    );

    import { download } from '@tauri-apps/plugin-upload';

    // when using `"withGlobalTauri": true`, you may use

    // const { download } = window.__TAURI__.upload;

    download(

      'https://example.com/file-download-link',

      './path/to/save/my/file.txt',

      ({ progress, total }) =>

        console.log(`Downloaded ${progress} of ${total} bytes`), // a callback that will be called with the download progress

      { 'Content-Type': 'text/plain' } // optional headers to send with the request

    );

## Permissions

名为“权限”的章节

默认情况下，所有潜在危险的插件命令和范围都被阻止，无法访问。您必须修改 `capabilities` 配置中的权限才能启用这些功能。

有关更多信息，请参阅[功能概述](/security/capabilities/)，并参阅[分步指南](/learn/security/using-plugin-permissions/)以使用插件权限。

src-tauri/capabilities/default.json

    {

      "permissions": [

        ...,

        "upload:default",

      ]

    }

## 默认权限

此权限集配置了上传插件可用的操作类型。

#### 已授予权限

默认情况下，所有操作均已启用。

#### 此默认权限集包括以下内容

  * `allow-upload`
  * `allow-download`

## 权限表

标识符 | 描述
---|---
`upload:allow-download` |  启用下载命令，且不带任何预配置的作用域。
`upload:deny-download` |  拒绝下载命令，且不带任何预配置的作用域。
`upload:allow-upload` |  启用上传命令，且不带任何预配置的作用域。
`upload:deny-upload` |  拒绝上传命令，且不带任何预配置的作用域。