# 深度链接

_Source: https://v2.tauri.org.cn/plugin/deep-linking/_

[ GitHub](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/deep-link) [ npm ](https://npmjs.net.cn/package/@tauri-apps/plugin-deep-link) [ crates.io ](https://crates.io/crates/tauri-plugin-deep-link)

API 参考 [ ](/reference/javascript/deep-link/) [ ](https://docs.rs/tauri-plugin-deep-link)

将你的 Tauri 应用程序设置为 URL 的默认处理程序。

## 支持的平台

标题为“支持的平台”的章节

_此插件需要 Rust 版本至少为 **1.77.2**_

平台 | 级别 | 备注
---|---|---
windows |  |
linux |  |
macos |  |  深度链接必须在配置中注册。不支持在运行时动态注册。
android |  |  深度链接必须在配置中注册。不支持在运行时动态注册。
ios |  |  深度链接必须在配置中注册。不支持在运行时动态注册。

## 设置

标题为“设置”的章节

安装 deep-link 插件以开始使用。

  * 自动
  * 手动

使用你的项目包管理器添加依赖项

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri add deep-link

    yarn run tauri add deep-link

    pnpm tauri add deep-link

    deno task tauri add deep-link

    bun tauri add deep-link

    cargo tauri add deep-link

  1. 在 `src-tauri` 文件夹中运行以下命令，将插件添加到项目的 `Cargo.toml` 依赖项中

         cargo add tauri-plugin-deep-link@2.0.0

  2. 修改 `lib.rs` 以初始化插件

src-tauri/src/lib.rs

         #[cfg_attr(mobile, tauri::mobile_entry_point)]

         pub fn run() {

             tauri::Builder::default()

                 .plugin(tauri_plugin_deep_link::init())

                 .run(tauri::generate_context!())

                 .expect("error while running tauri application");

         }

  3. 使用您首选的 JavaScript 包管理器安装 JavaScript 访客绑定

     * npm
     * yarn
     * pnpm
     * deno
     * bun

    npm install @tauri-apps/plugin-deep-link

    yarn add @tauri-apps/plugin-deep-link

    pnpm add @tauri-apps/plugin-deep-link

    deno add npm:@tauri-apps/plugin-deep-link

    bun add @tauri-apps/plugin-deep-link

## 设置

名为“设置”的章节

### Android

标题为“Android”的章节

在 Android 上，有两种方式可以通过链接打开你的应用

  1. **应用链接 (App Links, http/https + 主机名，已验证)** 对于 [应用链接](https://developer.android.com.cn/training/app-links#android-app-links)，你需要一台拥有 `.well-known/assetlinks.json` 端点的服务器，该端点必须返回指定格式的文本响应。

.well-known/assetlinks.json

    [

      {

        "relation": ["delegate_permission/common.handle_all_urls"],

        "target": {

          "namespace": "android_app",

          "package_name": "$APP_BUNDLE_ID",

          "sha256_cert_fingerprints": [

            $CERT_FINGERPRINT

          ]

        }

      }

    ]

其中 `$APP_BUNDLE_ID` 是 [`tauri.conf.json > identifier`](/reference/config/#identifier) 中定义的值（将 `-` 替换为 `_`），`$CERT_FINGERPRINT` 是你应用签名证书的 SHA256 指纹列表。详情请参阅 [验证 Android 应用链接](https://developer.android.com.cn/training/app-links/verify-android-applinks#web-assoc)。

  2. **自定义 URI 方案 (Custom URI schemes，无需主机名，无需验证)** 对于形如 `myapp://...` 的 URI，你可以声明一个自定义方案而无需托管任何文件。在移动端配置中使用 `scheme` 字段并省略 `host`。

### iOS

名为“iOS”的章节

在 iOS 上，有两种方式可以通过链接打开你的应用

  1. **通用链接 (Universal Links, https + 主机名，已验证)** 对于 [通用链接](https://developer.apple.com/documentation/xcode/allowing-apps-and-websites-to-link-to-your-content?language=objc)，你需要一台拥有 `.well-known/apple-app-site-association` 端点的服务器，该端点必须返回指定格式的 JSON 响应。

.well-known/apple-app-site-association

    {

      "applinks": {

        "details": [

          {

            "appIDs": ["$DEVELOPMENT_TEAM_ID.$APP_BUNDLE_ID"],

            "components": [

              {

                "/": "/open/*",

                "comment": "Matches any URL whose path starts with /open/"

              }

            ]

          }

        ]

      }

    }

注意

响应的 `Content-Type` 头必须是 `application/json`。

`.well-known/apple-app-site-association` 端点必须通过 HTTPS 提供服务。若要测试本地环境，你可以使用自签名 TLS 证书并将其安装到 iOS 模拟器，或使用 [ngrok](https://ngrok.com/) 等服务。

其中 `$DEVELOPMENT_TEAM_ID` 是 `tauri.conf.json > tauri > bundle > iOS > developmentTeam` 定义的值或 `TAURI_APPLE_DEVELOPMENT_TEAM` 环境变量的值，`$APP_BUNDLE_ID` 是在 [`tauri.conf.json > identifier`](/reference/config/#identifier) 中定义的值。

要验证你的域名是否已正确配置以公开应用关联，你可以运行以下命令，并将 `<host>` 替换为你的实际主机名

终端窗口

    curl -v https://app-site-association.cdn-apple.com/a/v1/<host>

有关更多信息，请参阅 [applinks.details](https://developer.apple.com/documentation/bundleresources/applinks/details-swift.dictionary)。

  2. **自定义 URI 方案 (Custom URI schemes，无主机名，无需验证)** 对于形如 `myapp://...` 的 URI，你可以在移动端配置中声明一个自定义方案，并将 `"appLink": false`（或省略）。该插件会在你的应用 `Info.plist` 中生成相应的 `CFBundleURLTypes` 条目。无需 `.well-known` 文件或 HTTPS 主机。

### Desktop

名为“Desktop”的章节

在 Linux 和 Windows 上，深度链接将作为命令行参数传递给新的应用进程。如果你希望由唯一的应用实例接收事件，深度链接插件已与 [single instance](/plugin/single-instance/) 插件集成。

  * 首先，你必须将 `deep-link` 功能添加到 single instance 插件中。

src-tauri/Cargo.toml

    [target."cfg(any(target_os = \"macos\", windows, target_os = \"linux\"))".dependencies]

    tauri-plugin-single-instance = { version = "2.0.0", features = ["deep-link"] }

  * 然后配置 single instance 插件，它应该是你注册的第一个插件。

src-tauri/lib.rs

    #[cfg_attr(mobile, tauri::mobile_entry_point)]

    pub fn run() {

        let mut builder = tauri::Builder::default();

        #[cfg(desktop)]

        {

            builder = builder.plugin(tauri_plugin_single_instance::init(|_app, argv, _cwd| {

              println!("a new app instance was opened with {argv:?} and the deep link event was already triggered");

              // when defining deep link schemes at runtime, you must also check `argv` here

            }));

        }

        builder = builder.plugin(tauri_plugin_deep_link::init());

    }

注意

用户可能通过将 URL 作为参数传入来手动触发伪造的深度链接。Tauri 会将命令行参数与配置的方案进行匹配以降低风险，但你仍应检查 URL 是否符合预期的格式。

这意味着 Tauri 仅处理静态配置方案的深度链接，而在运行时注册的方案必须使用 [`Env::args_os`](https://docs.rs/tauri/2.0.0/tauri/struct.Env.html#structfield.args_os) 进行手动检查。

## 配置

名为“配置”的章节

在 `tauri.conf.json > plugins > deep-link` 下，配置你想与应用关联的移动端域名/方案以及桌面端方案。

### 示例

名为“示例”的章节

**移动端自定义方案（无需服务器）**

tauri.conf.json

    {

      "plugins": {

        "deep-link": {

          "mobile": [

            {

              "scheme": ["ovi"],

              "appLink": false

            }

          ]

        }

      }

    }

这将注册 Android 和 iOS 上的 `ovi://*` 方案。

**应用链接 / 通用链接（已验证的 https + 主机名）**

    {

      "plugins": {

        "deep-link": {

          "mobile": [

            {

              "scheme": ["https"],

              "host": "your.website.com",

              "pathPrefix": ["/open"],

              "appLink": true

            }

          ]

        }

      }

    }

这将注册 `https://your.website.com/open/*` 作为应用链接/通用链接。

**桌面端自定义方案**

    {

      "plugins": {

        "deep-link": {

          "desktop": {

            "schemes": ["something", "my-tauri-app"]

          }

        }

      }

    }

## 用法

名为“用法”的章节

deep-link 插件同时支持 JavaScript 和 Rust。

### 监听深度链接

章节标题 “监听深度链接”

  * JavaScript
  * Rust

当深度链接在应用运行时触发时，会调用 `onOpenUrl` 回调。若要在应用启动时检测它是否是通过深度链接打开的，请使用 `getCurrent`。

    import { getCurrent, onOpenUrl } from '@tauri-apps/plugin-deep-link';

    // when using `"withGlobalTauri": true`, you may use

    // const { getCurrent, onOpenUrl } = window.__TAURI__.deepLink;

    const startUrls = await getCurrent();

    if (startUrls) {

      // App was likely started via a deep link

      // Note that getCurrent's return value will also get updated every time onOpenUrl gets triggered.

    }

    await onOpenUrl((urls) => {

      console.log('deep link:', urls);

    });

当深度链接在应用运行时触发时，会调用该插件的 `on_open_url` 闭包。若要在应用启动时检测它是否是通过深度链接打开的，请使用 `get_current`。

src-tauri/src/lib.rs

    use tauri_plugin_deep_link::DeepLinkExt;

    #[cfg_attr(mobile, tauri::mobile_entry_point)]

    pub fn run() {

        tauri::Builder::default()

            .plugin(tauri_plugin_deep_link::init())

            .setup(|app| {

                // Note that get_current's return value will also get updated every time on_open_url gets triggered.

                let start_urls = app.deep_link().get_current()?;

                if let Some(urls) = start_urls {

                    // app was likely started by a deep link

                    println!("deep link URLs: {:?}", urls);

                }

                app.deep_link().on_open_url(|event| {

                    println!("deep link URLs: {:?}", event.urls());

                });

                Ok(())

            })

            .run(tauri::generate_context!())

            .expect("error while running tauri application");

    }

注意

打开 URL 事件会触发一个 URL 列表（以兼容 macOS 的深度链接 API），但在大多数情况下，你的应用只会接收到一个 URL。

### 在运行时注册桌面端深度链接

章节标题 “在运行时注册桌面端深度链接”

配置章节描述了如何为你的应用定义静态深度链接方案。

在 Linux 和 Windows 上，也可以通过 Rust 的 `register` 函数在运行时将方案与你的应用关联。

在下面的片段中，我们将运行时注册 `my-app` 方案。应用首次执行后，操作系统将使用我们的应用程序打开 `my-app://*` URL。

src-tauri/src/lib.rs

    use tauri_plugin_deep_link::DeepLinkExt;

    #[cfg_attr(mobile, tauri::mobile_entry_point)]

    pub fn run() {

        tauri::Builder::default()

            .plugin(tauri_plugin_deep_link::init())

            .setup(|app| {

                #[cfg(desktop)]

                app.deep_link().register("my-app")?;

                Ok(())

            })

            .run(tauri::generate_context!())

            .expect("error while running tauri application");

    }

注意

在运行时注册深度链接对于 Linux 和 Windows 的开发非常有用，因为默认情况下深度链接仅在应用安装时注册。

安装 AppImage 可能很复杂，因为它需要 AppImage 启动器。

通常更倾向于在运行时注册深度链接，因此 Tauri 还包含一个辅助函数，用于强制在运行时注册所有静态配置的深度链接。调用此函数还可确保在开发模式下注册深度链接。

    #[cfg(any(target_os = "linux", all(debug_assertions, windows)))]

    {

      use tauri_plugin_deep_link::DeepLinkExt;

      app.deep_link().register_all()?;

    }

## 测试

名为“测试”的章节

测试应用的深度链接时需要注意一些事项。

### Desktop

名为“Desktop”的章节

深度链接仅对已安装的桌面端应用生效。在 Linux 和 Windows 上，你可以使用 Rust 函数 [`register_all`](https://docs.rs/tauri-plugin-deep-link/2.0.0/tauri_plugin_deep_link/struct.DeepLink.html#method.register_all) 来绕过此限制，该函数会注册所有已配置的方案以触发当前可执行文件。

src-tauri/src/lib.rs

    use tauri_plugin_deep_link::DeepLinkExt;

    #[cfg_attr(mobile, tauri::mobile_entry_point)]

    pub fn run() {

        tauri::Builder::default()

            .plugin(tauri_plugin_deep_link::init())

            .setup(|app| {

                #[cfg(any(windows, target_os = "linux"))]

                {

                    use tauri_plugin_deep_link::DeepLinkExt;

                    app.deep_link().register_all()?;

                }

                Ok(())

            })

            .run(tauri::generate_context!())

            .expect("error while running tauri application");

    }

注意

在 Linux 上安装支持深度链接的 AppImage 需要 AppImage 启动器将 AppImage 与操作系统集成。使用 `register_all` 函数，你无需用户安装外部工具即可开箱即用地支持深度链接。

当 AppImage 移动到文件系统中的不同位置时，深度链接会因其利用了可执行文件的绝对路径而失效，这使得在运行时注册方案显得更加重要。

有关更多信息，请参阅 在运行时注册桌面端深度链接 章节。

注意

在 macOS 上无法在运行时注册深度链接，因此深度链接只能在已打包的应用上进行测试，并且该应用必须安装在 `/Applications` 目录中。

#### Windows

名为“窗口”的章节

要在 Windows 上触发深度链接，你可以在浏览器中打开 `<scheme>://url` 或在终端中运行以下命令

终端窗口

    start <scheme>://url

#### Linux

名为“Linux”的章节

要在 Linux 上触发深度链接，你可以在浏览器中打开 `<scheme>://url` 或在终端中运行 `xdg-open`

终端窗口

    xdg-open <scheme>://url

### iOS

名为“iOS”的章节

要在 iOS 上触发应用链接，你可以在浏览器中打开 `https://<host>/path` URL。对于模拟器，你可以利用 `simctl` CLI 直接从终端打开链接。

终端窗口

    xcrun simctl openurl booted https://<host>/path

### Android

标题为“Android”的章节

要在 Android 上触发应用链接，你可以在浏览器中打开 `https://<host>/path` URL。对于模拟器，你可以利用 `adb` CLI 直接从终端打开链接。

终端窗口

    adb shell am start -a android.intent.action.VIEW -d https://<host>/path <bundle-identifier>

## Permissions

名为“权限”的章节

默认情况下，所有潜在危险的插件命令和范围都被阻止，无法访问。您必须修改 `capabilities` 配置中的权限才能启用这些功能。

有关更多信息，请参阅[功能概述](/security/capabilities/)，并参阅[分步指南](/learn/security/using-plugin-permissions/)以使用插件权限。

src-tauri/capabilities/default.json

    {

      "$schema": "../gen/schemas/mobile-schema.json",

      "identifier": "mobile-capability",

      "windows": ["main"],

      "platforms": ["iOS", "android"],

      "permissions": [

        // Usually you will need core:event:default to listen to the deep-link event

        "core:event:default",

        "deep-link:default"

      ]

    }

## 默认权限

允许通过 get_current 命令读取已打开的深度链接

#### 此默认权限集包括以下内容

  * `allow-get-current`

## 权限表

标识符 | 描述
---|---
`deep-link:allow-get-current` |  在没有预配置范围的情况下启用 get_current 命令。
`deep-link:deny-get-current` |  在没有预配置范围的情况下拒绝 get_current 命令。
`deep-link:allow-is-registered` |  在没有预配置范围的情况下启用 is_registered 命令。
`deep-link:deny-is-registered` |  在没有预配置范围的情况下拒绝 is_registered 命令。
`deep-link:allow-register` |  在没有预配置范围的情况下启用 register 命令。
`deep-link:deny-register` |  在没有预配置范围的情况下拒绝 register 命令。
`deep-link:allow-unregister` |  在没有预配置范围的情况下启用 unregister 命令。
`deep-link:deny-unregister` |  在没有预配置范围的情况下拒绝 unregister 命令。