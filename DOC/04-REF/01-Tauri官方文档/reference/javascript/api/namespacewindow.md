# window

_Source: https://v2.tauri.org.cn/reference/javascript/api/namespacewindow/_

提供用于创建窗口、与其他窗口通信以及操作当前窗口的 API。

#### 窗口事件

标题为“窗口事件”的章节

可以使用 [Window.listen](/reference/javascript/api/namespacewindow/#listen) 监听事件

    import { getCurrentWindow } from "@tauri-apps/api/window";

    getCurrentWindow().listen("my-window-event", ({ event, payload }) => { });

## 参考

标题为“参考”的章节

### Color

标题为“Color”的章节

重新导出 [Color](/reference/javascript/api/namespacewebview/#color)

### DragDropEvent

标题为“DragDropEvent”的章节

重新导出 [DragDropEvent](/reference/javascript/api/namespacewebview/#dragdropevent)

### LogicalPosition

标题为“LogicalPosition”的章节

重新导出 [LogicalPosition](/reference/javascript/api/namespacedpi/#logicalposition)

### LogicalSize

标题为“LogicalSize”的章节

重新导出 [LogicalSize](/reference/javascript/api/namespacedpi/#logicalsize)

### PhysicalPosition

标题为“PhysicalPosition”的章节

重新导出 [PhysicalPosition](/reference/javascript/api/namespacedpi/#physicalposition)

### PhysicalSize

标题为“PhysicalSize”的章节

重新导出 [PhysicalSize](/reference/javascript/api/namespacedpi/#physicalsize)

## 枚举

标题为“枚举”的章节

### BackgroundThrottlingPolicy

标题为“BackgroundThrottlingPolicy”的章节

后台节流策略

#### 始于

名为“起始版本”的部分

2.0.0

#### 枚举成员

标题为“枚举成员”的章节

##### Disabled

标题为“Disabled”的章节

    Disabled: "disabled";

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2091>

##### Suspend

标题为“Suspend”的章节

    Suspend: "suspend";

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2093>

##### Throttle

标题为“Throttle”的章节

    Throttle: "throttle";

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2092>

* * *

### Effect

标题为“Effect”的章节

特定于平台的窗口效果

#### 始于

名为“起始版本”的部分

2.0.0

#### 枚举成员

标题为“枚举成员”的章节

##### Acrylic

标题为“Acrylic”的章节

    Acrylic: "acrylic";

**Windows 10/11**

#### 备注

标题为“注意”的章节

在 Windows 10 v1903+ 和 Windows 11 build 22000 上调整大小/拖动窗口时，此效果性能不佳。

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2230>

##### ~~AppearanceBased~~

标题为“AppearanceBased”的章节

    AppearanceBased: "appearanceBased";

适用于视图 effectiveAppearance 的默认材质。**macOS 10.14-**

###### 已弃用

标题为“已弃用”的章节

自 macOS 10.14 起。您应该改用合适的语义材质。

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2130>

##### Blur

标题为“Blur”的章节

    Blur: "blur";

**仅限 Windows 7/10/11(22H1)**

#### 备注

标题为“注意”的章节

在 Windows 11 build 22621 上调整大小/拖动窗口时，此效果性能不佳。

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2222>

##### ContentBackground

标题为“ContentBackground”的章节

    ContentBackground: "contentBackground";

**macOS 10.14+**

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2202>

##### ~~Dark~~

标题为“Dark”的章节

    Dark: "dark";

**macOS 10.14-**

###### 已弃用

标题为“已弃用”的章节

自 macOS 10.14 起。请改用语义材质。

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2142>

##### FullScreenUI

标题为“FullScreenUI”的章节

    FullScreenUI: "fullScreenUI";

**macOS 10.14+**

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2194>

##### HeaderView

标题为“HeaderView”的章节

    HeaderView: "headerView";

**macOS 10.14+**

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2178>

##### HudWindow

标题为“HudWindow”的章节

    HudWindow: "hudWindow";

**macOS 10.14+**

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2190>

##### ~~Light~~

标题为“Light”的章节

    Light: "light";

**macOS 10.14-**

###### 已弃用

标题为“已弃用”的章节

自 macOS 10.14 起。请改用语义材质。

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2136>

##### ~~MediumLight~~

标题为“MediumLight”的章节

    MediumLight: "mediumLight";

**macOS 10.14-**

###### 已弃用

标题为“已弃用”的章节

自 macOS 10.14 起。请改用语义材质。

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2148>

##### 菜单

标题为“Menu”的章节

    Menu: "menu";

**macOS 10.11+**

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2166>

##### Mica

标题为“Mica”的章节

    Mica: "mica";

**仅限 Windows 11**

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2214>

##### Popover

标题为“Popover”的章节

    Popover: "popover";

**macOS 10.11+**

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2170>

##### Selection

标题为“Selection”的章节

    Selection: "selection";

**macOS 10.10+**

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2162>

##### Sheet

标题为“Sheet”的章节

    Sheet: "sheet";

**macOS 10.14+**

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2182>

##### Sidebar

标题为“Sidebar”的章节

    Sidebar: "sidebar";

**macOS 10.11+**

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2174>

##### Tabbed

标题为“Tabbed”的章节

    Tabbed: "tabbed";

符合系统深色偏好的标签页效果 **仅限 Windows 11**

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2234>

##### TabbedDark

标题为“TabbedDark”的章节

    TabbedDark: "tabbedDark";

深色模式下的标签页效果，仅当系统启用深色模式时生效 **仅限 Windows 11**

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2238>

##### TabbedLight

标题为“TabbedLight”的章节

    TabbedLight: "tabbedLight";

浅色模式下的标签页效果 **仅限 Windows 11**

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2242>

##### Titlebar

标题为“Titlebar”的章节

    Titlebar: "titlebar";

**macOS 10.10+**

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2158>

##### Tooltip

标题为“Tooltip”的章节

    Tooltip: "tooltip";

**macOS 10.14+**

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2198>

##### ~~UltraDark~~

标题为“UltraDark”的章节

    UltraDark: "ultraDark";

**macOS 10.14-**

###### 已弃用

标题为“已弃用”的章节

自 macOS 10.14 起。请改用语义材质。

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2154>

##### UnderPageBackground

标题为“UnderPageBackground”的章节

    UnderPageBackground: "underPageBackground";

**macOS 10.14+**

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2210>

##### UnderWindowBackground

标题为“UnderWindowBackground”的章节

    UnderWindowBackground: "underWindowBackground";

**macOS 10.14+**

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2206>

##### WindowBackground

标题为“WindowBackground”的章节

    WindowBackground: "windowBackground";

**macOS 10.14+**

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2186>

* * *

### EffectState

标题为“EffectState”的章节

窗口效果状态 **仅限 macOS**

#### 查看

标题为“参见”的章节

<https://developer.apple.com/documentation/appkit/nsvisualeffectview/state>

#### 始于

名为“起始版本”的部分

2.0.0

#### 枚举成员

标题为“枚举成员”的章节

##### Active

标题为“Active”的章节

    Active: "active";

使窗口效果状态始终处于活动状态 **仅限 macOS**

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2260>

##### FollowsWindowActiveState

标题为“FollowsWindowActiveState”的章节

    FollowsWindowActiveState: "followsWindowActiveState";

使窗口效果状态跟随窗口的活动状态 **仅限 macOS**

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2256>

##### Inactive

标题为“Inactive”的章节

    Inactive: "inactive";

使窗口效果状态始终处于非活动状态 **仅限 macOS**

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2264>

* * *

### ProgressBarStatus

标题为“ProgressBarStatus”的章节

#### 枚举成员

标题为“枚举成员”的章节

##### Error

标题为“Error”的章节

    Error: "error";

错误状态。**在 Linux 上被视为 Normal**

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L190>

##### Indeterminate

标题为“Indeterminate”的章节

    Indeterminate: "indeterminate";

不确定状态。**在 Linux 和 macOS 上被视为 Normal**

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L182>

##### 无

标题为“None”的章节

    None: "none";

隐藏进度条。

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L174>

##### Normal

标题为“Normal”的章节

    Normal: "normal";

正常状态。

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L178>

##### Paused

标题为“Paused”的章节

    Paused: "paused";

暂停状态。**在 Linux 上被视为 Normal**

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L186>

* * *

### ScrollBarStyle

标题为“ScrollBarStyle”的章节

webview 中使用的滚动条样式。

## 平台特定

名为“平台特定”的章节

**Windows** ：此选项对于所有 webview 必须给定相同的值。

#### 始于

名为“起始版本”的部分

2.8.0

#### 枚举成员

标题为“枚举成员”的章节

##### 默认

标题为“Default”的章节

    Default: "default";

webview 的默认滚动条样式。

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2109>

##### FluentOverlay

标题为“FluentOverlay”的章节

    FluentOverlay: "fluentOverlay";

Fluent UI 样式覆盖滚动条。**仅限 Windows**

需要 WebView2 Runtime 125.0.2535.41 或更高版本，在旧版本上不起作用，请参阅 <https://learn.microsoft.com/en-us/microsoft-edge/webview2/release-notes/?tabs=dotnetcsharp#10253541>

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2116>

* * *

### UserAttentionType

标题为“UserAttentionType”的章节

在窗口上请求的关注类型。

#### 始于

名为“起始版本”的部分

1.0.0

#### 枚举成员

标题为“枚举成员”的章节

##### Critical

标题为“Critical”的章节

    Critical: 1;

平台特定

  * **macOS：** 弹跳停靠栏图标，直到应用程序获得焦点。
  * **Windows：** 闪烁窗口和任务栏按钮，直到应用程序获得焦点。

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L99>

##### Informational

标题为“Informational”的章节

    Informational: 2;

平台特定

  * **macOS：** 弹跳停靠栏图标一次。
  * **Windows：** 闪烁任务栏按钮，直到应用程序获得焦点。

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L105>

## 类

名为“类”的部分

### CloseRequestedEvent

标题为“CloseRequestedEvent”的章节

#### 构造函数

名为“构造函数”的部分

##### new CloseRequestedEvent()

标题为“new CloseRequestedEvent()”的章节

    new CloseRequestedEvent(event): CloseRequestedEvent

###### 参数

名为“参数”的部分

参数| 类型
---|---
`event`| [`Event`](/reference/javascript/api/namespaceevent/#eventt)<`unknown`>

###### 返回

名为“返回值”的部分

[`CloseRequestedEvent`](/reference/javascript/api/namespacewindow/#closerequestedevent)

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L115>

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
`event`| [`EventName`](/reference/javascript/api/namespaceevent/#eventname)| 事件名称| **来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L110>
`id`| `数字`| 用于取消监听的事件标识符| **来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L112>

#### 方法

名为“方法”的部分

##### isPreventDefault()

标题为“isPreventDefault()”的章节

    isPreventDefault(): boolean

###### 返回

名为“返回值”的部分

`布尔值 (boolean)`

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L124>

##### preventDefault()

标题为“preventDefault()”的章节

    preventDefault(): void

###### 返回

名为“返回值”的部分

`空`

**来源** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L120>

* * *

### 窗口

标题为“Window”的章节

创建新窗口或获取现有窗口的句柄。

窗口通过 _标签 (label)_ 进行标识，这是一个可用于后续引用窗口的唯一标识符。它只能包含字母数字字符 `a-zA-Z` 以及以下特殊字符：`-`、`/`、`:` 和 `_`。

#### 示例

标题为“Example”的章节

    import { Window } from "@tauri-apps/api/window"

    const appWindow = new Window('theUniqueLabel');

    appWindow.once('tauri://created', function () {

     // window successfully created

    });

    appWindow.once('tauri://error', function (e) {

     // an error happened creating the window

    });

    // emit an event to the backend

    await appWindow.emit("some-event", "data");

    // listen to an event from the backend

    const unlisten = await appWindow.listen("event-name", e => {});

    unlisten();

#### 始于

名为“起始版本”的部分

2.0.0

#### 扩展自

名为“扩展”的部分

  * [`WebviewWindow`](/reference/javascript/api/namespacewebviewwindow/#webviewwindow)

#### 构造函数

名为“构造函数”的部分

##### new Window()

“new Window()” 章节

    new Window(label, options): Window

创建一个新的窗口。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`标签`| `string`| 唯一的窗口标签。必须是字母数字组合：`a-zA-Z-/:_`。
`选项`| [`WindowOptions`](/reference/javascript/api/namespacewindow/#windowoptions)| -

###### 返回

名为“返回值”的部分

[`窗口`](/reference/javascript/api/namespacewindow/#window)

用于与窗口进行通信的 [Window](/reference/javascript/api/namespacewindow/#window) 实例。

###### 示例

标题为“Example”的章节

    import { Window } from '@tauri-apps/api/window';

    const appWindow = new Window('my-label');

    appWindow.once('tauri://created', function () {

     // window successfully created

    });

    appWindow.once('tauri://error', function (e) {

     // an error happened creating the window

    });

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L298>

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
`label`| `string`| 窗口标签。这是窗口的唯一标识符，可用于后续引用该窗口。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L276>
`listeners`| [`Record`](https://typescript.net.cn/docs/handbook/utility-types.html#recordkeys-type)<`string`, [`EventCallback`](/reference/javascript/api/namespaceevent/#eventcallbackt)<`any`>[]>| 本地事件监听器。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L279>

#### 方法

名为“方法”的部分

##### center()

“center()” 章节

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

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L835>

##### clearEffects()

“clearEffects()” 章节

    clearEffects(): Promise<void>

如果可能，清除所有已应用的特效。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1223>

##### close()

名为“close()”的部分

    close(): Promise<void>

关闭窗口。

请注意，这会触发 closeRequested 事件，因此您可以拦截它。若要强制关闭窗口，请使用 [Window.destroy](/reference/javascript/api/namespacewindow/#destroy)。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    await getCurrentWindow().close();

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1144>

##### destroy()

“destroy()” 章节

    destroy(): Promise<void>

销毁窗口。其行为类似于 [Window.close](/reference/javascript/api/namespacewindow/#close)，但会强制关闭窗口，而不是发出 closeRequested 事件。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    await getCurrentWindow().destroy();

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

    import { getCurrentWindow } from '@tauri-apps/api/window';

    await getCurrentWindow().emit('window-loaded', { loggedIn: true, token: 'authToken' });

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L449>

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

    import { getCurrentWindow } from '@tauri-apps/api/window';

    await getCurrentWindow().emit('main', 'window-loaded', { loggedIn: true, token: 'authToken' });

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L476>

##### hide()

“hide()” 章节

    hide(): Promise<void>

将窗口可见性设置为 false。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    await getCurrentWindow().hide();

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1126>

##### innerPosition()

“innerPosition()” 章节

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

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L537>

##### innerSize()

“innerSize()” 章节

    innerSize(): Promise<PhysicalSize>

窗口客户区的物理尺寸。客户区是指窗口的内容区域，不包含标题栏和边框。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`PhysicalSize`](/reference/javascript/api/namespacedpi/#physicalsize)>

窗口的内部尺寸。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    const size = await getCurrentWindow().innerSize();

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L570>

##### isAlwaysOnTop()

“isAlwaysOnTop()” 章节

    isAlwaysOnTop(): Promise<boolean>

窗口是否被配置为始终置顶于其他窗口之上。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`boolean`>

窗口是否可见。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    const alwaysOnTop = await getCurrentWindow().isAlwaysOnTop();

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L817>

##### isClosable()

“isClosable()” 章节

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

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L750>

##### isDecorated()

“isDecorated()” 章节

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

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L671>

##### isEnabled()

“isEnabled()” 章节

    isEnabled(): Promise<boolean>

窗口是否启用或禁用。

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

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L927>

##### isFocused()

“isFocused()” 章节

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

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L655>

##### isFullscreen()

“isFullscreen()” 章节

    isFullscreen(): Promise<boolean>

获取窗口当前的各种全屏状态。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`boolean`>

窗口是否处于全屏模式。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    const fullscreen = await getCurrentWindow().isFullscreen();

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L609>

##### isMaximizable()

“isMaximizable()” 章节

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

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L708>

##### isMaximized()

“isMaximized()” 章节

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

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L639>

##### isMinimizable()

“isMinimizable()” 章节

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

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L729>

##### isMinimized()

“isMinimized()” 章节

    isMinimized(): Promise<boolean>

获取窗口当前的最小化状态。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`boolean`>

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    const minimized = await getCurrentWindow().isMinimized();

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L623>

##### isResizable()

“isResizable()” 章节

    isResizable(): Promise<boolean>

获取窗口当前的可调整大小状态。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`boolean`>

窗口是否可以调整大小。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    const resizable = await getCurrentWindow().isResizable();

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L687>

##### isVisible()

“isVisible()” 章节

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

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L766>

##### listen()

“listen()” 章节

    listen<T>(event, handler): Promise<UnlistenFn>

监听此窗口上的已触发事件。

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

    import { getCurrentWindow } from '@tauri-apps/api/window';

    const unlisten = await getCurrentWindow().listen<string>('state-changed', (event) => {

      console.log(`Got error: ${payload}`);

    });

    // you need to call unlisten if your handler goes out of scope e.g. the component is unmounted

    unlisten();

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L387>

##### maximize()

“maximize()” 章节

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

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1030>

##### minimize()

“minimize()” 章节

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

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1078>

##### onCloseRequested()

“onCloseRequested()” 章节

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

    import { getCurrentWindow } from "@tauri-apps/api/webview";

    const unlisten = await getCurrentWindow().onDragDropEvent((event) => {

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

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1923>

##### onFocusChanged()

“onFocusChanged()” 章节

    onFocusChanged(handler): Promise<UnlistenFn>

监听窗口焦点变化。

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

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2001>

##### onMoved()

“onMoved()” 章节

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

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1856>

##### onResized()

“onResized()” 章节

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

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1832>

##### onScaleChanged()

“onScaleChanged()” 章节

    onScaleChanged(handler): Promise<UnlistenFn>

监听窗口缩放比例变化。当窗口的缩放因子发生变化时触发。以下用户操作可能导致 DPI 变化：

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

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2041>

##### onThemeChanged()

“onThemeChanged()” 章节

    onThemeChanged(handler): Promise<UnlistenFn>

监听系统主题变化。

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

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2067>

##### once()

“once()” 章节

    once<T>(event, handler): Promise<UnlistenFn>

仅在此窗口上监听一次已触发事件。

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

    import { getCurrentWindow } from '@tauri-apps/api/window';

    const unlisten = await getCurrentWindow().once<null>('initialized', (event) => {

      console.log(`Window initialized!`);

    });

    // you need to call unlisten if your handler goes out of scope e.g. the component is unmounted

    unlisten();

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L422>

##### outerPosition()

“outerPosition()” 章节

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

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L553>

##### outerSize()

“outerSize()” 章节

    outerSize(): Promise<PhysicalSize>

整个窗口的物理尺寸。这些维度包括标题栏和边框。如果您不想要这些（通常都不想要），请改用 inner_size。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`PhysicalSize`](/reference/javascript/api/namespacedpi/#physicalsize)>

窗口的外部尺寸。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    const size = await getCurrentWindow().outerSize();

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L590>

##### requestUserAttention()

“requestUserAttention()” 章节

    requestUserAttention(requestType): Promise<void>

请求窗口引起用户注意，如果应用程序已经获得焦点，则此操作无效。请求用户注意的表现方式取决于平台，详情请参阅 `UserAttentionType`。

提供 `null` 将取消用户注意请求。当窗口接收输入时，窗口管理器可能不会自动取消用户注意请求。

平台特定

  * **macOS:** `null` 无效。
  * **Linux:** 紧急级别具有相同的效果。

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

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L861>

##### scaleFactor()

“scaleFactor()” 章节

    scaleFactor(): Promise<number>

可用于将物理像素映射为逻辑像素的缩放因子。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`number`>

窗口的显示器缩放因子。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    const factor = await getCurrentWindow().scaleFactor();

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L521>

##### setAlwaysOnBottom()

“setAlwaysOnBottom()” 章节

    setAlwaysOnBottom(alwaysOnBottom): Promise<void>

窗口是否应始终位于其他窗口下方。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`alwaysOnBottom`| `布尔值 (boolean)`| 窗口是否应始终置于其他窗口之下。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    await getCurrentWindow().setAlwaysOnBottom(true);

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1259>

##### setAlwaysOnTop()

“setAlwaysOnTop()” 章节

    setAlwaysOnTop(alwaysOnTop): Promise<void>

窗口是否应始终位于其他窗口之上。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`alwaysOnTop`| `布尔值 (boolean)`| 窗口是否应始终置于其他窗口之上。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    await getCurrentWindow().setAlwaysOnTop(true);

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1241>

##### setBackgroundColor()

“setBackgroundColor()” 章节

    setBackgroundColor(color): Promise<void>

设置窗口背景颜色。

平台特定

  * **Windows:** Alpha 通道会被忽略。
  * **iOS / Android:** 不支持。

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

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1593>

##### setBadgeCount()

“setBadgeCount()” 章节

    setBadgeCount(count?): Promise<void>

设置徽章计数。它是针对整个应用程序的，而不是针对特定窗口的。

平台特定

  * **Windows** : 不支持。请改用 @{linkcode Window.setOverlayIcon}。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`count`?| `数字`| 徽章计数。使用 `undefined` 可移除徽章。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    await getCurrentWindow().setBadgeCount(5);

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1685>

##### setBadgeLabel()

“setBadgeLabel()” 章节

    setBadgeLabel(label?): Promise<void>

设置徽章标签（**仅限 macOS** ）。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`标签`?| `string`| 徽章标签。使用 `undefined` 可移除徽章。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    await getCurrentWindow().setBadgeLabel("Hello");

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1704>

##### setClosable()

“setClosable()” 章节

    setClosable(closable): Promise<void>

设置是否启用窗口的原生关闭按钮。

平台特定

  * **Linux:** GTK+ 会尽力说服窗口管理器不显示关闭按钮。根据系统环境的不同，在已可见的窗口上调用此函数可能没有任何效果。
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

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L995>

##### setContentProtected()

“setContentProtected()” 章节

    setContentProtected(protected_): Promise<void>

防止窗口内容被其他应用程序捕获。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`内容保护（protected_）。`| `布尔值 (boolean)`

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    await getCurrentWindow().setContentProtected(true);

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1276>

##### setCursorGrab()

“setCursorGrab()” 章节

    setCursorGrab(grab): Promise<void>

锁定光标，防止其离开窗口。

不保证光标会被隐藏。如果您希望隐藏它，需要自行设置。

平台特定

  * **Linux：** 不支持。
  * **macOS:** 这会将光标锁定在固定位置，视觉上可能显得很突兀。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`grab`| `布尔值 (boolean)`| `true` 表示锁定光标，`false` 表示释放。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    await getCurrentWindow().setCursorGrab(true);

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1532>

##### setCursorIcon()

“setCursorIcon()” 章节

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

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1574>

##### setCursorPosition()

“setCursorPosition()” 章节

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

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1608>

##### setCursorVisible()

“setCursorVisible()” 章节

    setCursorVisible(visible): Promise<void>

修改光标的可见性。

平台特定

  * **Windows:** 光标仅在窗口范围内被隐藏。
  * **macOS:** 只要窗口拥有输入焦点，光标就会被隐藏，即使光标在窗口之外。

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

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1556>

##### setDecorations()

“setDecorations()” 章节

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

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1177>

##### setEffects()

“setEffects()” 章节

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

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L908>

##### setFocus()

“setFocus()” 章节

    setFocus(): Promise<void>

将窗口置于最前并获取焦点。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    await getCurrentWindow().setFocus();

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1434>

##### setFocusable()

“setFocusable()” 章节

    setFocusable(focusable): Promise<void>

设置窗口是否可以获得焦点。

平台特定

  * **macOS** : 如果窗口已经获得焦点，在调用 `set_focusable(false)` 后无法取消其焦点。在这种情况下，您可以考虑调用 [Window.setFocus](/reference/javascript/api/namespacewindow/#setfocus)，但这会将窗口移动到后面，即在 Z 轴顺序上处于最底部。

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

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1457>

##### setFullscreen()

“setFullscreen()” 章节

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

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1401>

##### setIcon()

“setIcon()” 章节

    setIcon(icon): Promise<void>

设置窗口图标。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`icon`| | `string` | [`Uint8Array`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array) | `number`[] | [`ArrayBuffer`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer) | [`Image`](/reference/javascript/api/namespaceimage/#image)| 图标字节数据或图标文件路径。

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

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1482>

##### setIgnoreCursorEvents()

“setIgnoreCursorEvents()” 章节

    setIgnoreCursorEvents(ignore): Promise<void>

更改光标事件的行为。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`ignore`| `布尔值 (boolean)`| `true` 表示忽略光标事件；`false` 表示正常处理。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    await getCurrentWindow().setIgnoreCursorEvents(true);

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1629>

##### setMaxSize()

“setMaxSize()” 章节

    setMaxSize(size): Promise<void>

设置窗口的最大内部尺寸。如果 `size` 参数未定义，则取消该约束。

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

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1332>

##### setMaximizable()

“setMaximizable()” 章节

    setMaximizable(maximizable): Promise<void>

设置窗口原生最大化按钮是否启用。如果 resizable 设置为 false，此设置将被忽略。

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

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L950>

##### setMinSize()

“setMinSize()” 章节

    setMinSize(size): Promise<void>

设置窗口的最小内部尺寸。如果未提供 `size` 参数，则取消该约束。

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

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1312>

##### setMinimizable()

“setMinimizable()” 章节

    setMinimizable(minimizable): Promise<void>

设置窗口原生最小化按钮是否启用。

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

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L972>

##### setOverlayIcon()

“setOverlayIcon()” 章节

    setOverlayIcon(icon?): Promise<void>

设置覆盖图标。**仅限 Windows** ，可以为每个窗口设置覆盖图标。

请注意，您可能需要 `image-ico` 或 `image-png` Cargo 功能才能使用此 API。要启用它，请更改您的 Cargo.toml 文件

    [dependencies]

    tauri = { version = "...", features = ["...", "image-png"] }

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`icon`?| | `string` | [`Uint8Array`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array) | `number`[] | [`ArrayBuffer`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer) | [`Image`](/reference/javascript/api/namespaceimage/#image)| 图标字节数据或图标文件路径。使用 `undefined` 可移除覆盖图标。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    await getCurrentWindow().setOverlayIcon("/tauri/awesome.png");

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1733>

##### setPosition()

“setPosition()” 章节

    setPosition(position): Promise<void>

设置窗口的外部位置。

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

    import { getCurrentWindow, LogicalPosition } from '@tauri-apps/api/window';

    await getCurrentWindow().setPosition(new LogicalPosition(600, 500));

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1381>

##### setProgressBar()

“setProgressBar()” 章节

    setProgressBar(state): Promise<void>

设置任务栏进度状态。

平台特定

  * **Linux / macOS** : 进度条是针对整个应用程序的，不是针对特定窗口的。
  * **Linux** : 仅支持具有 `libunity` 的桌面环境（例如 GNOME）。

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

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1761>

##### setResizable()

“setResizable()” 章节

    setResizable(resizable): Promise<void>

更新窗口可调整大小标志。

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

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L889>

##### setShadow()

“setShadow()” 章节

    setShadow(enable): Promise<void>

窗口是否应有阴影。

平台特定

  * **Windows**
    * `false` 对带装饰的窗口无效，阴影始终为开启状态。
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

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1203>

##### setSimpleFullscreen()

“setSimpleFullscreen()” 章节

    setSimpleFullscreen(fullscreen): Promise<void>

在 macOS 上，切换一种不需要新 macOS Space 的全屏模式。返回一个布尔值，指示转换是否成功（如果窗口已经处于原生全屏状态，则此操作无效）。这是 macOS 在 Lion 版本之前全屏的工作方式，它允许用户在不使用另一个 Space 或接管整个监视器的情况下拥有全屏窗口。

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

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1417>

##### setSize()

“setSize()” 章节

    setSize(size): Promise<void>

以新的内部尺寸调整窗口大小。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`尺寸`| [`LogicalSize`](/reference/javascript/api/namespacedpi/#logicalsize) | [`PhysicalSize`](/reference/javascript/api/namespacedpi/#physicalsize) | [`Size`](/reference/javascript/api/namespacedpi/#size)| 逻辑或物理内部尺寸。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow, LogicalSize } from '@tauri-apps/api/window';

    await getCurrentWindow().setSize(new LogicalSize(600, 500));

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1294>

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

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1352>

##### setSkipTaskbar()

名为“setSkipTaskbar()”的章节

    setSkipTaskbar(skip): Promise<void>

是否应在任务栏中隐藏窗口图标。

平台特定

  * **macOS:** 不支持。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`跳过`| `布尔值 (boolean)`| true 为隐藏窗口图标，false 为显示它。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    await getCurrentWindow().setSkipTaskbar(true);

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1506>

##### setTheme()

名为“setTheme()”的章节

    setTheme(theme?): Promise<void>

设置窗口主题，传入 `null` 或 `undefined` 以跟随系统主题

平台特定

  * **Linux / macOS** : 主题是全局应用，不针对特定窗口。
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

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1806>

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

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1013>

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

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1789>

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

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1777>

##### show()

名为“show()”的章节

    show(): Promise<void>

将窗口可见性设置为 true。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    await getCurrentWindow().show();

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1110>

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

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1646>

##### startResizeDragging()

名为“startResizeDragging()”的章节

    startResizeDragging(direction): Promise<void>

开始调整窗口大小的拖动。

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

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1662>

##### theme()

名为“theme()”的章节

    theme(): Promise<null | Theme>

获取窗口的当前主题。

平台特定

  * **macOS:** 主题在 macOS 10.14 上引入。macOS 10.13 及以下版本返回 `light`。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`null` | [`Theme`](/reference/javascript/api/namespacewindow/#theme-2)>

窗口主题。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    const theme = await getCurrentWindow().theme();

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L801>

##### title()

名为“title()”的章节

    title(): Promise<string>

获取窗口的当前标题。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`string`>

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    const title = await getCurrentWindow().title();

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L780>

##### toggleMaximize()

名为“toggleMaximize()”的章节

    toggleMaximize(): Promise<void>

切换窗口的最大化状态。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    await getCurrentWindow().toggleMaximize();

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1062>

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

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1046>

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

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1094>

##### getAll()

名为“getAll()”的章节

    static getAll(): Promise<Window[]>

获取所有可用窗口的 `Window` 实例列表。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`Window`](/reference/javascript/api/namespacewindow/#window)[]>

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L345>

##### getByLabel()

名为“getByLabel()”的章节

    static getByLabel(label): Promise<null | Window>

获取与给定标签关联的窗口。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`标签`| `string`| 窗口标签。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`null` | [`Window`](/reference/javascript/api/namespacewindow/#window)>

用于与窗口通信的 Window 实例，如果窗口不存在则返回 null。

###### 示例

标题为“Example”的章节

    import { Window } from '@tauri-apps/api/window';

    const mainWindow = Window.getByLabel('main');

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L331>

##### getCurrent()

名为“getCurrent()”的章节

    static getCurrent(): Window

获取当前窗口的 `Window` 实例。

###### 返回

名为“返回值”的部分

[`窗口`](/reference/javascript/api/namespacewindow/#window)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L338>

##### getFocusedWindow()

名为“getFocusedWindow()”的章节

    static getFocusedWindow(): Promise<null | Window>

获取获得焦点的窗口。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`null` | [`Window`](/reference/javascript/api/namespacewindow/#window)>

Window 实例，如果没有获得焦点的窗口则为 `undefined`。

###### 示例

标题为“Example”的章节

    import { Window } from '@tauri-apps/api/window';

    const focusedWindow = Window.getFocusedWindow();

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L359>

## 接口

名为“接口”的部分

### Effects

名为“Effects”的章节

窗口效果配置对象

#### 始于

名为“起始版本”的部分

2.0.0

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
`color?`| [`Color`](/reference/javascript/api/namespacewebview/#color)| 窗口效果颜色。仅在 Windows 10 v1903+ 上影响 [Effect.Blur](/reference/javascript/api/namespacewindow/#blur) 和 [Effect.Acrylic](/reference/javascript/api/namespacewindow/#acrylic)。在 Windows 7 或 Windows 11 上没有任何效果。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2289>
`effects`| [`Effect`](/reference/javascript/api/namespacewindow/#effect)[]| 应用于窗口的窗口效果列表。冲突的效果将应用第一个并忽略其余的。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2276>
`radius?`| `数字`| 窗口效果圆角半径 **仅限 macOS**| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2284>
`state?`| [`EffectState`](/reference/javascript/api/namespacewindow/#effectstate)| 窗口效果状态 **仅限 macOS**| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2280>

* * *

### Monitor

名为“Monitor”的章节

允许您检索有关给定显示器的信息。

#### 始于

名为“起始版本”的部分

1.0.0

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
`name`| `null` | `string`| 显示器的人类可读名称| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L49>
`position`| [`PhysicalPosition`](/reference/javascript/api/namespacedpi/#physicalposition)| 相对于较大全屏区域的显示器左上角位置。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L53>
`scaleFactor`| `数字`| 可用于将物理像素映射为逻辑像素的缩放因子。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L60>
`size`| [`PhysicalSize`](/reference/javascript/api/namespacedpi/#physicalsize)| 显示器的分辨率。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L51>
`workArea`| `对象`| 显示器的工作区。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L55>
`workArea.position`| [`PhysicalPosition`](/reference/javascript/api/namespacedpi/#physicalposition)| -| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L56>
`workArea.size`| [`PhysicalSize`](/reference/javascript/api/namespacedpi/#physicalsize)| -| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L57>

* * *

### ProgressBarState

名为“ProgressBarState”的章节

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
`progress?`| `数字`| 进度条进度。该值范围从 `0` 到 `100`| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L208>
`status?`| [`ProgressBarStatus`](/reference/javascript/api/namespacewindow/#progressbarstatus)| 进度条状态。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L204>

* * *

### ScaleFactorChanged

名为“ScaleFactorChanged”的章节

`scaleChange` 事件的载荷。

#### 始于

名为“起始版本”的部分

1.0.2

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
`scaleFactor`| `数字`| 新的窗口缩放因子。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L83>
`size`| [`PhysicalSize`](/reference/javascript/api/namespacedpi/#physicalsize)| 新的窗口尺寸| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L85>

* * *

### WindowOptions

名为“WindowOptions”的章节

要创建的窗口配置。

#### 始于

名为“起始版本”的部分

1.0.0

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
`allowLinkPreview?`| `布尔值 (boolean)`| 在 macOS 和 iOS 上，长按链接时会有链接预览，默认启用此功能。详见 <https://docs.rs/objc2-web-kit/latest/objc2_web_kit/struct.WKWebView.html#method.allowsLinkPreview>| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2491>
`alwaysOnBottom?`| `布尔值 (boolean)`| 窗口是否应始终位于其他窗口下方。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2364>
`alwaysOnTop?`| `布尔值 (boolean)`| 窗口是否应始终置于其他窗口之上。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2362>
`backgroundColor?`| [`Color`](/reference/javascript/api/namespacewebview/#color)| 设置窗口背景颜色。平台特定：- **Android / iOS:** 不支持。 - **Windows** : alpha 通道被忽略。 **自** 2.1.0 起| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2468>
`backgroundThrottling?`| [`BackgroundThrottlingPolicy`](/reference/javascript/api/namespacewindow/#backgroundthrottlingpolicy)| 更改默认的后台节流行为。## 平台特定 - **Linux / Windows / Android** : 不支持。挂起的 WebLock 事务等变通方法可能足以应对。 - **iOS** : 自 17.0+ 版本起支持。 - **macOS** : 自 14.0+ 版本起支持。请参阅 <https://github.com/tauri-apps/tauri/issues/5250#issuecomment-2569380578> **自** 2.3.0 起| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2482>
`center?`| `布尔值 (boolean)`| 在屏幕中心显示窗口。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2307>
`closable?`| `布尔值 (boolean)`| 是否启用窗口的原生关闭按钮。默认为 `true`。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2423>
`contentProtected?`| `布尔值 (boolean)`| 防止窗口内容被其他应用程序捕获。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2366>
`decorations?`| `布尔值 (boolean)`| 窗口是否应具有边框和标题栏。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2360>
`disableInputAccessoryView?`| `布尔值 (boolean)`| 允许在 iOS 上禁用输入附件视图。附件视图是当文本输入元素获得焦点时出现在键盘上方的视图。它通常显示一个带有“完成”、“下一步”按钮的视图。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2498>
`focus?`| `布尔值 (boolean)`| 窗口是否应初始获得焦点。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2346>
`focusable?`| `布尔值 (boolean)`| 窗口是否可聚焦。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2348>
`fullscreen?`| `布尔值 (boolean)`| 窗口是否处于全屏模式。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2344>
`height?`| `数字`| 初始高度（逻辑像素）。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2315>
`hiddenTitle?`| `布尔值 (boolean)`| 如果为 `true`，则在 macOS 上隐藏窗口标题。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2404>
`javascriptDisabled?`| `布尔值 (boolean)`| 我们是否应该禁用 webview 上的 JavaScript 代码执行。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2486>
`maxHeight?`| `数字`| 最大高度（逻辑像素）。仅在同时设置了 `maxWidth` 时有效。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2323>
`maxWidth?`| `数字`| 最大宽度（逻辑像素）。仅在同时设置了 `maxHeight` 时有效。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2321>
`maximizable?`| `布尔值 (boolean)`| 是否启用窗口的原生最大化按钮。默认为 `true`。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2415>
`maximized?`| `布尔值 (boolean)`| 窗口在创建时是否应最大化。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2356>
`minHeight?`| `数字`| 最小高度（逻辑像素）。仅在同时设置了 `minWidth` 时有效。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2319>
`minWidth?`| `数字`| 最小宽度（逻辑像素）。仅在同时设置了 `minHeight` 时有效。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2317>
`minimizable?`| `布尔值 (boolean)`| 是否启用窗口的原生最小化按钮。默认为 `true`。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2419>
`parent?`| `string` | [`Window`](/reference/javascript/api/namespacewindow/#window) | [`WebviewWindow`](/reference/javascript/api/namespacewebviewwindow/#webviewwindow)| 为要创建的窗口设置父窗口。可以是 [`Window`](/reference/javascript/api/namespacewindow/#window) 实例或窗口标签。平台特定 - **Windows** : 这将设置传入的父窗口作为要创建窗口的所有者窗口。来自 [MSDN owned windows docs](https://docs.microsoft.com/en-us/windows/win32/winmsg/window-features#owned-windows): - 拥有的窗口始终位于其所有者之上的 Z 轴顺序。 - 当所有者销毁时，系统会自动销毁拥有的窗口。 - 当所有者最小化时，拥有的窗口会被隐藏。 - **Linux** : 这使得新窗口成为父级的瞬态窗口，参见 <https://www.gtk.org/gtk3/method.Window.set_transient_for.html> \- **macOS** : 这将窗口添加为父级的子窗口，参见 <https://developer.apple.com/documentation/appkit/nswindow/1419152-addchildwindow?language=objc>| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2437>
`preventOverflow?`| `boolean` | `PreventOverflowMargin`| 防止窗口在创建时溢出工作区域（例如：显示器尺寸 - 任务栏尺寸），这意味着窗口大小将被限制为 `显示器尺寸 - 任务栏尺寸`。可以设置为 `true` 或 PreventOverflowMargin 对象，以设置一个应考虑的额外边距来确定工作区域（在这种情况下，窗口大小将限制为 `显示器尺寸 - 任务栏尺寸 - 边距`） **注意** ：溢出检查仅在窗口创建时执行，调整大小仍可能导致溢出。平台特定 - **iOS / Android:** 不支持。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2338>
`resizable?`| `布尔值 (boolean)`| 窗口是否可以调整大小。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2340>
`scrollBarStyle?`| [`ScrollBarStyle`](/reference/javascript/api/namespacewindow/#scrollbarstyle)| 指定与 Webview 一起使用的原生滚动条样式。修改滚动条的 CSS 样式应用于此处配置的原生外观之上。默认为 `default`，即浏览器默认值。## 平台特定 - **Windows** : - `fluentOverlay` 需要 WebView2 Runtime 125.0.2535.41 或更高版本，在旧版本上不起作用。 - 此选项必须为所有 Webview 提供相同的值。 - **Linux / Android / iOS / macOS** : 不支持。仅支持 `Default` 且不执行任何操作。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2513>
`shadow?`| `布尔值 (boolean)`| 窗口是否有阴影。平台特定 - **Windows:** \- `false` 对装饰窗口没有影响，阴影始终开启。 - `true` 将使无装饰窗口具有 1px 白色边框，在 Windows 11 上，它将具有圆角。 - **Linux:** 不支持。 **自** 2.0.0 起| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2382>
`skipTaskbar?`| `布尔值 (boolean)`| 窗口图标是否应添加到任务栏。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2368>
`tabbingIdentifier?`| `string`| 定义 macOS 上的窗口 [标签标识符](https://developer.apple.com/documentation/appkit/nswindow/1644704-tabbingidentifier)。具有相同标签标识符的窗口将被分组在一起。如果未设置标签标识符，自动标签将被禁用。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2411>
`theme?`| [`Theme`](/reference/javascript/api/namespacewindow/#theme-2)| 初始窗口主题。默认为系统主题。仅在 Windows 和 macOS 10.14+ 上实现。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2388>
`title?`| `string`| 窗口标题。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2342>
`titleBarStyle?`| [`TitleBarStyle`](/reference/javascript/api/namespacewindow/#titlebarstyle-1)| macOS 标题栏的样式。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2392>
`trafficLightPosition?`| [`LogicalPosition`](/reference/javascript/api/namespacedpi/#logicalposition)| macOS 上窗口控件的位置。需要 `titleBarStyle: 'overlay'` 和 `decorations: true`。 **自** 2.4.0 起| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2400>
`transparent?`| `布尔值 (boolean)`| 窗口是否透明。请注意，在 `macOS` 上，这需要 `macos-private-api` 功能标志，在 `tauri.conf.json > app > macOSPrivateApi` 下启用。警告：在 `macOS` 上使用私有 API 会阻止您的应用程序被 `App Store` 接受。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2354>
`visible?`| `布尔值 (boolean)`| 窗口在创建时是否应立即可见。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2358>
`visibleOnAllWorkspaces?`| `布尔值 (boolean)`| 窗口是否应在所有工作区或虚拟桌面上可见。平台特定 - **Windows / iOS / Android:** 不支持。 **自** 2.0.0 起| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2446>
`width?`| `数字`| 初始宽度（逻辑像素）。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2313>
`windowEffects?`| [`Effects`](/reference/javascript/api/namespacewindow/#effects)| 窗口效果。需要窗口是透明的。平台特定：- **Windows** : 如果使用装饰或阴影，您可能想尝试此变通方法 <https://github.com/tauri-apps/tao/issues/72#issuecomment-975607891> \- **Linux** : 不支持| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2457>
`x?`| `数字`| 初始垂直位置（逻辑像素）。仅在同时设置了 `y` 时有效。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2309>
`y?`| `数字`| 初始水平位置（逻辑像素）。仅在同时设置了 `x` 时有效。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2311>

* * *

### WindowSizeConstraints

名为“WindowSizeConstraints”的章节

#### 属性

名为“属性”的部分

属性| 类型| 定义于
---|---|---
`maxHeight?`| `数字`| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L197>
`maxWidth?`| `数字`| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L196>
`minHeight?`| `数字`| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L195>
`minWidth?`| `数字`| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L194>

## 类型别名 (Type Aliases)

名为“类型别名”的部分

### CursorIcon

名为“CursorIcon”的章节

    type CursorIcon:

      | "default"

      | "crosshair"

      | "hand"

      | "arrow"

      | "move"

      | "text"

      | "wait"

      | "help"

      | "progress"

      | "notAllowed"

      | "contextMenu"

      | "cell"

      | "verticalText"

      | "alias"

      | "copy"

      | "noDrop"

      | "grab"

      | "grabbing"

      | "allScroll"

      | "zoomIn"

      | "zoomOut"

      | "eResize"

      | "nResize"

      | "neResize"

      | "nwResize"

      | "sResize"

      | "seResize"

      | "swResize"

      | "wResize"

      | "ewResize"

      | "nsResize"

      | "neswResize"

      | "nwseResize"

      | "colResize"

      | "rowResize";

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L129>

* * *

### Theme

名为“Theme”的章节

    type Theme: "light" | "dark";

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L63>

* * *

### TitleBarStyle

名为“TitleBarStyle”的章节

    type TitleBarStyle: "visible" | "transparent" | "overlay";

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L64>

## 函数

名为“函数”的部分

### availableMonitors()

名为“availableMonitors()”的章节

    function availableMonitors(): Promise<Monitor[]>

返回系统上所有可用显示器的列表。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`Monitor`](/reference/javascript/api/namespacewindow/#monitor)[]>

#### 示例

标题为“Example”的章节

    import { availableMonitors } from '@tauri-apps/api/window';

    const monitors = await availableMonitors();

#### 始于

名为“起始版本”的部分

1.0.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2592>

* * *

### currentMonitor()

名为“currentMonitor()”的章节

    function currentMonitor(): Promise<Monitor | null>

返回窗口当前所在的显示器。如果无法检测到当前显示器，则返回 `null`。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`Monitor`](/reference/javascript/api/namespacewindow/#monitor) | `null`>

#### 示例

标题为“Example”的章节

    import { currentMonitor } from '@tauri-apps/api/window';

    const monitor = await currentMonitor();

#### 始于

名为“起始版本”的部分

1.0.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2542>

* * *

### cursorPosition()

名为“cursorPosition()”的章节

    function cursorPosition(): Promise<PhysicalPosition>

获取相对于桌面左上角的光标位置。

请注意，桌面的左上角不一定与屏幕相同。如果用户使用带有多个显示器的桌面，桌面的左上角是 Windows 和 macOS 上主显示器的左上角，或者 X11 上最左侧显示器的左上角。

如果窗口的左上角在可见屏幕区域之外，坐标可能为负数。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`PhysicalPosition`](/reference/javascript/api/namespacedpi/#physicalposition)>

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2608>

* * *

### getAllWindows()

名为“getAllWindows()”的章节

    function getAllWindows(): Promise<Window[]>

获取所有可用窗口的 `Window` 实例列表。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`Window`](/reference/javascript/api/namespacewindow/#window)[]>

#### 始于

名为“起始版本”的部分

1.0.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L228>

* * *

### getCurrentWindow()

名为“getCurrentWindow()”的章节

    function getCurrentWindow(): Window

获取当前窗口的 `Window` 实例。

#### 返回

名为“返回值”的部分

[`窗口`](/reference/javascript/api/namespacewindow/#window)

#### 始于

名为“起始版本”的部分

1.0.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L216>

* * *

### monitorFromPoint()

名为“monitorFromPoint()”的章节

    function monitorFromPoint(x, y): Promise<Monitor | null>

返回包含给定点的显示器。如果找不到任何显示器，则返回 `null`。

#### 参数

名为“参数”的部分

参数| 类型
---|---
`x`| `数字`
`y`| `数字`

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`Monitor`](/reference/javascript/api/namespacewindow/#monitor) | `null`>

#### 示例

标题为“Example”的章节

    import { monitorFromPoint } from '@tauri-apps/api/window';

    const monitor = await monitorFromPoint(100.0, 200.0);

#### 始于

名为“起始版本”的部分

1.0.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2575>

* * *

### primaryMonitor()

名为“primaryMonitor()”的章节

    function primaryMonitor(): Promise<Monitor | null>

返回系统的主显示器。如果无法确定任何显示器为主显示器，则返回 `null`。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`Monitor`](/reference/javascript/api/namespacewindow/#monitor) | `null`>

#### 示例

标题为“Example”的章节

    import { primaryMonitor } from '@tauri-apps/api/window';

    const monitor = await primaryMonitor();

#### 始于

名为“起始版本”的部分

1.0.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2559>