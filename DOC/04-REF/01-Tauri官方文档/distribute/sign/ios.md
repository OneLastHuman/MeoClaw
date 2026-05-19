# iOS 代码签名

_Source: https://v2.tauri.org.cn/distribute/sign/ios/_

iOS 代码签名是分发应用程序所必需的，无论是通过官方 [Apple App Store](https://www.apple.com/app-store/)，还是欧盟地区的替代应用市场，亦或是在终端用户设备上进行安装和执行，都必须进行签名。

## 先决条件

标题为“前提条件”的章节

iOS 代码签名要求加入 [Apple Developer](https://developer.apple.com) 计划，截至本文撰写时，费用为每年 99 美元。您还需要一台能够执行代码签名的 Apple 设备。这是签名过程的要求，也是 Apple 条款和条件所规定的。

若要分发 iOS 应用程序，必须在 App Store Connect 中注册您的 Bundle ID，拥有适当的 iOS 代码签名证书，以及一个将两者关联并启用 App 所需功能的移动预置描述文件 (Provisioning Profile)。这些要求可以通过 Xcode 自动管理，也可以手动提供。

## 自动签名

标题为“自动签名”的章节

让 Xcode 管理应用程序的签名和预置是导出 iOS App 以供分发的最便捷方式。它会自动注册您的 Bundle ID，管理 iOS 功能变更，并根据您的导出方法配置合适的证书。

自动签名是默认启用的，并在本地机器上使用 Xcode 中配置的账户进行身份验证。
要注册您的账户，请打开 Xcode 应用程序，进入 `Xcode > Settings` 菜单，切换到“Accounts”选项卡，然后点击 `+` 图标。

要在 CI/CD 平台中使用自动签名，您必须创建 App Store Connect API 密钥，并定义 `APPLE_API_ISSUER`、`APPLE_API_KEY` 和 `APPLE_API_KEY_PATH` 环境变量。
打开 [App Store Connect 的“用户和访问”页面](https://appstoreconnect.apple.com/access/users)，选择“Integrations”选项卡，点击“添加”按钮，选择一个名称并赋予“Admin”（管理员）访问权限。`APPLE_API_ISSUER` (Issuer ID) 显示在密钥表格的上方，`APPLE_API_KEY` 是该表格中“Key ID”列的值。您还需要下载私钥，该私钥只能下载一次，且仅在页面刷新后可见（按钮显示在新建密钥的表格行中）。私钥文件路径必须通过 `APPLE_API_KEY_PATH` 环境变量设置。

## 手动签名

标题为“手动签名”的章节

要手动签署 iOS 应用程序，您可以通过环境变量提供证书和移动预置描述文件：

  * **IOS_CERTIFICATE** ：从 Keychain 导出的证书的 base64 表示。
  * **IOS_CERTIFICATE_PASSWORD** ：从 Keychain 导出证书时设置的密码。
  * **IOS_MOBILE_PROVISION** ：预置描述文件的 base64 表示。

以下章节将解释如何获取这些值。

### 签名证书

标题为“签名证书”的章节

注册完成后，导航至 [Certificates](https://developer.apple.com/account/resources/certificates/list)（证书）页面以创建新的 Apple Distribution（Apple 分发）证书。下载新证书并将其安装到 macOS Keychain 中。

要导出证书密钥，请打开“钥匙串访问” (Keychain Access) 应用，展开证书条目，右键点击密钥项，选择“导出 <key-name>”项。选择导出 .p12 文件的路径并记住其密码。

运行以下 `base64` 命令将证书转换为 base64 并将其复制到剪贴板：

    base64 -i <path-to-certificate.p12> | pbcopy

剪贴板中的值现在就是签名证书的 base64 表示。保存该值并将其用作 `IOS_CERTIFICATE` 环境变量的值。

证书密码必须设置为 `IOS_CERTIFICATE_PASSWORD` 变量。

选择证书类型

对于每种导出方法，您必须使用适当的证书类型：

  * **调试 (debugging)** ：Apple Development 或 iOS App Development
  * **App Store Connect** ：Apple Distribution 或 iOS Distribution (App Store Connect 和 Ad Hoc)
  * **Ad Hoc** ：Apple Distribution 或 iOS Distribution (App Store Connect 和 Ad Hoc)

### 预置描述文件 (Provisioning Profile)

标题为“预置描述文件”的章节

此外，您必须为应用程序提供预置描述文件。在 [Identifiers](https://developer.apple.com/account/resources/identifiers/list)（标识符）页面中，创建一个新的 App ID，并确保其“Bundle ID”值与 [`identifier`](/reference/config/#identifier) 配置中设置的 ID 一致。

导航至 [Profiles](https://developer.apple.com/account/resources/profiles/list)（描述文件）页面以创建新的预置描述文件。对于 App Store 分发，它必须是“App Store Connect”类型的描述文件。选择相应的 App ID 并关联您之前创建的证书。

创建预置描述文件后，下载它并运行以下 `base64` 命令将描述文件转换并复制到剪贴板：

    base64 -i <path-to-profile.mobileprovision> | pbcopy

剪贴板中的值现在就是预置描述文件的 base64 表示。保存该值并将其用作 `IOS_MOBILE_PROVISION` 环境变量的值。

现在您可以构建您的 iOS 应用程序并将其分发到 App Store 了！