# WebdriverIO

_Source: https://v2.tauri.org.cn/develop/tests/webdriver/example/webdriverio/_

注意

请确保在按照本指南操作前，已阅读[先决条件说明](/develop/tests/webdriver/)。

此 WebDriver 测试示例将使用 [WebdriverIO](https://webdriverio.node.org.cn/) 及其测试套件。我们假设您已经安装了 Node.js 以及 `npm` 或 `yarn`，尽管[已完成的示例项目](https://github.com/tauri-apps/webdriver-example)使用的是 `pnpm`。

## 为测试创建目录

标题为“为测试创建目录”的章节

让我们在项目中创建一个空间来编写这些测试。在本示例项目中，我们将使用嵌套目录，因为后续我们还会介绍其他框架，但通常您只需要使用一个即可。使用 `mkdir e2e-tests` 创建我们将要使用的目录。本指南的其余部分假设您位于 `e2e-tests` 目录内。

## 初始化 WebdriverIO 项目

标题为“初始化 WebdriverIO 项目”的章节

我们将使用现有的 `package.json` 来引导此测试套件，因为我们已经选择了特定的 [WebdriverIO](https://webdriverio.node.org.cn/) 配置选项，并希望展示一个简单的可行方案。本节底部折叠处包含从零开始设置的指南。

`package.json`:

    {

      "name": "webdriverio",

      "version": "1.0.0",

      "private": true,

      "type": "module",

      "scripts": {

        "test": "wdio run wdio.conf.js"

      },

      "dependencies": {

        "@wdio/cli": "^9.19.0"

      },

      "devDependencies": {

        "@wdio/local-runner": "^9.19.0,

        "@wdio/mocha-framework": "^9.19.0",

        "@wdio/spec-reporter": "^9.19.0"

      }

    }

我们有一个脚本，它以 `test` 命令的形式运行 [WebdriverIO](https://webdriverio.node.org.cn/) 配置作为测试套件。我们还在最初设置时通过 `@wdio/cli` 命令添加了各种依赖项。简而言之，这些依赖项用于最简单的设置，包括使用本地 WebDriver 运行器、[Mocha](https://mocha.node.org.cn/) 作为测试框架，以及简单的 Spec Reporter（规格报告器）。

点击此处查看如何从零开始设置项目

CLI 是交互式的，您可以自行选择要使用的工具。请注意，您的设置可能会偏离本指南的其余部分，因此您需要自行处理差异。

让我们将 [WebdriverIO](https://webdriverio.node.org.cn/) CLI 添加到此 npm 项目中。

  * npm
  * yarn

    npm install @wdio/cli

    yarn add @wdio/cli

要运行交互式配置命令以设置 [WebdriverIO](https://webdriverio.node.org.cn/) 测试套件，您可以运行以下命令：

  * npm
  * yarn

    npx wdio config

    yarn wdio config

## Config

标题为“配置”的章节

您可能已经注意到，我们 `package.json` 中的 `test` 脚本提到了一个文件 `wdio.conf.js`。这就是控制我们测试套件大部分方面的 [WebdriverIO](https://webdriverio.node.org.cn/) 配置文件。

`wdio.conf.js`:

    import os from 'os';

    import path from 'path';

    import { spawn, spawnSync } from 'child_process';

    import { fileURLToPath } from 'url';

    const __dirname = fileURLToPath(new URL('.', import.meta.url));

    // keep track of the `tauri-driver` child process

    let tauriDriver;

    let exit = false;

    export const config = {

      host: '127.0.0.1',

      port: 4444,

      specs: ['./develop/tests/specs/**/*.js'],

      maxInstances: 1,

      capabilities: [

        {

          maxInstances: 1,

          'tauri:options': {

            application: '../src-tauri/target/debug/tauri-app',

          },

        },

      ],

      reporters: ['spec'],

      framework: 'mocha',

      mochaOpts: {

        ui: 'bdd',

        timeout: 60000,

      },

      // ensure the rust project is built since we expect this binary to exist for the webdriver sessions

      onPrepare: () => {

        // Remove the extra `--` if you're not using npm!

        spawnSync(

          'npm',

          ['run', 'tauri', 'build', '--', '--debug', '--no-bundle'],

          {

            cwd: path.resolve(__dirname, '..'),

            stdio: 'inherit',

            shell: true,

          }

        );

      },

      // ensure we are running `tauri-driver` before the session starts so that we can proxy the webdriver requests

      beforeSession: () => {

        tauriDriver = spawn(

          path.resolve(os.homedir(), '.cargo', 'bin', 'tauri-driver'),

          [],

          { stdio: [null, process.stdout, process.stderr] }

        );

        tauriDriver.on('error', (error) => {

          console.error('tauri-driver error:', error);

          process.exit(1);

        });

        tauriDriver.on('exit', (code) => {

          if (!exit) {

            console.error('tauri-driver exited with code:', code);

            process.exit(1);

          }

        });

      },

      // clean up the `tauri-driver` process we spawned at the start of the session

      // note that afterSession might not run if the session fails to start, so we also run the cleanup on shutdown

      afterSession: () => {

        closeTauriDriver();

      },

    };

    function closeTauriDriver() {

      exit = true;

      tauriDriver?.kill();

    }

    function onShutdown(fn) {

      const cleanup = () => {

        try {

          fn();

        } finally {

          process.exit();

        }

      };

      process.on('exit', cleanup);

      process.on('SIGINT', cleanup);

      process.on('SIGTERM', cleanup);

      process.on('SIGHUP', cleanup);

      process.on('SIGBREAK', cleanup);

    }

    // ensure tauri-driver is closed when our test process exits

    onShutdown(() => {

      closeTauriDriver();

    });

如果您对 `config` 对象上的属性感兴趣，我们[建议阅读相关文档](https://webdriverio.node.org.cn/docs/configurationfile)。对于非 WDIO 特有的项目，我们添加了注释，解释了为什么要运行 `onPrepare`、`beforeSession` 和 `afterSession` 中的命令。我们还将测试规范（specs）设置为 `"./test/specs/**/*.js"`，现在让我们创建一个 spec 文件。

## Spec（测试规格）

标题为“Spec”的章节

Spec 包含测试您实际应用程序的代码。测试运行程序将加载这些 spec 文件并按需自动运行它们。现在，让我们在指定的目录中创建我们的 spec 文件。

`test/specs/example.e2e.js`:

    // calculates the luma from a hex color `#abcdef`

    function luma(hex) {

      if (hex.startsWith('#')) {

        hex = hex.substring(1);

      }

      const rgb = parseInt(hex, 16);

      const r = (rgb >> 16) & 0xff;

      const g = (rgb >> 8) & 0xff;

      const b = (rgb >> 0) & 0xff;

      return 0.2126 * r + 0.7152 * g + 0.0722 * b;

    }

    describe('Hello Tauri', () => {

      it('should be cordial', async () => {

        const header = await $('body > h1');

        const text = await header.getText();

        expect(text).toMatch(/^[hH]ello/);

      });

      it('should be excited', async () => {

        const header = await $('body > h1');

        const text = await header.getText();

        expect(text).toMatch(/!$/);

      });

      it('should be easy on the eyes', async () => {

        const body = await $('body');

        const backgroundColor = await body.getCSSProperty('background-color');

        expect(luma(backgroundColor.parsed.hex)).toBeLessThan(100);

      });

    });

顶部的 `luma` 函数只是我们其中一个测试的辅助函数，与应用程序的实际测试无关。如果您熟悉其他测试框架，可能会注意到类似的函数（如 `describe`、`it` 和 `expect`）的使用。其他 API（例如 `$` 及其暴露的方法）可以在 [WebdriverIO API 文档](https://webdriverio.node.org.cn/docs/api)中找到。

## 运行测试套件

标题为“运行测试套件”的章节

现在我们已经设置好配置和 spec 文件，让我们来运行它！

  * npm
  * yarn

    npm test

    yarn test

我们应该会看到如下输出：

    ➜  webdriverio git:(main) ✗ yarn test

    yarn run v1.22.11

    $ wdio run wdio.conf.js

    Execution of 1 workers started at 2021-08-17T08:06:10.279Z

    [0-0] RUNNING in undefined - /develop/tests/specs/example.e2e.js

    [0-0] PASSED in undefined - /develop/tests/specs/example.e2e.js

     "spec" Reporter:

    ------------------------------------------------------------------

    [wry 0.12.1 linux #0-0] Running: wry (v0.12.1) on linux

    [wry 0.12.1 linux #0-0] Session ID: 81e0107b-4d38-4eed-9b10-ee80ca47bb83

    [wry 0.12.1 linux #0-0]

    [wry 0.12.1 linux #0-0] » /develop/tests/specs/example.e2e.js

    [wry 0.12.1 linux #0-0] Hello Tauri

    [wry 0.12.1 linux #0-0]    ✓ should be cordial

    [wry 0.12.1 linux #0-0]    ✓ should be excited

    [wry 0.12.1 linux #0-0]    ✓ should be easy on the eyes

    [wry 0.12.1 linux #0-0]

    [wry 0.12.1 linux #0-0] 3 passing (244ms)

    Spec Files:   1 passed, 1 total (100% completed) in 00:00:01

    Done in 1.98s.

我们可以看到 Spec Reporter 告诉我们 `test/specs/example.e2e.js` 文件中的所有 3 个测试均已运行，最终报告为 `Spec Files: 1 passed, 1 total (100% completed) in 00:00:01`。

通过使用 [WebdriverIO](https://webdriverio.node.org.cn/) 测试套件，我们仅通过几行配置和一个命令，就轻松为 Tauri 应用程序启用了端到端（e2e）测试！更棒的是，我们完全不需要修改应用程序代码。