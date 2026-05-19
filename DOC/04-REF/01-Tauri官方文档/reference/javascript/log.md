# @tauri-apps/plugin-log

_Source: https://v2.tauri.org.cn/reference/javascript/log/_

## 枚举

标题为“枚举”的章节

### 日志等级 (LogLevel)

名为“LogLevel”的章节

#### 枚举成员

标题为“枚举成员”的章节

##### Debug

名为“Debug”的章节

    Debug: 2;

“debug”级别。

指定较低优先级的信息。

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/log/guest-js/index.ts#L26>

##### 错误 (Error)

名为“Error”的章节

    Error: 5;

“error”级别。

指定非常严重的错误。

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/log/guest-js/index.ts#L44>

##### 信息 (Info)

名为“Info”的章节

    Info: 3;

“info”级别。

指定有用的信息。

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/log/guest-js/index.ts#L32>

##### 追踪 (Trace)

名为“Trace”的章节

    Trace: 1;

“trace”级别。

指定极低优先级的信息，通常极为冗长。

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/log/guest-js/index.ts#L20>

##### 警告 (Warn)

名为“Warn”的章节

    Warn: 4;

“warn”级别。

指定危险情况。

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/log/guest-js/index.ts#L38>

## 接口

名为“接口”的部分

### 日志选项 (LogOptions)

名为“LogOptions”的章节

#### 属性

名为“属性”的部分

属性| 类型| 定义于
---|---|---
`file?`| `string`| **源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/log/guest-js/index.ts#L9>
`keyValues?`| [`Record`](https://typescript.net.cn/docs/handbook/utility-types.html#recordkeys-type)<`string`, `undefined` | `string`>| **源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/log/guest-js/index.ts#L11>
`line?`| `数字`| **源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/log/guest-js/index.ts#L10>

## 函数

名为“函数”的部分

### attachConsole()

名为“attachConsole()”的章节

    function attachConsole(): Promise<UnlistenFn>

附加一个监听器，在日志条目进入时将其写入控制台。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`UnlistenFn`>

一个用于取消监听器的函数。

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/log/guest-js/index.ts#L277>

* * *

### attachLogger()

名为“attachLogger()”的章节

    function attachLogger(fn): Promise<UnlistenFn>

为日志附加一个监听器，并对每个日志条目调用传入的函数。

#### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`fn`| `LoggerFn`|

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`UnlistenFn`>

一个用于取消监听器的函数。

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/log/guest-js/index.ts#L256>

* * *

### debug()

名为“debug()”的章节

    function debug(message, options?): Promise<void>

以调试 (debug) 级别记录一条消息。

#### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`消息`| `string`| # 示例 `import { debug } from '@tauri-apps/plugin-log'; const pos = { x: 3.234, y: -1.223 }; debug(`New position: x: {pos.x}, y: {pos.y}`);`
`选项`?| [`日志选项 (LogOptions)`](/reference/javascript/log/#logoptions)| -

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/log/guest-js/index.ts#L214>

* * *

### error()

名为“error()”的章节

    function error(message, options?): Promise<void>

以错误 (error) 级别记录一条消息。

#### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`消息`| `string`| # 示例 `import { error } from '@tauri-apps/plugin-log'; const err_info = "No connection"; const port = 22; error(`Error: ${err_info} on port ${port}`);`
`选项`?| [`日志选项 (LogOptions)`](/reference/javascript/log/#logoptions)| -

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/log/guest-js/index.ts#L148>

* * *

### info()

名为“info()”的章节

    function info(message, options?): Promise<void>

以信息 (info) 级别记录一条消息。

#### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`消息`| `string`| # 示例 `import { info } from '@tauri-apps/plugin-log'; const conn_info = { port: 40, speed: 3.20 }; info(`Connected to port {conn_info.port} at {conn_info.speed} Mb/s`);`
`选项`?| [`日志选项 (LogOptions)`](/reference/javascript/log/#logoptions)| -

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/log/guest-js/index.ts#L192>

* * *

### trace()

名为“trace()”的章节

    function trace(message, options?): Promise<void>

以追踪 (trace) 级别记录一条消息。

#### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`消息`| `string`| # 示例 `import { trace } from '@tauri-apps/plugin-log'; let pos = { x: 3.234, y: -1.223 }; trace(`Position is: x: {pos.x}, y: {pos.y}`);`
`选项`?| [`日志选项 (LogOptions)`](/reference/javascript/log/#logoptions)| -

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/log/guest-js/index.ts#L236>

* * *

### warn()

名为“warn()”的章节

    function warn(message, options?): Promise<void>

以警告 (warn) 级别记录一条消息。

#### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`消息`| `string`| # 示例 `import { warn } from '@tauri-apps/plugin-log'; const warn_description = "Invalid Input"; warn(`Warning! {warn_description}!`);`
`选项`?| [`日志选项 (LogOptions)`](/reference/javascript/log/#logoptions)| -

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/log/guest-js/index.ts#L170>