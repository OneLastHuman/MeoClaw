# 创建项目

_Source: https://v2.tauri.org.cn/start/create-project/_

Tauri 的灵活性在于它几乎可以与任何前端框架协同工作。我们创建了 [`create-tauri-app`](https://github.com/tauri-apps/create-tauri-app) 工具，帮助你使用官方维护的前端框架模板快速创建新的 Tauri 项目。

`create-tauri-app` 目前包含的模板有：原生（无框架的 HTML、CSS 和 JavaScript）、[Vue.js](https://vuejs.ac.cn)、[Svelte](https://svelte.net.cn)、[React](https://reactjs.ac.cn/)、[SolidJS](https://www.solidjs.com/)、[Angular](https://angular.io/)、[Preact](https://preact.reactjs.ac.cn/)、[Yew](https://yew.rust-lang.net.cn/)、[Leptos](https://github.com/leptos-rs/leptos) 和 [Sycamore](https://sycamore.dev/)。你还可以在 [Awesome Tauri 仓库](https://github.com/tauri-apps/awesome-tauri)中找到或添加社区贡献的模板和框架。

或者，你也可以将 Tauri 添加到现有项目，从而快速将现有的代码库转换为 Tauri 应用。

## 使用 `create-tauri-app`

标题为“使用 create-tauri-app”的章节

若要开始使用 `create-tauri-app`，请在你想要建立项目的文件夹中运行以下命令。如果你不确定该使用哪条命令，我们建议在 Linux 和 macOS 上使用 Bash 命令，在 Windows 上使用 PowerShell 命令。

  * Bash
  * PowerShell
  * Fish
  * npm
  * Yarn
  * pnpm
  * deno
  * bun
  * Cargo

    sh <(curl https://create.tauri.app/sh)

    irm https://create.tauri.app/ps | iex

    sh (curl -sSL https://create.tauri.app/sh | psub)

    npm create tauri-app@latest

    yarn create tauri-app

    pnpm create tauri-app

    deno run -A npm:create-tauri-app

    bun create tauri-app

    cargo install create-tauri-app --locked

    cargo create-tauri-app

按照提示选择项目名称、前端语言、包管理器、前端框架以及（如果适用的话）前端框架的相关选项。

不知道选什么？

我们建议初学者从原生模板（无框架的 HTML、CSS 和 JavaScript）开始。你随时可以在稍后[集成前端框架](/start/frontend/)。

  * 选择前端使用的语言：`TypeScript / JavaScript`
  * 选择您的包管理器：`pnpm`
  * 选择您的 UI 模板：`Vanilla`
  * 选择 UI 类型：`TypeScript`

#### 搭建新项目

标题为“搭建新项目”的章节

  1. 选择项目名称和 Bundle Identifier（应用的唯一 ID）

         ? Project name (tauri-app) ›

         ? Identifier (com.tauri-app.app) ›

  2. 选择前端的构建风格。首先是语言

         ? Choose which language to use for your frontend ›

         Rust  (cargo)

         TypeScript / JavaScript  (pnpm, yarn, npm, bun)

         .NET  (dotnet)

  3. 选择包管理器（如果有多个可用）

**TypeScript / JavaScript** 的选项

         ? Choose your package manager ›

         pnpm

         yarn

         npm

         bun

  4. 选择 UI 模板和风格（如果有多个可用）

**Rust** 的选项

         ? Choose your UI template ›

         Vanilla

         Yew

         Leptos

         Sycamore

**TypeScript / JavaScript** 的选项

         ? Choose your UI template ›

         Vanilla

         Vue

         Svelte

         React

         Solid

         Angular

         Preact

         ? Choose your UI flavor ›

         TypeScript

         JavaScript

**.NET** 的选项

         ? Choose your UI template ›

         Blazor  (https://dotnet.microsoft.com/en-us/apps/aspnet/web-apps/blazor/)

完成后，该工具会提示模板已创建，并显示如何使用配置的包管理器运行它。如果检测到系统中缺少依赖项，它会打印一个包列表并提示如何安装它们。

#### 启动开发服务器

标题为“启动开发服务器”的章节

在 `create-tauri-app` 完成后，你可以进入项目文件夹，安装依赖项，然后使用 [Tauri CLI](/reference/cli/) 启动开发服务器。

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    cd tauri-app

    npm install

    npm run tauri dev

    cd tauri-app

    yarn install

    yarn tauri dev

    cd tauri-app

    pnpm install

    pnpm tauri dev

    cd tauri-app

    deno install

    deno task tauri dev

    cd tauri-app

    bun install

    bun tauri dev

    cd tauri-app

    cargo install tauri-cli --version "^2.0.0" --locked

    cargo tauri dev

你现在会看到一个新窗口打开，并运行着你的应用。

**恭喜！** 你已经创建了自己的 Tauri 应用！🚀

## 手动设置 (Tauri CLI)

标题为“手动设置 (Tauri CLI)”的章节

如果你已经有一个前端项目，或者更喜欢自己设置它，你可以使用 Tauri CLI 为你的项目单独初始化后端。

注意

以下示例假设你正在创建一个新项目。如果你已经初始化了应用的前端，可以跳过第一步。

  1. 为项目创建一个新目录并初始化前端。你可以使用纯 HTML、CSS 和 JavaScript，或者你喜欢的任何框架（如 Next.js、Nuxt、Svelte、Yew 或 Leptos）。你只需要一种在浏览器中运行该应用的方法。作为示例，这是构建简单 Vite 应用的方法。

     * npm
     * yarn
     * pnpm
     * deno
     * bun

    mkdir tauri-app

    cd tauri-app

    npm create vite@latest .

    mkdir tauri-app

    cd tauri-app

    yarn create vite .

    mkdir tauri-app

    cd tauri-app

    pnpm create vite .

    mkdir tauri-app

    cd tauri-app

    deno run -A npm:create-vite .

    mkdir tauri-app

    cd tauri-app

    bun create vite

  2. 然后，使用你选择的包管理器安装 Tauri 的 CLI 工具。如果你使用 `cargo` 安装 Tauri CLI，则必须全局安装它。

     * npm
     * yarn
     * pnpm
     * deno
     * bun
     * cargo

    npm install -D @tauri-apps/cli@latest

    yarn add -D @tauri-apps/cli@latest

    pnpm add -D @tauri-apps/cli@latest

    deno add -D npm:@tauri-apps/cli@latest

    bun add -D @tauri-apps/cli@latest

    cargo install tauri-cli --version "^2.0.0" --locked

  3. 确定前端开发服务器的 URL。这是 Tauri 用来加载内容的 URL。例如，如果你使用 Vite，默认 URL 是 `https://:5173`。

  4. 在项目目录中，初始化 Tauri

     * npm
     * yarn
     * pnpm
     * deno
     * bun
     * cargo

    npx tauri init

    yarn tauri init

    pnpm tauri init

    deno task tauri init

    bun tauri init

    cargo tauri init

运行命令后，它会显示一个提示，询问你各种选项。

    ✔ What is your app name? tauri-app

    ✔ What should the window title be? tauri-app

    ✔ Where are your web assets located? ..

    ✔ What is the url of your dev server? https://:5173

    ✔ What is your frontend dev command? pnpm run dev

    ✔ What is your frontend build command? pnpm run build

这将在你的项目中创建一个 `src-tauri` 目录，其中包含必要的 Tauri 配置文件。

  5. 通过运行开发服务器来验证你的 Tauri 应用是否正常工作

     * npm
     * yarn
     * pnpm
     * deno
     * bun
     * cargo

    npx tauri dev

    yarn tauri dev

    pnpm tauri dev

    deno task tauri dev

    bun tauri dev

    cargo tauri dev

此命令将编译 Rust 代码并打开一个显示你的 Web 内容的窗口。

**恭喜！** 你已经使用 Tauri CLI 创建了一个新的 Tauri 项目！🚀

## 下一步

名为“后续步骤”的章节

  * [了解项目布局及其文件功能](/start/project-structure/)
  * [添加并配置前端框架](/start/frontend/)
  * [Tauri 命令行界面 (CLI) 参考](/reference/cli/)
  * [了解如何开发你的 Tauri 应用](/develop/)
  * [发现扩展 Tauri 的其他功能](/plugin/)