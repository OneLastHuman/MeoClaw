# 测试

_Source: https://v2.tauri.org.cn/develop/tests/_

Tauri 支持使用模拟运行时（mock runtime）进行单元测试和集成测试。在模拟运行时下，原生 Webview 库不会被执行。[点击此处了解更多关于模拟运行时的信息](/develop/tests/mocking/)。

Tauri 还支持使用 WebDriver 协议进行端到端测试。桌面端和移动端均支持此功能，但 macOS 除外，因为它不提供桌面版 WebDriver 客户端。[点击此处了解更多关于 WebDriver 支持的信息](/develop/tests/webdriver/)。

我们提供了 [tauri-action](https://github.com/tauri-apps/tauri-action) 来帮助运行 GitHub Actions，但只要目标平台安装了所需的编译依赖库，任何 CI/CD 运行环境都可以与 Tauri 一起使用。