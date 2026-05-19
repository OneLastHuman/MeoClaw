# AppImage

_Source: https://v2.tauri.org.cn/distribute/appimage/_

`AppImage` 是一种分发格式，它不依赖于系统中安装的软件包，而是将应用程序所需的所有依赖项和文件打包在一起。因此，输出文件体积较大，但更易于分发，因为它可以在许多 Linux 发行版上运行，且无需安装。用户只需让文件具备可执行权限（`chmod a+x MyProject.AppImage`），然后即可运行它（`./MyProject.AppImage`）。

AppImage 很方便，如果你无法为特定发行版的包管理器制作安装包，它可以简化分发过程。不过，请谨慎使用，因为文件大小会从 2-6 MB 增长到 70 MB 以上。

注意

macOS 和 Linux 上的 GUI 应用不会从你的 shell 点文件（`.bashrc`、`.bash_profile`、`.zshrc` 等）继承 `$PATH`。查看 Tauri 的 [fix-path-env-rs](https://github.com/tauri-apps/fix-path-env-rs) crate 来解决此问题。

## 局限性

标题为“限制”的章节

像 glibc 这样的核心库经常会破坏与旧系统的兼容性。因此，你必须使用你打算支持的最旧的基础系统来构建你的 Tauri 应用程序。较旧的系统（例如 Ubuntu 18.04）比 Ubuntu 22.04 更合适，因为在 Ubuntu 22.04 上编译的二进制文件对 glibc 版本的要求更高，因此在较旧的系统上运行时，你会遇到类似 `/usr/lib/libc.so.6: version 'GLIBC_2.33' not found` 的运行时错误。我们建议使用 Docker 容器或 GitHub Actions 来为 Linux 构建你的 Tauri 应用程序。

有关更多信息，请参阅 issues [tauri-apps/tauri#1355](https://github.com/tauri-apps/tauri/issues/1355) 和 [rust-lang/rust#57497](https://github.com/rust-lang/rust/issues/57497)，以及 [AppImage 指南](https://docs.appimage.org/reference/best-practices.html#binaries-compiled-on-old-enough-base-system)。

## 通过 GStreamer 提供多媒体支持

章节标题 “通过 GStreamer 提供多媒体支持”

如果你的应用程序需要播放音频/视频，你需要启用 `tauri.conf.json > bundle > linux > appimage > bundleMediaFramework`。这会增加 AppImage 包的大小，以包含媒体播放所需的附加 GStreamer 文件。此标志目前仅在 Ubuntu 构建系统上得到完全支持。请确保你的构建系统拥有应用程序在运行时可能需要的所有插件。

注意

GStreamer `ugly` 包中的插件，其许可协议可能使你难以将其作为应用程序的一部分进行分发。

## 自定义文件

章节标题 “自定义文件”

若要向 AppImage 中添加自定义文件，且不想通过 Tauri 的 [`resources` 功能](/develop/resources/)来添加，你可以在 `tauri.conf.json > bundle > linux > appimage > files` 中提供文件或文件夹列表。该配置对象将 AppImage 中的路径映射到你文件系统中的文件路径（相对于 `tauri.conf.json` 文件）。以下是一个配置示例：

tauri.conf.json

    {

      "bundle": {

        "linux": {

          "appimage": {

            "files": {

              "/usr/share/README.md": "../README.md", // copies the ../README.md file to <appimage>/usr/share/README.md

              "/usr/assets": "../assets/" // copies the entire ../assets directory to <appimage>/usr/assets

            }

          }

        }

      }

    }

注意

请注意，目标路径目前必须以 `/usr/` 开头。

## 适用于 ARM 设备的 AppImage

章节标题 “适用于 ARM 设备的 AppImage”

2025 年 8 月更新

GitHub 已[发布](https://github.blog/changelog/2025-08-07-arm64-hosted-runners-for-public-repositories-are-now-generally-available/#get-started)了可公开使用的 `ubuntu-22.04-arm` 和 `ubuntu-24.04-arm` 运行器。你可以使用这些运行器来构建你的应用程序，无需进行任何更改，典型的构建过程大约需要 10 分钟。

Tauri 所使用的 AppImage 工具 `linuxdeploy` 目前[不支持交叉编译](https://github.com/linuxdeploy/linuxdeploy/issues/258) ARM AppImage。这意味着 ARM AppImage 只能在 ARM 设备或模拟器上构建。

查看我们的 [GitHub Action 指南](/distribute/pipelines/github/#arm-runner-compilation)以获取利用 QEMU 构建应用程序的工作流示例。请注意，这种方法极其缓慢，仅建议用于公共仓库（因为公共仓库的构建时长是免费的）。在私有仓库中，GitHub 的 ARM 运行器应更具成本效益且更容易设置。