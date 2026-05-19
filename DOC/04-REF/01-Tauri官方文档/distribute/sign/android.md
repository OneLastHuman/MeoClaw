# Android 代码签名

_Source: https://v2.tauri.org.cn/distribute/sign/android/_

要在 Play 商店发布应用，您需要使用数字证书对应用进行签名。

Android App Bundle (AAB) 和 APK 在上传分发前必须进行签名。

Google 还为分发到 Play 商店的 Android App Bundle 提供了一种额外的签名机制。更多信息请参阅[官方 Play App 签名文档](https://support.google.com/googleplay/android-developer/answer/9842756?hl=en&visit_id=638549803861403647-3347771264&rd=1)。

## 创建密钥库和上传密钥

标题为“创建密钥库和上传密钥”的章节

Android 签名需要一个 Java 密钥库 (Keystore) 文件，该文件可以使用官方的 `keytool` CLI 生成

  * macOS/Linux
  * Windows

    keytool -genkey -v -keystore ~/upload-keystore.jks -keyalg RSA -keysize 2048 -validity 10000 -alias upload

    keytool -genkey -v -keystore $env:USERPROFILE\upload-keystore.jks -storetype JKS -keyalg RSA -keysize 2048 -validity 10000 -alias upload

此命令将 `upload-keystore.jks` 文件存储在您的主目录中。如果您希望将其存储在其他位置，请更改传递给 `-keystore` 参数的路径。

提示

  * `keytool` 命令可能不在您的 PATH 环境变量中。您可以在 Android Studio 自带的 JDK 中找到它。

  * Linux
  * macOS
  * Windows

终端窗口

    /opt/android-studio/jbr/bin/keytool ...args

**Android Studio 的目录路径取决于您的 Linux 发行版。**

终端窗口

    /Applications/Android\ Studio.app/Contents/jbr/Contents/Home/bin/keytool ...args

终端窗口

    C:\\Program Files\\Android\\Android Studio\\jbr\\bin\\keytool.exe ...args

安全警告

请务必将 `keystore` 文件保密；不要将其提交到公共版本控制系统中！

更多信息请参见[官方文档](https://developer.android.com.cn/studio/publish/app-signing#generate-key)。

## 配置签名密钥

标题为“配置签名密钥”的章节

创建一个名为 `[project]/src-tauri/gen/android/keystore.properties` 的文件，其中包含对您的密钥库的引用

    password=<password defined when keytool was executed>

    keyAlias=upload

    storeFile=<location of the key store file, such as /Users/<user name>/upload-keystore.jks or C:\\Users\\<user name>\\upload-keystore.jks>

安全警告

请务必将 `keystore.properties` 文件保密；不要将其提交到公共版本控制系统中。

通常您会在 CI/CD 平台中生成此文件。以下片段包含了 GitHub Actions 的作业步骤示例

    - name: setup Android signing

      run: |

        cd src-tauri/gen/android

        echo "keyAlias=${{ secrets.ANDROID_KEY_ALIAS }}" > keystore.properties

        echo "password=${{ secrets.ANDROID_KEY_PASSWORD }}" >> keystore.properties

        base64 -d <<< "${{ secrets.ANDROID_KEY_BASE64 }}" > $RUNNER_TEMP/keystore.jks

        echo "storeFile=$RUNNER_TEMP/keystore.jks" >> keystore.properties

在此示例中，密钥库通过 `base64 -i /path/to/keystore.jks` 导出为 base64，并设置为 `ANDROID_KEY_BASE64` 密钥。

### 配置 Gradle 以使用签名密钥

标题为“配置 Gradle 以使用签名密钥”的章节

通过编辑 `[project]/src-tauri/gen/android/app/build.gradle.kts` 文件，配置 Gradle 在 release 模式下构建应用时使用您的上传密钥。

提示

典型的 Android 项目中存在多个不同的 `build.gradle.kts` 文件。如果没有 `buildTypes` 代码块，说明您打开的文件不对。您需要的文件位于相对于前一步骤中的密钥库文件所在的 `app/` 目录中。

点击此处查看显示其在典型文件树中位置的截图。

![build.gradle.kts location in file tree](/_astro/build-gradle-kts-filetree.PZAoMU-e_2mgYr6.webp)

  1. 在文件开头添加必要的导入

         import java.io.FileInputStream

  2. 在 `buildTypes` 代码块之前添加 `release` 签名配置

         signingConfigs {

             create("release") {

                 val keystorePropertiesFile = rootProject.file("keystore.properties")

                 val keystoreProperties = Properties()

                 if (keystorePropertiesFile.exists()) {

                     keystoreProperties.load(FileInputStream(keystorePropertiesFile))

                 }

                 keyAlias = keystoreProperties["keyAlias"] as String

                 keyPassword = keystoreProperties["password"] as String

                 storeFile = file(keystoreProperties["storeFile"] as String)

                 storePassword = keystoreProperties["password"] as String

             }

         }

         buildTypes {

             ...

         }

  3. 在 `buildTypes` 代码块的 `release` 配置中使用新的 `release` 签名配置

         buildTypes {

             getByName("release") {

                 signingConfig = signingConfigs.getByName("release")

             }

         }

现在，您的应用在发布版本构建时将自动进行签名。