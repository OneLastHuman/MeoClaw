# Google Play

_Source: https://v2.tauri.org.cn/distribute/google-play/_

Google Play 是由 Google 维护的 Android 应用分发服务。

本指南涵盖了在 Google Play 上发布 Android 应用的要求。

注意

Tauri 在底层使用 Android Studio 项目，因此构建和发布 Android 应用的所有官方最佳实践也同样适用于您的应用。有关更多信息，请参阅 [官方文档](https://developer.android.com.cn/distribute)。

## 要求

名为“要求”的章节

要在 Play 商店分发 Android 应用，您必须创建一个 [Play 管理中心 (Play Console)](https://play.google.com/console/developers) 开发者账号。

此外，您必须设置 [代码签名](/distribute/sign/android/)。

有关更多信息，请参阅 [发布清单 (release checklist)](https://play.google.com/console/about/guides/releasewithconfidence/)。

## 更改应用图标

名为“更改应用图标”的章节

运行 `tauri android init` 设置好 Android Studio 项目后，您可以使用 `tauri icon` 命令更新应用图标。

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri icon /path/to/app-icon.png

    yarn tauri icon /path/to/app-icon.png

    pnpm tauri icon /path/to/app-icon.png

    deno task tauri icon /path/to/app-icon.png

    bun tauri icon /path/to/app-icon.png

    cargo tauri icon /path/to/app-icon.png

## 设置

名为“设置”的章节

创建 Play 管理中心开发者账号后，您需要在 Google [Play 管理中心](https://play.google.com/console/developers) 网站上注册您的应用。该网站将引导您完成所有必要的表格填写和设置任务。

## 构建

名为“构建”的章节

您可以运行以下命令构建 Android App Bundle (AAB) 以上传至 Google Play

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri android build -- --aab

    yarn tauri android build --aab

    pnpm tauri android build --aab

    deno task tauri android build --aab

    bun tauri android build --aab

    cargo tauri android build --aab

Tauri 从 [`tauri.conf.json > version`](/reference/config/#version) 中定义的数值推导版本代码 (`versionCode = 主版本号*1000000 + 次版本号*1000 + 修订号`)。如果您需要不同的版本代码方案（例如连续的代码），可以在 [`tauri.conf.json > bundle > android > versionCode`] 配置中设置自定义版本代码。

tauri.conf.json

    {

      "bundle": {

        "android": {

          "versionCode": 100

        }

      }

    }

### 构建 APK

名为“构建 APK”的章节

AAB 格式是推荐上传至 Google Play 的包文件，但也可以生成用于测试或在商店外分发的 APK。要为您的应用编译 APK，可以使用 `--apk` 参数

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri android build -- --apk

    yarn tauri android build --apk

    pnpm tauri android build --apk

    deno task tauri android build --apk

    bun tauri android build --apk

    cargo tauri android build --apk

### 架构选择

名为“架构选择”的章节

默认情况下，Tauri 会为所有受支持的架构（aarch64、armv7、i686 和 x86_64）构建您的应用。如果仅编译目标架构的子集，可以使用 `--target` 参数

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri android build -- --aab --target aarch64 --target armv7

    yarn tauri android build --aab --target aarch64 --target armv7

    pnpm tauri android build --aab --target aarch64 --target armv7

    deno task tauri android build --aab --target aarch64 --target armv7

    bun tauri android build --aab --target aarch64 --target armv7

    cargo tauri android build --aab --target aarch64 --target armv7

### 为不同架构单独打包

名为“为不同架构单独打包”的章节

默认情况下，生成的 AAB 和 APK 是通用的，包含所有受支持的目标。若要为每个目标生成单独的包，请使用 `--split-per-abi` 参数。

注意

这仅适用于测试或在 Google Play 之外进行分发，因为它能减小文件大小，但上传起来不太方便。Google Play 会为您处理受支持的架构。

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri android build -- --apk --split-per-abi

    yarn tauri android build --apk --split-per-abi

    pnpm tauri android build --apk --split-per-abi

    deno task tauri android build --apk --split-per-abi

    bun tauri android build --apk --split-per-abi

    cargo tauri android build --apk --split-per-abi

### 更改最低支持的 Android 版本

名为“更改最低支持的 Android 版本”的章节

Tauri 应用支持的最低 Android 版本是 Android 7.0（代号 Nougat，SDK 24）。

有一些技术可以在支持旧系统的同时使用较新的 Android API。有关更多信息，请参阅 [Android 文档](https://developer.android.com.cn/training/basics/supporting-devices/platforms#version-codes)。

如果您的应用必须在较新的 Android 版本上运行，您可以配置 [`tauri.conf.json > bundle > android > minSdkVersion`]

tauri.conf.json

    {

      "bundle": {

        "android": {

          "minSdkVersion": 28

        }

      }

    }

## 上传

名为“上传”的章节

构建应用并生成 Android App Bundle 文件（位于 `gen/android/app/build/outputs/bundle/universalRelease/app-universal-release.aab`）后，您现在可以在 Google Play 管理中心创建新版本并上传它。

首次上传必须在网站上手动完成，以便它验证您的应用签名和包标识符。Tauri 目前不提供自动化创建 Android 版本流程的方法，该过程必须利用 [Google Play Developer API](https://developers.google.com/android-publisher/api-ref/rest)，但这属于正在进行中的工作。