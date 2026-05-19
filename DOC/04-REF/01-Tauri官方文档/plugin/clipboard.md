# 剪贴板

_Source: https://v2.tauri.org.cn/plugin/clipboard/_

[ GitHub](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/clipboard-manager) [ npm ](https://npmjs.net.cn/package/@tauri-apps/plugin-clipboard-manager) [ crates.io ](https://crates.io/crates/tauri-plugin-clipboard-manager)

API 参考 [ ](/reference/javascript/clipboard-manager/) [ ](https://docs.rs/tauri-plugin-clipboard-manager)

使用剪贴板插件读取和写入系统剪贴板。

## 支持的平台

标题为“支持的平台”的章节

_此插件需要 Rust 版本至少为 **1.77.2**_

平台 | 级别 | 备注
---|---|---
windows |  |
linux |  |
macos |  |
android |  |  仅支持纯文本内容
ios |  |  仅支持纯文本内容

## 设置

标题为“设置”的章节

安装剪贴板插件以开始使用。

  * 自动
  * 手动

使用你的项目包管理器添加依赖项

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri add clipboard-manager

    yarn run tauri add clipboard-manager

    pnpm tauri add clipboard-manager

    deno task tauri add clipboard-manager

    bun tauri add clipboard-manager

    cargo tauri add clipboard-manager

  1. 在 `src-tauri` 文件夹中运行以下命令，将插件添加到项目的 `Cargo.toml` 依赖项中

         cargo add tauri-plugin-clipboard-manager

  2. 修改 `lib.rs` 以初始化插件

src-tauri/src/lib.rs

         #[cfg_attr(mobile, tauri::mobile_entry_point)]

         pub fn run() {

             tauri::Builder::default()

                 .plugin(tauri_plugin_clipboard_manager::init())

                 .run(tauri::generate_context!())

                 .expect("error while running tauri application");

         }

  3. 如果您想在 JavaScript 中管理剪贴板，请同时安装 npm 包。

     * npm
     * yarn
     * pnpm
     * deno
     * bun

    npm install @tauri-apps/plugin-clipboard-manager

    yarn add @tauri-apps/plugin-clipboard-manager

    pnpm add @tauri-apps/plugin-clipboard-manager

    deno add npm:@tauri-apps/plugin-clipboard-manager

    bun add @tauri-apps/plugin-clipboard-manager

## 用法

名为“用法”的章节

剪贴板插件可在 JavaScript 和 Rust 中使用。

  * JavaScript
  * Rust

    import { writeText, readText } from '@tauri-apps/plugin-clipboard-manager';

    // when using `"withGlobalTauri": true`, you may use

    // const { writeText, readText } = window.__TAURI__.clipboardManager;

    // Write content to clipboard

    await writeText('Tauri is awesome!');

    // Read content from clipboard

    const content = await readText();

    console.log(content);

    // Prints "Tauri is awesome!" to the console

    use tauri_plugin_clipboard_manager::ClipboardExt;

    app.clipboard().write_text("Tauri is awesome!".to_string()).unwrap();

    // Read content from clipboard

    let content = app.clipboard().read_text();

    println!("{:?}", content.unwrap());

    // Prints "Tauri is awesome!" to the terminal

## 默认权限

默认情况下不启用任何功能，因为我们认为剪贴板可能存在安全隐患，而且是否需要读/写访问权限取决于具体的应用程序。

需要显式启用剪贴板交互。

## 权限表

标识符 | 描述
---|---
`clipboard-manager:allow-clear` |  在没有任何预配置范围的情况下启用 clear（清除）命令。
`clipboard-manager:deny-clear` |  在没有任何预配置范围的情况下禁用 clear（清除）命令。
`clipboard-manager:allow-read-image` |  在没有任何预配置范围的情况下启用 read_image（读取图像）命令。
`clipboard-manager:deny-read-image` |  在没有任何预配置范围的情况下禁用 read_image（读取图像）命令。
`clipboard-manager:allow-read-text` |  在没有任何预配置范围的情况下启用 read_text（读取文本）命令。
`clipboard-manager:deny-read-text` |  在没有任何预配置范围的情况下禁用 read_text（读取文本）命令。
`clipboard-manager:allow-write-html` |  在没有任何预配置范围的情况下启用 write_html（写入 HTML）命令。
`clipboard-manager:deny-write-html` |  在没有任何预配置范围的情况下禁用 write_html（写入 HTML）命令。
`clipboard-manager:allow-write-image` |  在没有任何预配置范围的情况下启用 write_image（写入图像）命令。
`clipboard-manager:deny-write-image` |  在没有任何预配置范围的情况下禁用 write_image（写入图像）命令。
`clipboard-manager:allow-write-text` |  在没有任何预配置范围的情况下启用 write_text（写入文本）命令。
`clipboard-manager:deny-write-text` |  在没有任何预配置范围的情况下禁用 write_text（写入文本）命令。