# Windows 安装程序

_Source: https://v2.tauri.org.cn/distribute/windows-installer/_

Tauri Windows 应用程序可以通过 [WiX Toolset v3](https://wixtoolset.org/documentation/manual/v3/) 分发为 Microsoft 安装程序（`.msi` 文件），或者通过 [NSIS](https://nsis.sourceforge.io/Main_Page) 分发为设置可执行文件（`-setup.exe` 文件）。

请注意，`.msi` 安装程序**只能在 Windows 上创建** ，因为 WiX 只能在 Windows 系统上运行。NSIS 安装程序的交叉编译方法如下所示。

本指南提供了有关安装程序可用自定义选项的信息。

## 构建

名为“构建”的章节

要将您的应用程序构建并打包成 Windows 安装程序，您可以使用 Tauri CLI 并在 Windows 计算机上运行 `tauri build` 命令。

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri build

    yarn tauri build

    pnpm tauri build

    deno task tauri build

    bun tauri build

    cargo tauri build

MSI 包的 VBSCRIPT 要求

构建 MSI 包（在 `tauri.conf.json` 中设置 `"targets": "msi"` 或 `"targets": "all"`）需要在 Windows 上启用可选的 VBSCRIPT 功能。此功能在大多数 Windows 安装中默认启用，但如果您遇到类似 `failed to run light.exe` 的错误，您可能需要通过**设置** → **应用** → **可选功能** → **更多 Windows 功能** 手动启用它。有关详细说明，请参阅[先决条件指南](/start/prerequisites/#vbscript-for-msi-installers)。

### 在 Linux 和 macOS 上构建 Windows 应用

标题为“在 Linux 和 macOS 上构建 Windows 应用”的章节

使用 [NSIS](https://nsis.sourceforge.io/Main_Page) 时，可以在 Linux 和 macOS 主机上交叉编译 Windows 应用程序，但有一些限制。它不像在 Windows 上直接编译那样简单，并且没有经过大量的测试。因此，只有在本地虚拟机或 GitHub Actions 等 CI 解决方案不适用于您的情况下，才应将其作为最后的手段。

注意

对交叉编译的 Windows 安装程序进行签名需要外部签名工具。有关更多信息，请参阅[签名文档](/distribute/sign/windows/)。

由于 Tauri 官方仅支持 MSVC Windows 目标，因此设置过程稍微复杂一些。

#### 安装 NSIS

标题为“安装 NSIS”的章节

  * Linux
  * macOS

一些 Linux 发行版的存储库中提供了 NSIS，例如在 Ubuntu 上，您可以通过运行此命令安装 NSIS

Ubuntu

    sudo apt install nsis

但在许多其他发行版上，您必须自己编译 NSIS，或者手动下载发行版二进制包中未包含的存根 (Stubs) 和插件。例如，Fedora 只提供了二进制文件，而不提供存根和插件。

Fedora

    sudo dnf in mingw64-nsis

    wget https://github.com/tauri-apps/binary-releases/releases/download/nsis-3/nsis-3.zip

    unzip nsis-3.zip

    sudo cp nsis-3.08/Stubs/* /usr/share/nsis/Stubs/

    sudo cp -r nsis-3.08/Plugins/** /usr/share/nsis/Plugins/

在 macOS 上，您需要使用 Homebrew 来安装 NSIS。

macOS

    brew install nsis

#### 安装 LLVM 和 LLD 链接器

标题为“安装 LLVM 和 LLD 链接器”的章节

由于默认的 Microsoft 链接器仅在 Windows 上工作，我们还需要安装一个新的链接器。为了编译用于设置应用程序图标（以及其他事项）的 Windows 资源文件，我们还需要 `llvm-rc` 二进制文件，它是 LLVM 项目的一部分。

  * Linux
  * macOS

Ubuntu

    sudo apt install lld llvm

在 Linux 上，如果您添加了作为构建脚本一部分编译 C/C++ 依赖项的依赖项，您还需要安装 `clang` 软件包。默认的 Tauri 应用程序不应需要此项。

macOS

    brew install llvm

在 macOS 上，您还必须按照安装输出的建议，将 `/opt/homebrew/opt/llvm/bin` 添加到您的 `$PATH` 中。

#### 安装 Windows Rust 目标

标题为“安装 Windows Rust 目标”的章节

假设您正在为 64 位 Windows 系统构建

终端窗口

    rustup target add x86_64-pc-windows-msvc

#### 安装 `cargo-xwin`

标题为“安装 cargo-xwin”的章节

与其手动设置 Windows SDK，不如使用 [`cargo-xwin`] 作为 Tauri 的“运行器 (runner)”。

终端窗口

    cargo install --locked cargo-xwin

默认情况下，`cargo-xwin` 会将 Windows SDK 下载到项目本地文件夹中。如果您有多个项目并希望共享这些文件，则可以设置 `XWIN_CACHE_DIR` 环境变量，并将其路径指向首选位置。

#### 构建应用程序

标题为“构建应用程序”的章节

现在，只需将运行器和目标添加到 `tauri build` 命令即可。

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri build -- --runner cargo-xwin --target x86_64-pc-windows-msvc

    yarn tauri build --runner cargo-xwin --target x86_64-pc-windows-msvc

    pnpm tauri build --runner cargo-xwin --target x86_64-pc-windows-msvc

    deno task tauri build --runner cargo-xwin --target x86_64-pc-windows-msvc

    bun tauri build --runner cargo-xwin --target x86_64-pc-windows-msvc

    cargo tauri build --runner cargo-xwin --target x86_64-pc-windows-msvc

构建输出将位于 `target/x86_64-pc-windows-msvc/release/bundle/nsis/`。

### 构建 32 位或 ARM 版本

标题为“构建 32 位或 ARM 版本”的章节

默认情况下，Tauri CLI 使用您机器的架构编译您的可执行文件。假设您是在 64 位机器上进行开发，CLI 将生成 64 位应用程序。

如果您需要支持 **32 位** 机器，可以使用 `--target` 标志使用**不同** 的 [Rust 目标](https://doc.rust-lang.net.cn/nightly/rustc/platform-support.html)来编译您的应用程序。

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri build -- --target i686-pc-windows-msvc

    yarn tauri build --target i686-pc-windows-msvc

    pnpm tauri build --target i686-pc-windows-msvc

    deno task tauri build --target i686-pc-windows-msvc

    bun tauri build --target i686-pc-windows-msvc

    cargo tauri build --target i686-pc-windows-msvc

默认情况下，Rust 仅安装针对您机器目标配置的工具链，因此您需要先安装 32 位 Windows 工具链：`rustup target add i686-pc-windows-msvc`。

如果您需要为 **ARM64** 构建，则首先需要安装额外的构建工具。为此，请打开 `Visual Studio Installer`，单击“修改”，然后在“单个组件”选项卡中安装“C++ ARM64 build tools”。在撰写本文时，VS2022 中的确切名称是 `MSVC v143 - VS 2022 C++ ARM64 build tools (Latest)`。现在，您可以使用 `rustup target add aarch64-pc-windows-msvc` 添加 rust 目标，然后使用上述方法编译您的应用程序。

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri build -- --target aarch64-pc-windows-msvc

    yarn tauri build --target aarch64-pc-windows-msvc

    pnpm tauri build --target aarch64-pc-windows-msvc

    deno task tauri build --target aarch64-pc-windows-msvc

    bun tauri build --target aarch64-pc-windows-msvc

    cargo tauri build --target aarch64-pc-windows-msvc

注意

请注意，NSIS 安装程序本身仍将是 x86 架构，并通过模拟在 ARM 机器上运行。应用程序本身将是原生的 ARM64 二进制文件。

## 支持 Windows 7

标题为“支持 Windows 7”的章节

默认情况下，Microsoft 安装程序（`.msi`）不能在 Windows 7 上工作，因为它需要在未安装时下载 WebView2 引导程序（如果操作系统中未启用 TLS 1.2，这可能会失败）。Tauri 包含一个嵌入 WebView2 引导程序的选项（请参阅下文“嵌入 WebView2 引导程序”一节）。基于 NSIS 的安装程序（`-setup.exe`）也支持 Windows 7 上的 `downloadBootstrapper` 模式。

此外，要在 Windows 7 中使用通知 API，您需要启用 `windows7-compat` Cargo 功能。

Cargo.toml

    [dependencies]

    tauri-plugin-notification = { version = "2.0.0", features = [ "windows7-compat" ] }

## FIPS 合规性

标题为“FIPS 合规性”的章节

如果您的系统要求 MSI 包符合 FIPS，您可以在运行 `tauri build` 之前将 `TAURI_BUNDLER_WIX_FIPS_COMPLIANT` 环境变量设置为 `true`。在 PowerShell 中，您可以像这样为当前终端会话设置它。

终端窗口

    $env:TAURI_BUNDLER_WIX_FIPS_COMPLIANT="true"

## WebView2 安装选项

标题为“WebView2 安装选项”的章节

安装程序默认下载 WebView2 引导程序，如果未安装运行时，则执行它。或者，您可以嵌入引导程序、嵌入离线安装程序或使用固定的 WebView2 运行时版本。请查看下表以了解这些方法之间的比较。

安装方法| 需要互联网连接？| 额外的安装程序大小| 备注
---|---|---|---
`downloadBootstrapper`| 是| 0MB| `默认`
安装程序体积较小，但不建议用于通过 `.msi` 文件在 Windows 7 上部署。
`embedBootstrapper`| 是| ~1.8MB| 在 Windows 7 上为 `.msi` 安装程序提供更好的支持。
`offlineInstaller`| 否| ~127MB| 嵌入 WebView2 安装程序。推荐用于离线环境。
`fixedVersion`| 否| ~180MB| 嵌入固定的 WebView2 版本。
`跳过`| 否| 0MB| ⚠️ 不推荐
不作为 Windows 安装程序的一部分安装 WebView2。

注意

在 Windows 10（2018 年 4 月更新或更高版本）和 Windows 11 上，WebView2 运行时作为操作系统的一部分进行分发。

### 下载引导程序 (Bootstrapper)

标题为“下载引导程序”的章节

这是构建 Windows 安装程序的默认设置。它会下载引导程序并运行它。需要互联网连接，但安装程序体积较小。如果您打算通过 `.msi` 安装程序分发到 Windows 7，则不推荐此设置。

tauri.conf.json

    {

      "bundle": {

        "windows": {

          "webviewInstallMode": {

            "type": "downloadBootstrapper"

          }

        }

      }

    }

### 嵌入式引导程序

标题为“嵌入式引导程序”的章节

要嵌入 WebView2 引导程序，请将 [webviewInstallMode](/reference/config/#webviewinstallmode) 设置为 `embedBootstrapper`。这会将安装程序大小增加约 1.8MB，但会提高在 Windows 7 系统上的兼容性。

tauri.conf.json

    {

      "bundle": {

        "windows": {

          "webviewInstallMode": {

            "type": "embedBootstrapper"

          }

        }

      }

    }

### 离线安装程序

标题为“离线安装程序”的章节

要嵌入 WebView2 离线安装程序，请将 [webviewInstallMode](/reference/config/#webviewinstallmode) 设置为 `offlineInstaller`。这会将安装程序大小增加约 127MB，但允许即使在没有互联网连接的情况下也能安装您的应用程序。

tauri.conf.json

    {

      "bundle": {

        "windows": {

          "webviewInstallMode": {

            "type": "offlineInstaller"

          }

        }

      }

    }

### 固定版本 (Fixed Version)

标题为“固定版本”的章节

使用系统提供的运行时对于安全性非常有益，因为 webview 漏洞补丁由 Windows 管理。如果您想控制每个应用程序上的 WebView2 分发（无论是为了自行管理发布补丁，还是在可能没有互联网连接的环境中分发应用程序），Tauri 可以为您打包运行时文件。

注意

分发固定的 WebView2 运行时版本会将 Windows 安装程序增大约 180MB。

  1. 从 [Microsoft 网站](https://developer.microsoft.com/en-us/microsoft-edge/webview2/#download-section)下载 WebView2 固定版本运行时。在此示例中，下载的文件名为 `Microsoft.WebView2.FixedVersionRuntime.128.0.2739.42.x64.cab`。
  2. 将文件提取到 core 文件夹中。

终端窗口

    Expand .\Microsoft.WebView2.FixedVersionRuntime.128.0.2739.42.x64.cab -F:* ./src-tauri

  3. 在 `tauri.conf.json` 中配置 WebView2 运行时路径。

tauri.conf.json

    {

      "bundle": {

        "windows": {

          "webviewInstallMode": {

            "type": "fixedRuntime",

            "path": "./Microsoft.WebView2.FixedVersionRuntime.98.0.1108.50.x64/"

          }

        }

      }

    }

  4. 运行 `tauri build` 以生成带有固定 WebView2 运行时的 Windows 安装程序。

### 跳过安装

标题为“跳过安装”的章节

您可以将 [webviewInstallMode](/reference/config/#webviewinstallmode) 设置为 `skip`，从安装程序中移除 WebView2 运行时下载检查。如果用户未安装该运行时，您的应用程序将无法运行。

如果用户未安装该运行时，您的应用程序将无法运行，并且也不会尝试安装它。

tauri.conf.json

    {

      "bundle": {

        "windows": {

          "webviewInstallMode": {

            "type": "skip"

          }

        }

      }

    }

## 自定义 WiX 安装程序

标题为“自定义 WiX 安装程序”的章节

请参阅 [WiX 配置](/reference/config/#wixconfig)以获取完整的自定义选项列表。

### 安装程序模板

标题为“安装程序模板”的章节

`.msi` Windows 安装程序包是使用 [WiX Toolset v3](https://wixtoolset.org/documentation/manual/v3/) 构建的。目前，除了预定义的 [配置](/reference/config/#wixconfig) 外，您可以通过使用自定义 WiX 源代码（扩展名为 `.wxs` 的 XML 文件）或通过 WiX 片段 (fragments) 来更改它。

#### 使用自定义 WiX 文件替换安装程序代码

标题为“使用自定义 WiX 文件替换安装程序代码”的章节

Tauri 定义的 Windows 安装程序 XML 配置为适用于简单的基于 webview 的应用程序的常见用例（您可以在[此处](https://github.com/tauri-apps/tauri/blob/dev/crates/tauri-bundler/src/bundle/windows/msi/main.wxs)找到它）。它使用 [handlebars](https://docs.rs/handlebars/latest/handlebars/)，因此 Tauri CLI 可以根据您的 `tauri.conf.json` 定义来定制您的安装程序。如果您需要完全不同的安装程序，可以在 [`tauri.bundle.windows.wix.template`](/reference/config/#template-2) 上配置自定义模板文件。

#### 使用 WiX 片段扩展安装程序

标题为“使用 WiX 片段扩展安装程序”的章节

一个 [WiX 片段](https://wixtoolset.org/documentation/manual/v3/xsd/wix/fragment.html) 是一个容器，您可以在其中配置 WiX 提供的几乎所有内容。在此示例中，我们将定义一个片段，用于写入两个注册表项。

    <?xml version="1.0" encoding="utf-8"?>

    <Wix xmlns="http://schemas.microsoft.com/wix/2006/wi">

      <Fragment>

        <!-- these registry entries should be installed

         to the target user's machine -->

        <DirectoryRef Id="TARGETDIR">

          <!-- groups together the registry entries to be installed -->

          <!-- Note the unique `Id` we provide here -->

          <Component Id="MyFragmentRegistryEntries" Guid="*">

            <!-- the registry key will be under

           HKEY_CURRENT_USER\Software\MyCompany\MyApplicationName -->

            <!-- Tauri uses the second portion of the

           bundle identifier as the `MyCompany` name

           (e.g. `tauri-apps` in `com.tauri-apps.test`)  -->

            <RegistryKey

              Root="HKCU"

              Key="Software\MyCompany\MyApplicationName"

              Action="createAndRemoveOnUninstall"

            >

              <!-- values to persist on the registry -->

              <RegistryValue

                Type="integer"

                Name="SomeIntegerValue"

                Value="1"

                KeyPath="yes"

              />

              <RegistryValue Type="string" Value="Default Value" />

            </RegistryKey>

          </Component>

        </DirectoryRef>

      </Fragment>

    </Wix>

将片段文件保存为扩展名为 `.wxs` 的文件，放入 `src-tauri/windows/fragments` 文件夹中，并在 `tauri.conf.json` 中引用它。

tauri.conf.json

    {

      "bundle": {

        "windows": {

          "wix": {

            "fragmentPaths": ["./windows/fragments/registry.wxs"],

            "componentRefs": ["MyFragmentRegistryEntries"]

          }

        }

      }

    }

请注意，必须在 `tauri.conf.json` 的 `wix` 对象中分别在 `componentGroupRefs`、`componentRefs`、`featureGroupRefs`、`featureRefs` 和 `mergeRefs` 中引用 `ComponentGroup`、`Component`、`FeatureGroup`、`Feature` 和 `Merge` 元素 ID，才能将其包含在安装程序中。

### 国际化 (i18n)

标题为“国际化 (i18n)”的章节

WiX 安装程序默认使用 `en-US` 语言构建。国际化 (i18n) 可以使用 [`tauri.bundle.windows.wix.language`](/reference/config/#language) 属性进行配置，定义 Tauri 应针对其构建安装程序的语言。您可以在 [Microsoft 网站](https://docs.microsoft.com/en-us/windows/win32/msi/localizing-the-error-and-actiontext-tables)上的“语言-文化”列中找到要使用的语言名称。

#### 为单一语言编译 WiX 安装程序

标题为“为单一语言编译 WiX 安装程序”的章节

要创建针对特定语言的单一安装程序，请将 `language` 值设置为字符串。

tauri.conf.json

    {

      "bundle": {

        "windows": {

          "wix": {

            "language": "fr-FR"

          }

        }

      }

    }

#### 为列表中的每种语言编译 WiX 安装程序

标题为“为列表中的每种语言编译 WiX 安装程序”的章节

要编译针对一系列语言的安装程序，请使用数组。将为每种语言创建一个特定的安装程序，并以语言键作为后缀。

tauri.conf.json

    {

      "bundle": {

        "windows": {

          "wix": {

            "language": ["en-US", "pt-BR", "fr-FR"]

          }

        }

      }

    }

#### 为每种语言配置 WiX 安装程序字符串

标题为“为每种语言配置 WiX 安装程序字符串”的章节

可以为每种语言定义一个配置对象，以配置本地化字符串。

tauri.conf.json

    {

      "bundle": {

        "windows": {

          "wix": {

            "language": {

              "en-US": null,

              "pt-BR": {

                "localePath": "./wix/locales/pt-BR.wxl"

              }

            }

          }

        }

      }

    }

`localePath` 属性定义了指向语言文件的路径，这是一个配置语言文化的 XML。

    <WixLocalization

      Culture="en-US"

      xmlns="http://schemas.microsoft.com/wix/2006/localization"

    >

      <String Id="LaunchApp"> Launch MyApplicationName </String>

      <String Id="DowngradeErrorMessage">

        A newer version of MyApplicationName is already installed.

      </String>

      <String Id="PathEnvVarFeature">

        Add the install location of the MyApplicationName executable to

        the PATH system environment variable. This allows the

        MyApplicationName executable to be called from any location.

      </String>

      <String Id="InstallAppFeature">

        Installs MyApplicationName.

      </String>

    </WixLocalization>

注意

`WixLocalization` 元素的 `Culture` 字段必须与配置的语言匹配。

目前，Tauri 引用了以下区域设置字符串：`LaunchApp`、`DowngradeErrorMessage`、`PathEnvVarFeature` 和 `InstallAppFeature`。您可以定义自己的字符串，并在自定义模板或片段中通过 `"!(loc.TheStringId)"` 引用它们。有关更多信息，请参阅 [WiX 本地化文档](https://wixtoolset.org/documentation/manual/v3/howtos/ui_and_localization/make_installer_localizable.html)。

## 自定义 NSIS 安装程序

标题为“自定义 NSIS 安装程序”的章节

请参阅 [NSIS 配置](/reference/config/#nsisconfig)以获取完整的自定义选项列表。

### 安装程序模板

标题为“安装程序模板”的章节

Tauri 定义的 NSIS 安装程序的 `.nsi` 脚本配置为适用于简单的基于 webview 的应用程序的常见用例（您可以在[此处](https://github.com/tauri-apps/tauri/blob/dev/crates/tauri-bundler/src/bundle/windows/nsis/installer.nsi)找到它）。它使用 [handlebars](https://docs.rs/handlebars/latest/handlebars/)，因此 Tauri CLI 可以根据您的 `tauri.conf.json` 定义来定制您的安装程序。如果您需要完全不同的安装程序，可以在 [`tauri.bundle.windows.nsis.template`](/reference/config/#template-1) 上配置自定义模板文件。

### 扩展安装程序

标题为“扩展安装程序”的章节

如果您只需要扩展某些安装步骤，或许可以使用安装程序钩子 (hooks) 来代替替换整个安装程序模板。

支持的钩子有：

  * `NSIS_HOOK_PREINSTALL`：在复制文件、设置注册表键值和创建快捷方式之前运行。
  * `NSIS_HOOK_POSTINSTALL`：在安装程序完成所有文件复制、设置注册表键和创建快捷方式之后运行。
  * `NSIS_HOOK_PREUNINSTALL`：在删除任何文件、注册表键和快捷方式之前运行。
  * `NSIS_HOOK_POSTUNINSTALL`：在文件、注册表键和快捷方式被删除之后运行。

例如，在 `src-tauri/windows` 文件夹中创建一个 `hooks.nsh` 文件，并定义您需要的钩子。

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

然后，您必须配置 Tauri 以使用该钩子文件。

tauri.conf.json

    {

      "bundle": {

        "windows": {

          "nsis": {

            "installerHooks": "./windows/hooks.nsh"

          }

        }

      }

    }

#### 使用钩子安装依赖项

标题为“使用钩子安装依赖项”的章节

您可以使用安装程序钩子来自动安装您的应用程序所需的系统依赖项。这对于运行时依赖项（如 Visual C++ 可再发行组件、DirectX、OpenSSL 或其他可能并非存在于所有 Windows 系统上的系统库）特别有用。

**MSI 安装程序示例（Visual C++ 可再发行组件）**

    !macro NSIS_HOOK_POSTINSTALL

      ; Check if Visual C++ 2019 Redistributable is installed (via Windows Registry)

      ReadRegDWord $0 HKLM "SOFTWARE\Microsoft\VisualStudio\14.0\VC\Runtimes\x64" "Installed"

      ${If} $0 == 1

        DetailPrint "Visual C++ Redistributable already installed"

        Goto vcredist_done

      ${EndIf}

      ; Install from bundled MSI if not installed

      ${If} ${FileExists} "$INSTDIR\resources\vc_redist.x64.msi"

        DetailPrint "Installing Visual C++ Redistributable..."

        ; Copy to TEMP folder and then execute installer

        CopyFiles "$INSTDIR\resources\vc_redist.x64.msi" "$TEMP\vc_redist.x64.msi"

        ExecWait 'msiexec /i "$TEMP\vc_redist.x64.msi" /passive /norestart' $0

        ; Check wether installation process exited successfully (code 0) or not

        ${If} $0 == 0

          DetailPrint "Visual C++ Redistributable installed successfully"

        ${Else}

          MessageBox MB_ICONEXCLAMATION "Visual C++ installation failed. Some features may not work."

        ${EndIf}

        ; Clean up setup files from TEMP and your installed app

        Delete "$TEMP\vc_redist.x64.msi"

        Delete "$INSTDIR\resources\vc_redist.x64.msi"

      ${EndIf}

      vcredist_done:

    !macroend

**关键考虑因素**

  * 一个好的做法是始终检查依赖项是否已经通过注册表键、文件存在性或 Windows [where](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/where) 命令安装。
  * 使用 `/passive`、`/quiet` 或 `/silent` 标志以避免中断安装流程。查看 `.msi` 文件的 [msiexec](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/msiexec) 选项，或查阅应用程序特定标志的设置手册。
  * 包括 `/norestart` 以防止对于会重启用户设备的安装程序在安装过程中自动重启系统。
  * 清理临时文件和捆绑的安装程序，以避免应用程序臃肿。
  * 考虑在卸载时依赖项可能会与其他应用程序共享。
  * 如果安装失败，请提供有意义的错误消息。

确保将依赖项安装程序捆绑在您的 `src-tauri/resources` 文件夹中，并将其添加到 `tauri.conf.json` 中，以便它们被捆绑，并可以在安装期间从 `$INSTDIR\resources\` 进行访问。

tauri.conf.json

    {

      "bundle": {

        "resources": [

          "resources/my-dependency.exe",

          "resources/another-one.msi

        ]

      }

    }

### 安装模式

标题为“安装模式”的章节

默认情况下，安装程序将仅为当前用户安装您的应用程序。此选项的优点是安装程序运行不需要管理员权限，但应用程序安装在 `%LOCALAPPDATA%` 文件夹中，而不是 `C:/Program Files` 中。

如果您希望您的应用程序安装在系统范围内可用（这需要管理员权限），您可以将 [installMode](/reference/config/#installmode) 设置为 `perMachine`。

tauri.conf.json

    {

      "bundle": {

        "windows": {

          "nsis": {

            "installMode": "perMachine"

          }

        }

      }

    }

或者，您可以通过将 [installMode](/reference/config/#installmode) 设置为 `both`，让用户选择应用程序是仅为当前用户安装还是在系统范围内安装。请注意，安装程序将需要管理员权限才能执行。

有关更多信息，请参阅 [NSISInstallerMode](/reference/config/#nsisinstallermode)。

### 国际化 (i18n)

标题为“国际化 (i18n)”的章节

NSIS 安装程序是一个多语言安装程序，这意味着您始终拥有一个包含所有选定翻译的安装程序。

您可以使用 [`tauri.bundle.windows.nsis.languages`](/reference/config/#languages) 属性指定要包含的语言。NSIS 支持的语言列表可在 [NSIS GitHub 项目](https://github.com/kichik/nsis/tree/9465c08046f00ccb6eda985abbdbf52c275c6c4d/Contrib/Language%20files)中找到。需要一些 [Tauri 特定的翻译](https://github.com/tauri-apps/tauri/tree/dev/crates/tauri-bundler/src/bundle/windows/nsis/languages)，所以如果您看到未翻译的文本，请随时在 [Tauri 的主仓库](https://github.com/tauri-apps/tauri/issues/new?assignees=&labels=type%3A+feature+request&template=feature_request.yml&title=%5Bfeat%5D+)中提交功能请求。您还可以提供[自定义翻译文件](/reference/config/#customlanguagefiles)。

默认情况下，使用操作系统的默认语言来确定安装程序语言。您还可以将安装程序配置为在渲染安装程序内容之前显示语言选择器。

tauri.conf.json

    {

      "bundle": {

        "windows": {

          "nsis": {

            "displayLanguageSelector": true

          }

        }

      }

    }

### 最低 Webview2 版本

标题为“最低 Webview2 版本”的章节

如果您的应用程序需要仅在较新 Webview2 版本中可用的功能（例如自定义 URI 方案），您可以指示 Windows 安装程序验证当前的 Webview2 版本，如果不匹配目标版本，则运行 Webview2 引导程序。

tauri.conf.json

    {

      "bundle": {

        "windows": {

          "nsis": {

            "minimumWebview2Version": "110.0.1531.0"

          }

        }

      }

    }