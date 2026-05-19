# 配置

_Source: https://v2.tauri.org.cn/reference/config/_

Tauri 配置对象。它从一个文件中读取，您可以在其中定义前端资源、配置打包程序并定义托盘图标。

配置文件由 [`tauri init`](https://v2.tauri.org.cn/reference/cli/#init) 命令生成，该文件位于您的 Tauri 应用程序源目录 (src-tauri) 中。

生成后，您可以随意修改它以自定义您的 Tauri 应用程序。

### 文件格式

名为“文件格式”的章节

默认情况下，配置文件定义为一个名为 `tauri.conf.json` 的 JSON 文件。

Tauri 还分别通过 `config-json5` 和 `config-toml` Cargo 功能支持 JSON5 和 TOML 文件。JSON5 文件名必须是 `tauri.conf.json` 或 `tauri.conf.json5`。TOML 文件名为 `Tauri.toml`。

### 平台特定配置

名为“平台特定配置”的章节

除了默认配置文件外，Tauri 还可以从 `tauri.linux.conf.json`、`tauri.windows.conf.json`、`tauri.macos.conf.json`、`tauri.android.conf.json` 和 `tauri.ios.conf.json`（或者如果使用 `Tauri.toml` 格式，则为 `Tauri.linux.toml`、`Tauri.windows.toml`、`Tauri.macos.toml`、`Tauri.android.toml` 和 `Tauri.ios.toml`）读取特定于平台的配置，这些配置将与主配置对象合并。

### 配置结构

名为“配置结构”的章节

配置由以下对象组成

  * `app`：Tauri 配置
  * `build`：构建配置
  * `bundle`：打包配置
  * `plugins`：插件配置

示例 tauri.config.json 文件

    {

      "productName": "tauri-app",

      "version": "0.1.0",

      "build": {

        "beforeBuildCommand": "",

        "beforeDevCommand": "",

        "devUrl": "https://:3000",

        "frontendDist": "../dist"

      },

      "app": {

        "security": {

          "csp": null

        },

        "windows": [

          {

            "fullscreen": false,

            "height": 600,

            "resizable": true,

            "title": "Tauri App",

            "width": 800

          }

        ]

      },

      "bundle": {},

      "plugins": {}

    }

**对象属性** :

  * app
  * build
  * bundle
  * 标识符 (必需)
  * mainBinaryName
  * plugins
  * productName
  * version

### app

名为“app”的章节

`AppConfig`

应用配置。

默认

    {

      "enableGTKAppId": false,

      "macOSPrivateApi": false,

      "security": {

        "assetProtocol": {

          "enable": false,

          "scope": []

        },

        "capabilities": [],

        "dangerousDisableAssetCspModification": false,

        "freezePrototype": false,

        "pattern": {

          "use": "brownfield"

        }

      },

      "windows": [],

      "withGlobalTauri": false

    }

### build

名为“build”的章节

`BuildConfig`

构建配置。

默认

    {

      "additionalWatchFolders": [],

      "removeUnusedCommands": false

    }

### bundle

名为“bundle”的章节

`BundleConfig`

打包程序配置。

默认

    {

      "active": false,

      "android": {

        "autoIncrementVersionCode": false,

        "minSdkVersion": 24

      },

      "createUpdaterArtifacts": false,

      "iOS": {

        "minimumSystemVersion": "14.0"

      },

      "icon": [],

      "linux": {

        "appimage": {

          "bundleMediaFramework": false,

          "files": {}

        },

        "deb": {

          "files": {}

        },

        "rpm": {

          "epoch": 0,

          "files": {},

          "release": "1"

        }

      },

      "macOS": {

        "dmg": {

          "appPosition": {

            "x": 180,

            "y": 170

          },

          "applicationFolderPosition": {

            "x": 480,

            "y": 170

          },

          "windowSize": {

            "height": 400,

            "width": 660

          }

        },

        "files": {},

        "hardenedRuntime": true,

        "minimumSystemVersion": "10.13"

      },

      "targets": "all",

      "useLocalToolsDir": false,

      "windows": {

        "allowDowngrades": true,

        "certificateThumbprint": null,

        "digestAlgorithm": null,

        "nsis": null,

        "signCommand": null,

        "timestampUrl": null,

        "tsp": false,

        "webviewInstallMode": {

          "silent": true,

          "type": "downloadBootstrapper"

        },

        "wix": null

      }

    }

### identifier

名为“identifier”的章节

`string`

使用反向域名表示法的应用程序标识符（例如 `com.tauri.example`）。此字符串在所有应用程序中必须是唯一的，因为它用于系统配置（如 bundle ID 和指向 WebView 数据目录的路径）。此字符串只能包含字母数字字符（A-Z、a-z 和 0-9）、连字符 (-) 和句点 (.)。

### mainBinaryName

名为“mainBinaryName”的章节

`string` | `null`

覆盖应用程序的主二进制文件名。

默认情况下，Tauri 使用来自 `cargo` 的输出二进制文件，通过设置此项，我们将在 `tauri-cli` 的 `tauri build` 命令中重命名该二进制文件，并让 `tauri bundle` 指向它。

如果可能，请改用 [`package name`](https://doc.rust-lang.net.cn/cargo/reference/manifest.html#the-name-field) 或设置 [`name 字段`](https://doc.rust-lang.net.cn/cargo/reference/cargo-targets.html#the-name-field)，如果这还不够，并且您正在使用 nightly 版本，请考虑改用 [`different-binary-name`](https://doc.rust-lang.net.cn/nightly/cargo/reference/unstable.html#different-binary-name) 功能。

注意：此配置不应包含二进制扩展名（例如 `.exe`），我们会为您添加。

### plugins

名为“plugins”的章节

`PluginConfig`

插件配置。

**默认** : `{}`

### productName

名为“productName”的章节

`string` | `null`，模式为 `^[^/\:*?"<>|]+$`

应用名称。

### version

名为“version”的章节

`string` | `null`

应用版本。它是一个 semver 版本号或指向包含 `version` 字段的 `package.json` 文件的路径。

如果移除，则使用 `Cargo.toml` 中的版本号。建议在 Tauri 配置中管理应用版本控制。

#### 平台特定

名为“平台特定”的章节

  * **macOS** ：转换为 bundle 的 CFBundleShortVersionString 属性，并用作默认的 CFBundleVersion。您可以使用 [`bundle &gt; macOS &gt; bundleVersion`](MacConfig::bundle_version) 设置特定的 bundle 版本。
  * **iOS** ：转换为 bundle 的 CFBundleShortVersionString 属性，并用作默认的 CFBundleVersion。您可以使用 [`bundle &gt; iOS &gt; bundleVersion`](IosConfig::bundle_version) 设置特定的 bundle 版本。`tauri ios build` CLI 命令有一个 `--build-number &lt;number&gt;` 选项，允许您将构建编号附加到应用版本。
  * **Android** ：默认使用 1.0 版本。您可以使用 [`bundle &gt; android &gt; versionCode`](AndroidConfig::version_code) 设置版本代码。

默认情况下，Android 使用 1.0 版本。

## 定义

名为“定义”的章节

### AndroidConfig

名为“AndroidConfig”的章节

Android 目标平台的常规配置。

**对象属性** :

  * autoIncrementVersionCode
  * minSdkVersion
  * versionCode

##### autoIncrementVersionCode

名为“autoIncrementVersionCode”的章节

`布尔值 (boolean)`

是否在每次构建时自动递增 `versionCode`。

  * 如果为 `true`，生成器将尝试从 `tauri.properties` 读取上一个 `versionCode`，并在每次构建时将其递增 1。
  * 如果为 `false` 或未设置，它将回退到 `version_code` 或基于 semver 的逻辑。

请注意，要使用此功能，您应该从 `src-tauri/gen/android/app/.gitignore` 中删除 `/tauri.properties`，以便将当前的 versionCode 提交到存储库中。

##### minSdkVersion

名为“minSdkVersion”的章节

`integer`，格式为 `uint32`

应用程序运行所需的最低 API 级别。如果系统的 API 级别低于指定值，Android 系统将阻止用户安装该应用程序。

**默认** : `24`

##### versionCode

名为“versionCode”的章节

`integer` | `null`，最大值为 `2100000000`，最小值为 `1`，格式为 `uint32`

应用程序的版本代码。根据 Google Play 商店要求，其上限为 2,100,000,000。

默认情况下，我们使用您配置的版本并进行以下计算：versionCode = version.major * 1000000 + version.minor * 1000 + version.patch

### AppConfig

名为“AppConfig”的章节

App 配置对象。

查看更多：<<https://v2.tauri.org.cn/reference/config/#appconfig>>

**对象属性** :

  * enableGTKAppId
  * macOSPrivateApi
  * security
  * trayIcon
  * windows
  * withGlobalTauri

##### enableGTKAppId

名为“enableGTKAppId”的章节

`布尔值 (boolean)`

如果设置为 true，“identifier”将被设置为 GTK 应用 ID（在使用 GTK 的系统上）。

##### macOSPrivateApi

名为“macOSPrivateApi”的章节

`布尔值 (boolean)`

MacOS 私有 API 配置。启用透明背景 API 并将 `fullScreenEnabled` 首选项设置为 `true`。

##### security

名为“security”的章节

`SecurityConfig`

安全配置。

默认

    {

      "assetProtocol": {

        "enable": false,

        "scope": []

      },

      "capabilities": [],

      "dangerousDisableAssetCspModification": false,

      "freezePrototype": false,

      "pattern": {

        "use": "brownfield"

      }

    }

##### trayIcon

名为“trayIcon”的章节

`TrayIconConfig` | `null`

应用托盘图标配置。

##### windows

名为“windows”的章节

`WindowConfig`[]

应用窗口配置。

###### 示例

名为“示例：”的章节

在应用启动时创建窗口

    {

      "app": {

        "windows": [

          { "width": 800, "height": 600 }

        ]

      }

    }

如果未指定，窗口的标签（其标识符）默认为“main”。您可以使用此标签通过 Rust 中的 `app.get_webview_window` 或 JavaScript 中的 `WebviewWindow.getByLabel` 获取窗口。

使用多个窗口时，每个窗口都需要一个唯一的标签。

    {

      "app": {

        "windows": [

          { "label": "main", "width": 800, "height": 600 },

          { "label": "secondary", "width": 800, "height": 600 }

        ]

      }

    }

您还可以将 `create` 设置为 false，并通过 Rust API 使用此配置。

    {

      "app": {

        "windows": [

          { "create": false, "width": 800, "height": 600 }

        ]

      }

    }

并像这样使用它

    tauri::Builder::default()

      .setup(|app| {

        tauri::WebviewWindowBuilder::from_config(app.handle(), &app.config().app.windows[0])?.build()?;

        Ok(())

      });

**默认** : `[]`

##### withGlobalTauri

名为“withGlobalTauri”的章节

`布尔值 (boolean)`

是否应在 `window.__TAURI__` 上注入 Tauri API。

### AppImageConfig

名为“AppImageConfig”的章节

AppImage 捆绑包配置。

查看更多：<<https://v2.tauri.org.cn/reference/config/#appimageconfig>>

**对象属性** :

  * bundleMediaFramework
  * files

##### bundleMediaFramework

名为“bundleMediaFramework”的章节

`布尔值 (boolean)`

包含音频和视频播放所需的额外 gstreamer 依赖项。根据您的构建系统，这会将捆绑包大小增加约 15-35MB。

##### files

名为“files”的章节

要包含在 AppImage 二进制文件中的文件。

**允许额外属性** : `string`

**默认** : `{}`

### AssetProtocolConfig

名为“AssetProtocolConfig”的章节

资产自定义协议配置。

查看更多：<<https://v2.tauri.org.cn/reference/config/#assetprotocolconfig>>

**对象属性** :

  * 启用
  * scope

##### 启用

名为“enable”的章节

`布尔值 (boolean)`

启用资产协议。

##### scope

名为“scope”的章节

`FsScope`

资产协议的访问范围。

**默认** : `[]`

### AssociationExt

名为“AssociationExt”的章节

`string`

`FileAssociation` 的扩展名。

开头的 `.` 会被自动去除。

### BackgroundThrottlingPolicy

名为“BackgroundThrottlingPolicy”的章节

**以下其中之一** :

  * `"disabled"`：禁用后台节流的策略。
  * `"suspend"`：当 WebView 不在窗口中时完全挂起任务的策略。如果未设置任何策略，这通常是默认行为。
  * `"throttle"`：当 WebView 不在窗口中时限制处理，但不会完全挂起任务的策略。

后台节流策略。

### BeforeDevCommand

名为“BeforeDevCommand”的章节

**以下任何一种** :

  * `string`：使用默认选项运行给定的脚本。
  * 使用自定义选项运行给定的脚本。**对象属性** ：- cwd - script（必需）- wait ##### cwd `string` | `null` 当前工作目录。 ##### script `string` 要执行的脚本。 ##### wait `boolean` `tauri dev` 是否应等待命令完成。默认为 `false`。

描述在 `tauri dev` 之前运行的 Shell 命令。

### BuildConfig

名为“BuildConfig”的章节

构建配置对象。

查看更多：<<https://v2.tauri.org.cn/reference/config/#buildconfig>>

**对象属性** :

  * additionalWatchFolders
  * beforeBuildCommand
  * beforeBundleCommand
  * beforeDevCommand
  * devUrl
  * features
  * frontendDist
  * removeUnusedCommands
  * runner

##### additionalWatchFolders

名为“additionalWatchFolders”的章节

`string`[]

运行 `tauri dev` 时要监视更改的其他路径。

**默认** : `[]`

##### beforeBuildCommand

名为“beforeBuildCommand”的章节

`HookCommand` | `null`

在 `tauri build` 开始之前运行的 Shell 命令。

如果您执行条件编译，则会设置 TAURI_ENV_PLATFORM、TAURI_ENV_ARCH、TAURI_ENV_FAMILY、TAURI_ENV_PLATFORM_VERSION、TAURI_ENV_PLATFORM_TYPE 和 TAURI_ENV_DEBUG 环境变量。

##### beforeBundleCommand

名为“beforeBundleCommand”的章节

`HookCommand` | `null`

在 `tauri build` 中的打包阶段开始之前运行的 Shell 命令。

如果您执行条件编译，则会设置 TAURI_ENV_PLATFORM、TAURI_ENV_ARCH、TAURI_ENV_FAMILY、TAURI_ENV_PLATFORM_VERSION、TAURI_ENV_PLATFORM_TYPE 和 TAURI_ENV_DEBUG 环境变量。

##### beforeDevCommand

名为“beforeDevCommand”的章节

`BeforeDevCommand` | `null`

在 `tauri dev` 开始之前运行的 Shell 命令。

如果您执行条件编译，则会设置 TAURI_ENV_PLATFORM、TAURI_ENV_ARCH、TAURI_ENV_FAMILY、TAURI_ENV_PLATFORM_VERSION、TAURI_ENV_PLATFORM_TYPE 和 TAURI_ENV_DEBUG 环境变量。

##### devUrl

名为“devUrl”的章节

`string` | `null`，格式为 `uri`

在开发中加载的 URL。

这通常是开发服务器的 URL，它通过热重载和 HMR 服务于您的应用程序资源。大多数现代 JavaScript 打包程序（如 [Vite](https://vite.org.cn/guide/)）都提供了一种默认启动开发服务器的方法。

如果您没有开发服务器或不想使用它，请忽略此选项，改用 [`frontendDist`](BuildConfig::frontend_dist) 并指向 Web 资源目录，Tauri CLI 将运行其内置的开发服务器并提供简单的热重载体验。

##### features

名为“features”的章节

`string`[] | `null`

传递给 `cargo` 命令的功能特性。

##### frontendDist

名为“frontendDist”的章节

`FrontendDist` | `null`

应用程序资源的路径（通常是您的 JavaScript 打包程序的 `dist` 文件夹），或者一个 URL，该 URL 可以是 Tauri 应用中注册的自定义协议（例如：`myprotocol://`），也可以是远程 URL（例如：`https://site.com/app`）。

当提供相对于配置文件的路径时，它会被递归读取，所有文件都会嵌入到应用程序二进制文件中。然后 Tauri 会寻找 `index.html` 并将其作为应用程序的默认入口点。

您还可以提供要嵌入的路径列表，这允许对添加到二进制文件中的文件进行细粒度控制。在这种情况下，所有文件都会被添加到根目录，您必须在 HTML 文件中以该方式引用它们。

当提供 URL 时，应用程序将不会有捆绑的资源，并且应用程序将默认加载该 URL。

##### removeUnusedCommands

名为“removeUnusedCommands”的章节

`布尔值 (boolean)`

尝试在 `tauri build` 期间根据 ACL 列表移除插件注册的未使用命令，其工作方式是 tauri-cli 将读取此内容并为构建脚本和宏设置环境变量，它们将尝试获取所有允许的命令并移除其余命令。

注意

  * 当您使用 `dynamic-acl`（目前默认启用）功能标志中的功能时，这不会考虑动态添加的 ACL，因此在使用时请务必检查它。
  * 此功能需要 tauri-plugin 2.1 和 tauri 2.4。

##### runner

名为“runner”的章节

`RunnerConfig` | `null`

用于构建和运行应用程序的二进制文件。

### BundleConfig

名为“BundleConfig”的章节

tauri-bundler 配置。

查看更多：<<https://v2.tauri.org.cn/reference/config/#bundleconfig>>

**对象属性** :

  * active
  * android
  * category
  * copyright
  * createUpdaterArtifacts
  * externalBin
  * fileAssociations
  * homepage
  * icon
  * iOS
  * license
  * licenseFile
  * linux
  * longDescription
  * macOS
  * publisher
  * resources
  * shortDescription
  * targets
  * useLocalToolsDir
  * windows

##### active

名为“active”的章节

`布尔值 (boolean)`

Tauri 是应该捆绑您的应用程序，还是仅输出可执行文件。

##### android

名为“android”的章节

`AndroidConfig`

Android 配置。

默认

    {

      "autoIncrementVersionCode": false,

      "minSdkVersion": 24

    }

##### category

名为“category”的章节

`string` | `null`

应用程序类别。

应为以下类别之一：Business, DeveloperTool, Education, Entertainment, Finance, Game, ActionGame, AdventureGame, ArcadeGame, BoardGame, CardGame, CasinoGame, DiceGame, EducationalGame, FamilyGame, KidsGame, MusicGame, PuzzleGame, RacingGame, RolePlayingGame, SimulationGame, SportsGame, StrategyGame, TriviaGame, WordGame, GraphicsAndDesign, HealthcareAndFitness, Lifestyle, Medical, Music, News, Photography, Productivity, Reference, SocialNetworking, Sports, Travel, Utility, Video, Weather。

##### copyright

名为“copyright”的章节

`string` | `null`

与您的应用程序关联的版权字符串。

##### createUpdaterArtifacts

名为“createUpdaterArtifacts”的章节

`更新器`

是否生成更新程序及其签名

##### externalBin

名为“externalBin”的章节

`string`[] | `null`

要随您的应用程序一起嵌入的二进制文件路径列表（可以是绝对路径或相对路径）。

请注意，Tauri 将按照 “binary-name{-target-triple}{.system-extension}” 的模式查找系统特定的二进制文件。

例如，对于外部二进制文件 “my-binary”，Tauri 会查找

  * Windows 下的 “my-binary-x86_64-pc-windows-msvc.exe”
  * macOS 下的 “my-binary-x86_64-apple-darwin”
  * Linux 下的 “my-binary-x86_64-unknown-linux-gnu”

所以请不要忘记为所有目标平台提供二进制文件。

##### fileAssociations

名为“fileAssociations”的章节

`FileAssociation`[] | `null`

要与应用程序关联的文件类型。

##### homepage

名为“homepage”的章节

`string` | `null`

应用程序主页的 URL。如果未设置，将回退到 `Cargo.toml` 中定义的 `homepage`。

支持的打包目标：`deb`, `rpm`, `nsis` 和 `msi`。

##### icon

名为“icon”的章节

`string`[]

应用图标

**默认** : `[]`

##### iOS

名为“iOS”的章节

`IosConfig`

iOS 配置。

默认

    {

      "minimumSystemVersion": "14.0"

    }

##### license

名为“license”的章节

`string` | `null`

要包含在适当捆绑包中的包许可证标识符。如果未设置，则默认为 Cargo.toml 文件中的许可证。

##### licenseFile

名为“licenseFile”的章节

`string` | `null`

要包含在适当捆绑包中的许可证文件路径。

##### linux

名为“linux”的章节

`LinuxConfig`

Linux 捆绑包配置。

默认

    {

      "appimage": {

        "bundleMediaFramework": false,

        "files": {}

      },

      "deb": {

        "files": {}

      },

      "rpm": {

        "epoch": 0,

        "files": {},

        "release": "1"

      }

    }

##### longDescription

名为“longDescription”的章节

`string` | `null`

应用程序的较长、多行描述。

##### macOS

名为“macOS”的章节

`MacConfig`

macOS 捆绑包配置。

默认

    {

      "dmg": {

        "appPosition": {

          "x": 180,

          "y": 170

        },

        "applicationFolderPosition": {

          "x": 480,

          "y": 170

        },

        "windowSize": {

          "height": 400,

          "width": 660

        }

      },

      "files": {},

      "hardenedRuntime": true,

      "minimumSystemVersion": "10.13"

    }

##### publisher

名为“publisher”的章节

`string` | `null`

应用程序的发布者。默认为标识符字符串中的第二个元素。

目前映射到 Windows 安装程序的 Manufacturer 属性，如果 Cargo.toml 没有 authors 字段，则映射到 debian 包的 Maintainer 字段。

##### resources

名为“resources”的章节

`BundleResources` | `null`

要捆绑的应用资源。每个资源都是文件或目录的路径。支持 Glob 模式。

###### 示例

名为“示例”的章节

包含文件列表

    {

      "bundle": {

        "resources": [

          "./path/to/some-file.txt",

          "/absolute/path/to/textfile.txt",

          "../relative/path/to/jsonfile.json",

          "some-folder/",

          "resources/**/*.md"

        ]

      }

    }

捆绑的文件将位于 `$RESOURCES/` 中，并保留原始目录结构，例如：`./path/to/some-file.txt` -> `$RESOURCE/path/to/some-file.txt`

要精细控制文件复制到的位置，请改用映射

    {

      "bundle": {

        "resources": {

          "/absolute/path/to/textfile.txt": "resources/textfile.txt",

          "relative/path/to/jsonfile.json": "resources/jsonfile.json",

          "resources/": "",

          "docs/**/*md": "website-docs/"

        }

      }

    }

请注意，在这种情况下使用 glob 模式时，不会保留原始目录结构，所有内容都会直接复制到目标目录中

查看更多：<<https://v2.tauri.org.cn/develop/resources/>>

##### shortDescription

名为“shortDescription”的章节

`string` | `null`

应用程序的简短描述。

##### targets

名为“targets”的章节

`BundleTarget`

捆绑目标，目前支持 [“deb”, “rpm”, “appimage”, “nsis”, “msi”, “app”, “dmg”] 或 “all”。

**默认** : `"all"`

##### useLocalToolsDir

名为“useLocalToolsDir”的章节

`布尔值 (boolean)`

在构建此应用程序时，是否使用项目的 `target` 目录来缓存构建工具（例如 Wix 和 NSIS）。默认为 `false`。

如果为 true，工具将缓存在 `target/.tauri/` 中。如果为 false，工具将缓存在当前用户的平台特定缓存目录中。

适合将此项设置为 `true` 的示例是在以 Windows 系统用户身份（例如 AWS EC2 工作负载）构建此应用程序时，因为 Windows 系统的 AppData 目录受到限制。

##### windows

名为“windows”的章节

`WindowsConfig`

Windows 捆绑包配置。

默认

    {

      "allowDowngrades": true,

      "certificateThumbprint": null,

      "digestAlgorithm": null,

      "nsis": null,

      "signCommand": null,

      "timestampUrl": null,

      "tsp": false,

      "webviewInstallMode": {

        "silent": true,

        "type": "downloadBootstrapper"

      },

      "wix": null

    }

### BundleResources

标题为“BundleResources”的章节

**以下任何一种** :

  * `string`[] 一个要包含的路径列表。
  * 源路径到目标路径的映射。**允许添加额外属性** ：`string`

捆绑资源（Bundle resources）的定义。既可以是一个要包含的路径列表，也可以是一个源路径到目标路径的映射。

### BundleTarget

标题为“BundleTarget”的章节

**以下任何一种** :

  * `"all"` 捆绑所有目标。
  * `BundleType`[] 捆绑目标列表。
  * `BundleType` 单个捆绑目标。

要捆绑的目标。每个值均不区分大小写。

### BundleType

标题为“BundleType”的章节

**以下其中之一** :

  * `"deb"` Debian 捆绑包 (.deb)。
  * `"rpm"` RPM 捆绑包 (.rpm)。
  * `"appimage"` AppImage 捆绑包 (.appimage)。
  * `"msi"` Microsoft 安装程序捆绑包 (.msi)。
  * `"nsis"` NSIS 捆绑包 (.exe)。
  * `"app"` macOS 应用程序捆绑包 (.app)。
  * `"dmg"` Apple 磁盘映像捆绑包 (.dmg)。

由 tauri-bundler 引用的捆绑包。

### BundleTypeRole

标题为“BundleTypeRole”的章节

**以下其中之一** :

  * `"Editor"` CFBundleTypeRole.Editor。文件可以被读取和编辑。
  * `"Viewer"` CFBundleTypeRole.Viewer。文件可以被读取。
  * `"Shell"` CFBundleTypeRole.Shell
  * `"QLGenerator"` CFBundleTypeRole.QLGenerator
  * `"None"` CFBundleTypeRole.None

仅限 macOS。对应 CFBundleTypeRole

### 功能

标题为“Capability”的章节

一种分组和边界机制，开发者可以使用它来隔离对 IPC 层的访问。

它控制应用程序窗口和 Webview 对 Tauri 核心、应用程序或插件命令的细粒度访问。如果 Webview 或其窗口不匹配任何能力（Capability），则它根本无法访问 IPC 层。

这样做可以根据窗口所需的系统访问权限对其进行分组，从而降低前端漏洞在权限较低窗口中的影响。可以通过精确名称（例如 `main-window`）或 glob 模式（如 `*` 或 `admin-*`）将窗口添加到某个能力中。一个窗口可以不关联、关联一个或关联多个能力。

##### 示例

标题为“Example”的章节

    {

      "identifier": "main-user-files-write",

      "description": "This capability allows the `main` window on macOS and Windows access to `filesystem` write related commands and `dialog` commands to enable programmatic access to files selected by the user.",

      "windows": [

        "main"

      ],

      "permissions": [

        "core:default",

        "dialog:open",

        {

          "identifier": "fs:allow-write-text-file",

          "allow": [{ "path": "$HOME/test.txt" }]

        },

      ],

      "platforms": ["macOS","windows"]

    }

**对象属性** :

  * description
  * 标识符 (必需)
  * local
  * permissions (必填)
  * platforms
  * remote
  * webviews
  * windows

##### description

标题为“description”的章节

`string`

此能力旨在允许关联窗口执行的操作的描述。

它应该包含对分组后的权限允许执行哪些操作的描述。

###### 示例

标题为“Example”的章节

此能力允许 `main` 窗口访问 `filesystem` 写入相关命令以及 `dialog` 命令，从而实现对用户所选文件的程序化访问。

##### identifier

名为“identifier”的章节

`string`

能力的标识符。

###### 示例

标题为“Example”的章节

`main-user-files-write`

##### local

标题为“local”的章节

`布尔值 (boolean)`

是否为本地应用 URL 启用此能力。默认为 `true`。

**默认值** ：`true`

##### permissions

标题为“permissions”的章节

`PermissionEntry`[] 每个条目必须唯一

附加到此能力的权限列表。

必须以 `${plugin-name}:${permission-name}` 的形式包含插件名称作为前缀。对于直接在应用程序本身中实现的命令，只需 `${permission-name}` 即可。

###### 示例

标题为“Example”的章节

    [

      "core:default",

      "shell:allow-open",

      "dialog:open",

      {

        "identifier": "fs:allow-write-text-file",

        "allow": [{ "path": "$HOME/test.txt" }]

      }

    ]

##### platforms

标题为“platforms”的章节

`Target`[] | `null`

限制此能力适用的目标平台。

默认情况下，所有平台都会被命中。

###### 示例

标题为“Example”的章节

`["macOS","windows"]`

##### remote

标题为“remote”的章节

`CapabilityRemote` | `null`

配置可以使用该能力权限的远程 URL。

此设置是可选的，默认情况下不进行设置，因为我们的默认用例是内容从我们的本地应用程序中提供。

注意

请确保您了解向远程源提供本地系统访问权限的安全影响。

###### 示例

标题为“Example”的章节

    {

      "urls": ["https://*.mydomain.dev"]

    }

##### webviews

标题为“webviews”的章节

`string`[]

受此能力影响的 Webview 列表。可以是 glob 模式。

该能力将在标签匹配此列表中任何模式的所有 Webview 上启用，无论该 Webview 的窗口标签是否匹配 [`Self::windows`] 中的模式。

###### 示例

标题为“Example”的章节

`["sub-webview-one", "sub-webview-two"]`

##### windows

名为“windows”的章节

`string`[]

受此能力影响的窗口列表。可以是 glob 模式。

如果窗口标签匹配此列表中的任何模式，则该能力将在该窗口的所有 Webview 上启用，无论 [`Self::webviews`] 的值如何。

在多 Webview 窗口上，建议指定 [`Self::webviews`] 并省略 [`Self::windows`] 以进行细粒度的访问控制。

###### 示例

标题为“Example”的章节

`["main"]`

### CapabilityEntry

标题为“CapabilityEntry”的章节

**以下任何一种** :

  * `Capability` 内联能力定义。
  * `string` 对能力标识符的引用。

一种能力条目，既可以是内联能力，也可以是对在单独文件中定义的能力的引用。

### CapabilityRemote

标题为“CapabilityRemote”的章节

与该能力关联的远程 URL 配置。

**对象属性** :

  * urls (必填)

##### urls

标题为“urls”的章节

`string`[]

此能力引用的远程域，使用 [URLPattern 标准](https://urlpattern.spec.whatwg.org/)。

###### 示例

名为“示例”的章节

  * “https://*.mydomain.dev”：允许 mydomain.dev 的子域
  * “<https://mydomain.dev/api/>*”：允许 mydomain.dev/api 的任何子路径

### Color

标题为“Color”的章节

**以下任何一种** :

  * `string` 模式 `^#?([A-Fa-f0-9]{3}|[A-Fa-f0-9]{6}|[A-Fa-f0-9]{8})$` 颜色十六进制字符串，例如：#fff、#ffffff 或 #ffffffff。
  * `integer` (格式为 `uint8`) | `integer` (格式为 `uint8`) | `integer` (格式为 `uint8`)[] 最大 3 项，最小 3 项 RGB 颜色数组。每个值的最小值均为 0，最大值为 255。
  * `integer` (格式为 `uint8`) | `integer` (格式为 `uint8`) | `integer` (格式为 `uint8`) | `integer` (格式为 `uint8`)[] 最大 4 项，最小 4 项 RGBA 颜色数组。每个值的最小值均为 0，最大值为 255。
  * 红、绿、蓝、透明度颜色值对象。每个值的最小值均为 0，最大值为 255。**对象属性** ： - alpha - blue (必填) - green (必填) - red (必填) ##### alpha `integer` (格式为 `uint8`) **默认值** ：`255` ##### blue `integer` (格式为 `uint8`) ##### green `integer` (格式为 `uint8`) ##### red `integer` (格式为 `uint8`)

### Csp

标题为“Csp”的章节

**以下任何一种** :

  * `string` 单个文本字符串中的完整 CSP 策略。
  * 将指令映射为其来源值（字符串列表）的对象。**允许添加额外属性** ：`CspDirectiveSources`

内容安全策略 (Content-Security-Policy) 定义。参见 <<https://mdn.org.cn/en-US/docs/Web/HTTP/CSP>>。

### CspDirectiveSources

标题为“CspDirectiveSources”的章节

**以下任何一种** :

  * `string` CSP 来源的内联列表。与 [`Self::List`] 相同，但使用空格分隔符连接。
  * `string`[] CSP 来源列表。集合将使用空格分隔符连接成 CSP 字符串。

内容安全策略指令来源列表。参见 <<https://mdn.org.cn/en-US/docs/Web/HTTP/Headers/Content-Security-Policy/Sources#sources>>。

### CustomSignCommandConfig

标题为“CustomSignCommandConfig”的章节

**以下任何一种** :

  * `string` 要执行的脚本的字符串表示法。“%1”将被替换为要签名的二进制文件的路径。这是命令的简单表示法。Tauri 将使用 `' '` 分割字符串，并将第一个元素作为命令名称，其余部分作为参数。如果您需要在命令或参数中使用空格，请使用对象表示法 [`Self::CommandWithOptions`]。
  * 命令的对象表示法。这是命令更复杂的表示法，但允许您在命令和参数中使用空格。**对象属性** ： - args (必填) - cmd (必填) ##### args `string`[] 传递给命令的参数。“%1”将被替换为要签名的二进制文件的路径。 ##### cmd `string` 用于对二进制文件进行签名的命令。

自定义签名命令配置。

### DebConfig

标题为“DebConfig”的章节

Debian (.deb) 捆绑包的配置。

查看更多：<<https://v2.tauri.org.cn/reference/config/#debconfig>>

**对象属性** :

  * changelog
  * conflicts
  * depends
  * desktopTemplate
  * files
  * postInstallScript
  * postRemoveScript
  * preInstallScript
  * preRemoveScript
  * priority
  * provides
  * recommends
  * replaces
  * section

##### changelog

标题为“changelog”的章节

`string` | `null`

未压缩的 Changelog 文件路径，将存储在 /usr/share/doc/package-name/changelog.gz。参见 <<https://www.debian.org/doc/debian-policy/ch-docs.html#changelog-files-and-release-notes>>

##### conflicts

标题为“conflicts”的章节

`string`[] | `null`

软件包冲突列表。

##### depends

标题为“depends”的章节

`string`[] | `null`

您的应用程序所依赖的 deb 依赖项列表。

##### desktopTemplate

标题为“desktopTemplate”的章节

`string` | `null`

自定义桌面文件 Handlebars 模板的路径。

可用变量：`categories`、`comment`（可选）、`exec`、`icon` 和 `name`。

##### files

名为“files”的章节

要包含在包中的文件。

**允许额外属性** : `string`

**默认** : `{}`

##### postInstallScript

标题为“postInstallScript”的章节

`string` | `null`

软件包解包后将执行的脚本路径。参见 <<https://www.debian.org/doc/debian-policy/ch-maintainerscripts.html>>

##### postRemoveScript

标题为“postRemoveScript”的章节

`string` | `null`

软件包移除后将执行的脚本路径。参见 <<https://www.debian.org/doc/debian-policy/ch-maintainerscripts.html>>

##### preInstallScript

标题为“preInstallScript”的章节

`string` | `null`

软件包解包前将执行的脚本路径。参见 <<https://www.debian.org/doc/debian-policy/ch-maintainerscripts.html>>

##### preRemoveScript

标题为“preRemoveScript”的章节

`string` | `null`

软件包移除前将执行的脚本路径。参见 <<https://www.debian.org/doc/debian-policy/ch-maintainerscripts.html>>

##### priority

标题为“priority”的章节

`string` | `null`

更改 Debian 软件包的优先级。默认设置为 `optional`。目前公认的优先级包括：`required`、`important`、`standard`、`optional`、`extra`

##### provides

标题为“provides”的章节

`string`[] | `null`

该软件包提供的依赖项列表。

##### recommends

标题为“recommends”的章节

`string`[] | `null`

您的应用程序推荐的 deb 依赖项列表。

##### replaces

标题为“replaces”的章节

`string`[] | `null`

软件包替换列表。

##### section

标题为“section”的章节

`string` | `null`

在 Debian Control 文件中定义部分。参见：<https://www.debian.org/doc/debian-policy/ch-archive.html#s-subsections>

### DisabledCspModificationKind

标题为“DisabledCspModificationKind”的章节

**以下任何一种** :

  * `boolean` 如果为 `true`，则禁用所有 CSP 修改。`false` 是默认值，它将 Tauri 配置为控制 CSP。
  * `string`[] 禁用给定列表中的 CSP 指令修改。

`dangerous_disable_asset_csp_modification` 配置选项的可能值。

### DmgConfig

标题为“DmgConfig”的章节

Apple 磁盘映像 (.dmg) 捆绑包的配置。

查看更多：<<https://v2.tauri.org.cn/reference/config/#dmgconfig>>

**对象属性** :

  * applicationFolderPosition
  * appPosition
  * background
  * windowPosition
  * windowSize

##### applicationFolderPosition

标题为“applicationFolderPosition”的章节

`Position`

窗口上应用程序文件夹的位置。

默认

    {

      "x": 480,

      "y": 170

    }

##### appPosition

标题为“appPosition”的章节

`Position`

窗口上应用文件的位置。

默认

    {

      "x": 180,

      "y": 170

    }

##### background

标题为“background”的章节

`string` | `null`

用作 dmg 文件背景的图像。支持的格式：`png`/`jpg`/`gif`。

##### windowPosition

标题为“windowPosition”的章节

`Position` | `null`

屏幕上卷窗口的位置。

##### windowSize

标题为“windowSize”的章节

`Size`

卷窗口的大小。

默认

    {

      "height": 400,

      "width": 660

    }

### ExportedFileAssociation

标题为“ExportedFileAssociation”的章节

导出的类型定义。在 macOS 上映射到 `UTExportedTypeDeclarations` 条目。

**对象属性** :

  * conformsTo
  * 标识符 (必需)

##### conformsTo

标题为“conformsTo”的章节

`string`[] | `null`

此类型所符合的类型。映射到 `UTTypeConformsTo`。

示例包括 `public.data`、`public.image`、`public.json` 和 `public.database`。

##### identifier

名为“identifier”的章节

`string`

导出类型的唯一标识符。映射到 `UTTypeIdentifier`。

### FileAssociation

标题为“FileAssociation”的章节

文件关联

**对象属性** :

  * contentTypes
  * description
  * exportedType
  * ext (必填)
  * mimeType
  * 名称 (name)
  * rank
  * role

##### contentTypes

标题为“contentTypes”的章节

`string`[] | `null`

声明对具有给定内容类型的文件支持。在 macOS 上映射到 `LSItemContentTypes`。

这允许支持由另一个符合此类型的应用程序声明的任何文件格式。新类型的声明可以通过 [`Self::exported_type`] 完成，并且链接到特定内容类型可以通过 [`ExportedFileAssociation::conforms_to`] 完成。

##### description

标题为“description”的章节

`string` | `null`

关联描述。仅限 Windows。它显示在 Windows 资源管理器的 `类型 (Type)` 列中。

##### exportedType

标题为“exportedType”的章节

`ExportedFileAssociation` | `null`

导出的类型定义。在 macOS 上映射到 `UTExportedTypeDeclarations` 条目。

如果关联的文件是您的应用程序定义的自定义文件类型，则应定义此项。

##### ext

标题为“ext”的章节

`AssociationExt`[]

与此应用程序关联的文件扩展名。例如：‘png’

##### mimeType

标题为“mimeType”的章节

`string` | `null`

MIME 类型，例如 ‘image/png’ 或 ‘text/plain’。仅限 Linux。

##### 名称 (name)

标题为“name”的章节

`string` | `null`

名称。在 macOS 上映射到 `CFBundleTypeName`。默认为 `ext[0]`

##### rank

标题为“rank”的章节

`HandlerRank`

此应用程序在自称为给定文件类型的编辑器或查看器的应用程序中的排名。在 macOS 上映射到 `LSHandlerRank`。

**默认值** ：`"Default"`

##### role

标题为“role”的章节

`BundleTypeRole`

应用程序关于该类型的角色。在 macOS 上映射到 `CFBundleTypeRole`。

**默认值** ：`"Editor"`

### FrontendDist

标题为“FrontendDist”的章节

**以下任何一种** :

  * `string` (格式为 `uri`) 应该用作默认应用程序 URL 的外部 URL。在这种情况下，没有资产被嵌入到应用程序中。
  * `string` 包含前端分发资产的目录路径。
  * `string`[] 要嵌入到应用程序中的文件数组。

定义要嵌入到应用程序中的 URL 或资产。

### FsScope

标题为“FsScope”的章节

**以下任何一种** :

  * `string`[] 此作用域允许的路径列表。
  * 完整的作用域配置。**对象属性** ： - allow - deny - requireLiteralLeadingDot ##### allow `string`[] 此作用域允许的路径列表。**默认值** ：`[]` ##### deny `string`[] 此作用域不允许的路径列表。这比 [`Self::Scope::allow`] 列表具有更高的优先级。**默认值** ：`[]` ##### requireLiteralLeadingDot `boolean` | `null` 包含以 `.` 开头的组件的路径是否需要 `.` 在模式中字面匹配；`*`、`?`、`**` 或 `[...]` 不会匹配。这很有用，因为此类文件在 Unix 系统上通常被视为隐藏文件，在列出文件时可能需要跳过它们。在 Unix 系统上默认为 `true`，在 Windows 上默认为 `false`

协议作用域定义。它是一个限制 Webview API 访问的 glob 模式列表。

每个模式都可以以解析为系统基本目录的变量开头。这些变量包括：`$AUDIO`、`$CACHE`、`$CONFIG`、`$DATA`、`$LOCALDATA`、`$DESKTOP`、`$DOCUMENT`、`$DOWNLOAD`、`$EXE`、`$FONT`、`$HOME`、`$PICTURE`、`$PUBLIC`、`$RUNTIME`、`$TEMPLATE`、`$VIDEO`、`$RESOURCE`、`$TEMP`、`$APPCONFIG`、`$APPDATA`、`$APPLOCALDATA`、`$APPCACHE`、`$APPLOG`。

### HandlerRank

标题为“HandlerRank”的章节

**以下其中之一** :

  * `"Default"` LSHandlerRank.Default。此应用是此类文件的打开者；如果没有指定排名，也使用此值。
  * `"Owner"` LSHandlerRank.Owner。此应用是此类文件的主要创建者。
  * `"Alternate"` LSHandlerRank.Alternate。此应用是此类文件的辅助查看器。
  * `"None"` LSHandlerRank.None。此应用从不被选择打开此类文件，但它接受此类文件的拖放。

对应 LSHandlerRank

### HeaderConfig

标题为“HeaderConfig”的章节

一个结构，其中键是特定的 http 头名称。

如果定义了这些键的值，它们将被作为响应消息的一部分发送。这不包括错误消息和 IPC 消息。

##### 配置示例

标题为“配置示例”的章节

    {

     //..

      app:{

        //..

        security: {

          headers: {

            "Cross-Origin-Opener-Policy": "same-origin",

            "Cross-Origin-Embedder-Policy": "require-corp",

            "Timing-Allow-Origin": [

              "https://mdn.org.cn",

              "https://example.com",

            ],

            "Access-Control-Expose-Headers": "Tauri-Custom-Header",

            "Tauri-Custom-Header": {

              "key1": "'value1' 'value2'",

              "key2": "'value3'"

            }

          },

          csp: "default-src 'self'; connect-src ipc: http://ipc.localhost",

        }

        //..

      }

     //..

    }

在此示例中，设置了 `Cross-Origin-Opener-Policy` 和 `Cross-Origin-Embedder-Policy` 以允许使用 [`SharedArrayBuffer`](https://mdn.org.cn/en-US/docs/Web/JavaScript/Reference/Global_Objects/SharedArrayBuffer)。结果是，这些头文件随后被设置在通过 crates/tauri/src/protocol/tauri.rs 中的 `get_response` 函数发送的每个响应上。Content-Security-Policy 头文件是单独定义的，因为它也是单独处理的。

对于 helloworld 示例，此配置转换为那些响应头

    access-control-allow-origin:  http://tauri.localhost

    access-control-expose-headers: Tauri-Custom-Header

    content-security-policy: default-src 'self'; connect-src ipc: http://ipc.localhost; script-src 'self' 'sha256-Wjjrs6qinmnr+tOry8x8PPwI77eGpUFR3EEGZktjJNs='

    content-type: text/html

    cross-origin-embedder-policy: require-corp

    cross-origin-opener-policy: same-origin

    tauri-custom-header: key1 'value1' 'value2'; key2 'value3'

    timing-allow-origin: https://mdn.org.cn, https://example.com

由于生成的头值始终是“字符串类”的，因此根据 HeaderSource 的数据类型，需要进行转换。

  * `String` (JS/Rust)：生成的头值保持不变
  * `Array` (JS) / `Vec<String>` (Rust)：项目通过 ”, ” 连接以生成头值
  * `Object` (JS) / `Hashmap<String,String>` (Rust)：项目由：键 + 空格 + 值组成。然后项目通过 ”; ” 连接以生成头值

**对象属性** :

  * Access-Control-Allow-Credentials
  * Access-Control-Allow-Headers
  * Access-Control-Allow-Methods
  * Access-Control-Expose-Headers
  * Access-Control-Max-Age
  * Cross-Origin-Embedder-Policy
  * Cross-Origin-Opener-Policy
  * Cross-Origin-Resource-Policy
  * Permissions-Policy
  * Service-Worker-Allowed
  * Tauri-Custom-Header
  * Timing-Allow-Origin
  * X-Content-Type-Options

##### Access-Control-Allow-Credentials

标题为“Access-Control-Allow-Credentials”的章节

`HeaderSource` | `null`

Access-Control-Allow-Credentials 响应头告诉浏览器服务器是否允许跨源 HTTP 请求包含凭据。

参见 <<https://mdn.org.cn/en-US/docs/Web/HTTP/Headers/Access-Control-Allow-Credentials>>

##### Access-Control-Allow-Headers

标题为“Access-Control-Allow-Headers”的章节

`HeaderSource` | `null`

Access-Control-Allow-Headers 响应头用于响应包含 Access-Control-Request-Headers 的预检请求，以指示在实际请求期间可以使用哪些 HTTP 头。

如果请求具有 Access-Control-Request-Headers 头，则此头是必需的。

请参阅 <<https://mdn.org.cn/en-US/docs/Web/HTTP/Headers/Access-Control-Allow-Headers>>

##### Access-Control-Allow-Methods

标题为“Access-Control-Allow-Methods”的章节

`HeaderSource` | `null`

Access-Control-Allow-Methods 响应标头用于在响应预检请求时指定访问资源时允许的一种或多种方法。

请参阅 <<https://mdn.org.cn/en-US/docs/Web/HTTP/Headers/Access-Control-Allow-Methods>>

##### Access-Control-Expose-Headers

标题为“Access-Control-Expose-Headers”的章节

`HeaderSource` | `null`

Access-Control-Expose-Headers 响应标头允许服务器指示在跨源请求响应中，哪些响应标头应可供浏览器中运行的脚本使用。

请参阅 <<https://mdn.org.cn/en-US/docs/Web/HTTP/Headers/Access-Control-Expose-Headers>>

##### Access-Control-Max-Age

标题为“Access-Control-Max-Age”的章节

`HeaderSource` | `null`

Access-Control-Max-Age 响应标头指示预检请求的结果（即包含在 Access-Control-Allow-Methods 和 Access-Control-Allow-Headers 标头中的信息）可以缓存多长时间。

请参阅 <<https://mdn.org.cn/en-US/docs/Web/HTTP/Headers/Access-Control-Max-Age>>

##### Cross-Origin-Embedder-Policy

标题为“Cross-Origin-Embedder-Policy”的章节

`HeaderSource` | `null`

HTTP 响应标头 Cross-Origin-Embedder-Policy (COEP) 用于配置将跨源资源嵌入到文档中。

请参阅 <<https://mdn.org.cn/en-US/docs/Web/HTTP/Headers/Cross-Origin-Embedder-Policy>>

##### Cross-Origin-Opener-Policy

标题为“Cross-Origin-Opener-Policy”的章节

`HeaderSource` | `null`

HTTP 响应标头 Cross-Origin-Opener-Policy (COOP) 允许您确保顶级文档不会与跨源文档共享浏览上下文组。COOP 将对您的文档进行进程隔离，潜在攻击者无法在弹出窗口中打开文档并访问您的全局对象，从而防止一类被称为 XS-Leaks 的跨源攻击。

请参阅 <<https://mdn.org.cn/en-US/docs/Web/HTTP/Headers/Cross-Origin-Opener-Policy>>

##### Cross-Origin-Resource-Policy

标题为“Cross-Origin-Resource-Policy”的章节

`HeaderSource` | `null`

HTTP 响应标头 Cross-Origin-Resource-Policy 传达了一种意愿，即由浏览器阻止对给定资源的 no-cors 跨源/跨站请求。

请参阅 <<https://mdn.org.cn/en-US/docs/Web/HTTP/Headers/Cross-Origin-Resource-Policy>>

##### Permissions-Policy

标题为“Permissions-Policy”的章节

`HeaderSource` | `null`

HTTP 标头 Permissions-Policy 提供了一种机制，允许或拒绝在文档或文档内的任何 <iframe> 元素中使用浏览器特性。

请参阅 <<https://mdn.org.cn/en-US/docs/Web/HTTP/Headers/Permissions-Policy>>

##### Service-Worker-Allowed

标题为“Service-Worker-Allowed”的章节

`HeaderSource` | `null`

HTTP 响应标头 Service-Worker-Allowed 用于拓宽 Service Worker 默认作用域的路径限制。

默认情况下，Service Worker 注册的作用域是 Service Worker 脚本所在的目录。例如，如果脚本 `sw.js` 位于 `/js/sw.js`，它默认只能控制 `/js/` 下的 URL。服务器可以使用 `Service-Worker-Allowed` 标头来允许 Service Worker 控制其自身目录之外的 URL。

请参阅 <<https://mdn.org.cn/en-US/docs/Web/HTTP/Reference/Headers/Service-Worker-Allowed>>

##### Tauri-Custom-Header

标题为“Tauri-Custom-Header”的章节

`HeaderSource` | `null`

一个自定义标头字段 Tauri-Custom-Header，请勿使用它。请记住相应地设置 Access-Control-Expose-Headers

**不建议用于生产环境**

##### Timing-Allow-Origin

标题为“Timing-Allow-Origin”的章节

`HeaderSource` | `null`

Timing-Allow-Origin 响应标头指定允许查看通过资源时序 API (Resource Timing API) 获取的属性值的源；否则，由于跨源限制，这些值会被报告为零。

请参阅 <<https://mdn.org.cn/en-US/docs/Web/HTTP/Headers/Timing-Allow-Origin>>

##### X-Content-Type-Options

标题为“X-Content-Type-Options”的章节

`HeaderSource` | `null`

X-Content-Type-Options HTTP 响应标头是服务器使用的标记，用于指示应遵循 Content-Type 标头中通告的 MIME 类型，而不应更改它们。该标头允许您通过声明 MIME 类型是刻意配置的来避免 MIME 类型嗅探。

请参阅 <<https://mdn.org.cn/en-US/docs/Web/HTTP/Headers/X-Content-Type-Options>>

### HeaderSource

标题为“HeaderSource”的章节

**以下任何一种** :

  * `string` 标头值的字符串版本
  * `string`[] 标头值的列表版本。各项通过 “,” 连接以形成真实的标头值
  * (Rust 结构体 | Json | JavaScript 对象) 等效的标头值。各项由：键 + 空格 + 值组成。各项随后通过 “;” 连接以形成真实的标头值。**允许其他属性** ： `string`

标头源的定义

标头名称对应的标头值

### HookCommand

标题为“HookCommand”的章节

**以下任何一种** :

  * `string`：使用默认选项运行给定的脚本。
  * 使用自定义选项运行给定脚本。 **对象属性** ： - cwd - script (必需) ##### cwd `string` | `null` 当前工作目录。 ##### script `string` 要执行的脚本。

描述触发 CLI 钩子时要执行的 shell 命令。

### 标识符

标题为“Identifier”的章节

`string`

### IosConfig

标题为“IosConfig”的章节

iOS 目标平台的常规配置。

**对象属性** :

  * bundleVersion
  * developmentTeam
  * frameworks
  * infoPlist
  * minimumSystemVersion
  * template

##### bundleVersion

标题为“bundleVersion”的章节

`string` | `null`

标识包迭代的构建版本。

对应于包的 CFBundleVersion 属性。

##### developmentTeam

标题为“developmentTeam”的章节

`string` | `null`

开发团队。此值对于 iOS 开发是必需的，因为强制执行代码签名。可以设置 `APPLE_DEVELOPMENT_TEAM` 环境变量来覆盖它。

##### frameworks

标题为“frameworks”的章节

`string`[] | `null`

指示需要与应用程序捆绑在一起的任何 iOS 框架的字符串列表。

请注意，您需要重新创建 iOS 项目以使更改生效。

##### infoPlist

标题为“infoPlist”的章节

`string` | `null`

要与默认 Info.plist 合并的 Info.plist 文件路径。

请注意，Tauri 也会在与 Tauri 配置文件相同的目录中查找 `Info.plist` 和 `Info.ios.plist` 文件。

##### minimumSystemVersion

标题为“minimumSystemVersion”的章节

`string`

指示捆绑应用程序支持的最低 iOS 版本的版本字符串。默认为 `13.0`。

映射到 IPHONEOS_DEPLOYMENT_TARGET 值。

**默认** : `"14.0"`

##### template

标题为“template”的章节

`string` | `null`

要使用的自定义 [XcodeGen](%3Chttps://github.com/yonaskolb/XcodeGen%3E) project.yml 模板。

### LinuxConfig

标题为“LinuxConfig”的章节

Linux 包配置。

更多信息： <<https://v2.tauri.org.cn/reference/config/#linuxconfig>>

**对象属性** :

  * appimage
  * deb
  * rpm

##### appimage

标题为“appimage”的章节

`AppImageConfig`

AppImage 包配置。

默认

    {

      "bundleMediaFramework": false,

      "files": {}

    }

##### deb

标题为“deb”的章节

`DebConfig`

Debian 包配置。

默认

    {

      "files": {}

    }

##### rpm

标题为“rpm”的章节

`RpmConfig`

RPM 包配置。

默认

    {

      "epoch": 0,

      "files": {},

      "release": "1"

    }

### LogicalPosition

标题为“LogicalPosition”的章节

位置坐标结构体。

**对象属性** :

  * x (必需)
  * y (必需)

##### x

标题为“x”的章节

`number` 格式为 `double`

X 坐标。

##### y

标题为“y”的章节

`number` 格式为 `double`

Y 坐标。

### MacConfig

标题为“MacConfig”的章节

macOS 捆绑包配置。

更多信息： <<https://v2.tauri.org.cn/reference/config/#macconfig>>

**对象属性** :

  * bundleName
  * bundleVersion
  * dmg
  * entitlements
  * exceptionDomain
  * files
  * frameworks
  * hardenedRuntime
  * infoPlist
  * minimumSystemVersion
  * providerShortName
  * signingIdentity

##### bundleName

标题为“bundleName”的章节

`string` | `null`

构建该包的构建器名称。

对应于包的 CFBundleName 属性。

如果未设置，默认为包的产品名称。

##### bundleVersion

标题为“bundleVersion”的章节

`string` | `null`

标识包迭代的构建版本。

对应于包的 CFBundleVersion 属性。

##### dmg

标题为“dmg”的章节

`DmgConfig`

DMG 特定的设置。

默认

    {

      "appPosition": {

        "x": 180,

        "y": 170

      },

      "applicationFolderPosition": {

        "x": 480,

        "y": 170

      },

      "windowSize": {

        "height": 400,

        "width": 660

      }

    }

##### entitlements

标题为“entitlements”的章节

`string` | `null`

授权文件路径。

##### exceptionDomain

标题为“exceptionDomain”的章节

`string` | `null`

允许您的应用程序与外部世界通信。它应该是一个小写的、不包含端口和协议的域名。

##### files

名为“files”的章节

相对于 Contents 目录包含在应用程序中的文件。

**允许额外属性** : `string`

**默认** : `{}`

##### frameworks

标题为“frameworks”的章节

`string`[] | `null`

指示需要与应用程序捆绑在一起的任何 macOS X 框架的字符串列表。

如果使用名称，则必须省略 “.framework”，它将会在标准安装位置中查找。您也可以使用特定框架的路径。

##### hardenedRuntime

标题为“hardenedRuntime”的章节

`布尔值 (boolean)`

代码签名是否应启用[强化运行时 (hardened runtime)](https://developer.apple.com/documentation/security/hardened_runtime)（针对可执行文件）。

**默认值** ：`true`

##### infoPlist

标题为“infoPlist”的章节

`string` | `null`

要与默认 Info.plist 合并的 Info.plist 文件路径。

请注意，Tauri 也会在与 Tauri 配置文件相同的目录中查找 `Info.plist` 文件。

##### minimumSystemVersion

标题为“minimumSystemVersion”的章节

`string` | `null`

指示捆绑应用程序支持的最低 macOS X 版本的版本字符串。默认为 `10.13`。

将其设置为 `null` 将完全从包的 `Info.plist` 中删除 `LSMinimumSystemVersion` 字段，并删除 `MACOSX_DEPLOYMENT_TARGET` 环境变量。

在 `tauri dev` 中会被忽略。

空字符串被视为无效值，因此将使用默认值。

**默认** : `"10.13"`

##### providerShortName

标题为“providerShortName”的章节

`string` | `null`

公证的提供者简称。

##### signingIdentity

标题为“signingIdentity”的章节

`string` | `null`

用于代码签名的身份。

### NsisCompression

标题为“NsisCompression”的章节

**以下其中之一** :

  * `"zlib"` ZLIB 使用 deflate 算法，是一种快速简单的方法。在默认压缩级别下，它占用约 300 KB 内存。
  * `"bzip2"` BZIP2 通常比 ZLIB 提供更好的压缩比，但稍慢且占用更多内存。在默认压缩级别下，它占用约 4 MB 内存。
  * `"lzma"` LZMA (默认) 是一种新的压缩方法，提供非常好的压缩比。解压缩速度很高（在 2 GHz CPU 上为 10-20 MB/s），压缩速度较低。解压缩使用的内存大小为字典大小加上几 KB，默认值为 8 MB。
  * `"none"` 禁用压缩

NSIS 安装程序中使用的压缩算法。

请参阅 <<https://nsis.sourceforge.io/Reference/SetCompressor>>

### NsisConfig

标题为“NsisConfig”的章节

使用 NSIS 的安装程序包配置。

**对象属性** :

  * compression
  * customLanguageFiles
  * displayLanguageSelector
  * headerImage
  * installerHooks
  * installerIcon
  * installMode
  * languages
  * minimumWebview2Version
  * sidebarImage
  * startMenuFolder
  * template

##### compression

标题为“compression”的章节

`NsisCompression`

设置用于压缩安装程序中文件的压缩算法。

请参阅 <<https://nsis.sourceforge.io/Reference/SetCompressor>>

**默认** : `"lzma"`

##### customLanguageFiles

标题为“customLanguageFiles”的章节

| `null`

键值对，其中键是语言，值是包含 Tauri 自定义消息翻译文本的自定义 `.nsh` 文件路径。

请参阅 <<https://github.com/tauri-apps/tauri/blob/dev/crates/tauri-bundler/src/bundle/windows/nsis/languages/English.nsh>> 获取示例 `.nsh` 文件。

**注意** ：键必须是有效的 NSIS 语言，并且必须将其添加到 [`NsisConfig`] 语言数组中。

**允许额外属性** : `string`

##### displayLanguageSelector

标题为“displayLanguageSelector”的章节

`布尔值 (boolean)`

是否在渲染安装程序和卸载程序窗口之前显示语言选择器对话框。默认选择操作系统语言，回退到 `languages` 数组中的第一种语言。

##### headerImage

标题为“headerImage”的章节

`string` | `null`

要在安装程序页面页眉显示的位图文件路径。

建议尺寸为 150px x 57px。

##### installerHooks

标题为“installerHooks”的章节

`string` | `null`

包含特殊 NSIS 宏的 `.nsh` 文件路径，用于挂钩到主 installer.nsi 脚本中。

支持的钩子有：

  * `NSIS_HOOK_PREINSTALL`：此钩子在复制文件、设置注册表键值和创建快捷方式之前运行。
  * `NSIS_HOOK_POSTINSTALL`：此钩子在安装程序完成复制所有文件、设置注册表键值和创建快捷方式之后运行。
  * `NSIS_HOOK_PREUNINSTALL`：此钩子在移除任何文件、注册表键值和快捷方式之前运行。
  * `NSIS_HOOK_POSTUNINSTALL`：此钩子在文件、注册表键值和快捷方式被移除之后运行。

###### 示例

标题为“Example”的章节

    !macro NSIS_HOOK_PREINSTALL

      MessageBox MB_OK "PreInstall"

    !macroend

    !macro NSIS_HOOK_POSTINSTALL

      MessageBox MB_OK "PostInstall"

    !macroend

    !macro NSIS_HOOK_PREUNINSTALL

      MessageBox MB_OK "PreUnInstall"

    !macroend

    !macro NSIS_HOOK_POSTUNINSTALL

      MessageBox MB_OK "PostUninstall"

    !macroend

##### installerIcon

标题为“installerIcon”的章节

`string` | `null`

用作安装程序图标的图标文件路径。

##### installMode

标题为“installMode”的章节

`NSISInstallerMode`

安装是针对所有用户还是仅针对当前用户。

**默认** : `"currentUser"`

##### languages

标题为“languages”的章节

`string`[] | `null`

安装程序语言列表。默认使用操作系统语言。如果操作系统语言不在语言列表中，则使用第一种语言。要允许用户选择语言，请将 `display_language_selector` 设置为 `true`。

请参阅 <<https://github.com/kichik/nsis/tree/9465c08046f00ccb6eda985abbdbf52c275c6c4d/Contrib/Language%20files>> 获取完整的语言列表。

##### minimumWebview2Version

标题为“minimumWebview2Version”的章节

`string` | `null`

尝试确保 WebView2 版本等于或高于此版本；如果用户的 WebView2 版本低于此版本，安装程序将尝试触发 WebView2 更新。

##### sidebarImage

标题为“sidebarImage”的章节

`string` | `null`

欢迎页面和完成页面的位图文件路径。

建议尺寸为 164px x 314px。

##### startMenuFolder

标题为“startMenuFolder”的章节

`string` | `null`

设置开始菜单快捷方式的文件夹名称。

如果您有多个应用程序并希望将它们的快捷方式归为一个文件夹，或者如果您通常喜欢将快捷方式设置在文件夹中，请使用此选项。

示例

  * `AwesomePublisher`，快捷方式将放置在 `%AppData%\Microsoft\Windows\Start Menu\Programs\AwesomePublisher\&lt;your-app&gt;.lnk`
  * 如果未设置，快捷方式将放置在 `%AppData%\Microsoft\Windows\Start Menu\Programs\&lt;your-app&gt;.lnk`

##### template

标题为“template”的章节

`string` | `null`

要使用的自定义 .nsi 模板。

### NSISInstallerMode

标题为“NSISInstallerMode”的章节

**以下其中之一** :

  * `"currentUser"` 安装程序的默认模式。默认将应用程序安装在不需要管理员权限的目录中。安装程序元数据将保存在 `HKCU` 注册表路径下。
  * `"perMachine"` 默认将应用程序安装在 `Program Files` 文件夹目录中，需要管理员权限进行安装。安装程序元数据将保存在 `HKLM` 注册表路径下。
  * `"both"` 结合了这两种模式，并允许用户在安装时选择是为当前用户安装还是按机器安装。请注意，即使在该模式下，如果用户只想为当前用户安装，该模式也需要管理员权限。安装程序元数据将根据用户的选择保存在 `HKLM` 或 `HKCU` 注册表路径下。

NSIS 安装程序的安装模式。

### 数字

标题为“Number”的章节

**以下任何一种** :

  * `integer` 格式为 `int64`，表示 [`i64`]。
  * `number` 格式为 `double`，表示 [`f64`]。

一个有效的 ACL 数字。

### PatternKind

标题为“PatternKind”的章节

**以下其中之一** :

  * 褐地 (Brownfield) 模式。 **对象属性** ： - use (必需) ##### use `"brownfield"`
  * 隔离模式。推荐用于安全目的。 **对象属性** ： - options (必需) - use (必需) ##### options **对象属性** ： - dir (必需) ###### dir `string` 包含索引 HTML 文件（包含安全隔离应用程序）的目录。 ##### use `"isolation"`

应用程序模式。

### PermissionEntry

标题为“PermissionEntry”的章节

**以下任何一种** :

  * `Identifier` 按标识符引用权限或权限集。
  * 按标识符引用权限或权限集并扩展其作用域。 **对象属性** ： - allow - deny - identifier (必需) ##### allow `Value`[] | `null` 定义作用域允许内容的数据。 ##### deny `Value`[] | `null` 定义作用域拒绝内容的数据。验证逻辑应优先处理此项。 ##### identifier `Identifier` 权限或权限集的标识符。

[`Capability`] 中权限条目的条目可以是原始权限 [`Identifier`]，也可以是引用权限并扩展其作用域的对象。

### PluginConfig

标题为“PluginConfig”的章节

插件配置包含一个将插件名称映射到其配置对象的 HashMap。

更多信息： <<https://v2.tauri.org.cn/reference/config/#pluginconfig>>

**允许其他属性** ： `true`

### Position

标题为“Position”的章节

位置坐标结构体。

**对象属性** :

  * x (必需)
  * y (必需)

##### x

标题为“x”的章节

`integer`，格式为 `uint32`

X 坐标。

##### y

标题为“y”的章节

`integer`，格式为 `uint32`

Y 坐标。

### PreventOverflowConfig

标题为“PreventOverflowConfig”的章节

**以下任何一种** :

  * `boolean` 是否启用溢出预防
  * `PreventOverflowMargin` 启用带有边距的溢出预防，这样窗口大小 + 此边距将不会超过工作区

带有边距的溢出预防

### PreventOverflowMargin

标题为“PreventOverflowMargin”的章节

启用带有边距的溢出预防，这样窗口大小 + 此边距将不会超过工作区

**对象属性** :

  * height (必需)
  * width (必需)

##### 高度

标题为“height”的章节

`integer`，格式为 `uint32`

垂直边距（物理像素）

##### 宽度

标题为“width”的章节

`integer`，格式为 `uint32`

水平边距（物理像素）

### RpmCompression

标题为“RpmCompression”的章节

**以下其中之一** :

  * Gzip 压缩 **对象属性** ： - level (必需) - type (必需) ##### level `integer` 格式为 `uint32` Gzip 压缩级别 ##### type `"gzip"`
  * Zstd 压缩 **对象属性** ： - level (必需) - type (必需) ##### level `integer` 格式为 `int32` Zstd 压缩级别 ##### type `"zstd"`
  * Xz 压缩 **对象属性** ： - level (必需) - type (必需) ##### level `integer` 格式为 `uint32` Xz 压缩级别 ##### type `"xz"`
  * Bzip2 压缩 **对象属性** ： - level (必需) - type (必需) ##### level `integer` 格式为 `uint32` Bzip2 压缩级别 ##### type `"bzip2"`
  * 禁用压缩 **对象属性** ： - type (必需) ##### type `"none"`

捆绑 RPM 包时使用的压缩算法。

### RpmConfig

标题为“RpmConfig”的章节

RPM 包配置。

**对象属性** :

  * compression
  * conflicts
  * depends
  * desktopTemplate
  * epoch
  * files
  * obsoletes
  * postInstallScript
  * postRemoveScript
  * preInstallScript
  * preRemoveScript
  * provides
  * recommends
  * release

##### compression

标题为“compression”的章节

`RpmCompression` | `null`

压缩算法和级别。默认为 `Gzip` 级别 6。

##### conflicts

标题为“conflicts”的章节

`string`[] | `null`

您的应用程序冲突的 RPM 依赖项列表。必须不存在这些依赖项才能安装该包。

##### depends

标题为“depends”的章节

`string`[] | `null`

您的应用程序所依赖的 RPM 依赖项列表。

##### desktopTemplate

标题为“desktopTemplate”的章节

`string` | `null`

自定义桌面文件 Handlebars 模板的路径。

可用变量：`categories`、`comment`（可选）、`exec`、`icon` 和 `name`。

##### epoch

标题为“epoch”的章节

`integer`，格式为 `uint32`

RPM epoch。

##### files

名为“files”的章节

要包含在包中的文件。

**允许额外属性** : `string`

**默认** : `{}`

##### obsoletes

标题为“obsoletes”的章节

`string`[] | `null`

您的应用程序所取代的 RPM 依赖项列表 - 如果安装了此包，列为“obsoletes”的包将自动被删除（如果它们存在）。

##### postInstallScript

标题为“postInstallScript”的章节

`string` | `null`

在包解压后执行的脚本路径。请参阅 <<http://ftp.rpm.org/max-rpm/s1-rpm-inside-scripts.html>>

##### postRemoveScript

标题为“postRemoveScript”的章节

`string` | `null`

在包移除后执行的脚本路径。请参阅 <<http://ftp.rpm.org/max-rpm/s1-rpm-inside-scripts.html>>

##### preInstallScript

标题为“preInstallScript”的章节

`string` | `null`

在包解压前执行的脚本路径。请参阅 <<http://ftp.rpm.org/max-rpm/s1-rpm-inside-scripts.html>>

##### preRemoveScript

标题为“preRemoveScript”的章节

`string` | `null`

在包移除前执行的脚本路径。请参阅 <<http://ftp.rpm.org/max-rpm/s1-rpm-inside-scripts.html>>

##### provides

标题为“provides”的章节

`string`[] | `null`

您的应用程序提供的 RPM 依赖项列表。

##### recommends

标题为“recommends”的章节

`string`[] | `null`

您的应用程序推荐的 RPM 依赖项列表。

##### release

标题为“release”的章节

`string`

RPM release 标签。

**默认** : `"1"`

### RunnerConfig

标题为“RunnerConfig”的章节

**以下任何一种** :

  * `string` 指定要运行的二进制文件的字符串。
  * 具有高级配置选项的对象。 **对象属性** ： - args - cmd (必需) - cwd ##### args `string`[] | `null` 传递给命令的参数。 ##### cmd `string` 要运行的二进制文件。 ##### cwd `string` | `null` 运行命令的当前工作目录。

运行程序配置。

### ScrollBarStyle

“ScrollBarStyle” 章节

**以下其中之一** :

  * `"default"` Webview 中使用的滚动条样式。
  * `"fluentOverlay"` Fluent UI 风格的覆盖式滚动条。**仅限 Windows** 。需要 WebView2 Runtime 125.0.2535.41 或更高版本，在旧版本上无效，详见 <https://learn.microsoft.com/en-us/microsoft-edge/webview2/release-notes/?tabs=dotnetcsharp#10253541>

Webview 中使用的滚动条样式。

##### 平台特定

名为“平台特定”的章节

  * **Windows** ：对于所有指向相同数据目录的 webview，此选项必须设置为相同的值。

### SecurityConfig

“SecurityConfig” 章节

安全配置。

查看更多：<<https://v2.tauri.org.cn/reference/config/#securityconfig>>

**对象属性** :

  * assetProtocol
  * capabilities
  * csp
  * dangerousDisableAssetCspModification
  * devCsp
  * freezePrototype
  * headers
  * pattern

##### assetProtocol

“assetProtocol” 章节

`AssetProtocolConfig`

自定义协议配置。

默认

    {

      "enable": false,

      "scope": []

    }

##### capabilities

“capabilities” 章节

`CapabilityEntry`[]

应用程序启用的功能列表。

默认情况下（未设置或列表为空），会包含 `./capabilities/` 下的所有功能文件。通过在此项中设置值，您可以细粒度地控制哪些功能被包含。

您既可以使用在 `./capabilities/` 中定义的功能文件的标识符进行引用，也可以直接嵌入一个 [`Capability`] 对象。

###### 示例

标题为“Example”的章节

    {

      "app": {

        "capabilities": [

          "main-window",

          {

            "identifier": "drag-window",

            "permissions": ["core:window:allow-start-dragging"]

          }

        ]

      }

    }

**默认** : `[]`

##### csp

“csp” 章节

`Csp` | `null`

构建后的应用程序中所有 HTML 文件将被注入的内容安全策略 (CSP)。如果未指定 `dev_csp`，该值也会在开发环境中注入。

这是配置中非常重要的一部分，因为它有助于确保您的 WebView 安全。请参阅 <<https://mdn.org.cn/en-US/docs/Web/HTTP/CSP>>。

##### dangerousDisableAssetCspModification

“dangerousDisableAssetCspModification” 章节

`DisabledCspModificationKind`

禁用 Tauri 注入的 CSP 源。

在编译时，Tauri 会解析所有前端资源，并通过注入 nonce 和 hash 源来修改内容安全策略 (CSP)，从而仅允许加载您自己的脚本和样式。这会严格限制您的 CSP，在与其他灵活源一起使用时可能会引入问题。

此配置选项支持布尔值和字符串列表作为值。布尔值指示 Tauri 禁用所有 CSP 注入，而字符串列表则表示 Tauri 不应注入的 CSP 指令。

**警告：** 仅在您清楚自己在做什么并正确配置了 CSP 的情况下才禁用此功能。没有此 Tauri 保护，您的应用程序可能容易受到 XSS 攻击。

##### devCsp

“devCsp” 章节

`Csp` | `null`

开发环境下将注入到所有 HTML 文件中的内容安全策略 (CSP)。

这是配置中非常重要的一部分，因为它有助于确保您的 WebView 安全。请参阅 <<https://mdn.org.cn/en-US/docs/Web/HTTP/CSP>>。

##### freezePrototype

“freezePrototype” 章节

`布尔值 (boolean)`

使用自定义协议时冻结 `Object.prototype`。

##### headers

“headers” 章节

`HeaderConfig` | `null`

添加到 Tauri 发送给 webview 的每个 HTTP 响应中的响应头。这不包括 IPC 消息和错误响应。

##### pattern

“pattern” 章节

`PatternKind`

使用的模式。

默认

    {

      "use": "brownfield"

    }

### Size

“Size” 章节

窗口大小。

**对象属性** :

  * height (必需)
  * width (必需)

##### 高度

标题为“height”的章节

`integer`，格式为 `uint32`

窗口高度。

##### 宽度

标题为“width”的章节

`integer`，格式为 `uint32`

窗口宽度。

### 目标

“Target” 章节

**以下其中之一** :

  * `"macOS"` macOS。
  * `"windows"` Windows。
  * `"linux"` Linux。
  * `"android"` Android。
  * `"iOS"` iOS。

平台目标。

### Theme

“Theme” 章节

**以下其中之一** :

  * `"Light"` 浅色主题。
  * `"Dark"` 深色主题。

系统主题。

### TitleBarStyle

“TitleBarStyle” 章节

**以下其中之一** :

  * `"Visible"` 普通标题栏。
  * `"Transparent"` 使标题栏透明，从而显示窗口背景颜色。如果您不需要在标题栏下方有实际的 HTML 内容，这很有用。这可以避免使用 `TitleBarStyle::Overlay` 的不足之处。当 Tauri 允许您设置自定义窗口背景颜色时，它将更有用。
  * `"Overlay"` 将标题栏显示为窗口内容的透明覆盖层。请注意：- 标题栏的高度在不同操作系统版本上会有所不同，这可能导致窗口控件和标题不在您预期的位置。- 您需要定义一个自定义拖拽区域以使窗口可拖拽，然而由于限制，当窗口未处于聚焦状态时无法拖拽窗口 <<https://github.com/tauri-apps/tauri/issues/4316>>。- 窗口标题的颜色取决于系统主题。

如何在 macOS 上显示窗口标题栏。

### TrayIconConfig

“TrayIconConfig” 章节

应用程序托盘图标的配置。

查看更多：<<https://v2.tauri.org.cn/reference/config/#trayiconconfig>>

**对象属性** :

  * iconAsTemplate
  * iconPath (必需)
  * ID
  * menuOnLeftClick
  * showMenuOnLeftClick
  * 标题
  * tooltip

##### iconAsTemplate

“iconAsTemplate” 章节

`布尔值 (boolean)`

一个布尔值，用于确定图像是否代表 macOS 上的 [模板](https://developer.apple.com/documentation/appkit/nsimage/1520017-template?language=objc) 图像。

##### iconPath

“iconPath” 章节

`string`

用作托盘图标的默认图标路径。

注意：这会将原始像素格式的图像存储到最终二进制文件中，因此请保持图标尺寸（宽度和高度）较小，否则会使最终可执行文件体积膨胀。

##### ID

“id” 章节

`string` | `null`

为此托盘图标设置一个 ID，以便后续引用。默认为 `main`。

##### menuOnLeftClick

“menuOnLeftClick” 章节

`布尔值 (boolean)`

一个布尔值，用于确定当托盘图标收到左键点击时是否应显示菜单。

###### 平台特定

“Platform-specific:” 章节

  * **Linux** ：不支持。

**默认值** ：`true`

##### showMenuOnLeftClick

“showMenuOnLeftClick” 章节

`布尔值 (boolean)`

一个布尔值，用于确定当托盘图标收到左键点击时是否应显示菜单。

###### 平台特定

“Platform-specific:” 章节

  * **Linux** ：不支持。

**默认值** ：`true`

##### 标题

“title” 章节

`string` | `null`

macOS 托盘的标题

##### tooltip

“tooltip” 章节

`string` | `null`

Windows 和 macOS 上的托盘图标工具提示

### 更新器

“Updater” 章节

**以下任何一种** :

  * `V1Compatible` 生成兼容 v1 的旧版压缩更新器
  * `boolean` 是否生成更新器及其签名

更新器类型

### V1Compatible

“V1Compatible” 章节

`"v1Compatible"`，生成兼容 v1 的旧版压缩更新器

生成兼容 v1 的旧版压缩更新器

### 值

“Value” 章节

**以下任何一种** :

  * `null` 表示一个空 JSON 值。
  * `boolean` 表示一个 [`bool`] 值。
  * `Number` 表示一个有效的 ACL [`Number`] 值。
  * `string` 表示一个 [`String`] 值。
  * `Value`[] 表示其他 [`Value`] 值的列表。
  * 表示从 [`String`] 键到 [`Value`] 值的映射。**允许附加属性** ：`Value`

所有支持的 ACL 值。

### WebviewInstallMode

“WebviewInstallMode” 章节

**以下其中之一** :

  * 不要将 Webview2 作为 Windows 安装程序的一部分进行安装。**对象属性** ：- type（必需） ##### type `"skip"`
  * 下载引导程序并运行它。需要互联网连接。安装程序体积更小，但不建议在 Windows 7 上使用。**对象属性** ：- silent - type（必需） ##### silent `boolean` 指示安装程序以静默模式运行引导程序。默认为 `true`。**默认值** ：`true` ##### type `"downloadBootstrapper"`
  * 嵌入引导程序并运行它。需要互联网连接。安装程序体积增加约 1.8MB，但在 Windows 7 上提供更好的支持。**对象属性** ：- silent - type（必需） ##### silent `boolean` 指示安装程序以静默模式运行引导程序。默认为 `true`。**默认值** ：`true` ##### type `"embedBootstrapper"`
  * 嵌入离线安装程序并运行它。不需要互联网连接。安装程序体积增加约 127MB。**对象属性** ：- silent - type（必需） ##### silent `boolean` 指示安装程序以静默模式运行安装程序。默认为 `true`。**默认值** ：`true` ##### type `"offlineInstaller"`
  * 嵌入固定版本的 Webview2 并在运行时使用它。安装程序体积增加约 180MB。**对象属性** ：- path（必需） - type（必需） ##### path `string` 要使用的固定运行时的路径。固定版本可在 [官方网站](https://developer.microsoft.com/en-us/microsoft-edge/webview2/#download-section) 下载。必须将 `.cab` 文件解压到一个文件夹，并在此字段中定义该文件夹路径。 ##### type `"fixedRuntime"`

Webview2 运行时的安装模式。请注意，对于更新器捆绑包，使用的是 [`Self::DownloadBootstrapper`]。

更多信息请参见 <<https://v2.tauri.org.cn/distribute/windows-installer/#webview2-installation-options>>。

### WebviewUrl

“WebviewUrl” 章节

**以下任何一种** :

  * 格式为 `uri` 的 `string`。必须使用 `http` 或 `https` 协议。
  * `string` 应用 URL 的路径部分。例如，要加载 `tauri:///users/john`，您只需在此配置中提供 `users/john`。
  * 格式为 `uri` 的 `string`。自定义协议 URL，例如 `doom://index.html`

在 Tauri webview 窗口中打开的 URL。

### WindowConfig

“WindowConfig” 章节

窗口配置对象。

查看更多：<<https://v2.tauri.org.cn/reference/config/#windowconfig>>

**对象属性** :

  * acceptFirstMouse
  * additionalBrowserArgs
  * allowLinkPreview
  * alwaysOnBottom
  * alwaysOnTop
  * backgroundColor
  * backgroundThrottling
  * browserExtensionsEnabled
  * center
  * 可关闭
  * contentProtected
  * create
  * dataDirectory
  * dataStoreIdentifier
  * 装饰
  * devtools
  * disableInputAccessoryView
  * dragDropEnabled
  * focus
  * 可聚焦
  * 全屏
  * 高度
  * hiddenTitle
  * incognito
  * javascriptDisabled
  * 标签
  * maxHeight
  * 可最大化
  * maximized
  * maxWidth
  * minHeight
  * 可最小化
  * minWidth
  * parent
  * preventOverflow
  * proxyUrl
  * 可调整大小
  * scrollBarStyle
  * shadow
  * skipTaskbar
  * tabbingIdentifier
  * 主题
  * 标题
  * titleBarStyle
  * trafficLightPosition
  * transparent
  * url
  * useHttpsScheme
  * userAgent
  * visible
  * visibleOnAllWorkspaces
  * 宽度
  * windowClassname
  * windowEffects
  * x
  * y
  * zoomHotkeysEnabled

##### acceptFirstMouse

“acceptFirstMouse” 章节

`布尔值 (boolean)`

在 macOS 上，点击非活动窗口是否也会透传点击到 webview。

##### additionalBrowserArgs

“additionalBrowserArgs” 章节

`string` | `null`

定义 Windows 上的其他浏览器参数。默认情况下，wry 会传递 `--disable-features=msWebOOUI,msPdfOOUI,msSmartScreenProtection`，因此如果您使用此方法，如果需要，您也需要自行禁用这些组件。

##### allowLinkPreview

“allowLinkPreview” 章节

`布尔值 (boolean)`

在 macOS 和 iOS 上，长按链接时会有链接预览，默认启用此功能。详见 <https://docs.rs/objc2-web-kit/latest/objc2_web_kit/struct.WKWebView.html#method.allowsLinkPreview>

**默认值** ：`true`

##### alwaysOnBottom

“alwaysOnBottom” 章节

`布尔值 (boolean)`

窗口是否应始终位于其他窗口下方。

##### alwaysOnTop

“alwaysOnTop” 章节

`布尔值 (boolean)`

窗口是否应始终位于其他窗口之上。

##### backgroundColor

“backgroundColor” 章节

`Color` | `null`

设置窗口和 webview 的背景颜色。

###### 平台特定

“Platform-specific:” 章节

  * **Windows** ：Alpha 通道在窗口层被忽略。
  * **Windows** ：在 Windows 7 上，Alpha 通道在 webview 层被忽略。
  * **Windows** ：在 Windows 8 及更高版本上，如果 Alpha 通道不为 `0`，它将在 webview 层被忽略。

##### backgroundThrottling

“backgroundThrottling” 章节

`BackgroundThrottlingPolicy` | `null`

更改默认的后台节流行为。

默认情况下，浏览器使用挂起策略，即当视图最小化或隐藏约 5 分钟后，浏览器会节流计时器甚至卸载整个选项卡（视图）以释放资源。这会暂停所有任务，直到文档可见性状态通过将视图带回前台而从隐藏变为可见。

###### 平台特定

名为“平台特定”的章节

  * **Linux / Windows / Android** ：不支持。像挂起的 WebLock 事务这样的变通方法可能足够。
  * **iOS** ：自 17.0+ 版本起支持。
  * **macOS** ：自 14.0+ 版本起支持。

详见 <https://github.com/tauri-apps/tauri/issues/5250#issuecomment-2569380578>

##### browserExtensionsEnabled

“browserExtensionsEnabled” 章节

`布尔值 (boolean)`

是否可以为 webview 进程安装浏览器扩展。

###### 平台特定

“Platform-specific:” 章节

  * **Windows** ：启用 WebView2 环境的 [`AreBrowserExtensionsEnabled`](https://learn.microsoft.com/en-us/microsoft-edge/webview2/reference/winrt/microsoft_web_webview2_core/corewebview2environmentoptions?view=webview2-winrt-1.0.2739.15#arebrowserextensionsenabled)。
  * **MacOS / Linux / iOS / Android** ：不支持。

##### center

“center” 章节

`布尔值 (boolean)`

窗口是否居中启动。

##### 可关闭

“closable” 章节

`布尔值 (boolean)`

窗口的原生关闭按钮是否启用。

###### 平台特定

名为“平台特定”的章节

  * **Linux** ：“GTK+ 将尽力说服窗口管理器不要显示关闭按钮。根据系统的不同，此函数在已可见的窗口上调用时可能无效。”
  * **iOS / Android:** 不支持。

**默认值** ：`true`

##### contentProtected

“contentProtected” 章节

`布尔值 (boolean)`

防止窗口内容被其他应用程序捕获。

##### create

“create” 章节

`布尔值 (boolean)`

Tauri 是否应该在应用启动时创建此窗口。

当此项设置为 `false` 时，您必须手动通过 `app.config().app.windows` 获取配置对象，并使用 [`WebviewWindowBuilder::from_config`](https://docs.rs/tauri/2/tauri/webview/struct.WebviewWindowBuilder.html#method.from_config) 进行创建。

###### 示例

名为“示例：”的章节

    tauri::Builder::default()

      .setup(|app| {

        tauri::WebviewWindowBuilder::from_config(app.handle(), &app.config().app.windows[0])?.build()?;

        Ok(())

      });

**默认值** ：`true`

##### dataDirectory

“dataDirectory” 章节

`string` | `null`

设置 webview 数据目录（localStorage、缓存等）的自定义路径，**相对于 [`appDataDir()`]/${label}**。

要设置绝对路径，请使用 [`WebviewWindowBuilder::data_directory`](https://docs.rs/tauri/2/tauri/webview/struct.WebviewWindowBuilder.html#method.data_directory)

###### 平台特定

“Platform-specific:” 章节

  * **Windows** ：对于诸如 `additionalBrowserArgs`、`browserExtensionsEnabled` 或 `scrollBarStyle` 等设置具有不同值的 WebViews，必须使用不同的数据目录。
  * **macOS / iOS** ：不支持，请改用 `dataStoreIdentifier`。
  * **Android** ：不支持。

##### dataStoreIdentifier

“dataStoreIdentifier” 章节

`integer` 格式为 `uint8`[] | `null`，最大 `16` 项，最小 `16` 项

使用自定义数据存储标识符初始化 WebView。这可以看作是 `dataDirectory` 的替代方案，而 `dataDirectory` 在 WKWebView 中不可用。详见 <https://developer.apple.com/documentation/webkit/wkwebsitedatastore/init(foridentifier:)?language=objc>

数组必须包含 16 个 u8 数字。

###### 平台特定

“Platform-specific:” 章节

  * **iOS** ：自 17.0+ 版本起支持。
  * **macOS** ：自 14.0+ 版本起支持。
  * **Windows / Linux / Android** ：不支持。

##### 装饰

“decorations” 章节

`布尔值 (boolean)`

窗口是否应该有边框和标题栏。

**默认值** ：`true`

##### devtools

“devtools” 章节

`boolean` | `null`

启用通常称为浏览器开发工具的 Web 检查器。默认启用。

此 API 在 **调试 (debug)** 构建中有效，但在 **发布 (release)** 构建中需要 `devtools` 功能标志才能启用。

###### 平台特定

名为“平台特定”的章节

  * macOS：这将在 **macOS** 上调用私有函数。
  * Android：在 Chrome 中打开 `chrome://inspect/#devices` 以获取 devtools 窗口。Android 不支持 Wry 的 `WebView` devtools API。
  * iOS：打开 Safari > 开发 > [您的设备名称] > [您的 WebView] 以获取 devtools 窗口。

##### disableInputAccessoryView

“disableInputAccessoryView” 章节

`布尔值 (boolean)`

允许禁用 iOS 上的输入附件视图。

附件视图是文本输入元素聚焦时出现在键盘上方的视图。它通常显示带有“完成”、“下一步”按钮的视图。

##### dragDropEnabled

“dragDropEnabled” 章节

`布尔值 (boolean)`

webview 上是否启用了拖放功能。默认情况下是启用的。

在 Windows 上，使用前端的 HTML5 拖放功能需要禁用它。

**默认值** ：`true`

##### focus

“focus” 章节

`布尔值 (boolean)`

窗口是否在初始时获得焦点。

**默认值** ：`true`

##### 可聚焦

“focusable” 章节

`布尔值 (boolean)`

窗口是否可聚焦。

**默认值** ：`true`

##### 全屏

“fullscreen” 章节

`布尔值 (boolean)`

窗口是否全屏启动。

##### 高度

标题为“height”的章节

`number` 格式为 `double`

以逻辑像素为单位的窗口高度。

**默认值** ：`600`

##### hiddenTitle

“hiddenTitle” 章节

`布尔值 (boolean)`

如果为 `true`，则设置在 macOS 上隐藏窗口标题。

##### incognito

“incognito” 章节

`布尔值 (boolean)`

webview 是否应以无痕模式启动。

###### 平台特定

“Platform-specific:” 章节

  * **Android** ：不支持。

##### javascriptDisabled

“javascriptDisabled” 章节

`布尔值 (boolean)`

我们是否应该禁用 webview 上的 JavaScript 代码执行。

##### 标签

“label” 章节

`string`

窗口标识符。必须是字母数字。

**默认值** ：`"main"`

##### maxHeight

“maxHeight” 章节

`number` | `null` 格式为 `double`

以逻辑像素为单位的最大窗口高度。

##### 可最大化

“maximizable” 章节

`布尔值 (boolean)`

是否启用窗口的原生最大化按钮。如果 resizable 设置为 false，则忽略此设置。

###### 平台特定

名为“平台特定”的章节

  * **macOS：** 禁用窗口标题栏中的“缩放”按钮，该按钮也用于进入全屏模式。
  * **Linux / iOS / Android：** 不支持。

**默认值** ：`true`

##### maximized

“maximized” 章节

`布尔值 (boolean)`

窗口是否最大化。

##### maxWidth

“maxWidth” 章节

`number` | `null` 格式为 `double`

以逻辑像素为单位的最大窗口宽度。

##### minHeight

“minHeight” 章节

`number` | `null` 格式为 `double`

以逻辑像素为单位的最小窗口高度。

##### 可最小化

“minimizable” 章节

`布尔值 (boolean)`

窗口的原生最小化按钮是否启用。

###### 平台特定

名为“平台特定”的章节

  * **Linux / iOS / Android：** 不支持。

**默认值** ：`true`

##### minWidth

“minWidth” 章节

`number` | `null` 格式为 `double`

以逻辑像素为单位的最小窗口宽度。

##### parent

标题为“parent”的章节

`string` | `null`

将与此标签关联的窗口设置为待创建窗口的父窗口。

###### 平台特定

名为“平台特定”的章节

  * **Windows** ：将传入的父窗口设置为待创建窗口的所有者窗口 (owner window)。参考 [MSDN 关于所有者窗口的文档](https://docs.microsoft.com/en-us/windows/win32/winmsg/window-features#owned-windows)
    * 所有者窗口始终位于其拥有者窗口的 Z 序上方。
    * 当所有者窗口被销毁时，系统会自动销毁其拥有者窗口。
    * 当所有者窗口被最小化时，其拥有者窗口会被隐藏。
  * **Linux** ：使新窗口成为父窗口的瞬态窗口 (transient)，详见 <<https://docs.gtk.org.cn/gtk3/method.Window.set_transient_for.html>>
  * **macOS** ：将窗口添加为父窗口的子窗口，详见 <<https://developer.apple.com/documentation/appkit/nswindow/1419152-addchildwindow?language=objc>>

##### preventOverflow

标题为“preventOverflow”的章节

`PreventOverflowConfig` | `null`

是否防止窗口超出工作区范围

###### 平台特定

名为“平台特定”的章节

  * **iOS / Android:** 不支持。

##### proxyUrl

标题为“proxyUrl”的章节

`string` | `null`，格式为 `uri`

WebView 所有网络请求的代理 URL。

必须是 `http://` 或 `socks5://` URL。

###### 平台特定

名为“平台特定”的章节

  * **macOS** ：需要 `macos-proxy` 功能标志，且仅在 macOS 14+ 上编译。

##### 可调整大小

标题为“resizable”的章节

`布尔值 (boolean)`

窗口是否可调整大小。当 resizable 设置为 false 时，原生窗口的最大化按钮将自动禁用。

**默认值** ：`true`

##### scrollBarStyle

标题为“scrollBarStyle”的章节

`ScrollBarStyle`

指定 Webview 使用的原生滚动条样式。修改滚动条的 CSS 样式会应用在此处配置的原生外观之上。

默认为 `default`，即浏览器默认样式。

###### 平台特定

名为“平台特定”的章节

  * **Windows** :
    * `fluentOverlay` 需要 WebView2 Runtime 125.0.2535.41 或更高版本，在旧版本上无效。
    * 对于指向同一数据目录的所有 Webview，此选项必须设置为相同的值。
  * **Linux / Android / iOS / macOS** ：不支持。仅支持 `Default` 且不执行任何操作。

**默认值** ：`"default"`

##### shadow

标题为“shadow”的章节

`布尔值 (boolean)`

窗口是否有阴影。

###### 平台特定

名为“平台特定”的章节

  * **Windows**
    * `false` 对装饰性窗口没有影响，阴影始终开启。
    * `true` 将使无装饰窗口有 1 像素的白色边框，在 Windows 11 上，它将有圆角。
  * **Linux：** 不支持。

**默认值** ：`true`

##### skipTaskbar

标题为“skipTaskbar”的章节

`布尔值 (boolean)`

如果为 `true`，则在 Windows 和 Linux 上从任务栏隐藏窗口图标。

##### tabbingIdentifier

标题为“tabbingIdentifier”的章节

`string` | `null`

定义 macOS 的窗口[标签标识符 (tabbing identifier)](%3Chttps://developer.apple.com/documentation/appkit/nswindow/1644704-tabbingidentifier%3E)。

具有相同标签标识符的窗口将被组合在一起。如果未设置标签标识符，自动分页功能将被禁用。

##### 主题

标题为“theme”的章节

`Theme` | `null`

初始窗口主题。默认为系统主题。仅在 Windows 和 macOS 10.14+ 上实现。

##### 标题

“title” 章节

`string`

窗口标题。

**默认值** ：`"Tauri App"`

##### titleBarStyle

标题为“titleBarStyle”的章节

`TitleBarStyle`

macOS 标题栏的样式。

**默认值** ：`"Visible"`

##### trafficLightPosition

标题为“trafficLightPosition”的章节

`LogicalPosition` | `null`

macOS 上窗口控件的位置。

需要 titleBarStyle: Overlay 和 decorations: true。

##### transparent

标题为“transparent”的章节

`布尔值 (boolean)`

窗口是否透明。

请注意，在 `macOS` 上这需要启用 `macos-private-api` 功能标志，在 `tauri > macOSPrivateApi` 下设置。警告：在 `macOS` 上使用私有 API 会导致您的应用程序无法上架 `App Store`。

##### url

标题为“url”的章节

`WebviewUrl`

窗口 Webview 的 URL。

**默认值** ：`"index.html"`

##### useHttpsScheme

标题为“useHttpsScheme”的章节

`布尔值 (boolean)`

设置自定义协议在 Windows 和 Android 上是否应使用 `https://<scheme>.localhost` 而不是默认的 `http://<scheme>.localhost`。默认为 `false`。

###### 注意

标题为“注意”的章节

使用 `https` 方案在尝试获取 `http` 端点时将不允许混合内容，因此不会与 macOS 和 Linux 上使用的 `<scheme>://` 协议行为一致。

###### 警告

标题为“警告”的章节

在版本发布之间更改此值会更改 IndexedDB、Cookie 和 LocalStorage 的位置，导致您的应用程序无法访问旧数据。

##### userAgent

标题为“userAgent”的章节

`string` | `null`

Webview 的用户代理 (User Agent)。

##### visible

标题为“visible”的章节

`布尔值 (boolean)`

窗口是否可见。

**默认值** ：`true`

##### visibleOnAllWorkspaces

标题为“visibleOnAllWorkspaces”的章节

`布尔值 (boolean)`

窗口是否应在所有工作区或虚拟桌面上可见。

###### 平台特定

名为“平台特定”的章节

  * **Windows / iOS / Android：** 不支持。

##### 宽度

标题为“width”的章节

`number` 格式为 `double`

窗口的逻辑像素宽度。

**默认值** ：`800`

##### windowClassname

标题为“windowClassname”的章节

`string` | `null`

Windows 上用于创建窗口的窗口类名。**仅限 Windows** 。

##### windowEffects

标题为“windowEffects”的章节

`WindowEffectsConfig` | `null`

窗口效果。

要求窗口为透明。

###### 平台特定

“Platform-specific:” 章节

  * **Windows** ：如果使用装饰或阴影，您可能需要尝试此变通方法 <<https://github.com/tauri-apps/tao/issues/72#issuecomment-975607891>>
  * **Linux** ：不支持

##### x

标题为“x”的章节

`number` | `null` 格式为 `double`

窗口左上角的水平位置（以逻辑像素为单位）

##### y

标题为“y”的章节

`number` | `null` 格式为 `double`

窗口左上角的垂直位置（以逻辑像素为单位）

##### zoomHotkeysEnabled

标题为“zoomHotkeysEnabled”的章节

`布尔值 (boolean)`

是否启用通过热键进行页面缩放

###### 平台特定

“Platform-specific:” 章节

  * **Windows** ：控制 WebView2 的 [`IsZoomControlEnabled`](https://learn.microsoft.com/en-us/microsoft-edge/webview2/reference/winrt/microsoft_web_webview2_core/corewebview2settings?view=webview2-winrt-1.0.2420.47#iszoomcontrolenabled) 设置。

  * **MacOS / Linux** ：注入一个 polyfill，使用 `ctrl/command` \+ `-/=` 进行缩放，每步 20%，范围从 20% 到 1000%。需要 `webview:allow-set-webview-zoom` 权限。

  * **Android / iOS** ：不支持。

### WindowEffect

标题为“WindowEffect”的章节

**以下其中之一** :

  * `"appearanceBased"` 适合视图 effectiveAppearance 的默认材质。**macOS 10.14-**
  * `"light"` **macOS 10.14-**
  * `"dark"` **macOS 10.14-**
  * `"mediumLight"` **macOS 10.14-**
  * `"ultraDark"` **macOS 10.14-**
  * `"titlebar"` **macOS 10.10+**
  * `"selection"` **macOS 10.10+**
  * `"menu"` **macOS 10.11+**
  * `"popover"` **macOS 10.11+**
  * `"sidebar"` **macOS 10.11+**
  * `"headerView"` **macOS 10.14+**
  * `"sheet"` **macOS 10.14+**
  * `"windowBackground"` **macOS 10.14+**
  * `"hudWindow"` **macOS 10.14+**
  * `"fullScreenUI"` **macOS 10.14+**
  * `"tooltip"` **macOS 10.14+**
  * `"contentBackground"` **macOS 10.14+**
  * `"underWindowBackground"` **macOS 10.14+**
  * `"underPageBackground"` **macOS 10.14+**
  * `"mica"` 符合系统深色偏好的 Mica 效果 **仅限 Windows 11**
  * `"micaDark"` 带有深色模式的 Mica 效果，但仅在系统启用深色模式时有效 **仅限 Windows 11**
  * `"micaLight"` 浅色模式的 Mica 效果 **仅限 Windows 11**
  * `"tabbed"` 符合系统深色偏好的标签页效果 **仅限 Windows 11**
  * `"tabbedDark"` 带有深色模式的标签页效果，但仅在系统启用深色模式时有效 **仅限 Windows 11**
  * `"tabbedLight"` 浅色模式的标签页效果 **仅限 Windows 11**
  * `"blur"` **仅限 Windows 7/10/11(22H1)** ##### 注意 在 Windows 11 build 22621 上调整大小/拖动窗口时，此效果性能较差。
  * `"acrylic"` **仅限 Windows 10/11** ##### 注意 在 Windows 10 v1903+ 和 Windows 11 build 22000 上调整大小/拖动窗口时，此效果性能较差。

平台特定的窗口效果

### WindowEffectsConfig

标题为“WindowEffectsConfig”的章节

窗口效果配置对象

**对象属性** :

  * 颜色
  * effects (必需)
  * radius
  * state

##### 颜色

标题为“color”的章节

`Color` | `null`

窗口效果颜色。仅影响 Windows 10 v1903+ 上的 [`WindowEffect::Blur`] 和 [`WindowEffect::Acrylic`]。在 Windows 7 或 Windows 11 上没有效果。

##### 效果

标题为“effects”的章节

`WindowEffect`[]

应用于窗口的窗口效果列表。冲突的效果将应用第一个并忽略其余效果。

##### radius

标题为“radius”的章节

`number` | `null` 格式为 `double`

窗口效果圆角半径 **仅限 macOS**

##### state

标题为“state”的章节

`WindowEffectState` | `null`

窗口效果状态 **仅限 macOS**

### WindowEffectState

标题为“WindowEffectState”的章节

**以下其中之一** :

  * `"followsWindowActiveState"` 使窗口效果状态跟随窗口的活动状态
  * `"active"` 使窗口效果状态始终处于活动状态
  * `"inactive"` 使窗口效果状态始终处于非活动状态

窗口效果状态 **仅限 macOS**

<<https://developer.apple.com/documentation/appkit/nsvisualeffectview/state>>

### WindowsConfig

标题为“WindowsConfig”的章节

Windows 打包器配置。

了解更多：<<https://v2.tauri.org.cn/reference/config/#windowsconfig>>

**对象属性** :

  * allowDowngrades
  * certificateThumbprint
  * digestAlgorithm
  * nsis
  * signCommand
  * timestampUrl
  * tsp
  * webviewInstallMode
  * wix

##### allowDowngrades

标题为“allowDowngrades”的章节

`布尔值 (boolean)`

验证二次安装，如果设置为 `false`，将阻止用户安装旧版本。

例如，如果安装了 `1.2.1`，用户将无法安装 `1.2.0` 或 `1.1.5` 版本的应用。

此标志的默认值为 `true`。

**默认值** ：`true`

##### certificateThumbprint

标题为“certificateThumbprint”的章节

`string` | `null`

指定签名证书的 SHA1 哈希值。

##### digestAlgorithm

标题为“digestAlgorithm”的章节

`string` | `null`

指定用于创建文件签名的文件摘要算法。代码签名必需。建议使用 SHA-256。

##### nsis

标题为“nsis”的章节

`NsisConfig` | `null`

使用 NSIS 生成的安装程序配置。

##### signCommand

标题为“signCommand”的章节

`CustomSignCommandConfig` | `null`

指定用于对二进制文件进行签名的自定义命令。此命令的参数中需要包含 `%1` 作为二进制文件路径的占位符，我们会在调用命令前检测并替换它。

默认情况下，我们使用仅在 Windows 上可用的 `signtool.exe`。因此，如果您在其他平台上并希望进行交叉编译和签名，则需要使用其他工具，如 `osslsigncode`。

##### timestampUrl

标题为“timestampUrl”的章节

`string` | `null`

时间戳期间使用的服务器。

##### tsp

标题为“tsp”的章节

`布尔值 (boolean)`

是否对时间戳服务器使用时间戳协议 (TSP，又称 RFC 3161)。您的代码签名提供商可能使用 TSP 时间戳服务器（例如 SSL.com）。如果是，请通过设置为 true 来启用 TSP。

##### webviewInstallMode

标题为“webviewInstallMode”的章节

`WebviewInstallMode`

Webview2 运行时的安装模式。

默认

    {

      "silent": true,

      "type": "downloadBootstrapper"

    }

##### wix

标题为“wix”的章节

`WixConfig` | `null`

使用 WiX 生成的 MSI 配置。

### WixConfig

标题为“WixConfig”的章节

使用 WiX 的 MSI 捆绑包配置。

了解更多：<<https://v2.tauri.org.cn/reference/config/#wixconfig>>

**对象属性** :

  * bannerPath
  * componentGroupRefs
  * componentRefs
  * dialogImagePath
  * enableElevatedUpdateTask
  * featureGroupRefs
  * featureRefs
  * fipsCompliant
  * fragmentPaths
  * language
  * mergeRefs
  * template
  * upgradeCode
  * version

##### bannerPath

标题为“bannerPath”的章节

`string` | `null`

用作安装程序用户界面横幅的位图文件路径。此位图将出现在除安装程序第一页外的所有页面顶部。

所需尺寸为 493px × 58px。

##### componentGroupRefs

标题为“componentGroupRefs”的章节

`string`[]

您希望从片段 (fragments) 中引用的 ComponentGroup 元素 ID。

**默认** : `[]`

##### componentRefs

标题为“componentRefs”的章节

`string`[]

您希望从片段中引用的 Component 元素 ID。

**默认** : `[]`

##### dialogImagePath

标题为“dialogImagePath”的章节

`string` | `null`

用于安装程序用户界面对话框的位图文件路径。它用于欢迎和完成对话框。

所需尺寸为 493px × 312px。

##### enableElevatedUpdateTask

标题为“enableElevatedUpdateTask”的章节

`布尔值 (boolean)`

在 Windows 任务计划程序中创建提升权限的更新任务。

##### featureGroupRefs

标题为“featureGroupRefs”的章节

`string`[]

您希望从片段中引用的 FeatureGroup 元素 ID。

**默认** : `[]`

##### featureRefs

标题为“featureRefs”的章节

`string`[]

您希望从片段中引用的 Feature 元素 ID。

**默认** : `[]`

##### fipsCompliant

标题为“fipsCompliant”的章节

`布尔值 (boolean)`

启用符合 FIPS 标准的算法。也可以通过 `TAURI_BUNDLER_WIX_FIPS_COMPLIANT` 环境变量启用。

##### fragmentPaths

标题为“fragmentPaths”的章节

`string`[]

包含要使用的 WiX 片段的 .wxs 文件路径列表。

**默认** : `[]`

##### language

标题为“language”的章节

`WixLanguage`

要构建的安装程序语言。详见 <<https://docs.microsoft.com/en-us/windows/win32/msi/localizing-the-error-and-actiontext-tables>>。

**默认值** ：`"en-US"`

##### mergeRefs

标题为“mergeRefs”的章节

`string`[]

您希望从片段中引用的 Merge 元素 ID。

**默认** : `[]`

##### template

标题为“template”的章节

`string` | `null`

要使用的自定义 .wxs 模板。

##### upgradeCode

标题为“upgradeCode”的章节

`string` | `null` 格式化为 `uuid`

用于 MSI 安装程序的 GUID 升级代码。此代码** _在所有更新中必须保持不变_** ，否则 Windows 会将您的更新视为不同的应用程序，您的用户将会拥有该应用程序的重复版本。

默认情况下，Tauri 会通过在 DNS 命名空间中使用字符串 `<productName>.exe.app.x64` 生成 Uuid v5 来生成此代码。您可以使用 Tauri 的 CLI 为您生成并打印此代码，运行 `tauri inspect wix-upgrade-code`。

建议您在 Tauri 配置文件中设置此值，以避免在更改产品名称时意外更改升级代码。

##### version

名为“version”的章节

`string` | `null`

格式为 `major.minor.patch.build`（build 为可选）的 MSI 安装程序版本。

由于 MSI 安装程序需要有效的版本号，如果未设置此字段，将从 [`Config::version`] 派生。

第一个字段为主版本号，最大值为 255。第二个字段为次版本号，最大值为 255。第三和第四个字段的最大值为 65,535。

更多信息请参见 <<https://learn.microsoft.com/en-us/windows/win32/msi/productversion>>。

### WixLanguage

标题为“WixLanguage”的章节

**以下任何一种** :

  * `string` 要构建的单一语言，不带配置。
  * `string`[] 要构建的语言列表，不带配置。
  * 语言及其配置的映射。**允许其他属性** ：`WixLanguageConfig`

使用 WiX 构建的语言。

### WixLanguageConfig

标题为“WixLanguageConfig”的章节

WiX 构建目标语言的配置。

了解更多：<<https://v2.tauri.org.cn/reference/config/#wixlanguageconfig>>

**对象属性** :

  * localePath

##### localePath

标题为“localePath”的章节

`string` | `null`

语言环境 (`.wxl`) 文件的路径。详见 <<https://wixtoolset.org/documentation/manual/v3/howtos/ui_and_localization/build_a_localized_version.html>>。