# 先决条件

_Source: https://v2.tauri.org.cn/start/prerequisites/_

为了开始使用 Tauri 构建项目，首先需要安装一些依赖项

  1. 系统依赖
  2. Rust
  3. 配置移动端目标平台（仅在开发移动端应用时需要）

## 系统依赖

标题为“系统依赖”的章节

点击链接，针对您的操作系统开始设置

  * Linux（查看下文了解具体发行版）
  * macOS Catalina (10.15) 及更高版本
  * Windows 7 及更高版本

### Linux

名为“Linux”的章节

Tauri 在 Linux 上开发需要各种系统依赖项。这些依赖项可能因发行版而异，但我们在下面列出了一些主流发行版以帮助您进行设置。

  * Debian
  * Arch
  * Fedora
  * Gentoo
  * OSTree
  * openSUSE
  * Alpine
  * NixOS

终端窗口

    sudo apt update

    sudo apt install libwebkit2gtk-4.1-dev \

      build-essential \

      curl \

      wget \

      file \

      libxdo-dev \

      libssl-dev \

      libayatana-appindicator3-dev \

      librsvg2-dev

终端窗口

    sudo pacman -Syu

    sudo pacman -S --needed \

      webkit2gtk-4.1 \

      base-devel \

      curl \

      wget \

      file \

      openssl \

      appmenu-gtk-module \

      libappindicator-gtk3 \

      librsvg \

      xdotool

终端窗口

    sudo dnf check-update

    sudo dnf install webkit2gtk4.1-devel \

      openssl-devel \

      curl \

      wget \

      file \

      libappindicator-gtk3-devel \

      librsvg2-devel \

      libxdo-devel

    sudo dnf group install "c-development"

终端窗口

    sudo emerge --ask \

      net-libs/webkit-gtk:4.1 \

      dev-libs/libayatana-appindicator \

      net-misc/curl \

      net-misc/wget \

      sys-apps/file

终端窗口

    sudo rpm-ostree install webkit2gtk4.1-devel \

      openssl-devel \

      curl \

      wget \

      file \

      libappindicator-gtk3-devel \

      librsvg2-devel \

      libxdo-devel \

      gcc \

      gcc-c++ \

      make

    sudo systemctl reboot

终端窗口

    sudo zypper up

    sudo zypper in webkit2gtk3-devel \

      libopenssl-devel \

      curl \

      wget \

      file \

      libappindicator3-1 \

      librsvg-devel

    sudo zypper in -t pattern devel_basis

终端窗口

    sudo apk add \

      build-base \

      webkit2gtk-4.1-dev \

      curl \

      wget \

      file \

      openssl \

      libayatana-appindicator-dev \

      librsvg

> 注意：Alpine Linux 容器默认不包含任何字体。为确保文本在您的 Tauri 应用中正确渲染，请至少安装一个字体包（例如 `font-dejavu `）。

注意

有关 Nix/NixOS 的说明可以在 [NixOS Wiki](https://wiki.nixos.cn/wiki/Tauri) 中找到。

如果您的发行版未包含在上方，您可以查看 GitHub 上的 [Awesome Tauri](https://github.com/tauri-apps/awesome-tauri#guides)，看看是否已经创建了相关指南。

下一步：安装 Rust

### macOS

名为“macOS”的章节

Tauri 使用 [Xcode](https://developer.apple.com/xcode/resources/) 以及各种 macOS 和 iOS 开发依赖项。

从以下任一位置下载并安装 Xcode

  * [Mac App Store](https://apps.apple.com/gb/app/xcode/id497799835?mt=12)
  * [Apple 开发者网站](https://developer.apple.com/xcode/resources/).

安装完成后请务必启动 Xcode，以便其完成相关设置。

仅开发桌面端目标？如果您仅计划开发桌面端应用，而不针对 iOS，则可以安装 Xcode 命令行工具 (Command Line Tools) 代替。

终端窗口

    xcode-select --install

下一步：安装 Rust

### Windows

名为“窗口”的章节

Tauri 使用 Microsoft C++ 构建工具进行开发，同时还需要 Microsoft Edge WebView2。这两者在 Windows 上开发都是必需的。

请按照以下步骤安装所需的依赖项。

#### Microsoft C++ 构建工具

标题为“Microsoft C++ 构建工具”的章节

  1. 下载 [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) 安装程序并打开以开始安装。
  2. 安装期间，请勾选“使用 C++ 的桌面开发 (Desktop development with C++)”选项。

![Visual Studio C++ Build Tools installer screenshot](/_astro/visual-studio-build-tools-installer.BWhlyd8N_J78Jx.webp)

下一步：安装 WebView2。

#### WebView2

标题为“WebView2”的章节

提示

WebView2 已预装在 Windows 10（1803 及更高版本）以及 Windows 的后续版本中。如果您在这些版本上进行开发，可以直接跳过此步，前往安装 Rust。

Tauri 使用 Microsoft Edge WebView2 在 Windows 上渲染内容。

通过访问 [WebView2 运行时下载页面](https://developer.microsoft.com/en-us/microsoft-edge/webview2/#download-section)来安装 WebView2。下载“Evergreen Bootstrapper”并运行安装。

下一步：检查 VBSCRIPT

#### VBSCRIPT（适用于 MSI 安装程序）

标题为“VBSCRIPT（适用于 MSI 安装程序）”的章节

仅限 MSI 包构建

这仅在您计划构建 MSI 安装包（即在 `tauri.conf.json` 中设置 `"targets": "msi"` 或 `"targets": "all"`）时才需要。

在 Windows 上构建 MSI 包需要启用 VBSCRIPT 可选功能。该功能在大多数 Windows 系统中默认启用，但在某些系统上可能被禁用了。

如果您在构建 MSI 包时遇到如 `failed to run light.exe` 的错误，您可能需要启用 VBSCRIPT 功能。

  1. 打开 **设置** → **应用** → **可选功能** → **更多 Windows 功能**
  2. 在列表中找到 **VBSCRIPT** 并确保已勾选
  3. 点击 **下一步** ，并在提示时重启您的计算机

**注意：** VBSCRIPT 目前在大多数 Windows 系统中默认启用，但[正在被弃用](https://techcommunity.microsoft.com/blog/windows-itpro-blog/vbscript-deprecation-timelines-and-next-steps/4148301)，可能会在未来的 Windows 版本中被禁用。

下一步：安装 Rust

## Rust

名为“Rust”的章节

Tauri 使用 [Rust](https://rust-lang.net.cn) 构建，开发时需要安装 Rust。请使用以下方法之一安装 Rust。您可以在 <https://rust-lang.net.cn/tools/install> 查看更多安装方法。

  * Linux 和 macOS
  * Windows

通过以下命令使用 [`rustup`](https://github.com/rust-lang/rustup) 安装

终端窗口

    curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh

安全提示

我们已经审核了这个 bash 脚本，它执行的功能符合预期。不过，在盲目运行 curl 脚本之前，先查看脚本内容总是明智的。

您可以查看纯脚本文件：[rustup.sh](https://sh.rustup.rs/)

访问 <https://rust-lang.net.cn/tools/install> 以安装 `rustup`。

或者，您可以使用 `winget` 在 PowerShell 中执行以下命令来安装 rustup

终端窗口

    winget install --id Rustlang.Rustup

默认使用 MSVC 工具链

为了完整支持 Tauri 及 [`trunk`](https://trunk-rs.github.io/trunk/) 等工具，请确保在安装对话框中将 MSVC Rust 工具链选为 `default host triple`。根据您的系统，它应该是 `x86_64-pc-windows-msvc`、`i686-pc-windows-msvc` 或 `aarch64-pc-windows-msvc`。

如果您已经安装了 Rust，可以通过运行以下命令确保已安装正确的工具链

终端窗口

    rustup default stable-msvc

**请务必重启您的终端（有时还需要重启系统）以使更改生效。**

下一步：如果您想为 Android 和 iOS 构建应用，请参阅配置移动端目标平台；如果您想使用 JavaScript 框架，请安装 Node。否则，请[创建项目](/start/create-project/)。

## Node.js

标题为“Node.js”的章节

JavaScript 生态系统

仅在您打算使用 JavaScript 前端框架时需要

  1. 前往 [Node.js 官网](https://node.org.cn)，下载长期支持 (LTS) 版本并安装。
  2. 通过运行以下命令检查 Node 是否成功安装

终端窗口

    node -v

    # v20.10.0

    npm -v

    # 10.2.3

重启终端以确保它识别到新的安装非常重要。某些情况下，您可能需要重启计算机。

虽然 npm 是 Node.js 的默认包管理器，但您也可以使用 pnpm 或 yarn 等其他工具。若要启用这些工具，请在终端运行 `corepack enable`。此步骤可选，仅在您倾向使用非 npm 包管理器时需要。

下一步：配置移动端目标平台或[创建项目](/start/create-project/)。

## 配置移动端目标平台

标题为“配置移动端目标平台”的章节

如果您想将应用发布到 Android 或 iOS，还需要安装一些额外的依赖项

  * Android
  * iOS

### Android

标题为“Android”的章节

  1. 从 [Android 开发者网站](https://developer.android.com.cn/studio)下载并安装 [Android Studio](https://developer.android.com.cn/studio)
  2. 设置 `JAVA_HOME` 环境变量

  * Linux
  * macOS
  * Windows

终端窗口

    export JAVA_HOME=/opt/android-studio/jbr

终端窗口

    export JAVA_HOME="/Applications/Android Studio.app/Contents/jbr/Contents/Home"

终端窗口

    [System.Environment]::SetEnvironmentVariable("JAVA_HOME", "C:\Program Files\Android\Android Studio\jbr", "User")

  3. 使用 Android Studio 中的 SDK 管理器 (SDK Manager) 安装以下内容

  * Android SDK Platform
  * Android SDK Platform-Tools
  * NDK (Side by side)
  * Android SDK Build-Tools
  * Android SDK Command-line Tools

在 SDK 管理器中勾选“Show Package Details”可以安装旧版本的包。仅在必要时安装旧版本，因为它们可能会引入兼容性问题或安全风险。

  4. 设置 `ANDROID_HOME` 和 `NDK_HOME` 环境变量。

  * Linux
  * macOS
  * Windows

终端窗口

    export ANDROID_HOME="$HOME/Android/Sdk"

    export NDK_HOME="$ANDROID_HOME/ndk/$(ls -1 $ANDROID_HOME/ndk)"

终端窗口

    export ANDROID_HOME="$HOME/Library/Android/sdk"

    export NDK_HOME="$ANDROID_HOME/ndk/$(ls -1 $ANDROID_HOME/ndk)"

终端窗口

    [System.Environment]::SetEnvironmentVariable("ANDROID_HOME", "$env:LocalAppData\Android\Sdk", "User")

    $VERSION = Get-ChildItem -Name "$env:LocalAppData\Android\Sdk\ndk" | Select-Object -Last 1

    [System.Environment]::SetEnvironmentVariable("NDK_HOME", "$env:LocalAppData\Android\Sdk\ndk\$VERSION", "User")

提示

大多数应用不会自动刷新环境变量，因此为了让它们生效，您可以重启终端和 IDE，或者在当前的 PowerShell 会话中通过以下命令刷新

终端窗口

    [System.Environment]::GetEnvironmentVariables("User").GetEnumerator() | % { Set-Item -Path "Env:\$($_.key)" -Value $_.value }

  5. 使用 `rustup` 添加 Android 目标

终端窗口

    rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android

下一步：设置 iOS 开发环境或[创建项目](/start/create-project/)。

### iOS

名为“iOS”的章节

仅限 macOS

iOS 开发需要 Xcode，且仅在 macOS 上可用。请确保您在 macOS 系统依赖章节中安装的是 Xcode 而非 Xcode 命令行工具。

  1. 在终端中使用 `rustup` 添加 iOS 目标

终端窗口

    rustup target add aarch64-apple-ios x86_64-apple-ios aarch64-apple-ios-sim

  2. 安装 [Homebrew](https://brew.sh.cn)

终端窗口

    /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

  3. 使用 Homebrew 安装 [Cocoapods](https://cocoapods.org.cn)

终端窗口

    brew install cocoapods

下一步：[创建项目](/start/create-project/)。

## 故障排除

名为“故障排除”的章节

如果您在安装过程中遇到任何问题，请务必查看[故障排除指南](/develop/debug/)或前往 [Tauri Discord](https://discord.com/invite/tauri) 寻求帮助。

下一步

现在您已经安装了所有先决条件，可以准备[创建您的第一个 Tauri 项目](/start/create-project/)了！