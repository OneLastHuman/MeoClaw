# Windows 代码签名

_Source: https://v2.tauri.org.cn/distribute/sign/windows/_

在 Windows 上必须进行代码签名，以便让您的应用程序能够列入 [Microsoft Store](https://apps.microsoft.com/)，并防止从浏览器下载时出现“应用程序不受信任且无法启动”的 [SmartScreen](https://en.wikipedia.org/wiki/Microsoft_SmartScreen) 警告。

在 Windows 上执行应用程序并不强制要求签名，只要您的最终用户愿意忽略 [SmartScreen](https://en.wikipedia.org/wiki/Microsoft_SmartScreen) 警告，或者用户不是通过浏览器下载的。本指南涵盖了使用 OV（组织验证）证书和 Azure Key Vault 进行签名的方法。如果您使用此处未记录的任何其他签名机制（例如 EV（扩展验证）证书），请查看您的证书颁发机构文档并参考自定义签名命令部分。

## OV 证书

名为“OV 证书”的章节

危险

本指南仅适用于 2023 年 6 月 1 日之前获取的 OV 代码签名证书！对于使用 EV 证书和在该日期之后收到的 OV 证书进行代码签名，请查阅您证书颁发机构的文档。

注意

如果您使用 EV 证书对应用程序进行签名，它将立即获得 Microsoft SmartScreen 的信誉，并且不会向用户显示任何警告。

如果您选择 OV 证书（通常更便宜且个人可用），Microsoft SmartScreen 在用户下载应用程序时仍会显示警告。您的证书可能需要一些时间才能建立足够的信誉。您可以选择向 Microsoft [提交您的应用程序](https://www.microsoft.com/en-us/wdsi/filesubmission/)进行人工审核。尽管不作保证，但如果应用程序不包含任何恶意代码，Microsoft 可能会授予额外信誉，并可能删除该特定上传文件的警告。

请参阅此[对比说明](https://www.digicert.com/difference-between-dv-ov-and-ev-ssl-certificates)，以了解更多关于 OV 和 EV 证书的区别。

### 先决条件

标题为“前提条件”的章节

  * Windows - 您可能可以使用其他平台，但本教程使用 Powershell 原生功能。
  * 一个可正常运行的 Tauri 应用程序
  * 代码签名证书 - 您可以在 [Microsoft 文档](https://learn.microsoft.com/en-us/windows-hardware/drivers/dashboard/code-signing-cert-manage)中列出的服务商处获取此类证书。非 EV 证书可能还有除列表之外的其他颁发机构，请自行进行比较并自担风险进行选择。
    * 请确保获取的是**代码签名** 证书，SSL 证书无效！

### 入门指南

名为“入门指南”的章节

为了让 Windows 做好代码签名的准备，我们需要执行一些步骤。这包括将证书转换为特定格式、安装该证书，以及解码证书中所需的信息。

  1. #### 将您的 `.cer` 转换为 `.pfx`

名为“将您的 .cer 转换为 .pfx”的章节

     * 您将需要以下内容

       * 证书文件（我的是 `cert.cer`）
       * 私钥文件（我的是 `private-key.key`）
     * 打开命令提示符，并使用 `cd Documents/Certs` 切换到您的当前目录

     * 使用 `openssl pkcs12 -export -in cert.cer -inkey private-key.key -out certificate.pfx` 将您的 `.cer` 转换为 `.pfx`

     * 系统会提示您输入导出密码，**千万不要忘记它！**

  2. #### 将您的 `.pfx` 文件导入密钥库。

名为“将您的 .pfx 文件导入密钥库。”的章节

     * 现在我们需要导入我们的 `.pfx` 文件。

     * 使用 `$WINDOWS_PFX_PASSWORD = 'MYPASSWORD'` 将您的导出密码分配给变量

     * 现在使用 `Import-PfxCertificate -FilePath certificate.pfx -CertStoreLocation Cert:\CurrentUser\My -Password (ConvertTo-SecureString -String $WINDOWS_PFX_PASSWORD -Force -AsPlainText)` 导入证书

  3. #### 准备变量

名为“准备变量”的章节

     * 开始 ➡️ `certmgr.msc` 以打开个人证书管理，然后打开“个人/证书”。

     * 找到我们刚刚导入的证书并双击它，然后点击“详细信息”选项卡。

     * 签名哈希算法将成为我们的 `digestAlgorithm`。（提示：这通常是 `sha256`）

     * 向下滚动到指纹 (Thumbprint)。应该有一个类似 `A1B1A2B2A3B3A4B4A5B5A6B6A7B7A8B8A9B9A0B0` 的值。这就是我们的 `certificateThumbprint`。

     * 我们还需要一个时间戳 URL；这是一个用于验证证书签名时间的时间服务器。我使用的是 `http://timestamp.comodoca.com`，但为您提供证书的机构可能也有一个。

### 准备 `tauri.conf.json` 文件

名为“准备 tauri.conf.json 文件”的章节

  1. 现在我们有了 `certificateThumbprint`、`digestAlgorithm` 和 `timestampUrl`，我们将打开 `tauri.conf.json`。

  2. 在 `tauri.conf.json` 中，您需要查找 `tauri` -> `bundle` -> `windows` 部分。这里有三个变量用于存储我们获取的信息。请按照以下方式填写。

    "windows": {

            "certificateThumbprint": "A1B1A2B2A3B3A4B4A5B5A6B6A7B7A8B8A9B9A0B0",

            "digestAlgorithm": "sha256",

            "timestampUrl": "http://timestamp.comodoca.com"

    }

  3. 保存并运行 `tauri build`

  4. 在控制台输出中，您应该看到以下内容。

    info: signing app

    info: running signtool "C:\\Program Files (x86)\\Windows Kits\\10\\bin\\10.0.19041.0\\x64\\signtool.exe"

    info: "Done Adding Additional Store\r\nSuccessfully signed: APPLICATION FILE PATH HERE

这表明您已成功对 `.exe` 进行了签名。

就是这样！您已经成功设置了 Tauri 应用程序的 Windows 签名。

### 使用 GitHub Actions 对应用程序进行签名。

名为“使用 GitHub Actions 对应用程序进行签名。”的章节

我们还可以创建一个工作流来使用 GitHub Actions 对应用程序进行签名。

#### GitHub Secrets

名为“GitHub Secrets”的章节

我们需要为 GitHub Action 的正确配置添加一些 GitHub Secrets。这些密钥名称可以根据您的喜好命名。

  * 您可以查看关于如何添加 GitHub Secrets 的[加密机密 (encrypted secrets)](https://githubdocs.cn/en/actions/reference/encrypted-secrets) 指南。

我们使用的密钥如下

GitHub Secrets| 变量的值
---|---
WINDOWS_CERTIFICATE| Base64 编码版本的 .pfx 证书，可以使用此命令完成：`certutil -encode certificate.pfx base64cert.txt`
WINDOWS_CERTIFICATE_PASSWORD| 创建 .pfx 证书时使用的导出密码

#### 工作流修改

名为“工作流修改”的章节

  1. 我们需要在工作流中添加一个步骤，将证书导入 Windows 环境。此工作流完成了以下操作：

     1. 将 GitHub Secrets 分配给环境变量
     2. 创建一个新的 `certificate` 目录
     3. 将 `WINDOWS_CERTIFICATE` 导入到 tempCert.txt
     4. 使用 `certutil` 将 tempCert.txt 从 base64 解码为 `.pfx` 文件。
     5. 删除 tempCert.txt
     6. 将 `.pfx` 文件导入 Windows 的证书存储区，并将 `WINDOWS_CERTIFICATE_PASSWORD` 转换为安全字符串，以便在导入命令中使用。
  2. 我们将使用 [`tauri-action` 发布模板](https://github.com/tauri-apps/tauri-action)。

    name: 'publish'

    on:

      push:

        branches:

          - release

    jobs:

      publish-tauri:

        strategy:

          fail-fast: false

          matrix:

            platform: [macos-latest, ubuntu-latest, windows-latest]

        runs-on: ${{ matrix.platform }}

        steps:

          - uses: actions/checkout@v2

          - name: setup node

            uses: actions/setup-node@v1

            with:

              node-version: 12

          - name: install Rust stable

            uses: actions-rs/toolchain@v1

            with:

              toolchain: stable

          - name: install webkit2gtk (ubuntu only)

            if: matrix.platform == 'ubuntu-latest'

            run: |

              sudo apt-get update

              sudo apt-get install -y webkit2gtk-4.0

          - name: install app dependencies and build it

            run: yarn && yarn build

          - uses: tauri-apps/tauri-action@v0

            env:

              GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}

            with:

              tagName: app-v__VERSION__ # the action automatically replaces \_\_VERSION\_\_ with the app version

              releaseName: 'App v__VERSION__'

              releaseBody: 'See the assets to download this version and install.'

              releaseDraft: true

              prerelease: false

  3. 在 `-name: install app dependencies and build it` 之前，您需要添加以下步骤

    - name: import windows certificate

      if: matrix.platform == 'windows-latest'

      env:

        WINDOWS_CERTIFICATE: ${{ secrets.WINDOWS_CERTIFICATE }}

        WINDOWS_CERTIFICATE_PASSWORD: ${{ secrets.WINDOWS_CERTIFICATE_PASSWORD }}

      run: |

        New-Item -ItemType directory -Path certificate

        Set-Content -Path certificate/tempCert.txt -Value $env:WINDOWS_CERTIFICATE

        certutil -decode certificate/tempCert.txt certificate/certificate.pfx

        Remove-Item -path certificate -include tempCert.txt

        Import-PfxCertificate -FilePath certificate/certificate.pfx -CertStoreLocation Cert:\CurrentUser\My -Password (ConvertTo-SecureString -String $env:WINDOWS_CERTIFICATE_PASSWORD -Force -AsPlainText)

  4. 保存并推送到您的仓库。

  5. 您的工作流现在可以导入您的 Windows 证书并将其导入到 GitHub Runner 中，从而实现自动化代码签名！

## Azure Key Vault

名为“Azure Key Vault”的章节

您可以通过提供 Azure Key Vault 证书和凭据来对 Windows 可执行文件进行签名。

注意

本指南使用 [relic](https://github.com/sassoftware/relic)，因为它支持基于密钥的身份验证，不过如果您愿意，也可以配置其他工具。要下载 relic，请访问其[发布页面](https://github.com/sassoftware/relic/releases/)或运行 `go install github.com/sassoftware/relic/v8@latest`。

  1. Key Vault

在 [Azure 门户](https://portal.azure.com)中，导航到 [Key Vault 服务](https://portal.azure.com/#browse/Microsoft.KeyVault%2Fvaults)，点击“创建”按钮来创建一个新的 Key Vault。请记住“Key vault 名称”，因为您在配置证书 URL 时需要该信息。

  2. 证书

创建 Key Vault 后，选择它并进入“对象 > 证书”页面以创建新证书，然后点击“生成/导入”按钮。请记住“证书名称”，因为在配置证书 URL 时需要该信息。

  3. Tauri 配置

[relic](https://github.com/sassoftware/relic) 使用配置文件来确定应使用哪个签名密钥。对于 Azure Key Vault，您还需要证书 URL。在 `src-tauri` 文件夹中创建一个 `relic.conf` 文件，并配置 relic 以使用您的证书

src-tauri/relic.conf

    tokens:

      azure:

        type: azure

    keys:

      azure:

        token: azure

        id: https://\<KEY_VAULT_NAME\>.vault.azure.net/certificates/\<CERTIFICATE_NAME\>

注意：您必须将 <KEY_VAULT_NAME> 和 <CERTIFICATE_NAME> 替换为之前步骤中的相应名称。

要配置 Tauri 使用您的 Azure Key Vault 配置进行签名，请更改 [bundle > windows > signCommand](/reference/config/#signcommand) 配置值

tauri.conf.json

    {

      "bundle": {

        "windows": {

          "signCommand": "relic sign --file %1 --key azure --config relic.conf"

        }

      }

    }

  4. 凭据

[relic](https://github.com/sassoftware/relic) 必须对 Azure 进行身份验证才能加载证书。在 Azure 门户登录页面中，转到“Microsoft Entra ID”服务，然后前往“管理 > 应用注册”页面。点击“新注册”以创建新应用。创建应用后，您将被重定向到应用详细信息页面，在那里可以看到“应用程序 (客户端) ID”和“目录 (租户) ID”的值。分别将这些 ID 设置为 `AZURE_CLIENT_ID` 和 `AZURE_TENANT_ID` 环境变量。

在“管理 > 证书与机密”页面中，点击“新客户端机密”按钮，并将“值”列中的文本设置为 `AZURE_CLIENT_SECRET` 环境变量。

设置好所有凭据后，返回您的 Key Vault 页面并导航到“访问控制 (IAM)”页面。您必须将“Key Vault 证书用户”和“Key Vault 加密用户”角色分配给您刚刚创建的应用程序。

设置好所有这些变量后，运行 `tauri build` 将会生成已签名的 Windows 安装程序！

## 自定义签名命令

名为“自定义签名命令”的章节

在上面的 Azure Key Vault 文档中，我们使用了强大的 Tauri Windows 签名配置来强制 Tauri CLI 使用特殊的 shell 命令来对 Windows 安装程序可执行文件进行签名。[bundle > windows > signCommand](/reference/config/#signcommand) 配置选项可用于任何能够对 Windows 可执行文件进行签名的代码签名工具。

提示

当从 Linux 和 macOS 机器交叉编译 Windows 安装程序时，您**必须** 使用自定义签名命令，因为默认实现仅适用于 Windows 机器。

## Azure 代码签名

名为“Azure 代码签名”的章节

您可以通过提供 Azure 代码签名证书和凭据来对 Windows 可执行文件进行签名。如果您还没有 Azure 代码签名账户，可以按照此[教程](https://melatonin.dev/blog/code-signing-on-windows-with-azure-trusted-signing/)操作。

### 先决条件

标题为“前提条件”的章节

如果您想通过 Github Actions 进行签名，应该已经安装好了一切。

  1. [受信任的签名账户](https://learn.microsoft.com/en-us/azure/trusted-signing/quickstart?tabs=registerrp-portal,account-portal,certificateprofile-portal,deleteresources-portal)及已配置的权限
  2. [.NET](https://dotnet.microsoft.com/en-us/download/dotnet/8.0)（建议使用 .NET 6 或更高版本）
  3. [Azure CLI](https://learn.microsoft.com/en-us/cli/azure/install-azure-cli-windows?tabs=azure-cli#install-or-update)
  4. [Signtool](https://learn.microsoft.com/en-us/dotnet/framework/tools/signtool-exe)（建议使用 Windows 11 SDK 10.0.22000.0 或更高版本）

### 入门指南

名为“入门指南”的章节

您需要安装 [trusted-signing-cli](https://github.com/Levminer/trusted-signing-cli) 并配置您的环境变量。

  1. #### 安装 trusted-signing-cli

名为“安装 trusted-signing-cli”的章节

     * `cargo install trusted-signing-cli`
  2. #### 配置环境变量

名为“配置环境变量”的章节

     * trusted-signing-cli 需要设置以下环境变量，别忘了将这些添加到 Github Actions 的 [Secrets](https://githubdocs.cn/en/actions/security-for-github-actions/security-guides/using-secrets-in-github-actions) 中
       * `AZURE_CLIENT_ID`：您的 [应用注册](https://melatonin.dev/blog/code-signing-on-windows-with-azure-trusted-signing/#step-4-create-app-registration-user-credentials) 的客户端 ID
       * `AZURE_CLIENT_SECRET`：[应用注册](https://melatonin.dev/blog/code-signing-on-windows-with-azure-trusted-signing/#step-4-create-app-registration-user-credentials) 的客户端机密
       * `AZURE_TENANT_ID`：您的 Azure 目录的租户 ID，您也可以从您的 [应用注册](https://melatonin.dev/blog/code-signing-on-windows-with-azure-trusted-signing/#step-4-create-app-registration-user-credentials) 中获取
  3. ### 修改您的 `tauri.conf.json` 文件

名为“修改您的 tauri.conf.json 文件”的章节

     * 您可以修改 `tauri.conf.json`，也可以为 Windows 创建特定的配置文件。请使用您自己的值替换 URL 和证书名称。
       * -e：您的 Azure 代码签名账户的端点
       * -a：您的 Azure 代码签名账户的名称
       * -c：您的 Azure 代码签名账户内的证书配置文件名称
       * -d：签名内容的描述（可选）。当对 .msi 安装程序进行签名时，此描述将作为 UAC 提示符中安装程序的名称显示，如果未设置，则会显示为一串随机字符。

tauri.conf.json

    {

      "bundle": {

        "windows": {

          "signCommand": "trusted-signing-cli -e https://wus2.codesigning.azure.net -a MyAccount -c MyProfile -d MyApp %1"

        }

      }

    }