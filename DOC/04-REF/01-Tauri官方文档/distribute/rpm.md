# RPM

_Source: https://v2.tauri.org.cn/distribute/rpm/_

注意

本指南中的某些部分是可选的，包括配置脚本和某些其他步骤。请根据您的具体需求和要求随意调整指南中的说明。

本指南介绍了如何分发和管理 RPM 软件包，包括检索软件包信息、配置脚本、设置依赖项以及签署软件包。

注意

macOS 和 Linux 上的 GUI 应用不会从你的 shell 点文件（`.bashrc`、`.bash_profile`、`.zshrc` 等）继承 `$PATH`。查看 Tauri 的 [fix-path-env-rs](https://github.com/tauri-apps/fix-path-env-rs) crate 来解决此问题。

## 局限性

标题为“限制”的章节

像 glibc 这样的核心库经常会破坏与旧系统的兼容性。因此，你必须使用你打算支持的最旧的基础系统来构建你的 Tauri 应用程序。较旧的系统（例如 Ubuntu 18.04）比 Ubuntu 22.04 更合适，因为在 Ubuntu 22.04 上编译的二进制文件对 glibc 版本的要求更高，因此在较旧的系统上运行时，你会遇到类似 `/usr/lib/libc.so.6: version 'GLIBC_2.33' not found` 的运行时错误。我们建议使用 Docker 容器或 GitHub Actions 来为 Linux 构建你的 Tauri 应用程序。

有关更多信息，请参阅 issues [tauri-apps/tauri#1355](https://github.com/tauri-apps/tauri/issues/1355) 和 [rust-lang/rust#57497](https://github.com/rust-lang/rust/issues/57497)，以及 [AppImage 指南](https://docs.appimage.org/reference/best-practices.html#binaries-compiled-on-old-enough-base-system)。

## 配置 RPM 软件包

名为“配置 RPM 软件包”的章节

Tauri 允许您通过添加脚本、设置依赖项、添加许可证、包含自定义文件等方式来配置 RPM 软件包。有关可配置选项的详细信息，请参阅：[RpmConfig](https://v2.tauri.org.cn/reference/config/#rpmconfig)。

### 向软件包添加安装前/后脚本及卸载前/后脚本

名为“向软件包添加安装前/后脚本及卸载前/后脚本”的章节

RPM 软件包管理器允许您在安装或卸载软件包之前或之后运行脚本。例如，您可以使用这些脚本在软件包安装后启动服务。

以下是如何添加这些脚本的示例：

  1. 在项目的 `src-tauri` 目录下创建一个名为 `scripts` 的文件夹。

终端窗口

    mkdir src-tauri/scripts

  2. 在该文件夹中创建脚本文件。

终端窗口

    touch src-tauri/scripts/postinstall.sh \

    touch src-tauri/scripts/preinstall.sh \

    touch src-tauri/scripts/preremove.sh \

    touch src-tauri/scripts/postremove.sh

现在，如果我们查看 `/src-tauri/scripts` 目录，将会看到：

终端窗口

    ls src-tauri/scripts/

    postinstall.sh  postremove.sh  preinstall.sh  preremove.sh

  3. 在脚本中添加内容：

preinstall.sh (安装前)

    echo "-------------"

    echo "This is pre"

    echo "Install Value: $1"

    echo "Upgrade Value: $1"

    echo "Uninstall Value: $1"

    echo "-------------"

postinstall.sh (安装后)

    echo "-------------"

    echo "This is post"

    echo "Install Value: $1"

    echo "Upgrade Value: $1"

    echo "Uninstall Value: $1"

    echo "-------------"

preremove.sh (卸载前)

    echo "-------------"

    echo "This is preun"

    echo "Install Value: $1"

    echo "Upgrade Value: $1"

    echo "Uninstall Value: $1"

    echo "-------------"

postremove.sh (卸载后)

    echo "-------------"

    echo "This is postun"

    echo "Install Value: $1"

    echo "Upgrade Value: $1"

    echo "Uninstall Value: $1"

    echo "-------------"

  4. 将脚本添加到 `tauri.conf.json` 文件中：

tauri.conf.json

    {

      "bundle": {

        "linux": {

          "rpm": {

            "epoch": 0,

            "files": {},

            "release": "1",

            // add the script here

            "preInstallScript": "/path/to/your/project/src-tauri/scripts/prescript.sh",

            "postInstallScript": "/path/to/your/project/src-tauri/scripts/postscript.sh",

            "preRemoveScript": "/path/to/your/project/src-tauri/scripts/prescript.sh",

            "postRemoveScript": "/path/to/your/project/src-tauri/scripts/postscript.sh"

          }

        }

      }

    }

### 设置冲突 (Conflict)、提供 (Provides)、依赖 (Depends)、文件 (Files)、废弃 (Obsoletes)、桌面模板 (DesktopTemplate) 和纪元 (Epoch)

名为“设置冲突、提供、依赖、文件、废弃、桌面模板和纪元”的章节

  * **conflict (冲突)** : 如果软件包与另一个软件包发生冲突，则阻止安装。例如，如果您更新了您的应用所依赖的 RPM 包，而新版本与您的应用不兼容。

  * **provides (提供)** : 列出您的应用程序提供的 RPM 依赖项。

  * **depends (依赖)** : 列出您的应用程序运行所需的 RPM 依赖项。

  * **files (文件)** : 指定软件包中包含哪些文件。

  * **obsoletes (废弃)** : 列出您的应用程序所废弃的 RPM 依赖项。

注意

如果安装了此软件包，列为“obsoletes”的软件包（如果存在）将被自动移除。

  * **desktopTemplate (桌面模板)** : 向软件包添加自定义的桌面文件 (desktop file)。

  * **epoch (纪元)** : 基于版本号定义加权依赖关系。

注意

除非必要，否则不建议使用 epoch，因为它会改变包管理器比较软件包版本的方式。有关 epoch 的更多信息，请查看：[RPM 打包指南](https://rpm-packaging-guide.github.io/#epoch-scriptlets-and-triggers)。

要使用这些选项，请将以下内容添加到您的 `tauri.conf.json`：

tauri.conf.json

    {

      "bundle": {

        "linux": {

          "rpm": {

            "postRemoveScript": "/path/to/your/project/src-tauri/scripts/postscript.sh",

            "conflicts": ["oldLib.rpm"],

            "depends": ["newLib.rpm"],

            "obsoletes": ["veryoldLib.rpm"],

            "provides": ["coolLib.rpm"],

            "desktopTemplate": "/path/to/your/project/src-tauri/desktop-template.desktop"

          }

        }

      }

    }

### 向软件包添加许可证

名为“向软件包添加许可证”的章节

要向软件包添加许可证，请将以下内容添加到 `src-tauri/Cargo.toml` 或 `src-tauri/tauri.conf.json` 文件中：

src-tauri/Cargo.toml

    [package]

    name = "tauri-app"

    version = "0.0.0"

    description = "A Tauri App"

    authors = ["you"]

    edition = "2021"

    license = "MIT" # add the license here

    # ...  rest of the file

对于 `src-tauri/tauri.conf.json`：

src-tauri/tauri.conf.json

    {

      "bundle": {

        "licenseFile": "../LICENSE", // put the path to the license file here

        "license": "MIT" // add the license here

      }

    }

## 构建 RPM 软件包

名为“构建 RPM 软件包”的章节

要构建 RPM 软件包，可以使用以下命令：

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

此命令将在 `src-tauri/target/release/bundle/rpm` 目录下构建 RPM 软件包。

## 签署 RPM 软件包

名为“签署 RPM 软件包”的章节

Tauri 允许您在构建过程中使用系统中的密钥对软件包进行签名。为此，您需要生成一个 GPG 密钥。

#### 生成 GPG 密钥

名为“生成 GPG 密钥”的章节

要生成 GPG 密钥，可以使用以下命令：

终端窗口

    gpg --gen-key

按照说明生成密钥。

密钥生成后，您需要将其添加到环境变量中。可以通过将以下内容添加到 .bashrc 或 .zshrc 文件中，或者直接在终端中 export 出来：

终端窗口

    export TAURI_SIGNING_RPM_KEY=$(cat /home/johndoe/my_super_private.key)

如果密钥有密码 (passphrase)，可以将其添加到环境变量中：

终端窗口

    export TAURI_SIGNING_RPM_KEY_PASSPHRASE=password

现在，您可以使用以下命令构建软件包：

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

### 验证签名

名为“验证签名”的章节

注意

仅在本地测试签名时才需要这样做。

在验证签名之前，您需要将公钥创建并导入到 RPM 数据库中：

终端窗口

    gpg --export -a 'Tauri-App' > RPM-GPG-KEY-Tauri-App

终端窗口

    sudo rpm --import RPM-GPG-KEY-Tauri-App

密钥导入后，我们需要编辑 `~/.rpmmacros` 文件以使用该密钥。

~/.rpmmacros

    %_signature gpg

    %_gpg_path /home/johndoe/.gnupg

    %_gpg_name Tauri-App

    %_gpgbin /usr/bin/gpg2

    %__gpg_sign_cmd %{__gpg} \

        gpg --force-v3-sigs --digest-algo=sha1 --batch --no-verbose --no-armor \

        --passphrase-fd 3 --no-secmem-warning -u "%{_gpg_name}" \

        -sbo %{__signature_filename} %{__plaintext_filename}

最后，您可以使用以下命令验证软件包：

终端窗口

    rpm  -v --checksig tauri-app-0.0.0-1.x86_64.rpm

## 调试 RPM 软件包

名为“调试 RPM 软件包”的章节

在本节中，我们将学习如何通过检查软件包内容及获取软件包信息来调试 RPM 软件包。

### 获取软件包信息

名为“获取软件包信息”的章节

要获取有关软件包的信息（如版本、发布号和架构），请使用以下命令：

终端窗口

    rpm -qip package_name.rpm

### 查询软件包的特定信息

名为“查询软件包的特定信息”的章节

例如，如果您想获取软件包的名称、版本、发布号、架构和大小，请使用以下命令：

终端窗口

    rpm  -qp --queryformat '[%{NAME} %{VERSION} %{RELEASE} %{ARCH} %{SIZE}\n]' package_name.rpm

注意

 _`--queryformat`_ 是一个格式字符串，可用于获取有关软件包的特定信息。可以检索的信息均来自于 rpm -qip 命令。

### 检查软件包内容

名为“检查软件包内容”的章节

要检查软件包的内容，请使用以下命令：

终端窗口

    rpm -qlp package_name.rpm

此命令将列出软件包中包含的所有文件。

### 调试脚本

名为“调试脚本”的章节

要调试安装前/后脚本或卸载前/后脚本，请使用以下命令：

终端窗口

    rpm -qp --scripts package_name.rpm

此命令将打印脚本的内容。

### 检查依赖项

名为“检查依赖项”的章节

要检查软件包的依赖项，请使用以下命令：

终端窗口

    rpm -qp --requires package_name.rpm

### 列出依赖于特定软件包的包

名为“列出依赖于特定软件包的包”的章节

要列出依赖于特定软件包的包，请使用以下命令：

终端窗口

    rpm -q --whatrequires package_name.rpm

### 调试安装问题

名为“调试安装问题”的章节

如果您在安装 RPM 软件包时遇到问题，可以使用 `-vv`（非常详细）选项来获取详细的输出信息：

终端窗口

    rpm -ivvh package_name.rpm

或者，对于已安装的软件包：

终端窗口

    rpm -Uvvh package_name.rpm

## 针对 ARM 设备进行交叉编译

名为“针对 ARM 设备进行交叉编译”的章节

本指南介绍的是手动编译方式。请查看我们的 [GitHub Action 指南](/distribute/pipelines/github/#arm-runner-compilation)，了解利用 QEMU 构建应用的工作流示例。这种方式速度会慢很多，但也能构建 AppImages。

手动编译适用于无需频繁编译应用程序且偏好一次性设置的情况。以下步骤假定您使用的是基于 Debian/Ubuntu 的 Linux 发行版。

  1. #### 为您的目标架构安装 Rust target

名为“为您的目标架构安装 Rust target”的章节

     * 对于 ARMv7 (32-bit): `rustup target add armv7-unknown-linux-gnueabihf`
     * 对于 ARMv8 (ARM64, 64-bit): `rustup target add aarch64-unknown-linux-gnu`
  2. #### 为所选架构安装对应的链接器 (linker)

名为“为所选架构安装对应的链接器”的章节

     * 对于 ARMv7: `sudo apt install gcc-arm-linux-gnueabihf`
     * 对于 ARMv8 (ARM64): `sudo apt install gcc-aarch64-linux-gnu`
  3. #### 打开或创建文件 `<project-root>/.cargo/config.toml`，并根据需要添加以下配置：

名为“打开或创建文件 <project-root>/.cargo/config.toml，并根据需要添加以下配置”的章节

         [target.armv7-unknown-linux-gnueabihf]

         linker = "arm-linux-gnueabihf-gcc"

         [target.aarch64-unknown-linux-gnu]

         linker = "aarch64-linux-gnu-gcc"

  4. #### 在包管理器中启用相应的架构：

名为“在包管理器中启用相应的架构”的章节

     * 对于 ARMv7: `sudo dpkg --add-architecture armhf`
     * 对于 ARMv8 (ARM64): `sudo dpkg --add-architecture arm64`
  5. #### 调整软件包源 (Package Sources)

名为“调整软件包源”的章节

在 Debian 上，此步骤通常非必要，但在其他发行版上，您可能需要编辑 /etc/apt/sources.list 以包含 ARM 架构变体。例如，在 Ubuntu 22.04 上，请将这些行添加到文件末尾（记得将 jammy 替换为您 Ubuntu 版本的代号）：

         deb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy main restricted

         deb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy-updates main restricted

         deb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy universe

         deb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy-updates universe

         deb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy multiverse

         deb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy-updates multiverse

         deb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy-backports main restricted universe multiverse

         deb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy-security main restricted

         deb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy-security universe

         deb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy-security multiverse

然后，为防止主软件包出现问题，您必须将正确的“主架构”添加到文件中之前已有的所有其他行中。对于标准的 64 位系统，您需要添加 [arch=amd64]，这样 Ubuntu 22.04 上的完整文件看起来类似于这样：

显示解决方案

         # See http://help.ubuntu.com/community/UpgradeNotes for how to upgrade to

         # newer versions of the distribution.

         deb [arch=amd64] http://archive.ubuntu.com/ubuntu/ jammy main restricted

         # deb-src http://archive.ubuntu.com/ubuntu/ jammy main restricted

         ## Major bug fix updates produced after the final release of the

         ## distribution.

         deb [arch=amd64] http://archive.ubuntu.com/ubuntu/ jammy-updates main restricted

         # deb-src http://archive.ubuntu.com/ubuntu/ jammy-updates main restricted

         ## N.B. software from this repository is ENTIRELY UNSUPPORTED by the Ubuntu

         ## team. Also, please note that software in universe WILL NOT receive any

         ## review or updates from the Ubuntu security team.

         deb [arch=amd64] http://archive.ubuntu.com/ubuntu/ jammy universe

         # deb-src http://archive.ubuntu.com/ubuntu/ jammy universe

         deb [arch=amd64] http://archive.ubuntu.com/ubuntu/ jammy-updates universe

         # deb-src http://archive.ubuntu.com/ubuntu/ jammy-updates universe

         ## N.B. software from this repository is ENTIRELY UNSUPPORTED by the Ubuntu

         ## team, and may not be under a free licence. Please satisfy yourself as to

         ## your rights to use the software. Also, please note that software in

         ## multiverse WILL NOT receive any review or updates from the Ubuntu

         ## security team.

         deb [arch=amd64] http://archive.ubuntu.com/ubuntu/ jammy multiverse

         # deb-src http://archive.ubuntu.com/ubuntu/ jammy multiverse

         deb [arch=amd64] http://archive.ubuntu.com/ubuntu/ jammy-updates multiverse

         ## N.B. software from this repository may not have been tested as

         ## extensively as that contained in the main release, although it includes

         ## newer versions of some applications which may provide useful features.

         ## Also, please note that software in backports WILL NOT receive any review

         ## or updates from the Ubuntu security team.

         deb [arch=amd64] http://archive.ubuntu.com/ubuntu/ jammy-backports main restricted universe multiverse

         # deb-src http://archive.ubuntu.com/ubuntu/ jammy-backports main restricted universe multiverse

         deb [arch=amd64] http://security.ubuntu.com/ubuntu/ jammy-security main restricted

         # deb-src http://security.ubuntu.com/ubuntu/ jammy-security main restricted

         deb [arch=amd64] http://security.ubuntu.com/ubuntu/ jammy-security universe

         # deb-src http://security.ubuntu.com/ubuntu/ jammy-security universe

         deb [arch=amd64] http://security.ubuntu.com/ubuntu/ jammy-security multiverse

         # deb-src http://security.ubuntu.com/ubuntu/ jammy-security multiverse

         deb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy main restricted

         deb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy-updates main restricted

         deb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy universe

         deb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy-updates universe

         deb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy multiverse

         deb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy-updates multiverse

         deb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy-backports main restricted universe multiverse

         deb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy-security main restricted

         deb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy-security universe

         deb [arch=armhf,arm64] http://ports.ubuntu.com/ubuntu-ports jammy-security multiverse

  6. #### 更新软件包信息：`sudo apt-get update && sudo apt-get upgrade -y`

名为“更新软件包信息：sudo apt-get update && sudo apt-get upgrade -y”的章节

  7. #### 为所选架构安装所需的 webkitgtk 库：

名为“为所选架构安装所需的 webkitgtk 库”的章节

     * 对于 ARMv7: `sudo apt install libwebkit2gtk-4.1-dev:armhf`
     * 对于 ARMv8 (ARM64): `sudo apt install libwebkit2gtk-4.1-dev:arm64`
  8. #### 安装 OpenSSL 或使用 vendored 版本：

名为“安装 OpenSSL 或使用 vendored 版本”的章节

这并不总是必需的，因此您可以先尝试继续，如果遇到 `Failed to find OpenSSL development headers` 等错误，再进行处理。

     * 您可以选择在系统范围内安装开发头文件：
       * 对于 ARMv7: `sudo apt install libssl-dev:armhf`
       * 对于 ARMv8 (ARM64): `sudo apt install libssl-dev:arm64`
     * 或者，为您项目中 Cargo.toml 依赖部分中的 OpenSSL Rust crate 启用“vendor”特性，这将影响所有使用相同次要版本的其他 Rust 依赖项：

    openssl-sys = {version = "0.9", features = ["vendored"]}

  9. #### 根据所选架构设置 `PKG_CONFIG_SYSROOT_DIR` 到对应的目录：

名为“根据所选架构设置 PKG_CONFIG_SYSROOT_DIR 到对应的目录”的章节

     * 对于 ARMv7: `export PKG_CONFIG_SYSROOT_DIR=/usr/arm-linux-gnueabihf/`
     * 对于 ARMv8 (ARM64): `export PKG_CONFIG_SYSROOT_DIR=/usr/aarch64-linux-gnu/`
  10. #### 为所需的 ARM 版本构建应用：

名为“为所需的 ARM 版本构建应用”的章节

     * 对于 ARMv7: cargo tauri build --target armv7-unknown-linux-gnueabihf
     * 对于 ARMv8 (ARM64): cargo tauri build --target aarch64-unknown-linux-gnu

请根据您是要将 Tauri 应用程序交叉编译为 ARMv7 还是 ARMv8 (ARM64)，选择相应的指令。请注意，具体步骤可能会根据您的 Linux 发行版和环境设置而有所不同。