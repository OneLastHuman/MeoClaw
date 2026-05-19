# 日志

_Source: https://v2.tauri.org.cn/plugin/logging/_

[ GitHub](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/log) [ npm ](https://npmjs.net.cn/package/@tauri-apps/plugin-log) [ crates.io ](https://crates.io/crates/tauri-plugin-log)

API 参考 [ ](/reference/javascript/log/) [ ](https://docs.rs/tauri-plugin-log)

为您的 Tauri 应用提供可配置的日志记录。

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

安装日志插件以开始使用。

  * 自动
  * 手动

使用你的项目包管理器添加依赖项

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri add log

    yarn run tauri add log

    pnpm tauri add log

    deno task tauri add log

    bun tauri add log

    cargo tauri add log

  1. 在 `src-tauri` 文件夹中运行以下命令，将插件添加到项目的 `Cargo.toml` 依赖项中

         cargo add tauri-plugin-log

  2. 修改 `lib.rs` 以初始化插件

src-tauri/src/lib.rs

         #[cfg_attr(mobile, tauri::mobile_entry_point)]

         pub fn run() {

             tauri::Builder::default()

                 .plugin(tauri_plugin_log::Builder::new().build())

                 .run(tauri::generate_context!())

                 .expect("error while running tauri application");

         }

  3. 使用您首选的 JavaScript 包管理器安装 JavaScript 访客绑定

     * npm
     * yarn
     * pnpm
     * deno
     * bun

    npm install @tauri-apps/plugin-log

    yarn add @tauri-apps/plugin-log

    pnpm add @tauri-apps/plugin-log

    deno add npm:@tauri-apps/plugin-log

    bun add @tauri-apps/plugin-log

## 用法

名为“用法”的章节

  1. 首先，您需要在 Tauri 中注册该插件。

src-tauri/src/lib.rs

         use tauri_plugin_log::{Target, TargetKind};

         #[cfg_attr(mobile, tauri::mobile_entry_point)]

         pub fn run() {

             tauri::Builder::default()

                 .plugin(tauri_plugin_log::Builder::new().build())

                 .run(tauri::generate_context!())

                 .expect("error while running tauri application");

         }

  2. 之后，所有的插件 API 都可以通过 JavaScript 客体绑定（guest bindings）进行访问。

         import {

           warn,

           debug,

           trace,

           info,

           error,

           attachConsole,

           attachLogger,

         } from '@tauri-apps/plugin-log';

         // when using `"withGlobalTauri": true`, you may use

         // const { warn, debug, trace, info, error, attachConsole, attachLogger } = window.__TAURI__.log;

## 日志

名为“日志记录”的章节

  * JavaScript
  * Rust

使用插件的 `warn`、`debug`、`trace`、`info` 或 `error` API 中的任意一个，即可从 JavaScript 代码生成日志记录。

    import { warn, debug, trace, info, error } from '@tauri-apps/plugin-log';

    trace('Trace');

    info('Info');

    error('Error');

要将所有 `console` 消息自动转发到日志插件，您可以重写它们。

    import { warn, debug, trace, info, error } from '@tauri-apps/plugin-log';

    function forwardConsole(

      fnName: 'log' | 'debug' | 'info' | 'warn' | 'error',

      logger: (message: string) => Promise<void>

    ) {

      const original = console[fnName];

      console[fnName] = (message) => {

        original(message);

        logger(message);

      };

    }

    forwardConsole('log', trace);

    forwardConsole('debug', debug);

    forwardConsole('info', info);

    forwardConsole('warn', warn);

    forwardConsole('error', error);

要在 Rust 端创建自己的日志，可以使用 [`log` crate](https://crates.io/crates/log)。

    log::error!("something bad happened!");

    log::info!("Tauri is awesome!");

请注意，必须将 [`log` crate](https://crates.io/crates/log) 添加到您的 `Cargo.toml` 文件中。

    [dependencies]

    log = "0.4"

## 日志目标

名为“日志目标”的章节

日志插件构建器有一个 `targets` 函数，允许您配置应用程序所有日志的通用目标路径。

注意

默认情况下，插件会将日志记录到标准输出（stdout）以及应用程序日志目录中的文件中。要仅使用您自己的日志目标，请调用 `clear_targets`。

    tauri_plugin_log::Builder::new()

    .clear_targets()

    .build()

### 将日志打印到终端

名为“将日志打印到终端”的章节

要将所有日志转发到终端，请启用 `Stdout` 或 `Stderr` 目标。

    tauri_plugin_log::Builder::new()

      .target(tauri_plugin_log::Target::new(

        tauri_plugin_log::TargetKind::Stdout,

      ))

      .build()

此目标默认处于启用状态。

### 记录到 Webview 控制台

名为“记录到 Webview 控制台”的章节

要在 Webview 控制台中查看所有 Rust 日志，请启用 `Webview` 目标，并在您的前端运行 `attachConsole`。

    tauri_plugin_log::Builder::new()

      .target(tauri_plugin_log::Target::new(

        tauri_plugin_log::TargetKind::Webview,

      ))

      .build()

    import { attachConsole } from '@tauri-apps/plugin-log';

    const detach = await attachConsole();

    // call detach() if you do not want to print logs to the console anymore

### 持久化日志

名为“持久化日志”的章节

要将所有日志写入文件，您可以使用 `LogDir` 或 `Folder` 目标。

  * `LogDir`:

    tauri_plugin_log::Builder::new()

      .target(tauri_plugin_log::Target::new(

        tauri_plugin_log::TargetKind::LogDir {

          file_name: Some("logs".to_string()),

        },

      ))

      .build()

使用 LogDir 目标时，所有日志都会存储在推荐的日志目录中。下表描述了不同平台下的日志存放位置：

平台| 值| 示例
---|---|---
Linux| `$XDG_DATA_HOME/{bundleIdentifier}/logs` 或 `$HOME/.local/share/{bundleIdentifier}/logs`| `/home/alice/.local/share/com.tauri.dev/logs`
macOS| `{homeDir}/Library/Logs/{bundleIdentifier}`| `/Users/Alice/Library/Logs/com.tauri.dev`
Windows| `{FOLDERID_LocalAppData}/{bundleIdentifier}/logs`| `C:\Users\Alice\AppData\Local\com.tauri.dev\logs`

  * `Folder`:

Folder 目标允许您将日志写入文件系统中的自定义位置。

    tauri_plugin_log::Builder::new()

      .target(tauri_plugin_log::Target::new(

        tauri_plugin_log::TargetKind::Folder {

          path: std::path::PathBuf::from("/path/to/logs"),

          file_name: None,

        },

      ))

      .build()

默认的 `file_name`（文件名）为应用程序名称。

#### 配置日志文件行为

名为“配置日志文件行为”的章节

默认情况下，当日志文件达到最大大小时，会被丢弃。最大文件大小可以通过构建器的 `max_file_size` 函数进行配置。

    tauri_plugin_log::Builder::new()

      .max_file_size(50_000 /* bytes */)

      .build()

Tauri 可以在日志文件达到大小限制时自动进行轮换（rotate），而不是直接丢弃旧文件。此行为可以使用 `rotation_strategy` 进行配置。

    tauri_plugin_log::Builder::new()

      .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepAll)

      .build()

### 过滤

名为“过滤”的章节

默认情况下，**所有** 日志都会被处理。可以通过一些机制来减少日志量，只过滤出相关的信息。

### 最大日志级别

名为“最大日志级别”的章节

要设置最大日志级别，请使用 `level` 函数。

    tauri_plugin_log::Builder::new()

      .level(log::LevelFilter::Info)

      .build()

在此示例中，debug 和 trace 日志会被丢弃，因为它们的级别低于 _info_ 。

也可以为各个模块定义单独的最大级别。

    tauri_plugin_log::Builder::new()

      .level(log::LevelFilter::Info)

      // verbose logs only for the commands module

      .level_for("my_crate_name::commands", log::LevelFilter::Trace)

      .build()

请注意，这些 API 使用了 [`log` crate](https://crates.io/crates/log)，必须将其添加到您的 `Cargo.toml` 文件中。

    [dependencies]

    log = "0.4"

### 目标过滤器

名为“目标过滤器”的章节

可以通过定义 `filter` 函数来检查日志的元数据，从而丢弃不需要的日志。

    tauri_plugin_log::Builder::new()

      // exclude logs with target `"hyper"`

      .filter(|metadata| metadata.target() != "hyper")

      .build()

### 格式化

名为“格式化”的章节

日志插件将每个日志记录格式化为 `DATE[TARGET][LEVEL] MESSAGE`。可以使用 `format` 提供自定义格式函数。

    tauri_plugin_log::Builder::new()

      .format(|out, message, record| {

        out.finish(format_args!(

          "[{} {}] {}",

          record.level(),

          record.target(),

          message

        ))

      })

      .build()

#### 日志日期

名为“日志日期”的章节

默认情况下，日志插件使用 UTC 时区来格式化日期，但您可以配置它使用本地时区，通过 `timezone_strategy`。

    tauri_plugin_log::Builder::new()

      .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal)

      .build()

## Permissions

名为“权限”的章节

默认情况下，所有插件命令都是阻塞的，无法直接访问。您必须在 `capabilities` 配置中定义权限列表。

有关更多信息，请参阅[功能概述](/security/capabilities/)，并参阅[分步指南](/learn/security/using-plugin-permissions/)以使用插件权限。

src-tauri/capabilities/default.json

    {

      "$schema": "../gen/schemas/desktop-schema.json",

      "identifier": "main-capability",

      "description": "Capability for the main window",

      "windows": ["main"],

      "permissions": ["log:default"]

    }

## 默认权限

允许 log 命令

#### 此默认权限集包括以下内容

  * `allow-log`

## 权限表

标识符 | 描述
---|---
`log:allow-log` |  在没有任何预配置作用域的情况下启用 log 命令。
`log:deny-log` |  在没有任何预配置作用域的情况下禁用 log 命令。