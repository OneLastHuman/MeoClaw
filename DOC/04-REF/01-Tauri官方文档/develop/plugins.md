# 插件开发

_Source: https://v2.tauri.org.cn/develop/plugins/_

插件开发

本指南适用于 Tauri 插件开发。如果您正在寻找当前可用插件的列表及其使用方法，请访问 [功能与指南列表](/plugin/)。

插件能够挂载到 Tauri 的生命周期中，公开依赖于 WebView API 的 Rust 代码，使用 Rust、Kotlin 或 Swift 代码处理命令，以及更多功能。

Tauri 提供了一个带有 WebView 功能的窗口系统，一种在 Rust 进程和 WebView 之间发送消息的方式，以及一个事件系统，还有多种增强开发体验的工具。Tauri 核心的设计理念是不包含并非人人所需的功能。相反，它提供了一种称为“插件”的机制，用于将外部功能添加到 Tauri 应用程序中。

Tauri 插件由一个 Cargo crate 和一个可选的 NPM 包组成，该包为其命令和事件提供 API 绑定。此外，插件项目还可以包含一个 Android 库项目和一个用于 iOS 的 Swift 包。您可以在 [移动端插件开发指南](/develop/plugins/develop-mobile/) 中了解更多关于开发 Android 和 iOS 插件的信息。

## 命名约定

章节标题 “命名约定”

Tauri 插件拥有一个前缀，后面紧跟插件名称。插件名称是在 [`tauri.conf.json > plugins`](/reference/config/#pluginconfig) 配置项下指定的。

默认情况下，Tauri 会为您的插件 crate 添加 `tauri-plugin-` 前缀。这有助于 Tauri 社区发现您的插件，并配合 Tauri CLI 使用。在初始化新插件项目时，您必须提供其名称。生成的 crate 名称将为 `tauri-plugin-{plugin-name}`，JavaScript NPM 包名称将为 `tauri-plugin-{plugin-name}-api`（尽管我们建议尽可能使用 [NPM scope](https://docs.npmjs.net.cn/about-scopes)）。Tauri 的 NPM 包命名约定为 `@scope-name/plugin-{plugin-name}`。

## 初始化插件项目

章节标题 “初始化插件项目”

要引导一个新插件项目，请运行 `plugin new`。如果您不需要 NPM 包，请使用 `--no-api` CLI 标志。如果您想在初始化插件时支持 Android 和/或 iOS，请使用 `--android` 和/或 `--ios` 标志。

安装后，您可以运行以下命令来创建插件项目

  * npm

    npx @tauri-apps/cli plugin new [name]

这将在 `tauri-plugin-[name]` 目录下初始化插件，根据使用的 CLI 标志，生成的项目结构如下所示

    . tauri-plugin-[name]/

    ├── src/                - Rust code

    │ ├── commands.rs       - Defines the commands the webview can use

    | ├── desktop.rs        - Desktop implementation

    | ├── error.rs          - Default error type to use in returned results

    │ ├── lib.rs            - Re-exports appropriate implementation, setup state...

    │ ├── mobile.rs         - Mobile implementation

    │ └── models.rs         - Shared structs

    ├── permissions/        - This will host (generated) permission files for commands

    ├── android             - Android library

    ├── ios                 - Swift package

    ├── guest-js            - Source code of the JavaScript API bindings

    ├── dist-js             - Transpiled assets from guest-js

    ├── Cargo.toml          - Cargo crate metadata

    └── package.json        - NPM package metadata

如果您已经有一个插件，并希望为其添加 Android 或 iOS 功能，可以使用 `plugin android add` 和 `plugin ios add` 来引导移动端库项目，并引导您完成所需的更改。

## 移动端插件开发

章节标题 “移动端插件开发”

插件可以运行使用 Kotlin（或 Java）和 Swift 编写的原生移动端代码。默认的插件模板包含一个使用 Kotlin 的 Android 库项目和一个 Swift 包。它包含一个示例移动端命令，展示了如何从 Rust 代码触发其执行。

在 [移动端插件开发指南](/develop/plugins/develop-mobile/) 中阅读更多关于开发移动端插件的内容。

## 插件配置

章节标题 “插件配置”

在通过使用该插件的 Tauri 应用程序中，插件配置在 `tauri.conf.json` 中指定，其中 `plugin-name` 是插件的名称

    {

      "build": { ... },

      "tauri": { ... },

      "plugins": {

        "plugin-name": {

          "timeout": 30

        }

      }

    }

插件的配置设置在 `Builder` 上，并在运行时解析。以下是使用 `Config` 结构体来指定插件配置的示例

src/lib.rs

    use serde::Deserialize;

    use tauri::{

        plugin::{Builder, TauriPlugin},

        Runtime,

    };

    // Define the plugin config

    #[derive(Deserialize)]

    pub struct Config {

      timeout: usize,

    }

    pub fn init<R: Runtime>() -> TauriPlugin<R, Config> {

      // Make the plugin config optional

      // by using `Builder::<R, Option<Config>>` instead

      Builder::<R, Config>::new("<plugin-name>")

        .setup(|app, api| {

          let timeout = api.config().timeout;

          Ok(())

        })

        .build()

    }

## 生命周期事件

章节标题 “生命周期事件”

插件可以挂载到多个生命周期事件中

  * setup: 插件正在初始化
  * on_navigation: WebView 正在尝试执行导航
  * on_webview_ready: 新窗口正在创建
  * on_event: 事件循环事件
  * on_drop: 插件正在被解构

还有其他用于 [移动端插件的生命周期事件](/develop/plugins/develop-mobile/#lifecycle-events)。

### setup

章节标题 “setup”

  * **何时** ：插件正在初始化时
  * **用途** ：注册移动端插件、管理状态、运行后台任务

src/lib.rs

    use tauri::{Manager, plugin::Builder};

    use std::{collections::HashMap, sync::Mutex, time::Duration};

    struct DummyStore(Mutex<HashMap<String, String>>);

    Builder::new("<plugin-name>")

      .setup(|app, api| {

        app.manage(DummyStore(Default::default()));

        let app_ = app.clone();

        std::thread::spawn(move || {

          loop {

            app_.emit("tick", ());

            std::thread::sleep(Duration::from_secs(1));

          }

        });

        Ok(())

      })

### on_navigation

章节标题 “on_navigation”

  * **何时** ：WebView 正在尝试执行导航时
  * **用途** ：验证导航或跟踪 URL 更改

返回 `false` 将取消导航。

src/lib.rs

    use tauri::plugin::Builder;

    Builder::new("<plugin-name>")

      .on_navigation(|window, url| {

        println!("window {} is navigating to {}", window.label(), url);

        // Cancels the navigation if forbidden

        url.scheme() != "forbidden"

      })

### on_webview_ready

章节标题 “on_webview_ready”

  * **何时** ：新窗口已创建时
  * **用途** ：为每个窗口执行初始化脚本

src/lib.rs

    use tauri::plugin::Builder;

    Builder::new("<plugin-name>")

      .on_webview_ready(|window| {

        window.listen("content-loaded", |event| {

          println!("webview content has been loaded");

        });

      })

### on_event

章节标题 “on_event”

  * **何时** ：事件循环事件发生时
  * **用途** ：处理核心事件，例如窗口事件、菜单事件和应用程序退出请求

通过此生命周期钩子，您可以获取任何事件循环 [事件](https://docs.rs/tauri/2.0.0/tauri/enum.RunEvent.html) 的通知。

src/lib.rs

    use std::{collections::HashMap, fs::write, sync::Mutex};

    use tauri::{plugin::Builder, Manager, RunEvent};

    struct DummyStore(Mutex<HashMap<String, String>>);

    Builder::new("<plugin-name>")

      .setup(|app, _api| {

        app.manage(DummyStore(Default::default()));

        Ok(())

      })

      .on_event(|app, event| {

        match event {

          RunEvent::ExitRequested { api, .. } => {

            // user requested a window to be closed and there's no windows left

            // we can prevent the app from exiting:

            api.prevent_exit();

          }

          RunEvent::Exit => {

            // app is going to exit, you can cleanup here

            let store = app.state::<DummyStore>();

            write(

              app.path().app_local_data_dir().unwrap().join("store.json"),

              serde_json::to_string(&*store.0.lock().unwrap()).unwrap(),

            )

            .unwrap();

          }

          _ => {}

        }

      })

### on_drop

章节标题 “on_drop”

  * **何时** ：插件正在被解构时
  * **用途** ：在插件被销毁时执行代码

有关更多信息，请参阅 [`Drop`](https://doc.rust-lang.net.cn/std/ops/trait.Drop.html)。

src/lib.rs

    use tauri::plugin::Builder;

    Builder::new("<plugin-name>")

      .on_drop(|app| {

        // plugin has been destroyed...

      })

## 公开 Rust API

章节标题 “公开 Rust API”

项目中 `desktop.rs` 和 `mobile.rs` 中定义的插件 API 会以与插件同名（PascalCase 格式）的结构体形式导出给用户。当插件设置完成后，该结构体的一个实例会被创建并作为状态进行管理，以便用户可以随时通过插件中定义的扩展 trait，使用 `Manager` 实例（例如 `AppHandle`、`App` 或 `Window`）获取它。

例如，[`global-shortcut 插件`](/plugin/global-shortcut/) 定义了一个 `GlobalShortcut` 结构体，用户可以通过使用 `GlobalShortcutExt` trait 的 `global_shortcut` 方法来读取它。

src-tauri/src/lib.rs

    use tauri_plugin_global_shortcut::GlobalShortcutExt;

    tauri::Builder::default()

      .plugin(tauri_plugin_global_shortcut::init())

      .setup(|app| {

        app.global_shortcut().register(...);

        Ok(())

      })

## 添加命令

章节标题 “添加命令”

命令定义在 `commands.rs` 文件中。它们是常规的 Tauri 应用程序命令。它们可以直接访问 `AppHandle` 和 `Window` 实例，访问状态，并以与应用程序命令相同的方式接收输入。有关 Tauri 命令的更多详情，请阅读 [命令指南](/develop/calling-rust/)。

此命令展示了如何通过依赖注入获取 `AppHandle` 和 `Window` 实例，并接收两个输入参数（`on_progress` 和 `url`）

src/commands.rs

    use tauri::{command, ipc::Channel, AppHandle, Runtime, Window};

    #[command]

    async fn upload<R: Runtime>(app: AppHandle<R>, window: Window<R>, on_progress: Channel, url: String) {

      // implement command logic here

      on_progress.send(100).unwrap();

    }

要将命令公开给 WebView，您必须在 `lib.rs` 中挂载到 `invoke_handler()` 调用中

src/lib.rs

    Builder::new("<plugin-name>")

        .invoke_handler(tauri::generate_handler![commands::upload])

在 `webview-src/index.ts` 中定义一个绑定函数，以便插件用户可以轻松地在 JavaScript 中调用该命令

    import { invoke, Channel } from '@tauri-apps/api/core'

    export async function upload(url: string, onProgressHandler: (progress: number) => void): Promise<void> {

      const onProgress = new Channel<number>()

      onProgress.onmessage = onProgressHandler

      await invoke('plugin:<plugin-name>|upload', { url, onProgress })

    }

在测试之前，请确保构建了 TypeScript 代码。

### 命令权限

章节标题 “命令权限”

默认情况下，您的命令对前端是不可访问的。如果您尝试执行其中一个命令，将会收到拒绝错误。要真正公开命令，您还需要定义允许每个命令的权限。

#### 权限文件

章节标题 “权限文件”

权限定义为 `permissions` 目录下的 JSON 或 TOML 文件。每个文件可以定义一组权限、权限集以及您插件的默认权限。

##### Permissions

名为“权限”的章节

权限描述了插件命令的特权。它可以允许或拒绝一系列命令，并关联命令特定的作用域或全局作用域。

permissions/start-server.toml

    "$schema" = "schemas/schema.json"

    [[permission]]

    identifier = "allow-start-server"

    description = "Enables the start_server command."

    commands.allow = ["start_server"]

    [[permission]]

    identifier = "deny-start-server"

    description = "Denies the start_server command."

    commands.deny = ["start_server"]

##### 范围

章节标题 “作用域 (Scope)”

作用域允许您的插件为单个命令定义更深层次的限制。每个权限都可以定义一系列作用域对象，这些对象定义了针对特定命令或插件全局的允许或拒绝规则。

让我们定义一个示例结构体，用于保存 `shell` 插件允许生成的二进制文件列表的作用域数据

src/scope.rs

    #[derive(Debug, schemars::JsonSchema)]

    pub struct Entry {

        pub binary: String,

    }

###### 命令作用域

章节标题 “命令作用域”

您的插件使用者可以在其功能配置文件中为特定命令定义作用域（请参阅 [文档](/reference/acl/scope/)）。您可以使用 [`tauri::ipc::CommandScope`](https://docs.rs/tauri/2.0.0/tauri/ipc/struct.CommandScope.html) 结构体读取命令特定的作用域。

src/commands.rs

    use tauri::ipc::CommandScope;

    use crate::scope::Entry;

    async fn spawn<R: tauri::Runtime>(app: tauri::AppHandle<R>, command_scope: CommandScope<'_, Entry>) -> Result<()> {

      let allowed = command_scope.allows();

      let denied = command_scope.denies();

      todo!()

    }

###### 全局作用域

章节标题 “全局作用域”

当权限未定义任何要允许或拒绝的命令时，它被视为范围权限，且应仅为您的插件定义全局作用域。

permissions/spawn-node.toml

    [[permission]]

    identifier = "allow-spawn-node"

    description = "This scope permits spawning the `node` binary."

    [[permission.scope.allow]]

    binary = "node"

您可以使用 [`tauri::ipc::GlobalScope`](https://docs.rs/tauri/2.0.0/tauri/ipc/struct.GlobalScope.html) 结构体读取全局作用域。

src/commands.rs

    use tauri::ipc::GlobalScope;

    use crate::scope::Entry;

    async fn spawn<R: tauri::Runtime>(app: tauri::AppHandle<R>, scope: GlobalScope<'_, Entry>) -> Result<()> {

      let allowed = scope.allows();

      let denied = scope.denies();

      todo!()

    }

注意

我们建议同时检查全局和命令作用域以获得灵活性。

###### Schema

章节标题 “Schema”

作用域条目需要 `schemars` 依赖来生成 JSON schema，以便插件使用者了解作用域的格式，并在其 IDE 中获得自动补全功能。

要定义 schema，首先将依赖项添加到您的 Cargo.toml 文件中

    # we need to add schemars to both dependencies and build-dependencies because the scope.rs module is shared between the app code and build script

    [dependencies]

    schemars = "0.8"

    [build-dependencies]

    schemars = "0.8"

在您的构建脚本中，添加以下代码

build.rs

    #[path = "src/scope.rs"]

    mod scope;

    const COMMANDS: &[&str] = &[];

    fn main() {

        tauri_plugin::Builder::new(COMMANDS)

            .global_scope_schema(schemars::schema_for!(scope::Entry))

            .build();

    }

##### 权限集

章节标题 “权限集”

权限集是一组单独的权限，可以帮助用户以更高层次的抽象管理您的插件。例如，如果单个 API 使用多个命令，或者一组命令之间存在逻辑连接，您应该定义一个包含它们的权限集。

permissions/websocket.toml

    "$schema" = "schemas/schema.json"

    [[set]]

    identifier = "allow-websocket"

    description = "Allows connecting and sending messages through a WebSocket"

    permissions = ["allow-connect", "allow-send"]

##### 默认权限

章节标题 “默认权限”

默认权限是一个标识符为 `default` 的特殊权限集。建议默认启用必需的命令。例如，如果没有允许 `request` 命令，`http` 插件是无用的。

permissions/default.toml

    "$schema" = "schemas/schema.json"

    [default]

    description = "Allows making HTTP requests"

    permissions = ["allow-request"]

#### 自动生成的权限

章节标题 “自动生成的权限”

为每个命令定义权限最简单的方法是使用在 `build.rs` 文件中定义的插件构建脚本中的自动生成选项。在 `COMMANDS` 常量中，以 snake_case 格式定义命令列表（应与命令函数名称匹配），Tauri 将自动生成 `allow-$commandname` 和 `deny-$commandname` 权限。

以下示例生成了 `allow-upload` 和 `deny-upload` 权限

src/commands.rs

    const COMMANDS: &[&str] = &["upload"];

    fn main() {

        tauri_plugin::Builder::new(COMMANDS).build();

    }

有关更多信息，请参阅 [权限概述](/security/permissions/) 文档。

## 管理状态

章节标题 “管理状态”

插件可以像 Tauri 应用程序一样管理状态。有关更多信息，请阅读 [状态管理指南](/develop/state-management/)。