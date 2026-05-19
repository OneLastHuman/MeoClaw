# 应用商店

_Source: https://v2.tauri.org.cn/distribute/app-store/_

[Apple App Store](https://www.apple.com/store) 是由 Apple 维护的应用市场。您可以将您的 Tauri 应用通过该 App Store 分发至 macOS 和 iOS 平台。

本指南仅涵盖直接分发应用至 App Store 的相关细节。有关 macOS 分发选项和配置的更多信息，请参阅通用的[应用包 (App Bundle)](/distribute/macos-application-bundle/) 指南。

## 要求

名为“要求”的章节

分发 iOS 和 macOS 应用需要加入 [Apple 开发者](https://developer.apple.com)计划。

此外，您必须为 [macOS](/distribute/sign/macos/) 和 [iOS](/distribute/sign/ios/) 设置代码签名。

## 更改应用图标

名为“更改应用图标”的章节

在运行 `tauri ios init` 设置 Xcode 项目后，您可以使用 `tauri icon` 命令来更新应用图标。

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri icon /path/to/app-icon.png -- --ios-color '#fff'

    yarn tauri icon /path/to/app-icon.png --ios-color '#fff'

    pnpm tauri icon /path/to/app-icon.png --ios-color '#fff'

    deno task tauri icon /path/to/app-icon.png --ios-color '#fff'

    bun tauri icon /path/to/app-icon.png --ios-color '#fff'

    cargo tauri icon /path/to/app-icon.png --ios-color '#fff'

`--ios-color` 参数用于定义 iOS 图标的背景颜色。

## 设置

名为“设置”的章节

在加入 Apple 开发者计划后，在 App Store 中分发 Tauri 应用的第一步是在 [App Store Connect](https://appstoreconnect.apple.com/apps) 中注册您的应用。

注意

 _Bundle ID_ 字段中提供的值**必须** 与 [`tauri.conf.json > identifier`](/reference/config/#identifier) 中定义的标识符匹配。

## 构建与上传

名为“构建与上传”的章节

Tauri CLI 可以为您打包 macOS 和 iOS 应用。要求必须在 macOS 机器上运行。

Tauri 从 [`tauri.conf.json > version`] 中定义的值导出 [`CFBundleVersion`](https://developer.apple.com/documentation/bundleresources/information-property-list/cfbundleversion)。如果您需要不同的捆绑包版本方案（例如顺序编码），可以在 [`tauri.conf.json > bundle > iOS > bundleVersion`] 或 [`tauri.conf.json > bundle > macOS > bundleVersion`] 配置中设置自定义版本号。

tauri.conf.json

    {

      "bundle": {

        "iOS": {

          "bundleVersion": "100"

        }

      }

    }

注意

必须进行代码签名。请参阅 [macOS](/distribute/sign/macos/) 和 [iOS](/distribute/sign/ios/) 的相关文档。

请注意，Tauri 利用 Xcode 进行 iOS 应用开发，因此您可以使用 Xcode 对 iOS 应用进行归档 (Archive) 和分发，而不必使用 Tauri CLI。要打开 Xcode 中的 iOS 项目进行构建，必须运行以下命令

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri ios build -- --open

    yarn tauri ios build --open

    pnpm tauri ios build --open

    deno task tauri ios build --open

    bun tauri ios build --open

    cargo tauri ios build --open

### macOS

名为“macOS”的章节

要将应用上传到 App Store，首先必须确保设置了所有必需的配置选项，以便打包 App Bundle、创建已签名的 `.pkg` 文件并进行上传。

以下章节将引导您完成整个过程。

#### 设置

标题为“设置”的章节

您的应用必须包含某些配置才能被 App Store 验证系统接受。

提示

以下章节将引导您完成 App Store 提交的应用配置。

若要仅在构建 App Store 版本时应用以下配置更改，您可以创建一个单独的 Tauri 配置文件

"src-tauri/tauri.appstore.conf.json

    {

      "bundle": {

        "macOS": {

          "entitlements": "./Entitlements.plist",

          "files": {

            "embedded.provisionprofile": "path/to/profile-name.provisionprofile"

          }

        }

      }

    }

然后在为 App Store 打包 Tauri 应用时，将该配置文件与主配置文件进行合并。

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri build -- --no-bundle

    npm run tauri bundle -- --bundles app --target universal-apple-darwin --config src-tauri/tauri.appstore.conf.json

    yarn tauri build --no-bundle

    yarn tauri bundle --bundles app --target universal-apple-darwin --config src-tauri/tauri.appstore.conf.json

    pnpm tauri build --no-bundle

    pnpm tauri bundle --bundles app --target universal-apple-darwin --config src-tauri/tauri.appstore.conf.json

    deno task tauri build --no-bundle

    deno task tauri bundle --bundles app --target universal-apple-darwin --config src-tauri/tauri.appstore.conf.json

    bun tauri build --no-bundle

    bun tauri bundle --bundles app --target universal-apple-darwin --config src-tauri/tauri.appstore.conf.json

    cargo tauri build --no-bundle

    cargo tauri bundle --bundles app --target universal-apple-darwin --config src-tauri/tauri.appstore.conf.json

这在设置 CI/CD 以将应用上传到 App Store 时特别有用，无需在本地配置预配描述文件 (provision profile)，或者在编译非 App Store 分发的应用时也可以使用。

  * 类别

您的应用必须在 [`tauri.conf.json > bundle > category`](/reference/config/#category) 中定义其类别，以便在 App Store 中显示。

tauri.conf.json

    {

      "bundle": {

        "category": "Utility"

      }

    }

  * 预配描述文件 (Provisioning profile)

您还必须为您的应用创建一个预配描述文件，以便通过 Apple 的验证。

在 [Identifiers](https://developer.apple.com/account/resources/identifiers/list) 页面中，创建一个新的 App ID，并确保其“Bundle ID”值与 [`tauri.conf.json > identifier`](/reference/config/#identifier) 中设置的标识符相匹配。

导航至 [Profiles](https://developer.apple.com/account/resources/profiles/list) 页面创建一个新的预配描述文件。对于 macOS App Store 分发，必须是“Mac App Store Connect”描述文件。选择合适的 App ID 并关联您用于代码签名的证书。

创建预配描述文件后，将其下载并保存到已知位置，并配置 Tauri 将其包含在您的应用包中。

tauri.conf.json

    {

      "bundle": {

        "macOS": {

          "files": {

            "embedded.provisionprofile": "path/to/profile-name.provisionprofile"

          }

        }

      }

    }

  * Info.plist

您的应用必须遵守加密出口法规。有关更多信息，请参阅[官方文档](https://developer.apple.com/documentation/security/complying-with-encryption-export-regulations?language=objc)。

在 src-tauri 文件夹中创建一个 Info.plist 文件

    <?xml version="1.0" encoding="UTF-8"?>

    <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">

    <plist version="1.0">

    <dict>

      <key>ITSAppUsesNonExemptEncryption</key>

      <false/> # or `true` if your app uses encryption

    </dict>

    </plist>

  * Entitlements（授权）

您的应用必须包含 App Sandbox 能力才能在 App Store 中分发。此外，您还必须在代码签名授权中设置您的 App ID 和 Team ID。

在 `src-tauri` 文件夹中创建一个 `Entitlements.plist` 文件

    <?xml version="1.0" encoding="UTF-8"?>

    <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">

    <plist version="1.0">

    <dict>

        <key>com.apple.security.app-sandbox</key>

        <true/>

        <key>com.apple.application-identifier</key>

        <string>$TEAM_ID.$IDENTIFIER</string>

        <key>com.apple.developer.team-identifier</key>

        <string>$TEAM_ID</string>

    </dict>

    </plist>

请注意，您必须将 `$IDENTIFIER` 替换为 [`tauri.conf.json > identifier`](/reference/config/#identifier) 的值，并将 `$TEAM_ID` 替换为您的 Apple 开发者团队 ID。该 ID 可以在您为预配描述文件创建的 [Identifier](https://developer.apple.com/account/resources/identifiers/list) 中的 `App ID Prefix` 部分找到。

并在 macOS 包配置 [`tauri.conf.json > bundle > macOS > entitlements`](/reference/config/#entitlements) 中引用该文件

tauri.conf.json

    {

      "bundle": {

        "macOS": {

          "entitlements": "./Entitlements.plist"

        }

      }

    }

现在，您必须在启用代码签名的情况下构建应用程序，以便应用这些授权配置。

确保您的应用在 App Sandbox 环境中正常工作。

#### 构建

名为“构建”的章节

您必须将您的 macOS 应用程序作为 `.pkg` 文件上传到 App Store。运行以下命令将您的应用打包为 macOS 应用包（`.app` 扩展名）

    tauri build --bundles app --target universal-apple-darwin

注意

上述命令会创建一个支持 Apple Silicon 和 Intel 处理器的通用应用二进制文件 (Universal App Binary)。

如果您希望仅支持 Apple Silicon，则必须将 [`tauri.conf.json > bundle > macOS > minimumSystemVersion`](/reference/config/#minimumsystemversion) 更改为 `12.0`

tauri.conf.json

    {

      "bundle": {

        "macOS": {

          "minimumSystemVersion": "12.0"

        }

      }

    }

并根据您运行的 Mac 系统更改 CLI 命令和输出路径。

  * 如果您的构建系统使用的是 Apple Silicon 芯片，请移除 `--target universal-apple-darwin` 参数，并在下方引用的路径中使用 `target/release`，而不是 `target/universal-apple-darwin/release`。
  * 如果您的构建系统使用的是 Intel 芯片
    * 安装 Rust 的 Apple Silicon 目标架构

          rustup target add aarch64-apple-darwin

    * 将 `universal-apple-darwin` 参数更改为 `aarch64-apple-darwin`，并在下方引用的路径中使用 `target/aarch64-apple-darwin/release`，而不是 `target/universal-apple-darwin/release`。

有关配置选项的更多信息，请参阅 [应用包分发指南](/distribute/macos-application-bundle/)。

要从应用包生成已签名的 `.pkg`，请运行以下命令

    xcrun productbuild --sign "<certificate signing identity>" --component "target/universal-apple-darwin/release/bundle/macos/$APPNAME.app" /Applications "$APPNAME.pkg"

请注意，您必须将 _$APPNAME_ 替换为您的应用名称。

注意

您必须使用 _Mac Installer Distribution_ 签名证书对 PKG 进行签名。

#### 上传

名为“上传”的章节

现在，您可以使用 [`altool`](https://help.apple.com/itc/apploader/#/apdATD1E53-D1E1A1303-D1E53A1126) CLI 将您的应用 PKG 上传到 App Store

    xcrun altool --upload-app --type macos --file "$APPNAME.pkg" --apiKey $APPLE_API_KEY_ID --apiIssuer $APPLE_API_ISSUER

请注意，`altool` 上传应用需要 App Store Connect API 密钥。有关更多信息，请参阅身份验证部分。

您的应用将由 Apple 进行验证，如果获得批准，将可在 TestFlight 中使用。

### iOS

名为“iOS”的章节

要构建您的 iOS 应用，请运行 `tauri ios build` 命令

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri ios build -- --export-method app-store-connect

    yarn tauri ios build --export-method app-store-connect

    pnpm tauri ios build --export-method app-store-connect

    deno task tauri ios build --export-method app-store-connect

    bun tauri ios build --export-method app-store-connect

    cargo tauri ios build --export-method app-store-connect

生成的 IPA 文件可在 `src-tauri/gen/apple/build/arm64/$APPNAME.ipa` 中找到。

请注意，您必须将 _$APPNAME_ 替换为您的应用名称。

现在，您可以使用 `altool` CLI 将您的 iOS 应用上传到 App Store

    xcrun altool --upload-app --type ios --file "src-tauri/gen/apple/build/arm64/$APPNAME.ipa" --apiKey $APPLE_API_KEY_ID --apiIssuer $APPLE_API_ISSUER

请注意，`altool` 上传应用需要 App Store Connect API 密钥。有关更多信息，请参阅身份验证部分。

您的应用将由 Apple 进行验证，如果获得批准，将可在 TestFlight 中使用。

### 身份验证

名为“身份验证”的章节

iOS 和 macOS 应用通过 `altool` 上传，它使用 App Store Connect API 密钥进行身份验证。

要创建新的 API 密钥，请打开 [App Store Connect 的“用户和访问”页面](https://appstoreconnect.apple.com/access/users)，选择“集成”>“密钥”选项卡，点击“添加”按钮，选择一个名称和“开发者”权限。`APPLE_API_ISSUER` (Issuer ID) 显示在密钥表格上方，`APPLE_API_KEY_ID` 是该表格中“Key ID”列的值。您还需要下载私钥，该操作只能执行一次，并且仅在页面重新加载后可见（按钮显示在新创建密钥的表格行中）。私钥文件路径必须以 `AuthKey_<APPLE_API_KEY_ID>.p8` 命名，并保存在以下目录之一中：`<当前工作目录>/private_keys`、`~/private_keys`、`~/.private_keys` 或 `~/.appstoreconnect/private_keys`。