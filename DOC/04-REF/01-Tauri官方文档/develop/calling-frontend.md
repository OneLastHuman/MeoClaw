# 从 Rust 调用前端

_Source: https://v2.tauri.org.cn/develop/calling-frontend/_

本文档包含有关如何从 Rust 代码与应用程序前端进行通信的指南。要了解如何从前端与 Rust 代码通信，请参阅 [从前端调用 Rust](/develop/calling-rust/)。

Tauri 应用程序的 Rust 端可以通过利用 Tauri 事件系统、使用通道 (channels) 或直接执行 JavaScript 代码来调用前端。

## 事件系统

名为“事件系统”的章节

Tauri 提供了一个简单的事件系统，您可以使用它在 Rust 和前端之间进行双向通信。

事件系统专为需要传输少量数据或需要实现多消费者多生产者模式（例如推送通知系统）的情况而设计。

事件系统并非为低延迟或高吞吐量场景而设计。有关针对数据流优化的实现，请参阅 通道 (Channels) 部分。

Tauri 命令 (Command) 和 Tauri 事件之间的主要区别在于：事件没有强类型支持，事件负载始终是 JSON 字符串，因此不适合传递大数据，且不支持 [能力 (capabilities)](/security/capabilities/) 系统来精细控制事件数据和通道。

[AppHandle](https://docs.rs/tauri/2.0.0/tauri/struct.AppHandle.html) 和 [WebviewWindow](https://docs.rs/tauri/2.0.0/tauri/webview/struct.WebviewWindow.html) 类型实现了事件系统特征 [Listener](https://docs.rs/tauri/2.0.0/tauri/trait.Listener.html) 和 [Emitter](https://docs.rs/tauri/2.0.0/tauri/trait.Emitter.html)。

事件要么是全局的（分发给所有监听器），要么是特定于 Webview 的（仅分发给匹配给定标签的 Webview）。

### 全局事件

标题为“全局事件”的章节

要触发全局事件，可以使用 [Emitter#emit](https://docs.rs/tauri/2.0.0/tauri/trait.Emitter.html#tymethod.emit) 函数。

src-tauri/src/lib.rs

    use tauri::{AppHandle, Emitter};

    #[tauri::command]

    fn download(app: AppHandle, url: String) {

      app.emit("download-started", &url).unwrap();

      for progress in [1, 15, 50, 80, 100] {

        app.emit("download-progress", progress).unwrap();

      }

      app.emit("download-finished", &url).unwrap();

    }

注意

全局事件会发送给**所有** 监听器。

### Webview 事件

标题为“Webview 事件”的章节

要触发特定 Webview 注册的监听器，可以使用 [Emitter#emit_to](https://docs.rs/tauri/2.0.0/tauri/trait.Emitter.html#tymethod.emit_to) 函数。

src-tauri/src/lib.rs

    use tauri::{AppHandle, Emitter};

    #[tauri::command]

    fn login(app: AppHandle, user: String, password: String) {

      let authenticated = user == "tauri-apps" && password == "tauri";

      let result = if authenticated { "loggedIn" } else { "invalidCredentials" };

      app.emit_to("login", "login-result", result).unwrap();

    }

还可以通过调用 [Emitter#emit_filter](https://docs.rs/tauri/2.0.0/tauri/trait.Emitter.html#tymethod.emit_filter) 向一系列 Webview 触发事件。在下面的示例中，我们向 main 和 file-viewer Webview 发送了一个 open-file 事件。

src-tauri/src/lib.rs

    use tauri::{AppHandle, Emitter, EventTarget};

    #[tauri::command]

    fn open_file(app: AppHandle, path: std::path::PathBuf) {

      app.emit_filter("open-file", path, |target| match target {

        EventTarget::WebviewWindow { label } => label == "main" || label == "file-viewer",

        _ => false,

      }).unwrap();

    }

注意

特定于 Webview 的事件**不会** 触发常规的全局事件监听器。要监听**任何** 事件，必须使用 `listen_any` 函数代替 `listen`，该函数定义监听器作为已触发事件的“捕获所有”监听器。

### 事件负载 (Payload)

标题为“事件负载”的章节

事件负载可以是任何同时实现了 [可序列化 (Serializable)](https://serde.rs/impl-serialize.html) 特征和 [Clone](https://doc.rust-lang.net.cn/std/clone/trait.Clone.html) 特征的类型。让我们通过使用对象来增强下载事件示例，以便在每个事件中发出更多信息。

src-tauri/src/lib.rs

    use tauri::{AppHandle, Emitter};

    use serde::Serialize;

    #[derive(Clone, Serialize)]

    #[serde(rename_all = "camelCase")]

    struct DownloadStarted<'a> {

      url: &'a str,

      download_id: usize,

      content_length: usize,

    }

    #[derive(Clone, Serialize)]

    #[serde(rename_all = "camelCase")]

    struct DownloadProgress {

      download_id: usize,

      chunk_length: usize,

    }

    #[derive(Clone, Serialize)]

    #[serde(rename_all = "camelCase")]

    struct DownloadFinished {

      download_id: usize,

    }

    #[tauri::command]

    fn download(app: AppHandle, url: String) {

      let content_length = 1000;

      let download_id = 1;

      app.emit("download-started", DownloadStarted {

        url: &url,

        download_id,

        content_length

      }).unwrap();

      for chunk_length in [15, 150, 35, 500, 300] {

        app.emit("download-progress", DownloadProgress {

          download_id,

          chunk_length,

        }).unwrap();

      }

      app.emit("download-finished", DownloadFinished { download_id }).unwrap();

    }

### 监听事件

标题为“监听事件”的章节

Tauri 提供了在 Webview 和 Rust 接口上监听事件的 API。

#### 在前端监听事件

标题为“在前端监听事件”的章节

`@tauri-apps/api` NPM 包提供了用于监听全局事件和特定于 Webview 事件的 API。

  * 监听全局事件

        import { listen } from '@tauri-apps/api/event';

        type DownloadStarted = {

          url: string;

          downloadId: number;

          contentLength: number;

        };

        listen<DownloadStarted>('download-started', (event) => {

          console.log(

            `downloading ${event.payload.contentLength} bytes from ${event.payload.url}`

          );

        });

  * 监听特定于 Webview 的事件

        import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';

        const appWebview = getCurrentWebviewWindow();

        appWebview.listen<string>('logged-in', (event) => {

          localStorage.setItem('session-token', event.payload);

        });

`listen` 函数会将事件监听器注册在应用程序的整个生命周期内。要停止监听事件，可以使用 `listen` 函数返回的 `unlisten` 函数。

    import { listen } from '@tauri-apps/api/event';

    const unlisten = await listen('download-started', (event) => {});

    unlisten();

注意

当您的执行上下文超出范围（例如组件卸载时），请务必使用 unlisten 函数。

当页面重新加载或导航到另一个 URL 时，监听器会自动注销。但这不适用于单页应用 (SPA) 路由。

此外，Tauri 提供了一个用于仅监听一次事件的实用函数。

    import { once } from '@tauri-apps/api/event';

    import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';

    once('ready', (event) => {});

    const appWebview = getCurrentWebviewWindow();

    appWebview.once('ready', () => {});

注意

在前端发出的事件也会触发通过这些 API 注册的监听器。有关更多信息，请参阅 [从前端调用 Rust](/develop/calling-rust/) 文档。

#### 在 Rust 中监听事件

标题为“在 Rust 中监听事件”的章节

全局和特定于 Webview 的事件也会发送给在 Rust 中注册的监听器。

  * 监听全局事件

src-tauri/src/lib.rs

        use tauri::Listener;

        #[cfg_attr(mobile, tauri::mobile_entry_point)]

        pub fn run() {

          tauri::Builder::default()

            .setup(|app| {

              app.listen("download-started", |event| {

                if let Ok(payload) = serde_json::from_str::<DownloadStarted>(&event.payload()) {

                  println!("downloading {}", payload.url);

                }

              });

              Ok(())

            })

            .run(tauri::generate_context!())

            .expect("error while running tauri application");

        }

  * 监听特定于 Webview 的事件

src-tauri/src/lib.rs

        use tauri::{Listener, Manager};

        #[cfg_attr(mobile, tauri::mobile_entry_point)]

        pub fn run() {

          tauri::Builder::default()

            .setup(|app| {

              let webview = app.get_webview_window("main").unwrap();

              webview.listen("logged-in", |event| {

                let session_token = event.data;

                // save token..

              });

              Ok(())

            })

            .run(tauri::generate_context!())

            .expect("error while running tauri application");

        }

`listen` 函数会将事件监听器注册在应用程序的整个生命周期内。要停止监听事件，可以使用 `unlisten` 函数。

    // unlisten outside of the event handler scope:

    let event_id = app.listen("download-started", |event| {});

    app.unlisten(event_id);

    // unlisten when some event criteria is matched

    let handle = app.handle().clone();

    app.listen("status-changed", |event| {

      if event.data == "ready" {

        handle.unlisten(event.id);

      }

    });

此外，Tauri 提供了一个用于仅监听一次事件的实用函数。

    app.once("ready", |event| {

      println!("app is ready");

    });

在这种情况下，事件监听器在第一次触发后会立即注销。

## 通道

标题为“通道 (Channels)”的章节

事件系统旨在实现一种在应用程序中全局可用的简单双向通信。在底层，它直接执行 JavaScript 代码，因此可能不适合发送大量数据。

通道 (Channels) 的设计目的是快速并传递有序数据。它们在内部用于流式传输操作，例如下载进度、子进程输出和 WebSocket 消息。

让我们重写下载命令示例，使用通道而不是事件系统。

src-tauri/src/lib.rs

    use tauri::{AppHandle, ipc::Channel};

    use serde::Serialize;

    #[derive(Clone, Serialize)]

    #[serde(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "event", content = "data")]

    enum DownloadEvent<'a> {

      Started {

        url: &'a str,

        download_id: usize,

        content_length: usize,

      },

      Progress {

        download_id: usize,

        chunk_length: usize,

      },

      Finished {

        download_id: usize,

      },

    }

    #[tauri::command]

    fn download(app: AppHandle, url: String, on_event: Channel<DownloadEvent>) {

      let content_length = 1000;

      let download_id = 1;

      on_event.send(DownloadEvent::Started {

        url: &url,

        download_id,

        content_length,

      }).unwrap();

      for chunk_length in [15, 150, 35, 500, 300] {

        on_event.send(DownloadEvent::Progress {

          download_id,

          chunk_length,

        }).unwrap();

      }

      on_event.send(DownloadEvent::Finished { download_id }).unwrap();

    }

调用下载命令时，必须创建通道并将其作为参数提供。

    import { invoke, Channel } from '@tauri-apps/api/core';

    type DownloadEvent =

      | {

          event: 'started';

          data: {

            url: string;

            downloadId: number;

            contentLength: number;

          };

        }

      | {

          event: 'progress';

          data: {

            downloadId: number;

            chunkLength: number;

          };

        }

      | {

          event: 'finished';

          data: {

            downloadId: number;

          };

        };

    const onEvent = new Channel<DownloadEvent>();

    onEvent.onmessage = (message) => {

      console.log(`got download event ${message.event}`);

    };

    await invoke('download', {

      url: 'https://raw.githubusercontent.com/tauri-apps/tauri/dev/crates/tauri-schema-generator/schemas/config.schema.json',

      onEvent,

    });

## 执行 JavaScript

标题为“执行 JavaScript”的章节

要直接在 Webview 上下文中执行任何 JavaScript 代码，可以使用 [`WebviewWindow#eval`](https://docs.rs/tauri/2.0.0/tauri/webview/struct.WebviewWindow.html#method.eval) 函数。

src-tauri/src/lib.rs

    use tauri::Manager;

    tauri::Builder::default()

      .setup(|app| {

        let webview = app.get_webview_window("main").unwrap();

        webview.eval("console.log('hello from Rust')")?;

        Ok(())

      })

如果需要评估的脚本比较复杂，并且必须使用来自 Rust 对象的数据，我们建议使用 [serialize-to-javascript](https://docs.rs/serialize-to-javascript/latest/serialize_to_javascript/) crate。