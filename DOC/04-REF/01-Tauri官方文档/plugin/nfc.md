# NFC

_Source: https://v2.tauri.org.cn/plugin/nfc/_

[ GitHub](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/nfc) [ npm ](https://npmjs.net.cn/package/@tauri-apps/plugin-nfc) [ crates.io ](https://crates.io/crates/tauri-plugin-nfc)

API 参考 [ ](/reference/javascript/nfc/) [ ](https://docs.rs/tauri-plugin-nfc)

在 Android 和 iOS 上读写 NFC 标签。

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

安装 nfc 插件以开始使用。

  * 自动
  * 手动

使用你的项目包管理器添加依赖项

  * npm
  * yarn
  * pnpm
  * bun
  * cargo

    npm run tauri add nfc

    yarn run tauri add nfc

    pnpm tauri add nfc

    bun tauri add nfc

    cargo tauri add nfc

  1. 在 `src-tauri` 文件夹中运行以下命令，将插件添加到项目的 `Cargo.toml` 依赖项中

         cargo add tauri-plugin-nfc --target 'cfg(any(target_os = "android", target_os = "ios"))'

  2. 修改 `lib.rs` 以初始化插件

src-tauri/src/lib.rs

         #[cfg_attr(mobile, tauri::mobile_entry_point)]

         pub fn run() {

             tauri::Builder::default()

                 .setup(|app| {

                     #[cfg(mobile)]

                     app.handle().plugin(tauri_plugin_nfc::init());

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

    npm install @tauri-apps/plugin-nfc

    yarn add @tauri-apps/plugin-nfc

    pnpm add @tauri-apps/plugin-nfc

    deno add npm:@tauri-apps/plugin-nfc

    bun add @tauri-apps/plugin-nfc

## 配置

名为“配置”的章节

NFC 插件在 iOS 上需要原生配置。

### iOS

名为“iOS”的章节

若要在 iOS 上访问 NFC API，您必须调整目标 iOS 版本，在 Info.plist 文件中配置使用描述，并为您的应用添加 NFC 功能。

#### 目标 iOS 版本

标题为“目标 iOS 版本”的章节

NFC 插件需要 iOS 14+。这是使用 Tauri CLI v2.8 及以上版本创建的 Tauri 应用的默认设置，但您可以编辑 Xcode 项目进行配置。

在 `src-tauri/gen/apple/<project-name>.xcodeproj/project.pbxproj` 文件中，将所有 `IPHONEOS_DEPLOYMENT_TARGET` 属性设置为 `14.0`

    /* Begin XCBuildConfiguration section */

        <random-id> /* release */ = {

          isa = XCBuildConfiguration;

          buildSettings = {

            ...

            IPHONEOS_DEPLOYMENT_TARGET = 14.0;

          };

          name = release;

        };

        <random-id> /* debug */ = {

          isa = XCBuildConfiguration;

          buildSettings = {

            ...

            IPHONEOS_DEPLOYMENT_TARGET = 14.0;

          };

          name = debug;

        };

或者，您可以在 Xcode 的 `General > Minimum Deployments > iOS` 配置中设置部署目标。

#### Info.plist

标题为“Info.plist”的章节

在 iOS 上，NFC 插件需要 `NFCReaderUsageDescription` 属性列表值，该值应描述您的应用为何需要扫描或写入 NFC 标签。

在 `src-tauri/Info.ios.plist` 文件中，添加以下代码片段

src-tauri/Info.ios.plist

    <?xml version="1.0" encoding="UTF-8"?>

    <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">

    <plist version="1.0">

      <dict>

        <key>NFCReaderUsageDescription</key>

        <string>Read and write various NFC tags</string>

      </dict>

    </plist>

#### NFC 功能 (Capability)

标题为“NFC 功能”的章节

此外，iOS 要求将 NFC 功能关联到您的应用程序。

该功能可以在 Xcode 项目配置的“Signing & Capabilities”选项卡中通过点击“+ Capability”按钮并选择“Near Field Communication Tag Reading”功能来添加（更多信息请参见[为目标添加功能](https://help.apple.com/xcode/mac/current/#/dev88ff319e7)），或者通过向 `gen/apple/<app-name>_iOS/<app-name>_iOS.entitlements` 文件添加以下配置来完成。

gen/apple/<app-name>_iOS/<app-name>_iOS.entitlements

    <?xml version="1.0" encoding="UTF-8"?>

    <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">

    <plist version="1.0">

    <dict>

      <key>com.apple.developer.nfc.readersession.formats</key>

      <array>

        <string>TAG</string>

      </array>

    </dict>

    </plist>

## 用法

名为“用法”的章节

NFC 插件同时提供 JavaScript 和 Rust API，允许您扫描和写入 NFC 标签。

### 检查 NFC 是否受支持

标题为“检查 NFC 是否受支持”的章节

并非所有移动设备都具备扫描 NFC 标签的能力，因此在调用扫描和写入 API 之前，您应先检查可用性。

  * JavaScript
  * Rust

    import { isAvailable } from '@tauri-apps/plugin-nfc';

    const canScanNfc = await isAvailable();

    tauri::Builder::default()

      .setup(|app| {

        #[cfg(mobile)]

        {

          use tauri_plugin_nfc::NfcExt;

          app.handle().plugin(tauri_plugin_nfc::init());

          let can_scan_nfc = app.nfc().is_available()?;

        }

        Ok(())

      })

### 扫描 NFC 标签

标题为“扫描 NFC 标签”的章节

该插件既可以扫描普通 NFC 标签，也可以扫描带有 NDEF (NFC 数据交换格式) 消息的 NFC 标签，NDEF 是在 NFC 标签中封装类型化数据的标准格式。

  * JavaScript
  * Rust

    import { scan } from '@tauri-apps/plugin-nfc';

    const scanType = {

      type: 'ndef', // or 'tag',

    };

    const options = {

      keepSessionAlive: false,

      // configure the messages displayed in the "Scan NFC" dialog on iOS

      message: 'Scan a NFC tag',

      successMessage: 'NFC tag successfully scanned',

    };

    const tag = await scan(scanType, options);

    tauri::Builder::default()

      .setup(|app| {

        #[cfg(mobile)]

        {

          use tauri_plugin_nfc::NfcExt;

          app.handle().plugin(tauri_plugin_nfc::init());

          let tag = app

            .nfc()

            .scan(tauri_plugin_nfc::ScanRequest {

                kind: tauri_plugin_nfc::ScanKind::Ndef {

                    mime_type: None,

                    uri: None,

                    tech_list: None,

                },

                keep_session_alive: false,

            })?

            .tag;

        }

        Ok(())

      })

注意

`keepSessionAlive` 选项可用于随后直接写入已扫描的 NFC 标签。

如果您不提供该选项，会话将在下一次 `write()` 调用时重新创建，这意味着应用程序将尝试重新扫描标签。

#### 过滤器

标题为“过滤器”的章节

NFC 扫描仪还可以通过 URI 格式、MIME 类型或 NFC 标签技术来过滤标签。在这种情况下，扫描将仅检测匹配所提供过滤器的标签。

注意

过滤功能仅在 Android 上可用，因此您应始终检查扫描到的 NFC 标签内容。

MIME 类型区分大小写，且必须以小写字母形式提供。

  * JavaScript
  * Rust

    import { scan, TechKind } from '@tauri-apps/plugin-nfc';

    const techLists = [

      // capture anything using NfcF

      [TechKind.NfcF],

      // capture all MIFARE Classics with NDEF payloads

      [TechKind.NfcA, TechKind.MifareClassic, TechKind.Ndef],

    ];

    const tag = await scan({

      type: 'ndef', // or 'tag'

      mimeType: 'text/plain',

      uri: {

        scheme: 'https',

        host: 'my.domain.com',

        pathPrefix: '/app',

      },

      techLists,

    });

    tauri::Builder::default()

      .setup(|app| {

        #[cfg(mobile)]

        {

          use tauri_plugin_nfc::NfcExt;

          app.handle().plugin(tauri_plugin_nfc::init());

          let tag = app

            .nfc()

            .scan(tauri_plugin_nfc::ScanRequest {

                kind: tauri_plugin_nfc::ScanKind::Ndef {

                    mime_type: Some("text/plain".to_string()),

                    uri: Some(tauri_plugin_nfc::UriFilter {

                      scheme: Some("https".to_string()),

                      host: Some("my.domain.com".to_string()),

                      path_prefix: Some("/app".to_string()),

                    }),

                    tech_list: Some(vec![

                      vec![tauri_plugin_nfc::TechKind::Ndef],

                    ]),

                },

            })?

            .tag;

        }

        Ok(())

      })

### 写入 NFC 标签

标题为“写入 NFC 标签”的章节

`write` API 可用于向 NFC 标签写入有效载荷 (payload)。如果没有使用 `keepSessionAlive: true` 扫描到的标签，应用程序将首先扫描一个 NFC 标签。

  * JavaScript
  * Rust

    import { write, textRecord, uriRecord } from '@tauri-apps/plugin-nfc';

    const payload = [uriRecord('https://tauri.org.cn'), textRecord('some payload')];

    const options = {

      // the kind is only required if you do not have a scanned tag session alive

      // its format is the same as the argument provided to scan()

      kind: {

        type: 'ndef',

      },

      // configure the messages displayed in the "Scan NFC" dialog on iOS

      message: 'Scan a NFC tag',

      successfulReadMessage: 'NFC tag successfully scanned',

      successMessage: 'NFC tag successfully written',

    };

    await write(payload, options);

注意

目前 Rust API 仅提供用于写入 NFC 有效载荷的低级接口。

该 API 很快将会得到增强。

    tauri::Builder::default()

      .setup(|app| {

        #[cfg(mobile)]

        {

          use tauri_plugin_nfc::NfcExt;

          app.handle().plugin(tauri_plugin_nfc::init());

          app

            .nfc()

            .write(vec![

              tauri_plugin_nfc::NfcRecord {

                format: tauri_plugin_nfc::NFCTypeNameFormat::NfcWellKnown,

                kind: vec![0x55], // URI record

                id: vec![],

                payload: vec![], // insert payload here

              }

            ])?;

        }

        Ok(())

      })

## Permissions

名为“权限”的章节

默认情况下，所有潜在危险的插件命令和范围都被阻止，无法访问。您必须修改 `capabilities` 配置中的权限才能启用这些功能。

有关更多信息，请参阅[功能概述](/security/capabilities/)，并参阅[分步指南](/learn/security/using-plugin-permissions/)以使用插件权限。

src-tauri/capabilities/default.json

    {

      "permissions": [

        ...,

        "nfc:default",

      ]

    }

## 默认权限

此权限集配置 NFC 插件可执行哪些类型的操作。

#### 已授予权限

允许检查 NFC 功能是否可用以及扫描附近标签。写入标签的功能需要手动启用。

#### 此默认权限集包括以下内容

  * `allow-is-available`
  * `allow-scan`

## 权限表

标识符 | 描述
---|---
`nfc:allow-is-available` |  启用 is_available 命令，无需任何预先配置的作用域。
`nfc:deny-is-available` |  禁用 is_available 命令，无需任何预先配置的作用域。
`nfc:allow-scan` |  启用 scan 命令，无需任何预先配置的作用域。
`nfc:deny-scan` |  禁用 scan 命令，无需任何预先配置的作用域。
`nfc:allow-write` |  启用 write 命令，无需任何预先配置的作用域。
`nfc:deny-write` |  禁用 write 命令，无需任何预先配置的作用域。