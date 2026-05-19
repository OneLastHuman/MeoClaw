# AUR

_Source: https://v2.tauri.org.cn/distribute/aur/_

# 发布到 Arch 用户仓库 (AUR)

名为“发布到 Arch 用户仓库 (AUR)”的章节

## 设置

标题为“设置”的章节

首先前往 `https://aur.archlinux.org` 并创建一个账号。请务必添加正确的 SSH 密钥。接下来，使用以下命令克隆一个空的 Git 仓库。

终端窗口

    git clone https://aur.archlinux.org/your-repo-name

完成上述步骤后，创建一个名为 `PKGBUILD` 的文件。文件创建完成后，即可进行下一步。

### 编写 PKGBUILD 文件

名为“编写 PKGBUILD 文件”的章节

PKGBUILD

    pkgname=<pkgname>

    pkgver=1.0.0

    pkgrel=1

    pkgdesc="Description of your app"

    arch=('x86_64' 'aarch64')

    url="https://github.com/<user>/<project>"

    license=('MIT')

    depends=('cairo' 'desktop-file-utils' 'gdk-pixbuf2' 'glib2' 'gtk3' 'hicolor-icon-theme' 'libsoup' 'pango' 'webkit2gtk-4.1')

    options=('!strip' '!emptydirs')

    install=${pkgname}.install

    source_x86_64=("${url}/releases/download/v${pkgver}/appname_${pkgver}_amd64.deb")

    source_aarch64=("${url}/releases/download/v${pkgver}/appname_"${pkgver}_arm64.deb")

  * 在文件顶部，定义您的软件包名称并将其赋值给 `pkgname` 变量。
  * 设置您的 `pkgver` 变量。通常，在 source 变量中使用此变量以提高可维护性是最佳做法。
  * `pkgdesc` 变量会显示在您的 AUR 仓库页面上，用于告知访客您的应用的功能。
  * `arch` 变量用于控制哪些架构可以安装您的软件包。
  * `url` 变量虽然不是必需的，但有助于使您的软件包看起来更专业。
  * `install` 变量用于指定 .install 脚本的名称，该脚本将在安装、删除或升级软件包时运行。
  * `depends` 变量包含运行您的应用所需的项目列表。对于任何 Tauri 应用，您必须包含上面显示的所有依赖项。
  * `source` 变量是必需的，它定义了上游软件包所在的位置。通过在变量名末尾添加架构，您可以使 `source` 具有架构特异性。

### 生成 `.SRCINFO`

名为“生成 .SRCINFO”的章节

为了将您的仓库推送到 AUR，必须生成一个 `.SRCINFO` 文件。这可以通过此命令完成。

终端窗口

    makepkg --printsrcinfo > .SRCINFO

### 测试

名为“测试”的章节

测试应用非常简单。您只需在与 `PKGBUILD` 文件相同的目录中运行 `makepkg`，看看它是否能正常工作即可。

### 发布

名为“发布”的章节

最后，在测试阶段结束后，您可以使用这些命令将应用发布到 AUR (Arch 用户仓库)。

终端窗口

    git add .

    git commit -m "Initial Commit"

    git push

如果一切顺利，您的仓库现在应该会出现在 AUR 网站上。

## 示例

名为“示例”的章节

### 从 Debian 软件包中提取

名为“从 Debian 软件包中提取”的章节

PKGBUILD

    # Maintainer:

    # Contributor:

    pkgname=<pkgname>

    pkgver=1.0.0

    pkgrel=1

    pkgdesc="Description of your app"

    arch=('x86_64' 'aarch64')

    url="https://github.com/<user>/<project>"

    license=('MIT')

    depends=('cairo' 'desktop-file-utils' 'gdk-pixbuf2' 'glib2' 'gtk3' 'hicolor-icon-theme' 'libsoup' 'pango' 'webkit2gtk-4.1')

    options=('!strip' '!debug')

    install=${pkgname}.install

    source_x86_64=("${url}/releases/download/v${pkgver}/appname_${pkgver}_amd64.deb")

    source_aarch64=("${url}/releases/download/v${pkgver}/appname_${pkgver}_arm64.deb")

    sha256sums_x86_64=('ca85f11732765bed78f93f55397b4b4cbb76685088553dad612c5062e3ec651f')

    sha256sums_aarch64=('ed2dc3169d34d91188fb55d39867713856dd02a2360ffe0661cb2e19bd701c3c')

    package() {

      # Extract package data

      tar -xvf data.tar.gz -C "${pkgdir}"

    }

my-tauri-app.install

    post_install() {

      gtk-update-icon-cache -q -t -f usr/share/icons/hicolor

      update-desktop-database -q

    }

    post_upgrade() {

      post_install

    }

    post_remove() {

      gtk-update-icon-cache -q -t -f usr/share/icons/hicolor

      update-desktop-database -q

    }

### 从源代码构建

名为“从源代码构建”的章节

PKGBUILD

    # Maintainer:

    pkgname=<pkgname>-git

    pkgver=<pkgver>

    pkgrel=1

    pkgdesc="Description of your app"

    arch=('x86_64' 'aarch64')

    url="https://github.com/<user>/<project>"

    license=('MIT')

    depends=('cairo' 'desktop-file-utils' 'gdk-pixbuf2' 'glib2' 'gtk3' 'hicolor-icon-theme' 'libsoup' 'pango' 'webkit2gtk-4.1')

    makedepends=('git' 'openssl' 'appmenu-gtk-module' 'libappindicator-gtk3' 'librsvg' 'cargo' 'pnpm' 'nodejs')

    provides=('<pkgname>')

    conflicts=('<binname>' '<pkgname>')

    source=("git+${url}.git")

    sha256sums=('SKIP')

    pkgver() {

      cd <project>

      ( set -o pipefail

        git describe --long --abbrev=7 2>/dev/null | sed 's/\([^-]*-g\)/r\1/;s/-/./g' ||

        printf "r%s.%s" "$(git rev-list --count HEAD)" "$(git rev-parse --short=7 HEAD)"

      )

    }

    prepare() {

      cd <project>

      pnpm install

    }

    build() {

      cd <project>

      pnpm tauri build -b deb

    }

    package() {

      cp -a <project>/src-tauri/target/release/bundle/deb/<project>_${pkgver}_*/data/* "${pkgdir}"

    }