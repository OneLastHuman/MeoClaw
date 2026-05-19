# 状态管理

_Source: https://v2.tauri.org.cn/develop/state-management/_

在 Tauri 应用中，你通常需要跟踪应用的当前状态或管理与其相关的生命周期。Tauri 提供了一种简单的方法，利用 [`Manager`](https://docs.rs/tauri/latest/tauri/trait.Manager.html) API 来管理应用状态，并在调用命令时读取它。

这是一个简单的示例

    use tauri::{Builder, Manager};

    struct AppData {

      welcome_message: &'static str,

    }

    fn main() {

      Builder::default()

        .setup(|app| {

          app.manage(AppData {

            welcome_message: "Welcome to Tauri!",

          });

          Ok(())

        })

        .run(tauri::generate_context!())

        .unwrap();

    }

稍后，你可以使用任何实现了 [`Manager`](https://docs.rs/tauri/latest/tauri/trait.Manager.html) 特型的类型（例如 [`App`](https://docs.rs/tauri/latest/tauri/struct.App.html) 实例）来访问你的状态。

    let data = app.state::<AppData>();

更多信息（包括如何在命令中访问状态）请参见访问状态章节。

## 可变性 (Mutability)

标题为“可变性”的章节

在 Rust 中，你不能直接修改在多个线程之间共享的值，或者当所有权通过共享指针（如 [`Arc`](https://doc.rust-lang.net.cn/stable/std/sync/struct.Arc.html) 或 Tauri 的 [`State`](https://docs.rs/tauri/latest/tauri/struct.State.html)）控制时。这样做可能会导致数据竞争（例如，两个写操作同时发生）。

为了解决这个问题，你可以使用一个称为[内部可变性 (interior mutability)](https://doc.rust-lang.net.cn/book/ch15-05-interior-mutability.html)的概念。例如，标准库的 [`Mutex`](https://doc.rust-lang.net.cn/stable/std/sync/struct.Mutex.html) 可以用来包裹你的状态。这允许你在需要修改值时锁定它，并在完成后将其解锁。

    use std::sync::Mutex;

    use tauri::{Builder, Manager};

    #[derive(Default)]

    struct AppState {

      counter: u32,

    }

    fn main() {

      Builder::default()

        .setup(|app| {

          app.manage(Mutex::new(AppState::default()));

          Ok(())

        })

        .run(tauri::generate_context!())

        .unwrap();

    }

现在可以通过锁定互斥锁来修改状态

    let state = app.state::<Mutex<AppState>>();

    // Lock the mutex to get mutable access:

    let mut state = state.lock().unwrap();

    // Modify the state:

    state.counter += 1;

在作用域结束时，或当 `MutexGuard` 被丢弃时，互斥锁会自动解锁，以便应用的其他部分可以访问并修改其中的数据。

### 何时使用异步互斥锁 (async mutex)

标题为“何时使用异步互斥锁”的章节

引用 [Tokio 文档](https://docs.rs/tokio/latest/tokio/sync/struct.Mutex.html#which-kind-of-mutex-should-you-use)中的说法，在异步代码中使用标准库的 [`Mutex`](https://doc.rust-lang.net.cn/stable/std/sync/struct.Mutex.html) 而不是 Tokio 提供的异步互斥锁通常是可以的：

> 与普遍的看法相反，在异步代码中使用来自标准库的普通 Mutex 是可以的，而且通常也是首选……异步互斥锁的主要用例是为 IO 资源（例如数据库连接）提供共享的可变访问。

完整阅读链接中的文档以了解两者之间的权衡是个好主意。你需要异步互斥锁的一个原因是：如果你需要在 await 点之间持有 `MutexGuard`。

### 你需要 `Arc` 吗？

标题为“需要 Arc 吗？”的章节

在 Rust 中经常看到使用 [`Arc`](https://doc.rust-lang.net.cn/stable/std/sync/struct.Arc.html) 来跨线程共享值的所有权（通常与 [`Mutex`](https://doc.rust-lang.net.cn/stable/std/sync/struct.Mutex.html) 搭配，形式为 `Arc<Mutex<T>>`）。然而，你不需要为存储在 [`State`](https://docs.rs/tauri/latest/tauri/struct.State.html) 中的内容使用 [`Arc`](https://doc.rust-lang.net.cn/stable/std/sync/struct.Arc.html)，因为 Tauri 会为你处理。

如果 `State` 的生命周期要求阻止你将状态移入新线程，你可以改为将 `AppHandle` 移入线程，然后按照“使用 Manager 特型访问状态”部分所述获取你的状态。`AppHandle` 可以轻量地克隆，适用于此类用例。

## 访问状态

标题为“访问状态”的章节

### 在命令中访问状态

标题为“在命令中访问状态”的章节

    #[tauri::command]

    fn increase_counter(state: State<'_, Mutex<AppState>>) -> u32 {

      let mut state = state.lock().unwrap();

      state.counter += 1;

      state.counter

    }

有关命令的更多信息，请参见[从前端调用 Rust](/develop/calling-rust/)。

#### 异步命令

标题为“异步命令”的章节

如果你正在使用 `async` 命令并希望使用 Tokio 的异步 [`Mutex`](https://docs.rs/tokio/latest/tokio/sync/struct.Mutex.html)，你可以以相同的方式进行设置，并按如下方式访问状态

    #[tauri::command]

    async fn increase_counter(state: State<'_, Mutex<AppState>>) -> Result<u32, ()> {

      let mut state = state.lock().await;

      state.counter += 1;

      Ok(state.counter)

    }

请注意，如果你使用异步命令，返回类型必须是 [`Result`](https://doc.rust-lang.net.cn/stable/std/result/index.html)。

### 使用 [`Manager`](https://docs.rs/tauri/latest/tauri/trait.Manager.html) 特型访问状态

标题为“使用 Manager 特型访问状态”的章节

有时你可能需要在命令之外访问状态，例如在不同的线程中，或在类似 `on_window_event` 的事件处理程序中。在这种情况下，你可以使用实现了 [`Manager`](https://docs.rs/tauri/latest/tauri/trait.Manager.html) 特型的类型（例如 `AppHandle`）的 `state()` 方法来获取状态。

    use std::sync::Mutex;

    use tauri::{Builder, Window, WindowEvent, Manager};

    #[derive(Default)]

    struct AppState {

      counter: u32,

    }

    // In an event handler:

    fn on_window_event(window: &Window, _event: &WindowEvent) {

        // Get a handle to the app so we can get the global state.

        let app_handle = window.app_handle();

        let state = app_handle.state::<Mutex<AppState>>();

        // Lock the mutex to mutably access the state.

        let mut state = state.lock().unwrap();

        state.counter += 1;

    }

    fn main() {

      Builder::default()

        .setup(|app| {

          app.manage(Mutex::new(AppState::default()));

          Ok(())

        })

        .on_window_event(on_window_event)

        .run(tauri::generate_context!())

        .unwrap();

    }

当无法依赖命令注入时，此方法非常有用。例如，如果你需要将状态移入一个使用 `AppHandle` 更容易的线程，或者如果你不在命令上下文中。

## 类型不匹配

标题为“类型不匹配”的章节

注意

如果你为 [`State`](https://docs.rs/tauri/latest/tauri/struct.State.html) 参数使用了错误的类型，将会导致运行时恐慌 (panic) 而不是编译时错误。

例如，如果你使用 `State<'_, AppState>` 而不是 `State<'_, Mutex<AppState>>`，则不会有任何状态以该类型进行管理。

如果你愿意，可以使用类型别名来包裹你的状态，以防止出现这种错误。

    use std::sync::Mutex;

    #[derive(Default)]

    struct AppStateInner {

      counter: u32,

    }

    type AppState = Mutex<AppStateInner>;

但是，请确保直接使用该类型别名，而不要将其再次包裹在 [`Mutex`](https://doc.rust-lang.net.cn/stable/std/sync/struct.Mutex.html) 中，否则你将遇到同样的问题。