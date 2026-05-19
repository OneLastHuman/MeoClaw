# WebDriver

_Source: https://v2.tauri.org.cn/develop/tests/webdriver/_

[WebDriver](https://www.w3.org/TR/webdriver/) 是一种用于与 Web 文档交互的标准化接口，主要用于自动化测试。Tauri 通过在跨平台封装 [`tauri-driver`](https://crates.io/crates/tauri-driver) 下利用原生平台的 [WebDriver](https://www.w3.org/TR/webdriver/) 服务器来支持 [WebDriver](https://www.w3.org/TR/webdriver/) 接口。在桌面端，由于 macOS 没有可用的 WKWebView 驱动工具，目前仅支持 Windows 和 Linux。iOS 和 Android 可通过 Appium 2 实现，但该流程目前尚未简化。

## 系统依赖

标题为“系统依赖”的章节

安装最新版的 [`tauri-driver`](https://crates.io/crates/tauri-driver) 或通过运行以下命令更新现有安装：

终端窗口

    cargo install tauri-driver --locked

由于我们目前使用平台原生的 [WebDriver](https://www.w3.org/TR/webdriver/) 服务器，因此在支持的平台上运行 [`tauri-driver`](https://crates.io/crates/tauri-driver) 有一些要求。

### Linux

名为“Linux”的章节

我们在 Linux 平台上使用 `WebKitWebDriver`。请通过运行 `which WebKitWebDriver` 命令检查该二进制文件是否已存在，因为某些发行版将其与标准的 WebKit 包捆绑在一起。其他平台可能有单独的包，例如基于 Debian 的发行版中的 `webkit2gtk-driver`。

### Windows

标题为“Windows”的章节

请确保获取与您 Windows Edge 版本匹配的 [Microsoft Edge Driver](https://developer.microsoft.com/en-us/microsoft-edge/tools/webdriver/) 版本，该应用程序就是在该版本下构建和测试的。在更新的 Windows 安装上，这几乎总是最新的稳定版本。如果两个版本不匹配，您的 WebDriver 测试套件在尝试连接时可能会挂起。

您可以使用 [msedgedriver-tool](https://github.com/chippers/msedgedriver-tool) 下载合适的 Microsoft Edge Driver。

终端窗口

    cargo install --git https://github.com/chippers/msedgedriver-tool

    & "$HOME/.cargo/bin/msedgedriver-tool.exe"

下载包中包含一个名为 `msedgedriver.exe` 的二进制文件。[`tauri-driver`](https://crates.io/crates/tauri-driver) 会在 `$PATH` 中寻找该文件，因此请确保它已添加到路径中，或者在 [`tauri-driver`](https://crates.io/crates/tauri-driver) 上使用 `--native-driver` 选项。您可能希望将其作为 CI 设置流程的一部分自动下载，以确保 Windows CI 机器上的 Edge 和 Edge Driver 版本保持同步。关于如何操作的指南可能会在以后添加。

## 示例应用

标题为“示例应用”的章节

以下是分步指南，展示如何创建一个使用 WebDriver 测试的最小示例应用程序。

如果您希望查看该指南的结果并查阅已完成的最小代码库，可以访问 <https://github.com/tauri-apps/webdriver-example>。

[ Selenium ](/develop/tests/webdriver/example/selenium/)

[ WebdriverIO ](/develop/tests/webdriver/example/webdriverio/)

## 持续集成 (CI)

标题为“持续集成 (CI)”的章节

上述示例还附带了一个使用 GitHub Actions 进行测试的 CI 脚本，但您可能仍然会对下方的 WebDriver CI 指南感兴趣，因为它对相关概念做了进一步解释。

[ 持续集成 (CI) ](/develop/tests/webdriver/ci/)