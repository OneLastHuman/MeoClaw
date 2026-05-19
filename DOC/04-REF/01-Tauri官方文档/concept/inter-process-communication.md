# 进程间通信

_Source: https://v2.tauri.org.cn/concept/inter-process-communication/_

进程间通信 (IPC) 允许隔离的进程安全地进行通信，这是构建复杂应用程序的关键。

通过以下指南了解更多关于特定 IPC 模式的信息

[ Brownfield (遗留集成) ](/concept/inter-process-communication/brownfield/)

[ 隔离 ](/concept/inter-process-communication/isolation/)

Tauri 使用一种称为[异步消息传递](https://en.wikipedia.org/wiki/Message_passing#Asynchronous_message_passing)的进程间通信方式，即进程之间交换使用简单数据表示形式序列化的 _请求_ 和 _响应_ 。对于有 Web 开发经验的人来说，消息传递的概念应该很熟悉，因为互联网上的客户端-服务器通信也是采用这种范式。

消息传递比共享内存或直接函数访问更安全，因为接收方可以自由地根据需要拒绝或丢弃请求。例如，如果 Tauri Core 进程判定某个请求是恶意的，它只需丢弃该请求，而不会执行对应的函数。

下面，我们将详细说明 Tauri 的两种 IPC 原语——`事件 (Events)` 和 `命令 (Commands)`。

## 事件

标题为“事件”的章节

事件是“即发即弃”的单向 IPC 消息，最适合用于通信生命周期事件和状态变化。与 命令 (Commands) 不同，事件既可以由前端发出，也可以由 Tauri Core 发出。

![Diagram](/d2/docs/concept/Inter-Process%20Communication/index-0.svg)在 Core 和 Webview 之间发送的事件。

## 命令

标题为“命令”的章节

Tauri 还在 IPC 消息之上提供了一种类似于[外部函数接口 (FFI)](https://en.wikipedia.org/wiki/Foreign_function_interface) 的抽象1。其主要 API `invoke` 与浏览器的 `fetch` API 类似，允许前端调用 Rust 函数、传递参数并接收数据。

由于该机制在底层使用类 [JSON-RPC](https://www.jsonrpc.org) 协议来序列化请求和响应，因此所有参数和返回数据必须能够序列化为 JSON。

![Diagram](/d2/docs/concept/Inter-Process%20Communication/index-1.svg)命令调用中涉及的 IPC 消息。

## 脚注

标题为“脚注”的章节

  1. 由于命令在底层仍然使用消息传递，因此它们不会像真正的 FFI 接口那样存在相同的安全隐患。 ↩