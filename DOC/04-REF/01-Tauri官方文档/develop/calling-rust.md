# 从前端调用 Rust

_Source: https://v2.tauri.org.cn/develop/calling-rust/_

本文档包含有关如何从应用程序前端与 Rust 代码进行通信的指南。要了解如何从 Rust 代码与前端进行通信，请参阅 [从 Rust 调用前端](/develop/calling-frontend/)。

Tauri 提供了一个用于以类型安全方式访问 Rust 函数的 命令 (command) 原语，以及一个更具动态性的 事件系统。

## 命令

标题为“命令”的章节

Tauri 提供了一个简单而强大的 `command` 系统，用于从你的 Web 应用调用 Rust 函数。命令可以接受参数并返回值。它们还可以返回错误并使用 `async` 修饰。

### 基本示例

名为“基本示例”的章节

可以在 `src-tauri/src/lib.rs` 文件中定义命令。要创建一个命令，只需添加一个函数并使用 `#[tauri::command]` 进行标注。

src-tauri/src/lib.rs

    #[tauri::command]

    fn my_custom_command() {

      println!("I was invoked from JavaScript!");

    }

注意

命令名称必须是唯一的。

注意

由于胶水代码生成中的限制，`lib.rs` 文件中定义的命令不能标记为 `pub`。如果你将其标记为公共函数，将会看到如下错误：

    error[E0255]: the name `__cmd__command_name` is defined multiple times

      --> src/lib.rs:28:8

       |

    27 | #[tauri::command]

       | ----------------- previous definition of the macro `__cmd__command_name` here

    28 | pub fn x() {}

       |        ^ `__cmd__command_name` reimported here

       |

       = note: `__cmd__command_name` must be defined only once in the macro namespace of this module

你需要将你的命令列表提供给构建器函数，如下所示：

src-tauri/src/lib.rs

    #[cfg_attr(mobile, tauri::mobile_entry_point)]

    pub fn run() {

      tauri::Builder::default()

        .invoke_handler(tauri::generate_handler![my_custom_command])

        .run(tauri::generate_context!())

        .expect("error while running tauri application");

    }

现在，你可以从 JavaScript 代码中调用该命令：

    // When using the Tauri API npm package:

    import { invoke } from '@tauri-apps/api/core';

    // When using the Tauri global script (if not using the npm package)

    // Be sure to set `app.withGlobalTauri` in `tauri.conf.json` to true

    const invoke = window.__TAURI__.core.invoke;

    // Invoke the command

    invoke('my_custom_command');

#### 在单独的模块中定义命令

名为“在单独的模块中定义命令”的章节

如果你的应用程序定义了许多组件，或者它们可以进行分组，你可以将命令定义在单独的模块中，而不是让 `lib.rs` 文件变得臃肿。

例如，让我们在 `src-tauri/src/commands.rs` 文件中定义一个命令：

src-tauri/src/commands.rs

    #[tauri::command]

    pub fn my_custom_command() {

      println!("I was invoked from JavaScript!");

    }

注意

在单独的模块中定义命令时，它们应该被标记为 `pub`。

注意

命令名称在模块之间不具备作用域，因此即使在不同模块之间，它们也必须是唯一的。

在 `lib.rs` 文件中，定义该模块并相应地提供命令列表；

src-tauri/src/lib.rs

    mod commands;

    #[cfg_attr(mobile, tauri::mobile_entry_point)]

    pub fn run() {

      tauri::Builder::default()

        .invoke_handler(tauri::generate_handler![commands::my_custom_command])

        .run(tauri::generate_context!())

        .expect("error while running tauri application");

    }

注意命令列表中的 `commands::` 前缀，它表示命令函数的完整路径。

本例中的命令名称是 `my_custom_command`，因此你仍然可以通过在前端执行 `invoke("my_custom_command")` 来调用它，`commands::` 前缀会被忽略。

#### WASM

名为“WASM”的章节

当使用 Rust 前端在不带参数的情况下调用 `invoke()` 时，你需要如下适配前端代码。原因是 Rust 不支持可选参数。

    #[wasm_bindgen]

    extern "C" {

        // invoke without arguments

        #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]

        async fn invoke_without_args(cmd: &str) -> JsValue;

        // invoke with arguments (default)

        #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]

        async fn invoke(cmd: &str, args: JsValue) -> JsValue;

        // They need to have different names!

    }

### 传递参数

名为“传递参数”的章节

你的命令处理器可以接收参数。

    #[tauri::command]

    fn my_custom_command(invoke_message: String) {

      println!("I was invoked from JavaScript, with this message: {}", invoke_message);

    }

参数应作为带有 camelCase（驼峰命名）键的 JSON 对象传递。

    invoke('my_custom_command', { invokeMessage: 'Hello!' });

注意

你可以使用 `rename_all` 属性将参数命名为 `snake_case`。

    #[tauri::command(rename_all = "snake_case")]

    fn my_custom_command(invoke_message: String) {}

相应的 JavaScript 代码：

    invoke('my_custom_command', { invoke_message: 'Hello!' });

参数可以是任何类型，只要它们实现了 [`serde::Deserialize`](https://docs.serde.rs/serde/trait.Deserialize.html)。

### 返回数据

名为“返回数据”的章节

命令处理器也可以返回数据。

    #[tauri::command]

    fn my_custom_command() -> String {

      "Hello from Rust!".into()

    }

`invoke` 函数返回一个 Promise，该 Promise 会解析为返回的值。

    invoke('my_custom_command').then((message) => console.log(message));

返回的数据可以是任何类型，只要它实现了 [`serde::Serialize`](https://docs.serde.rs/serde/trait.Serialize.html)。

#### 返回 Array Buffers

名为“返回 Array Buffers”的章节

当响应发送到前端时，实现了 [`serde::Serialize`](https://docs.serde.rs/serde/trait.Serialize.html) 的返回值会被序列化为 JSON。如果你尝试返回大量数据（如文件或 HTTP 下载响应），这可能会减慢你的应用程序速度。要以优化的方式返回 Array Buffers，请使用 [`tauri::ipc::Response`](https://docs.rs/tauri/2.0.0/tauri/ipc/struct.Response.html)。

    use tauri::ipc::Response;

    #[tauri::command]

    fn read_file() -> Response {

      let data = std::fs::read("/path/to/file").unwrap();

      tauri::ipc::Response::new(data)

    }

### 错误处理

名为“错误处理”的章节

如果你的处理器可能会失败并需要返回一个错误，请让该函数返回一个 `Result`。

    #[tauri::command]

    fn login(user: String, password: String) -> Result<String, String> {

      if user == "tauri" && password == "tauri" {

        // resolve

        Ok("logged_in".to_string())

      } else {

        // reject

        Err("invalid credentials".to_string())

      }

    }

如果命令返回错误，Promise 将会被拒绝 (reject)；否则，它会解析 (resolve)。

    invoke('login', { user: 'tauri', password: '0j4rijw8=' })

      .then((message) => console.log(message))

      .catch((error) => console.error(error));

如上所述，从命令返回的所有内容都必须实现 [`serde::Serialize`](https://docs.serde.rs/serde/trait.Serialize.html)，包括错误。如果你正在处理来自 Rust 标准库或外部 crate 的错误类型，这可能会有问题，因为大多数错误类型并未实现该 trait。在简单的场景下，你可以使用 `map_err` 将这些错误转换为 `String`。

    #[tauri::command]

    fn my_custom_command() -> Result<(), String> {

      std::fs::File::open("path/to/file").map_err(|err| err.to_string())?;

      // Return `null` on success

      Ok(())

    }

由于这不够惯用，你可能希望创建自己的错误类型，并实现 `serde::Serialize`。在下面的示例中，我们使用 [`thiserror`](https://github.com/dtolnay/thiserror) crate 来帮助创建错误类型。它允许你通过派生 `thiserror::Error` trait 将枚举转换为错误类型。你可以查阅其文档以获取更多详细信息。

    // create the error type that represents all errors possible in our program

    #[derive(Debug, thiserror::Error)]

    enum Error {

      #[error(transparent)]

      Io(#[from] std::io::Error)

    }

    // we must manually implement serde::Serialize

    impl serde::Serialize for Error {

      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>

      where

        S: serde::ser::Serializer,

      {

        serializer.serialize_str(self.to_string().as_ref())

      }

    }

    #[tauri::command]

    fn my_custom_command() -> Result<(), Error> {

      // This will return an error

      std::fs::File::open("path/that/does/not/exist")?;

      // Return `null` on success

      Ok(())

    }

自定义错误类型的好处是让所有可能的错误变得显式，从而使读者可以快速识别可能发生的错误。这在后续代码审查和重构时，可以为他人（以及你自己）节省大量时间。
它还让你能够完全控制错误类型的序列化方式。在上面的示例中，我们只是将错误消息作为字符串返回，但你可以为每个错误分配一个代码，以便更轻松地将其映射到类似的 TypeScript 错误枚举中。

    #[derive(Debug, thiserror::Error)]

    enum Error {

      #[error(transparent)]

      Io(#[from] std::io::Error),

      #[error("failed to parse as string: {0}")]

      Utf8(#[from] std::str::Utf8Error),

    }

    #[derive(serde::Serialize)]

    #[serde(tag = "kind", content = "message")]

    #[serde(rename_all = "camelCase")]

    enum ErrorKind {

      Io(String),

      Utf8(String),

    }

    impl serde::Serialize for Error {

      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>

      where

        S: serde::ser::Serializer,

      {

        let error_message = self.to_string();

        let error_kind = match self {

          Self::Io(_) => ErrorKind::Io(error_message),

          Self::Utf8(_) => ErrorKind::Utf8(error_message),

        };

        error_kind.serialize(serializer)

      }

    }

    #[tauri::command]

    fn read() -> Result<Vec<u8>, Error> {

      let data = std::fs::read("/path/to/file")?;

      Ok(data)

    }

现在，在前端你将得到一个 `{ kind: 'io' | 'utf8', message: string }` 的错误对象。

    type ErrorKind = {

      kind: 'io' | 'utf8';

      message: string;

    };

    invoke('read').catch((e: ErrorKind) => {});

### 异步命令

名为“异步命令”的章节

在 Tauri 中，建议使用异步命令来执行繁重的任务，以避免 UI 卡顿或变慢。

注意

异步命令使用 [`async_runtime::spawn`](https://docs.rs/tauri/2.0.0/tauri/async_runtime/fn.spawn.html) 在单独的异步任务中执行。没有 _async_ 关键字的命令在主线程上执行，除非使用 _#[tauri::command(async)]_ 定义。

**如果你的命令需要异步运行，只需将其声明为`async` 即可。**

注意

在使用 Tauri 创建异步函数时需要小心。目前，你不能简单地在异步函数的签名中包含借用参数。常见的此类类型包括 `&str` 和 `State<'_, Data>`。此限制正在此处跟踪：<https://github.com/tauri-apps/tauri/issues/2533>，下面给出了解决方法。

处理借用类型时，你必须进行额外的更改。这是你的两个主要选项：

**选项 1** ：将类型（如 `&str`）转换为非借用的类似类型（如 `String`）。这可能不适用于所有类型，例如 `State<'_, Data>`。

_示例_

    // Declare the async function using String instead of &str, as &str is borrowed and thus unsupported

    #[tauri::command]

    async fn my_custom_command(value: String) -> String {

      // Call another async function and wait for it to finish

      some_async_function().await;

      value

    }

**选项 2** ：将返回类型包装在 [`Result`](https://doc.rust-lang.net.cn/std/result/index.html) 中。这实现起来稍微困难一点，但适用于所有类型。

使用返回类型 `Result<a, b>`，将 `a` 替换为你想要返回的类型，或者如果你想返回 `null` 则使用 `()`；将 `b` 替换为如果出错时要返回的错误类型，或者如果你不想返回可选错误则使用 `()`。例如：

  * `Result<String, ()>` 返回 String，且无错误。
  * `Result<(), ()>` 返回 `null`。
  * `Result<bool, Error>` 返回布尔值或错误，如上面的 错误处理 章节所示。

_示例_

    // Return a Result<String, ()> to bypass the borrowing issue

    #[tauri::command]

    async fn my_custom_command(value: &str) -> Result<String, ()> {

      // Call another async function and wait for it to finish

      some_async_function().await;

      // Note that the return value must be wrapped in `Ok()` now.

      Ok(format!(value))

    }

##### 从 JavaScript 调用

名为“从 JavaScript 调用”的章节

由于从 JavaScript 调用命令已经返回一个 Promise，它的工作方式与其他任何命令一样。

    invoke('my_custom_command', { value: 'Hello, Async!' }).then(() =>

      console.log('Completed!')

    );

### 通道

名为“通道 (Channels)”的章节

Tauri 通道是向前端流式传输数据（例如流式 HTTP 响应）的推荐机制。以下示例读取一个文件，并以 4096 字节的块通知前端进度：

    use tokio::io::AsyncReadExt;

    #[tauri::command]

    async fn load_image(path: std::path::PathBuf, reader: tauri::ipc::Channel<&[u8]>) {

      // for simplicity this example does not include error handling

      let mut file = tokio::fs::File::open(path).await.unwrap();

      let mut chunk = vec![0; 4096];

      loop {

        let len = file.read(&mut chunk).await.unwrap();

        if len == 0 {

          // Length of zero means end of file.

          break;

        }

        reader.send(&chunk).unwrap();

      }

    }

有关更多信息，请参阅 [通道文档](/develop/calling-frontend/#channels)。

### 在命令中访问 WebviewWindow

名为“在命令中访问 WebviewWindow”的章节

命令可以访问调用消息的 `WebviewWindow` 实例。

src-tauri/src/lib.rs

    #[tauri::command]

    async fn my_custom_command(webview_window: tauri::WebviewWindow) {

      println!("WebviewWindow: {}", webview_window.label());

    }

### 在命令中访问 AppHandle

名为“在命令中访问 AppHandle”的章节

命令可以访问 `AppHandle` 实例。

src-tauri/src/lib.rs

    #[tauri::command]

    async fn my_custom_command(app_handle: tauri::AppHandle) {

      let app_dir = app_handle.path().app_dir();

      use tauri::GlobalShortcutManager;

      app_handle.global_shortcut_manager().register("CTRL + U", move || {});

    }

提示

`AppHandle` 和 `WebviewWindow` 都采用泛型参数 `R: Runtime`。当在 `tauri` 中启用了 `wry` 特性（默认启用）时，我们将泛型默认为 `Wry` 运行时，因此你可以直接使用它。但如果你想使用不同的运行时（例如 [mock 运行时](https://docs.rs/tauri/2.0.0/tauri/test/struct.MockRuntime.html)），则需要这样编写函数：

src-tauri/src/lib.rs

    use tauri::{AppHandle, GlobalShortcutManager, Runtime, WebviewWindow};

    #[tauri::command]

    async fn my_custom_command<R: Runtime>(app_handle: AppHandle<R>, webview_window: WebviewWindow<R>) {

      let app_dir = app_handle.path().app_dir();

      app_handle

        .global_shortcut_manager()

        .register("CTRL + U", move || {});

      println!("WebviewWindow: {}", webview_window.label());

    }

### 访问受管状态

名为“访问受管状态”的章节

Tauri 可以使用 `tauri::Builder` 上的 `manage` 函数来管理状态。可以通过 `tauri::State` 在命令中访问该状态。

src-tauri/src/lib.rs

    struct MyState(String);

    #[tauri::command]

    fn my_custom_command(state: tauri::State<MyState>) {

      assert_eq!(state.0 == "some state value", true);

    }

    #[cfg_attr(mobile, tauri::mobile_entry_point)]

    pub fn run() {

      tauri::Builder::default()

        .manage(MyState("some state value".into()))

        .invoke_handler(tauri::generate_handler![my_custom_command])

        .run(tauri::generate_context!())

        .expect("error while running tauri application");

    }

### 访问原始请求

名为“访问原始请求”的章节

Tauri 命令也可以访问完整的 [`tauri::ipc::Request`](https://docs.rs/tauri/2.0.0/tauri/ipc/struct.Request.html) 对象，其中包含原始主体负载和请求头。

    #[derive(Debug, thiserror::Error)]

    enum Error {

      #[error("unexpected request body")]

      RequestBodyMustBeRaw,

      #[error("missing `{0}` header")]

      MissingHeader(&'static str),

    }

    impl serde::Serialize for Error {

      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>

      where

        S: serde::ser::Serializer,

      {

        serializer.serialize_str(self.to_string().as_ref())

      }

    }

    #[tauri::command]

    fn upload(request: tauri::ipc::Request) -> Result<(), Error> {

      let tauri::ipc::InvokeBody::Raw(upload_data) = request.body() else {

        return Err(Error::RequestBodyMustBeRaw);

      };

      let Some(authorization_header) = request.headers().get("Authorization") else {

        return Err(Error::MissingHeader("Authorization"));

      };

      // upload...

      Ok(())

    }

在前端，你可以调用 invoke() 并通过在负载参数中提供 ArrayBuffer 或 Uint8Array 来发送原始请求主体，并在第三个参数中包含请求头。

    const data = new Uint8Array([1, 2, 3]);

    await __TAURI__.core.invoke('upload', data, {

      headers: {

        Authorization: 'apikey',

      },

    });

### 创建多个命令

名为“创建多个命令”的章节

`tauri::generate_handler!` 宏接受一个命令数组。要注册多个命令，你不能多次调用 invoke_handler。只有最后一次调用会被使用。你必须将每个命令传递给 `tauri::generate_handler!` 的单次调用中。

src-tauri/src/lib.rs

    #[tauri::command]

    fn cmd_a() -> String {

      "Command a"

    }

    #[tauri::command]

    fn cmd_b() -> String {

      "Command b"

    }

    #[cfg_attr(mobile, tauri::mobile_entry_point)]

    pub fn run() {

      tauri::Builder::default()

        .invoke_handler(tauri::generate_handler![cmd_a, cmd_b])

        .run(tauri::generate_context!())

        .expect("error while running tauri application");

    }

### 完整示例

名为“完整示例”的章节

以上任何或所有特性都可以结合使用。

src-tauri/src/lib.rs

    struct Database;

    #[derive(serde::Serialize)]

    struct CustomResponse {

      message: String,

      other_val: usize,

    }

    async fn some_other_function() -> Option<String> {

      Some("response".into())

    }

    #[tauri::command]

    async fn my_custom_command(

      window: tauri::Window,

      number: usize,

      database: tauri::State<'_, Database>,

    ) -> Result<CustomResponse, String> {

      println!("Called from {}", window.label());

      let result: Option<String> = some_other_function().await;

      if let Some(message) = result {

        Ok(CustomResponse {

          message,

          other_val: 42 + number,

        })

      } else {

        Err("No result".into())

      }

    }

    #[cfg_attr(mobile, tauri::mobile_entry_point)]

    pub fn run() {

      tauri::Builder::default()

        .manage(Database {})

        .invoke_handler(tauri::generate_handler![my_custom_command])

        .run(tauri::generate_context!())

        .expect("error while running tauri application");

    }

    import { invoke } from '@tauri-apps/api/core';

    // Invocation from JavaScript

    invoke('my_custom_command', {

      number: 42,

    })

      .then((res) =>

        console.log(`Message: ${res.message}, Other Val: ${res.other_val}`)

      )

      .catch((e) => console.error(e));

## 事件系统

名为“事件系统”的章节

事件系统是一种在你的前端和 Rust 之间进行通信的更简单的机制。与命令不同，事件不是类型安全的，总是异步的，不能返回值，并且仅支持 JSON 负载。

### 全局事件

名为“全局事件”的章节

要触发全局事件，你可以使用 [event.emit](/reference/javascript/api/namespaceevent/#emit) 或 [WebviewWindow#emit](/reference/javascript/api/namespacewebviewwindow/#emit) 函数。

    import { emit } from '@tauri-apps/api/event';

    import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';

    // emit(eventName, payload)

    emit('file-selected', '/path/to/file');

    const appWebview = getCurrentWebviewWindow();

    appWebview.emit('route-changed', { url: window.location.href });

注意

全局事件会被发送到**所有** 监听器。

### Webview 事件

名为“Webview 事件”的章节

要向由特定 webview 注册的监听器触发事件，你可以使用 [event.emitTo](/reference/javascript/api/namespaceevent/#emitto) 或 [WebviewWindow#emitTo](/reference/javascript/api/namespacewebviewwindow/#emitto) 函数。

    import { emitTo } from '@tauri-apps/api/event';

    import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';

    // emitTo(webviewLabel, eventName, payload)

    emitTo('settings', 'settings-update-requested', {

      key: 'notification',

      value: 'all',

    });

    const appWebview = getCurrentWebviewWindow();

    appWebview.emitTo('editor', 'file-changed', {

      path: '/path/to/file',

      contents: 'file contents',

    });

注意

Webview 特定的事件**不会** 触发常规的全局事件监听器。要监听**任何** 事件，你必须向 [event.listen](/reference/javascript/api/namespaceevent/#listen) 函数提供 `{ target: { kind: 'Any' } }` 选项，这将使监听器作为已触发事件的“全能捕获器”。

    import { listen } from '@tauri-apps/api/event';

    listen(

      'state-changed',

      (event) => {

        console.log('got state changed event', event);

      },

      {

        target: { kind: 'Any' },

      }

    );

### 监听事件

名为“监听事件”的章节

`@tauri-apps/api` NPM 包提供了监听全局事件和 webview 特定事件的 API。

  * 监听全局事件：

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

  * 监听 webview 特定事件：

        import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';

        const appWebview = getCurrentWebviewWindow();

        appWebview.listen<string>('logged-in', (event) => {

          localStorage.setItem('session-token', event.payload);

        });

`listen` 函数会将事件监听器注册在应用程序的整个生命周期内。要停止监听事件，你可以使用由 `listen` 函数返回的 `unlisten` 函数。

    import { listen } from '@tauri-apps/api/event';

    const unlisten = await listen('download-started', (event) => {});

    unlisten();

注意

当你的执行上下文超出作用域（例如组件被卸载时），务必使用 unlisten 函数。

当页面重新加载或你导航到另一个 URL 时，监听器会自动取消注册。但这不适用于单页应用 (SPA) 路由。

此外，Tauri 提供了一个用于仅监听一次事件的实用函数：

    import { once } from '@tauri-apps/api/event';

    import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';

    once('ready', (event) => {});

    const appWebview = getCurrentWebviewWindow();

    appWebview.once('ready', () => {});

注意

在前端触发的事件也会触发由这些 API 注册的监听器。有关更多信息，请参阅 [从前端调用 Rust](/develop/calling-rust/) 文档。

#### 在 Rust 中监听事件

名为“在 Rust 中监听事件”的章节

全局事件和 webview 特定事件也会发送到在 Rust 中注册的监听器。

  * 监听全局事件：

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

  * 监听 webview 特定事件：

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

`listen` 函数会将事件监听器注册在应用程序的整个生命周期内。要停止监听事件，你可以使用 `unlisten` 函数。

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

此外，Tauri 提供了一个用于仅监听一次事件的实用函数：

    app.once("ready", |event| {

      println!("app is ready");

    });

在这种情况下，事件监听器在第一次触发后会立即取消注册。

要了解如何监听事件以及从 Rust 代码中触发事件，请参阅 [Rust 事件系统文档](/develop/calling-frontend/#event-system)。