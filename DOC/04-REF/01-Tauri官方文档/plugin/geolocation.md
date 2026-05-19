# 地理位置 (Geolocation)

_Source: https://v2.tauri.org.cn/plugin/geolocation/_

[ GitHub](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/geolocation) [ npm ](https://npmjs.net.cn/package/@tauri-apps/plugin-geolocation) [ crates.io ](https://crates.io/crates/tauri-plugin-geolocation)

API 参考 [ ](/reference/javascript/geolocation/) [ ](https://docs.rs/tauri-plugin-geolocation)

获取并追踪设备的当前位置，包括海拔、航向和速度信息（如果可用）。

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

安装地理位置插件以开始使用。

  * 自动
  * 手动

使用你的项目包管理器添加依赖项

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri add geolocation

    yarn run tauri add geolocation

    pnpm tauri add geolocation

    deno task tauri add geolocation

    bun tauri add geolocation

    cargo tauri add geolocation

  1. 在 `src-tauri` 文件夹中运行以下命令，将插件添加到项目的 `Cargo.toml` 依赖项中

         cargo add tauri-plugin-geolocation --target 'cfg(any(target_os = "android", target_os = "ios"))'

  2. 修改 `lib.rs` 以初始化插件

src-tauri/src/lib.rs

         #[cfg_attr(mobile, tauri::mobile_entry_point)]

         pub fn run() {

             tauri::Builder::default()

                 .setup(|app| {

                     #[cfg(mobile)]

                     app.handle().plugin(tauri_plugin_geolocation::init());

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

    npm install @tauri-apps/plugin-geolocation

    yarn add @tauri-apps/plugin-geolocation

    pnpm add @tauri-apps/plugin-geolocation

    deno add npm:@tauri-apps/plugin-geolocation

    bun add @tauri-apps/plugin-geolocation

## 配置

名为“配置”的章节

### iOS

名为“iOS”的章节

Apple 要求在 Info.plist 中为位置信息指定隐私描述，您应在此处说明应用为何需要访问该信息。下文展示了一个示例描述。

    <?xml version="1.0" encoding="UTF-8"?>

    <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">

    <plist version="1.0">

        <dict>

            <key>NSLocationWhenInUseUsageDescription</key>

            <string>Required to do XY</string>

        </dict>

    </plist>

### Android

标题为“Android”的章节

此插件会自动将以下权限添加到您的 `AndroidManifest.xml` 文件中：

    <uses-permission android:name="android.permission.ACCESS_COARSE_LOCATION" />

    <uses-permission android:name="android.permission.ACCESS_FINE_LOCATION" />

如果您的应用需要 GPS 功能才能运行，则应将以下内容添加到您的 `AndroidManifest.xml` 文件中：

    <uses-feature android:name="android.hardware.location.gps" android:required="true" />

Google Play 商店使用此属性来决定是否应向没有 GPS 功能的设备展示该应用。

## 用法

名为“用法”的章节

地理位置插件可在 JavaScript 中使用。

    import {

      checkPermissions,

      requestPermissions,

      getCurrentPosition,

      watchPosition,

    } from '@tauri-apps/plugin-geolocation';

    let permissions = await checkPermissions();

    if (

      permissions.location === 'prompt' ||

      permissions.location === 'prompt-with-rationale'

    ) {

      permissions = await requestPermissions(['location']);

    }

    if (permissions.location === 'granted') {

      const pos = await getCurrentPosition();

      await watchPosition(

        { enableHighAccuracy: true, timeout: 10000, maximumAge: 0 },

        (pos) => {

          console.log(pos);

        }

      );

    }

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

        "core:default",

        "geolocation:allow-check-permissions",

        "geolocation:allow-request-permissions",

        "geolocation:allow-get-current-position",

        "geolocation:allow-watch-position"

      ]

    }

## 权限表

标识符 | 描述
---|---
`geolocation:allow-check-permissions` |  在没有任何预配置作用域的情况下启用 check_permissions 命令。
`geolocation:deny-check-permissions` |  在没有任何预配置作用域的情况下拒绝 check_permissions 命令。
`geolocation:allow-clear-permissions` |  启用 clear_permissions 命令，且无需任何预配置的作用域。
`geolocation:deny-clear-permissions` |  禁用 clear_permissions 命令，且无需任何预配置的作用域。
`geolocation:allow-clear-watch` |  启用 clear_watch 命令，且无需任何预配置的作用域。
`geolocation:deny-clear-watch` |  禁用 clear_watch 命令，且无需任何预配置的作用域。
`geolocation:allow-get-current-position` |  启用 get_current_position 命令，且无需任何预配置的作用域。
`geolocation:deny-get-current-position` |  禁用 get_current_position 命令，且无需任何预配置的作用域。
`geolocation:allow-request-permissions` |  启用 request_permissions 命令，且无需任何预配置的作用域。
`geolocation:deny-request-permissions` |  禁用 request_permissions 命令，且无需任何预配置的作用域。
`geolocation:allow-watch-position` |  启用 watch_position 命令，且无需任何预配置的作用域。
`geolocation:deny-watch-position` |  禁用 watch_position 命令，且无需任何预配置的作用域。