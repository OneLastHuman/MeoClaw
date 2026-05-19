# Linux 代码签名

_Source: https://v2.tauri.org.cn/distribute/sign/linux/_

本指南提供了有关 Linux 软件包代码签名的信息。虽然在 Linux 上部署应用程序并不强制要求对制品进行签名，但签名可以增加用户对所部署应用程序的信任。对二进制文件进行签名可以让最终用户验证这些文件是真实的，且未被其他未受信任的实体修改过。

## AppImage 签名

章节标题 “AppImage 签名”

AppImage 可以使用 gpg 或 gpg2 进行签名。

### 先决条件

标题为“前提条件”的章节

必须准备一个用于签名的密钥。可以使用以下命令生成新密钥：

终端窗口

    gpg2 --full-gen-key

有关更多信息，请参考 gpg 或 gpg2 的文档。你应该特别注意将私钥和公钥备份到安全的位置。

### 签名

名为“签名”的章节

你可以通过设置以下环境变量在 AppImage 中嵌入签名：

  * **SIGN** : 设置为 `1` 以对 AppImage 进行签名。
  * **SIGN_KEY** : 可选变量，用于指定签名所用的特定 GPG 密钥 ID。
  * **APPIMAGETOOL_SIGN_PASSPHRASE** : 签名密钥的密码。如果未设置，gpg 将显示一个对话框供你输入。在 CI/CD 平台构建时必须设置此项。
  * **APPIMAGETOOL_FORCE_SIGN** : 默认情况下，即使签名失败，AppImage 仍会生成。若要签名失败时退出构建，可以将此变量设置为 `1`。

你可以通过运行以下命令来显示嵌入在 AppImage 中的签名：

终端窗口

    ./src-tauri/target/release/bundle/appimage/$APPNAME_$VERSION_amd64.AppImage --appimage-signature

请注意，你需要根据你的配置将 $APPNAME 和 $VERSION 的值替换为正确的值。

注意

**签名未经校验**

AppImage 本身不会校验签名，因此你不能仅仅依赖它来检查文件是否被篡改。用户必须使用 AppImage 校验工具手动验证签名。这要求你在经过身份验证的渠道（例如通过 TLS 服务的网站）上发布你的密钥 ID，以便最终用户查看和验证。

有关更多信息，请参阅 [AppImage 官方文档](https://docs.appimage.org/packaging-guide/optional/signatures.html)。

### 验证签名

章节标题 “验证签名”

AppImage 校验工具可从[此处](https://github.com/AppImageCommunity/AppImageUpdate/releases/tag/continuous)下载。选择其中一个 `validate-$PLATFORM.AppImage` 文件。

运行以下命令来验证签名：

终端窗口

    chmod +x validate-$PLATFORM.AppImage

    ./validate-$PLATFORM.AppImage $TAURI_OUTPUT.AppImage

如果签名有效，输出将是：

    Validation result: validation successful

    Signatures found with key fingerprints: $KEY_ID

    ====================

    Validator report:

    Signature checked for key with fingerprint $KEY_ID:

    Validation successful