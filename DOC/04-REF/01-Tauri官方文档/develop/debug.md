# Debug

_Source: https://v2.tauri.org.cn/develop/debug/_

由于 Tauri 涉及许多动态部分，你可能会遇到需要调试的问题。错误详情会在多个位置输出，并且 Tauri 包含一些工具，可以使调试过程更加直观。

## 仅供开发使用的代码

名为“仅供开发使用的代码”的章节

调试工具包中最有用的工具之一是能够在代码中添加调试语句。然而，通常你不希望这些语句出现在生产环境中，这时能够检查当前是否处于开发模式就显得非常有用。

### 在 Rust 中

名为“在 Rust 中”的章节

    fn main() {

      // Whether the current instance was started with `tauri dev` or not.

      #[cfg(dev)]

      {

        // `tauri dev` only code

      }

      if cfg!(dev) {

        // `tauri dev` only code

      } else {

        // `tauri build` only code

      }

      let is_dev: bool = tauri::is_dev();

      // Whether debug assertions are enabled or not. This is true for `tauri dev` and `tauri build --debug`.

      #[cfg(debug_assertions)]

      {

        // Debug only code

      }

      if cfg!(debug_assertions) {

        // Debug only code

      } else {

        // Production only code

      }

    }

## Rust 控制台

名为“Rust 控制台”的章节

查找错误的首选位置是 Rust 控制台。这就是你运行例如 `tauri dev` 命令的终端。你可以使用以下代码在 Rust 文件中向该控制台打印内容：

    println!("Message from Rust: {}", msg);

有时你的 Rust 代码可能会出错，而 Rust 编译器可以提供大量信息。例如，如果 `tauri dev` 崩溃，你可以按照以下方式在 Linux 和 macOS 上重新运行它：

    RUST_BACKTRACE=1 tauri dev

或者在 Windows (PowerShell) 上这样运行：

    $env:RUST_BACKTRACE=1

    tauri dev

此命令将为你提供详细的堆栈跟踪。通常情况下，Rust 编译器会通过提供有关问题的详细信息来帮助你，例如：

    error[E0425]: cannot find value `sun` in this scope

      --> src/main.rs:11:5

       |

    11 |     sun += i.to_string().parse::<u64>().unwrap();

       |     ^^^ help: a local variable with a similar name exists: `sum`

    error: aborting due to previous error

    For more information about this error, try `rustc --explain E0425`.

## WebView 控制台

名为“WebView 控制台”的章节

在 WebView 中右键单击并选择 `Inspect Element`（检查元素）。这将打开一个类似于你习惯使用的 Chrome 或 Firefox 开发工具的 Web 检查器。你也可以使用 Linux 和 Windows 上的 `Ctrl + Shift + i` 快捷键，或 macOS 上的 `Command + Option + i` 快捷键来打开检查器。

检查器是特定于平台的，在 Linux 上渲染 webkit2gtk WebInspector，在 macOS 上渲染 Safari 检查器，在 Windows 上渲染 Microsoft Edge 开发工具。

### 以编程方式打开开发者工具

名为“以编程方式打开开发者工具”的章节

你可以使用 [`WebviewWindow::open_devtools`](https://docs.rs/tauri/2.0.0/tauri/webview/struct.WebviewWindow.html#method.open_devtools) 和 [`WebviewWindow::close_devtools`](https://docs.rs/tauri/2.0.0/tauri/webview/struct.WebviewWindow.html#method.close_devtools) 函数来控制检查器窗口的可见性。

    tauri::Builder::default()

      .setup(|app| {

        #[cfg(debug_assertions)] // only include this code on debug builds

        {

          let window = app.get_webview_window("main").unwrap();

          window.open_devtools();

          window.close_devtools();

        }

        Ok(())

      });

### 在生产环境中使用检查器

名为“在生产环境中使用检查器”的章节

默认情况下，除非你通过 Cargo 功能启用它，否则检查器仅在开发和调试版本中启用。

#### 创建调试版本

名为“创建调试版本”的章节

要创建调试版本，请运行 `tauri build --debug` 命令。

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri build -- --debug

    yarn tauri build --debug

    pnpm tauri build --debug

    deno task tauri build --debug

    bun tauri build --debug

    cargo tauri build --debug

与常规构建和开发过程一样，首次运行此命令时构建需要一些时间，但在后续运行中速度会显著加快。最终打包的应用程序已启用了开发控制台，并放置在 `src-tauri/target/debug/bundle` 中。

你还可以直接从终端运行已构建的应用程序，这样可以查看到 Rust 编译器的注解（如果出现错误）或你的 `println` 消息。浏览到文件 `src-tauri/target/(release|debug)/[应用程序名称]` 并在控制台中直接运行它，或在文件系统中双击该可执行文件（注意：使用此方法时，如果出现错误，控制台会立即关闭）。

##### 启用开发者工具功能

名为“启用开发者工具功能”的章节

危险

开发者工具 API 在 macOS 上是私有的。在 macOS 上使用私有 API 会导致你的应用程序无法被 App Store 接受。

要在**生产版本** 中启用开发者工具，你必须在 `src-tauri/Cargo.toml` 文件中启用 `devtools` Cargo 功能：

    [dependencies]

    tauri = { version = "...", features = ["...", "devtools"] }

## 调试核心进程

名为“调试核心进程”的章节

核心进程由 Rust 提供支持，因此你可以使用 GDB 或 LLDB 来调试它。你可以按照[在 VS Code 中调试](/develop/debug/vscode/)指南，了解如何使用 LLDB VS Code 扩展来调试 Tauri 应用程序的核心进程。