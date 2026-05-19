# 命令行界面

_Source: https://v2.tauri.org.cn/reference/cli/_

Tauri 命令行界面 (CLI) 是您在整个开发生命周期中与 Tauri 交互的方式。

您可以使用您选择的包管理器将 Tauri CLI 添加到当前项目中

  * npm
  * yarn
  * pnpm
  * deno
  * cargo

    npm install --save-dev @tauri-apps/cli@latest

    yarn add -D @tauri-apps/cli@latest

    pnpm add -D @tauri-apps/cli@latest

    deno add -D npm:@tauri-apps/cli@latest

    cargo install tauri-cli --version "^2.0.0" --locked

开发插件

有关开发插件的 CLI 命令，请访问 [开发 Tauri 插件指南](/develop/plugins/)。

## 命令列表

名为“命令列表”的章节

命令| 描述
---|---
`init`| 在现有目录中初始化 Tauri 项目
`dev`| 以开发模式运行您的应用
`build`| 以发布模式构建您的应用并生成安装包和安装程序
`bundle`| 为您的应用生成安装包和安装程序（已由 `tauri build` 构建）
`android`| Android 命令
`android init`| 在项目中初始化 Android 目标
`android dev`| 在 Android 上以开发模式运行您的应用
`android build`| 以发布模式构建 Android 应用并生成 APK 和 AAB
`android run`| 在 Android 上以生产模式运行您的应用
`ios`| iOS 命令
`ios init`| 在项目中初始化 iOS 目标
`ios dev`| 在 iOS 上以开发模式运行您的应用
`ios build`| 以发布模式构建 iOS 应用并生成 IPA
`ios run`| 在 iOS 上以生产模式运行您的应用
`migrate`| 从 v1 迁移到 v2
`info`| 显示有关环境、Rust、Node.js 及其版本的简明信息列表，以及一些相关的项目配置
`add`| 向项目添加 Tauri 插件
`remove`| 从项目中删除 Tauri 插件
`plugin`| 管理或创建 Tauri 插件
`plugin new`| 初始化一个新的 Tauri 插件项目
`plugin init`| 在现有目录上初始化 Tauri 插件项目
`plugin android`| 管理 Tauri 插件的 Android 项目
`plugin ios`| 管理 Tauri 插件的 iOS 项目
`plugin android init`| 为现有 Tauri 插件初始化 Android 项目
`plugin ios init`| 为现有 Tauri 插件初始化 iOS 项目
`icon`| 为所有主要平台生成各种图标
`signer`| 生成用于 Tauri 更新程序的签名密钥或对文件进行签名
`signer sign`| 对文件进行签名
`signer generate`| 生成用于对文件进行签名的新签名密钥
`completions`| 为 Bash、Zsh、PowerShell 或 Fish 生成 Tauri CLI Shell 补全脚本
`permission`| 管理或创建您应用或插件的权限
`permission new`| 创建一个新的权限文件
`permission add`| 向能力 (capabilities) 添加权限
`permission rm`| 删除权限文件及其在任何能力中的引用
`permission ls`| 列出应用程序可用的权限
`capability`| 管理或创建您应用的能力 (capabilities)
`capability new`| 创建一个新的权限文件
`inspect`| 检查 Tauri 使用的值
`inspect wix-upgrade-code`| 打印从 productName 导出的 MSI 安装程序使用的默认升级代码

### `init`

名为“init”的章节

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri init

    yarn tauri init

    pnpm tauri init

    deno task tauri init

    bun tauri init

    cargo tauri init

    Initialize a Tauri project in an existing directory

    Usage: tauri init [OPTIONS]

    Options:

          --ci

              Skip prompting for values [env: CI=true]

      -v, --verbose...

              Enables verbose logging

      -f, --force

              Force init to overwrite the src-tauri folder

      -l, --log

              Enables logging

      -d, --directory <DIRECTORY>

              Set target directory for init [default: /opt/build/repo/packages/cli-generator]

      -t, --tauri-path <TAURI_PATH>

              Path of the Tauri project to use (relative to the cwd)

      -A, --app-name <APP_NAME>

              Name of your Tauri application

      -W, --window-title <WINDOW_TITLE>

              Window title of your Tauri application

      -D, --frontend-dist <FRONTEND_DIST>

              Web assets location, relative to <project-dir>/src-tauri

      -P, --dev-url <DEV_URL>

              Url of your dev server

          --before-dev-command <BEFORE_DEV_COMMAND>

              A shell command to run before `tauri dev` kicks in

          --before-build-command <BEFORE_BUILD_COMMAND>

              A shell command to run before `tauri build` kicks in

      -h, --help

              Print help

      -V, --version

              Print version

### `dev`

名为“dev”的章节

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri dev

    yarn tauri dev

    pnpm tauri dev

    deno task tauri dev

    bun tauri dev

    cargo tauri dev

    Run your app in development mode with hot-reloading for the Rust code. It makes use of the `build.devUrl` property from your `tauri.conf.json` file. It also runs your `build.beforeDevCommand` which usually starts your frontend devServer.

    Usage: tauri dev [OPTIONS] [ARGS]...

    Arguments:

      [ARGS]...

              Command line arguments passed to the runner. Use `--` to explicitly mark the start of the arguments. Arguments after a second `--` are passed to the application e.g. `tauri dev -- [runnerArgs] -- [appArgs]`

    Options:

      -r, --runner <RUNNER>

              Binary to use to run the application

      -v, --verbose...

              Enables verbose logging

      -t, --target <TARGET>

              Target triple to build against

      -f, --features [<FEATURES>...]

              List of cargo features to activate

      -e, --exit-on-panic

              Exit on panic

      -c, --config <CONFIG>

              JSON strings or paths to JSON, JSON5 or TOML files to merge with the default configuration file

              Configurations are merged in the order they are provided, which means a particular value overwrites previous values when a config key-value pair conflicts.

              Note that a platform-specific file is looked up and merged with the default file by default (tauri.macos.conf.json, tauri.linux.conf.json, tauri.windows.conf.json, tauri.android.conf.json and tauri.ios.conf.json) but you can use this for more specific use cases such as different build flavors.

          --release

              Run the code in release mode

          --no-dev-server-wait

              Skip waiting for the frontend dev server to start before building the tauri application

              [env: TAURI_CLI_NO_DEV_SERVER_WAIT=]

          --no-watch

              Disable the file watcher

          --additional-watch-folders <ADDITIONAL_WATCH_FOLDERS>

              Additional paths to watch for changes

          --no-dev-server

              Disable the built-in dev server for static files

          --port <PORT>

              Specify port for the built-in dev server for static files. Defaults to 1430

              [env: TAURI_CLI_PORT=]

      -h, --help

              Print help (see a summary with '-h')

      -V, --version

              Print version

### `build`

名为“build”的章节

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

    Build your app in release mode and generate bundles and installers. It makes use of the `build.frontendDist` property from your `tauri.conf.json` file. It also runs your `build.beforeBuildCommand` which usually builds your frontend into `build.frontendDist`. This will also run `build.beforeBundleCommand` before generating the bundles and installers of your app.

    Usage: tauri build [OPTIONS] [ARGS]...

    Arguments:

      [ARGS]...

              Command line arguments passed to the runner. Use `--` to explicitly mark the start of the arguments

    Options:

      -r, --runner <RUNNER>

              Binary to use to build the application, defaults to `cargo`

      -v, --verbose...

              Enables verbose logging

      -d, --debug

              Builds with the debug flag

      -t, --target <TARGET>

              Target triple to build against.

              It must be one of the values outputted by `$rustc --print target-list` or `universal-apple-darwin` for an universal macOS application.

              Note that compiling an universal macOS application requires both `aarch64-apple-darwin` and `x86_64-apple-darwin` targets to be installed.

      -f, --features [<FEATURES>...]

              Space or comma separated list of features to activate

      -b, --bundles [<BUNDLES>...]

              Space or comma separated list of bundles to package

              [possible values: deb, rpm, appimage]

          --no-bundle

              Skip the bundling step even if `bundle > active` is `true` in tauri config

      -c, --config <CONFIG>

              JSON strings or paths to JSON, JSON5 or TOML files to merge with the default configuration file

              Configurations are merged in the order they are provided, which means a particular value overwrites previous values when a config key-value pair conflicts.

              Note that a platform-specific file is looked up and merged with the default file by default (tauri.macos.conf.json, tauri.linux.conf.json, tauri.windows.conf.json, tauri.android.conf.json and tauri.ios.conf.json) but you can use this for more specific use cases such as different build flavors.

          --ci

              Skip prompting for values

              [env: CI=true]

          --skip-stapling

              Whether to wait for notarization to finish and `staple` the ticket onto the app.

              Gatekeeper will look for stapled tickets to tell whether your app was notarized without reaching out to Apple's servers which is helpful in offline environments.

              Enabling this option will also result in `tauri build` not waiting for notarization to finish which is helpful for the very first time your app is notarized as this can take multiple hours. On subsequent runs, it's recommended to disable this setting again.

          --ignore-version-mismatches

              Do not error out if a version mismatch is detected on a Tauri package.

              Only use this when you are sure the mismatch is incorrectly detected as version mismatched Tauri packages can lead to unknown behavior.

          --no-sign

              Skip code signing when bundling the app

      -h, --help

              Print help (see a summary with '-h')

      -V, --version

              Print version

### `bundle`

名为“bundle”的章节

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri bundle

    yarn tauri bundle

    pnpm tauri bundle

    deno task tauri bundle

    bun tauri bundle

    cargo tauri bundle

    Generate bundles and installers for your app (already built by `tauri build`). This run `build.beforeBundleCommand` before generating the bundles and installers of your app.

    Usage: tauri bundle [OPTIONS]

    Options:

      -d, --debug

              Builds with the debug flag

      -v, --verbose...

              Enables verbose logging

      -b, --bundles [<BUNDLES>...]

              Space or comma separated list of bundles to package

              [possible values: deb, rpm, appimage]

      -c, --config <CONFIG>

              JSON strings or paths to JSON, JSON5 or TOML files to merge with the default configuration file

              Configurations are merged in the order they are provided, which means a particular value overwrites previous values when a config key-value pair conflicts.

              Note that a platform-specific file is looked up and merged with the default file by default (tauri.macos.conf.json, tauri.linux.conf.json, tauri.windows.conf.json, tauri.android.conf.json and tauri.ios.conf.json) but you can use this for more specific use cases such as different build flavors.

      -f, --features [<FEATURES>...]

              Space or comma separated list of features, should be the same features passed to `tauri build` if any

      -t, --target <TARGET>

              Target triple to build against.

              It must be one of the values outputted by `$rustc --print target-list` or `universal-apple-darwin` for an universal macOS application.

              Note that compiling an universal macOS application requires both `aarch64-apple-darwin` and `x86_64-apple-darwin` targets to be installed.

          --ci

              Skip prompting for values

              [env: CI=true]

          --skip-stapling

              Whether to wait for notarization to finish and `staple` the ticket onto the app.

              Gatekeeper will look for stapled tickets to tell whether your app was notarized without reaching out to Apple's servers which is helpful in offline environments.

              Enabling this option will also result in `tauri build` not waiting for notarization to finish which is helpful for the very first time your app is notarized as this can take multiple hours. On subsequent runs, it's recommended to disable this setting again.

          --no-sign

              Skip code signing during the build or bundling process.

              Useful for local development and CI environments where signing certificates or environment variables are not available or not needed.

      -h, --help

              Print help (see a summary with '-h')

      -V, --version

              Print version

### `android`

名为“android”的章节

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri android

    yarn tauri android

    pnpm tauri android

    deno task tauri android

    bun tauri android

    cargo tauri android

    Android commands

    Usage: tauri android [OPTIONS] <COMMAND>

    Commands:

      init   Initialize Android target in the project

      dev    Run your app in development mode on Android

      build  Build your app in release mode for Android and generate APKs and AABs

      run    Run your app in production mode on Android

      help   Print this message or the help of the given subcommand(s)

    Options:

      -v, --verbose...  Enables verbose logging

      -h, --help        Print help

      -V, --version     Print version

#### `android init`

名为“android init”的章节

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri android init

    yarn tauri android init

    pnpm tauri android init

    deno task tauri android init

    bun tauri android init

    cargo tauri android init

    Initialize Android target in the project

    Usage: tauri android init [OPTIONS]

    Options:

          --ci

              Skip prompting for values

              [env: CI=true]

      -v, --verbose...

              Enables verbose logging

          --skip-targets-install

              Skips installing rust toolchains via rustup

      -c, --config <CONFIG>

              JSON strings or paths to JSON, JSON5 or TOML files to merge with the default configuration file

              Configurations are merged in the order they are provided, which means a particular value overwrites previous values when a config key-value pair conflicts.

              Note that a platform-specific file is looked up and merged with the default file by default (tauri.macos.conf.json, tauri.linux.conf.json, tauri.windows.conf.json, tauri.android.conf.json and tauri.ios.conf.json) but you can use this for more specific use cases such as different build flavors.

      -h, --help

              Print help (see a summary with '-h')

      -V, --version

              Print version

#### `android dev`

名为“android dev”的章节

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri android dev

    yarn tauri android dev

    pnpm tauri android dev

    deno task tauri android dev

    bun tauri android dev

    cargo tauri android dev

    Run your app in development mode on Android with hot-reloading for the Rust code. It makes use of the `build.devUrl` property from your `tauri.conf.json` file. It also runs your `build.beforeDevCommand` which usually starts your frontend devServer.

    Usage: tauri android dev [OPTIONS] [DEVICE] [-- <ARGS>...]

    Arguments:

      [DEVICE]

              Runs on the given device name

      [ARGS]...

              Command line arguments passed to the runner. Use `--` to explicitly mark the start of the arguments. e.g. `tauri android dev -- [runnerArgs]`

    Options:

      -f, --features [<FEATURES>...]

              List of cargo features to activate

      -v, --verbose...

              Enables verbose logging

      -e, --exit-on-panic

              Exit on panic

      -c, --config <CONFIG>

              JSON strings or paths to JSON, JSON5 or TOML files to merge with the default configuration file

              Configurations are merged in the order they are provided, which means a particular value overwrites previous values when a config key-value pair conflicts.

              Note that a platform-specific file is looked up and merged with the default file by default (tauri.macos.conf.json, tauri.linux.conf.json, tauri.windows.conf.json, tauri.android.conf.json and tauri.ios.conf.json) but you can use this for more specific use cases such as different build flavors.

          --release

              Run the code in release mode

          --no-dev-server-wait

              Skip waiting for the frontend dev server to start before building the tauri application

              [env: TAURI_CLI_NO_DEV_SERVER_WAIT=]

          --no-watch

              Disable the file watcher

          --additional-watch-folders <ADDITIONAL_WATCH_FOLDERS>

              Additional paths to watch for changes

      -o, --open

              Open Android Studio instead of trying to run on a connected device

          --force-ip-prompt

              Force prompting for an IP to use to connect to the dev server on mobile

          --host [<HOST>]

              Use the public network address for the development server. If an actual address it provided, it is used instead of prompting to pick one.

              On Windows we use the public network address by default.

              This option is particularly useful along the `--open` flag when you intend on running on a physical device.

              This replaces the devUrl configuration value to match the public network address host, it is your responsibility to set up your development server to listen on this address by using 0.0.0.0 as host for instance.

              When this is set or when running on an iOS device the CLI sets the `TAURI_DEV_HOST` environment variable so you can check this on your framework's configuration to expose the development server on the public network address.

              [default: <none>]

          --no-dev-server

              Disable the built-in dev server for static files

          --port <PORT>

              Specify port for the built-in dev server for static files. Defaults to 1430

              [env: TAURI_CLI_PORT=]

          --root-certificate-path <ROOT_CERTIFICATE_PATH>

              Path to the certificate file used by your dev server. Required for mobile dev when using HTTPS

              [env: TAURI_DEV_ROOT_CERTIFICATE_PATH=]

      -h, --help

              Print help (see a summary with '-h')

      -V, --version

              Print version

#### `android build`

名为“android build”的章节

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri android build

    yarn tauri android build

    pnpm tauri android build

    deno task tauri android build

    bun tauri android build

    cargo tauri android build

    Build your app in release mode for Android and generate APKs and AABs. It makes use of the `build.frontendDist` property from your `tauri.conf.json` file. It also runs your `build.beforeBuildCommand` which usually builds your frontend into `build.frontendDist`.

    Usage: tauri android build [OPTIONS] [-- <ARGS>...]

    Arguments:

      [ARGS]...

              Command line arguments passed to the runner. Use `--` to explicitly mark the start of the arguments. e.g. `tauri android build -- [runnerArgs]`

    Options:

      -d, --debug

              Builds with the debug flag

      -v, --verbose...

              Enables verbose logging

      -t, --target [<TARGETS>...]

              Which targets to build (all by default)

              [possible values: aarch64, armv7, i686, x86_64]

      -f, --features [<FEATURES>...]

              List of cargo features to activate

      -c, --config <CONFIG>

              JSON strings or paths to JSON, JSON5 or TOML files to merge with the default configuration file

              Configurations are merged in the order they are provided, which means a particular value overwrites previous values when a config key-value pair conflicts.

              Note that a platform-specific file is looked up and merged with the default file by default (tauri.macos.conf.json, tauri.linux.conf.json, tauri.windows.conf.json, tauri.android.conf.json and tauri.ios.conf.json) but you can use this for more specific use cases such as different build flavors.

          --split-per-abi

              Whether to split the APKs and AABs per ABIs

          --apk

              Build APKs

          --aab

              Build AABs

      -o, --open

              Open Android Studio

          --ci

              Skip prompting for values

              [env: CI=true]

          --ignore-version-mismatches

              Do not error out if a version mismatch is detected on a Tauri package.

              Only use this when you are sure the mismatch is incorrectly detected as version mismatched Tauri packages can lead to unknown behavior.

      -h, --help

              Print help (see a summary with '-h')

      -V, --version

              Print version

#### `android run`

名为“android run”的章节

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri android run

    yarn tauri android run

    pnpm tauri android run

    deno task tauri android run

    bun tauri android run

    cargo tauri android run

    Run your app in production mode on Android. It makes use of the `build.frontendDist` property from your `tauri.conf.json` file. It also runs your `build.beforeBuildCommand` which usually builds your frontend into `build.frontendDist`.

    Usage: tauri android run [OPTIONS] [DEVICE] [-- <ARGS>...]

    Arguments:

      [DEVICE]

              Runs on the given device name

      [ARGS]...

              Command line arguments passed to the runner. Use `--` to explicitly mark the start of the arguments. e.g. `tauri android build -- [runnerArgs]`

    Options:

      -r, --release

              Run the app in release mode

      -v, --verbose...

              Enables verbose logging

      -f, --features [<FEATURES>...]

              List of cargo features to activate

      -c, --config <CONFIG>

              JSON strings or paths to JSON, JSON5 or TOML files to merge with the default configuration file

              Configurations are merged in the order they are provided, which means a particular value overwrites previous values when a config key-value pair conflicts.

              Note that a platform-specific file is looked up and merged with the default file by default (tauri.macos.conf.json, tauri.linux.conf.json, tauri.windows.conf.json, tauri.android.conf.json and tauri.ios.conf.json) but you can use this for more specific use cases such as different build flavors.

          --no-watch

              Disable the file watcher

          --additional-watch-folders <ADDITIONAL_WATCH_FOLDERS>

              Additional paths to watch for changes

      -o, --open

              Open Android Studio

          --ignore-version-mismatches

              Do not error out if a version mismatch is detected on a Tauri package.

              Only use this when you are sure the mismatch is incorrectly detected as version mismatched Tauri packages can lead to unknown behavior.

      -h, --help

              Print help (see a summary with '-h')

      -V, --version

              Print version

### `ios`

名为“ios”的章节

_所有 iOS 命令仅在 macOS 主机上可用。_

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri ios

    yarn tauri ios

    pnpm tauri ios

    deno task tauri ios

    bun tauri ios

    cargo tauri ios

    iOS commands

    Usage: tauri ios [OPTIONS] <COMMAND>

    Commands:

      init   Initialize iOS target in the project

      dev    Run your app in development mode on iOS

      build  Build your app in release mode for iOS and generate IPAs

      run    Run your app in production mode on iOS

      help   Print this message or the help of the given subcommand(s)

    Options:

      -v, --verbose...  Enables verbose logging

      -h, --help        Print help

      -V, --version     Print version

#### `ios init`

名为“ios init”的章节

_所有 iOS 命令仅在 macOS 主机上可用。_

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri ios init

    yarn tauri ios init

    pnpm tauri ios init

    deno task tauri ios init

    bun tauri ios init

    cargo tauri ios init

    Initialize iOS target in the project

    Usage: tauri ios init [OPTIONS]

    Options:

          --ci

              Skip prompting for values

              [env: CI=]

      -v, --verbose...

              Enables verbose logging

      -r, --reinstall-deps

              Reinstall dependencies

          --skip-targets-install

              Skips installing rust toolchains via rustup

      -c, --config <CONFIG>

              JSON strings or paths to JSON, JSON5 or TOML files to merge with the default configuration file

              Configurations are merged in the order they are provided, which means a particular value overwrites previous values when a config key-value pair conflicts.

              Note that a platform-specific file is looked up and merged with the default file by default (tauri.macos.conf.json, tauri.linux.conf.json, tauri.windows.conf.json, tauri.android.conf.json and tauri.ios.conf.json) but you can use this for more specific use cases such as different build flavors.

      -h, --help

              Print help (see a summary with '-h')

      -V, --version

              Print version

#### `ios dev`

名为“ios dev”的章节

_所有 iOS 命令仅在 macOS 主机上可用。_

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri ios dev

    yarn tauri ios dev

    pnpm tauri ios dev

    deno task tauri ios dev

    bun tauri ios dev

    cargo tauri ios dev

    Run your app in development mode on iOS with hot-reloading for the Rust code.

    It makes use of the `build.devUrl` property from your `tauri.conf.json` file.

    It also runs your `build.beforeDevCommand` which usually starts your frontend devServer.

    When connected to a physical iOS device, the public network address must be used instead of `localhost`

    for the devUrl property. Tauri makes that change automatically, but your dev server might need

    a different configuration to listen on the public address. You can check the `TAURI_DEV_HOST`

    environment variable to determine whether the public network should be used or not.

    Usage: tauri ios dev [OPTIONS] [DEVICE] [-- <ARGS>...]

    Arguments:

      [DEVICE]

              Runs on the given device name

      [ARGS]...

              Command line arguments passed to the runner. Use `--` to explicitly mark the start of the arguments. e.g. `tauri ios dev -- [runnerArgs]`

    Options:

      -f, --features [<FEATURES>...]

              List of cargo features to activate

      -v, --verbose...

              Enables verbose logging

      -e, --exit-on-panic

              Exit on panic

      -c, --config <CONFIG>

              JSON strings or paths to JSON, JSON5 or TOML files to merge with the default configuration file

              Configurations are merged in the order they are provided, which means a particular value overwrites previous values when a config key-value pair conflicts.

              Note that a platform-specific file is looked up and merged with the default file by default (tauri.macos.conf.json, tauri.linux.conf.json, tauri.windows.conf.json, tauri.android.conf.json and tauri.ios.conf.json) but you can use this for more specific use cases such as different build flavors.

          --release

              Run the code in release mode

          --no-dev-server-wait

              Skip waiting for the frontend dev server to start before building the tauri application

              [env: TAURI_CLI_NO_DEV_SERVER_WAIT=]

          --no-watch

              Disable the file watcher

          --additional-watch-folders <ADDITIONAL_WATCH_FOLDERS>

              Additional paths to watch for changes

      -o, --open

              Open Xcode instead of trying to run on a connected device

          --force-ip-prompt

              Force prompting for an IP to use to connect to the dev server on mobile

          --host [<HOST>]

              Use the public network address for the development server. If an actual address it provided, it is used instead of prompting to pick one.

              This option is particularly useful along the `--open` flag when you intend on running on a physical device.

              This replaces the devUrl configuration value to match the public network address host, it is your responsibility to set up your development server to listen on this address by using 0.0.0.0 as host for instance.

              When this is set or when running on an iOS device the CLI sets the `TAURI_DEV_HOST` environment variable so you can check this on your framework's configuration to expose the development server on the public network address.

              [default: <none>]

          --no-dev-server

              Disable the built-in dev server for static files

          --port <PORT>

              Specify port for the built-in dev server for static files. Defaults to 1430

              [env: TAURI_CLI_PORT=]

          --root-certificate-path <ROOT_CERTIFICATE_PATH>

              Path to the certificate file used by your dev server. Required for mobile dev when using HTTPS

              [env: TAURI_DEV_ROOT_CERTIFICATE_PATH=]

      -h, --help

              Print help (see a summary with '-h')

      -V, --version

              Print version

#### `ios build`

名为“ios build”的章节

_所有 iOS 命令仅在 macOS 主机上可用。_

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri ios build

    yarn tauri ios build

    pnpm tauri ios build

    deno task tauri ios build

    bun tauri ios build

    cargo tauri ios build

    Build your app in release mode for iOS and generate IPAs. It makes use of the `build.frontendDist` property from your `tauri.conf.json` file. It also runs your `build.beforeBuildCommand` which usually builds your frontend into `build.frontendDist`.

    Usage: tauri ios build [OPTIONS] [-- <ARGS>...]

    Arguments:

      [ARGS]...

              Command line arguments passed to the runner. Use `--` to explicitly mark the start of the arguments. e.g. `tauri ios build -- [runnerArgs]`

    Options:

      -d, --debug

              Builds with the debug flag

      -v, --verbose...

              Enables verbose logging

      -t, --target [<TARGETS>...]

              Which targets to build

              [default: aarch64]

              [possible values: aarch64, aarch64-sim, x86_64]

      -f, --features [<FEATURES>...]

              List of cargo features to activate

      -c, --config <CONFIG>

              JSON strings or paths to JSON, JSON5 or TOML files to merge with the default configuration file

              Configurations are merged in the order they are provided, which means a particular value overwrites previous values when a config key-value pair conflicts.

              Note that a platform-specific file is looked up and merged with the default file by default (tauri.macos.conf.json, tauri.linux.conf.json, tauri.windows.conf.json, tauri.android.conf.json and tauri.ios.conf.json) but you can use this for more specific use cases such as different build flavors.

          --build-number <BUILD_NUMBER>

              Build number to append to the app version

      -o, --open

              Open Xcode

          --ci

              Skip prompting for values

              [env: CI=]

          --export-method <EXPORT_METHOD>

              Describes how Xcode should export the archive.

              Use this to create a package ready for the App Store (app-store-connect option) or TestFlight (release-testing option).

              [possible values: app-store-connect, release-testing, debugging]

          --ignore-version-mismatches

              Do not error out if a version mismatch is detected on a Tauri package.

              Only use this when you are sure the mismatch is incorrectly detected as version mismatched Tauri packages can lead to unknown behavior.

      -h, --help

              Print help (see a summary with '-h')

      -V, --version

              Print version

#### `ios run`

名为“ios run”的章节

_所有 iOS 命令仅在 macOS 主机上可用。_

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri ios run

    yarn tauri ios run

    pnpm tauri ios run

    deno task tauri ios run

    bun tauri ios run

    cargo tauri ios run

    Run your app in production mode on iOS. It makes use of the `build.frontendDist` property from your `tauri.conf.json` file. It also runs your `build.beforeBuildCommand` which usually builds your frontend into `build.frontendDist`.

    Usage: tauri ios run [OPTIONS] [DEVICE] [-- <ARGS>...]

    Arguments:

      [DEVICE]

              Runs on the given device name

      [ARGS]...

              Command line arguments passed to the runner. Use `--` to explicitly mark the start of the arguments. e.g. `tauri android build -- [runnerArgs]`

    Options:

      -r, --release

              Run the app in release mode

      -v, --verbose...

              Enables verbose logging

      -f, --features [<FEATURES>...]

              List of cargo features to activate

      -c, --config <CONFIG>

              JSON strings or paths to JSON, JSON5 or TOML files to merge with the default configuration file

              Configurations are merged in the order they are provided, which means a particular value overwrites previous values when a config key-value pair conflicts.

              Note that a platform-specific file is looked up and merged with the default file by default (tauri.macos.conf.json, tauri.linux.conf.json, tauri.windows.conf.json, tauri.android.conf.json and tauri.ios.conf.json) but you can use this for more specific use cases such as different build flavors.

          --no-watch

              Disable the file watcher

          --additional-watch-folders <ADDITIONAL_WATCH_FOLDERS>

              Additional paths to watch for changes

      -o, --open

              Open Xcode

          --ignore-version-mismatches

              Do not error out if a version mismatch is detected on a Tauri package.

              Only use this when you are sure the mismatch is incorrectly detected as version mismatched Tauri packages can lead to unknown behavior.

      -h, --help

              Print help (see a summary with '-h')

      -V, --version

              Print version

### `migrate`

名为“migrate”的章节

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri migrate

    yarn tauri migrate

    pnpm tauri migrate

    deno task tauri migrate

    bun tauri migrate

    cargo tauri migrate

    Migrate from v1 to v2

    Usage: tauri migrate [OPTIONS]

    Options:

      -v, --verbose...  Enables verbose logging

      -h, --help        Print help

      -V, --version     Print version

### `info`

名为“info”的章节

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri info

    yarn tauri info

    pnpm tauri info

    deno task tauri info

    bun tauri info

    cargo tauri info

    Show a concise list of information about the environment, Rust, Node.js and their versions as well as a few relevant project configurations

    Usage: tauri info [OPTIONS]

    Options:

          --interactive  Interactive mode to apply automatic fixes

      -v, --verbose...   Enables verbose logging

      -h, --help         Print help

      -V, --version      Print version

### `add`

名为“add”的章节

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri add

    yarn tauri add

    pnpm tauri add

    deno task tauri add

    bun tauri add

    cargo tauri add

    Add a tauri plugin to the project

    Usage: tauri add [OPTIONS] <PLUGIN>

    Arguments:

      <PLUGIN>  The plugin to add

    Options:

      -t, --tag <TAG>        Git tag to use

      -v, --verbose...       Enables verbose logging

      -r, --rev <REV>        Git rev to use

      -b, --branch <BRANCH>  Git branch to use

          --no-fmt           Don't format code with rustfmt

      -h, --help             Print help

      -V, --version          Print version

### `remove`

名为“remove”的章节

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri remove

    yarn tauri remove

    pnpm tauri remove

    deno task tauri remove

    bun tauri remove

    cargo tauri remove

    Remove a tauri plugin from the project

    Usage: tauri remove [OPTIONS] <PLUGIN>

    Arguments:

      <PLUGIN>  The plugin to remove

    Options:

      -v, --verbose...  Enables verbose logging

      -h, --help        Print help

      -V, --version     Print version

### `plugin`

名为“plugin”的章节

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri plugin

    yarn tauri plugin

    pnpm tauri plugin

    deno task tauri plugin

    bun tauri plugin

    cargo tauri plugin

    Manage or create Tauri plugins

    Usage: tauri plugin [OPTIONS] <COMMAND>

    Commands:

      new      Initializes a new Tauri plugin project

      init     Initialize a Tauri plugin project on an existing directory

      android  Manage the Android project for a Tauri plugin

      ios      Manage the iOS project for a Tauri plugin

      help     Print this message or the help of the given subcommand(s)

    Options:

      -v, --verbose...  Enables verbose logging

      -h, --help        Print help

      -V, --version     Print version

#### `plugin new`

名为“plugin new”的章节

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri plugin new

    yarn tauri plugin new

    pnpm tauri plugin new

    deno task tauri plugin new

    bun tauri plugin new

    cargo tauri plugin new

    Initializes a new Tauri plugin project

    Usage: tauri plugin new [OPTIONS] <PLUGIN_NAME>

    Arguments:

      <PLUGIN_NAME>

              Name of your Tauri plugin

    Options:

          --no-api

              Initializes a Tauri plugin without the TypeScript API

      -v, --verbose...

              Enables verbose logging

          --no-example

              Initialize without an example project

      -d, --directory <DIRECTORY>

              Set target directory for init

      -a, --author <AUTHOR>

              Author name

          --android

              Whether to initialize an Android project for the plugin

          --ios

              Whether to initialize an iOS project for the plugin

          --mobile

              Whether to initialize Android and iOS projects for the plugin

          --ios-framework <IOS_FRAMEWORK>

              Type of framework to use for the iOS project

              [default: spm]

              Possible values:

              - spm:   Swift Package Manager project

              - xcode: Xcode project

          --github-workflows

              Generate github workflows

      -t, --tauri-path <TAURI_PATH>

              Path of the Tauri project to use (relative to the cwd)

      -h, --help

              Print help (see a summary with '-h')

      -V, --version

              Print version

#### `plugin init`

名为“plugin init”的章节

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri plugin init

    yarn tauri plugin init

    pnpm tauri plugin init

    deno task tauri plugin init

    bun tauri plugin init

    cargo tauri plugin init

    Initialize a Tauri plugin project on an existing directory

    Usage: tauri plugin init [OPTIONS] [PLUGIN_NAME]

    Arguments:

      [PLUGIN_NAME]

              Name of your Tauri plugin. If not specified, it will be inferred from the current directory

    Options:

          --no-api

              Initializes a Tauri plugin without the TypeScript API

      -v, --verbose...

              Enables verbose logging

          --no-example

              Initialize without an example project

      -d, --directory <DIRECTORY>

              Set target directory for init

              [default: /opt/build/repo/packages/cli-generator]

      -a, --author <AUTHOR>

              Author name

          --android

              Whether to initialize an Android project for the plugin

          --ios

              Whether to initialize an iOS project for the plugin

          --mobile

              Whether to initialize Android and iOS projects for the plugin

          --ios-framework <IOS_FRAMEWORK>

              Type of framework to use for the iOS project

              [default: spm]

              Possible values:

              - spm:   Swift Package Manager project

              - xcode: Xcode project

          --github-workflows

              Generate github workflows

      -t, --tauri-path <TAURI_PATH>

              Path of the Tauri project to use (relative to the cwd)

      -h, --help

              Print help (see a summary with '-h')

      -V, --version

              Print version

#### `plugin android`

名为“plugin android”的章节

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri plugin android

    yarn tauri plugin android

    pnpm tauri plugin android

    deno task tauri plugin android

    bun tauri plugin android

    cargo tauri plugin android

    Manage the Android project for a Tauri plugin

    Usage: tauri plugin android [OPTIONS] <COMMAND>

    Commands:

      init  Initializes the Android project for an existing Tauri plugin

      help  Print this message or the help of the given subcommand(s)

    Options:

      -v, --verbose...  Enables verbose logging

      -h, --help        Print help

      -V, --version     Print version

##### `plugin android init`

名为“plugin android init”的章节

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri plugin android init

    yarn tauri plugin android init

    pnpm tauri plugin android init

    deno task tauri plugin android init

    bun tauri plugin android init

    cargo tauri plugin android init

    Initializes the Android project for an existing Tauri plugin

    Usage: tauri plugin android init [OPTIONS] [PLUGIN_NAME]

    Arguments:

      [PLUGIN_NAME]  Name of your Tauri plugin. Must match the current plugin's name. If not specified, it will be inferred from the current directory

    Options:

      -o, --out-dir <OUT_DIR>  The output directory [default: /opt/build/repo/packages/cli-generator]

      -v, --verbose...         Enables verbose logging

      -h, --help               Print help

      -V, --version            Print version

#### `plugin ios`

名为“plugin ios”的章节

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri plugin ios

    yarn tauri plugin ios

    pnpm tauri plugin ios

    deno task tauri plugin ios

    bun tauri plugin ios

    cargo tauri plugin ios

    Manage the iOS project for a Tauri plugin

    Usage: tauri plugin ios [OPTIONS] <COMMAND>

    Commands:

      init  Initializes the iOS project for an existing Tauri plugin

      help  Print this message or the help of the given subcommand(s)

    Options:

      -v, --verbose...  Enables verbose logging

      -h, --help        Print help

      -V, --version     Print version

##### `plugin ios init`

名为“plugin ios init”的章节

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri plugin ios init

    yarn tauri plugin ios init

    pnpm tauri plugin ios init

    deno task tauri plugin ios init

    bun tauri plugin ios init

    cargo tauri plugin ios init

    Initializes the iOS project for an existing Tauri plugin

    Usage: tauri plugin ios init [OPTIONS] [PLUGIN_NAME]

    Arguments:

      [PLUGIN_NAME]

              Name of your Tauri plugin. Must match the current plugin's name. If not specified, it will be inferred from the current directory

    Options:

      -o, --out-dir <OUT_DIR>

              The output directory

              [default: /opt/build/repo/packages/cli-generator]

      -v, --verbose...

              Enables verbose logging

          --ios-framework <IOS_FRAMEWORK>

              Type of framework to use for the iOS project

              [default: spm]

              Possible values:

              - spm:   Swift Package Manager project

              - xcode: Xcode project

      -h, --help

              Print help (see a summary with '-h')

      -V, --version

              Print version

### `icon`

名为“icon”的章节

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri icon

    yarn tauri icon

    pnpm tauri icon

    deno task tauri icon

    bun tauri icon

    cargo tauri icon

    Generate various icons for all major platforms

    Usage: tauri icon [OPTIONS] [INPUT]

    Arguments:

      [INPUT]

              Path to the source icon (squared PNG or SVG file with transparency) or a manifest file.

              The manifest file is a JSON file with the following structure: { "default": "app-icon.png", "bg_color": "#fff", "android_bg": "app-icon-bg.png", "android_fg": "app-icon-fg.png", "android_fg_scale": 85, "android_monochrome": "app-icon-monochrome.png" }

              All file paths defined in the manifest JSON are relative to the manifest file path.

              Only the `default` manifest property is required.

              The `bg_color` manifest value overwrites the `--ios-color` option if set.

              [default: ./app-icon.png]

    Options:

      -o, --output <OUTPUT>

              Output directory. Default: 'icons' directory next to the tauri.conf.json file

      -v, --verbose...

              Enables verbose logging

      -p, --png <PNG>

              Custom PNG icon sizes to generate. When set, the default icons are not generated

          --ios-color <IOS_COLOR>

              The background color of the iOS icon - string as defined in the W3C's CSS Color Module Level 4 <https://www.w3.org/TR/css-color-4/>

              [default: #fff]

      -h, --help

              Print help (see a summary with '-h')

      -V, --version

              Print version

### `signer`

名为“signer”的章节

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri signer

    yarn tauri signer

    pnpm tauri signer

    deno task tauri signer

    bun tauri signer

    cargo tauri signer

    Generate signing keys for Tauri updater or sign files

    Usage: tauri signer [OPTIONS] <COMMAND>

    Commands:

      sign      Sign a file

      generate  Generate a new signing key to sign files

      help      Print this message or the help of the given subcommand(s)

    Options:

      -v, --verbose...  Enables verbose logging

      -h, --help        Print help

      -V, --version     Print version

#### `signer sign`

名为“signer sign”的章节

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri signer sign

    yarn tauri signer sign

    pnpm tauri signer sign

    deno task tauri signer sign

    bun tauri signer sign

    cargo tauri signer sign

    Sign a file

    Usage: tauri signer sign [OPTIONS] <FILE>

    Arguments:

      <FILE>  Sign the specified file

    Options:

      -k, --private-key <PRIVATE_KEY>

              Load the private key from a string [env: TAURI_SIGNING_PRIVATE_KEY=]

      -v, --verbose...

              Enables verbose logging

      -f, --private-key-path <PRIVATE_KEY_PATH>

              Load the private key from a file [env: TAURI_SIGNING_PRIVATE_KEY_PATH=]

      -p, --password <PASSWORD>

              Set private key password when signing [env: TAURI_SIGNING_PRIVATE_KEY_PASSWORD=]

      -h, --help

              Print help

      -V, --version

              Print version

#### `signer generate`

名为“signer generate”的章节

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri signer generate

    yarn tauri signer generate

    pnpm tauri signer generate

    deno task tauri signer generate

    bun tauri signer generate

    cargo tauri signer generate

    Generate a new signing key to sign files

    Usage: tauri signer generate [OPTIONS]

    Options:

      -p, --password <PASSWORD>      Set private key password when signing

      -v, --verbose...               Enables verbose logging

      -w, --write-keys <WRITE_KEYS>  Write private key to a file

      -f, --force                    Overwrite private key even if it exists on the specified path

          --ci                       Skip prompting for values [env: CI=true]

      -h, --help                     Print help

      -V, --version                  Print version

### `completions`

名为“completions”的章节

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri completions

    yarn tauri completions

    pnpm tauri completions

    deno task tauri completions

    bun tauri completions

    cargo tauri completions

    Generate Tauri CLI shell completions for Bash, Zsh, PowerShell or Fish

    Usage: tauri completions [OPTIONS] --shell <SHELL>

    Options:

      -s, --shell <SHELL>    Shell to generate a completion script for. [possible values: bash, elvish, fish, powershell, zsh]

      -v, --verbose...       Enables verbose logging

      -o, --output <OUTPUT>  Output file for the shell completions. By default the completions are printed to stdout

      -h, --help             Print help

      -V, --version          Print version

### `permission`

名为“permission”的章节

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri permission

    yarn tauri permission

    pnpm tauri permission

    deno task tauri permission

    bun tauri permission

    cargo tauri permission

    Manage or create permissions for your app or plugin

    Usage: tauri permission [OPTIONS] <COMMAND>

    Commands:

      new   Create a new permission file

      add   Add a permission to capabilities

      rm    Remove a permission file, and its reference from any capability

      ls    List permissions available to your application

      help  Print this message or the help of the given subcommand(s)

    Options:

      -v, --verbose...  Enables verbose logging

      -h, --help        Print help

      -V, --version     Print version

#### `permission new`

名为“permission new”的章节

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri permission new

    yarn tauri permission new

    pnpm tauri permission new

    deno task tauri permission new

    bun tauri permission new

    cargo tauri permission new

    Create a new permission file

    Usage: tauri permission new [OPTIONS] [IDENTIFIER]

    Arguments:

      [IDENTIFIER]  Permission identifier

    Options:

          --description <DESCRIPTION>  Permission description

      -v, --verbose...                 Enables verbose logging

      -a, --allow <ALLOW>              List of commands to allow

      -d, --deny <DENY>                List of commands to deny

          --format <FORMAT>            Output file format [default: json] [possible values: json, toml]

      -o, --out <OUT>                  The output file

      -h, --help                       Print help

      -V, --version                    Print version

#### `permission add`

名为“permission add”的章节

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri permission add

    yarn tauri permission add

    pnpm tauri permission add

    deno task tauri permission add

    bun tauri permission add

    cargo tauri permission add

    Add a permission to capabilities

    Usage: tauri permission add [OPTIONS] <IDENTIFIER> [CAPABILITY]

    Arguments:

      <IDENTIFIER>  Permission to add

      [CAPABILITY]  Capability to add the permission to

    Options:

      -v, --verbose...  Enables verbose logging

      -h, --help        Print help

      -V, --version     Print version

#### `permission rm`

名为“permission rm”的章节

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri permission rm

    yarn tauri permission rm

    pnpm tauri permission rm

    deno task tauri permission rm

    bun tauri permission rm

    cargo tauri permission rm

    Remove a permission file, and its reference from any capability

    Usage: tauri permission rm [OPTIONS] <IDENTIFIER>

    Arguments:

      <IDENTIFIER>

              Permission to remove.

              To remove all permissions for a given plugin, provide `<plugin-name>:*`

    Options:

      -v, --verbose...

              Enables verbose logging

      -h, --help

              Print help (see a summary with '-h')

      -V, --version

              Print version

#### `permission ls`

名为“permission ls”的章节

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri permission ls

    yarn tauri permission ls

    pnpm tauri permission ls

    deno task tauri permission ls

    bun tauri permission ls

    cargo tauri permission ls

    List permissions available to your application

    Usage: tauri permission ls [OPTIONS] [PLUGIN]

    Arguments:

      [PLUGIN]  Name of the plugin to list permissions

    Options:

      -f, --filter <FILTER>  Permission identifier filter

      -v, --verbose...       Enables verbose logging

      -h, --help             Print help

      -V, --version          Print version

### `capability`

名为“capability”的章节

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri capability

    yarn tauri capability

    pnpm tauri capability

    deno task tauri capability

    bun tauri capability

    cargo tauri capability

    Manage or create capabilities for your app

    Usage: tauri capability [OPTIONS] <COMMAND>

    Commands:

      new   Create a new permission file

      help  Print this message or the help of the given subcommand(s)

    Options:

      -v, --verbose...  Enables verbose logging

      -h, --help        Print help

      -V, --version     Print version

#### `capability new`

名为“capability new”的章节

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri capability new

    yarn tauri capability new

    pnpm tauri capability new

    deno task tauri capability new

    bun tauri capability new

    cargo tauri capability new

    Create a new permission file

    Usage: tauri capability new [OPTIONS] [IDENTIFIER]

    Arguments:

      [IDENTIFIER]  Capability identifier

    Options:

          --description <DESCRIPTION>  Capability description

      -v, --verbose...                 Enables verbose logging

          --windows <WINDOWS>          Capability windows

          --permission <PERMISSION>    Capability permissions

          --format <FORMAT>            Output file format [default: json] [possible values: json, toml]

      -o, --out <OUT>                  The output file

      -h, --help                       Print help

      -V, --version                    Print version

### `inspect`

名为“inspect”的章节

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri inspect

    yarn tauri inspect

    pnpm tauri inspect

    deno task tauri inspect

    bun tauri inspect

    cargo tauri inspect

    Inspect values used by Tauri

    Usage: tauri inspect [OPTIONS] <COMMAND>

    Commands:

      wix-upgrade-code  Print the default Upgrade Code used by MSI installer derived from productName

      help              Print this message or the help of the given subcommand(s)

    Options:

      -v, --verbose...  Enables verbose logging

      -h, --help        Print help

      -V, --version     Print version

#### `inspect wix-upgrade-code`

名为“inspect wix-upgrade-code”的章节

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri inspect wix-upgrade-code

    yarn tauri inspect wix-upgrade-code

    pnpm tauri inspect wix-upgrade-code

    deno task tauri inspect wix-upgrade-code

    bun tauri inspect wix-upgrade-code

    cargo tauri inspect wix-upgrade-code

    Print the default Upgrade Code used by MSI installer derived from productName

    Usage: tauri inspect wix-upgrade-code [OPTIONS]

    Options:

      -v, --verbose...  Enables verbose logging

      -h, --help        Print help

      -V, --version     Print version