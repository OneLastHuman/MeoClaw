# webviewWindow

_Source: https://v2.tauri.org.cn/reference/javascript/api/namespacewebviewwindow/_

## 参考

标题为“参考”的章节

### Color

标题为“Color”的章节

重新导出 [Color](/reference/javascript/api/namespacewebview/#color)

### DragDropEvent

标题为“DragDropEvent”的章节

重新导出 [DragDropEvent](/reference/javascript/api/namespacewebview/#dragdropevent)

## 类

名为“类”的部分

### WebviewWindow

名为“WebviewWindow”的章节

创建新的 webview 或获取现有 webview 的句柄。

Webview 通过 _标签（label）_ 进行标识，这是一个唯一标识符，稍后可用于引用它。它只能包含字母数字字符 `a-zA-Z` 以及以下特殊字符：`-`、`/`、`:` 和 `_`。

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

#### 继承 (Extends)

名为“继承自”的章节

  * [`Webview`](/reference/javascript/api/namespacewebview/#webview).[`Window`](/reference/javascript/api/namespacewindow/#window)

#### 构造函数

名为“构造函数”的部分

##### new WebviewWindow()

名为“new WebviewWindow()”的章节

    new WebviewWindow(label, options): WebviewWindow

创建一个托管 [Webview](/reference/javascript/api/namespacewebview/#webview) 的新 [Window](/reference/javascript/api/namespacewindow/#window)。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`标签`| `string`| 唯一的 webview 标签。必须是字母数字：`a-zA-Z-/:_`。
`选项`| [`Omit`](https://typescript.net.cn/docs/handbook/utility-types.html#omittype-keys)<[`WebviewOptions`](/reference/javascript/api/namespacewebview/#webviewoptions), `"x"` | `"y"` | `"width"` | `"height"`> & [`WindowOptions`](/reference/javascript/api/namespacewindow/#windowoptions)| -

###### 返回

名为“返回值”的部分

[`WebviewWindow`](/reference/javascript/api/namespacewebviewwindow/#webviewwindow)

用于与窗口和 webview 通信的 [WebviewWindow](/reference/javascript/api/namespacewebviewwindow/#webviewwindow) 实例。

###### 示例

标题为“Example”的章节

    import { WebviewWindow } from '@tauri-apps/api/webviewWindow'

    const webview = new WebviewWindow('my-label', {

      url: 'https://github.com/tauri-apps/tauri'

    });

    webview.once('tauri://created', function () {

     // webview successfully created

    });

    webview.once('tauri://error', function (e) {

     // an error happened creating the webview

    });

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`constructor`](/reference/javascript/api/namespacewindow/#constructors-1)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webviewWindow.ts#L75>

#### 属性

名为“属性”的部分

属性| 类型| 描述| 继承自 (Inherited from)| 定义于
---|---|---|---|---
`label`| `string`| webview 标签。它是 webview 的唯一标识符，稍后可用于引用它。| [`Window`](/reference/javascript/api/namespacewindow/#window).[`label`](/reference/javascript/api/namespacewindow/#label)| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webviewWindow.ts#L51>
`listeners`| [`Record`](https://typescript.net.cn/docs/handbook/utility-types.html#recordkeys-type)<`string`, [`EventCallback`](/reference/javascript/api/namespaceevent/#eventcallbackt)<`any`>[]>| 本地事件监听器。| [`Window`](/reference/javascript/api/namespacewindow/#window).[`listeners`](/reference/javascript/api/namespacewindow/#listeners)| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webviewWindow.ts#L54>
`window`| [`窗口`](/reference/javascript/api/namespacewindow/#window)| 托管此 webview 的窗口。| [`Webview`](/reference/javascript/api/namespacewebview/#webview).[`window`](/reference/javascript/api/namespacewebview/#window)| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L157>

#### 方法

名为“方法”的部分

##### center()

名为“center()”的章节

    center(): Promise<void>

使窗口居中。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    await getCurrentWindow().center();

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`center`](/reference/javascript/api/namespacewindow/#center)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L835>

##### clearAllBrowsingData()

名为“clearAllBrowsingData()”的章节

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

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Webview`](/reference/javascript/api/namespacewebview/#webview).[`clearAllBrowsingData`](/reference/javascript/api/namespacewebview/#clearallbrowsingdata)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L589>

##### clearEffects()

名为“clearEffects()”的章节

    clearEffects(): Promise<void>

如果可能，清除所有已应用的特效。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`clearEffects`](/reference/javascript/api/namespacewindow/#cleareffects)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1223>

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

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`close`](/reference/javascript/api/namespacewindow/#close)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L436>

##### destroy()

名为“destroy()”的章节

    destroy(): Promise<void>

销毁窗口。其行为类似于 [Window.close](/reference/javascript/api/namespacewindow/#close)，但会强制关闭窗口而不是触发 closeRequested 事件。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    await getCurrentWindow().destroy();

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`destroy`](/reference/javascript/api/namespacewindow/#destroy)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1160>

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

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`emit`](/reference/javascript/api/namespacewindow/#emit)

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

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`emitTo`](/reference/javascript/api/namespacewindow/#emitto)

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

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`hide`](/reference/javascript/api/namespacewindow/#hide)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L523>

##### innerPosition()

名为“innerPosition()”的章节

    innerPosition(): Promise<PhysicalPosition>

窗口客户区左上角相对于桌面左上角的位置。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`PhysicalPosition`](/reference/javascript/api/namespacedpi/#physicalposition)>

窗口的内部位置。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    const position = await getCurrentWindow().innerPosition();

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`innerPosition`](/reference/javascript/api/namespacewindow/#innerposition)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L537>

##### innerSize()

名为“innerSize()”的章节

    innerSize(): Promise<PhysicalSize>

窗口客户区的物理尺寸。客户区是窗口的内容部分，不包括标题栏和边框。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`PhysicalSize`](/reference/javascript/api/namespacedpi/#physicalsize)>

窗口的内部尺寸。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    const size = await getCurrentWindow().innerSize();

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`innerSize`](/reference/javascript/api/namespacewindow/#innersize)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L570>

##### isAlwaysOnTop()

名为“isAlwaysOnTop()”的章节

    isAlwaysOnTop(): Promise<boolean>

窗口是否配置为始终置顶显示。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`boolean`>

窗口是否可见。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    const alwaysOnTop = await getCurrentWindow().isAlwaysOnTop();

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`isAlwaysOnTop`](/reference/javascript/api/namespacewindow/#isalwaysontop)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L817>

##### isClosable()

名为“isClosable()”的章节

    isClosable(): Promise<boolean>

获取窗口原生关闭按钮的状态。

平台特定

  * **iOS / Android:** 不支持。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`boolean`>

窗口的原生关闭按钮是否启用。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    const closable = await getCurrentWindow().isClosable();

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`isClosable`](/reference/javascript/api/namespacewindow/#isclosable)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L750>

##### isDecorated()

名为“isDecorated()”的章节

    isDecorated(): Promise<boolean>

获取窗口当前的装饰状态。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`boolean`>

窗口是否带有装饰。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    const decorated = await getCurrentWindow().isDecorated();

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`isDecorated`](/reference/javascript/api/namespacewindow/#isdecorated)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L671>

##### isEnabled()

“isEnabled()” 章节

    isEnabled(): Promise<boolean>

窗口是否处于启用状态。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`boolean`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    await getCurrentWindow().setEnabled(false);

###### 始于

名为“起始版本”的部分

2.0.0

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`isEnabled`](/reference/javascript/api/namespacewindow/#isenabled)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L927>

##### isFocused()

名为“isFocused()”的章节

    isFocused(): Promise<boolean>

获取窗口当前的焦点状态。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`boolean`>

窗口是否处于焦点中。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    const focused = await getCurrentWindow().isFocused();

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`isFocused`](/reference/javascript/api/namespacewindow/#isfocused)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L655>

##### isFullscreen()

名为“isFullscreen()”的章节

    isFullscreen(): Promise<boolean>

获取窗口当前的全屏状态。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`boolean`>

窗口是否处于全屏模式。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    const fullscreen = await getCurrentWindow().isFullscreen();

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`isFullscreen`](/reference/javascript/api/namespacewindow/#isfullscreen)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L609>

##### isMaximizable()

名为“isMaximizable()”的章节

    isMaximizable(): Promise<boolean>

获取窗口原生最大化按钮的状态。

平台特定

  * **Linux / iOS / Android：** 不支持。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`boolean`>

窗口的原生最大化按钮是否启用。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    const maximizable = await getCurrentWindow().isMaximizable();

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`isMaximizable`](/reference/javascript/api/namespacewindow/#ismaximizable)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L708>

##### isMaximized()

名为“isMaximized()”的章节

    isMaximized(): Promise<boolean>

获取窗口当前的最大化状态。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`boolean`>

窗口是否最大化。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    const maximized = await getCurrentWindow().isMaximized();

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`isMaximized`](/reference/javascript/api/namespacewindow/#ismaximized)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L639>

##### isMinimizable()

名为“isMinimizable()”的章节

    isMinimizable(): Promise<boolean>

获取窗口原生最小化按钮的状态。

平台特定

  * **Linux / iOS / Android：** 不支持。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`boolean`>

窗口的原生最小化按钮是否启用。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    const minimizable = await getCurrentWindow().isMinimizable();

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`isMinimizable`](/reference/javascript/api/namespacewindow/#isminimizable)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L729>

##### isMinimized()

名为“isMinimized()”的章节

    isMinimized(): Promise<boolean>

获取窗口当前的最小化状态。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`boolean`>

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    const minimized = await getCurrentWindow().isMinimized();

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`isMinimized`](/reference/javascript/api/namespacewindow/#isminimized)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L623>

##### isResizable()

名为“isResizable()”的章节

    isResizable(): Promise<boolean>

获取窗口当前的可调整大小状态。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`boolean`>

窗口是否可调整大小。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    const resizable = await getCurrentWindow().isResizable();

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`isResizable`](/reference/javascript/api/namespacewindow/#isresizable)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L687>

##### isVisible()

名为“isVisible()”的章节

    isVisible(): Promise<boolean>

获取窗口当前的可见状态。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`boolean`>

窗口是否可见。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    const visible = await getCurrentWindow().isVisible();

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`isVisible`](/reference/javascript/api/namespacewindow/#isvisible)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L766>

##### listen()

“listen()” 章节

    listen<T>(event, handler): Promise<UnlistenFn>

监听此 webview 窗口上触发的事件。

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

    import { WebviewWindow } from '@tauri-apps/api/webviewWindow';

    const unlisten = await WebviewWindow.getCurrent().listen<string>('state-changed', (event) => {

      console.log(`Got error: ${payload}`);

    });

    // you need to call unlisten if your handler goes out of scope e.g. the component is unmounted

    unlisten();

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`listen`](/reference/javascript/api/namespacewindow/#listen)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webviewWindow.ts#L155>

##### maximize()

名为“maximize()”的章节

    maximize(): Promise<void>

最大化窗口。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    await getCurrentWindow().maximize();

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`maximize`](/reference/javascript/api/namespacewindow/#maximize)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1030>

##### minimize()

名为“minimize()”的章节

    minimize(): Promise<void>

最小化窗口。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    await getCurrentWindow().minimize();

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`minimize`](/reference/javascript/api/namespacewindow/#minimize)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1078>

##### onCloseRequested()

名为“onCloseRequested()”的章节

    onCloseRequested(handler): Promise<UnlistenFn>

监听窗口关闭请求。当用户请求关闭窗口时触发。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`处理程序`| (`event`) => `void` | [`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`UnlistenFn`](/reference/javascript/api/namespaceevent/#unlistenfn)>

一个解析为取消监听事件函数的 Promise。请注意，如果您的监听器超出范围（例如组件已卸载），则必须删除该监听器。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from "@tauri-apps/api/window";

    import { confirm } from '@tauri-apps/api/dialog';

    const unlisten = await getCurrentWindow().onCloseRequested(async (event) => {

      const confirmed = await confirm('Are you sure?');

      if (!confirmed) {

        // user did not confirm closing the window; let's prevent it

        event.preventDefault();

      }

    });

    // you need to call unlisten if your handler goes out of scope e.g. the component is unmounted

    unlisten();

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`onCloseRequested`](/reference/javascript/api/namespacewindow/#oncloserequested)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1885>

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

当调试器面板打开时，由于已知限制，此事件的放置位置可能不准确。要获取正确的放置位置，请分离调试器。

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`onDragDropEvent`](/reference/javascript/api/namespacewindow/#ondragdropevent)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L641>

##### onFocusChanged()

名为“onFocusChanged()”的章节

    onFocusChanged(handler): Promise<UnlistenFn>

监听窗口焦点更改。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`处理程序`| [`EventCallback`](/reference/javascript/api/namespaceevent/#eventcallbackt)<`boolean`>

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`UnlistenFn`](/reference/javascript/api/namespaceevent/#unlistenfn)>

一个解析为取消监听事件函数的 Promise。请注意，如果您的监听器超出范围（例如组件已卸载），则必须删除该监听器。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from "@tauri-apps/api/window";

    const unlisten = await getCurrentWindow().onFocusChanged(({ payload: focused }) => {

     console.log('Focus changed, window is focused? ' + focused);

    });

    // you need to call unlisten if your handler goes out of scope e.g. the component is unmounted

    unlisten();

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`onFocusChanged`](/reference/javascript/api/namespacewindow/#onfocuschanged)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2001>

##### onMoved()

名为“onMoved()”的章节

    onMoved(handler): Promise<UnlistenFn>

监听窗口移动。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`处理程序`| [`EventCallback`](/reference/javascript/api/namespaceevent/#eventcallbackt)<[`PhysicalPosition`](/reference/javascript/api/namespacedpi/#physicalposition)>

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`UnlistenFn`](/reference/javascript/api/namespaceevent/#unlistenfn)>

一个解析为取消监听事件函数的 Promise。请注意，如果您的监听器超出范围（例如组件已卸载），则必须删除该监听器。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from "@tauri-apps/api/window";

    const unlisten = await getCurrentWindow().onMoved(({ payload: position }) => {

     console.log('Window moved', position);

    });

    // you need to call unlisten if your handler goes out of scope e.g. the component is unmounted

    unlisten();

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`onMoved`](/reference/javascript/api/namespacewindow/#onmoved)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1856>

##### onResized()

名为“onResized()”的章节

    onResized(handler): Promise<UnlistenFn>

监听窗口调整大小。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`处理程序`| [`EventCallback`](/reference/javascript/api/namespaceevent/#eventcallbackt)<[`PhysicalSize`](/reference/javascript/api/namespacedpi/#physicalsize)>

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`UnlistenFn`](/reference/javascript/api/namespaceevent/#unlistenfn)>

一个解析为取消监听事件函数的 Promise。请注意，如果您的监听器超出范围（例如组件已卸载），则必须删除该监听器。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from "@tauri-apps/api/window";

    const unlisten = await getCurrentWindow().onResized(({ payload: size }) => {

     console.log('Window resized', size);

    });

    // you need to call unlisten if your handler goes out of scope e.g. the component is unmounted

    unlisten();

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`onResized`](/reference/javascript/api/namespacewindow/#onresized)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1832>

##### onScaleChanged()

名为“onScaleChanged()”的章节

    onScaleChanged(handler): Promise<UnlistenFn>

监听窗口缩放更改。当窗口的缩放因子发生变化时触发。以下用户操作可能导致 DPI 更改：

  * 更改显示器的分辨率。
  * 更改显示器的缩放因子（例如在 Windows 的控制面板中）。
  * 将窗口移动到具有不同缩放因子的显示器上。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`处理程序`| [`EventCallback`](/reference/javascript/api/namespaceevent/#eventcallbackt)<[`ScaleFactorChanged`](/reference/javascript/api/namespacewindow/#scalefactorchanged)>

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`UnlistenFn`](/reference/javascript/api/namespaceevent/#unlistenfn)>

一个解析为取消监听事件函数的 Promise。请注意，如果您的监听器超出范围（例如组件已卸载），则必须删除该监听器。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from "@tauri-apps/api/window";

    const unlisten = await getCurrentWindow().onScaleChanged(({ payload }) => {

     console.log('Scale changed', payload.scaleFactor, payload.size);

    });

    // you need to call unlisten if your handler goes out of scope e.g. the component is unmounted

    unlisten();

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`onScaleChanged`](/reference/javascript/api/namespacewindow/#onscalechanged)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2041>

##### onThemeChanged()

名为“onThemeChanged()”的章节

    onThemeChanged(handler): Promise<UnlistenFn>

监听系统主题更改。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`处理程序`| [`EventCallback`](/reference/javascript/api/namespaceevent/#eventcallbackt)<[`Theme`](/reference/javascript/api/namespacewindow/#theme-2)>

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`UnlistenFn`](/reference/javascript/api/namespaceevent/#unlistenfn)>

一个解析为取消监听事件函数的 Promise。请注意，如果您的监听器超出范围（例如组件已卸载），则必须删除该监听器。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from "@tauri-apps/api/window";

    const unlisten = await getCurrentWindow().onThemeChanged(({ payload: theme }) => {

     console.log('New theme: ' + theme);

    });

    // you need to call unlisten if your handler goes out of scope e.g. the component is unmounted

    unlisten();

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`onThemeChanged`](/reference/javascript/api/namespacewindow/#onthemechanged)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2067>

##### once()

“once()” 章节

    once<T>(event, handler): Promise<UnlistenFn>

监听此 webview 窗口上触发的仅一次性事件。

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

    import { WebviewWindow } from '@tauri-apps/api/webviewWindow';

    const unlisten = await WebviewWindow.getCurrent().once<null>('initialized', (event) => {

      console.log(`Webview initialized!`);

    });

    // you need to call unlisten if your handler goes out of scope e.g. the component is unmounted

    unlisten();

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`once`](/reference/javascript/api/namespacewindow/#once)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webviewWindow.ts#L190>

##### outerPosition()

名为“outerPosition()”的章节

    outerPosition(): Promise<PhysicalPosition>

窗口左上角相对于桌面左上角的位置。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`PhysicalPosition`](/reference/javascript/api/namespacedpi/#physicalposition)>

窗口的外部位置。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    const position = await getCurrentWindow().outerPosition();

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`outerPosition`](/reference/javascript/api/namespacewindow/#outerposition)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L553>

##### outerSize()

名为“outerSize()”的章节

    outerSize(): Promise<PhysicalSize>

整个窗口的物理尺寸。这些尺寸包括标题栏和边框。如果您不需要（通常是不需要的），请改用 inner_size。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`PhysicalSize`](/reference/javascript/api/namespacedpi/#physicalsize)>

窗口的外部尺寸。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    const size = await getCurrentWindow().outerSize();

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`outerSize`](/reference/javascript/api/namespacewindow/#outersize)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L590>

##### position()

名为“position()”的章节

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

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Webview`](/reference/javascript/api/namespacewebview/#webview).[`position`](/reference/javascript/api/namespacewebview/#position)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L398>

##### reparent()

名为“reparent()”的章节

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

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Webview`](/reference/javascript/api/namespacewebview/#webview).[`reparent`](/reference/javascript/api/namespacewebview/#reparent)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L572>

##### requestUserAttention()

名为“requestUserAttention()”的章节

    requestUserAttention(requestType): Promise<void>

请求用户对窗口的关注，如果应用程序已经获得焦点，此操作无效。请求用户关注的表现形式取决于平台，详见 `UserAttentionType`。

提供 `null` 将取消对用户关注的请求。当窗口接收到输入时，WM（窗口管理器）可能不会自动取消对用户关注的请求。

平台特定

  * **macOS：** `null` 无效。
  * **Linux：** 紧急级别具有相同的效果。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`requestType`| `null` | [`UserAttentionType`](/reference/javascript/api/namespacewindow/#userattentiontype)

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    await getCurrentWindow().requestUserAttention();

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`requestUserAttention`](/reference/javascript/api/namespacewindow/#requestuserattention)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L861>

##### scaleFactor()

名为“scaleFactor()”的章节

    scaleFactor(): Promise<number>

可用于将物理像素映射到逻辑像素的缩放因子。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`number`>

窗口的显示器缩放因子。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    const factor = await getCurrentWindow().scaleFactor();

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`scaleFactor`](/reference/javascript/api/namespacewindow/#scalefactor)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L521>

##### setAlwaysOnBottom()

名为“setAlwaysOnBottom()”的章节

    setAlwaysOnBottom(alwaysOnBottom): Promise<void>

窗口是否应始终位于其他窗口下方。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`alwaysOnBottom`| `布尔值 (boolean)`| 窗口是否应始终置于其他窗口下方。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    await getCurrentWindow().setAlwaysOnBottom(true);

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`setAlwaysOnBottom`](/reference/javascript/api/namespacewindow/#setalwaysonbottom)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1259>

##### setAlwaysOnTop()

名为“setAlwaysOnTop()”的章节

    setAlwaysOnTop(alwaysOnTop): Promise<void>

窗口是否应始终位于其他窗口之上。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`alwaysOnTop`| `布尔值 (boolean)`| 窗口是否应始终置于其他窗口上方。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    await getCurrentWindow().setAlwaysOnTop(true);

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`setAlwaysOnTop`](/reference/javascript/api/namespacewindow/#setalwaysontop)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1241>

##### setAutoResize()

名为“setAutoResize()”的章节

    setAutoResize(autoResize): Promise<void>

设置当父窗口调整大小时，webview 是否应自动调整大小和位置。

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

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Webview`](/reference/javascript/api/namespacewebview/#webview).[`setAutoResize`](/reference/javascript/api/namespacewebview/#setautoresize)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L506>

##### setBackgroundColor()

“setBackgroundColor()” 章节

    setBackgroundColor(color): Promise<void>

设置窗口和 webview 的背景颜色。

平台特定

  * **Android / iOS：** 窗口层不支持。
  * **macOS / iOS：** Webview 层未实现。
  * **Windows** :
    * 窗口层的 alpha 通道被忽略。
    * 在 Windows 7 上，webview 层的 alpha 通道被忽略。
    * 在 Windows 8 及更高版本上，如果 alpha 通道不为 `0`，它将被忽略。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`颜色`| [`Color`](/reference/javascript/api/namespacewebview/#color)

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 始于

名为“起始版本”的部分

2.1.0

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`setBackgroundColor`](/reference/javascript/api/namespacewindow/#setbackgroundcolor)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webviewWindow.ts#L222>

##### setBadgeCount()

名为“setBadgeCount()”的章节

    setBadgeCount(count?): Promise<void>

设置徽章计数。它是应用程序范围的，不特定于此窗口。

平台特定

  * **Windows：** 不支持。请改用 @{linkcode Window.setOverlayIcon}。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`count`?| `数字`| 徽章计数。使用 `undefined` 移除徽章。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    await getCurrentWindow().setBadgeCount(5);

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`setBadgeCount`](/reference/javascript/api/namespacewindow/#setbadgecount)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1685>

##### setBadgeLabel()

名为“setBadgeLabel()”的章节

    setBadgeLabel(label?): Promise<void>

设置徽章标签 **仅限 macOS** 。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`标签`?| `string`| 徽章标签。使用 `undefined` 移除徽章。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    await getCurrentWindow().setBadgeLabel("Hello");

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`setBadgeLabel`](/reference/javascript/api/namespacewindow/#setbadgelabel)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1704>

##### setClosable()

名为“setClosable()”的章节

    setClosable(closable): Promise<void>

设置窗口的原生关闭按钮是否启用。

平台特定

  * **Linux：** GTK+ 将尽力说服窗口管理器不显示关闭按钮。根据系统的不同，在已经可见的窗口上调用此函数可能无效。
  * **iOS / Android:** 不支持。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`可关闭`| `布尔值 (boolean)`

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    await getCurrentWindow().setClosable(false);

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`setClosable`](/reference/javascript/api/namespacewindow/#setclosable)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L995>

##### setContentProtected()

名为“setContentProtected()”的章节

    setContentProtected(protected_): Promise<void>

防止窗口内容被其他应用程序捕获。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`protected_`| `布尔值 (boolean)`

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    await getCurrentWindow().setContentProtected(true);

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`setContentProtected`](/reference/javascript/api/namespacewindow/#setcontentprotected)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1276>

##### setCursorGrab()

名为“setCursorGrab()”的章节

    setCursorGrab(grab): Promise<void>

锁定光标，防止其离开窗口。

不保证光标会被隐藏。如果您希望隐藏光标，请自行处理。

平台特定

  * **Linux：** 不支持。
  * **macOS：** 这会将光标锁定在固定位置，视觉上看起来比较尴尬。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`grab`| `布尔值 (boolean)`| `true` 锁定光标，`false` 释放它。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    await getCurrentWindow().setCursorGrab(true);

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`setCursorGrab`](/reference/javascript/api/namespacewindow/#setcursorgrab)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1532>

##### setCursorIcon()

名为“setCursorIcon()”的章节

    setCursorIcon(icon): Promise<void>

修改窗口的光标图标。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`icon`| [`CursorIcon`](/reference/javascript/api/namespacewindow/#cursoricon)| 新的光标图标。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    await getCurrentWindow().setCursorIcon('help');

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`setCursorIcon`](/reference/javascript/api/namespacewindow/#setcursoricon)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1574>

##### setCursorPosition()

名为“setCursorPosition()”的章节

    setCursorPosition(position): Promise<void>

更改窗口坐标系中光标的位置。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`位置`| [`LogicalPosition`](/reference/javascript/api/namespacedpi/#logicalposition) | [`PhysicalPosition`](/reference/javascript/api/namespacedpi/#physicalposition) | [`Position`](/reference/javascript/api/namespacedpi/#position)| 新的光标位置。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow, LogicalPosition } from '@tauri-apps/api/window';

    await getCurrentWindow().setCursorPosition(new LogicalPosition(600, 300));

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`setCursorPosition`](/reference/javascript/api/namespacewindow/#setcursorposition)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1608>

##### setCursorVisible()

名为“setCursorVisible()”的章节

    setCursorVisible(visible): Promise<void>

修改光标的可见性。

平台特定

  * **Windows：** 光标仅在窗口范围内隐藏。
  * **macOS：** 只要窗口具有输入焦点，即使光标在窗口外，光标也会被隐藏。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`visible`| `布尔值 (boolean)`| 如果为 `false`，将隐藏光标。如果为 `true`，将显示光标。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    await getCurrentWindow().setCursorVisible(false);

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`setCursorVisible`](/reference/javascript/api/namespacewindow/#setcursorvisible)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1556>

##### setDecorations()

名为“setDecorations()”的章节

    setDecorations(decorations): Promise<void>

窗口是否应该有边框和标题栏。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`装饰`| `布尔值 (boolean)`| 窗口是否应该有边框和标题栏。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    await getCurrentWindow().setDecorations(false);

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`setDecorations`](/reference/javascript/api/namespacewindow/#setdecorations)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1177>

##### setEffects()

名为“setEffects()”的章节

    setEffects(effects): Promise<void>

设置窗口特效。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`效果`| [`Effects`](/reference/javascript/api/namespacewindow/#effects)

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`setEffects`](/reference/javascript/api/namespacewindow/#seteffects)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1213>

##### setEnabled()

“setEnabled()” 章节

    setEnabled(enabled): Promise<void>

启用或禁用窗口。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`enabled`| `布尔值 (boolean)`

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    await getCurrentWindow().setEnabled(false);

###### 始于

名为“起始版本”的部分

2.0.0

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`setEnabled`](/reference/javascript/api/namespacewindow/#setenabled)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L908>

##### setFocus()

“setFocus()” 章节

    setFocus(): Promise<void>

将 webview 置于最前并获取焦点。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWebview } from '@tauri-apps/api/webview';

    await getCurrentWebview().setFocus();

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`setFocus`](/reference/javascript/api/namespacewindow/#setfocus)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L490>

##### setFocusable()

名为“setFocusable()”的章节

    setFocusable(focusable): Promise<void>

设置窗口是否可以获得焦点。

平台特定

  * **macOS** ：如果窗口已经处于焦点状态，在调用 `set_focusable(false)` 后将无法取消其焦点。在这种情况下，您可以考虑调用 [Window.setFocus](/reference/javascript/api/namespacewindow/#setfocus)，但这会将窗口移动到背景，即在 z 轴顺序上处于最底层。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`可聚焦`| `布尔值 (boolean)`| 窗口是否可以获得焦点。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    await getCurrentWindow().setFocusable(true);

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`setFocusable`](/reference/javascript/api/namespacewindow/#setfocusable)

**源码** ：<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1457>

##### setFullscreen()

名为“setFullscreen()”的章节

    setFullscreen(fullscreen): Promise<void>

设置窗口的全屏状态。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`全屏`| `布尔值 (boolean)`| 窗口是否应进入全屏模式。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    await getCurrentWindow().setFullscreen(true);

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`setFullscreen`](/reference/javascript/api/namespacewindow/#setfullscreen)

**源码** ：<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1401>

##### setIcon()

“setIcon()” 章节

    setIcon(icon): Promise<void>

设置窗口图标。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`icon`| | `string` | [`Uint8Array`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array) | `number`[] | [`ArrayBuffer`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer) | [`Image`](/reference/javascript/api/namespaceimage/#image)| 图标字节或图标文件的路径。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    await getCurrentWindow().setIcon('/tauri/awesome.png');

请注意，您可能需要 `image-ico` 或 `image-png` Cargo 功能才能使用此 API。要启用它，请更改您的 Cargo.toml 文件

    [dependencies]

    tauri = { version = "...", features = ["...", "image-png"] }

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`setIcon`](/reference/javascript/api/namespacewindow/#seticon)

**源码** ：<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1482>

##### setIgnoreCursorEvents()

名为“setIgnoreCursorEvents()”的章节

    setIgnoreCursorEvents(ignore): Promise<void>

更改光标事件的行为。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`ignore`| `布尔值 (boolean)`| `true` 则忽略光标事件；`false` 则照常处理。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    await getCurrentWindow().setIgnoreCursorEvents(true);

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`setIgnoreCursorEvents`](/reference/javascript/api/namespacewindow/#setignorecursorevents)

**源码** ：<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1629>

##### setMaxSize()

名为“setMaxSize()”的章节

    setMaxSize(size): Promise<void>

设置窗口最大内部尺寸。如果 `size` 参数未定义，则取消约束。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`尺寸`| | `undefined` | `null` | [`LogicalSize`](/reference/javascript/api/namespacedpi/#logicalsize) | [`PhysicalSize`](/reference/javascript/api/namespacedpi/#physicalsize) | [`Size`](/reference/javascript/api/namespacedpi/#size)| 逻辑或物理内部尺寸，或 `null` 以取消约束。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow, LogicalSize } from '@tauri-apps/api/window';

    await getCurrentWindow().setMaxSize(new LogicalSize(600, 500));

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`setMaxSize`](/reference/javascript/api/namespacewindow/#setmaxsize)

**源码** ：<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1332>

##### setMaximizable()

名为“setMaximizable()”的章节

    setMaximizable(maximizable): Promise<void>

设置窗口的原生最大化按钮是否启用。如果设置了不可调整大小（resizable 为 false），则此设置将被忽略。

平台特定

  * **macOS：** 禁用窗口标题栏中的“缩放”按钮，该按钮也用于进入全屏模式。
  * **Linux / iOS / Android：** 不支持。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`可最大化`| `布尔值 (boolean)`

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    await getCurrentWindow().setMaximizable(false);

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`setMaximizable`](/reference/javascript/api/namespacewindow/#setmaximizable)

**源码** ：<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L950>

##### setMinSize()

名为“setMinSize()”的章节

    setMinSize(size): Promise<void>

设置窗口最小内部尺寸。如果未提供 `size` 参数，则取消约束。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`尺寸`| | `undefined` | `null` | [`LogicalSize`](/reference/javascript/api/namespacedpi/#logicalsize) | [`PhysicalSize`](/reference/javascript/api/namespacedpi/#physicalsize) | [`Size`](/reference/javascript/api/namespacedpi/#size)| 逻辑或物理内部尺寸，或 `null` 以取消约束。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow, PhysicalSize } from '@tauri-apps/api/window';

    await getCurrentWindow().setMinSize(new PhysicalSize(600, 500));

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`setMinSize`](/reference/javascript/api/namespacewindow/#setminsize)

**源码** ：<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1312>

##### setMinimizable()

名为“setMinimizable()”的章节

    setMinimizable(minimizable): Promise<void>

设置窗口的原生最小化按钮是否启用。

平台特定

  * **Linux / iOS / Android：** 不支持。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`可最小化`| `布尔值 (boolean)`

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    await getCurrentWindow().setMinimizable(false);

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`setMinimizable`](/reference/javascript/api/namespacewindow/#setminimizable)

**源码** ：<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L972>

##### setOverlayIcon()

名为“setOverlayIcon()”的章节

    setOverlayIcon(icon?): Promise<void>

设置覆盖图标。**仅限 Windows** ：每个窗口都可以设置覆盖图标。

请注意，您可能需要 `image-ico` 或 `image-png` Cargo 功能才能使用此 API。要启用它，请更改您的 Cargo.toml 文件

    [dependencies]

    tauri = { version = "...", features = ["...", "image-png"] }

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`icon`?| | `string` | [`Uint8Array`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array) | `number`[] | [`ArrayBuffer`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer) | [`Image`](/reference/javascript/api/namespaceimage/#image)| 图标字节或图标文件的路径。使用 `undefined` 来移除覆盖图标。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    await getCurrentWindow().setOverlayIcon("/tauri/awesome.png");

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`setOverlayIcon`](/reference/javascript/api/namespacewindow/#setoverlayicon)

**源码** ：<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1733>

##### setPosition()

“setPosition()” 章节

    setPosition(position): Promise<void>

设置 WebView 的位置。

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

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`setPosition`](/reference/javascript/api/namespacewindow/#setposition)

**源码** ：<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L471>

##### setProgressBar()

名为“setProgressBar()”的章节

    setProgressBar(state): Promise<void>

设置任务栏进度状态。

平台特定

  * **Linux / macOS** ：进度条是整个应用程序范围的，不针对特定窗口。
  * **Linux** ：仅支持带有 `libunity` 的桌面环境（例如 GNOME）。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`state`| [`ProgressBarState`](/reference/javascript/api/namespacewindow/#progressbarstate)

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow, ProgressBarStatus } from '@tauri-apps/api/window';

    await getCurrentWindow().setProgressBar({

      status: ProgressBarStatus.Normal,

      progress: 50,

    });

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`setProgressBar`](/reference/javascript/api/namespacewindow/#setprogressbar)

**源码** ：<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1761>

##### setResizable()

名为“setResizable()”的章节

    setResizable(resizable): Promise<void>

更新窗口的可调整大小标志。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`可调整大小`| `布尔值 (boolean)`

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    await getCurrentWindow().setResizable(false);

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`setResizable`](/reference/javascript/api/namespacewindow/#setresizable)

**源码** ：<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L889>

##### setShadow()

名为“setShadow()”的章节

    setShadow(enable): Promise<void>

窗口是否应具有阴影。

平台特定

  * **Windows**
    * `false` 对装饰性窗口没有影响，阴影始终处于开启状态。
    * `true` 将使无装饰窗口有 1 像素的白色边框，在 Windows 11 上，它将有圆角。
  * **Linux：** 不支持。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`启用`| `布尔值 (boolean)`

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    await getCurrentWindow().setShadow(false);

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`setShadow`](/reference/javascript/api/namespacewindow/#setshadow)

**源码** ：<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1203>

##### setSimpleFullscreen()

名为“setSimpleFullscreen()”的章节

    setSimpleFullscreen(fullscreen): Promise<void>

在 macOS 上，切换一种不需要新的 macOS 空间的“简单”全屏模式。返回一个布尔值，指示转换是否成功（如果窗口已经处于原生全屏状态，此操作将无效）。这是 macOS 在 Lion 版本之前实现全屏的方式。它允许用户拥有一个全屏窗口，而无需使用另一个空间或接管整个监视器。

在其他平台上，这与 [Window.setFullscreen](/reference/javascript/api/namespacewindow/#setfullscreen) 相同。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`全屏`| `布尔值 (boolean)`| 窗口是否应进入简单全屏模式。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`setSimpleFullscreen`](/reference/javascript/api/namespacewindow/#setsimplefullscreen)

**源码** ：<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1417>

##### setSize()

“setSize()” 章节

    setSize(size): Promise<void>

调整 WebView 的大小。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`尺寸`| [`LogicalSize`](/reference/javascript/api/namespacedpi/#logicalsize) | [`PhysicalSize`](/reference/javascript/api/namespacedpi/#physicalsize) | [`Size`](/reference/javascript/api/namespacedpi/#size)| 逻辑或物理尺寸。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrent, LogicalSize } from '@tauri-apps/api/webview';

    await getCurrentWebview().setSize(new LogicalSize(600, 500));

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`setSize`](/reference/javascript/api/namespacewindow/#setsize)

**源码** ：<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L453>

##### setSizeConstraints()

名为“setSizeConstraints()”的章节

    setSizeConstraints(constraints): Promise<void>

设置窗口内部尺寸约束。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`constraints`| `undefined` | `null` | [`WindowSizeConstraints`](/reference/javascript/api/namespacewindow/#windowsizeconstraints)| 逻辑或物理内部尺寸，或 `null` 以取消约束。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    await getCurrentWindow().setSizeConstraints({ minWidth: 300 });

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`setSizeConstraints`](/reference/javascript/api/namespacewindow/#setsizeconstraints)

**源码** ：<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1352>

##### setSkipTaskbar()

名为“setSkipTaskbar()”的章节

    setSkipTaskbar(skip): Promise<void>

是否应从任务栏中隐藏窗口图标。

平台特定

  * **macOS** ：不支持。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`跳过`| `布尔值 (boolean)`| true 隐藏窗口图标，false 显示它。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    await getCurrentWindow().setSkipTaskbar(true);

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`setSkipTaskbar`](/reference/javascript/api/namespacewindow/#setskiptaskbar)

**源码** ：<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1506>

##### setTheme()

名为“setTheme()”的章节

    setTheme(theme?): Promise<void>

设置窗口主题，传入 `null` 或 `undefined` 以跟随系统主题

平台特定

  * **Linux / macOS** ：主题是应用程序范围的，不针对特定窗口。
  * **iOS / Android:** 不支持。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`主题`?| `null` | [`Theme`](/reference/javascript/api/namespacewindow/#theme-2)

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

###### 始于

名为“起始版本”的部分

2.0.0

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`setTheme`](/reference/javascript/api/namespacewindow/#settheme)

**源码** ：<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1806>

##### setTitle()

名为“setTitle()”的章节

    setTitle(title): Promise<void>

设置窗口标题。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`标题`| `string`| 新标题

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    await getCurrentWindow().setTitle('Tauri');

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`setTitle`](/reference/javascript/api/namespacewindow/#settitle)

**源码** ：<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1013>

##### setTitleBarStyle()

名为“setTitleBarStyle()”的章节

    setTitleBarStyle(style): Promise<void>

设置标题栏样式。**仅限 macOS** 。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`style`| [`TitleBarStyle`](/reference/javascript/api/namespacewindow/#titlebarstyle-1)

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

###### 始于

名为“起始版本”的部分

2.0.0

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`setTitleBarStyle`](/reference/javascript/api/namespacewindow/#settitlebarstyle)

**源码** ：<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1789>

##### setVisibleOnAllWorkspaces()

名为“setVisibleOnAllWorkspaces()”的章节

    setVisibleOnAllWorkspaces(visible): Promise<void>

设置窗口是否应在所有工作区或虚拟桌面上可见。

平台特定

  * **Windows / iOS / Android：** 不支持。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`visible`| `布尔值 (boolean)`

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

###### 始于

名为“起始版本”的部分

2.0.0

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`setVisibleOnAllWorkspaces`](/reference/javascript/api/namespacewindow/#setvisibleonallworkspaces)

**源码** ：<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1777>

##### setZoom()

名为“setZoom()”的章节

    setZoom(scaleFactor): Promise<void>

设置 WebView 缩放级别。

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

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Webview`](/reference/javascript/api/namespacewebview/#webview).[`setZoom`](/reference/javascript/api/namespacewebview/#setzoom)

**源码** ：<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L555>

##### show()

名为“show()”的章节

    show(): Promise<void>

显示 WebView。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWebview } from '@tauri-apps/api/webview';

    await getCurrentWebview().show();

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`show`](/reference/javascript/api/namespacewindow/#show)

**源码** ：<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L539>

##### size()

名为“size()”的章节

    size(): Promise<PhysicalSize>

WebView 客户区的物理尺寸。客户区是 WebView 的内容区域，不包含标题栏和边框。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`PhysicalSize`](/reference/javascript/api/namespacedpi/#physicalsize)>

WebView 的尺寸。

###### 示例

标题为“Example”的章节

    import { getCurrentWebview } from '@tauri-apps/api/webview';

    const size = await getCurrentWebview().size();

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Webview`](/reference/javascript/api/namespacewebview/#webview).[`size`](/reference/javascript/api/namespacewebview/#size)

**源码** ：<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts#L415>

##### startDragging()

名为“startDragging()”的章节

    startDragging(): Promise<void>

开始拖动窗口。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    await getCurrentWindow().startDragging();

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`startDragging`](/reference/javascript/api/namespacewindow/#startdragging)

**源码** ：<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1646>

##### startResizeDragging()

名为“startResizeDragging()”的章节

    startResizeDragging(direction): Promise<void>

开始调整窗口大小的拖动操作。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`direction`| `ResizeDirection`

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    await getCurrentWindow().startResizeDragging();

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`startResizeDragging`](/reference/javascript/api/namespacewindow/#startresizedragging)

**源码** ：<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1662>

##### theme()

名为“theme()”的章节

    theme(): Promise<null | Theme>

获取窗口当前主题。

平台特定

  * **macOS** ：主题是在 macOS 10.14 上引入的。在 macOS 10.13 及更低版本上返回 `light`。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`null` | [`Theme`](/reference/javascript/api/namespacewindow/#theme-2)>

窗口主题。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    const theme = await getCurrentWindow().theme();

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`theme`](/reference/javascript/api/namespacewindow/#theme)

**源码** ：<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L801>

##### title()

名为“title()”的章节

    title(): Promise<string>

获取窗口当前标题。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`string`>

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    const title = await getCurrentWindow().title();

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`title`](/reference/javascript/api/namespacewindow/#title)

**源码** ：<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L780>

##### toggleMaximize()

名为“toggleMaximize()”的章节

    toggleMaximize(): Promise<void>

切换窗口最大化状态。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    await getCurrentWindow().toggleMaximize();

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`toggleMaximize`](/reference/javascript/api/namespacewindow/#togglemaximize)

**源码** ：<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1062>

##### unmaximize()

名为“unmaximize()”的章节

    unmaximize(): Promise<void>

取消窗口最大化。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    await getCurrentWindow().unmaximize();

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`unmaximize`](/reference/javascript/api/namespacewindow/#unmaximize)

**源码** ：<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1046>

##### unminimize()

名为“unminimize()”的章节

    unminimize(): Promise<void>

取消窗口最小化。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    await getCurrentWindow().unminimize();

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`unminimize`](/reference/javascript/api/namespacewindow/#unminimize)

**源码** ：<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1094>

##### getAll()

名为“getAll()”的章节

    static getAll(): Promise<WebviewWindow[]>

获取所有可用 WebView 的 `Webview` 实例列表。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`WebviewWindow`](/reference/javascript/api/namespacewebviewwindow/#webviewwindow)[]>

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`getAll`](/reference/javascript/api/namespacewindow/#getall)

**源码** ：<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webviewWindow.ts#L132>

##### getByLabel()

名为“getByLabel()”的章节

    static getByLabel(label): Promise<null | WebviewWindow>

获取与给定标签关联的 WebView 的实例。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`标签`| `string`| WebView 标签。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`null` | [`WebviewWindow`](/reference/javascript/api/namespacewebviewwindow/#webviewwindow)>

与 WebView 通信的 Webview 实例，如果 WebView 不存在，则返回 null。

###### 示例

标题为“Example”的章节

    import { WebviewWindow } from '@tauri-apps/api/webviewWindow';

    const mainWebview = WebviewWindow.getByLabel('main');

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`getByLabel`](/reference/javascript/api/namespacewindow/#getbylabel)

**源码** ：<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webviewWindow.ts#L112>

##### getCurrent()

名为“getCurrent()”的章节

    static getCurrent(): WebviewWindow

获取当前 WebView 的 `Webview` 实例。

###### 返回

名为“返回值”的部分

[`WebviewWindow`](/reference/javascript/api/namespacewebviewwindow/#webviewwindow)

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Window`](/reference/javascript/api/namespacewindow/#window).[`getCurrent`](/reference/javascript/api/namespacewindow/#getcurrent)

**源码** ：<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webviewWindow.ts#L125>

## 函数

名为“函数”的部分

### getAllWebviewWindows()

名为“getAllWebviewWindows()”的章节

    function getAllWebviewWindows(): Promise<WebviewWindow[]>

获取所有可用 WebView 窗口的 `Webview` 实例列表。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`WebviewWindow`](/reference/javascript/api/namespacewebviewwindow/#webviewwindow)[]>

#### 始于

名为“起始版本”的部分

2.0.0

**源码** ：<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webviewWindow.ts#L34>

* * *

### getCurrentWebviewWindow()

名为“getCurrentWebviewWindow()”的章节

    function getCurrentWebviewWindow(): WebviewWindow

获取当前 WebView 窗口的 `Webview` 实例。

#### 返回

名为“返回值”的部分

[`WebviewWindow`](/reference/javascript/api/namespacewebviewwindow/#webviewwindow)

#### 始于

名为“起始版本”的部分

2.0.0

**源码** ：<https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webviewWindow.ts#L23>