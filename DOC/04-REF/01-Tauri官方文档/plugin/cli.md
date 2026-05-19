# 命令行界面 (CLI)

_Source: https://v2.tauri.org.cn/plugin/cli/_

[ GitHub](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/cli) [ npm ](https://npmjs.net.cn/package/@tauri-apps/plugin-cli) [ crates.io ](https://crates.io/crates/tauri-plugin-cli)

API 参考 [ ](/reference/javascript/cli/) [ ](https://docs.rs/tauri-plugin-cli)

Tauri 通过 [clap](https://github.com/clap-rs/clap)（一个强大的命令行参数解析器）使你的应用能够拥有 CLI。通过在 `tauri.conf.json` 文件中定义简单的 CLI 配置，你可以定义你的接口，并在 JavaScript 和/或 Rust 中读取其参数匹配映射。

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

  * Windows
    * 受限于操作系统，生产环境下的应用默认无法将文本写回调用它的控制台。请查看 [tauri#8305](https://github.com/tauri-apps/tauri/issues/8305#issuecomment-1826871949) 获取解决方案。

## 设置

标题为“设置”的章节

安装 CLI 插件以开始使用。

  * 自动
  * 手动

使用你的项目包管理器添加依赖项

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri add cli

    yarn run tauri add cli

    pnpm tauri add cli

    deno task tauri add cli

    bun tauri add cli

    cargo tauri add cli

  1. 在 `src-tauri` 文件夹中运行以下命令，将插件添加到项目的 `Cargo.toml` 依赖项中

         cargo add tauri-plugin-cli --target 'cfg(any(target_os = "macos", windows, target_os = "linux"))'

     2. 修改 `lib.rs` 以初始化插件

src-tauri/src/lib.rs

    #[cfg_attr(mobile, tauri::mobile_entry_point)]

    pub fn run() {

        tauri::Builder::default()

            .setup(|app| {

                #[cfg(desktop)]

                app.handle().plugin(tauri_plugin_cli::init());

                Ok(())

            })

            .run(tauri::generate_context!())

            .expect("error while running tauri application");

    }

     3. 使用您首选的 JavaScript 包管理器安装 JavaScript 访客绑定

     * npm
     * yarn
     * pnpm
     * deno
     * bun

    npm install @tauri-apps/plugin-cli

    yarn add @tauri-apps/plugin-cli

    pnpm add @tauri-apps/plugin-cli

    deno add npm:@tauri-apps/plugin-cli

    bun add @tauri-apps/plugin-cli

## 基础配置

标题为“基础配置”的章节

在 `tauri.conf.json` 中，你有以下结构来配置该接口

src-tauri/tauri.conf.json

    {

      "plugins": {

        "cli": {

          "description": "Tauri CLI Plugin Example",

          "args": [

            {

              "short": "v",

              "name": "verbose",

              "description": "Verbosity level"

            }

          ],

          "subcommands": {

            "run": {

              "description": "Run the application",

              "args": [

                {

                  "name": "debug",

                  "description": "Run application in debug mode"

                },

                {

                  "name": "release",

                  "description": "Run application in release mode"

                }

              ]

            }

          }

        }

      }

    }

注意

此处所有的 JSON 配置仅为示例，为了清晰起见，省略了许多其他字段。

## 添加参数

标题为“添加参数”的章节

`args` 数组表示其命令或子命令所接受的参数列表。

### 位置参数

标题为“位置参数”的章节

位置参数由其在参数列表中的位置来标识。使用以下配置

src-tauri/tauri.conf.json

    {

      "args": [

        {

          "name": "source",

          "index": 1,

          "takesValue": true

        },

        {

          "name": "destination",

          "index": 2,

          "takesValue": true

        }

      ]

    }

用户可以这样运行你的应用：`./app tauri.txt dest.txt`，此时参数匹配映射会将 `source` 定义为 `"tauri.txt"`，将 `destination` 定义为 `"dest.txt"`。

### 命名参数

标题为“命名参数”的章节

命名参数是一个 (键, 值) 对，其中键标识该值。使用以下配置

tauri-src/tauri.conf.json

    {

      "args": [

        {

          "name": "type",

          "short": "t",

          "takesValue": true,

          "multiple": true,

          "possibleValues": ["foo", "bar"]

        }

      ]

    }

用户可以这样运行你的应用：`./app --type foo bar`、`./app -t foo -t bar` 或 `./app --type=foo,bar`，此时参数匹配映射会将 `type` 定义为 `["foo", "bar"]`。

### 标志参数

标题为“标志参数”的章节

标志参数是一个独立的键，它的出现与否为你的应用提供信息。使用以下配置

tauri-src/tauri.conf.json

    {

      "args": [

        {

          "name": "verbose",

          "short": "v"

        }

      ]

    }

用户可以这样运行你的应用：`./app -v -v -v`、`./app --verbose --verbose --verbose` 或 `./app -vvv`，此时参数匹配映射会将 `verbose` 定义为 `true`，且 `occurrences = 3`。

## 子命令

标题为“子命令”的章节

一些 CLI 应用具有作为子命令的额外接口。例如，`git` CLI 具有 `git branch`、`git commit` 和 `git push`。你可以使用 `subcommands` 数组来定义额外的嵌套接口

tauri-src/tauri.conf.json

    {

      "cli": {

        ...

        "subcommands": {

          "branch": {

            "args": []

          },

          "push": {

            "args": []

          }

        }

      }

    }

其配置与根应用配置相同，包含 `description`、`longDescription`、`args` 等。

## 用法

名为“用法”的章节

CLI 插件可在 JavaScript 和 Rust 中使用。

  * JavaScript
  * Rust

    import { getMatches } from '@tauri-apps/plugin-cli';

    // when using `"withGlobalTauri": true`, you may use

    // const { getMatches } = window.__TAURI__.cli;

    const matches = await getMatches();

    if (matches.subcommand?.name === 'run') {

      // `./your-app run $ARGS` was executed

      const args = matches.subcommand.matches.args;

      if (args.debug?.value === true) {

        // `./your-app run --debug` was executed

      }

      if (args.release?.value === true) {

        // `./your-app run --release` was executed

      }

    }

src-tauri/src/lib.rs

    use tauri_plugin_cli::CliExt;

    #[cfg_attr(mobile, tauri::mobile_entry_point)]

    pub fn run() {

       tauri::Builder::default()

           .plugin(tauri_plugin_cli::init())

           .setup(|app| {

               match app.cli().matches() {

                   // `matches` here is a Struct with { args, subcommand }.

                   // `args` is `HashMap<String, ArgData>` where `ArgData` is a struct with { value, occurrences }.

                   // `subcommand` is `Option<Box<SubcommandMatches>>` where `SubcommandMatches` is a struct with { name, matches }.

                   Ok(matches) => {

                       println!("{:?}", matches)

                   }

                   Err(_) => {}

               }

               Ok(())

           })

           .run(tauri::generate_context!())

           .expect("error while running tauri application");

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

      "permissions": ["cli:default"]

    }

## 默认权限

允许读取 CLI 匹配项

#### 此默认权限集包括以下内容

  * `allow-cli-matches`

## 权限表

标识符 | 描述
---|---
`cli:allow-cli-matches` |  在没有任何预配置作用域的情况下启用 cli_matches 命令。
`cli:deny-cli-matches` |  在没有任何预配置作用域的情况下拒绝 cli_matches 命令。