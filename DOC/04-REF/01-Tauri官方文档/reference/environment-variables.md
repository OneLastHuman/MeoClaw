# 环境变量

_Source: https://v2.tauri.org.cn/reference/environment-variables/_

本文档记录了 Tauri 核心 crate 和 Tauri CLI 使用的所有环境变量。

## Tauri CLI

标题为“Tauri CLI”的章节

这些环境变量是 CLI 的输入，其中一些可能有对应的 CLI 标志（Flag）。

环境变量优先级

如果同时使用了环境变量和 CLI 标志，CLI 标志具有优先权。

  * `CI` — 如果设置，CLI 将以 CI 模式运行，且不需要任何用户交互。
  * `TAURI_CLI_CONFIG_DEPTH` — 查找 Tauri 配置文件时遍历的目录层级数。
  * `TAURI_CLI_PORT` — 用于 CLI 内置开发服务器的端口。
  * `TAURI_CLI_WATCHER_IGNORE_FILENAME` — 用于控制 `dev` 命令中 CLI 应监视哪些文件的 `.gitignore` 风格的文件名。CLI 将在每个目录中查找此文件名。
  * `TAURI_CLI_NO_DEV_SERVER_WAIT` — 在构建 Tauri 应用之前，跳过等待前端开发服务器启动的过程。
  * `TAURI_LINUX_AYATANA_APPINDICATOR` — 将此变量设置为 `true` 或 `1`，以强制在 Linux 上使用 `libayatana-appindicator` 作为系统托盘。
  * `TAURI_BUNDLER_WIX_FIPS_COMPLIANT` — 指定打包程序的 WiX `FipsCompliant` 选项。
  * `TAURI_BUNDLER_TOOLS_GITHUB_MIRROR` \- 指定一个 GitHub 镜像以供 Tauri 打包程序下载文件和工具。
  * `TAURI_BUNDLER_TOOLS_GITHUB_MIRROR_TEMPLATE` \- 指定一个 GitHub 镜像模板以供 Tauri 打包程序下载文件和工具，例如：`https://mirror.example.com/<owner>/<repo>/releases/download/<version>/<asset>`。
  * `TAURI_SKIP_SIDECAR_SIGNATURE_CHECK` \- 跳过 Sidecar 签名检查。
  * `TAURI_SIGNING_PRIVATE_KEY` — 用于签署应用包的私钥，可以是字符串，也可以是文件路径。
  * `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` — 签名私钥的密码，请参阅 `TAURI_SIGNING_PRIVATE_KEY`。
  * `TAURI_SIGNING_RPM_KEY` — 用于签署 RPM 包的私有 GPG 密钥，以其 ASCII 装甲格式导出。
  * `TAURI_SIGNING_RPM_KEY_PASSPHRASE` — `TAURI_SIGNING_RPM_KEY` 的 GPG 密钥密码短语（如有需要）。
  * `TAURI_WINDOWS_SIGNTOOL_PATH` — 指定用于在 Windows 上进行代码签名的 `signtool.exe` 的路径。
  * `APPLE_CERTIFICATE` — 用于代码签名的 `.p12` 证书的 Base64 编码。要获取此值，请运行 `openssl base64 -A -in MyCertificate.p12 -out MyCertificate-base64.txt`。
  * `APPLE_CERTIFICATE_PASSWORD` — 您用于导出证书的密码。
  * `APPLE_ID` — 用于公证（notarize）应用的 Apple ID。如果提供了此环境变量，则必须同时设置 `APPLE_PASSWORD` 和 `APPLE_TEAM_ID`。或者，可以使用 `APPLE_API_KEY` 和 `APPLE_API_ISSUER` 进行身份验证。
  * `APPLE_PASSWORD` — 用于应用公证身份验证的 Apple 密码。如果指定了 `APPLE_ID`，则此项为必填项。可以使用 [App 专用密码](https://support.apple.com/en-ca/HT204397)。除了以明文输入密码外，还可以使用 '@keychain:' 或 '@env:' 前缀，后跟钥匙串密码项目名称或环境变量名称来指定。
  * `APPLE_TEAM_ID`: 开发者团队 ID。要查找您的团队 ID，请前往 Apple 开发者网站的 [帐户](https://developer.apple.com/account) 页面，并查看您的会员详细信息。
  * `APPLE_API_KEY` — `APPLE_ID` 和 `APPLE_PASSWORD` 的替代方案，用于通过 JWT 进行公证身份验证。它也是允许自动管理 iOS 证书和预置描述文件的选项。
    * 请参阅 [创建 API 密钥](https://developer.apple.com/documentation/appstoreconnectapi/creating_api_keys_for_app_store_connect_api) 以获取更多信息。
  * `API_PRIVATE_KEYS_DIR` — 指定您的 AuthKey 文件所在的目录。请参阅 `APPLE_API_KEY`。
  * `APPLE_API_ISSUER` — 发行者 ID。如果指定了 `APPLE_API_KEY`，则此项为必填项。
  * `APPLE_API_KEY_PATH` \- API 密钥 `.p8` 文件的路径。如果未指定，对于 macOS 应用，打包程序会按顺序搜索以下目录以查找名称为 'AuthKey_<api_key>.p8' 的私钥文件：'./private_keys'、'~/private_keys'、'~/.private_keys' 和 '~/.appstoreconnect/private_keys'。**对于 iOS，此变量为必填项** 。
  * `APPLE_SIGNING_IDENTITY` — 用于代码签名的标识。会覆盖 `tauri.conf.json > bundle > macOS > signingIdentity`。如果两者都未设置，则会在提供 `APPLE_CERTIFICATE` 时从其中推断。
  * `APPLE_PROVIDER_SHORT_NAME` — 如果您的 Apple ID 关联了多个团队，则必须指定用于公证应用的团队的提供商短名称（Provider Short Name）。会覆盖 `tauri.conf.json > bundle > macOS > providerShortName`。
  * `APPLE_DEVELOPMENT_TEAM` — 用于在 iOS 上进行代码签名的团队 ID。会覆盖 `tauri.conf.json > bundle > iOS > developmentTeam`。可在 <https://developer.apple.com/account#MembershipDetailsCard> 中找到。
  * `TAURI_WEBVIEW_AUTOMATION` — 启用 Webview 自动化（仅限 Linux）。
  * `TAURI_ANDROID_PROJECT_PATH` — Tauri Android 项目的路径，通常为 `<project>/src-tauri/gen/android`。
  * `TAURI_IOS_PROJECT_PATH` — Tauri iOS 项目的路径，通常为 `<project>/src-tauri/gen/ios`。

## Tauri CLI 钩子命令

标题为“Tauri CLI 钩子命令”的章节

这些环境变量是为每个钩子命令（`beforeDevCommand`、`beforeBuildCommand` 等）设置的，这对于有条件地构建前端或执行特定操作可能很有用。

  * `TAURI_ENV_DEBUG` — 对于 `dev` 命令或 `build --debug` 为 `true`，否则为 `false`。
  * `TAURI_ENV_TARGET_TRIPLE` — CLI 正在构建的目标三元组（Target Triple）。
  * `TAURI_ENV_ARCH` — 目标架构，`x86_64`、`aarch64` 等。
  * `TAURI_ENV_PLATFORM` — 目标平台，`windows`、`darwin`、`linux` 等。
  * `TAURI_ENV_PLATFORM_VERSION` — 构建平台版本。
  * `TAURI_ENV_FAMILY` — 目标平台系列，`unix` 或 `windows`。