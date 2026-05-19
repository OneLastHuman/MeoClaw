# 生物识别

_Source: https://v2.tauri.org.cn/plugin/biometric/_

[ GitHub](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/biometric) [ npm ](https://npmjs.net.cn/package/@tauri-apps/plugin-biometric) [ crates.io ](https://crates.io/crates/tauri-plugin-biometric)

API 参考 [ ](/reference/javascript/biometric/) [ ](https://docs.rs/tauri-plugin-biometric)

在 Android 和 iOS 上提示用户进行生物识别身份验证。

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

安装生物识别插件以开始使用。

  * 自动
  * 手动

使用你的项目包管理器添加依赖项

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri add biometric

    yarn run tauri add biometric

    pnpm tauri add biometric

    deno task tauri add biometric

    bun tauri add biometric

    cargo tauri add biometric

  1. 在 `src-tauri` 文件夹中运行以下命令，将插件添加到项目的 `Cargo.toml` 依赖项中

         cargo add tauri-plugin-biometric --target 'cfg(any(target_os = "android", target_os = "ios"))'

  2. 修改 `lib.rs` 以初始化插件

src-tauri/src/lib.rs

         #[cfg_attr(mobile, tauri::mobile_entry_point)]

         pub fn run() {

             tauri::Builder::default()

                 .setup(|app| {

                     #[cfg(mobile)]

                     app.handle().plugin(tauri_plugin_biometric::Builder::new().build());

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

    npm install @tauri-apps/plugin-biometric

    yarn add @tauri-apps/plugin-biometric

    pnpm add @tauri-apps/plugin-biometric

    deno add npm:@tauri-apps/plugin-biometric

    bun add @tauri-apps/plugin-biometric

## 配置

名为“配置”的章节

在 iOS 上，生物识别插件需要配置 `NSFaceIDUsageDescription` 信息属性列表键值，该值应描述您的应用为何需要使用生物识别身份验证。

在 `src-tauri/Info.ios.plist` 文件中，添加以下代码片段

src-tauri/Info.ios.plist

    <?xml version="1.0" encoding="UTF-8"?>

    <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">

    <plist version="1.0">

      <dict>

        <key>NSFaceIDUsageDescription</key>

        <string>Authenticate with biometric</string>

      </dict>

    </plist>

## 用法

名为“用法”的章节

该插件使您能够验证设备上生物识别身份验证的可用性、提示用户进行生物识别身份验证，并检查结果以确定身份验证是否成功。

### 检查状态

名为“检查状态”的章节

您可以检查生物识别身份验证的状态，包括其可用性以及支持的生物识别身份验证方法类型。

  * JavaScript
  * Rust

    import { checkStatus } from '@tauri-apps/plugin-biometric';

    const status = await checkStatus();

    if (status.isAvailable) {

      console.log('Yes! Biometric Authentication is available');

    } else {

      console.log(

        'No! Biometric Authentication is not available due to ' + status.error

      );

    }

    use tauri_plugin_biometric::BiometricExt;

    fn check_biometric(app_handle: tauri::AppHandle) {

        let status = app_handle.biometric().status().unwrap();

        if status.is_available {

            println!("Yes! Biometric Authentication is available");

        } else {

            println!("No! Biometric Authentication is not available due to: {}", status.error.unwrap());

        }

    }

### 身份验证

名为“身份验证”的章节

要提示用户进行生物识别身份验证，请使用 `authenticate()` 方法。

  * JavaScript
  * Rust

    import { authenticate } from '@tauri-apps/plugin-biometric';

    const options = {

      // Set true if you want the user to be able to authenticate using phone password

      allowDeviceCredential: false,

      cancelTitle: "Feature won't work if Canceled",

      // iOS only feature

      fallbackTitle: 'Sorry, authentication failed',

      // Android only features

      title: 'Tauri feature',

      subtitle: 'Authenticate to access the locked Tauri function',

      confirmationRequired: true,

    };

    try {

      await authenticate('This feature is locked', options);

      console.log(

        'Hooray! Successfully Authenticated! We can now perform the locked Tauri function!'

      );

    } catch (err) {

      console.log('Oh no! Authentication failed because ' + err.message);

    }

    use tauri_plugin_biometric::{BiometricExt, AuthOptions};

    fn bio_auth(app_handle: tauri::AppHandle) {

        let options = AuthOptions {

            // Set True if you want the user to be able to authenticate using phone password

            allow_device_credential:false,

            cancel_title: Some("Feature won't work if Canceled".to_string()),

            // iOS only feature

            fallback_title: Some("Sorry, authentication failed".to_string()),

            // Android only features

            title: Some("Tauri feature".to_string()),

            subtitle: Some("Authenticate to access the locked Tauri function".to_string()),

            confirmation_required: Some(true),

        };

        // if the authentication was successful, the function returns Result::Ok()

        // otherwise returns Result::Error()

        match app_handle.biometric().authenticate("This feature is locked".to_string(), options) {

            Ok(_) => {

                println!("Hooray! Successfully Authenticated! We can now perform the locked Tauri function!");

            }

            Err(e) => {

                println!("Oh no! Authentication failed because : {e}");

            }

        }

    }

## Permissions

名为“权限”的章节

默认情况下，所有潜在危险的插件命令和范围都被阻止，无法访问。您必须修改 `capabilities` 配置中的权限才能启用这些功能。

有关更多信息，请参阅[功能概述](/security/capabilities/)，并参阅[分步指南](/learn/security/using-plugin-permissions/)以使用插件权限。

src-tauri/capabilities/default.json

    {

      "$schema": "../gen/schemas/desktop-schema.json",

      "identifier": "main-capability",

      "description": "Capability for the main window",

      "windows": ["main"],

      "permissions": ["biometric:default"]

    }

## 默认权限

此权限集配置了默认公开的生物识别功能。

#### 已授予权限

它允许执行所有生物识别命令。

#### 此默认权限集包括以下内容

  * `allow-authenticate`
  * `allow-status`

## 权限表

标识符 | 描述
---|---
`biometric:allow-authenticate` |  在没有任何预配置范围的情况下启用 authenticate 命令。
`biometric:deny-authenticate` |  在没有任何预配置范围的情况下禁用 authenticate 命令。
`biometric:allow-status` |  在没有任何预配置范围的情况下启用 status 命令。
`biometric:deny-status` |  在没有任何预配置范围的情况下禁用 status 命令。