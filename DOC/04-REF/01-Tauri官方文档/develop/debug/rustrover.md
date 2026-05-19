# 在 JetBrains IDE 中调试

_Source: https://v2.tauri.org.cn/develop/debug/rustrover/_

在本指南中，我们将设置 JetBrains RustRover 来调试 [Tauri 应用的核心进程](/concept/process-model/#the-core-process)。本指南大部分内容也适用于 IntelliJ 和 CLion。

## 设置 Cargo 项目

章节标题“设置 Cargo 项目”

根据项目中使用的前端技术栈，项目目录可能是一个 Cargo 项目，也可能不是。默认情况下，Tauri 将 Rust 项目放在名为 `src-tauri` 的子目录中。只有在前端开发也使用 Rust 时，它才会在根目录中创建 Cargo 项目。

如果顶层没有 `Cargo.toml` 文件，则需要手动添加该项目。打开 Cargo 工具窗口（在主菜单中，转到 **View | Tool Windows | Cargo** ），点击工具栏上的 **+** （**Attach Cargo Project** ），然后选择 `src-tauri/Cargo.toml` 文件。

或者，您可以通过在项目根目录中添加以下文件来手动创建一个顶层 Cargo 工作空间

Cargo.toml

    [workspace]

    members = ["src-tauri"]

在继续之前，请确保您的项目已完全加载。如果 Cargo 工具窗口显示了工作空间中的所有模块和目标，那么您可以开始了。

## 设置运行配置

章节标题“设置运行配置”

您需要设置两个独立的运行/调试配置：

  * 一个用于以调试模式启动 Tauri 应用，
  * 另一个用于运行您选择的前端开发服务器。

### Tauri 应用

章节标题“Tauri 应用”

  1. 在主菜单中，转到 **Run | Edit Configurations** 。
  2. 在 **Run/Debug Configurations** 对话框中

  * 要创建新配置，请点击工具栏上的 **+** 并选择 **Cargo** 。

![Add Run/Debug Configuration](/_astro/add-cargo-config-light.Bfob59bJ_1pRmgh.webp)

创建完成后，我们需要配置 RustRover，让其指示 Cargo 在构建应用时不使用任何默认特性。这将告知 Tauri 使用您的开发服务器，而不是从磁盘读取资源。通常此标志由 Tauri CLI 传递，但由于我们在这里完全避开了它，因此需要手动传递该标志。

![Add --no-default-features flag](/_astro/set-no-default-features-light.CcoA2Q8U_tmggj.webp)

现在我们可以选择重命名该运行/调试配置，以便于记忆。在本例中，我们将其命名为“Run Tauri App”，但您可以随意命名。

![Rename Configuration](/_astro/rename-configuration-light.B4TFjRby_Z1MQo6T.webp)

### 开发服务器

标题为“开发服务器”的章节

上述配置将直接使用 Cargo 来构建 Rust 应用程序并附加调试器。这意味着我们完全绕过了 Tauri CLI，因此 `beforeDevCommand` 和 `beforeBuildCommand` 等功能**不会** 被执行。我们需要通过手动运行开发服务器来解决这个问题。

要创建相应的运行配置，您需要检查正在使用的实际开发服务器。查看 `src-tauri/tauri.conf.json` 文件并找到以下行

        "beforeDevCommand": "pnpm dev"

对于 `npm`、`pnpm` 或 `yarn`，您可以使用 **npm** 运行配置，例如

![NPM Configuration](/_astro/npm-configuration-light.B2Rk0eaZ_Z1NO8qD.webp)

请确保**命令** （Command）、**脚本** （Scripts）和**包管理器** （Package Manager）字段中的值正确。

如果您的开发服务器是用于基于 Rust 的 WebAssembly 前端框架的 `trunk`，则可以使用通用的 **Shell Script** 运行配置

![Trunk Serve Configuration](/_astro/trunk-configuration-light.78simsDr_1bNJJx.webp)

## 启动调试会话

章节标题“启动调试会话”

要启动调试会话，您首先需要运行开发服务器，然后点击“运行配置切换器”旁边的 **Debug** 按钮启动 Tauri 应用的调试。RustRover 会自动识别放置在项目中任何 Rust 文件中的断点，并在命中第一个断点时停止。

![Debug Session](/_astro/debug-session-light.BcVudV4z_ZIN7aw.webp)

至此，您可以浏览变量的值、进一步进入代码，并在运行时详细查看正在发生的情况。