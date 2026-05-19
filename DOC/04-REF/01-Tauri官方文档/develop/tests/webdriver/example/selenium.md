# Selenium

_Source: https://v2.tauri.org.cn/develop/tests/webdriver/example/selenium/_

注意

在开始本指南之前，请确保先完成[先决条件说明](/develop/tests/webdriver/)。

此 WebDriver 测试示例将使用 [Selenium](https://selenium.net.cn/) 和一个流行的 Node.js 测试套件。你需要预先安装 Node.js 以及 `npm` 或 `yarn`，尽管[完整的示例项目](https://github.com/tauri-apps/webdriver-example)使用的是 `pnpm`。

## 创建测试目录

章节标题：“创建测试目录”

让我们在项目中创建一个空间来编写这些测试。对于这个示例项目，我们将使用一个嵌套目录，因为稍后我们还将介绍其他框架，但通常你只需要使用一个。使用 `mkdir -p e2e-tests` 创建我们将要使用的目录。本指南的其余部分将假设你位于 `e2e-tests` 目录内。

## 初始化 Selenium 项目

章节标题：“初始化 Selenium 项目”

我们将使用预先存在的 `package.json` 来引导此测试套件，因为我们已经选择了特定的依赖项，并希望展示一个简单可行的解决方案。本节底部有一个折叠的指南，介绍了如何从零开始设置它。

`package.json`:

    {

      "name": "selenium",

      "version": "1.0.0",

      "private": true,

      "type": "module",

      "scripts": {

        "test": "mocha"

      },

      "dependencies": {

        "chai": "^5.2.1",

        "mocha": "^11.7.1",

        "selenium-webdriver": "^4.34.0"

      }

    }

我们有一个脚本，将 [Mocha](https://mocha.node.org.cn/) 作为测试框架暴露为 `test` 命令。我们还拥有将用于运行测试的各种依赖项。[Mocha](https://mocha.node.org.cn/) 作为测试框架，[Chai](https://chai.node.org.cn/) 作为断言库，以及 [`selenium-webdriver`](https://npmjs.net.cn/package/selenium-webdriver)，即 Node.js 的 [Selenium](https://selenium.net.cn/) 包。

点击我查看如何从头开始设置项目

如果你想从零开始安装依赖项，只需运行以下命令。

  * npm
  * yarn

    npm install mocha chai selenium-webdriver

    yarn add mocha chai selenium-webdriver

我还建议在 `package.json` 的 `"scripts"` 键中添加一个 `"test": "mocha"` 项，这样运行 Mocha 就可以简单地通过以下方式调用

  * npm
  * yarn

    npm test

    yarn test

## 测试

名为“测试”的章节

与 [WebdriverIO 测试套件](/develop/tests/webdriver/example/webdriverio/#config)不同，Selenium 没有开箱即用的测试套件，而是让开发者自行构建。我们选择了 [Mocha](https://mocha.node.org.cn/)，它非常中立，与 WebDriver 无关，因此我们的脚本需要做一些额外工作来按正确顺序为我们设置所有内容。[Mocha](https://mocha.node.org.cn/) 默认期望在 `test/test.js` 中有一个测试文件，所以让我们现在创建该文件。

`test/test.js`:

    import os from 'os';

    import path from 'path';

    import { expect } from 'chai';

    import { spawn, spawnSync } from 'child_process';

    import { Builder, By, Capabilities } from 'selenium-webdriver';

    import { fileURLToPath } from 'url';

    const __dirname = fileURLToPath(new URL('.', import.meta.url));

    // create the path to the expected application binary

    const application = path.resolve(

      __dirname,

      '..',

      '..',

      'src-tauri',

      'target',

      'debug',

      'tauri-app'

    );

    // keep track of the webdriver instance we create

    let driver;

    // keep track of the tauri-driver process we start

    let tauriDriver;

    let exit = false;

    before(async function () {

      // set timeout to 2 minutes to allow the program to build if it needs to

      this.timeout(120000);

      // ensure the app has been built

      spawnSync('yarn', ['tauri', 'build', '--debug', '--no-bundle'], {

        cwd: path.resolve(__dirname, '../..'),

        stdio: 'inherit',

        shell: true,

      });

      // start tauri-driver

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

      const capabilities = new Capabilities();

      capabilities.set('tauri:options', { application });

      capabilities.setBrowserName('wry');

      // start the webdriver client

      driver = await new Builder()

        .withCapabilities(capabilities)

        .usingServer('http://127.0.0.1:4444/')

        .build();

    });

    after(async function () {

      // stop the webdriver session

      await closeTauriDriver();

    });

    describe('Hello Tauri', () => {

      it('should be cordial', async () => {

        const text = await driver.findElement(By.css('body > h1')).getText();

        expect(text).to.match(/^[hH]ello/);

      });

      it('should be excited', async () => {

        const text = await driver.findElement(By.css('body > h1')).getText();

        expect(text).to.match(/!$/);

      });

      it('should be easy on the eyes', async () => {

        // selenium returns color css values as rgb(r, g, b)

        const text = await driver

          .findElement(By.css('body'))

          .getCssValue('background-color');

        const rgb = text.match(/^rgb\((?<r>\d+), (?<g>\d+), (?<b>\d+)\)$/).groups;

        expect(rgb).to.have.all.keys('r', 'g', 'b');

        const luma = 0.2126 * rgb.r + 0.7152 * rgb.g + 0.0722 * rgb.b;

        expect(luma).to.be.lessThan(100);

      });

    });

    async function closeTauriDriver() {

      exit = true;

      // kill the tauri-driver process

      tauriDriver.kill();

      // stop the webdriver session

      await driver.quit();

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

    onShutdown(() => {

      closeTauriDriver();

    });

如果你熟悉 JS 测试框架，那么 `describe`、`it` 和 `expect` 看起来应该很熟悉。我们还有稍微复杂的 `before()` 和 `after()` 回调来设置和拆卸 Mocha。非测试代码行有注释解释了设置和拆卸代码。如果你熟悉 [WebdriverIO 示例](/develop/tests/webdriver/example/webdriverio/#spec)中的 Spec 文件，你会注意到这里有更多非测试代码，因为我们必须设置更多与 WebDriver 相关的项目。

## 运行测试套件

章节标题：“运行测试套件”

现在我们已经完成了依赖项和测试脚本的设置，让我们运行它！

  * npm
  * yarn

    npm test

    yarn test

我们应该会看到以下输出

    ➜  selenium git:(main) ✗ yarn test

    yarn run v1.22.11

    $ Mocha

      Hello Tauri

        ✔ should be cordial (120ms)

        ✔ should be excited

        ✔ should be easy on the eyes

      3 passing (588ms)

    Done in 0.93s.

我们可以看到，我们用 `describe` 创建的 `Hello Tauri` 测试套件中，所有用 `it` 创建的 3 个项目都通过了测试！

通过 [Selenium](https://selenium.net.cn/) 并将其连接到测试套件，我们无需修改 Tauri 应用程序本身即可实现端到端 (e2e) 测试！