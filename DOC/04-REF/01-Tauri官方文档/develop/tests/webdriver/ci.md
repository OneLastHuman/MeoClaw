# 持续集成

_Source: https://v2.tauri.org.cn/develop/tests/webdriver/ci/_

可以在你的 CI 中使用 [`tauri-driver`](https://crates.io/crates/tauri-driver) 运行 [WebDriver](https://www.w3.org/TR/webdriver/) 测试。以下示例使用了我们[之前一起构建](/develop/tests/webdriver/example/webdriverio/)的 [WebdriverIO](https://webdriverio.node.org.cn/) 示例以及 GitHub Actions。

WebDriver 测试是通过在 Linux 上创建虚拟显示器（fake display）来执行的。一些 CI 系统（例如 GitHub Actions）也支持在 Windows 上运行 WebDriver 测试。

## GitHub Actions

标题为“GitHub Actions”的章节

以下 GitHub Actions 假设：

  1. Tauri 应用程序位于 `src-tauri` 文件夹中。
  2. [WebDriverIO](https://webdriverio.node.org.cn/) 测试运行器位于 `e2e-tests` 目录中，并且在该目录中使用 `yarn test` 时会运行。

.github/workflows/webdriver.yml

    # run this action when the repository is pushed to

    on: [push]

    # the name of our workflow

    name: WebDriver

    jobs:

      # a single job named test

      test:

        # the display name of the test job

        name: WebDriverIO Test Runner

        # run on the matrix platform

        runs-on: ${{ matrix.platform }}

        strategy:

          # do not fail other matrix runs if one fails

          fail-fast: false

          # set all platforms our test should run on

          matrix:

            platform: [ubuntu-latest, windows-latest]

        # the steps our job runs **in order**

        steps:

          # checkout the code on the workflow runner

          - uses: actions/checkout@v4

          # install system dependencies that Tauri needs to compile on Linux.

          # note the extra dependencies for `tauri-driver` to run which are: `webkit2gtk-driver` and `xvfb`

          - name: Tauri dependencies

            if: matrix.platform == 'ubuntu-latest'

            run: |

              sudo apt-get update &&

              sudo apt-get install -y \

              libwebkit2gtk-4.1-dev \

              libayatana-appindicator3-dev \

              webkit2gtk-driver \

              xvfb

          # install a matching Microsoft Edge Driver version using msedgedriver-tool

          - name: install msdgedriver (Windows)

            if: matrix.platform == 'windows-latest'

            run: |

              cargo install --git https://github.com/chippers/msedgedriver-tool

              & "$HOME/.cargo/bin/msedgedriver-tool.exe"

              $PWD.Path >> $env:GITHUB_PATH

          # install latest stable Rust release

          - name: Setup rust-toolchain stable

            uses: dtolnay/rust-toolchain@stable

          # setup caching for the Rust target folder

          - name: Setup Rust cache

            uses: Swatinem/rust-cache@v2

            with:

              workspaces: src-tauri

          # we run our Rust tests before the webdriver tests to avoid testing a broken application

          - name: Cargo test

            run: cargo test

          # install the latest stable node version at the time of writing

          - name: Node 24

            uses: actions/setup-node@v4

            with:

              node-version: 24

              cache: 'yarn'

          # install the application Node.js dependencies with Yarn

          - name: Yarn install

            run: yarn install --frozen-lockfile

          # install the e2e-tests Node.js dependencies with Yarn

          - name: Yarn install

            run: yarn install --frozen-lockfile

            working-directory: e2e-tests

          # install the latest version of `tauri-driver`.

          # note: the tauri-driver version is independent of any other Tauri versions

          - name: Install tauri-driver

            run: cargo install tauri-driver --locked

          # run the WebdriverIO test suite on Linux.

          # we run it through `xvfb-run` (the dependency we installed earlier) to have a fake

          # display server which allows our application to run headless without any changes to the code

          - name: WebdriverIO (Linux)

            if: matrix.platform == 'ubuntu-latest'

            run: xvfb-run yarn test

            working-directory: e2e-tests

          # run the WebdriverIO test suite on Windows.

          # in this case we can run the tests directly.

          - name: WebdriverIO (Windows)

            if: matrix.platform == 'windows-latest'

            run: yarn test

            working-directory: e2e-tests