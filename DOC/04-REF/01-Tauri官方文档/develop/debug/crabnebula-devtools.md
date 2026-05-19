# CrabNebula DevTools

_Source: https://v2.tauri.org.cn/develop/debug/crabnebula-devtools/_

[CrabNebula](https://crabnebula.dev/) 作为与 Tauri 项目合作伙伴关系的一部分，为 Tauri 提供了一个免费的 [DevTools](https://crabnebula.dev/devtools/) 应用程序。该应用程序允许您通过捕获 Tauri 应用的嵌入式资产、Tauri 配置文件、日志和跨度（spans）来对其进行监测，并提供了一个 Web 前端，以便无缝地实时可视化数据。

通过 CrabNebula DevTools，您可以检查应用的日志事件（包括来自依赖项的日志）、跟踪命令调用的性能以及整体的 Tauri API 使用情况。它还为 Tauri 事件和命令提供了专门的界面，包括负载（payload）、响应以及内部日志和执行跨度。

要启用 CrabNebula DevTools，请安装 devtools crate

    cargo add tauri-plugin-devtools@2.0.0

并尽快在您的 main 函数中初始化该插件

    fn main() {

        // This should be called as early in the execution of the app as possible

        #[cfg(debug_assertions)] // only enable instrumentation in development builds

        let devtools = tauri_plugin_devtools::init();

        let mut builder = tauri::Builder::default();

        #[cfg(debug_assertions)]

        {

            builder = builder.plugin(devtools);

        }

        builder

            .run(tauri::generate_context!())

            .expect("error while running tauri application");

    }

然后像往常一样运行您的应用程序。如果一切设置正确，devtools 将打印以下消息

![DevTools message on terminal](/_astro/crabnebula-devtools.7F1z87oz_Z1ikk1h.webp)

注意

在这种情况下，我们仅为调试应用程序初始化 devtools 插件，这是推荐的做法。

有关更多信息，请参阅 [CrabNebula DevTools](https://docs.crabnebula.dev/devtools/get-started/) 文档。