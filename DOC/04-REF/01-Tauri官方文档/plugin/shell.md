# Shell

_Source: https://v2.tauri.org.cn/plugin/shell/_

[ GitHub](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/shell) [ npm ](https://npmjs.net.cn/package/@tauri-apps/plugin-shell) [ crates.io ](https://crates.io/crates/tauri-plugin-shell)

API 参考 [ ](/reference/javascript/shell/) [ ](https://docs.rs/tauri-plugin-shell)

访问系统 shell。允许你生成子进程。

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

## 开启器

名为“Opener”的章节

如果你正在查找 `shell.open` API 的文档，请查看新的 [Opener 插件](../opener/)。

## 设置

标题为“设置”的章节

安装 shell 插件以开始使用。

  * 自动
  * 手动

使用你的项目包管理器添加依赖项

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri add shell

    yarn run tauri add shell

    pnpm tauri add shell

    deno task tauri add shell

    bun tauri add shell

    cargo tauri add shell

  1. 在 `src-tauri` 文件夹中运行以下命令，将插件添加到项目的 `Cargo.toml` 依赖项中

         cargo add tauri-plugin-shell

  2. 修改 `lib.rs` 以初始化插件

src-tauri/src/lib.rs

         #[cfg_attr(mobile, tauri::mobile_entry_point)]

         pub fn run() {

             tauri::Builder::default()

                 .plugin(tauri_plugin_shell::init())

                 .run(tauri::generate_context!())

                 .expect("error while running tauri application");

         }

  3. 使用您首选的 JavaScript 包管理器安装 JavaScript 访客绑定

     * npm
     * yarn
     * pnpm
     * deno
     * bun

    npm install @tauri-apps/plugin-shell

    yarn add @tauri-apps/plugin-shell

    pnpm add @tauri-apps/plugin-shell

    deno add npm:@tauri-apps/plugin-shell

    bun add @tauri-apps/plugin-shell

## 用法

名为“用法”的章节

shell 插件支持 JavaScript 和 Rust。

  * JavaScript
  * Rust

    import { Command } from '@tauri-apps/plugin-shell';

    // when using `"withGlobalTauri": true`, you may use

    // const { Command } = window.__TAURI__.shell;

    let result = await Command.create('exec-sh', [

      '-c',

      "echo 'Hello World!'",

    ]).execute();

    console.log(result);

    use tauri_plugin_shell::ShellExt;

    let shell = app_handle.shell();

    let output = tauri::async_runtime::block_on(async move {

        shell

            .command("echo")

            .args(["Hello from Rust!"])

            .output()

            .await

            .unwrap()

    });

    if output.status.success() {

        println!("Result: {:?}", String::from_utf8(output.stdout));

    } else {

        println!("Exit with code: {}", output.status.code().unwrap());

    }

## Permissions

名为“权限”的章节

默认情况下，所有潜在危险的插件命令和范围都被阻止，无法访问。您必须修改 `capabilities` 配置中的权限才能启用这些功能。

有关更多信息，请参阅[功能概述](/security/capabilities/)，并参阅[分步指南](/learn/security/using-plugin-permissions/)以使用插件权限。

src-tauri/capabilities/default.json

    {

      "$schema": "../gen/schemas/desktop-schema.json",

      "identifier": "main-capability",

      "description": "Capability for the main window",

      "windows": ["main"],

      "permissions": [

        {

          "identifier": "shell:allow-execute",

          "allow": [

            {

              "name": "exec-sh",

              "cmd": "sh",

              "args": [

                "-c",

                {

                  "validator": "\\S+"

                }

              ],

              "sidecar": false

            }

          ]

        }

      ]

    }

## 默认权限

此权限集配置默认公开哪些 shell 功能。

#### 已授予权限

它允许使用预配置了合理作用域的 `open` 功能。它将允许打开 `http(s)://`、`tel:` 和 `mailto:` 链接。

#### 此默认权限集包括以下内容

  * `allow-open`

## 权限表

标识符 | 描述
---|---
`shell:allow-execute` |  启用 execute 命令，且无需任何预配置的作用域。
`shell:deny-execute` |  禁用 execute 命令，且无需任何预配置的作用域。
`shell:allow-kill` |  启用 kill 命令，且无需任何预配置的作用域。
`shell:deny-kill` |  禁用 kill 命令，且无需任何预配置的作用域。
`shell:allow-open` |  启用打开命令，不带任何预配置范围。
`shell:deny-open` |  禁用打开命令，不带任何预配置范围。
`shell:allow-spawn` |  启用 spawn 命令，且无需任何预配置的作用域。
`shell:deny-spawn` |  禁用 spawn 命令，且无需任何预配置的作用域。
`shell:allow-stdin-write` |  启用 stdin_write 命令，且无需任何预配置的作用域。
`shell:deny-stdin-write` |  禁用 stdin_write 命令，且无需任何预配置的作用域。