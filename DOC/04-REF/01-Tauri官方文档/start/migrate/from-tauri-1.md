# 从 Tauri 1.0 升级

_Source: https://v2.tauri.org.cn/start/migrate/from-tauri-1/_

本指南将引导您将 Tauri 1.0 应用程序升级到 Tauri 2.0。

## 准备移动端支持

名为“准备移动端支持”的章节

Tauri 的移动端接口要求您的项目输出一个共享库。如果您现有的应用程序以移动端为目标，则必须修改 crate 以生成该类型的构件以及桌面可执行文件。

  1. 更改 Cargo 清单以生成库。追加以下代码块

src-tauri/Cargo.toml

    [lib]

    name = "app_lib"

    crate-type = ["staticlib", "cdylib", "rlib"]

  2. 将 `src-tauri/src/main.rs` 重命名为 `src-tauri/src/lib.rs`。此文件将由桌面和移动端目标共享。

  3. 将 `lib.rs` 中的 `main` 函数头重命名为如下形式

src-tauri/src/lib.rs

    #[cfg_attr(mobile, tauri::mobile_entry_point)]

    pub fn run() {

        // your code here

    }

`tauri::mobile_entry_point` 宏会准备好您的函数，以便在移动端上执行。

  4. 重新创建调用共享运行函数的 `main.rs` 文件

src-tauri/src/main.rs

    #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

    fn main() {

      app_lib::run();

    }

## 自动化迁移

名为“自动化迁移”的章节

危险

此命令不能替代本指南！无论您是否选择使用该命令，请阅读 _整个_ 页面。

Tauri v2 CLI 包含一个 `migrate` 命令，该命令可自动化大部分过程并帮助您完成迁移。

  * npm
  * yarn
  * pnpm
  * cargo

    npm install @tauri-apps/cli@latest

    npm run tauri migrate

    yarn upgrade @tauri-apps/cli@latest

    yarn tauri migrate

    pnpm update @tauri-apps/cli@latest

    pnpm tauri migrate

    cargo install tauri-cli --version "^2.0.0" --locked

    cargo tauri migrate

在 [命令行界面参考](/reference/cli/#migrate) 中了解有关 `migrate` 命令的更多信息

## 变更总结

名为“变更总结”的章节

以下是从 Tauri 1.0 到 Tauri 2.0 的变更总结

### Tauri 配置

名为“Tauri 配置”的章节

  * `package > productName` 和 `package > version` 移到了顶层对象。
  * 二进制文件名不再自动重命名以匹配 `productName`，因此您必须在顶层对象中添加一个匹配 `productName` 的 `mainBinaryName` 字符串。
  * `package` 被移除。
  * `tauri` 键重命名为 `app`。
  * `tauri > allowlist` 被移除。请参阅 迁移权限。
  * `tauri > allowlist > protocol > assetScope` 移至 `app > security > assetProtocol > scope`。
  * `tauri > cli` 移至 `plugins > cli`。
  * `tauri > windows > fileDropEnabled` 重命名为 `app > windows > dragDropEnabled`。
  * `tauri > updater > active` 被移除。
  * `tauri > updater > dialog` 被移除。
  * `tauri > updater` 移至 `plugins > updater`。
  * 添加了 `bundle > createUpdaterArtifacts`，使用应用更新器时必须设置此项。
    * 从 v1 已分发的应用程序升级时，请将其设置为 `v1Compatible`。有关更多信息，请参阅 [更新器指南](/plugin/updater/)。
  * `tauri > systemTray` 重命名为 `app > trayIcon`。
  * `tauri > pattern` 移至 `app > security > pattern`。
  * `tauri > bundle` 移至顶层。
  * `tauri > bundle > identifier` 移至顶层对象。
  * `tauri > bundle > dmg` 移至 `bundle > macOS > dmg`
  * `tauri > bundle > deb` 移至 `bundle > linux > deb`
  * `tauri > bundle > appimage` 移至 `bundle > linux > appimage`
  * `tauri > bundle > macOS > license` 被移除，请改用 `bundle > licenseFile`。
  * `tauri > bundle > windows > wix > license` 被移除，请改用 `bundle > licenseFile`。
  * `tauri > bundle > windows > nsis > license` 被移除，请改用 `bundle > licenseFile`。
  * `tauri > bundle > windows > webviewFixedRuntimePath` 被移除，请改用 `bundle > windows > webviewInstallMode`。
  * `build > withGlobalTauri` 移至 `app > withGlobalTauri`。
  * `build > distDir` 重命名为 `frontendDist`。
  * `build > devPath` 重命名为 `devUrl`。

[Tauri 2.0 配置 API 参考](/reference/config/)

### 新的 Cargo 特性

名为“新的 Cargo 特性”的章节

  * linux-protocol-body：启用自定义协议请求体解析，允许 IPC 使用它。需要 webkit2gtk 2.40。

### 已移除的 Cargo 特性

名为“已移除的 Cargo 特性”的章节

  * reqwest-client：reqwest 现在是唯一支持的客户端。
  * reqwest-native-tls-vendored：请改用 `native-tls-vendored`。
  * process-command-api：请改用 `shell` 插件（请参阅下一节中的说明）。
  * shell-open-api：请改用 `shell` 插件（请参阅下一节中的说明）。
  * windows7-compat：已移至 `notification` 插件。
  * updater：Updater 现在是一个插件。
  * linux-protocol-headers：由于我们升级了 webkit2gtk 的最低版本要求，现在默认启用。
  * system-tray：重命名为 `tray-icon`。

### Rust Crate 变更

名为“Rust Crate 变更”的章节

  * `api` 模块已移除。每个 API 模块都可以在相应的 Tauri 插件中找到。
  * `api::dialog` 模块已移除。请改用 `tauri-plugin-dialog`。迁移指南
  * `api::file` 模块已移除。请改用 Rust 的 [`std::fs`](https://doc.rust-lang.net.cn/std/fs/)。
  * `api::http` 模块已移除。请改用 `tauri-plugin-http`。迁移指南
  * `api::ip` 模块已重写并移至 `tauri::ipc`。请查阅新的 API，特别是 `tauri::ipc::Channel`。
  * `api::path` 模块函数和 `tauri::PathResolved` 已移至 `tauri::Manager::path`。迁移指南
  * `api::process::Command`、`tauri::api::shell` 和 `tauri::Manager::shell_scope` API 已移除。请改用 `tauri-plugin-shell`。迁移指南
  * `api::process::current_binary` 和 `tauri::api::process::restart` 已移至 `tauri::process`。
  * `api::version` 模块已移除。请改用 [semver crate](https://docs.rs/semver/latest/semver/)。
  * `App::clipboard_manager` 和 `AppHandle::clipboard_manager` 已移除。请改用 `tauri-plugin-clipboard`。迁移指南
  * `App::get_cli_matches` 已移除。请改用 `tauri-plugin-cli`。迁移指南
  * `App::global_shortcut_manager` 和 `AppHandle::global_shortcut_manager` 已移除。请改用 `tauri-plugin-global-shortcut`。迁移指南
  * `Manager::fs_scope` 已移除。文件系统作用域可通过 `tauri_plugin_fs::FsExt` 访问。
  * `Plugin::PluginApi` 现在接收插件配置作为第二个参数。
  * `Plugin::setup_with_config` 已移除。请改用更新后的 `tauri::Plugin::PluginApi`。
  * `scope::ipc::RemoteDomainAccessScope::enable_tauri_api` 和 `scope::ipc::RemoteDomainAccessScope::enables_tauri_api` 已移除。请改用 `scope::ipc::RemoteDomainAccessScope::add_plugin` 单独启用每个核心插件。
  * `scope::IpcScope` 已移除，请改用 `scope::ipc::Scope`。
  * `scope::FsScope`、`scope::GlobPattern` 和 `scope::FsScopeEvent` 已移除，请分别使用 `scope::fs::Scope`、`scope::fs::Pattern` 和 `scope::fs::Event`。
  * `updater` 模块已移除。请改用 `tauri-plugin-updater`。迁移指南
  * `Env.args` 字段已移除，请改用 `Env.args_os` 字段。
  * `Menu`、`MenuEvent`、`CustomMenuItem`、`Submenu`、`WindowMenuEvent`、`MenuItem` 和 `Builder::on_menu_event` API 已移除。迁移指南
  * `SystemTray`、`SystemTrayHandle`、`SystemTrayMenu`、`SystemTrayMenuItemHandle`、`SystemTraySubmenu`、`MenuEntry` 和 `SystemTrayMenuItem` API 已移除。迁移指南

### JavaScript API 变更

名为“JavaScript API 变更”的章节

`@tauri-apps/api` 包不再提供非核心模块。仅导出之前的 `tauri`（现为 `core`）、`path`、`event` 和 `window` 模块。所有其他模块均已移至插件。

  * `@tauri-apps/api/tauri` 模块重命名为 `@tauri-apps/api/core`。迁移指南
  * `@tauri-apps/api/cli` 模块已移除。请改用 `@tauri-apps/plugin-cli`。迁移指南
  * `@tauri-apps/api/clipboard` 模块已移除。请改用 `@tauri-apps/plugin-clipboard`。迁移指南
  * `@tauri-apps/api/dialog` 模块已移除。请改用 `@tauri-apps/plugin-dialog`。迁移指南
  * `@tauri-apps/api/fs` 模块已移除。请改用 `@tauri-apps/plugin-fs`。迁移指南
  * `@tauri-apps/api/global-shortcut` 模块已移除。请改用 `@tauri-apps/plugin-global-shortcut`。迁移指南
  * `@tauri-apps/api/http` 模块已移除。请改用 `@tauri-apps/plugin-http`。迁移指南
  * `@tauri-apps/api/os` 模块已移除。请改用 `@tauri-apps/plugin-os`。迁移指南
  * `@tauri-apps/api/notification` 模块已移除。请改用 `@tauri-apps/plugin-notification`。迁移指南
  * `@tauri-apps/api/process` 模块已移除。请改用 `@tauri-apps/plugin-process`。迁移指南
  * `@tauri-apps/api/shell` 模块已移除。请改用 `@tauri-apps/plugin-shell`。迁移指南
  * `@tauri-apps/api/updater` 模块已移除。请改用 `@tauri-apps/plugin-updater` 迁移指南
  * `@tauri-apps/api/window` 模块重命名为 `@tauri-apps/api/webviewWindow`。迁移指南

v1 插件现在作为 `@tauri-apps/plugin-` 发布。之前它们可以通过 git 作为 `tauri-plugin--api` 获取。

### 环境变量变更

名为“环境变量变更”的章节

Tauri CLI 读取和写入的大多数环境变量已重命名，以保持一致性并防止错误

  * `TAURI_PRIVATE_KEY` -> `TAURI_SIGNING_PRIVATE_KEY`
  * `TAURI_KEY_PASSWORD` -> `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`
  * `TAURI_SKIP_DEVSERVER_CHECK` -> `TAURI_CLI_NO_DEV_SERVER_WAIT`
  * `TAURI_DEV_SERVER_PORT` -> `TAURI_CLI_PORT`
  * `TAURI_PATH_DEPTH` -> `TAURI_CLI_CONFIG_DEPTH`
  * `TAURI_FIPS_COMPLIANT` -> `TAURI_BUNDLER_WIX_FIPS_COMPLIANT`
  * `TAURI_DEV_WATCHER_IGNORE_FILE` -> `TAURI_CLI_WATCHER_IGNORE_FILENAME`
  * `TAURI_TRAY` -> `TAURI_LINUX_AYATANA_APPINDICATOR`
  * `TAURI_APPLE_DEVELOPMENT_TEAM` -> `APPLE_DEVELOPMENT_TEAM`
  * `TAURI_PLATFORM` -> `TAURI_ENV_PLATFORM`
  * `TAURI_ARCH` -> `TAURI_ENV_ARCH`
  * `TAURI_FAMILY` -> `TAURI_ENV_FAMILY`
  * `TAURI_PLATFORM_VERSION` -> `TAURI_ENV_PLATFORM_VERSION`
  * `TAURI_PLATFORM_TYPE` -> `TAURI_ENV_PLATFORM_TYPE`
  * `TAURI_DEBUG` -> `TAURI_ENV_DEBUG`

### 事件系统

名为“事件系统”的章节

事件系统已重新设计，使其更易于使用。它不再依赖事件源，而是采用了一种基于事件目标的更简单的实现。

  * `emit` 函数现在将事件发送给所有事件监听器。
  * 添加了新的 `emit_to`/`emitTo` 函数，以向特定目标触发事件。
  * `emit_filter` 现在基于 [`EventTarget`](https://docs.rs/tauri/2.0.0/tauri/event/enum.EventTarget.html) 而不是窗口进行过滤。
  * 将 `listen_global` 重命名为 `listen_any`。它现在监听所有事件，无论其过滤器和目标如何。
  * JavaScript：`event.listen()` 的行为类似于 `listen_any`。它现在监听所有事件，无论其过滤器和目标如何，除非在 `Options` 中设置了目标。
  * JavaScript：`WebviewWindow.listen` 等仅监听发送到各自 `EventTarget` 的事件。

### 多 WebView 支持

名为“多 WebView 支持”的章节

Tauri v2 引入了多 WebView 支持，目前处于 `unstable` 特性标志后。为了支持它，我们将 Rust 的 `Window` 类型重命名为 `WebviewWindow`，将管理器的 `get_window` 函数重命名为 `get_webview_window`。

`WebviewWindow` JS API 类型现在从 `@tauri-apps/api/webviewWindow` 而不是 `@tauri-apps/api/window` 重新导出。

### Windows 上的新来源 URL

名为“Windows 上的新来源 URL”的章节

在 Windows 上，生产应用程序中的前端文件现在托管在 `http://tauri.localhost` 而不是 `https://tauri.localhost` 上。因此，除非在 v1 中使用了 `dangerousUseHttpScheme`，否则 IndexedDB、LocalStorage 和 Cookies 将被重置。为防止此情况，您可以将 `app > windows > useHttpsScheme` 设置为 `true`，或者使用 `WebviewWindowBuilder::use_https_scheme` 继续使用 `https` 方案。

## 详细迁移步骤

名为“详细迁移步骤”的章节

将 Tauri 1.0 应用程序迁移到 Tauri 2.0 时可能会遇到的常见场景。

### 迁移到 Core 模块

名为“迁移到 Core 模块”的章节

`@tauri-apps/api/tauri` 模块已重命名为 `@tauri-apps/api/core`。只需重命名模块导入即可

    import { invoke } from "@tauri-apps/api/tauri"

    import { invoke } from "@tauri-apps/api/core"

### 迁移到 CLI 插件

名为“迁移到 CLI 插件”的章节

Rust `App::get_cli_matches` 和 JavaScript `@tauri-apps/api/cli` API 已移除。请改用 `@tauri-apps/plugin-cli` 插件

  1. 添加到 Cargo 依赖项

Cargo.toml

    [dependencies]

    tauri-plugin-cli = "2"

  2. 在 JavaScript 或 Rust 项目中使用

  * JavaScript
  * Rust

    fn main() {

        tauri::Builder::default()

            .plugin(tauri_plugin_cli::init())

    }

package.json

    {

      "dependencies": {

        "@tauri-apps/plugin-cli": "^2.0.0"

      }

    }

    import { getMatches } from '@tauri-apps/plugin-cli';

    const matches = await getMatches();

    fn main() {

        use tauri_plugin_cli::CliExt;

        tauri::Builder::default()

            .plugin(tauri_plugin_cli::init())

            .setup(|app| {

                let cli_matches = app.cli().matches()?;

                Ok(())

            })

    }

### 迁移到 Clipboard 插件

名为“迁移到 Clipboard 插件”的章节

Rust `App::clipboard_manager` 和 `AppHandle::clipboard_manager` 以及 JavaScript `@tauri-apps/api/clipboard` API 已移除。请改用 `@tauri-apps/plugin-clipboard-manager` 插件

    [dependencies]

    tauri-plugin-clipboard-manager = "2"

  * JavaScript
  * Rust

    fn main() {

        tauri::Builder::default()

            .plugin(tauri_plugin_clipboard_manager::init())

    }

package.json

    {

      "dependencies": {

        "@tauri-apps/plugin-clipboard-manager": "^2.0.0"

      }

    }

    import { writeText, readText } from '@tauri-apps/plugin-clipboard-manager';

    await writeText('Tauri is awesome!');

    assert(await readText(), 'Tauri is awesome!');

    use tauri_plugin_clipboard::{ClipboardExt, ClipKind};

    tauri::Builder::default()

        .plugin(tauri_plugin_clipboard::init())

        .setup(|app| {

            app.clipboard().write(ClipKind::PlainText {

                label: None,

                text: "Tauri is awesome!".into(),

            })?;

            Ok(())

        })

### 迁移到 Dialog 插件

名为“迁移到 Dialog 插件”的章节

Rust `tauri::api::dialog` 和 JavaScript `@tauri-apps/api/dialog` API 已移除。请改用 `@tauri-apps/plugin-dialog` 插件

  1. 添加到 Cargo 依赖项

Cargo.toml

    [dependencies]

    tauri-plugin-dialog = "2"

  2. 在 JavaScript 或 Rust 项目中使用

  * JavaScript
  * Rust

    fn main() {

        tauri::Builder::default()

            .plugin(tauri_plugin_dialog::init())

    }

package.json

    {

      "dependencies": {

        "@tauri-apps/plugin-dialog": "^2.0.0"

      }

    }

    import { save } from '@tauri-apps/plugin-dialog';

    const filePath = await save({

      filters: [

        {

          name: 'Image',

          extensions: ['png', 'jpeg'],

        },

      ],

    });

    use tauri_plugin_dialog::DialogExt;

    tauri::Builder::default()

        .plugin(tauri_plugin_dialog::init())

        .setup(|app| {

            app.dialog().file().pick_file(|file_path| {

                // do something with the optional file path here

                // the file path is `None` if the user closed the dialog

            });

            app.dialog().message("Tauri is Awesome!").show();

            Ok(())

         })

### 迁移到文件系统插件

名为“迁移到文件系统插件”的章节

Rust `App::get_cli_matches` 和 JavaScript `@tauri-apps/api/fs` API 已移除。Rust 请改用 [`std::fs`](https://doc.rust-lang.net.cn/std/fs/)，JavaScript 请改用 `@tauri-apps/plugin-fs` 插件

  1. 添加到 Cargo 依赖项

Cargo.toml

    [dependencies]

    tauri-plugin-fs = "2"

  2. 在 JavaScript 或 Rust 项目中使用

  * JavaScript
  * Rust

    fn main() {

        tauri::Builder::default()

            .plugin(tauri_plugin_fs::init())

    }

package.json

    {

      "dependencies": {

        "@tauri-apps/plugin-fs": "^2.0.0"

      }

    }

    import { mkdir, BaseDirectory } from '@tauri-apps/plugin-fs';

    await mkdir('db', { baseDir: BaseDirectory.AppLocalData });

部分函数和类型已被重命名或移除

  * `Dir` 枚举别名已移除，请使用 `BaseDirectory`。
  * `FileEntry`、`FsBinaryFileOption`、`FsDirOptions`、`FsOptions`、`FsTextFileOption` 和 `BinaryFileContents` 接口及类型别名已移除，并替换为适合每个函数的新接口。
  * `createDir` 重命名为 `mkdir`。
  * `readBinaryFile` 重命名为 `readFile`。
  * `removeDir` 已移除，并替换为 `remove`。
  * `removeFile` 已移除，并替换为 `remove`。
  * `renameFile` 已移除，并替换为 `rename`。
  * `writeBinaryFile` 重命名为 `writeFile`。

请使用 Rust 的 [`std::fs`](https://doc.rust-lang.net.cn/std/fs/) 函数。

### 迁移到全局快捷键插件

名为“迁移到全局快捷键插件”的章节

Rust `App::global_shortcut_manager` 和 `AppHandle::global_shortcut_manager` 以及 JavaScript `@tauri-apps/api/global-shortcut` API 已移除。请改用 `@tauri-apps/plugin-global-shortcut` 插件

  1. 添加到 Cargo 依赖项

Cargo.toml

    [dependencies]

    [target."cfg(not(any(target_os = \"android\", target_os = \"ios\")))".dependencies]

    tauri-plugin-global-shortcut = "2"

  2. 在 JavaScript 或 Rust 项目中使用

  * JavaScript
  * Rust

    fn main() {

        tauri::Builder::default()

            .plugin(tauri_plugin_global_shortcut::Builder::default().build())

    }

package.json

    {

      "dependencies": {

        "@tauri-apps/plugin-global-shortcut": "^2.0.0"

      }

    }

    import { register } from '@tauri-apps/plugin-global-shortcut';

    await register('CommandOrControl+Shift+C', () => {

      console.log('Shortcut triggered');

    });

    use tauri_plugin_global_shortcut::GlobalShortcutExt;

    tauri::Builder::default()

        .plugin(

            tauri_plugin_global_shortcut::Builder::new().with_handler(|app, shortcut| {

                println!("Shortcut triggered: {:?}", shortcut);

            })

            .build(),

        )

        .setup(|app| {

            // register a global shortcut

            // on macOS, the Cmd key is used

            // on Windows and Linux, the Ctrl key is used

            app.global_shortcut().register("CmdOrCtrl+Y")?;

            Ok(())

        })

### 迁移到 HTTP 插件

名为“迁移到 HTTP 插件”的章节

Rust `tauri::api::http` 和 JavaScript `@tauri-apps/api/http` API 已移除。请改用 `@tauri-apps/plugin-http` 插件

  1. 添加到 Cargo 依赖项

Cargo.toml

    [dependencies]

    tauri-plugin-http = "2"

  2. 在 JavaScript 或 Rust 项目中使用

  * JavaScript
  * Rust

    fn main() {

        tauri::Builder::default()

            .plugin(tauri_plugin_http::init())

    }

package.json

    {

      "dependencies": {

        "@tauri-apps/plugin-http": "^2.0.0"

      }

    }

    import { fetch } from '@tauri-apps/plugin-http';

    const response = await fetch(

      'https://raw.githubusercontent.com/tauri-apps/tauri/dev/package.json'

    );

    use tauri_plugin_http::reqwest;

    tauri::Builder::default()

        .plugin(tauri_plugin_http::init())

        .setup(|app| {

            let response_data = tauri::async_runtime::block_on(async {

                let response = reqwest::get(

                    "https://raw.githubusercontent.com/tauri-apps/tauri/dev/package.json",

                )

                .await

                .unwrap();

                response.text().await

            })?;

            Ok(())

        })

HTTP 插件重新导出了 [reqwest](https://docs.rs/reqwest/latest/reqwest/)，因此您可以查看其文档以获取更多信息。

### 迁移到通知插件

名为“迁移到通知插件”的章节

Rust `tauri::api::notification` 和 JavaScript `@tauri-apps/api/notification` API 已移除。请改用 `@tauri-apps/plugin-notification` 插件

  1. 添加到 Cargo 依赖项

Cargo.toml

    [dependencies]

    tauri-plugin-notification = "2"

  2. 在 JavaScript 或 Rust 项目中使用

  * JavaScript
  * Rust

    fn main() {

        tauri::Builder::default()

            .plugin(tauri_plugin_notification::init())

    }

package.json

    {

      "dependencies": {

        "@tauri-apps/plugin-notification": "^2.0.0"

      }

    }

    import { sendNotification } from '@tauri-apps/plugin-notification';

    sendNotification('Tauri is awesome!');

    use tauri_plugin_notification::NotificationExt;

    use tauri::plugin::PermissionState;

    fn main() {

        tauri::Builder::default()

            .plugin(tauri_plugin_notification::init())

            .setup(|app| {

                if app.notification().permission_state()? == PermissionState::Unknown {

                    app.notification().request_permission()?;

                }

                if app.notification().permission_state()? == PermissionState::Granted {

                    app.notification()

                        .builder()

                        .body("Tauri is awesome!")

                        .show()?;

                }

                Ok(())

            })

    }

### 迁移到 Menu 模块

名为“迁移到 Menu 模块”的章节

Rust `Menu` API 已移至 `tauri::menu` 模块，并重构为使用 [muda crate](https://github.com/tauri-apps/muda)。

#### 请使用 `tauri::menu::MenuBuilder`

名为“请使用 tauri::menu::MenuBuilder”的章节

请使用 `tauri::menu::MenuBuilder` 代替 `tauri::Menu`。请注意，其构造函数需要一个 Manager 实例（即 `App`、`AppHandle` 或 `WebviewWindow` 中的一个）作为参数

    use tauri::menu::MenuBuilder;

    tauri::Builder::default()

        .setup(|app| {

            let menu = MenuBuilder::new(app)

                .copy()

                .paste()

                .separator()

                .undo()

                .redo()

                .text("open-url", "Open URL")

                .check("toggle", "Toggle")

                .icon("show-app", "Show App", app.default_window_icon().cloned().unwrap())

                .build()?;

            app.set_menu(menu);

            Ok(())

        })

#### 请使用 `tauri::menu::PredefinedMenuItem`

名为“请使用 tauri::menu::PredefinedMenuItem”的章节

请使用 `tauri::menu::PredefinedMenuItem` 代替 `tauri::MenuItem`

    use tauri::menu::{MenuBuilder, PredefinedMenuItem};

    tauri::Builder::default()

        .setup(|app| {

            let menu = MenuBuilder::new(app).item(&PredefinedMenuItem::copy(app)?).build()?;

            Ok(())

        })

提示

菜单构建器具有添加每个预定义菜单项的专用方法，因此您可以调用 `.copy()` 而不是 `.item(&PredefinedMenuItem::copy(app, None)?)`。

#### 请使用 `tauri::menu::MenuItemBuilder`

名为“请使用 tauri::menu::MenuItemBuilder”的章节

请使用 `tauri::menu::MenuItemBuilder` 代替 `tauri::CustomMenuItem`

    use tauri::menu::MenuItemBuilder;

    tauri::Builder::default()

        .setup(|app| {

            let toggle = MenuItemBuilder::new("Toggle").accelerator("Ctrl+Shift+T").build(app)?;

            Ok(())

        })

#### 请使用 `tauri::menu::SubmenuBuilder`

名为“请使用 tauri::menu::SubmenuBuilder”的章节

请使用 `tauri::menu::SubmenuBuilder` 代替 `tauri::Submenu`

    use tauri::menu::{MenuBuilder, SubmenuBuilder};

    tauri::Builder::default()

        .setup(|app| {

            let submenu = SubmenuBuilder::new(app, "Sub")

                .text("Tauri")

                .separator()

                .check("Is Awesome")

                .build()?;

            let menu = MenuBuilder::new(app).item(&submenu).build()?;

            Ok(())

        })

`tauri::Builder::menu` 现在接收一个闭包，因为菜单构建需要一个 Manager 实例。有关更多信息，请参阅 [文档](https://docs.rs/tauri/2.0.0/tauri/struct.Builder.html#method.menu)。

#### 菜单事件

名为“菜单事件”的章节

Rust `tauri::Builder::on_menu_event` API 已移除。请改用 `tauri::App::on_menu_event` 或 `tauri::AppHandle::on_menu_event`

    use tauri::menu::{CheckMenuItemBuilder, MenuBuilder, MenuItemBuilder};

    tauri::Builder::default()

        .setup(|app| {

            let toggle = MenuItemBuilder::with_id("toggle", "Toggle").build(app)?;

            let check = CheckMenuItemBuilder::new("Mark").build(app)?;

            let menu = MenuBuilder::new(app).items(&[&toggle, &check]).build()?;

            app.set_menu(menu)?;

            app.on_menu_event(move |app, event| {

                if event.id() == check.id() {

                    println!("`check` triggered, do something! is checked? {}", check.is_checked().unwrap());

                } else if event.id() == "toggle" {

                    println!("toggle triggered!");

                }

            });

            Ok(())

        })

请注意，有两种方法可以检查选择了哪个菜单项：将该项移动到事件处理程序闭包中并比较 ID，或者通过 `with_id` 构造函数为该项定义自定义 ID，并使用该 ID 字符串进行比较。

提示

菜单项可以在多个菜单之间共享，且菜单事件绑定到菜单项而不是菜单或窗口。如果您不希望在选择菜单项时触发所有监听器，请不要共享菜单项，而应使用专用实例，您可以将其移动到 `tauri::WebviewWindow/WebviewWindowBuilder::on_menu_event` 闭包中。

### 迁移到 OS 插件

名为“迁移到 OS 插件”的章节

Rust `tauri::api::os` 和 JavaScript `@tauri-apps/api/os` API 已移除。请改用 `@tauri-apps/plugin-os` 插件

  1. 添加到 Cargo 依赖项

Cargo.toml

    [dependencies]

    tauri-plugin-os = "2"

  2. 在 JavaScript 或 Rust 项目中使用

  * JavaScript
  * Rust

    fn main() {

        tauri::Builder::default()

            .plugin(tauri_plugin_os::init())

    }

package.json

    {

      "dependencies": {

        "@tauri-apps/plugin-os": "^2.0.0"

      }

    }

    import { arch } from '@tauri-apps/plugin-os';

    const architecture = await arch();

    fn main() {

        tauri::Builder::default()

            .plugin(tauri_plugin_os::init())

            .setup(|app| {

                let os_arch = tauri_plugin_os::arch();

                Ok(())

            })

    }

### 迁移到 Process 插件

名为“迁移到 Process 插件”的章节

Rust `tauri::api::process` 和 JavaScript `@tauri-apps/api/process` API 已移除。请改用 `@tauri-apps/plugin-process` 插件

  1. 添加到 Cargo 依赖项

Cargo.toml

    [dependencies]

    tauri-plugin-process = "2"

  2. 在 JavaScript 或 Rust 项目中使用

  * JavaScript
  * Rust

    fn main() {

        tauri::Builder::default()

            .plugin(tauri_plugin_process::init())

    }

package.json

    {

      "dependencies": {

        "@tauri-apps/plugin-process": "^2.0.0"

      }

    }

    import { exit, relaunch } from '@tauri-apps/plugin-process';

    await exit(0);

    await relaunch();

    fn main() {

        tauri::Builder::default()

            .plugin(tauri_plugin_process::init())

            .setup(|app| {

                // exit the app with a status code

                app.handle().exit(1);

                // restart the app

                app.handle().restart();

                Ok(())

            })

    }

### 迁移到 Shell 插件

名为“迁移到 Shell 插件”的章节

Rust `tauri::api::shell` 和 JavaScript `@tauri-apps/api/shell` API 已移除。请改用 `@tauri-apps/plugin-shell` 插件

  1. 添加到 Cargo 依赖项

Cargo.toml

    [dependencies]

    tauri-plugin-shell = "2"

  2. 在 JavaScript 或 Rust 项目中使用

  * JavaScript
  * Rust

    fn main() {

        tauri::Builder::default()

            .plugin(tauri_plugin_shell::init())

    }

package.json

    {

      "dependencies": {

        "@tauri-apps/plugin-shell": "^2.0.0"

      }

    }

    import { Command, open } from '@tauri-apps/plugin-shell';

    const output = await Command.create('echo', 'message').execute();

    await open('https://github.com/tauri-apps/tauri');

  * 打开 URL

    use tauri_plugin_shell::ShellExt;

    fn main() {

        tauri::Builder::default()

            .plugin(tauri_plugin_shell::init())

            .setup(|app| {

                app.shell().open("https://github.com/tauri-apps/tauri", None)?;

                Ok(())

            })

    }

  * 生成子进程并获取状态码

    use tauri_plugin_shell::ShellExt;

    fn main() {

        tauri::Builder::default()

            .plugin(tauri_plugin_shell::init())

            .setup(|app| {

                let status = tauri::async_runtime::block_on(async move { app.shell().command("which").args(["ls"]).status().await.unwrap() });

                println!("`which` finished with status: {:?}", status.code());

                Ok(())

            })

    }

  * 生成子进程并捕获其输出

    use tauri_plugin_shell::ShellExt;

    fn main() {

        tauri::Builder::default()

            .plugin(tauri_plugin_shell::init())

            .setup(|app| {

                let output = tauri::async_runtime::block_on(async move { app.shell().command("echo").args(["TAURI"]).output().await.unwrap() });

                assert!(output.status.success());

                assert_eq!(String::from_utf8(output.stdout).unwrap(), "TAURI");

                Ok(())

            })

    }

  * 生成子进程并异步读取其事件

    use tauri_plugin_shell::{ShellExt, process::CommandEvent};

    fn main() {

        tauri::Builder::default()

            .plugin(tauri_plugin_shell::init())

            .setup(|app| {

                let handle = app.handle().clone();

                tauri::async_runtime::spawn(async move {

                    let (mut rx, mut child) = handle.shell().command("cargo")

                        .args(["tauri", "dev"])

                        .spawn()

                        .expect("Failed to spawn cargo");

                    let mut i = 0;

                    while let Some(event) = rx.recv().await {

                        if let CommandEvent::Stdout(line) = event {

                            println!("got: {}", String::from_utf8(line).unwrap());

                           i += 1;

                           if i == 4 {

                               child.write("message from Rust\n".as_bytes()).unwrap();

                               i = 0;

                           }

                       }

                    }

                });

                Ok(())

            })

    }

### 迁移到系统托盘图标模块

名为“迁移到系统托盘图标模块”的章节

Rust `SystemTray` API 已重命名为 `TrayIcon` 以保持一致性。新的 API 可以在 Rust `tray` 模块中找到。

#### 请使用 `tauri::tray::TrayIconBuilder`

名为“请使用 tauri::tray::TrayIconBuilder”的章节

请使用 `tauri::tray::TrayIconBuilder` 代替 `tauri::SystemTray`

    let tray = tauri::tray::TrayIconBuilder::with_id("my-tray").build(app)?;

有关更多信息，请参阅 [TrayIconBuilder](https://docs.rs/tauri/2.0.0/tauri/tray/struct.TrayIconBuilder.html)。

#### 迁移到 Menu

名为“迁移到 Menu”的章节

请使用 `tauri::menu::Menu` 代替 `tauri::SystemTrayMenu`，使用 `tauri::menu::Submenu` 代替 `tauri::SystemTraySubmenu`，使用 `tauri::menu::PredefinedMenuItem` 代替 `tauri::SystemTrayMenuItem`。

#### 托盘事件

名为“托盘事件”的章节

`tauri::SystemTray::on_event` 已拆分为 `tauri::tray::TrayIconBuilder::on_menu_event` 和 `tauri::tray::TrayIconBuilder::on_tray_icon_event`

    use tauri::{

        menu::{MenuBuilder, MenuItemBuilder},

        tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},

    };

    tauri::Builder::default()

        .setup(|app| {

            let toggle = MenuItemBuilder::with_id("toggle", "Toggle").build(app)?;

            let menu = MenuBuilder::new(app).items(&[&toggle]).build()?;

            let tray = TrayIconBuilder::new()

                .menu(&menu)

                .on_menu_event(move |app, event| match event.id().as_ref() {

                    "toggle" => {

                        println!("toggle clicked");

                    }

                    _ => (),

                })

                .on_tray_icon_event(|tray, event| {

                    if let TrayIconEvent::Click {

                            button: MouseButton::Left,

                            button_state: MouseButtonState::Up,

                            ..

                    } = event

                    {

                        let app = tray.app_handle();

                        if let Some(webview_window) = app.get_webview_window("main") {

                           let _ = webview_window.unminimize();

                           let _ = webview_window.show();

                           let _ = webview_window.set_focus();

                        }

                    }

                })

                .build(app)?;

            Ok(())

        })

### 迁移到更新器插件

名为“迁移到更新器插件”的章节

默认行为变更

带有自动更新检查的内置对话框已被移除，请改为使用 Rust 和 JS API 来检查和安装更新。如果不这样做，将导致您的用户无法获取后续更新！

Rust `tauri::updater` 和 JavaScript `@tauri-apps/api-updater` API 已移除。要使用 `@tauri-apps/plugin-updater` 设置自定义更新器目标

  1. 添加到 Cargo 依赖项

    [dependencies]

    tauri-plugin-updater = "2"

  2. 在 JavaScript 或 Rust 项目中使用

  * JavaScript
  * Rust

    fn main() {

        tauri::Builder::default()

            .plugin(tauri_plugin_updater::Builder::new().build())

    }

package.json

    {

      "dependencies": {

        "@tauri-apps/plugin-updater": "^2.0.0"

      }

    }

    import { check } from '@tauri-apps/plugin-updater';

    import { relaunch } from '@tauri-apps/plugin-process';

    const update = await check();

    if (update?.available) {

      console.log(`Update to ${update.version} available! Date: ${update.date}`);

      console.log(`Release notes: ${update.body}`);

      await update.downloadAndInstall();

      // requires the `process` plugin

      await relaunch();

    }

检查更新

    use tauri_plugin_updater::UpdaterExt;

    fn main() {

        tauri::Builder::default()

            .plugin(tauri_plugin_updater::Builder::new().build())

            .setup(|app| {

                let handle = app.handle();

                tauri::async_runtime::spawn(async move {

                    let response = handle.updater().check().await;

                });

                Ok(())

            })

    }

设置自定义更新器目标

    fn main() {

        let mut updater = tauri_plugin_updater::Builder::new();

        #[cfg(target_os = "macos")]

        {

            updater = updater.target("darwin-universal");

        }

        tauri::Builder::default()

            .plugin(updater.build())

    }

### 将 Path 迁移至 Tauri 管理器

名为“将 Path 迁移至 Tauri 管理器”的章节

Rust `tauri::api::path` 模块函数和 `tauri::PathResolver` 已移至 `tauri::Manager::path`

    use tauri::{path::BaseDirectory, Manager};

    tauri::Builder::default()

        .setup(|app| {

            let home_dir_path = app.path().home_dir().expect("failed to get home dir");

            let path = app.path().resolve("path/to/something", BaseDirectory::Config)?;

            Ok(())

      })

### 迁移到新的 Window API

名为“迁移到新的 Window API”的章节

在 Rust 端，`Window` 被重命名为 `WebviewWindow`，其构建器 `WindowBuilder` 现在命名为 `WebviewWindowBuilder`，`WindowUrl` 现在命名为 `WebviewUrl`。

此外，`Manager::get_window` 函数重命名为 `get_webview_window`，窗口的 `parent_window` API 重命名为 `parent_raw`，以支持更高级别的窗口父级 API。

在 JavaScript 端，`WebviewWindow` 类现在在 `@tauri-apps/api/webviewWindow` 路径下导出。

`onMenuClicked` 函数已被移除，您可以在 JavaScript 中创建菜单时拦截菜单事件。

### 迁移嵌入的附加文件（资源）

名为“迁移嵌入的附加文件（资源）”的章节

在 JavaScript 端，确保您已 迁移到文件系统插件。此外，请注意 v1 allowlist 在 迁移权限 中的变更。

在 Rust 端，确保您已 将 Path 迁移至 Tauri 管理器。

### 迁移嵌入的外部二进制文件（Sidecar）

名为“迁移嵌入的外部二进制文件（Sidecar）”的章节

在 Tauri v1 中，外部二进制文件及其参数是在 allowlist 中定义的。在 v2 中，请使用新的权限系统。请阅读 迁移权限 以获取更多信息。

在 JavaScript 端，确保您已 迁移到 Shell 插件。

在 Rust 端，`tauri::api::process` API 已被移除。请改用 `tauri_plugin_shell::ShellExt` 和 `tauri_plugin_shell::process::CommandEvent` API。阅读 [嵌入外部二进制文件](/develop/sidecar/#running-it-from-rust) 指南以查看如何操作。

“process-command-api”特性标志在 v2 中已被移除。因此，运行外部二进制文件不再需要在 Tauri 配置中定义此特性。

### 迁移权限

名为“迁移权限”的章节

v1 的 allowlist 已被重写为一套全新的权限系统，该系统适用于各个插件，并且对于多窗口和远程 URL 支持具有更高的可配置性。这个新系统的工作方式类似于访问控制列表 (ACL)，您可以在其中允许或拒绝命令、为特定窗口和域分配权限以及定义访问作用域。

要为您的应用程序启用权限，您必须在 `src-tauri/capabilities` 文件夹内创建 capability 文件，Tauri 将自动为您配置其余部分。

`migrate` CLI 命令会自动解析您的 v1 allowlist 并生成关联的 capability 文件。

要了解有关权限和功能的更多信息，请参阅 [安全文档](/security/)。