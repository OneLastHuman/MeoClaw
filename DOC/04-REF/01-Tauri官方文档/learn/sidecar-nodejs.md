# Node.js 作为 Sidecar

_Source: https://v2.tauri.org.cn/learn/sidecar-nodejs/_

在本指南中，我们将把一个 Node.js 应用程序打包成一个独立的二进制文件，作为 Tauri 应用程序的 Sidecar 使用，而无需终端用户安装 Node.js。本教程示例仅适用于桌面操作系统。

我们建议先阅读通用的 [Sidecar 指南](/develop/sidecar/)，以深入了解 Tauri Sidecar 的工作原理。

## 目标

章节标题 “目标”

  * 将 Node.js 应用程序打包为二进制文件。
  * 将此二进制文件作为 Tauri Sidecar 集成。

## 实现细节

章节标题 “实现细节”

  * 为此，我们使用 [pkg](https://github.com/yao-pkg/pkg) 工具，但任何其他能将 JavaScript 或 TypeScript 编译为二进制应用程序的工具也适用。
  * 你也可以将 Node 运行时本身嵌入到 Tauri 应用程序中，并将打包好的 JavaScript 作为资源进行分发，但这会使 JavaScript 内容以可读文件形式存在，且运行时的体积通常比使用 `pkg` 打包的应用程序要大。

在本示例中，我们将创建一个 Node.js 应用程序，它通过命令行 [process.argv](https://node.org.cn/docs/latest/api/process.html#processargv) 读取输入，并使用 [console.log](https://node.org.cn/api/console.html#consolelogdata-args) 将输出写入 stdout。
你可以利用其他进程间通信 (IPC) 系统，例如本地服务器、stdin/stdout 或本地套接字。请注意，每种方案都有其优缺点及安全考虑。

## 先决条件

标题为“前提条件”的章节

一个配置了 shell 插件、且能在本地编译运行的现有 Tauri 应用程序。

创建一个实验应用

如果你不是高级用户，**强烈建议** 使用此处提供的选项和框架。这只是一个实验，完成后你可以删除该项目。

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

  * 项目名称: `node-sidecar-lab`
  * 选择前端语言: `Typescript / Javascript`
  * 选择您的包管理器：`pnpm`
  * 选择您的 UI 模板：`Vanilla`
  * 选择 UI 偏好: `Typescript`

注意

请先按照 [shell 插件指南](/plugin/shell/) 正确设置和初始化插件。如果插件未正确初始化和配置，该示例将无法运行。

## 指南

章节标题 “指南”

  1. ##### 初始化 Sidecar 项目

章节标题 “初始化 Sidecar 项目”

让我们创建一个新的 Node.js 项目来包含我们的 Sidecar 实现。在 **Tauri 应用程序根目录中** 创建一个新目录（在本示例中我们将其命名为 `sidecar-app`），并在该目录内运行你偏好的 Node.js 包管理器的 `init` 命令。

     * npm
     * yarn
     * pnpm

    npm init

    yarn init

    pnpm init

我们将使用 [pkg](https://github.com/yao-pkg/pkg) 等工具将 Node.js 应用程序编译为独立二进制文件。让我们将其作为开发依赖安装到新的 `sidecar-app` 中。

     * npm
     * yarn
     * pnpm

    npm add @yao-pkg/pkg --save-dev

    yarn add @yao-pkg/pkg --dev

    pnpm add @yao-pkg/pkg --save-dev

  2. ##### 编写 Sidecar 逻辑

章节标题 “编写 Sidecar 逻辑”

现在我们可以开始编写将由 Tauri 应用程序执行的 JavaScript 代码了。

在本示例中，我们将处理来自命令行参数的指令，并将输出写入 stdout，这意味着我们的进程寿命较短，每次仅处理一条指令。如果你的应用程序必须长期运行，请考虑使用其他进程间通信系统。

让我们在 `sidecar-app` 目录中创建一个 `index.js` 文件，并编写一个基础的 Node.js 程序：

sidecar-app/index.js

         const command = process.argv[2];

         switch (command) {

           case 'hello':

             const message = process.argv[3];

             console.log(`Hello ${message}!`);

             break;

           default:

             console.error(`unknown command ${command}`);

             process.exit(1);

         }

  3. ##### 打包 Sidecar

章节标题 “打包 Sidecar”

要将我们的 Node.js 应用程序打包成独立的二进制文件，请在 `package.json` 中创建一个脚本。

sidecar-app/package.json

         {

           "scripts": {

             "build": "pkg index.ts --output my-sidecar"

           }

         }

     * npm
     * yarn
     * pnpm

    npm run build

    yarn build

    pnpm build

这将在 Linux 和 macOS 上创建 `sidecar-app/my-sidecar` 二进制文件，在 Windows 上创建 `sidecar-app/my-sidecar.exe` 可执行文件。

对于 Sidecar 应用程序，我们需要确保二进制文件符合特定的命名模式。有关详细信息，请参阅 [嵌入外部二进制文件](https://tauri.org.cn/develop/sidecar/)。为了将此文件重命名为符合 Tauri 预期的 Sidecar 文件名并移动到我们的 Tauri 项目中，我们可以使用以下 Node.js 脚本作为示例：

sidecar-app/rename.js

    import { execSync } from 'child_process';

    import fs from 'fs';

    const ext = process.platform === 'win32' ? '.exe' : '';

    const targetTriple = execSync('rustc --print host-tuple').toString().trim();

    if (!targetTriple) {

      console.error('Failed to determine platform target triple');

    }

    // TODO: create `src-tauri/binaries` dir

    fs.renameSync(

      `my-sidecar${ext}`,

      `../src-tauri/binaries/my-sidecar-${targetTriple}${ext}`

    );

注意

`--print host-tuple` 标志是在 Rust 1.84.0 中添加的。如果你使用的是旧版本，则需要解析 `rustc -Vv` 的输出。

    const rustInfo = execSync('rustc -vV');

    const targetTriple = /host: (\S+)/g.exec(rustInfo)[1];

然后在 `sidecar-app` 目录中运行 `node rename.js`。

在此步骤完成后，`/src-tauri/binaries` 目录中应包含重命名后的 Sidecar 二进制文件。

  4. ##### 设置 plugin-shell 权限

章节标题 “设置 plugin-shell 权限”

安装 [shell 插件](/plugin/shell/) 后，请确保配置了所需的权限 (capabilities)。

注意：我们使用了 `"args": true`，但你也可以提供一个数组 `["hello"]`，[阅读更多](/develop/sidecar/#passing-arguments)。

src-tauri/capabilities/default.json

         {

           "permissions": [

             "core:default",

             "opener:default",

             {

               "identifier": "shell:allow-execute",

               "allow": [

                 {

                   "args": true,

                   "name": "binaries/my-sidecar",

                   "sidecar": true

                 }

               ]

             }

           ]

         }

  5. ##### 在 Tauri 应用程序中配置 Sidecar

章节标题 “在 Tauri 应用程序中配置 Sidecar”

现在 Node.js 应用程序已准备就绪，我们可以通过配置 [`bundle > externalBin`](/reference/config/#externalbin) 数组将其连接到 Tauri 应用程序。

src-tauri/tauri.conf.json

         {

           "bundle": {

             "externalBin": ["binaries/my-sidecar"]

           }

         }

只要 Sidecar 二进制文件以 `src-tauri/binaries/my-sidecar-<target-triple>` 的格式存在，Tauri CLI 就会处理其打包工作。

  6. ##### 执行 Sidecar

章节标题 “执行 Sidecar”

我们可以从 Rust 代码或直接从 JavaScript 运行 Sidecar 二进制文件。

     * JavaScript
     * Rust

让我们直接执行 Node.js Sidecar 中的 `hello` 指令。

    import { Command } from '@tauri-apps/plugin-shell';

    const message = 'Tauri';

    const command = Command.sidecar('binaries/my-sidecar', ['hello', message]);

    const output = await command.execute();

    // once everything is configured it should log "Hello Tauri" in the browser console.

    console.log(output.stdout)

让我们将 `hello` Tauri 指令通过管道传递给 Node.js Sidecar。

    use tauri_plugin_shell::ShellExt;

    #[tauri::command]

    async fn hello(app: tauri::AppHandle, cmd: String, message: String) -> String {

        let sidecar_command = app

            .shell()

            .sidecar("my-sidecar")

            .unwrap()

            .arg(cmd)

            .arg(message);

        let output = sidecar_command.output().await.unwrap();

        String::from_utf8(output.stdout).unwrap()

    }

在 `invoke_handler` 中注册它，并在前端调用它。

    import { invoke } from "@tauri-apps/api/core";

    const message = "Tauri"

    console.log(await invoke("hello", { cmd: 'hello', message }))

  7. ##### 运行

章节标题 “运行”

测试一下。

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

按下 F12（或 macOS 上的 `Cmd+Option+I`）打开开发者工具，你应该能看到 Sidecar 指令的输出。

如果发现任何问题，请在 [GitHub](https://github.com/tauri-apps/tauri-docs) 上提交 issue。