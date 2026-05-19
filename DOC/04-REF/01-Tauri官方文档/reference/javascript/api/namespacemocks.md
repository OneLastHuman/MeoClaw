# mocks

_Source: https://v2.tauri.org.cn/reference/javascript/api/namespacemocks/_

## 接口

名为“接口”的部分

### MockIPCOptions

MockIPCOptions 章节

`mockIPC` 的选项。

# 选项

选项章节

`shouldMockEvents`：如果设为 true，`listen` 和 `emit` 函数将被模拟，允许你在没有真实后端的情况下测试事件处理。**这将消耗所有以`plugin:event` 为前缀发出的事件。**

#### 始于

名为“起始版本”的部分

2.7.0

#### 属性

名为“属性”的部分

属性| 类型| 定义于
---|---|---
`shouldMockEvents?`| `布尔值 (boolean)`| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/mocks.ts#L24>

## 函数

名为“函数”的部分

### clearMocks()

clearMocks() 章节

    function clearMocks(): void

清除由本模块中其他函数注入的模拟函数/数据。当使用不为每个测试提供全新 window 对象的测试运行器时，调用此函数将重置 Tauri 特定的属性。

# 示例

标题为“Example”的章节

    import { mockWindows, clearMocks } from "@tauri-apps/api/mocks"

    afterEach(() => {

       clearMocks()

    })

    test("mocked windows", () => {

       mockWindows("main", "second", "third");

       expect(window.__TAURI_INTERNALS__).toHaveProperty("metadata")

    })

    test("no mocked windows", () => {

       expect(window.__TAURI_INTERNALS__).not.toHaveProperty("metadata")

    })

#### 返回

名为“返回值”的部分

`空`

#### 始于

名为“起始版本”的部分

1.0.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/mocks.ts#L316>

* * *

### mockConvertFileSrc()

mockConvertFileSrc() 章节

    function mockConvertFileSrc(osName): void

模拟 `convertFileSrc` 函数

#### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`osName`| `string`| 要模拟的操作系统，可以是 linux、macos 或 windows

#### 返回

名为“返回值”的部分

`空`

#### 示例

标题为“Example”的章节

    import { mockConvertFileSrc } from "@tauri-apps/api/mocks";

    import { convertFileSrc } from "@tauri-apps/api/core";

    mockConvertFileSrc("windows")

    const url = convertFileSrc("C:\\Users\\user\\file.txt")

#### 始于

名为“起始版本”的部分

1.6.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/mocks.ts#L277>

* * *

### mockIPC()

mockIPC() 章节

    function mockIPC(cb, options?): void

使用给定的模拟处理器拦截所有 IPC 请求。

此函数可用于测试 Tauri 前端应用，或在静态站点生成期间在 Node.js 环境中运行前端时使用。

# 示例

名为“示例”的章节

使用 Vitest 进行测试配置

    import { mockIPC, clearMocks } from "@tauri-apps/api/mocks"

    import { invoke } from "@tauri-apps/api/core"

    afterEach(() => {

       clearMocks()

    })

    test("mocked command", () => {

     mockIPC((cmd, payload) => {

      switch (cmd) {

        case "add":

          return (payload.a as number) + (payload.b as number);

        default:

          break;

        }

     });

     expect(invoke('add', { a: 12, b: 15 })).resolves.toBe(27);

    })

回调函数也可以返回一个 Promise

    import { mockIPC, clearMocks } from "@tauri-apps/api/mocks"

    import { invoke } from "@tauri-apps/api/core"

    afterEach(() => {

       clearMocks()

    })

    test("mocked command", () => {

     mockIPC((cmd, payload) => {

      if(cmd === "get_data") {

       return fetch("https://example.com/data.json")

         .then((response) => response.json())

      }

     });

     expect(invoke('get_data')).resolves.toBe({ foo: 'bar' });

    })

`listen` 也可以通过直接调用 `emit` 函数来模拟。此功能通过 `shouldMockEvents` 选项选择开启。

    import { mockIPC, clearMocks } from "@tauri-apps/api/mocks"

    import { emit, listen } from "@tauri-apps/api/event"

    afterEach(() => {

       clearMocks()

    })

    test("mocked event", () => {

     mockIPC(() => {}, { shouldMockEvents: true }); // enable event mocking

     const eventHandler = vi.fn();

     listen('test-event', eventHandler); // typically in component setup or similar

     emit('test-event', { foo: 'bar' });

     expect(eventHandler).toHaveBeenCalledWith({

       event: 'test-event',

       payload: { foo: 'bar' }

     });

    })

此模拟实现目前**不支持** `emitTo`。

#### 参数

名为“参数”的部分

参数| 类型
---|---
`cb`| (`cmd`, `payload`?) => `unknown`
`选项`?| [`MockIPCOptions`](/reference/javascript/api/namespacemocks/#mockipcoptions)

#### 返回

名为“返回值”的部分

`空`

#### 始于

名为“起始版本”的部分

1.0.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/mocks.ts#L104>

* * *

### mockWindows()

mockWindows() 章节

    function mockWindows(current, ..._additionalWindows): void

模拟一个或多个窗口标签。在非 Tauri 环境中，必须在调用 `@tauri-apps/api/window` 模块 _之前_ 调用此函数。

此函数仅模拟窗口的 _存在_ ，窗口属性（如宽度和高度）可以使用 `mockIPC` 函数像普通的 IPC 调用一样进行模拟。

# 示例

名为“示例”的章节

    import { mockWindows } from "@tauri-apps/api/mocks";

    import { getCurrentWindow } from "@tauri-apps/api/window";

    mockWindows("main", "second", "third");

    const win = getCurrentWindow();

    win.label // "main"

    import { mockWindows } from "@tauri-apps/api/mocks";

    mockWindows("main", "second", "third");

    mockIPC((cmd, args) => {

     if (cmd === "plugin:event|emit") {

       console.log('emit event', args?.event, args?.payload);

     }

    });

    const { emit } = await import("@tauri-apps/api/event");

    await emit('loaded'); // this will cause the mocked IPC handler to log to the console.

#### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`current`| `string`| 当前 JavaScript 环境正在运行的窗口标签。
…`_additionalWindows`| `string`[]| -

#### 返回

名为“返回值”的部分

`空`

#### 始于

名为“起始版本”的部分

1.0.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/mocks.ts#L248>