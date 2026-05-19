# 配置文件

_Source: https://v2.tauri.org.cn/develop/configuration-files/_

由于 Tauri 是一个构建应用程序的工具包，因此可能存在许多用于配置项目设置的文件。您可能会遇到的一些常见文件包括 `tauri.conf.json`、`package.json` 和 `Cargo.toml`。我们在本页面简要解释了每个文件，以帮助您明确应该修改哪些文件。

## Tauri 配置

标题为“Tauri 配置”的章节

Tauri 配置用于定义 Web 应用的来源、描述应用程序的元数据、配置打包、设置插件配置、通过配置窗口、托盘图标、菜单等来修改运行时行为。

该文件由 Tauri 运行时和 Tauri CLI 使用。您可以定义构建设置（例如 [在 `tauri build` 或 `tauri dev` 启动前运行的命令](/reference/config/#beforebuildcommand)）、设置 [应用程序名称](/reference/config/#productname) 和 [版本](/reference/config/#version)、[控制 Tauri 运行时](/reference/config/#appconfig) 以及 [配置插件](/reference/config/#plugins)。

提示

您可以在 [配置参考](/reference/config/) 中找到所有选项。

### 支持的格式

标题为“支持的格式”的章节

默认的 Tauri 配置格式是 JSON。通过在 `Cargo.toml` 中的 `tauri` 和 `tauri-build` 依赖项上添加 `config-json5` 或 `config-toml` 功能标志，可以启用 JSON5 或 TOML 格式。

Cargo.toml

    [build-dependencies]

    tauri-build = { version = "2.0.0", features = [ "config-json5" ] }

    [dependencies]

    tauri = { version = "2.0.0", features = [  "config-json5" ] }

结构和值在所有格式中都是相同的，但是格式应与各自文件的格式保持一致。

tauri.conf.json

    {

      build: {

        devUrl: 'https://:3000',

        // start the dev server

        beforeDevCommand: 'npm run dev',

      },

      bundle: {

        active: true,

        icon: ['icons/app.png'],

      },

      app: {

        windows: [

          {

            title: 'MyApp',

          },

        ],

      },

      plugins: {

        updater: {

          pubkey: 'updater pub key',

          endpoints: ['https://my.app.updater/{{target}}/{{current_version}}'],

        },

      },

    }

Tauri.toml

    [build]

    dev-url = "https://:3000"

    # start the dev server

    before-dev-command = "npm run dev"

    [bundle]

    active = true

    icon = ["icons/app.png"]

    [[app.windows]]

    title = "MyApp"

    [plugins.updater]

    pubkey = "updater pub key"

    endpoints = ["https://my.app.updater/{{target}}/{{current_version}}"]

请注意，JSON5 和 TOML 支持注释，TOML 可以为配置名称使用更符合惯用的 kebab-case。在所有 3 种格式中，字段名称均区分大小写。

### 平台特定配置

标题为“平台特定配置”的章节

除了默认的配置文件外，Tauri 还可以从以下位置读取平台特定的配置：

  * Linux 使用 `tauri.linux.conf.json` 或 `Tauri.linux.toml`
  * Windows 使用 `tauri.windows.conf.json` 或 `Tauri.windows.toml`
  * macOS 使用 `tauri.macos.conf.json` 或 `Tauri.macos.toml`
  * Android 使用 `tauri.android.conf.json` 或 `Tauri.android.toml`
  * iOS 使用 `tauri.ios.conf.json` 或 `Tauri.ios.toml`

平台特定配置文件会按照 [JSON Merge Patch (RFC 7396)](https://datatracker.ietf.org/doc/html/rfc7396) 规范与主配置对象合并。

例如，给定以下基础 `tauri.conf.json`

tauri.conf.json

    {

      "productName": "MyApp",

      "bundle": {

        "resources": ["./resources"]

      },

      "plugins": {

        "deep-link": {}

      }

    }

以及给定的 `tauri.linux.conf.json`

tauri.linux.conf.json

    {

      "productName": "my-app",

      "bundle": {

        "resources": ["./linux-assets"]

      },

      "plugins": {

        "cli": {

          "description": "My app",

          "subcommands": {

            "update": {}

          }

        },

        "deep-link": {}

      }

    }

最终解析出的 Linux 配置将是以下对象

    {

      "productName": "my-app",

      "bundle": {

        "resources": ["./linux-assets"]

      },

      "plugins": {

        "cli": {

          "description": "My app",

          "subcommands": {

            "update": {}

          }

        },

        "deep-link": {}

      }

    }

此外，您可以通过 CLI 提供合并配置，有关更多信息，请参阅下一节。

### 扩展配置

标题为“扩展配置”的章节

Tauri CLI 允许您在运行 `dev`、`android dev`、`ios dev`、`build`、`android build`、`ios build` 或 `bundle` 命令之一时扩展 Tauri 配置。配置扩展可以通过 `--config` 参数提供，可以是原始 JSON 字符串，也可以是 JSON 文件的路径。Tauri 使用 [JSON Merge Patch (RFC 7396)](https://datatracker.ietf.org/doc/html/rfc7396) 规范将提供的配置值与最初解析的配置对象合并。

此机制可用于定义应用程序的多个变体，或者在配置应用程序包时获得更大的灵活性。

例如，要分发一个完全隔离的 _beta_ 应用程序，您可以使用此功能来配置单独的应用程序名称和标识符

src-tauri/tauri.beta.conf.json

    {

      "productName": "My App Beta",

      "identifier": "com.myorg.myappbeta"

    }

要分发此单独的 _beta_ 应用程序，您可以在构建时提供此配置文件

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri build -- --config src-tauri/tauri.beta.conf.json

    yarn tauri build --config src-tauri/tauri.beta.conf.json

    pnpm tauri build --config src-tauri/tauri.beta.conf.json

    deno task tauri build --config src-tauri/tauri.beta.conf.json

    bun tauri build --config src-tauri/tauri.beta.conf.json

    cargo tauri build --config src-tauri/tauri.beta.conf.json

## `Cargo.toml`

标题为“Cargo.toml”的章节

Cargo 的清单文件用于声明您的应用所依赖的 Rust crate、有关您应用的元数据以及其他与 Rust 相关的功能。如果您不打算使用 Rust 为您的应用进行后端开发，那么您可能不会过多地修改它，但了解它的存在及其作用非常重要。

以下是 Tauri 项目的精简 `Cargo.toml` 文件示例

Cargo.toml

    [package]

    name = "app"

    version = "0.1.0"

    description = "A Tauri App"

    authors = ["you"]

    license = ""

    repository = ""

    default-run = "app"

    edition = "2021"

    rust-version = "1.57"

    [build-dependencies]

    tauri-build = { version = "2.0.0" }

    [dependencies]

    serde_json = "1.0"

    serde = { version = "1.0", features = ["derive"] }

    tauri = { version = "2.0.0", features = [ ] }

最需要注意的部分是 `tauri-build` 和 `tauri` 依赖项。通常，它们都必须与 Tauri CLI 保持相同的最新次版本号，但这不是严格要求的。如果您在尝试运行应用程序时遇到问题，您应该检查所有 Tauri 版本（`tauri` 和 `tauri-cli`）是否都处于其各自次版本的最新版本。

Cargo 版本号使用 [语义化版本控制 (Semantic Versioning)](https://semver.org)。在 `src-tauri` 文件夹中运行 `cargo update` 将获取所有依赖项的最新可用符合 Semver 标准的版本。例如，如果您为 `tauri-build` 指定版本为 `2.0.0`，Cargo 将检测并下载版本 `2.0.0`，因为它是可用的最新符合 Semver 标准的版本。Tauri 将在引入重大更改时更新主版本号，这意味着您应该始终能够安全地升级到最新的次版本和补丁版本，而无需担心代码损坏。

如果您想使用特定的 crate 版本，可以通过在依赖项的版本号前加上 `=` 来使用确切版本

    tauri-build = { version = "=2.0.0" }

另一个需要注意的地方是 `tauri` 依赖项中的 `features=[]` 部分。运行 `tauri dev` 和 `tauri build` 将根据您的 Tauri 配置自动管理项目中需要启用哪些功能。有关 `tauri` 功能标志的更多信息，请参阅 [文档](https://docs.rs/tauri/2.0.0/tauri/#cargo-features)。

当您构建应用程序时，会生成一个 `Cargo.lock` 文件。该文件主要用于确保在开发过程中跨机器使用相同的依赖项（类似于 Node.js 中的 `yarn.lock`、`pnpm-lock.yaml` 或 `package-lock.json`）。建议将此文件提交到您的源代码仓库，以便获得一致的构建。

要了解有关 Cargo 清单文件的更多信息，请参阅 [官方文档](https://doc.rust-lang.net.cn/cargo/reference/manifest.html)。

## `package.json`

标题为“package.json”的章节

这是 Node.js 使用的包文件。如果您的 Tauri 应用的前端是使用基于 Node.js 的技术（如 `npm`、`yarn` 或 `pnpm`）开发的，则此文件用于配置前端依赖项和脚本。

Tauri 项目的精简 `package.json` 文件示例可能如下所示

package.json

    {

      "scripts": {

        "dev": "command to start your app development mode",

        "build": "command to build your app frontend",

        "tauri": "tauri"

      },

      "dependencies": {

        "@tauri-apps/api": "^2.0.0.0",

        "@tauri-apps/cli": "^2.0.0.0"

      }

    }

通常使用 `"scripts"` 部分来存储用于启动和构建 Tauri 应用程序所使用的前端的命令。上述 `package.json` 文件指定了 `dev` 命令（您可以使用 `yarn dev` 或 `npm run dev` 运行）来启动前端框架，以及 `build` 命令（您可以使用 `yarn build` 或 `npm run build` 运行）来构建您的前端 Web 资源，供 Tauri 在生产环境中使用。使用这些脚本最方便的方法是通过 Tauri 配置的 [beforeDevCommand](/reference/config/#beforedevcommand-1) 和 [beforeBuildCommand](/reference/config/#beforebuildcommand) 钩子将它们与 Tauri CLI 挂钩。

tauri.conf.json

    {

      "build": {

        "beforeDevCommand": "yarn dev",

        "beforeBuildCommand": "yarn build"

      }

    }

注意

`"tauri"` 脚本仅在使用 `npm` 时需要

dependencies 对象指定了当您运行 `yarn`、`pnpm install` 或 `npm install` 时 Node.js 应该下载哪些依赖项（在本例中为 Tauri CLI 和 API）。

除了 `package.json` 文件外，您可能还会看到 `yarn.lock`、`pnpm-lock.yaml` 或 `package-lock.json` 文件。这些文件有助于确保以后下载依赖项时，能够获得与开发期间使用的完全相同的版本（类似于 Rust 中的 `Cargo.lock`）。

要了解有关 `package.json` 文件格式的更多信息，请参阅 [官方文档](https://docs.npmjs.net.cn/cli/v8/configuring-npm/package-json)。