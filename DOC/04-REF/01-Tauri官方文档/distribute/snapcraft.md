# Snapcraft

_Source: https://v2.tauri.org.cn/distribute/snapcraft/_

## 先决条件

标题为“前提条件”的章节

**1\. 安装`snap`**

  * Debian
  * Arch
  * Fedora

终端窗口

    sudo apt install snapd

终端窗口

    sudo pacman -S --needed git base-devel

    git clone https://aur.archlinux.org/snapd.git

    cd snapd

    makepkg -si

    sudo systemctl enable --now snapd.socket

    sudo systemctl start snapd.socket

    sudo systemctl enable --now snapd.apparmor.service

终端窗口

    sudo dnf install snapd

    # Enable classic snap support

    sudo ln -s /var/lib/snapd/snap /snap

安装后重启系统。

**2\. 安装基础 snap (base snap)**

终端窗口

    sudo snap install core22

**3\. 安装`snapcraft`**

终端窗口

    sudo snap install snapcraft --classic

## 配置

名为“配置”的章节

  1. 创建一个 UbuntuOne 账户。
  2. 前往 [Snapcraft](https://snapcraft.io) 网站并注册一个应用名称。
  3. 在你的项目根目录下创建一个 snapcraft.yaml 文件。
  4. 调整 snapcraft.yaml 文件中的名称。

    name: appname

    base: core22

    version: '0.1.0'

    summary: Your summary # 79 char long summary

    description: |

      Your description

    grade: stable

    confinement: strict

    layout:

      /usr/lib/$SNAPCRAFT_ARCH_TRIPLET/webkit2gtk-4.1:

        bind: $SNAP/usr/lib/$SNAPCRAFT_ARCH_TRIPLET/webkit2gtk-4.1

    apps:

      appname:

        command: usr/bin/appname

        desktop: usr/share/applications/appname.desktop

        extensions: [gnome]

        #plugs:

        #  - network

        # Add whatever plugs you need here, see https://snapcraft.io/docs/snapcraft-interfaces for more info.

        # The gnome extension already includes [ desktop, desktop-legacy, gsettings, opengl, wayland, x11, mount-observe, calendar-service ]

        #  - single-instance-plug # add this if you're using the single-instance plugin

        #slots:

        # Add the slots you need to expose to other snaps

        #  - single-instance-plug # add this if you're using the single-instance plugin

    # Add these lines only if you're using the single-instance plugin

    # Check https://v2.tauri.org.cn/plugin/single-instance/ for details

    #slots:

    #  single-instance:

    #    interface: dbus

    #    bus: session

    #    name: org.net_mydomain_MyApp.SingleInstance # Remember to change net_mydomain_MyApp to your app ID with "_" instead of "." and "-"

    #

    #plugs:

    #  single-instance-plug:

    #    interface: dbus

    #    bus: session

    #    name: org.net_mydomain_MyApp.SingleInstance # Remember to change net_mydomain_MyApp to your app ID with "_" instead of "." and "-"

    package-repositories:

      - type: apt

        components: [main]

        suites: [noble]

        key-id: 78E1918602959B9C59103100F1831DDAFC42E99D

        url: http://ppa.launchpad.net/snappy-dev/snapcraft-daily/ubuntu

    parts:

      build-app:

        plugin: dump

        build-snaps:

          - node/20/stable

          - rustup/latest/stable

        build-packages:

          - libwebkit2gtk-4.1-dev

          - build-essential

          - curl

          - wget

          - file

          - libxdo-dev

          - libssl-dev

          - libayatana-appindicator3-dev

          - librsvg2-dev

          - dpkg

        stage-packages:

          - libwebkit2gtk-4.1-0

          - libayatana-appindicator3-1

        source: .

        override-build: |

          set -eu

          npm install

          npm run tauri build -- --bundles deb

          dpkg -x src-tauri/target/release/bundle/deb/*.deb $SNAPCRAFT_PART_INSTALL/

          sed -i -e "s|Icon=appname|Icon=/usr/share/icons/hicolor/32x32/apps/appname.png|g" $SNAPCRAFT_PART_INSTALL/usr/share/applications/appname.desktop

### 说明

名为“说明”的章节

  * `name` 变量定义了你的应用名称，且必须设置为你之前注册过的名称。
  * `base` 变量定义了你所使用的核心 (core)。
  * `version` 变量定义了版本号，每当源码仓库发生变更时都应更新该版本号。
  * `apps` 部分允许你暴露桌面文件和二进制文件，以便用户可以运行你的应用。
  * `package-repositories` 部分允许你添加包仓库，以帮助满足你的依赖项需求。
  * `build-packages`/`build-snaps` 定义了 snap 的构建依赖。
  * `stage-packages`/`stage-snaps` 定义了 snap 的运行时依赖。
  * `override-build` 部分会在拉取源码后运行一系列命令。

## 构建

名为“构建”的章节

终端窗口

    sudo snapcraft

## 测试

名为“测试”的章节

终端窗口

    snap run your-app

## 手动发布

名为“手动发布”的章节

终端窗口

    snapcraft login # Login with your UbuntuOne credentials

    snapcraft upload --release=stable mysnap_latest_amd64.snap

## 自动构建

名为“自动构建”的章节

  1. 在你的应用开发者页面，点击 `builds` 选项卡。
  2. 点击 `login with github`（使用 GitHub 登录）。
  3. 输入你的仓库详细信息。