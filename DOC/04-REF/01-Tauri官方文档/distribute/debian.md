# Debian

_Source: https://v2.tauri.org.cn/distribute/debian/_

由 Tauri 打包器生成的标准 Debian 软件包包含将应用程序发布到基于 Debian 的 Linux 发行版所需的一切，包括定义应用程序的图标、生成 Desktop 文件，以及指定依赖项 `libwebkit2gtk-4.1-0` 和 `libgtk-3-0`，如果应用使用了系统托盘，还包括 `libappindicator3-1`。

注意

macOS 和 Linux 上的 GUI 应用不会从你的 shell 点文件（`.bashrc`、`.bash_profile`、`.zshrc` 等）继承 `$PATH`。查看 Tauri 的 [fix-path-env-rs](https://github.com/tauri-apps/fix-path-env-rs) crate 来解决此问题。

## 局限性

标题为“限制”的章节

像 glibc 这样的核心库经常会破坏与旧系统的兼容性。因此，你必须使用你打算支持的最旧的基础系统来构建你的 Tauri 应用程序。较旧的系统（例如 Ubuntu 18.04）比 Ubuntu 22.04 更合适，因为在 Ubuntu 22.04 上编译的二进制文件对 glibc 版本的要求更高，因此在较旧的系统上运行时，你会遇到类似 `/usr/lib/libc.so.6: version 'GLIBC_2.33' not found` 的运行时错误。我们建议使用 Docker 容器或 GitHub Actions 来为 Linux 构建你的 Tauri 应用程序。

有关更多信息，请参阅 issues [tauri-apps/tauri#1355](https://github.com/tauri-apps/tauri/issues/1355) 和 [rust-lang/rust#57497](https://github.com/rust-lang/rust/issues/57497)，以及 [AppImage 指南](https://docs.appimage.org/reference/best-practices.html#binaries-compiled-on-old-enough-base-system)。

## 自定义文件

标题为“自定义文件”的章节

如果需要更多控制，Tauri 为 Debian 软件包提供了一些配置。

如果你的应用依赖于额外的系统依赖项，你可以在 `tauri.conf.json > bundle > linux > deb` 中指定它们。

要将自定义文件包含在 Debian 软件包中，你可以在 `tauri.conf.json > bundle > linux > deb > files` 中提供文件或文件夹列表。该配置对象将 Debian 软件包中的路径映射到你文件系统上的文件路径（相对于 `tauri.conf.json` 文件）。以下是示例配置：

    {

      "bundle": {

        "linux": {

          "deb": {

            "files": {

              "/usr/share/README.md": "../README.md", // copies the README.md file to /usr/share/README.md

              "/usr/share/assets": "../assets/" // copies the entire assets directory to /usr/share/assets

            }

          }

        }

      }

    }

## 针对基于 ARM 的设备进行交叉编译

标题为“针对基于 ARM 的设备进行交叉编译”的章节

本指南涵盖手动编译。请查看我们的 [GitHub Action 指南](/distribute/pipelines/github/#arm-runner-compilation)，了解一个利用 QEMU 构建应用程序的工作流程示例。这会慢得多，但也能构建 AppImage。

当你不需要频繁编译应用程序并更喜欢一次性设置时，手动编译非常适合。以下步骤假设你使用的是基于 Debian/Ubuntu 的 Linux 发行版。

  1. #### 为你的目标架构安装 Rust 目标

标题为“为你的目标架构安装 Rust 目标”的章节

     * 对于 ARMv7（32 位）：`rustup target add armv7-unknown-linux-gnueabihf`
     * 对于 ARMv8（ARM64，64 位）：`rustup target add aarch64-unknown-linux-gnu`
  2. #### 为选定架构安装相应的链接器

标题为“为选定架构安装相应的链接器”的章节

     * 对于 ARMv7：`sudo apt install gcc-arm-linux-gnueabihf`
     * 对于 ARMv8（ARM64）：`sudo apt install gcc-aarch64-linux-gnu`
  3. #### 打开或创建文件 `<project-root>/.cargo/config.toml` 并相应地添加以下配置

标题为“打开或创建文件 <project-root>/.cargo/config.toml 并相应地添加以下配置”的章节

         [target.armv7-unknown-linux-gnueabihf]

         linker = "arm-linux-gnueabihf-gcc"

         [target.aarch64-unknown-linux-gnu]

         linker = "aarch64-linux-gnu-gcc"

  4. #### 在软件包管理器中启用相应的架构

标题为“在软件包管理器中启用相应的架构”的章节

     * 对于 ARMv7：`sudo dpkg --add-architecture armhf`
     * 对于 ARMv8（ARM64）：`sudo dpkg --add-architecture arm64`
  5. #### 调整软件包源

标题为“调整软件包源”的章节

在 Debian 上，此步骤应该是不必要的，但在其他发行版上，你可能需要编辑 /etc/apt/sources.list 以包含 ARM 架构变体。例如，在 Ubuntu 22.04 上，将这些行添加到文件底部（记得将 jammy 替换为你 Ubuntu 版本的代号）

         deb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy main restricted

         deb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy-updates main restricted

         deb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy universe

         deb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy-updates universe

         deb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy multiverse

         deb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy-updates multiverse

         deb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy-backports main restricted universe multiverse

         deb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy-security main restricted

         deb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy-security universe

         deb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy-security multiverse

然后，为了防止主软件包出现问题，你必须将正确的架构添加到文件中预先包含的所有其他行中。对于标准的 64 位系统，你需要添加 [arch=amd64]，Ubuntu 22.04 上的完整文件看起来类似于这样

显示解决方案

         # See http://help.ubuntu.com/community/UpgradeNotes for how to upgrade to

         # newer versions of the distribution.

         deb [arch=amd64] http://archive.ubuntu.com/ubuntu/ jammy main restricted

         # deb-src http://archive.ubuntu.com/ubuntu/ jammy main restricted

         ## Major bug fix updates produced after the final release of the

         ## distribution.

         deb [arch=amd64] http://archive.ubuntu.com/ubuntu/ jammy-updates main restricted

         # deb-src http://archive.ubuntu.com/ubuntu/ jammy-updates main restricted

         ## N.B. software from this repository is ENTIRELY UNSUPPORTED by the Ubuntu

         ## team. Also, please note that software in universe WILL NOT receive any

         ## review or updates from the Ubuntu security team.

         deb [arch=amd64] http://archive.ubuntu.com/ubuntu/ jammy universe

         # deb-src http://archive.ubuntu.com/ubuntu/ jammy universe

         deb [arch=amd64] http://archive.ubuntu.com/ubuntu/ jammy-updates universe

         # deb-src http://archive.ubuntu.com/ubuntu/ jammy-updates universe

         ## N.B. software from this repository is ENTIRELY UNSUPPORTED by the Ubuntu

         ## team, and may not be under a free licence. Please satisfy yourself as to

         ## your rights to use the software. Also, please note that software in

         ## multiverse WILL NOT receive any review or updates from the Ubuntu

         ## security team.

         deb [arch=amd64] http://archive.ubuntu.com/ubuntu/ jammy multiverse

         # deb-src http://archive.ubuntu.com/ubuntu/ jammy multiverse

         deb [arch=amd64] http://archive.ubuntu.com/ubuntu/ jammy-updates multiverse

         ## N.B. software from this repository may not have been tested as

         ## extensively as that contained in the main release, although it includes

         ## newer versions of some applications which may provide useful features.

         ## Also, please note that software in backports WILL NOT receive any review

         ## or updates from the Ubuntu security team.

         deb [arch=amd64] http://archive.ubuntu.com/ubuntu/ jammy-backports main restricted universe multiverse

         # deb-src http://archive.ubuntu.com/ubuntu/ jammy-backports main restricted universe multiverse

         deb [arch=amd64] http://security.ubuntu.com/ubuntu/ jammy-security main restricted

         # deb-src http://security.ubuntu.com/ubuntu/ jammy-security main restricted

         deb [arch=amd64] http://security.ubuntu.com/ubuntu/ jammy-security universe

         # deb-src http://security.ubuntu.com/ubuntu/ jammy-security universe

         deb [arch=amd64] http://security.ubuntu.com/ubuntu/ jammy-security multiverse

         # deb-src http://security.ubuntu.com/ubuntu/ jammy-security multiverse

         deb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy main restricted

         deb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy-updates main restricted

         deb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy universe

         deb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy-updates universe

         deb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy multiverse

         deb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy-updates multiverse

         deb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy-backports main restricted universe multiverse

         deb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy-security main restricted

         deb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy-security universe

         deb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy-security multiverse

  6. #### 更新软件包信息：`sudo apt-get update && sudo apt-get upgrade -y`

标题为“更新软件包信息：sudo apt-get update && sudo apt-get upgrade -y”的章节

  7. #### 为选定架构安装所需的 webkitgtk 库

标题为“为选定架构安装所需的 webkitgtk 库”的章节

     * 对于 ARMv7：`sudo apt install libwebkit2gtk-4.1-dev:armhf`
     * 对于 ARMv8（ARM64）：`sudo apt install libwebkit2gtk-4.1-dev:arm64`
  8. #### 安装 OpenSSL 或使用 vendored 版本

标题为“安装 OpenSSL 或使用 vendored 版本”的章节

这并不总是必需的，所以你可以先继续操作，看看是否遇到像 `Failed to find OpenSSL development headers` 这样的错误。

     * 要么在系统范围内安装开发头文件
       * 对于 ARMv7：`sudo apt install libssl-dev:armhf`
       * 对于 ARMv8（ARM64）：`sudo apt install libssl-dev:arm64`
     * 或者为 OpenSSL Rust crate 启用 vendor 功能，这将影响所有使用相同次要版本的其他 Rust 依赖项。你可以通过将此内容添加到 `Cargo.toml` 文件的 dependencies 部分来做到这一点

    openssl-sys = {version = "0.9", features = ["vendored"]}

  9. #### 根据你选定的架构将 `PKG_CONFIG_SYSROOT_DIR` 设置为相应的目录

标题为“根据你选定的架构将 PKG_CONFIG_SYSROOT_DIR 设置为相应的目录”的章节

     * 对于 ARMv7：`export PKG_CONFIG_SYSROOT_DIR=/usr/arm-linux-gnueabihf/`
     * 对于 ARMv8（ARM64）：`export PKG_CONFIG_SYSROOT_DIR=/usr/aarch64-linux-gnu/`
  10. #### 为你的目标 ARM 版本构建应用

标题为“为你的目标 ARM 版本构建应用”的章节

     * 对于 ARMv7：cargo tauri build —target armv7-unknown-linux-gnueabihf
     * 对于 ARMv8（ARM64）：cargo tauri build —target aarch64-unknown-linux-gnu

根据你是想为 ARMv7 还是 ARMv8 (ARM64) 交叉编译 Tauri 应用程序，选择相应的说明集。请注意，具体步骤可能会根据你的 Linux 发行版和设置而有所不同。