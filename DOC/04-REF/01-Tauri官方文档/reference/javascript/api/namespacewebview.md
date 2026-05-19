# webview

_Source: https://v2.tauri.org.cn/reference/javascript/api/namespacewebview/_

提供用于创建 webview、与其他 webview 进行通信以及操作当前 webview 的 API。

#### Webview 事件

名为“Webview 事件”的部分

可以使用 [Webview.listen](/reference/javascript/api/namespacewebview/#listen) 监听事件

    import { getCurrentWebview } from "@tauri-apps/api/webview";

    getCurrentWebview().listen("my-webview-event", ({ event, payload }) => { });

## 类

名为“类”的部分

### Webview

名为“Webview”的部分

创建新的 webview 或获取现有 webview 的句柄。

Webview 通过 _标签 (label)_ 进行标识，这是一个唯一标识符，稍后可用于引用它。它只能包含字母数字字符 `a-zA-Z` 以及以下特殊字符：`-`、`/`、`:` 和 `_`。

#### 示例

标题为“Example”的章节

    import { Window } from "@tauri-apps/api/window"

    import { Webview } from "@tauri-apps/api/webview"

    const appWindow = new Window('uniqueLabel');

    appWindow.once('tauri://created', async function () {

      // `new Webview` Should be called after the window is successfully created,

      // or webview may not be attached to the window since window is not created yet.

      // loading embedded asset:

      const webview = new Webview(appWindow, 'theUniqueLabel', {

        url: 'path/to/page.html',

        // create a webview with specific logical position and size

        x: 0,

        y: 0,

        width: 800,

        height: 600,

      });

      // alternatively, load a remote URL:

      const webview = new Webview(appWindow, 'theUniqueLabel', {

        url: 'https://github.com/tauri-apps/tauri',

        // create a webview with specific logical position and size

        x: 0,

        y: 0,

        width: 800,

        height: 600,

      });

      webview.once('tauri://created', function () {

        // webview successfully created

      });

      webview.once('tauri://error', function (e) {

        // an error happened creating the webview

      });

      // emit an event to the backend

      await webview.emit("some-event", "data");

      // listen to an event from the backend

      const unlisten = await webview.listen("event-name", e => { });

      unlisten();

    });

#### 始于

名为“起始版本”的部分

2.0.0

#### 扩展自

名为“扩展”的部分

  * [`WebviewWindow`](/reference/javascript/api/namespacewebviewwindow/#webviewwindow)

#### 构造函数

名为“构造函数”的部分

##### new Webview()

名为“new Webview()”的部分

    new Webview(

       window,

       label,

       options): Webview

创建一个新的 Webview。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`window`| [`窗口`](/reference/javascript/api/namespacewindow/#window)| 要将此 webview 添加到的窗口。
`标签`| `string`| 唯一的 webview 标签。必须是字母数字：`a-zA-Z-/:_`。
`选项`| [`WebviewOptions`](/reference/javascript/api/namespacewebview/#webviewoptions)| -

###### 返回

名为“返回值”的部分

[`Webview`](/reference/javascript/api/namespacewebview/#webview)

用于与 webview 通信的 [Webview](/reference/javascript/api/namespacewebview/#webview) 实例。

###### 示例

标题为“Example”的章节

    import { Window } from '@tauri-apps/api/window'

    import { Webview } from '@tauri-apps/api/webview'

    const appWindow = new Window('my-label')

    appWindow.once('tauri://created', async function() {

      const webview = new Webview(appWindow, 'my-label', {

        url: 'https://github.com/tauri-apps/tauri',

        // create a webview with specific logical position and size

        x: 0,

        y: 0,

        width: 800,

        height: 600,

      });

      webview.once('tauri://created', function () {

        // webview successfully created

      });

      webview.once('tauri://error', function (e) {

        // an error happened creating the webview

      });

    });

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L194>

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
`label`| `string`| webview 标签。它是 webview 的唯一标识符，稍后可用于引用它。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L155>
`listeners`| [`Record`](https://typescript.net.cn/docs/handbook/utility-types.html#recordkeys-type)<`string`, [`EventCallback`](/reference/javascript/api/namespaceevent/#eventcallbackt)<`any`>[]>| 本地事件监听器。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L160>
`window`| [`窗口`](/reference/javascript/api/namespacewindow/#window)| 承载此 webview 的窗口。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L157>

#### 方法

名为“方法”的部分

##### clearAllBrowsingData()

名为“clearAllBrowsingData()”的部分

    clearAllBrowsingData(): Promise<void>

清除此 webview 的所有浏览数据。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWebview } from '@tauri-apps/api/webview';

    await getCurrentWebview().clearAllBrowsingData();

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L589>

##### close()

名为“close()”的部分

    close(): Promise<void>

关闭 webview。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWebview } from '@tauri-apps/api/webview';

    await getCurrentWebview().close();

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L436>

##### emit()

“emit()” 章节

    emit<T>(event, payload?): Promise<void>

向所有 [目标](/reference/javascript/api/namespaceevent/#eventtarget) 发出事件。

###### 类型参数

名为“类型参数”的部分

类型参数
---
`T`

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`event`| `string`| 事件名称。必须只包含字母数字字符、`-`、`/`、`:` 和 `_`。
`载荷 (payload)`?| `T`| 事件载荷。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

###### 示例

标题为“Example”的章节

    import { getCurrentWebview } from '@tauri-apps/api/webview';

    await getCurrentWebview().emit('webview-loaded', { loggedIn: true, token: 'authToken' });

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L325>

##### emitTo()

“emitTo()” 章节

    emitTo<T>(

       target,

       event,

    payload?): Promise<void>

向所有与给定目标匹配的 [目标](/reference/javascript/api/namespaceevent/#eventtarget) 发出事件。

###### 类型参数

名为“类型参数”的部分

类型参数
---
`T`

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`目标`| `string` | [`EventTarget`](/reference/javascript/api/namespaceevent/#eventtarget)| 目标窗口/WebView/WebviewWindow 的标签，或原始 [EventTarget](/reference/javascript/api/namespaceevent/#eventtarget) 对象。
`event`| `string`| 事件名称。必须只包含字母数字字符、`-`、`/`、`:` 和 `_`。
`载荷 (payload)`?| `T`| 事件载荷。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

###### 示例

标题为“Example”的章节

    import { getCurrentWebview } from '@tauri-apps/api/webview';

    await getCurrentWebview().emitTo('main', 'webview-loaded', { loggedIn: true, token: 'authToken' });

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L353>

##### hide()

“hide()” 章节

    hide(): Promise<void>

隐藏 webview。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWebview } from '@tauri-apps/api/webview';

    await getCurrentWebview().hide();

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L523>

##### listen()

“listen()” 章节

    listen<T>(event, handler): Promise<UnlistenFn>

监听在此 webview 上发出的事件。

###### 类型参数

名为“类型参数”的部分

类型参数
---
`T`

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`event`| [`EventName`](/reference/javascript/api/namespaceevent/#eventname)| 事件名称。必须只包含字母数字字符、`-`、`/`、`:` 和 `_`。
`处理程序`| [`EventCallback`](/reference/javascript/api/namespaceevent/#eventcallbackt)<`T`>| 事件处理程序。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`UnlistenFn`](/reference/javascript/api/namespaceevent/#unlistenfn)>

一个解析为取消监听事件函数的 Promise。请注意，如果您的监听器超出范围（例如组件已卸载），则必须删除该监听器。

###### 示例

标题为“Example”的章节

    import { getCurrentWebview } from '@tauri-apps/api/webview';

    const unlisten = await getCurrentWebview().listen<string>('state-changed', (event) => {

      console.log(`Got error: ${payload}`);

    });

    // you need to call unlisten if your handler goes out of scope e.g. the component is unmounted

    unlisten();

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L262>

##### onDragDropEvent()

“onDragDropEvent()” 章节

    onDragDropEvent(handler): Promise<UnlistenFn>

监听文件拖放事件。当用户将选定的文件悬停在 webview 上、拖放文件或取消操作时，会触发监听器。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`处理程序`| [`EventCallback`](/reference/javascript/api/namespaceevent/#eventcallbackt)<[`DragDropEvent`](/reference/javascript/api/namespacewebview/#dragdropevent)>

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`UnlistenFn`](/reference/javascript/api/namespaceevent/#unlistenfn)>

一个解析为取消监听事件函数的 Promise。请注意，如果您的监听器超出范围（例如组件已卸载），则必须删除该监听器。

###### 示例

标题为“Example”的章节

    import { getCurrentWebview } from "@tauri-apps/api/webview";

    const unlisten = await getCurrentWebview().onDragDropEvent((event) => {

     if (event.payload.type === 'over') {

       console.log('User hovering', event.payload.position);

     } else if (event.payload.type === 'drop') {

       console.log('User dropped', event.payload.paths);

     } else {

       console.log('File drop cancelled');

     }

    });

    // you need to call unlisten if your handler goes out of scope e.g. the component is unmounted

    unlisten();

当调试器面板打开时，由于已知的限制，此事件的放置位置可能不准确。要获取正确的放置位置，请分离调试器。

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L641>

##### once()

“once()” 章节

    once<T>(event, handler): Promise<UnlistenFn>

仅在 webview 上监听一次发出的事件。

###### 类型参数

名为“类型参数”的部分

类型参数
---
`T`

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`event`| [`EventName`](/reference/javascript/api/namespaceevent/#eventname)| 事件名称。必须只包含字母数字字符、`-`、`/`、`:` 和 `_`。
`处理程序`| [`EventCallback`](/reference/javascript/api/namespaceevent/#eventcallbackt)<`T`>| 事件处理程序。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`UnlistenFn`](/reference/javascript/api/namespaceevent/#unlistenfn)>

一个解析为取消监听事件函数的 Promise。请注意，如果您的监听器超出范围（例如组件已卸载），则必须删除该监听器。

###### 示例

标题为“Example”的章节

    import { getCurrentWebview } from '@tauri-apps/api/webview';

    const unlisten = await getCurrent().once<null>('initialized', (event) => {

      console.log(`Webview initialized!`);

    });

    // you need to call unlisten if your handler goes out of scope e.g. the component is unmounted

    unlisten();

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L297>

##### position()

名为“position()”的部分

    position(): Promise<PhysicalPosition>

webview 客户区左上角相对于桌面左上角的位置。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`PhysicalPosition`](/reference/javascript/api/namespacedpi/#physicalposition)>

webview 的位置。

###### 示例

标题为“Example”的章节

    import { getCurrentWebview } from '@tauri-apps/api/webview';

    const position = await getCurrentWebview().position();

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L398>

##### reparent()

名为“reparent()”的部分

    reparent(window): Promise<void>

将此 webview 移动到给定的标签。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`window`| `string` | [`Window`](/reference/javascript/api/namespacewindow/#window) | [`WebviewWindow`](/reference/javascript/api/namespacewebviewwindow/#webviewwindow)

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWebview } from '@tauri-apps/api/webview';

    await getCurrentWebview().reparent('other-window');

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L572>

##### setAutoResize()

名为“setAutoResize()”的部分

    setAutoResize(autoResize): Promise<void>

设置父窗口调整大小时，webview 是否应自动调整其大小和位置。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`autoResize`| `布尔值 (boolean)`

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWebview } from '@tauri-apps/api/webview';

    await getCurrentWebview().setAutoResize(true);

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L506>

##### setBackgroundColor()

“setBackgroundColor()” 章节

    setBackgroundColor(color): Promise<void>

指定 webview 背景颜色。

#### 平台特定 (Platform-specific)

名为“平台特定 (Platform-specific):”的部分

  * **macOS / iOS** : 未实现。
  * **Windows** :
    * 在 Windows 7 上，不支持透明度，Alpha 值将被忽略。
    * 在高于 7 的 Windows 版本上：不支持半透明颜色，因此除 `0` 以外的任何 Alpha 值都将替换为 `255`

###### 参数

名为“参数”的部分

参数| 类型
---|---
`颜色`| `null` | [`Color`](/reference/javascript/api/namespacewebview/#color)

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 始于

名为“起始版本”的部分

2.1.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L607>

##### setFocus()

“setFocus()” 章节

    setFocus(): Promise<void>

将 webview 置于最前并获得焦点。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWebview } from '@tauri-apps/api/webview';

    await getCurrentWebview().setFocus();

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L490>

##### setPosition()

“setPosition()” 章节

    setPosition(position): Promise<void>

设置 webview 位置。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`位置`| [`LogicalPosition`](/reference/javascript/api/namespacedpi/#logicalposition) | [`PhysicalPosition`](/reference/javascript/api/namespacedpi/#physicalposition) | [`Position`](/reference/javascript/api/namespacedpi/#position)| 新位置，以逻辑像素或物理像素表示。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrent, LogicalPosition } from '@tauri-apps/api/webview';

    await getCurrentWebview().setPosition(new LogicalPosition(600, 500));

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L471>

##### setSize()

“setSize()” 章节

    setSize(size): Promise<void>

调整 webview 大小。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`尺寸`| [`LogicalSize`](/reference/javascript/api/namespacedpi/#logicalsize) | [`PhysicalSize`](/reference/javascript/api/namespacedpi/#physicalsize) | [`Size`](/reference/javascript/api/namespacedpi/#size)| 逻辑大小或物理大小。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrent, LogicalSize } from '@tauri-apps/api/webview';

    await getCurrentWebview().setSize(new LogicalSize(600, 500));

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L453>

##### setZoom()

名为“setZoom()”的部分

    setZoom(scaleFactor): Promise<void>

设置 webview 缩放级别。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`缩放因子`| `数字`

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWebview } from '@tauri-apps/api/webview';

    await getCurrentWebview().setZoom(1.5);

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L555>

##### show()

名为“show()”的章节

    show(): Promise<void>

显示 webview。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWebview } from '@tauri-apps/api/webview';

    await getCurrentWebview().show();

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L539>

##### size()

名为“size()”的章节

    size(): Promise<PhysicalSize>

webview 客户区的物理大小。客户区是 webview 的内容，不包括标题栏和边框。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`PhysicalSize`](/reference/javascript/api/namespacedpi/#physicalsize)>

webview 的大小。

###### 示例

标题为“Example”的章节

    import { getCurrentWebview } from '@tauri-apps/api/webview';

    const size = await getCurrentWebview().size();

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L415>

##### getAll()

名为“getAll()”的章节

    static getAll(): Promise<Webview[]>

获取所有可用 webview 的 `Webview` 实例列表。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`Webview`](/reference/javascript/api/namespacewebview/#webview)[]>

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L239>

##### getByLabel()

名为“getByLabel()”的章节

    static getByLabel(label): Promise<null | Webview>

获取与给定标签关联的 webview 的 Webview 实例。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`标签`| `string`| webview 标签。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`null` | [`Webview`](/reference/javascript/api/namespacewebview/#webview)>

用于与 webview 通信的 Webview 实例，如果 webview 不存在则为 null。

###### 示例

标题为“Example”的章节

    import { Webview } from '@tauri-apps/api/webview';

    const mainWebview = Webview.getByLabel('main');

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L225>

##### getCurrent()

名为“getCurrent()”的章节

    static getCurrent(): Webview

获取当前 webview 的 `Webview` 实例。

###### 返回

名为“返回值”的部分

[`Webview`](/reference/javascript/api/namespacewebview/#webview)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L232>

## 接口

名为“接口”的部分

### WebviewOptions

名为“WebviewOptions”的部分

要创建的 webview 的配置。

#### 始于

名为“起始版本”的部分

2.0.0

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
`acceptFirstMouse?`| `布尔值 (boolean)`| 在 macOS 上，点击非活动 webview 是否也点击该 webview。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L746>
`allowLinkPreview?`| `布尔值 (boolean)`| 在 macOS 和 iOS 上，长按链接时会有链接预览，默认启用此功能。详见 <https://docs.rs/objc2-web-kit/latest/objc2_web_kit/struct.WKWebView.html#method.allowsLinkPreview>| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L850>
`backgroundColor?`| [`Color`](/reference/javascript/api/namespacewebview/#color)| 设置窗口和 webview 的背景颜色。平台特定： - **macOS / iOS** : 未实现。 - **Windows** : - 在 Windows 7 上，Alpha 通道被忽略。 - 在 Windows 8 及更高版本上，如果 Alpha 通道不为 `0`，它将被忽略。 **自** 2.1.0 版本起| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L822>
`backgroundThrottling?`| [`BackgroundThrottlingPolicy`](/reference/javascript/api/namespacewindow/#backgroundthrottlingpolicy)| 更改默认的后台节流行为。默认情况下，浏览器使用暂停策略，该策略会在视图最小化或隐藏后约 5 分钟内节流计时器，甚至卸载整个选项卡（视图）以释放资源。这将暂停所有任务，直到文档的可见性状态通过将视图带回前台从隐藏变为可见。## 平台特定 - **Linux / Windows / Android** : 不支持。像挂起的 WebLock 事务这样的解决方法可能就足够了。 - **iOS** : 17.0+ 版本支持。 - **macOS** : 14.0+ 版本支持。请参阅 <https://github.com/tauri-apps/tauri/issues/5250#issuecomment-2569380578> **自** 2.3.0 版本起| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L841>
`dataDirectory?`| `string`| 为 webview 的数据目录（localStorage、缓存等）设置一个自定义路径，**相对于 [`appDataDir()`]/${label}**。出于安全原因，该位置之外的路径只能在 Rust 端配置。平台特定： - **Windows** : 设置值（如 `additionalBrowserArgs`、`browserExtensionsEnabled` 或 `scrollBarStyle`）不同的 WebViews 必须有不同的数据目录。 - **macOS / iOS** : 不支持，请改用 `dataStoreIdentifier`。 - **Android** : 不支持。 **自** 2.9.0 版本起| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L870>
`dataStoreIdentifier?`| `数字`[]| 使用自定义数据存储标识符初始化 WebView。这可以看作是 WKWebView 中不可用的 `dataDirectory` 的替代方案。请参阅 <https://developer.apple.com/documentation/webkit/wkwebsitedatastore/init(foridentifier:)?language=objc> 该数组必须包含 16 个 u8 数字。平台特定： - **macOS / iOS** : 在 macOS >= 14 和 iOS >= 17 上可用 - **Windows / Linux / Android** : 不支持。 **自** 2.9.0 版本起| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L884>
`devtools?`| `布尔值 (boolean)`| 是否启用通常称为浏览器 devtools 的 web 检查器。默认启用。此 API 在**调试** 构建中有效，但在**发布** 构建中需要 `devtools` 功能标志才能启用它。平台特定 - macOS: 这将调用 **macOS** 上的私有函数。 - Android: 在 Chrome 中打开 `chrome://inspect/#devices` 以获取 devtools 窗口。Wry 的 `WebView` devtools API 在 Android 上不受支持。 - iOS: 打开 Safari > 开发 > [您的设备名称] > [您的 WebView] 以获取 devtools 窗口。 **自** 2.1.0 版本起| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L809>
`disableInputAccessoryView?`| `布尔值 (boolean)`| 允许在 iOS 上禁用输入附件视图。附件视图是当文本输入元素获得焦点时出现在键盘上方的视图。它通常显示带有“完成”、“下一个”按钮的视图。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L857>
`dragDropEnabled?`| `布尔值 (boolean)`| webview 上是否启用了拖放功能。默认启用。在 Windows 上，若要在前端使用 HTML5 拖放，则必须禁用它。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L742>
`focus?`| `布尔值 (boolean)`| webview 是否应该获得焦点 **自** 2.1.0 版本起| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L736>
`height`| `数字`| 初始高度（以逻辑像素为单位）。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L724>
`incognito?`| `布尔值 (boolean)`| webview 是否应以隐身模式启动。平台特定 - **Android:** 不支持。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L758>
`javascriptDisabled?`| `布尔值 (boolean)`| 我们是否应该禁用 webview 上的 JavaScript 代码执行。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L845>
`proxyUrl?`| `string`| 用于所有网络请求的 WebView 代理 URL。必须是 `http://` 或 `socks5://` URL。平台特定 - **macOS** : 需要 `macos-proxy` 功能标志，且仅为 macOS 14+ 编译。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L768>
`scrollBarStyle?`| [`ScrollBarStyle`](/reference/javascript/api/namespacewindow/#scrollbarstyle)| 指定与 webview 一起使用的原生滚动条样式。修改滚动条的 CSS 样式应用于此处配置的原生外观之上。默认为 `default`，即浏览器默认值。## 平台特定 - **Windows** : - `fluentOverlay` 需要 WebView2 Runtime 125.0.2535.41 或更高版本，在旧版本上无效。 - 此选项必须为所有 webview 指定相同的值。 - **Linux / Android / iOS / macOS** : 不支持。仅支持 `Default` 且不执行任何操作。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L899>
`transparent?`| `布尔值 (boolean)`| webview 是否透明。注意，在 `macOS` 上，这需要 `macos-private-api` 功能标志，在 `tauri.conf.json > app > macOSPrivateApi` 下启用。警告：在 `macOS` 上使用私有 API 会导致您的应用程序无法被 `App Store` 接受。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L730>
`url?`| `string`| 要打开的远程 URL 或本地文件路径。 - URL（例如 `https://github.com/tauri-apps`）直接在 Tauri webview 中打开。 - data: URL（例如 `data:text/html,<html>...`）仅在 `tauri` 依赖项启用 `webview-data-url` Cargo 功能时才支持。 - 本地文件路径或路由（例如 `/path/to/page.html` 或 `/users`）会被追加到应用程序 URL（开发环境为 devServer URL，生产环境为 `tauri:///` 和 `https://tauri.localhost/`）。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L716>
`useHttpsScheme?`| `布尔值 (boolean)`| 设置自定义协议在 Windows 和 Android 上是否应使用 `https://<scheme>.localhost` 而不是默认的 `http://<scheme>.localhost`。默认为 `false`。#### 注意 使用 `https` 方案在尝试获取 `http` 端点时将不会允许混合内容，因此不会匹配 macOS 和 Linux 上使用的 `<scheme>://` 协议的行为。#### 警告 在版本之间更改此值将更改 IndexedDB、cookie 和 localstorage 的位置，您的应用程序将无法访问它们。 **自** 2.1.0 版本起| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L795>
`userAgent?`| `string`| webview 的用户代理 (user agent)。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L750>
`width`| `数字`| 初始宽度（以逻辑像素为单位）。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L722>
`x`| `数字`| 初始垂直位置（以逻辑像素为单位）。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L718>
`y`| `数字`| 初始水平位置（以逻辑像素为单位）。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L720>
`zoomHotkeysEnabled?`| `布尔值 (boolean)`| 是否启用通过热键进行页面缩放。平台特定： - **Windows** : 控制 WebView2 的 [`IsZoomControlEnabled`](https://learn.microsoft.com/en-us/microsoft-edge/webview2/reference/winrt/microsoft_web_webview2_core/corewebview2settings?view=webview2-winrt-1.0.2420.47#iszoomcontrolenabled) 设置。 - **MacOS / Linux** : 注入一个使用 `ctrl/command` \+ `-/=` 进行缩放的 polyfill，每步 20%，范围从 20% 到 1000%。需要 `webview:allow-set-webview-zoom` 权限 - **Android / iOS** : 不支持。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L780>

## 类型别名 (Type Aliases)

名为“类型别名”的部分

### Color

标题为“Color”的章节

    type Color: [number, number, number] | [number, number, number, number] | object | string;

RGBA 颜色。每个值最小为 0，最大为 255。

它可以是字符串 `#ffffff`、3 或 4 个元素的数组或对象。

#### 始于

名为“起始版本”的部分

2.0.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2079>

* * *

### DragDropEvent

标题为“DragDropEvent”的章节

    type DragDropEvent: object | object | object | object;

拖放事件类型。

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L43>

## 函数

名为“函数”的部分

### getAllWebviews()

名为“getAllWebviews()”的部分

    function getAllWebviews(): Promise<Webview[]>

获取所有可用 webview 的 `Webview` 实例列表。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`Webview`](/reference/javascript/api/namespacewebview/#webview)[]>

#### 始于

名为“起始版本”的部分

2.0.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L70>

* * *

### getCurrentWebview()

名为“getCurrentWebview()”的部分

    function getCurrentWebview(): Webview

获取当前 webview 的 `Webview` 实例。

#### 返回

名为“返回值”的部分

[`Webview`](/reference/javascript/api/namespacewebview/#webview)

#### 始于

名为“起始版本”的部分

2.0.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L54>