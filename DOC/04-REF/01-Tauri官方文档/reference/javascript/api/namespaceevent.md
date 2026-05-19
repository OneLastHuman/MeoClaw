# event

_Source: https://v2.tauri.org.cn/reference/javascript/api/namespaceevent/_

事件系统允许你向后端发送事件并监听来自后端的事件。

当 `tauri.conf.json` 中的 [`app.withGlobalTauri`](https://v2.tauri.org.cn/reference/config/#withglobaltauri) 设置为 `true` 时，也可以通过 `window.__TAURI__.event` 访问此包。

## 枚举

标题为“枚举”的章节

### TauriEvent

标题为“TauriEvent”的章节

#### 始于

名为“起始版本”的部分

1.1.0

#### 枚举成员

标题为“枚举成员”的章节

##### DRAG_DROP

标题为“DRAG_DROP”的章节

    DRAG_DROP: "tauri://drag-drop";

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L71>

##### DRAG_ENTER

标题为“DRAG_ENTER”的章节

    DRAG_ENTER: "tauri://drag-enter";

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L69>

##### DRAG_LEAVE

标题为“DRAG_LEAVE”的章节

    DRAG_LEAVE: "tauri://drag-leave";

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L72>

##### DRAG_OVER

标题为“DRAG_OVER”的章节

    DRAG_OVER: "tauri://drag-over";

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L70>

##### WEBVIEW_CREATED

标题为“WEBVIEW_CREATED”的章节

    WEBVIEW_CREATED: "tauri://webview-created";

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L68>

##### WINDOW_BLUR

标题为“WINDOW_BLUR”的章节

    WINDOW_BLUR: "tauri://blur";

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L64>

##### WINDOW_CLOSE_REQUESTED

标题为“WINDOW_CLOSE_REQUESTED”的章节

    WINDOW_CLOSE_REQUESTED: "tauri://close-requested";

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L61>

##### WINDOW_CREATED

标题为“WINDOW_CREATED”的章节

    WINDOW_CREATED: "tauri://window-created";

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L67>

##### WINDOW_DESTROYED

标题为“WINDOW_DESTROYED”的章节

    WINDOW_DESTROYED: "tauri://destroyed";

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L62>

##### WINDOW_FOCUS

标题为“WINDOW_FOCUS”的章节

    WINDOW_FOCUS: "tauri://focus";

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L63>

##### WINDOW_MOVED

标题为“WINDOW_MOVED”的章节

    WINDOW_MOVED: "tauri://move";

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L60>

##### WINDOW_RESIZED

标题为“WINDOW_RESIZED”的章节

    WINDOW_RESIZED: "tauri://resize";

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L59>

##### WINDOW_SCALE_FACTOR_CHANGED

标题为“WINDOW_SCALE_FACTOR_CHANGED”的章节

    WINDOW_SCALE_FACTOR_CHANGED: "tauri://scale-change";

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L65>

##### WINDOW_THEME_CHANGED

标题为“WINDOW_THEME_CHANGED”的章节

    WINDOW_THEME_CHANGED: "tauri://theme-changed";

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L66>

## 接口

名为“接口”的部分

### Event<T>

标题为“Event<T>”的章节

#### 类型参数

名为“类型参数”的部分

类型参数
---
`T`

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
`event`| [`EventName`](/reference/javascript/api/namespaceevent/#eventname)| 事件名称| **来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L32>
`id`| `数字`| 用于取消监听的事件标识符| **来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L34>
`payload`| `T`| 事件负载| **来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L36>

* * *

### 选项

选项章节

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
`target?`| `string` | [`EventTarget`](/reference/javascript/api/namespaceevent/#eventtarget)| 要监听的事件目标，默认为 `{ kind: 'Any' }`，请参阅 [EventTarget](/reference/javascript/api/namespaceevent/#eventtarget)。如果提供了字符串，则使用 EventTarget.AnyLabel。| **来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L52>

## 类型别名 (Type Aliases)

名为“类型别名”的部分

### EventCallback()<T>

标题为“EventCallback()<T>”的章节

    type EventCallback<T>: (event) => void;

#### 类型参数

名为“类型参数”的部分

类型参数
---
`T`

#### 参数

名为“参数”的部分

参数| 类型
---|---
`event`| [`Event`](/reference/javascript/api/namespaceevent/#eventt)<`T`>

#### 返回

名为“返回值”的部分

`空`

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L39>

* * *

### EventName

标题为“EventName”的章节

    type EventName: `${TauriEvent}` | string & Record<never, never>;

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L44>

* * *

### EventTarget

标题为“EventTarget”的章节

    type EventTarget:

      | object

      | object

      | object

      | object

      | object

      | object;

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L22>

* * *

### UnlistenFn()

标题为“UnlistenFn()”的章节

    type UnlistenFn: () => void;

#### 返回

名为“返回值”的部分

`空`

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L42>

## 函数

名为“函数”的部分

### emit()

“emit()” 章节

    function emit<T>(event, payload?): Promise<void>

向所有 [目标](/reference/javascript/api/namespaceevent/#eventtarget) 发出事件。

#### 类型参数

名为“类型参数”的部分

类型参数
---
`T`

#### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`event`| `string`| 事件名称。必须只包含字母数字字符、`-`、`/`、`:` 和 `_`。
`载荷 (payload)`?| `T`| 事件载荷。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

#### 示例

标题为“Example”的章节

    import { emit } from '@tauri-apps/api/event';

    await emit('frontend-loaded', { loggedIn: true, token: 'authToken' });

#### 始于

名为“起始版本”的部分

1.0.0

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L186>

* * *

### emitTo()

“emitTo()” 章节

    function emitTo<T>(

       target,

       event,

    payload?): Promise<void>

向所有与给定目标匹配的 [目标](/reference/javascript/api/namespaceevent/#eventtarget) 发出事件。

#### 类型参数

名为“类型参数”的部分

类型参数
---
`T`

#### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`目标`| `string` | [`EventTarget`](/reference/javascript/api/namespaceevent/#eventtarget)| 目标窗口/WebView/WebviewWindow 的标签，或原始 [EventTarget](/reference/javascript/api/namespaceevent/#eventtarget) 对象。
`event`| `string`| 事件名称。必须只包含字母数字字符、`-`、`/`、`:` 和 `_`。
`载荷 (payload)`?| `T`| 事件载荷。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

#### 示例

标题为“Example”的章节

    import { emitTo } from '@tauri-apps/api/event';

    await emitTo('main', 'frontend-loaded', { loggedIn: true, token: 'authToken' });

#### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L208>

* * *

### listen()

“listen()” 章节

    function listen<T>(

       event,

       handler,

    options?): Promise<UnlistenFn>

监听发往任何 [目标](/reference/javascript/api/namespaceevent/#eventtarget) 的已触发事件。

#### 类型参数

名为“类型参数”的部分

类型参数
---
`T`

#### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`event`| [`EventName`](/reference/javascript/api/namespaceevent/#eventname)| 事件名称。必须只包含字母数字字符、`-`、`/`、`:` 和 `_`。
`处理程序`| [`EventCallback`](/reference/javascript/api/namespaceevent/#eventcallbackt)<`T`>| 事件处理程序回调。
`选项`?| [`选项`](/reference/javascript/api/namespaceevent/#options)| 事件监听选项。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`UnlistenFn`](/reference/javascript/api/namespaceevent/#unlistenfn)>

一个解析为取消监听事件函数的 Promise。请注意，如果您的监听器超出范围（例如组件已卸载），则必须删除该监听器。

#### 示例

标题为“Example”的章节

    import { listen } from '@tauri-apps/api/event';

    const unlisten = await listen<string>('error', (event) => {

      console.log(`Got error, payload: ${event.payload}`);

    });

    // you need to call unlisten if your handler goes out of scope e.g. the component is unmounted

    unlisten();

#### 始于

名为“起始版本”的部分

1.0.0

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L113>

* * *

### once()

“once()” 章节

    function once<T>(

       event,

       handler,

    options?): Promise<UnlistenFn>

仅监听一次发往任何 [目标](/reference/javascript/api/namespaceevent/#eventtarget) 的已触发事件。

#### 类型参数

名为“类型参数”的部分

类型参数
---
`T`

#### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`event`| [`EventName`](/reference/javascript/api/namespaceevent/#eventname)| 事件名称。必须只包含字母数字字符、`-`、`/`、`:` 和 `_`。
`处理程序`| [`EventCallback`](/reference/javascript/api/namespaceevent/#eventcallbackt)<`T`>| 事件处理程序回调。
`选项`?| [`选项`](/reference/javascript/api/namespaceevent/#options)| 事件监听选项。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`UnlistenFn`](/reference/javascript/api/namespaceevent/#unlistenfn)>

一个解析为取消监听事件函数的 Promise。请注意，如果您的监听器超出范围（例如组件已卸载），则必须删除该监听器。

#### 示例

标题为“Example”的章节

    import { once } from '@tauri-apps/api/event';

    interface LoadedPayload {

      loggedIn: boolean,

      token: string

    }

    const unlisten = await once<LoadedPayload>('loaded', (event) => {

      console.log(`App is loaded, loggedIn: ${event.payload.loggedIn}, token: ${event.payload.token}`);

    });

    // you need to call unlisten if your handler goes out of scope e.g. the component is unmounted

    unlisten();

#### 始于

名为“起始版本”的部分

1.0.0

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/event.ts#L157>