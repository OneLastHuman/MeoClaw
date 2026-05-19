# macOS 代码签名

_Source: https://v2.tauri.org.cn/distribute/sign/macos/_

在 macOS 上，代码签名是必要的。这不仅是为了让您的应用程序能够列入 [Apple App Store](https://www.apple.com/app-store/)，还可以防止用户从浏览器下载应用时弹出“应用已损坏，无法启动”的警告。

## 先决条件

标题为“前提条件”的章节

在 macOS 上进行代码签名需要一个 [Apple Developer](https://developer.apple.com) 账号，可以是付费账号（每年 99 美元）或免费计划（仅限测试和开发目的）。此外，您还需要一台用于执行代码签名的 Apple 设备。这是 Apple 的条款和条件及签名流程所要求的。

注意

注意：如果您使用的是免费的 Apple Developer 账号，将无法对应用程序进行公证，并且在打开应用时，它仍会显示为“未经验证”。

## 签名

名为“签名”的章节

要配置 macOS 的代码签名，您必须创建一个 Apple 代码签名证书，并将其安装到 Mac 的钥匙串（Keychain）中，或者导出该证书以便在 CI/CD 平台中使用。

### 创建签名证书

名为“创建签名证书”的章节

要创建新的签名证书，您必须在 Mac 电脑上生成一个证书签名请求 (CSR) 文件。请参考 [创建证书签名请求](https://developer.apple.com/help/account/create-certificates/create-a-certificate-signing-request) 以了解如何制作用于代码签名的 CSR。

登录您的 Apple Developer 账号，导航至 [Certificates, IDs & Profiles（证书、标识符和配置文件）页面](https://developer.apple.com/account/resources/certificates/list)，点击 `Create a certificate`（创建证书）按钮打开创建界面。选择合适的证书类型（选择 `Apple Distribution` 用于提交应用到 App Store，选择 `Developer ID Application` 用于在 App Store 之外发布应用）。上传您的 CSR，证书即可创建完成。

注意

只有 Apple Developer 的 `Account Holder`（账户持有人）才能创建 _Developer ID Application_ 证书。但是，通过使用其他用户的电子邮件地址创建 CSR，可以将其与不同的 Apple ID 关联。

### 下载证书

名为“下载证书”的章节

在 [Certificates, IDs & Profiles（证书、标识符和配置文件）页面](https://developer.apple.com/account/resources/certificates/list)上，点击您要使用的证书，然后点击 `Download`（下载）按钮。这会保存一个 `.cer` 文件，打开后即可将证书安装到钥匙串中。

### 配置 Tauri

名为“配置 Tauri”的章节

您可以配置 Tauri，以便在本地构建 macOS 应用或使用 CI/CD 平台时使用您的证书。

#### 本地签名

名为“本地签名”的章节

将证书安装到 Mac 的钥匙串中后，您可以配置 Tauri 将其用于代码签名。

证书在钥匙串中的条目名称即为 `signing identity`（签名标识），也可以通过运行以下命令找到：

终端窗口

    security find-identity -v -p codesigning

此标识可以在 [`tauri.conf.json > bundle > macOS > signingIdentity`](/reference/config/#signingidentity) 配置项中提供，或者通过 `APPLE_SIGNING_IDENTITY` 环境变量设置。

注意

签名证书必须与您的 Apple ID 关联才有效。如果证书无效，它将不会出现在 _钥匙串访问 (Keychain Access) > 我的证书_ 标签页或 _security find-identity -v -p codesigning_ 的输出结果中。如果证书未下载到正确位置，请确保下载 .cer 文件时在 _钥匙串访问_ 的“默认钥匙串”下选择了“登录 (login)”。

#### 在 CI/CD 平台中签名

名为“在 CI/CD 平台中签名”的章节

若要在 CI/CD 平台中使用证书，您必须将证书导出为 base64 字符串，并配置 `APPLE_CERTIFICATE` 和 `APPLE_CERTIFICATE_PASSWORD` 环境变量。

  1. 打开 `钥匙串访问` 应用，点击 _登录 (login)_ 钥匙串中的 _我的证书_ 标签页，找到您的证书条目。
  2. 展开该条目，右键点击密钥项，选择 `导出 "$KEYNAME"`。
  3. 选择保存证书 `.p12` 文件的路径，并为导出的证书设置一个密码。
  4. 在终端运行以下脚本，将 `.p12` 文件转换为 base64：

终端窗口

    openssl base64 -A -in /path/to/certificate.p12 -out certificate-base64.txt

  5. 将 `certificate-base64.txt` 文件的内容设置为 `APPLE_CERTIFICATE` 环境变量。
  6. 将证书密码设置为 `APPLE_CERTIFICATE_PASSWORD` 环境变量。

GitHub Actions 配置示例

所需 Secrets

  * `APPLE_ID` \- 您的 Apple ID 电子邮件地址
  * `APPLE_PASSWORD` \- 您的 Apple ID 密码
  * `APPLE_CERTIFICATE` \- base64 编码的 `.p12` 文件
  * `APPLE_CERTIFICATE_PASSWORD` \- 导出 `.p12` 文件时设置的密码
  * `KEYCHAIN_PASSWORD` \- 您的钥匙串密码

查看 GitHub 官方指南，了解 [如何设置 Secret](https://githubdocs.cn/en/actions/security-for-github-actions/security-guides/using-secrets-in-github-actions#creating-secrets-for-a-repository)。

    name: 'build'

    on:

      push:

        branches:

          - main

    jobs:

      build-macos:

        needs: prepare

        strategy:

          matrix:

            include:

              - args: '--target aarch64-apple-darwin'

                arch: 'silicon'

              - args: '--target x86_64-apple-darwin'

                arch: 'intel'

        runs-on: macos-latest

        env:

          APPLE_ID: ${{ secrets.APPLE_ID }}

          APPLE_PASSWORD: ${{ secrets.APPLE_PASSWORD }}

        steps:

          - name: Import Apple Developer Certificate

            env:

              APPLE_CERTIFICATE: ${{ secrets.APPLE_CERTIFICATE }}

              APPLE_CERTIFICATE_PASSWORD: ${{ secrets.APPLE_CERTIFICATE_PASSWORD }}

              KEYCHAIN_PASSWORD: ${{ secrets.KEYCHAIN_PASSWORD }}

            run: |

              echo $APPLE_CERTIFICATE | base64 --decode > certificate.p12

              security create-keychain -p "$KEYCHAIN_PASSWORD" build.keychain

              security default-keychain -s build.keychain

              security unlock-keychain -p "$KEYCHAIN_PASSWORD" build.keychain

              security set-keychain-settings -t 3600 -u build.keychain

              security import certificate.p12 -k build.keychain -P "$APPLE_CERTIFICATE_PASSWORD" -T /usr/bin/codesign

              security set-key-partition-list -S apple-tool:,apple:,codesign: -s -k "$KEYCHAIN_PASSWORD" build.keychain

              security find-identity -v -p codesigning build.keychain

          - name: Verify Certificate

            run: |

              CERT_INFO=$(security find-identity -v -p codesigning build.keychain | grep "Apple Development")

              CERT_ID=$(echo "$CERT_INFO" | awk -F'"' '{print $2}')

              echo "CERT_ID=$CERT_ID" >> $GITHUB_ENV

              echo "Certificate imported."

          - uses: tauri-apps/tauri-action@v0

            env:

              GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}

              APPLE_CERTIFICATE: ${{ secrets.APPLE_CERTIFICATE }}

              APPLE_CERTIFICATE_PASSWORD: ${{ secrets.APPLE_CERTIFICATE_PASSWORD }}

              APPLE_SIGNING_IDENTITY: ${{ env.CERT_ID }}

            with:

              args: ${{ matrix.args }}

## 公证 (Notarization)

名为“公证 (Notarization)”的章节

要公证您的应用程序，您必须提供凭据以供 Tauri 与 Apple 进行身份验证。这可以通过 App Store Connect API 或使用您的 Apple ID 来完成。

  * App Store Connect
  * Apple ID

  1. 打开 [App Store Connect 的“用户和职能”页面](https://appstoreconnect.apple.com/access/users)，选择“集成”标签页，点击添加按钮，输入名称并授予“开发者”访问权限。
  2. 将 `APPLE_API_ISSUER` 环境变量设置为密钥表上方显示的值。
  3. 将 `APPLE_API_KEY` 环境变量设置为该表中“密钥 ID”列的值。
  4. 下载私钥。私钥只能下载一次，且仅在页面刷新后可见（按钮显示在新创建密钥的表格行中）。
  5. 将 `APPLE_API_KEY_PATH` 环境变量设置为下载的私钥的文件路径。

  1. 将 `APPLE_ID` 环境变量设置为您的 Apple 账号电子邮件地址。
  2. 将 `APPLE_PASSWORD` 环境变量设置为您的 Apple 账号的 [应用专用密码 (app-specific password)](https://support.apple.com/en-ca/HT204397)。
  3. 将 `APPLE_TEAM_ID` 环境变量设置为您的 Apple 团队 ID。您可以在 [账号的成员资格页面](https://developer.apple.com/account#MembershipDetailsCard) 找到您的团队 ID。

注意

使用 _Developer ID Application_ 证书时，公证是必需的。

## Ad-Hoc 签名

名为“Ad-Hoc 签名”的章节

如果您不想提供 Apple 认证的身份，但仍希望对应用程序进行签名，可以配置 _ad-hoc_ 签名。

这在 ARM (Apple Silicon) 设备上非常有用，因为这些设备要求从互联网下载的所有应用都必须进行代码签名。

注意

Ad-hoc 代码签名并不能阻止 macOS 要求用户 [在“隐私与安全性”设置中将安装加入白名单](https://support.apple.com/guide/mac-help/open-a-mac-app-from-an-unknown-developer-mh40616/mac)。

要配置 ad-hoc 签名，只需向 Tauri 提供伪标识 `-`，例如：

    "signingIdentity": "-"

有关配置 Tauri 签名标识的详细信息，请参阅 上文。