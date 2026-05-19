# GitHub

_Source: https://v2.tauri.org.cn/distribute/pipelines/github/_

本指南将向您展示如何在 [GitHub Actions](https://githubdocs.cn/en/actions) 中使用 [tauri-action](https://github.com/tauri-apps/tauri-action) 来轻松构建并上传您的应用，以及如何使 Tauri 的更新程序查询新创建的 GitHub 发布版本以获取更新。

最后，它还将展示如何为 Linux Arm AppImage 设置一个更复杂的构建流水线。

代码签名

要在您的工作流程中为 Windows 和 macOS 设置代码签名，请遵循每个平台的特定指南：

  * [Windows 代码签名](/distribute/sign/windows/)
  * [macOS 代码签名](/distribute/sign/macos/)

## 入门指南

名为“入门指南”的章节

要设置 `tauri-action`，您必须首先设置一个 GitHub 仓库。您也可以在尚未配置 Tauri 的仓库上使用此操作，因为它能为您自动初始化 Tauri。请查看 [该操作的自述文件 (README)](https://github.com/tauri-apps/tauri-action/#project-initialization) 以了解必要的配置选项。

转到 GitHub 项目页面上的“Actions”选项卡并选择“New workflow”，然后选择“Set up a workflow yourself”。将文件替换为下方的工作流程，或从 [该操作的示例](https://github.com/tauri-apps/tauri-action/tree/dev/examples) 中选择一个。

## 配置

名为“配置”的章节

请查看 `tauri-action` 的 [自述文件 (README)](https://github.com/tauri-apps/tauri-action/#inputs) 以获取所有可用的配置选项。

当您的应用不在仓库的根目录时，请使用 `projectPath` 输入项。

您可以随意修改工作流程名称、更改其触发器，并添加更多步骤，例如 `npm run lint` 或 `npm run test`。重要的是，请务必保留工作流程末尾的下方代码行，因为它会运行构建脚本并发布您的应用。

### 如何触发

名为“如何触发”的章节

下方及 `tauri-action` 示例中展示的发布工作流程是由推送到 `release` 分支触发的。该操作会自动使用应用程序版本创建一个 git 标签 (tag) 和 GitHub 发布版本的标题。

作为另一个示例，您还可以将触发器更改为在推送版本 git 标签（例如 `app-v0.7.0`）时运行工作流程。

    name: 'publish'

    on:

      push:

        tags:

          - 'app-v*'

有关可能触发器配置的完整列表，请查看官方 [GitHub 文档](https://githubdocs.cn/en/actions/using-workflows/events-that-trigger-workflows)。

## 工作流程示例

名为“工作流程示例”的章节

以下是一个工作流程示例，它被设置为在您每次推送到 `release` 分支时运行。

此工作流程将为 Windows x64、Linux x64、Linux Arm64、macOS x64 和 macOS Arm64（M1 及更高版本）构建并发布您的应用。

该工作流程执行的步骤包括：

  1. 使用 `actions/checkout@v4` 检出仓库。
  2. 安装构建应用所需的 Linux 系统依赖项。
  3. 使用 `actions/setup-node@v4` 设置 Node.js LTS，并为全局 npm/yarn/pnpm 包数据设置缓存。
  4. 使用 `dtolnay/rust-toolchain@stable` 和 `swatinem/rust-cache@v2` 设置 Rust 环境并缓存 Rust 构建产物。
  5. 安装前端依赖项，如果未配置为 [`beforeBuildCommand`](/reference/config/#beforebuildcommand)，则运行 Web 应用的构建脚本。
  6. 最后，使用 `tauri-apps/tauri-action@v0` 运行 `tauri build`，生成产物并创建 GitHub 发布版本。

    name: 'publish'

    on:

      workflow_dispatch:

      push:

        branches:

          - release

    jobs:

      publish-tauri:

        permissions:

          contents: write

        strategy:

          fail-fast: false

          matrix:

            include:

              - platform: 'macos-latest' # for Arm based macs (M1 and above).

                args: '--target aarch64-apple-darwin'

              - platform: 'macos-latest' # for Intel based macs.

                args: '--target x86_64-apple-darwin'

              - platform: 'ubuntu-22.04'

                args: ''

              - platform: 'ubuntu-22.04-arm' # Only available in public repos.

                args: ''

              - platform: 'windows-latest'

                args: ''

        runs-on: ${{ matrix.platform }}

        steps:

          - uses: actions/checkout@v4

          - name: install dependencies (ubuntu only)

            if: matrix.platform == 'ubuntu-22.04' || matrix.platform == 'ubuntu-22.04-arm' # This must match the platform value defined above.

            run: |

              sudo apt-get update

              sudo apt-get install -y libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf

          - name: setup node

            uses: actions/setup-node@v4

            with:

              node-version: lts/*

              cache: 'yarn' # Set this to npm, yarn or pnpm.

          - name: install Rust stable

            uses: dtolnay/rust-toolchain@stable # Set this to dtolnay/rust-toolchain@nightly

            with:

              # Those targets are only used on macos runners so it's in an `if` to slightly speed up windows and linux builds.

              targets: ${{ matrix.platform == 'macos-latest' && 'aarch64-apple-darwin,x86_64-apple-darwin' || '' }}

          - name: Rust cache

            uses: swatinem/rust-cache@v2

            with:

              workspaces: './src-tauri -> target'

          - name: install frontend dependencies

            # If you don't have `beforeBuildCommand` configured you may want to build your frontend here too.

            run: yarn install # change this to npm or pnpm depending on which one you use.

          - uses: tauri-apps/tauri-action@v0

            env:

              GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}

            with:

              tagName: app-v__VERSION__ # the action automatically replaces \_\_VERSION\_\_ with the app version.

              releaseName: 'App v__VERSION__'

              releaseBody: 'See the assets to download this version and install.'

              releaseDraft: true

              prerelease: false

              args: ${{ matrix.args }}

有关更多配置选项，请查看 [`tauri-action`](https://github.com/tauri-apps/tauri-action) 仓库及其 [示例](https://github.com/tauri-apps/tauri-action/blob/dev/examples/)。

注意

请仔细阅读 GitHub Actions 的 [使用限制、计费和管理](https://githubdocs.cn/en/actions/learn-github-actions/usage-limits-billing-and-administration) 文档。

## Arm Runner 编译

名为“Arm Runner 编译”的章节

2025 年 8 月更新

Github 已经 [发布](https://github.blog/changelog/2025-08-07-arm64-hosted-runners-for-public-repositories-are-now-generally-available/#get-started) 了公开可用的 `ubuntu-22.04-arm` 和 `ubuntu-24.04-arm` 运行器 (runners)。您可以使用它们在公共仓库中通过上述工作流程示例构建 Arm64 应用。

此工作流程使用 [`pguyot/arm-runner-action`](https://github.com/pguyot/arm-runner-action) 直接在模拟的 Arm 运行器上进行编译。这弥补了 AppImage 工具中缺乏跨架构构建支持的问题。

危险

`arm-runner-action` 比 GitHub 的标准运行器 **慢得多** ，因此在需要支付构建分钟数的私有仓库中请务必小心。一个未经缓存的 `create-tauri-app` 项目构建大约需要 1 小时。

    name: 'Publish Linux Arm builds'

    on:

      workflow_dispatch:

      push:

        branches:

          - release

    jobs:

      build:

        runs-on: ubuntu-22.04

        strategy:

          matrix:

            arch: [aarch64, armv7l]

            include:

              - arch: aarch64

                cpu: cortex-a72

                base_image: https://dietpi.com/downloads/images/DietPi_RPi5-ARMv8-Bookworm.img.xz

                deb: arm64

                rpm: aarch64

                appimage: aarch64

              - arch: armv7l

                cpu: cortex-a53

                deb: armhfp

                rpm: arm

                appimage: armhf

                base_image: https://dietpi.com/downloads/images/DietPi_RPi-ARMv7-Bookworm.img.xz

        steps:

          - uses: actions/checkout@v3

          - name: Cache rust build artifacts

            uses: Swatinem/rust-cache@v2

            with:

              workspaces: src-tauri

              cache-on-failure: true

          - name: Build app

            uses: pguyot/arm-runner-action@v2.6.5

            with:

              base_image: ${{ matrix.base_image }}

              cpu: ${{ matrix.cpu }}

              bind_mount_repository: true

              image_additional_mb: 10240

              optimize_image: no

              #exit_on_fail: no

              commands: |

                # Prevent Rust from complaining about $HOME not matching eid home

                export HOME=/root

                # Workaround to CI worker being stuck on Updating crates.io index

                export CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

                # Install setup prerequisites

                apt-get update -y --allow-releaseinfo-change

                apt-get autoremove -y

                apt-get install -y --no-install-recommends --no-install-suggests curl libwebkit2gtk-4.1-dev build-essential libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev patchelf libfuse2 file

                curl https://sh.rustup.rs -sSf | sh -s -- -y

                . "$HOME/.cargo/env"

                curl -fsSL https://deb.nodesource.com/setup_lts.x | bash

                apt-get install -y nodejs

                # Install frontend dependencies

                npm install

                # Build the application

                npm run tauri build -- --verbose

          - name: Get app version

            run: echo "APP_VERSION=$(jq -r .version src-tauri/tauri.conf.json)" >> $GITHUB_ENV

          # TODO: Combine this with the basic workflow and upload the files to the Release.

          - name: Upload deb bundle

            uses: actions/upload-artifact@v3

            with:

              name: Debian Bundle

              path: ${{ github.workspace }}/src-tauri/target/release/bundle/deb/appname_${{ env.APP_VERSION }}_${{ matrix.deb }}.deb

          - name: Upload rpm bundle

            uses: actions/upload-artifact@v3

            with:

              name: RPM Bundle

              path: ${{ github.workspace }}/src-tauri/target/release/bundle/rpm/appname-${{ env.APP_VERSION }}-1.${{ matrix.rpm }}.rpm

          - name: Upload appimage bundle

            uses: actions/upload-artifact@v3

            with:

              name: AppImage Bundle

              path: ${{ github.workspace }}/src-tauri/target/release/bundle/appimage/appname_${{ env.APP_VERSION }}_${{ matrix.appimage }}.AppImage

## 故障排除

名为“故障排除”的章节

### GitHub 环境令牌

名为“GitHub 环境令牌”的章节

GitHub 令牌是由 GitHub 为每次工作流程运行自动颁发的，无需额外配置，这意味着不存在密钥泄露的风险。但是，此令牌默认仅具有读取权限，您在运行工作流程时可能会收到“Resource not accessible by integration”（集成无法访问资源）错误。如果发生这种情况，您可能需要向该令牌添加写入权限。要执行此操作，请转到您的 GitHub 项目设置，选择 `Actions`，向下滚动到 `Workflow permissions`，并选中“Read and write permissions”。

您可以通过工作流程中的这一行代码查看传递给工作流程的 GitHub 令牌：

    env:

      GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}