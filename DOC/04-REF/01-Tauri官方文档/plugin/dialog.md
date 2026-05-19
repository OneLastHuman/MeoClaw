# 对话框

_Source: https://v2.tauri.org.cn/plugin/dialog/_

[ GitHub](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/dialog) [ npm ](https://npmjs.net.cn/package/@tauri-apps/plugin-dialog) [ crates.io ](https://crates.io/crates/tauri-plugin-dialog)

API 参考 [ ](/reference/javascript/dialog/) [ ](https://docs.rs/tauri-plugin-dialog)

用于打开和保存文件以及消息对话框的原生系统对话框。

## 支持的平台

标题为“支持的平台”的章节

_此插件需要 Rust 版本至少为 **1.77.2**_

平台 | 级别 | 备注
---|---|---
windows |  |
linux |  |
macos |  |
android |  |  不支持文件夹选择器
ios |  |  不支持文件夹选择器

## 设置

标题为“设置”的章节

安装 dialog 插件以开始使用。

  * 自动
  * 手动

使用你的项目包管理器添加依赖项

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri add dialog

    yarn run tauri add dialog

    pnpm tauri add dialog

    deno task tauri add dialog

    bun tauri add dialog

    cargo tauri add dialog

  1. 在 `src-tauri` 文件夹中运行以下命令，将插件添加到项目的 `Cargo.toml` 依赖项中

         cargo add tauri-plugin-dialog

  2. 修改 `lib.rs` 以初始化插件

src-tauri/src/lib.rs

         #[cfg_attr(mobile, tauri::mobile_entry_point)]

         pub fn run() {

             tauri::Builder::default()

                 .plugin(tauri_plugin_dialog::init())

                 .run(tauri::generate_context!())

                 .expect("error while running tauri application");

         }

  3. 如果您想在 JavaScript 中创建对话框，也请安装 npm 包

     * npm
     * yarn
     * pnpm
     * deno
     * bun

    npm install @tauri-apps/plugin-dialog

    yarn add @tauri-apps/plugin-dialog

    pnpm add @tauri-apps/plugin-dialog

    deno add npm:@tauri-apps/plugin-dialog

    bun add @tauri-apps/plugin-dialog

## 用法

名为“用法”的章节

dialog 插件同时支持 JavaScript 和 Rust。以下是使用方法：

在 JavaScript 中

  * 创建“是/否”对话框
  * 创建“确定/取消”对话框
  * 创建消息对话框
  * 打开文件选择对话框
  * 保存文件对话框

在 Rust 中

  * 构建询问对话框
  * 构建消息对话框
  * 构建文件选择对话框

注意

文件对话框 API 在 Linux、Windows 和 macOS 上返回文件系统路径。

在 iOS 上，返回 `file://<path>` URI。

在 Android 上，返回 [内容 URI](https://developer.android.com.cn/guide/topics/providers/content-provider-basics)。

[文件系统插件](/plugin/file-system/) 可直接处理任何路径格式。

### JavaScript

名为“JavaScript”的章节

请参阅 JavaScript API 参考中所有的 [对话框选项](/reference/javascript/dialog/)。

#### 创建“是/否”对话框

章节标题“创建“是/否”对话框”

显示一个带有 `是` 和 `否` 按钮的询问对话框。

    import { ask } from '@tauri-apps/plugin-dialog';

    // when using `"withGlobalTauri": true`, you may use

    // const { ask } = window.__TAURI__.dialog;

    // Create a Yes/No dialog

    const answer = await ask('This action cannot be reverted. Are you sure?', {

      title: 'Tauri',

      kind: 'warning',

    });

    console.log(answer);

    // Prints boolean to the console

#### 创建“确定/取消”对话框

章节标题“创建“确定/取消”对话框”

显示一个带有 `确定` 和 `取消` 按钮的询问对话框。

    import { confirm } from '@tauri-apps/plugin-dialog';

    // when using `"withGlobalTauri": true`, you may use

    // const { confirm } = window.__TAURI__.dialog;

    // Creates a confirmation Ok/Cancel dialog

    const confirmation = await confirm(

      'This action cannot be reverted. Are you sure?',

      { title: 'Tauri', kind: 'warning' }

    );

    console.log(confirmation);

    // Prints boolean to the console

#### 创建消息对话框

章节标题“创建消息对话框”

显示一个带有 `确定` 按钮的消息对话框。请记住，如果用户关闭对话框，它将返回 `false`。

    import { message } from '@tauri-apps/plugin-dialog';

    // when using `"withGlobalTauri": true`, you may use

    // const { message } = window.__TAURI__.dialog;

    // Shows message

    await message('File not found', { title: 'Tauri', kind: 'error' });

#### 打开文件选择对话框

章节标题“打开文件选择对话框”

打开文件/目录选择对话框。

`multiple` 选项用于控制对话框是否允许进行多选，而 `directory` 则控制是否为目录选择。

    import { open } from '@tauri-apps/plugin-dialog';

    // when using `"withGlobalTauri": true`, you may use

    // const { open } = window.__TAURI__.dialog;

    // Open a dialog

    const file = await open({

      multiple: false,

      directory: false,

    });

    console.log(file);

    // Prints file path or URI

#### 保存文件对话框

章节标题“保存文件对话框”

打开文件/目录保存对话框。

    import { save } from '@tauri-apps/plugin-dialog';

    // when using `"withGlobalTauri": true`, you may use

    // const { save } = window.__TAURI__.dialog;

    // Prompt to save a 'My Filter' with extension .png or .jpeg

    const path = await save({

      filters: [

        {

          name: 'My Filter',

          extensions: ['png', 'jpeg'],

        },

      ],

    });

    console.log(path);

    // Prints the chosen path

* * *

### Rust

名为“Rust”的章节

请参考 [Rust API 参考](https://docs.rs/tauri-plugin-dialog/) 以查看所有可用选项。

#### 构建询问对话框

章节标题“构建询问对话框”

显示一个带有 `绝对是` 和 `完全是` 按钮的询问对话框。

    use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};

    let answer = app.dialog()

            .message("Tauri is Awesome")

            .title("Tauri is Awesome")

            .buttons(MessageDialogButtons::OkCancelCustom("Absolutely", "Totally"))

            .blocking_show();

如果您需要非阻塞操作，可以使用 `show()`

    use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};

    app.dialog()

        .message("Tauri is Awesome")

        .title("Tauri is Awesome")

       .buttons(MessageDialogButtons::OkCancelCustom("Absolutely", "Totally"))

        .show(|result| match result {

            true => // do something,

            false =>// do something,

        });

#### 构建消息对话框

章节标题“构建消息对话框”

显示一个带有 `确定` 按钮的消息对话框。请记住，如果用户关闭对话框，它将返回 `false`。

    use tauri_plugin_dialog::{DialogExt, MessageDialogKind};

    let ans = app.dialog()

        .message("File not found")

        .kind(MessageDialogKind::Error)

        .title("Warning")

        .blocking_show();

如果您需要非阻塞操作，可以使用 `show()`

    use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};

    app.dialog()

        .message("Tauri is Awesome")

        .kind(MessageDialogKind::Info)

        .title("Information")

        .buttons(MessageDialogButtons::OkCustom("Absolutely"))

        .show(|result| match result {

            true => // do something,

            false => // do something,

        });

#### 构建文件选择对话框

章节标题“构建文件选择对话框”

#### 选择文件

章节标题“选择文件”

    use tauri_plugin_dialog::DialogExt;

    let file_path = app.dialog().file().blocking_pick_file();

    // return a file_path `Option`, or `None` if the user closes the dialog

如果您需要非阻塞操作，可以使用 `pick_file()`

    use tauri_plugin_dialog::DialogExt;

    app.dialog().file().pick_file(|file_path| {

        // return a file_path `Option`, or `None` if the user closes the dialog

        })

#### 保存文件

章节标题“保存文件”

    use tauri_plugin_dialog::DialogExt;

    let file_path = app

        .dialog()

        .file()

        .add_filter("My Filter", &["png", "jpeg"])

        .blocking_save_file();

        // do something with the optional file path here

        // the file path is `None` if the user closed the dialog

或者，也可以

    use tauri_plugin_dialog::DialogExt;

    app.dialog()

        .file()

        .add_filter("My Filter", &["png", "jpeg"])

        .pick_file(|file_path| {

            // return a file_path `Option`, or `None` if the user closes the dialog

        });

## 默认权限

此权限集用于配置 dialog 插件可用的对话框类型。

#### 已授予权限

所有对话框类型均已启用。

#### 此默认权限集包括以下内容

  * `allow-message`
  * `允许保存`
  * `allow-open`

## 权限表

标识符 | 描述
---|---
`dialog:allow-ask` |  在没有预先配置范围的情况下启用 ask 命令。（**已弃用** ：现在这是 `allow-message` 的别名，将在 v3 中删除）
`dialog:deny-ask` |  在没有预先配置范围的情况下禁止 ask 命令。（**已弃用** ：现在这是 `deny-message` 的别名，将在 v3 中删除）
`dialog:allow-message` |  在没有预先配置范围的情况下启用 message 命令。
`dialog:deny-message` |  在没有预先配置范围的情况下禁止 message 命令。
`dialog:allow-open` |  启用打开命令，不带任何预配置范围。
`dialog:deny-open` |  禁用打开命令，不带任何预配置范围。
`dialog:allow-save` |  启用保存命令，不带任何预配置范围。
`dialog:deny-save` |  禁用保存命令，不带任何预配置范围。
`dialog:allow-confirm` |  在没有预先配置范围的情况下启用 confirm 命令。（**已弃用** ：现在这是 `allow-message` 的别名，将在 v3 中删除）
`dialog:deny-confirm` |  在没有预先配置范围的情况下禁止 confirm 命令。（**已弃用** ：现在这是 `deny-message` 的别名，将在 v3 中删除）