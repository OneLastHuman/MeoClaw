# 能力

_Source: https://v2.tauri.org.cn/security/capabilities/_

Tauri 为应用程序和插件开发者提供了一套能力系统，用于细粒度地启用和限制系统 WebView 中运行的应用前端对核心功能的访问。

能力定义了针对特定窗口或 webview 授予或拒绝哪些 [权限](/security/permissions/)。

能力可以作用于多个窗口和 webview，且这些窗口/webview 可以在多个能力中被引用。

安全提示

属于多个能力的窗口和 WebView，实际上会合并所有相关能力的安全边界和权限。

能力文件定义为 `src-tauri/capabilities` 目录下的 JSON 或 TOML 文件。

最佳实践是使用独立文件，并在 `tauri.conf.json` 中仅通过标识符（identifier）进行引用，但也可以直接在 `capabilities` 字段中定义它们。

`capabilities` 目录下的所有能力默认都是自动启用的。一旦在 `tauri.conf.json` 中显式启用了某些能力，构建应用时就只会使用这些被启用的能力。

有关配置方案的完整参考，请参阅 [参考](/reference/config/) 部分。

以下 JSON 示例定义了一个能力，允许主窗口使用核心插件的默认功能以及 `window.setTitle` API。

src-tauri/capabilities/default.json

    {

      "$schema": "../gen/schemas/desktop-schema.json",

      "identifier": "main-capability",

      "description": "Capability for the main window",

      "windows": ["main"],

      "permissions": [

        "core:path:default",

        "core:event:default",

        "core:window:default",

        "core:app:default",

        "core:resources:default",

        "core:menu:default",

        "core:tray:default",

        "core:window:allow-set-title"

      ]

    }

这些代码片段是 [Tauri 配置文件](/develop/configuration-files/#tauri-config) 的一部分。

这很可能是最常见的配置方法，即在行内（inline）定义单个能力，并仅通过标识符引用权限。

这要求在 `capabilities` 目录中有定义良好的能力文件。

src-tauri/tauri.conf.json

    {

      "app": {

        "security": {

          "capabilities": ["my-capability", "main-capability"]

        }

      }

    }

行内能力可以与预定义的能力混合使用。

src-tauri/tauri.conf.json

    {

      "app": {

        "security": {

          "capabilities": [

            {

              "identifier": "my-capability",

              "description": "My application capability used for all windows",

              "windows": ["*"],

              "permissions": ["fs:default", "allow-home-read-extended"]

            },

            "my-second-capability"

          ]

        }

      }

    }

默认情况下，你在应用中注册的所有命令（使用 [`tauri::Builder::invoke_handler`](https://docs.rs/tauri/2.0.0/tauri/struct.Builder.html#method.invoke_handler) 函数）都可以被应用的所有窗口和 webview 使用。如需更改此设置，请考虑使用 [`AppManifest::commands`](https://docs.rs/tauri-build/2.0.0/tauri_build/struct.AppManifest.html#method.commands)。

src-tauri/build.rs

    fn main() {

        tauri_build::try_build(

            tauri_build::Attributes::new()

                .app_manifest(tauri_build::AppManifest::new().commands(&["your_command"])),

        )

        .unwrap();

    }

## 目标平台

名为“目标平台”的章节

能力可以通过定义 `platforms` 数组来做到特定于平台。默认情况下，该能力适用于所有目标，但你可以选择 `linux`、`macOS`、`windows`、`iOS` 和 `android` 目标的子集。

例如，一个针对桌面操作系统的能力。注意，它启用了仅在桌面端可用的插件权限。

src-tauri/capabilities/desktop.json

    {

      "$schema": "../gen/schemas/desktop-schema.json",

      "identifier": "desktop-capability",

      "windows": ["main"],

      "platforms": ["linux", "macOS", "windows"],

      "permissions": ["global-shortcut:allow-register"]

    }

这是另一个针对移动设备的能力示例。注意，它启用了仅在移动端可用的插件权限。

src-tauri/capabilities/mobile.json

    {

      "$schema": "../gen/schemas/mobile-schema.json",

      "identifier": "mobile-capability",

      "windows": ["main"],

      "platforms": ["iOS", "android"],

      "permissions": [

        "nfc:allow-scan",

        "biometric:allow-authenticate",

        "barcode-scanner:allow-scan"

      ]

    }

## 远程 API 访问

名为“远程 API 访问”的章节

默认情况下，API 仅对随 Tauri 应用打包的代码可见。要允许远程来源访问特定的 Tauri 命令，可以在能力配置文件中进行定义。

此示例允许从 `tauri.app` 的所有子域名扫描 NFC 标签并使用条形码扫描器。

src-tauri/capabilities/remote-tags.json

    {

      "$schema": "../gen/schemas/remote-schema.json",

      "identifier": "remote-tag-capability",

      "windows": ["main"],

      "remote": {

        "urls": ["https://*.tauri.app"]

      },

      "platforms": ["iOS", "android"],

      "permissions": ["nfc:allow-scan", "barcode-scanner:allow-scan"]

    }

注意

在 Linux 和 Android 上，Tauri 无法区分来自嵌入式 `<iframe>` 的请求和来自窗口本身的请求。

请务必非常谨慎地使用此功能，并阅读该功能参考部分中关于你所针对操作系统的具体安全影响。

## 安全边界

名为“安全边界”的章节

_它能防范什么？_

根据权限和能力的不同，它能够：

  * 最小化前端受损带来的影响
  * 防止或减少对本地系统接口和数据的（意外）暴露
  * 防止或减少前端到后端/系统的提权可能

_它**不能** 防范什么？_

  * 恶意或不安全的 Rust 代码
  * 过于宽松的范围和配置
  * 命令实现中不正确的范围检查
  * 来自 Rust 代码的蓄意绕过
  * 基本上任何写入应用程序 Rust 核心的代码
  * 系统 WebView 中的 0-day 或未修补的 1-day 漏洞
  * 供应链攻击或其他已受损的开发者系统

安全提示

安全边界依赖于窗口标签（**而非标题** ）。我们建议仅对高权限窗口暴露窗口创建功能。

## Schema 文件

名为“Schema 文件”的章节

Tauri 通过 `tauri-build` 为你的应用程序生成包含所有可用权限的 JSON schema，从而允许在 IDE 中进行自动补全。要使用 schema，请在配置文件（.json 或 .toml）中将 `$schema` 属性设置为 `gen/schemas` 目录下的某个平台特定 schema。通常你会将其设置为 `../gen/schemas/desktop-schema.json` 或 `../gen/schemas/mobile-schema.json`，不过你也可以为特定目标平台定义能力。

## 配置文件

名为“配置文件”的章节

Tauri 应用程序目录结构的简化示例

终端窗口

    tauri-app

    ├── index.html

    ├── package.json

    ├── src/

    ├── src-tauri/

    │   ├── Cargo.toml

    │   ├── capabilities/

    │   │  └── <identifier>.json/toml

    │   ├── src/

    │   ├── tauri.conf.json

所有内容都可以内联到 `tauri.conf.json` 中，但即使是稍微复杂一点的配置也会使该文件变得臃肿，而这种方法的目标是尽可能抽象权限，并使其易于理解。

## 核心权限

名为“核心权限”的章节

所有核心权限的列表可以在 [核心权限](/reference/acl/core-permissions/) 页面找到。