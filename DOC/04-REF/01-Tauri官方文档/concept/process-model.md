# 进程模型

_Source: https://v2.tauri.org.cn/concept/process-model/_

Tauri 采用了类似于 Electron 或许多现代网页浏览器的多进程架构。本指南将探讨做出这一设计选择的原因，以及它为何是编写安全应用程序的关键。

## 为什么要使用多进程？

标题为“为什么要使用多进程？”的章节

在 GUI 应用程序的早期，通常使用单个进程来执行计算、绘制界面并响应用户输入。正如你可能猜到的，这意味着长时间运行的昂贵计算会导致用户界面失去响应，或者更糟糕的是，应用程序的一个组件发生故障会导致整个应用程序崩溃。

显而易见，我们需要一种更具弹性的架构，于是应用程序开始在不同的进程中运行不同的组件。这能更好地利用现代多核 CPU，并构建出更安全的应用程序。如果一个组件崩溃，不会再影响整个系统，因为组件被隔离在不同的进程中。如果某个进程进入了无效状态，我们可以轻松地将其重启。

我们还可以通过仅向每个进程授予完成其工作所需的最低限度的权限，来限制潜在漏洞的影响范围。这种模式被称为“[最小特权原则](https://en.wikipedia.org/wiki/Principle_of_least_privilege)”，你在现实生活中经常会见到它。如果你请园丁来修剪树篱，你会给他们花园的钥匙。你**绝不会**给他们房子的钥匙；他们为什么要访问那里呢？同样的概念也适用于计算机程序。我们给予它们的访问权限越少，如果它们被攻破，所能造成的危害就越小。

## 核心进程

标题为“核心进程”的章节

每个 Tauri 应用程序都有一个核心进程，它是应用程序的入口点，也是唯一拥有完整操作系统访问权限的组件。

核心的主要职责是利用这种访问权限来创建和编排应用程序窗口、系统托盘菜单或通知。Tauri 实现了必要的跨平台抽象，使这些操作变得简单。它还将所有[进程间通信 (IPC)](/concept/inter-process-communication/) 路由通过核心进程，允许你在一个中心位置拦截、过滤和操作 IPC 消息。

核心进程还应负责管理全局状态，例如设置或数据库连接。这使你可以轻松地在窗口之间同步状态，并保护你的业务敏感数据免受前端窥探。

我们选择 Rust 来实现 Tauri，是因为其“[所有权](https://doc.rust-lang.net.cn/book/ch04-01-what-is-ownership.html)”概念在保持卓越性能的同时，保证了内存安全。

![Diagram](/d2/docs/concept/process-model-0.svg) Tauri 进程模型的简化表示。单个核心进程管理一个或多个 WebView 进程。

## WebView 进程

标题为“WebView 进程”的章节

核心进程本身不渲染实际的用户界面 (UI)；它会启动 WebView 进程，利用操作系统提供的 WebView 库。WebView 是一个类浏览器环境，用于执行你的 HTML、CSS 和 JavaScript。

这意味着你可以使用在传统 Web 开发中使用的绝大多数技术和工具来创建 Tauri 应用程序。例如，许多 Tauri 示例都是使用 [Svelte](https://svelte.net.cn/) 前端框架和 [Vite](https://vite.org.cn/) 打包工具编写的。

安全最佳实践同样适用；例如，你必须始终对用户输入进行清理，严禁在前端处理机密信息，并且理想情况下应尽可能将业务逻辑交给核心进程处理，以减小你的攻击面。

与其他类似解决方案不同，WebView 库**不会** 包含在你的最终可执行文件中，而是在运行时动态链接1。这使你的应用程序体积 _显著_ 更小，但也意味着你需要像传统 Web 开发一样，考虑到不同平台之间的差异。

## 脚注

标题为“脚注”的章节

  1. 目前，Tauri 在 Windows 上使用 [Microsoft Edge WebView2](https://docs.microsoft.com/en-us/microsoft-edge/webview2/)，在 macOS 上使用 [WKWebView](https://developer.apple.com/documentation/webkit/wkwebview)，在 Linux 上使用 [webkitgtk](https://webkitgtk.org)。↩