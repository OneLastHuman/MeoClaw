# Microsoft Store

_Source: https://v2.tauri.org.cn/distribute/microsoft-store/_

Microsoft Store 是由微软运营的 Windows 应用商店。

本指南仅涵盖直接向 Microsoft Store 分发 Windows 应用的详细信息。有关 Windows 安装程序分发选项和配置的更多信息，请参阅[ Windows 安装程序指南](/distribute/windows-installer/)。

## 要求

名为“要求”的章节

要在 Microsoft Store 上发布应用，您必须拥有 Microsoft 账户并以个人或公司身份[注册](https://learn.microsoft.com/en-us/windows/apps/get-started/sign-up)为开发者。

## 更改应用图标

名为“更改应用图标”的章节

Tauri CLI 可以生成您的应用所需的所有图标，包括 Microsoft Store 图标。使用 `tauri icon` 命令，可以从单个 PNG 或 SVG 源文件生成应用图标。

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

使用 Microsoft 账户注册为开发者后，您需要在[应用和游戏 (Apps and Games)](https://partner.microsoft.com/en-us/dashboard/apps-and-games/overview) 页面注册您的应用。点击 `新建产品 (New Product)`，选择 `EXE 或 MSI 应用`，并为您的应用预留一个唯一的名称。

## 构建与上传

名为“构建与上传”的章节

目前 Tauri 仅生成 [EXE 和 MSI](/distribute/windows-installer/) 安装程序，因此您必须创建一个仅链接到解压后应用程序的 Microsoft Store 应用。Microsoft 安装程序中链接的安装程序必须是离线的，支持[自动更新](/plugin/updater/)，并经过[代码签名](/distribute/sign/windows/)。

有关更多信息，请参阅[官方发布文档](https://learn.microsoft.com/en-us/windows/apps/publish/)。

### 离线安装程序

名为“离线安装程序”的章节

通过 Microsoft Store 分发的 Windows 安装程序必须使用 [离线安装程序](/distribute/windows-installer/#offline-installer) Webview2 安装选项。

要仅在为 Microsoft Store 打包时应用此安装程序配置，您可以定义一个单独的 Tauri 配置文件：

"src-tauri/tauri.microsoftstore.conf.json"

    {

      "bundle": {

        "windows": {

          "webviewInstallMode": {

            "type": "offlineInstaller"

          }

        }

      }

    }

然后在为 Microsoft Store 打包 Tauri 应用时，将该配置文件与主配置文件合并。

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri build -- --no-bundle

    npm run tauri bundle -- --config src-tauri/tauri.microsoftstore.conf.json

    yarn tauri build --no-bundle

    yarn tauri bundle --config src-tauri/tauri.microsoftstore.conf.json

    pnpm tauri build --no-bundle

    pnpm tauri bundle --config src-tauri/tauri.microsoftstore.conf.json

    deno task tauri build --no-bundle

    deno task tauri bundle --config src-tauri/tauri.microsoftstore.conf.json

    bun tauri build --no-bundle

    bun tauri bundle --config src-tauri/tauri.microsoftstore.conf.json

    cargo tauri build --no-bundle

    cargo tauri bundle --config src-tauri/tauri.microsoftstore.conf.json

这在设置 CI/CD 以将应用上传到 Microsoft Store 时特别有用，因为您可以为在应用商店之外分发的 Windows 安装程序保留单独的配置。

### 发布者

名为“发布者”的章节

您的应用[发布者 (publisher)](/reference/config/#publisher) 名称不能与应用产品名称相同。

如果未设置发布者配置值，Tauri 会从您的 Bundle Identifier 的第二部分推导得出。由于发布者名称不能与产品名称相同，以下配置是无效的：

tauri.conf.json

    {

      "productName": "Example",

      "identifier": "com.example.app"

    }

在这种情况下，您可以单独定义 [发布者](/reference/config/#publisher) 值来解决此冲突。

tauri.conf.json

    {

      "productName": "Example",

      "identifier": "com.example.app",

      "bundle": {

        "publisher": "Example Inc."

      }

    }

### 上传

名为“上传”的章节

为 Microsoft Store 构建 Windows 安装程序后，您可以将其上传到您选择的分发服务中，并在 Microsoft Store 网站上的应用页面中进行链接。