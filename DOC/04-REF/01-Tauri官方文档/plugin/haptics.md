# 触觉反馈 (Haptics)

_Source: https://v2.tauri.org.cn/plugin/haptics/_

[ GitHub](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/haptics) [ npm ](https://npmjs.net.cn/package/@tauri-apps/plugin-haptics) [ crates.io ](https://crates.io/crates/tauri-plugin-haptics)

API 参考 [ ](/reference/javascript/haptics/) [ ](https://docs.rs/tauri-plugin-haptics)

Android 和 iOS 上的触觉反馈和振动。

Android 对振动支持没有统一的标准或要求，因此触觉反馈 API 在一些经济型手机（包括近期发布的产品）上可能无法正常工作。

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

安装触觉插件以开始使用。

  * 自动
  * 手动

使用你的项目包管理器添加依赖项

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri add haptics

    yarn run tauri add haptics

    pnpm tauri add haptics

    deno task tauri add haptics

    bun tauri add haptics

    cargo tauri add haptics

  1. 在 `src-tauri` 文件夹中运行以下命令，将插件添加到项目的 `Cargo.toml` 依赖项中

         cargo add tauri-plugin-haptics --target 'cfg(any(target_os = "android", target_os = "ios"))'

  2. 修改 `lib.rs` 以初始化插件

src-tauri/src/lib.rs

         #[cfg_attr(mobile, tauri::mobile_entry_point)]

         pub fn run() {

             tauri::Builder::default()

                 .setup(|app| {

                     #[cfg(mobile)]

                     app.handle().plugin(tauri_plugin_haptics::init());

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

    npm install @tauri-apps/plugin-haptics

    yarn add @tauri-apps/plugin-haptics

    pnpm add @tauri-apps/plugin-haptics

    deno add npm:@tauri-apps/plugin-haptics

    bun add @tauri-apps/plugin-haptics

## 用法

名为“用法”的章节

触觉插件可在 JavaScript 中使用。

    import {

      vibrate,

      impactFeedback,

      notificationFeedback,

      selectionFeedback,

    } from '@tauri-apps/plugin-haptics';

    await vibrate(1);

    await impactFeedback('medium');

    await notificationFeedback('warning');

    await selectionFeedback();

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

      "permissions": [

        "haptics:allow-impact-feedback",

        "haptics:allow-notification-feedback",

        "haptics:allow-selection-feedback",

        "haptics:allow-vibrate"

      ]

    }

## 权限表

标识符 | 描述
---|---
`haptics:allow-impact-feedback` |  在没有任何预配置范围的情况下启用 impact_feedback 命令。
`haptics:deny-impact-feedback` |  在没有任何预配置范围的情况下禁用 impact_feedback 命令。
`haptics:allow-notification-feedback` |  在没有任何预配置范围的情况下启用 notification_feedback 命令。
`haptics:deny-notification-feedback` |  在没有任何预配置范围的情况下禁用 notification_feedback 命令。
`haptics:allow-selection-feedback` |  在没有任何预配置范围的情况下启用 selection_feedback 命令。
`haptics:deny-selection-feedback` |  在没有任何预配置范围的情况下禁用 selection_feedback 命令。
`haptics:allow-vibrate` |  在没有任何预配置范围的情况下启用 vibrate 命令。
`haptics:deny-vibrate` |  在没有任何预配置范围的情况下禁用 vibrate 命令。