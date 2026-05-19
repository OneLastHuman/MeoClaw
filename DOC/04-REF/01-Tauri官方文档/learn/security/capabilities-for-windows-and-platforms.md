# 不同窗口和平台的能力

_Source: https://v2.tauri.org.cn/learn/security/capabilities-for-windows-and-platforms/_

本指南将帮助您自定义 Tauri 应用的权限。

## 本指南内容

名为“本指南内容”的章节

  * 在 Tauri 应用中创建多个窗口
  * 为不同窗口使用不同的权限
  * 使用特定于平台的权限

## 先决条件

标题为“前提条件”的章节

本练习建议在完成 [`使用插件权限`](/learn/security/using-plugin-permissions/) 后阅读。

## 指南

名为“指南”的章节

  1. ### 在 Tauri 应用程序中创建多个窗口

名为“在 Tauri 应用程序中创建多个窗口”的章节

这里我们创建一个带有两个窗口的应用程序，分别标记为 `first` 和 `second`。在 Tauri 应用程序中创建窗口有多种方法。

#### 使用 Tauri 配置文件创建窗口

名为“使用 Tauri 配置文件创建窗口”的章节

在 Tauri 配置文件中（通常命名为 `tauri.conf.json`）

显示解决方案

         "productName": "multiwindow",

           ...

           "app": {

             "windows": [

               {

                 "label": "first",

                 "title": "First",

                 "width": 800,

                 "height": 600

               },

               {

                 "label": "second",

                 "title": "Second",

                 "width": 800,

                 "height": 600

               }

             ],

           },

           ...

         }

#### 以编程方式创建窗口

名为“以编程方式创建窗口”的章节

在用于创建 Tauri 应用的 Rust 代码中

显示解决方案

         tauri::Builder::default()

             .invoke_handler(tauri::generate_handler![greet])

             .setup(|app| {

                 let webview_url = tauri::WebviewUrl::App("index.html".into());

                 // First window

                 tauri::WebviewWindowBuilder::new(app, "first", webview_url.clone())

                     .title("First")

                     .build()?;

                 // Second window

                 tauri::WebviewWindowBuilder::new(app, "second", webview_url)

                     .title("Second")

                     .build()?;

                 Ok(())

             })

             .run(context)

             .expect("error while running tauri application");

  2. ### 为不同窗口应用不同的权限

名为“为不同窗口应用不同的权限”的章节

Tauri 应用的窗口可以使用 Tauri 后端的不同功能或插件。为了获得更好的安全性，建议仅向每个窗口授予必要的权限。我们模拟一个场景：`first` 窗口使用文件系统和对话框功能，而 `second` 窗口仅需要对话框功能。

#### 按类别分离权限文件

名为“按类别分离权限文件”的章节

建议根据权限所启用的操作类别，将权限文件分开存放。

显示解决方案

`src-tauri/capabilities` 目录下的 JSON 文件会被权限系统纳入考虑。在此，我们将文件系统和对话框窗口相关的权限分别分离到 `filesystem.json` 和 `dialog.json` 中。

 _Tauri 项目的文件树_

         /src

         /src-tauri

           /capabilities

             filesystem.json

             dialog.json

           tauri.conf.json

         package.json

         README.md

#### 为 `first` 窗口授予文件系统权限

名为“为 first 窗口授予文件系统权限”的章节

我们授予 `first` 窗口读取 `$HOME` 目录内容的权限。

显示解决方案

在权限文件中使用 `windows` 字段，并指定一个或多个窗口标签。

filesystem.json

         {

           "identifier": "fs-read-home",

           "description": "Allow access file access to home directory",

           "local": true,

           "windows": ["first"],

           "permissions": [

             "fs:allow-home-read",

           ]

         }

#### 为 `first` 和 `second` 窗口授予对话框权限

名为“为 first 和 second 窗口授予对话框权限”的章节

我们授予 `first` 和 `second` 窗口创建“是/否”对话框的权限。

显示解决方案

在权限文件中使用 `windows` 字段，并指定一个或多个窗口标签。

dialog.json

         {

           "identifier": "dialog",

           "description": "Allow to open a dialog",

           "local": true,

           "windows": ["first", "second"],

           "permissions": ["dialog:allow-ask"]

         }

  3. ### 设置平台相关的权限

名为“设置平台相关的权限”的章节

现在我们想要自定义权限，使其仅在特定平台上生效。我们将文件系统权限配置为仅在 `linux` 和 `windows` 上生效。

显示解决方案

在权限文件中使用 `platforms` 字段来设置平台相关的权限。

filesystem.json

         {

           "identifier": "fs-read-home",

           "description": "Allow access file access to home directory",

           "local": true,

           "windows": ["first"],

           "permissions": [

             "fs:allow-home-read",

           ],

           "platforms": ["linux", "windows"]

         }

当前可用的平台有 `linux`、`windows`、`macos`、`android` 和 `ios`。

## 结论与资源

名为“结论与资源”的章节

我们学习了如何在 Tauri 应用中创建多个窗口并授予它们特定的权限。此外，这些权限还可以针对特定平台进行设置。

一个使用窗口权限的示例应用程序可以在 [`api` 示例](https://github.com/tauri-apps/tauri/tree/dev/examples/api)（位于 [Tauri Github 仓库](https://github.com/tauri-apps/tauri)中）中找到。权限文件中可使用的字段列表，请参考 [Capability（权限）](/reference/acl/capability/)参考文档。