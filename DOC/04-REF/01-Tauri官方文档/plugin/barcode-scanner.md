# 条形码扫描器

_Source: https://v2.tauri.org.cn/plugin/barcode-scanner/_

[ GitHub](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/barcode-scanner) [ npm ](https://npmjs.net.cn/package/@tauri-apps/plugin-barcode-scanner) [ crates.io ](https://crates.io/crates/tauri-plugin-barcode-scanner)

API 参考 [ ](/reference/javascript/barcode-scanner/) [ ](https://docs.rs/tauri-plugin-barcode-scanner)

允许您的移动应用使用摄像头扫描二维码、EAN-13 及其他类型的条形码。

## 支持的平台

标题为“支持的平台”的章节

_此插件需要 Rust 版本至少为 **1.77.2**_

平台 | 级别 | 备注
---|---|---
windows |  |
linux |  |
macos |  |
android |  |
ios |  |

## 设置

标题为“设置”的章节

安装 barcode-scanner 插件即可开始使用。

  * 自动
  * 手动

使用你的项目包管理器添加依赖项

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri add barcode-scanner

    yarn run tauri add barcode-scanner

    pnpm tauri add barcode-scanner

    deno task tauri add barcode-scanner

    bun tauri add barcode-scanner

    cargo tauri add barcode-scanner

  1. 在 `src-tauri` 文件夹中运行以下命令，将插件添加到项目的 `Cargo.toml` 依赖项中

         cargo add tauri-plugin-barcode-scanner --target 'cfg(any(target_os = "android", target_os = "ios"))'

  2. 修改 `lib.rs` 以初始化插件

src-tauri/src/lib.rs

         #[cfg_attr(mobile, tauri::mobile_entry_point)]

         pub fn run() {

             tauri::Builder::default()

                 .setup(|app| {

                     #[cfg(mobile)]

                     app.handle().plugin(tauri_plugin_barcode_scanner::init());

                     Ok(())

                 })

                 .run(tauri::generate_context!())

                 .expect("error while running tauri application");

         }

  3. 使用您首选的 JavaScript 包管理器安装 JavaScript 访客绑定

     * npm
     * yarn
     * pnpm
     * deno
     * bun

    npm install @tauri-apps/plugin-barcode-scanner

    yarn add @tauri-apps/plugin-barcode-scanner

    pnpm add @tauri-apps/plugin-barcode-scanner

    deno add npm:@tauri-apps/plugin-barcode-scanner

    bun add @tauri-apps/plugin-barcode-scanner

## 配置

名为“配置”的章节

在 iOS 上，条形码扫描器插件需要在 `NSCameraUsageDescription` 信息属性列表（Info.plist）中进行配置，该配置应说明您的应用为何需要使用摄像头。

在 `src-tauri/Info.ios.plist` 文件中，添加以下代码片段

src-tauri/Info.ios.plist

    <?xml version="1.0" encoding="UTF-8"?>

    <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">

    <plist version="1.0">

      <dict>

        <key>NSCameraUsageDescription</key>

        <string>Read QR codes</string>

      </dict>

    </plist>

## 用法

名为“用法”的章节

条形码扫描器插件可在 JavaScript 中使用。

    import { scan, Format } from '@tauri-apps/plugin-barcode-scanner';

    // when using `"withGlobalTauri": true`, you may use

    // const { scan, Format } = window.__TAURI__.barcodeScanner;

    // `windowed: true` actually sets the webview to transparent

    // instead of opening a separate view for the camera

    // make sure your user interface is ready to show what is underneath with a transparent element

    scan({ windowed: true, formats: [Format.QRCode] });

## Permissions

名为“权限”的章节

默认情况下，所有潜在危险的插件命令和范围都被阻止，无法访问。您必须修改 `capabilities` 配置中的权限才能启用这些功能。

有关更多信息，请参阅[功能概述](/security/capabilities/)，并参阅[分步指南](/learn/security/using-plugin-permissions/)以使用插件权限。

src-tauri/capabilities/mobile.json

    {

      "$schema": "../gen/schemas/mobile-schema.json",

      "identifier": "mobile-capability",

      "windows": ["main"],

      "platforms": ["iOS", "android"],

      "permissions": ["barcode-scanner:allow-scan", "barcode-scanner:allow-cancel"]

    }

## 默认权限

此权限集用于配置默认公开哪些条形码扫描功能。

#### 已授予权限

它允许所有与条形码相关的功能。

#### 此默认权限集包括以下内容

  * `allow-cancel`
  * `allow-check-permissions`
  * `allow-open-app-settings`
  * `allow-request-permissions`
  * `allow-scan`
  * `allow-vibrate`

## 权限表

标识符 | 描述
---|---
`barcode-scanner:allow-cancel` |  在没有任何预配置作用域的情况下启用 cancel 命令。
`barcode-scanner:deny-cancel` |  在没有任何预配置作用域的情况下拒绝 cancel 命令。
`barcode-scanner:allow-check-permissions` |  在没有任何预配置作用域的情况下启用 check_permissions 命令。
`barcode-scanner:deny-check-permissions` |  在没有任何预配置作用域的情况下拒绝 check_permissions 命令。
`barcode-scanner:allow-open-app-settings` |  在没有任何预配置作用域的情况下启用 open_app_settings 命令。
`barcode-scanner:deny-open-app-settings` |  在没有任何预配置作用域的情况下拒绝 open_app_settings 命令。
`barcode-scanner:allow-request-permissions` |  在没有任何预配置作用域的情况下启用 request_permissions 命令。
`barcode-scanner:deny-request-permissions` |  在没有任何预配置作用域的情况下拒绝 request_permissions 命令。
`barcode-scanner:allow-scan` |  在没有任何预配置作用域的情况下启用 scan 命令。
`barcode-scanner:deny-scan` |  在没有任何预配置作用域的情况下拒绝 scan 命令。
`barcode-scanner:allow-vibrate` |  在没有任何预配置作用域的情况下启用 vibrate 命令。
`barcode-scanner:deny-vibrate` |  在没有任何预配置作用域的情况下拒绝 vibrate 命令。