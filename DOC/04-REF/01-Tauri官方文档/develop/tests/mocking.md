# 模拟 Tauri API

_Source: https://v2.tauri.org.cn/develop/tests/mocking/_

在编写前端测试时，使用一个“伪造”的 Tauri 环境来模拟窗口或拦截 IPC 调用（即所谓的 _模拟/mocking_ ）是很常见的。 [`@tauri-apps/api/mocks`](/reference/javascript/api/namespacemocks/) 模块提供了一些实用的工具来简化这一过程。

注意

请记住在每次测试运行后清理模拟，以撤销运行之间的模拟状态更改！更多信息，请参阅 [`clearMocks()`](/reference/javascript/api/namespacemocks/#clearmocks) 文档。

## IPC 请求

名为“IPC 请求”的章节

最常见的情况是拦截 IPC 请求；这在多种场景下都很有帮助：

  * 确保调用了正确的后端函数
  * 模拟后端函数的不同结果

Tauri 提供了 mockIPC 函数来拦截 IPC 请求。你可以在[这里](/reference/javascript/api/namespacemocks/#mockipc)找到关于该特定 API 的详细信息。

注意

以下示例使用了 [Vitest](https://vitest.vite.org.cn)，但你也可以使用任何其他前端测试库，如 Jest。

### 模拟 `invoke` 命令

名为“模拟 invoke 命令”的章节

    import { beforeAll, expect, test } from "vitest";

    import { randomFillSync } from "crypto";

    import { mockIPC } from "@tauri-apps/api/mocks";

    import { invoke } from "@tauri-apps/api/core";

    // jsdom doesn't come with a WebCrypto implementation

    beforeAll(() => {

      Object.defineProperty(window, 'crypto', {

        value: {

          // @ts-ignore

          getRandomValues: (buffer) => {

            return randomFillSync(buffer);

          },

        },

      });

    });

    test("invoke simple", async () => {

      mockIPC((cmd, args) => {

        // simulated rust command called "add" that just adds two numbers

        if(cmd === "add") {

          return (args.a as number) + (args.b as number);

        }

      });

    });

有时你想要跟踪有关 IPC 调用的更多信息；例如命令被调用了多少次？它是否被调用过？你可以结合其他侦听和模拟工具使用 [`mockIPC()`](/reference/javascript/api/namespacemocks/#mockipc) 来测试这些情况。

    import { beforeAll, expect, test, vi } from "vitest";

    import { randomFillSync } from "crypto";

    import { mockIPC } from "@tauri-apps/api/mocks";

    import { invoke } from "@tauri-apps/api/core";

    // jsdom doesn't come with a WebCrypto implementation

    beforeAll(() => {

      Object.defineProperty(window, 'crypto', {

        value: {

          // @ts-ignore

          getRandomValues: (buffer) => {

            return randomFillSync(buffer);

          },

        },

      });

    });

    test("invoke", async () => {

      mockIPC((cmd, args) => {

        // simulated rust command called "add" that just adds two numbers

        if(cmd === "add") {

          return (args.a as number) + (args.b as number);

        }

      });

      // we can use the spying tools provided by vitest to track the mocked function

      const spy = vi.spyOn(window.__TAURI_INTERNALS__, "invoke");

      expect(invoke("add", { a: 12, b: 15 })).resolves.toBe(27);

      expect(spy).toHaveBeenCalled();

    });

要模拟针对 Sidecar 或 shell 命令的 IPC 请求，你需要获取当 `spawn()` 或 `execute()` 被调用时事件处理程序的 ID，并使用该 ID 来触发后端原本会发送回来的事件。

    mockIPC(async (cmd, args) => {

      if (args.message.cmd === 'execute') {

        const eventCallbackId = `_${args.message.onEventFn}`;

        const eventEmitter = window[eventCallbackId];

        // 'Stdout' event can be called multiple times

        eventEmitter({

          event: 'Stdout',

          payload: 'some data sent from the process',

        });

        // 'Terminated' event must be called at the end to resolve the promise

        eventEmitter({

          event: 'Terminated',

          payload: {

            code: 0,

            signal: 'kill',

          },

        });

      }

    });

### 模拟事件

名为“模拟事件”的章节

[ 自 `2.7.0` 起支持](https://v2.tauri.org.cn/release/tauri/v2.7.0)

通过 `shouldMockEvents` 选项，可以部分支持 [事件系统](/develop/calling-frontend/#event-system)，以模拟 Rust 代码发出的事件。

    import { mockIPC, clearMocks } from '@tauri-apps/api/mocks';

    import { emit, listen } from '@tauri-apps/api/event';

    import { afterEach, expect, test, vi } from 'vitest';

    test('mocked event', () => {

      mockIPC(() => {}, { shouldMockEvents: true }); // enable event mocking

      const eventHandler = vi.fn();

      listen('test-event', eventHandler);

      emit('test-event', { foo: 'bar' });

      expect(eventHandler).toHaveBeenCalledWith({

        event: 'test-event',

        payload: { foo: 'bar' },

      });

    });

`emitTo` 和 `emit_filter` 尚**不** 支持。

## Windows

名为“窗口”的章节

有时你会编写特定于窗口的代码（例如启动画面窗口），因此你需要模拟不同的窗口。你可以使用 [`mockWindows()`](/reference/javascript/api/namespacemocks/#mockwindows) 方法来创建伪造的窗口标签。第一个字符串标识“当前”窗口（即 JavaScript 认为自己所在的窗口），所有其他字符串都被视为附加窗口。

注意

[`mockWindows()`](/reference/javascript/api/namespacemocks/#mockwindows) 仅伪造了窗口的存在，而不包含任何窗口属性。要模拟窗口属性，你需要使用 [`mockIPC()`](/reference/javascript/api/namespacemocks/#mockipc) 拦截正确的调用。

    import { beforeAll, expect, test } from 'vitest';

    import { randomFillSync } from 'crypto';

    import { mockWindows } from '@tauri-apps/api/mocks';

    // jsdom doesn't come with a WebCrypto implementation

    beforeAll(() => {

      Object.defineProperty(window, 'crypto', {

        value: {

          // @ts-ignore

          getRandomValues: (buffer) => {

            return randomFillSync(buffer);

          },

        },

      });

    });

    test('invoke', async () => {

      mockWindows('main', 'second', 'third');

      const { getCurrent, getAll } = await import('@tauri-apps/api/webviewWindow');

      expect(getCurrent()).toHaveProperty('label', 'main');

      expect(getAll().map((w) => w.label)).toEqual(['main', 'second', 'third']);

    });