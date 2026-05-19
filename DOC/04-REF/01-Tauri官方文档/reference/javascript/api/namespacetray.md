# tray

_Source: https://v2.tauri.org.cn/reference/javascript/api/namespacetray/_

## 类

名为“类”的部分

### TrayIcon

标题为“TrayIcon”的章节

托盘图标类及相关方法。此类型的构造函数是私有的，你应该使用静态方法 [`TrayIcon.new`](/reference/javascript/api/namespacetray/#new)。

#### 警告

标题为“Warning”的章节

与 Rust 不同，JavaScript 没有办法在对象被垃圾回收时运行清理代码，但此托盘图标会在 Tauri 应用退出时被清理。不过，如果你想提前清理此对象，你需要调用 [`TrayIcon.close`](/reference/javascript/api/namespacecore/#close)。

#### 示例

标题为“Example”的章节

    import { TrayIcon } from '@tauri-apps/api/tray';

    const tray = await TrayIcon.new({ tooltip: 'awesome tray tooltip' });

    tray.set_tooltip('new tooltip');

#### 继承 (Extends)

名为“继承自”的章节

  * [`Resource`](/reference/javascript/api/namespacecore/#resource)

#### 属性

名为“属性”的部分

属性| 修饰符| 类型| 描述| 定义于
---|---|---|---|---
`id`| `public`| `string`| 与此托盘图标关联的 ID。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L160>

#### 访问器

名为“访问器”的部分

##### rid

名为“rid”的部分

    get rid(): number

###### 返回

名为“返回值”的部分

`数字`

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Resource`](/reference/javascript/api/namespacecore/#resource).[`rid`](/reference/javascript/api/namespacecore/#rid)

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

###### 继承自 (Inherited from)

名为“继承自”的章节

[`Resource`](/reference/javascript/api/namespacecore/#resource).[`close`](/reference/javascript/api/namespacecore/#close)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/core.ts#L330>

##### setIcon()

“setIcon()” 章节

    setIcon(icon): Promise<void>

设置一个新的托盘图标。如果传入 `null`，将移除该图标。

请注意，您可能需要 `image-ico` 或 `image-png` Cargo 功能才能使用此 API。要启用它，请更改您的 Cargo.toml 文件

    [dependencies]

    tauri = { version = "...", features = ["...", "image-png"] }

###### 参数

名为“参数”的部分

参数| 类型
---|---
`icon`| | `null` | `string` | [`Uint8Array`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array) | `number`[] | [`ArrayBuffer`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer) | [`Image`](/reference/javascript/api/namespaceimage/#image)

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L224>

##### setIconAsTemplate()

标题为“setIconAsTemplate()”的章节

    setIconAsTemplate(asTemplate): Promise<void>

将当前图标设置为[模板 (template)](https://developer.apple.com/documentation/appkit/nsimage/1520017-template?language=objc)。**仅限 macOS**

###### 参数

名为“参数”的部分

参数| 类型
---|---
`asTemplate`| `布尔值 (boolean)`

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L292>

##### setMenu()

标题为“setMenu()”的章节

    setMenu(menu): Promise<void>

设置一个新的托盘菜单。

平台特定

  * **Linux** ：一旦设置了菜单，就无法将其移除，因此 `null` 无效

###### 参数

名为“参数”的部分

参数| 类型
---|---
`menu`| `null` | [`Submenu`](/reference/javascript/api/namespacemenu/#submenu) | [`Menu`](/reference/javascript/api/namespacemenu/#menu)

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L241>

##### ~~setMenuOnLeftClick()~~

标题为“setMenuOnLeftClick()”的章节

    setMenuOnLeftClick(onLeft): Promise<void>

禁用或启用左键点击时显示托盘菜单。

平台特定

  * **Linux** ：不支持。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`onLeft`| `布尔值 (boolean)`

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

###### 已弃用

标题为“已弃用”的章节

请改用 [`TrayIcon.setShowMenuOnLeftClick`](/reference/javascript/api/namespacetray/#setshowmenuonleftclick)。

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L308>

##### setShowMenuOnLeftClick()

标题为“setShowMenuOnLeftClick()”的章节

    setShowMenuOnLeftClick(onLeft): Promise<void>

禁用或启用左键点击时显示托盘菜单。

平台特定

  * **Linux** ：不支持。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`onLeft`| `布尔值 (boolean)`

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

###### 始于

名为“起始版本”的部分

2.2.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L324>

##### setTempDirPath()

标题为“setTempDirPath()”的章节

    setTempDirPath(path): Promise<void>

设置托盘图标临时目录路径。**仅限 Linux** 。

在 Linux 上，我们需要将图标写入磁盘，通常是 `$XDG_RUNTIME_DIR/tray-icon` 或 `$TEMP/tray-icon`。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`path`| `null` | `string`

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L287>

##### setTitle()

名为“setTitle()”的章节

    setTitle(title): Promise<void>

设置此托盘图标的工具提示 (tooltip)。

平台特定

  * **Linux：** 除非同时存在图标，否则不会显示标题。标题适用于数字和其他频繁更新的信息。通常情况下，除非用户主动请求，否则不建议显示它，因为它会占用用户面板上的大量空间。在所有可视化界面中可能并不都会显示。
  * **Windows：** 不支持

###### 参数

名为“参数”的部分

参数| 类型
---|---
`标题`| `null` | `string`

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L272>

##### setTooltip()

标题为“setTooltip()”的章节

    setTooltip(tooltip): Promise<void>

设置此托盘图标的工具提示 (tooltip)。

平台特定

  * **Linux：** 不支持

###### 参数

名为“参数”的部分

参数| 类型
---|---
`tooltip`| `null` | `string`

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L256>

##### setVisible()

标题为“setVisible()”的章节

    setVisible(visible): Promise<void>

显示或隐藏此托盘图标。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`visible`| `布尔值 (boolean)`

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L277>

##### getById()

标题为“getById()”的章节

    static getById(id): Promise<null | TrayIcon>

使用提供的 ID 获取托盘图标。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`ID`| `string`

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`null` | [`TrayIcon`](/reference/javascript/api/namespacetray/#trayicon)>

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L168>

##### new()

名为“new()”的章节

    static new(options?): Promise<TrayIcon>

创建一个新的 [`TrayIcon`](/reference/javascript/api/namespacetray/#trayicon)

平台特定

  * **Linux：** 有时如果不设置菜单，图标将不可见。设置一个空的 [`Menu`](/reference/javascript/api/namespacemenu/#menu) 就足够了。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`选项`?| [`TrayIconOptions`](/reference/javascript/api/namespacetray/#trayiconoptions)

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`TrayIcon`](/reference/javascript/api/namespacetray/#trayicon)>

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L192>

##### removeById()

标题为“removeById()”的章节

    static removeById(id): Promise<void>

从 Tauri 的内部状态中移除具有指定 ID 的托盘图标。

请注意，如果图标未在其他地方克隆或被 JS 引用，这可能会导致托盘图标消失。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`ID`| `string`

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L180>

## 接口

名为“接口”的部分

### TrayIconOptions

标题为“TrayIconOptions”的章节

[`TrayIcon`](/reference/javascript/api/namespacetray/#new) 创建选项

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
`action?`| (`event`: [`TrayIconEvent`](/reference/javascript/api/namespacetray/#trayiconevent)) => `void`| 托盘图标事件的处理程序。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L137>
`icon?`| | `string` | [`Uint8Array`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array) | `number`[] | [`ArrayBuffer`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer) | [`Image`](/reference/javascript/api/namespaceimage/#image)| 托盘图标，可以是图标字节或图标文件的路径。请注意，你可能需要 `image-ico` 或 `image-png` Cargo 功能才能使用此 API。要启用它，请修改你的 Cargo.toml 文件： `[dependencies] tauri = { version = "...", features = ["...", "image-png"] }`| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L89>
`iconAsTemplate?`| `布尔值 (boolean)`| 将图标用作[模板](https://developer.apple.com/documentation/appkit/nsimage/1520017-template?language=objc)。**仅限 macOS** 。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L115>
`id?`| `string`| 托盘图标 ID。如果未定义，将分配一个随机 ID| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L76>
`menu?`| [`Submenu`](/reference/javascript/api/namespacemenu/#submenu) | [`Menu`](/reference/javascript/api/namespacemenu/#menu)| 托盘图标菜单| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L78>
~~`menuOnLeftClick?`~~| `布尔值 (boolean)`| 是否在左键点击时显示托盘菜单，默认为 `true`。平台特定：- **Linux** ：不支持。**已弃用** ，请改用 [`TrayIconOptions.showMenuOnLeftClick`](/reference/javascript/api/namespacetray/#showmenuonleftclick)。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L125>
`showMenuOnLeftClick?`| `布尔值 (boolean)`| 是否在左键点击时显示托盘菜单，默认为 `true`。平台特定：- **Linux** ：不支持。**从 2.2.0 版本开始**| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L135>
`tempDirPath?`| `string`| 托盘图标临时目录路径。**仅限 Linux** 。在 Linux 上，我们需要将图标写入磁盘，通常是 `$XDG_RUNTIME_DIR/tray-icon` 或 `$TEMP/tray-icon`。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L111>
`title?`| `string`| 托盘标题。平台特定： - **Linux：** 除非同时存在图标，否则不会显示标题。标题适用于数字和其他频繁更新的信息。通常情况下，除非用户主动请求，否则不建议显示它，因为它会占用用户面板上的大量空间。在所有可视化界面中可能并不都会显示。 - **Windows：** 不支持。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L104>
`tooltip?`| `string`| 托盘图标工具提示| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L91>

## 类型别名 (Type Aliases)

名为“类型别名”的部分

### MouseButton

标题为“MouseButton”的章节

    type MouseButton: "Left" | "Right" | "Middle";

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L11>

* * *

### MouseButtonState

标题为“MouseButtonState”的章节

    type MouseButtonState: "Up" | "Down";

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L10>

* * *

### TrayIconClickEvent

标题为“TrayIconClickEvent”的章节

    type TrayIconClickEvent: object;

#### 类型声明

名为“类型定义”的章节

名称| 类型| 描述| 定义于
---|---|---|---
`button`| [`MouseButton`](/reference/javascript/api/namespacetray/#mousebutton)| 触发此事件的鼠标按钮。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L35>
`buttonState`| [`MouseButtonState`](/reference/javascript/api/namespacetray/#mousebuttonstate)| 触发此事件时的鼠标按钮状态。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L37>

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L33>

* * *

### TrayIconEvent

标题为“TrayIconEvent”的章节

    type TrayIconEvent:

      | TrayIconEventBase<"Click"> & TrayIconClickEvent

      | TrayIconEventBase<"DoubleClick"> & Omit<TrayIconClickEvent, "buttonState">

      | TrayIconEventBase<"Enter">

      | TrayIconEventBase<"Move">

    | TrayIconEventBase<"Leave">;

描述托盘图标事件。

平台特定

  * **Linux** ：不支持。即使显示了图标，该事件也不会发出，但右键点击图标时仍会显示上下文菜单。

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L48>

* * *

### TrayIconEventBase<T>

标题为“TrayIconEventBase<T>”的章节

    type TrayIconEventBase<T>: object;

#### 类型参数

名为“类型参数”的部分

类型参数
---
`T` _继承自_ [`TrayIconEventType`](/reference/javascript/api/namespacetray/#trayiconeventtype)

#### 类型声明

名为“类型定义”的章节

名称| 类型| 描述| 定义于
---|---|---|---
`ID`| `string`| 触发此事件的托盘图标 ID。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L23>
`位置`| [`PhysicalPosition`](/reference/javascript/api/namespacedpi/#physicalposition)| 触发此事件的物理点击位置。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L25>
`rect`| `对象`| 托盘图标的位置和大小。| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L27>
`rect.position`| [`PhysicalPosition`](/reference/javascript/api/namespacedpi/#physicalposition)| -| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L28>
`rect.size`| [`PhysicalSize`](/reference/javascript/api/namespacedpi/#physicalsize)| -| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L29>
`type`| `T`| 托盘图标事件类型| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L21>

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L19>

* * *

### TrayIconEventType

标题为“TrayIconEventType”的章节

    type TrayIconEventType:

      | "Click"

      | "DoubleClick"

      | "Enter"

      | "Move"

      | "Leave";

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/tray.ts#L12>