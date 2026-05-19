# Tauri 架构

_Source: https://v2.tauri.org.cn/concept/architecture/_

## 简介

名为“简介”的章节

Tauri 是一个多语言且通用的工具包，具有极高的可组合性，允许工程师构建各种各样的应用程序。它用于结合 Rust 工具和在 Webview 中渲染的 HTML 来构建桌面应用程序。使用 Tauri 构建的应用程序可以携带任意数量的可选 JS API 和 Rust API，以便 Webview 可以通过消息传递来控制系统。开发者可以轻松地使用自己的功能扩展默认 API，并连接 Webview 和基于 Rust 的后端。

Tauri 应用可以拥有[托盘界面](/learn/system-tray/)。它们可以被[更新](/plugin/updater/)，并像预期的那样由用户的操作系统管理。由于它们使用操作系统的 Webview，因此体积非常小。它们不包含运行时，因为最终的二进制文件是从 Rust 编译而来的。这使得[逆向工程 Tauri 应用变得不再是一件轻松的事](/security/)。

### Tauri 不是什么

名为“Tauri 不是什么”的章节

Tauri 不是轻量级的内核包装器。相反，它直接使用 WRY 和 TAO 来处理向操作系统进行系统调用的繁重工作。

Tauri 不是虚拟机或虚拟化环境。相反，它是一个允许构建 Webview 操作系统应用程序的工具包。

## 核心生态系统

名为“核心生态系统”的章节

![Diagram](/d2/docs/concept/architecture-0.svg)Tauri 架构的简化表示。

### tauri

名为“tauri”的章节

[在 GitHub 上查看](https://github.com/tauri-apps/tauri/tree/dev/crates/tauri)

这是将一切整合在一起的主要 crate。它将运行时、宏、实用程序和 API 汇集到一个最终产品中。它在编译时读取 [`tauri.conf.json`](/reference/config/) 文件以引入功能并执行应用程序的实际配置（甚至包括项目文件夹中的 `Cargo.toml` 文件）。它在运行时处理脚本注入（用于 polyfills / 原型修订），托管用于系统交互的 API，甚至管理更新过程。

### tauri-runtime

名为“tauri-runtime”的章节

[在 GitHub 上查看](https://github.com/tauri-apps/tauri/tree/dev/crates/tauri-runtime)

Tauri 本身与底层 Webview 库之间的粘合层。

### tauri-macros

名为“tauri-macros”的章节

[在 GitHub 上查看](https://github.com/tauri-apps/tauri/tree/dev/crates/tauri-macros)

通过利用 [`tauri-codegen`](https://github.com/tauri-apps/tauri/tree/dev/crates/tauri-codegen) crate 为上下文、处理程序和命令创建宏。

### tauri-utils

名为“tauri-utils”的章节

[在 GitHub 上查看](https://github.com/tauri-apps/tauri/tree/dev/crates/tauri-utils)

在许多地方重用的通用代码，提供有用的实用程序，如解析配置文件、检测平台三元组、注入 CSP 和管理资产。

### tauri-build

名为“tauri-build”的章节

[在 GitHub 上查看](https://github.com/tauri-apps/tauri/tree/dev/crates/tauri-build)

在构建时应用宏，以配置 `cargo` 所需的一些特殊功能。

### tauri-codegen

名为“tauri-codegen”的章节

[在 GitHub 上查看](https://github.com/tauri-apps/tauri/tree/dev/crates/tauri-codegen)

嵌入、哈希和压缩资产，包括应用的图标以及系统托盘。在编译时解析 [`tauri.conf.json`](/reference/config/) 并生成 Config 结构体。

### tauri-runtime-wry

名为“tauri-runtime-wry”的章节

[在 GitHub 上查看](https://github.com/tauri-apps/tauri/tree/dev/crates/tauri-runtime-wry)

这个 crate 专门为 WRY 开启了直接的系统级交互，如打印、监视器检测以及其他与窗口相关的任务。

## Tauri 工具链

名为“Tauri 工具链”的章节

### API (JavaScript / TypeScript)

名为“API (JavaScript / TypeScript)”的章节

[在 GitHub 上查看](https://github.com/tauri-apps/tauri/tree/dev/packages/api)

一个 TypeScript 库，它为您创建 `cjs` 和 `esm` JavaScript 端点，以便导入到您的前端框架中，从而使 Webview 可以调用和监听后端活动。它也以纯 TypeScript 形式发布，因为对于某些框架来说，这是更优化的选择。它利用了 Webview 与其宿主之间的消息传递。

### Bundler (Rust / Shell)

名为“Bundler (Rust / Shell)”的章节

[在 GitHub 上查看](https://github.com/tauri-apps/tauri/tree/dev/crates/tauri-bundler)

一个为它检测到的或指定的平台构建 Tauri 应用的库。目前支持 macOS、Windows 和 Linux——但在不久的将来也将支持移动平台。可以在 Tauri 项目之外使用。

### cli.rs (Rust)

名为“cli.rs (Rust)”的章节

[在 GitHub 上查看](https://github.com/tauri-apps/tauri/tree/dev/crates/tauri-cli)

这个 Rust 可执行文件提供了对所有需要 CLI 的活动的全功能接口。它运行在 macOS、Windows 和 Linux 上。

### cli.js (JavaScript)

名为“cli.js (JavaScript)”的章节

[在 GitHub 上查看](https://github.com/tauri-apps/tauri/tree/dev/packages/cli)

使用 [`napi-rs`](https://github.com/napi-rs/napi-rs) 对 [`cli.rs`](https://github.com/tauri-apps/tauri/blob/dev/crates/tauri-cli) 进行的包装，旨在为每个平台生成 npm 包。

### create-tauri-app (JavaScript)

名为“create-tauri-app (JavaScript)”的章节

[在 GitHub 上查看](https://github.com/tauri-apps/create-tauri-app)

一个工具包，使工程团队能够使用他们选择的前端框架（只要已配置）快速搭建一个新的 `tauri-apps` 项目。

## 上游 Crates

名为“上游 Crates”的章节

Tauri-Apps 组织维护着 Tauri 的两个“上游”crate，即用于创建和管理应用程序窗口的 TAO，以及用于与窗口内的 Webview 进行交互的 WRY。

### TAO

名为“TAO”的章节

[在 GitHub 上查看](https://github.com/tauri-apps/tao)

Rust 编写的跨平台应用程序窗口创建库，支持 Windows、macOS、Linux、iOS 和 Android 等所有主要平台。它使用 Rust 编写，是 [winit](https://github.com/rust-windowing/winit) 的一个分支，我们根据自己的需求对其进行了扩展——例如菜单栏和系统托盘。

### WRY

名为“WRY”的章节

[在 GitHub 上查看](https://github.com/tauri-apps/wry)

WRY 是一个 Rust 编写的跨平台 WebView 渲染库，支持 Windows、macOS 和 Linux 等所有主要桌面平台。Tauri 使用 WRY 作为抽象层，负责确定使用哪个 Webview（以及如何进行交互）。

## 其他工具

名为“其他工具”的章节

### tauri-action

名为“tauri-action”的章节

[在 GitHub 上查看](https://github.com/tauri-apps/tauri-action)

用于为所有平台构建 Tauri 二进制文件的 GitHub 工作流。即使没有设置 Tauri，也可以创建（非常基础的）Tauri 应用。

### tauri-vscode

名为“tauri-vscode”的章节

[在 GitHub 上查看](https://github.com/tauri-apps/tauri-vscode)

该项目通过一些实用的功能增强了 Visual Studio Code 界面。

### vue-cli-plugin-tauri

名为“vue-cli-plugin-tauri”的章节

[在 GitHub 上查看](https://github.com/tauri-apps/vue-cli-plugin-tauri)

允许您在 vue-cli 项目中非常快速地安装 Tauri。

## 插件

名为“插件”的章节

[Tauri 插件指南](/develop/plugins/)

总的来说，插件是由第三方编写的（尽管可能有官方支持的插件）。一个插件通常做 3 件事

  1. 使 Rust 代码能够做“某事”。
  2. 提供接口粘合代码，以便轻松集成到应用程序中。
  3. 提供用于与 Rust 代码交互的 JavaScript API。

以下是一些 Tauri 插件的示例

  * [tauri-plugin-fs](https://github.com/tauri-apps/tauri-plugin-fs)
  * [tauri-plugin-sql](https://github.com/tauri-apps/tauri-plugin-sql)
  * [tauri-plugin-stronghold](https://github.com/tauri-apps/tauri-plugin-stronghold)

## 许可证

名为“许可证”的章节

Tauri 本身在 MIT 或 Apache-2.0 下获得许可。如果您重新打包并修改了任何源代码，您有责任验证是否符合所有上游许可证。Tauri 按“原样”提供，不对任何目的的适用性做出明确声明。

在这里，您可以仔细阅读我们的 [软件物料清单 (SBOM)](https://app.fossa.com/projects/git%2Bgithub.com%2Ftauri-apps%2Ftauri)。