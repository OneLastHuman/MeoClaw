# 项目结构

_Source: https://v2.tauri.org.cn/start/project-structure/_

Tauri 项目通常由两部分组成：一个 Rust 项目和一个 JavaScript 项目（可选）。典型的设置如下所示：

    .

    ├── package.json

    ├── index.html

    ├── src/

    │   ├── main.js

    ├── src-tauri/

    │   ├── Cargo.toml

    │   ├── Cargo.lock

    │   ├── build.rs

    │   ├── tauri.conf.json

    │   ├── src/

    │   │   ├── main.rs

    │   │   └── lib.rs

    │   ├── icons/

    │   │   ├── icon.png

    │   │   ├── icon.icns

    │   │   └── icon.ico

    │   └── capabilities/

    │       └── default.json

在此方案中，JavaScript 项目位于顶层，而 Rust 项目位于 `src-tauri/` 文件夹内。这个 Rust 项目是一个标准的 [Cargo 项目](https://doc.rust-lang.net.cn/cargo/guide/project-layout.html)，但包含了一些额外的文件。

  * `tauri.conf.json` 是 Tauri 的主要配置文件。它包含了从应用程序标识符到开发服务器 URL 等所有信息。此文件也是 [Tauri CLI](/reference/cli/) 用来定位 Rust 项目的标记。若要了解更多信息，请参阅 [Tauri 配置](/develop/configuration-files/#tauri-config)。
  * `capabilities/` 目录是 Tauri 读取[功能（Capability）文件](/security/capabilities/)的默认文件夹（简而言之，你需要在此处允许命令，以便在 JavaScript 代码中使用它们）。若要了解更多信息，请参阅[安全性](/security/)。
  * `icons/` 目录是 [`tauri icon`](/reference/cli/#icon) 命令的默认输出目录。它通常在 `tauri.conf.json > bundle > icon` 中被引用，并用于应用的图标。
  * `build.rs` 包含了用于 Tauri 构建系统的 `tauri_build::build()` 调用。
  * `src/lib.rs` 包含了 Rust 代码和移动端入口点（标记为 `#[cfg_attr(mobile, tauri::mobile_entry_point)]` 的函数）。我们不在 `main.rs` 中直接编写代码，是因为在移动端构建时，我们将应用编译为库，并通过平台框架加载它。
  * `src/main.rs` 是桌面端的默认入口点。我们在 `main` 中运行 `app_lib::run()` 以复用与移动端相同的入口点。为了保持简单，请不要修改此文件，而是修改 `lib.rs`。注意 `app_lib` 对应于 `Cargo.toml` 中的 `[lib.name]`。

Tauri 的工作方式类似于静态 Web 主机。其构建过程是：首先将 JavaScript 项目编译为静态文件，然后编译 Rust 项目，并将这些静态文件打包进去。因此，JavaScript 项目的设置基本与构建静态网站相同。若要了解更多信息，请参阅 [前端配置](/start/frontend/)。

如果你只想使用 Rust 代码，只需移除其他所有内容，并将 `src-tauri/` 文件夹作为你的顶层项目，或者作为 Rust 工作空间（Workspace）的一个成员即可。

## 下一步

名为“后续步骤”的章节

  * [添加并配置前端框架](/start/frontend/)
  * [Tauri 命令行界面 (CLI) 参考](/reference/cli/)
  * [学习如何开发你的 Tauri 应用](/develop/)
  * [探索扩展 Tauri 的更多功能](/plugin/)