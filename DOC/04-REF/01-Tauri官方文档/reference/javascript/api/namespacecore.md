# core

_Source: https://v2.tauri.org.cn/reference/javascript/api/namespacecore/_

调用您的自定义命令。

当 `tauri.conf.json` 中的 [`app.withGlobalTauri`](https://v2.tauri.org.cn/reference/config/#withglobaltauri) 设置为 `true` 时，也可以通过 `window.__TAURI__.core` 访问此包。

## 类

名为“类”的部分

### Channel<T>

名为“Channel<T>”的部分

#### 类型参数

名为“类型参数”的部分

类型参数| 默认类型
---|---
`T`| `未知`

#### 构造函数

名为“构造函数”的部分

##### new Channel()

名为“new Channel()”的部分

    new Channel<T>(onmessage?): Channel<T>

###### 参数

名为“参数”的部分

参数| 类型
---|---
`onmessage`?| (`response`) => `void`

###### 返回

名为“返回值”的部分

[`Channel`](/reference/javascript/api/namespacecore/#channelt)<`T`>

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/core.ts#L87>

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
`id`| `数字`| 从 [`transformCallback`](/reference/javascript/api/namespacecore/#transformcallback) 返回的 callback id| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/core.ts#L79>

#### 访问器

名为“访问器”的部分

##### onmessage

名为“onmessage”的部分

    get onmessage(): (response) => void

    set onmessage(handler): void

###### 参数

名为“参数”的部分

参数| 类型
---|---
`处理程序`| (`response`) => `void`

###### 返回

名为“返回值”的部分

`函数`

###### 参数

名为“参数”的部分

参数| 类型
---|---
`response`| `T`

###### 返回

名为“返回值”的部分

`空`

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/core.ts#L142>

#### 方法

名为“方法”的部分

##### __TAURI_TO_IPC_KEY__()

名为“__TAURI_TO_IPC_KEY__()”的部分

    __TAURI_TO_IPC_KEY__(): string

###### 返回

名为“返回值”的部分

`string`

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/core.ts#L146>

##### toJSON()

名为“toJSON()”的部分

    toJSON(): string

###### 返回

名为“返回值”的部分

`string`

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/core.ts#L150>

* * *

### PluginListener

名为“PluginListener”的部分

#### 构造函数

名为“构造函数”的部分

##### new PluginListener()

名为“new PluginListener()”的部分

    new PluginListener(

       plugin,

       event,

       channelId): PluginListener

###### 参数

名为“参数”的部分

参数| 类型
---|---
`plugin`| `string`
`event`| `string`
`channelId`| `数字`

###### 返回

名为“返回值”的部分

[`PluginListener`](/reference/javascript/api/namespacecore/#pluginlistener)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/core.ts#L161>

#### 属性

名为“属性”的部分

属性| 类型| 定义于
---|---|---
`channelId`| `数字`| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/core.ts#L159>
`event`| `string`| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/core.ts#L158>
`plugin`| `string`| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/core.ts#L157>

#### 方法

名为“方法”的部分

##### unregister()

名为“unregister()”的部分

    unregister(): Promise<void>

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/core.ts#L167>

* * *

### Resource

名为“Resource”的部分

一个通过 `tauri::Manager::resources_table` API 存储的 Rust 支持的资源。

该资源存在于主进程中，在 Javascript 世界中不存在，因此除非应用程序退出，否则不会自动清理。如果您想提前清理它，请调用 [`Resource.close`](/reference/javascript/api/namespacecore/#close)

#### 示例

标题为“Example”的章节

    import { Resource, invoke } from '@tauri-apps/api/core';

    export class DatabaseHandle extends Resource {

      static async open(path: string): Promise<DatabaseHandle> {

        const rid: number = await invoke('open_db', { path });

        return new DatabaseHandle(rid);

      }

      async execute(sql: string): Promise<void> {

        await invoke('execute_sql', { rid: this.rid, sql });

      }

    }

#### 扩展自

名为“扩展”的部分

  * [`图像`](/reference/javascript/api/namespaceimage/#image)
  * [`TrayIcon`](/reference/javascript/api/namespacetray/#trayicon)

#### 构造函数

名为“构造函数”的部分

##### new Resource()

名为“new Resource()”的部分

    new Resource(rid): Resource

###### 参数

名为“参数”的部分

参数| 类型
---|---
`rid`| `数字`

###### 返回

名为“返回值”的部分

[`Resource`](/reference/javascript/api/namespacecore/#resource)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/core.ts#L322>

#### 访问器

名为“访问器”的部分

##### rid

名为“rid”的部分

    get rid(): number

###### 返回

名为“返回值”的部分

`数字`

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/core.ts#L318>

#### 方法

名为“方法”的部分

##### close()

名为“close()”的部分

    close(): Promise<void>

销毁并从内存中清理此资源。**您不应再对此对象调用任何方法，并应放弃对它的任何引用。**

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/core.ts#L330>

## 接口

名为“接口”的部分

### InvokeOptions

名为“InvokeOptions”的部分

#### 始于

名为“起始版本”的部分

2.0.0

#### 属性

名为“属性”的部分

属性| 类型| 定义于
---|---|---
`headers`| `HeadersInit`| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/core.ts#L233>

## 类型别名 (Type Aliases)

名为“类型别名”的部分

### InvokeArgs

名为“InvokeArgs”的部分

    type InvokeArgs: Record<string, unknown> | number[] | ArrayBuffer | Uint8Array;

命令参数。

#### 始于

名为“起始版本”的部分

1.0.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/core.ts#L227>

* * *

### PermissionState

名为“PermissionState”的部分

    type PermissionState: "granted" | "denied" | "prompt" | "prompt-with-rationale";

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/core.ts#L202>

## Variables

名为“变量”的部分

### SERIALIZE_TO_IPC_FN

名为“SERIALIZE_TO_IPC_FN”的部分

    const SERIALIZE_TO_IPC_FN: "__TAURI_TO_IPC_KEY__" = '__TAURI_TO_IPC_KEY__';

一个键，用于在您的类型上实现特殊函数，定义您的类型在通过 IPC 传递时应如何序列化。

#### 示例

标题为“Example”的章节

给定一个如下所示的 Rust 类型

    #[derive(serde::Serialize, serde::Deserialize)

    enum UserId {

      String(String),

      Number(u32),

    }

`UserId::String("id")` 将被序列化为 `{ String: "id" }`，因此我们需要将相同的结构传回给 Rust

    import { SERIALIZE_TO_IPC_FN } from "@tauri-apps/api/core"

    class UserIdString {

      id

      constructor(id) {

        this.id = id

      }

      [SERIALIZE_TO_IPC_FN]() {

        return { String: this.id }

      }

    }

    class UserIdNumber {

      id

      constructor(id) {

        this.id = id

      }

      [SERIALIZE_TO_IPC_FN]() {

        return { Number: this.id }

      }

    }

    type UserId = UserIdString | UserIdNumber

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/core.ts#L59>

## 函数

名为“函数”的部分

### addPluginListener()

名为“addPluginListener()”的部分

    function addPluginListener<T>(

       plugin,

       event,

    cb): Promise<PluginListener>

添加一个插件事件的监听器。

#### 类型参数

名为“类型参数”的部分

类型参数
---
`T`

#### 参数

名为“参数”的部分

参数| 类型
---|---
`plugin`| `string`
`event`| `string`
`cb`| (`payload`) => `void`

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`PluginListener`](/reference/javascript/api/namespacecore/#pluginlistener)>

用于停止监听事件的监听器对象。

#### 始于

名为“起始版本”的部分

2.0.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/core.ts#L182>

* * *

### checkPermissions()

名为“checkPermissions()”的部分

    function checkPermissions<T>(plugin): Promise<T>

获取插件的权限状态。

插件作者应使用此方法来包装其实际实现。

#### 类型参数

名为“类型参数”的部分

类型参数
---
`T`

#### 参数

名为“参数”的部分

参数| 类型
---|---
`plugin`| `string`

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`T`>

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/core.ts#L209>

* * *

### convertFileSrc()

名为“convertFileSrc()”的部分

    function convertFileSrc(filePath, protocol): string

将设备文件路径转换为可由 webview 加载的 URL。注意，必须将 `asset:` 和 `http://asset.localhost` 添加到 `tauri.conf.json` 中的 [`app.security.csp`](https://v2.tauri.org.cn/reference/config/#csp-1)。CSP 值示例：`"csp": "default-src 'self' ipc: http://ipc.localhost; img-src 'self' asset: http://asset.localhost"` 以在图像源上使用资产协议。

此外，必须将 `"enable" : "true"` 添加到 `tauri.conf.json` 中的 [`app.security.assetProtocol`](https://v2.tauri.org.cn/reference/config/#assetprotocolconfig)，并且必须在同一个 `assetProtocol` 对象的 `scope` 数组上定义其访问范围。

#### 参数

名为“参数”的部分

参数| 类型| 默认值| 描述
---|---|---|---
`filePath`| `string`| `未定义`| 文件路径。
`protocol`| `string`| `'asset'`| 要使用的协议。默认为 `asset`。仅在使用自定义协议时需要设置此项。

#### 返回

名为“返回值”的部分

`string`

可用作 webview 源的 URL。

#### 示例

标题为“Example”的章节

    import { appDataDir, join } from '@tauri-apps/api/path';

    import { convertFileSrc } from '@tauri-apps/api/core';

    const appDataDirPath = await appDataDir();

    const filePath = await join(appDataDirPath, 'assets/video.mp4');

    const assetUrl = convertFileSrc(filePath);

    const video = document.getElementById('my-video');

    const source = document.createElement('source');

    source.type = 'video/mp4';

    source.src = assetUrl;

    video.appendChild(source);

    video.load();

#### 始于

名为“起始版本”的部分

1.0.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/core.ts#L289>

* * *

### invoke()

名为“invoke()”的部分

    function invoke<T>(

       cmd,

       args,

    options?): Promise<T>

向后端发送消息。

#### 类型参数

名为“类型参数”的部分

类型参数
---
`T`

#### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`cmd`| `string`| 命令名称。
`args`| [`InvokeArgs`](/reference/javascript/api/namespacecore/#invokeargs)| 传递给命令的可选参数。
`选项`?| [`InvokeOptions`](/reference/javascript/api/namespacecore/#invokeoptions)| 请求选项。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`T`>

一个解析或拒绝为后端响应的 Promise。

#### 示例

标题为“Example”的章节

    import { invoke } from '@tauri-apps/api/core';

    await invoke('login', { user: 'tauri', password: 'poiwe3h4r5ip3yrhtew9ty' });

#### 始于

名为“起始版本”的部分

1.0.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/core.ts#L251>

* * *

### isTauri()

名为“isTauri()”的部分

    function isTauri(): boolean

#### 返回

名为“返回值”的部分

`布尔值 (boolean)`

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/core.ts#L337>

* * *

### requestPermissions()

名为“requestPermissions()”的部分

    function requestPermissions<T>(plugin): Promise<T>

请求权限。

插件作者应使用此方法来包装其实际实现。

#### 类型参数

名为“类型参数”的部分

类型参数
---
`T`

#### 参数

名为“参数”的部分

参数| 类型
---|---
`plugin`| `string`

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`T`>

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/core.ts#L218>

* * *

### transformCallback()

名为“transformCallback()”的部分

    function transformCallback<T>(callback?, once?): number

将回调存储在已知位置，并返回一个可传递给后端的标识符。后端使用该标识符来 `eval()` 该回调。

#### 类型参数

名为“类型参数”的部分

类型参数| 默认类型
---|---
`T`| `未知`

#### 参数

名为“参数”的部分

参数| 类型| 默认值
---|---|---
`callback`?| (`response`) => `void`| `未定义`
`once`?| `布尔值 (boolean)`| `false`

#### 返回

名为“返回值”的部分

`数字`

与回调函数关联的唯一标识符。

#### 始于

名为“起始版本”的部分

1.0.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/core.ts#L69>