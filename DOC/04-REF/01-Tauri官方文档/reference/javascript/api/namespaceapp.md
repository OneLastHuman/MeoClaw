# app

_Source: https://v2.tauri.org.cn/reference/javascript/api/namespaceapp/_

## 枚举

标题为“枚举”的章节

### BundleType

名为“BundleType”的章节

当前应用程序的打包类型。

#### 枚举成员

标题为“枚举成员”的章节

##### App

名为“App”的章节

    App: "app";

macOS 应用程序包

**源文件** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/app.ts#L48>

##### AppImage

名为“AppImage”的章节

    AppImage: "appimage";

Linux AppImage

**源文件** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/app.ts#L46>

##### Deb

名为“Deb”的章节

    Deb: "deb";

Linux Debian 软件包

**源文件** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/app.ts#L42>

##### Msi

名为“Msi”的章节

    Msi: "msi";

Windows MSI

**源文件** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/app.ts#L40>

##### Nsis

名为“Nsis”的章节

    Nsis: "nsis";

Windows NSIS

**源文件** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/app.ts#L38>

##### Rpm

名为“Rpm”的章节

    Rpm: "rpm";

Linux RPM

**源文件** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/app.ts#L44>

## 类型别名 (Type Aliases)

名为“类型别名”的部分

### DataStoreIdentifier

名为“DataStoreIdentifier”的章节

    type DataStoreIdentifier: [number, number, number, number, number, number, number, number, number, number, number, number, number, number, number, number];

用于 macOS 和 iOS 上数据存储的标识符类型。

表示一个 128 位标识符，通常表示为 16 字节的 UUID。

**源文件** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/app.ts#L14>

* * *

### OnBackButtonPressPayload

名为“OnBackButtonPressPayload”的章节

    type OnBackButtonPressPayload: object;

onBackButtonPress 事件的载荷。

#### 类型声明

名为“类型定义”的章节

名称| 类型| 描述| 定义于
---|---|---|---
`canGoBack`| `布尔值 (boolean)`| webview 的 canGoBack 属性是否为真。| **源文件** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/app.ts#L260>

**源文件** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/app.ts#L258>

## 函数

名为“函数”的部分

### defaultWindowIcon()

名为“defaultWindowIcon()”的章节

    function defaultWindowIcon(): Promise<Image | null>

获取默认窗口图标。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`Image`](/reference/javascript/api/namespaceimage/#image) | `null`>

#### 示例

标题为“Example”的章节

    import { defaultWindowIcon } from '@tauri-apps/api/app';

    const icon = await defaultWindowIcon();

#### 始于

名为“起始版本”的部分

2.0.0

**源文件** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/app.ts#L197>

* * *

### fetchDataStoreIdentifiers()

名为“fetchDataStoreIdentifiers()”的章节

    function fetchDataStoreIdentifiers(): Promise<DataStoreIdentifier[]>

获取 macOS 和 iOS 上的数据存储标识符。

更多信息请参见 <https://developer.apple.com/documentation/webkit/wkwebsitedatastore>。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`DataStoreIdentifier`](/reference/javascript/api/namespaceapp/#datastoreidentifier)[]>

#### 示例

标题为“Example”的章节

    import { fetchDataStoreIdentifiers } from '@tauri-apps/api/app';

    const ids = await fetchDataStoreIdentifiers();

#### 始于

名为“起始版本”的部分

2.4.0

**源文件** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/app.ts#L160>

* * *

### getBundleType()

名为“getBundleType()”的章节

    function getBundleType(): Promise<BundleType>

获取应用程序包类型。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`BundleType`](/reference/javascript/api/namespaceapp/#bundletype)>

#### 示例

标题为“Example”的章节

    import { getBundleType } from '@tauri-apps/api/app';

    const type = await getBundleType();

#### 始于

名为“起始版本”的部分

2.5.0

**源文件** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/app.ts#L251>

* * *

### getIdentifier()

名为“getIdentifier()”的章节

    function getIdentifier(): Promise<string>

获取应用程序标识符。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`string`>

`tauri.conf.json` 中配置的应用程序标识符。

#### 示例

标题为“Example”的章节

    import { getIdentifier } from '@tauri-apps/api/app';

    const identifier = await getIdentifier();

#### 始于

名为“起始版本”的部分

2.4.0

**源文件** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/app.ts#L112>

* * *

### getName()

名为“getName()”的章节

    function getName(): Promise<string>

获取应用程序名称。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`string`>

#### 示例

标题为“Example”的章节

    import { getName } from '@tauri-apps/api/app';

    const appName = await getName();

#### 始于

名为“起始版本”的部分

1.0.0

**源文件** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/app.ts#L81>

* * *

### getTauriVersion()

名为“getTauriVersion()”的章节

    function getTauriVersion(): Promise<string>

获取此应用程序使用的 Tauri 框架版本。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`string`>

#### 示例

标题为“Example”的章节

    import { getTauriVersion } from '@tauri-apps/api/app';

    const tauriVersion = await getTauriVersion();

#### 始于

名为“起始版本”的部分

1.0.0

**源文件** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/app.ts#L96>

* * *

### getVersion()

名为“getVersion()”的章节

    function getVersion(): Promise<string>

获取应用程序版本。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`string`>

#### 示例

标题为“Example”的章节

    import { getVersion } from '@tauri-apps/api/app';

    const appVersion = await getVersion();

#### 始于

名为“起始版本”的部分

1.0.0

**源文件** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/app.ts#L67>

* * *

### hide()

“hide()” 章节

    function hide(): Promise<void>

在 macOS 上隐藏应用程序。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

#### 示例

标题为“Example”的章节

    import { hide } from '@tauri-apps/api/app';

    await hide();

#### 始于

名为“起始版本”的部分

1.2.0

**源文件** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/app.ts#L143>

* * *

### onBackButtonPress()

名为“onBackButtonPress()”的章节

    function onBackButtonPress(handler): Promise<PluginListener>

监听 Android 上的 backButton 事件。

#### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`处理程序`| (`payload`) => `void`|

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`PluginListener`](/reference/javascript/api/namespacecore/#pluginlistener)>

**源文件** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/app.ts#L267>

* * *

### removeDataStore()

名为“removeDataStore()”的章节

    function removeDataStore(uuid): Promise<void>

删除具有给定标识符的数据存储。

请注意，在使用此 API 之前，应关闭任何使用此数据存储的 webview。

更多信息请参见 <https://developer.apple.com/documentation/webkit/wkwebsitedatastore>。

#### 参数

名为“参数”的部分

参数| 类型
---|---
`uuid`| [`DataStoreIdentifier`](/reference/javascript/api/namespaceapp/#datastoreidentifier)

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

#### 示例

标题为“Example”的章节

    import { fetchDataStoreIdentifiers, removeDataStore } from '@tauri-apps/api/app';

    for (const id of (await fetchDataStoreIdentifiers())) {

      await removeDataStore(id);

    }

#### 始于

名为“起始版本”的部分

2.4.0

**源文件** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/app.ts#L181>

* * *

### setDockVisibility()

名为“setDockVisibility()”的章节

    function setDockVisibility(visible): Promise<void>

设置 macOS 上应用程序的 dock 可见性。

#### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`visible`| `布尔值 (boolean)`| Dock 是否应该可见。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

#### 示例

标题为“Example”的章节

    import { setDockVisibility } from '@tauri-apps/api/app';

    await setDockVisibility(false);

#### 始于

名为“起始版本”的部分

2.5.0

**源文件** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/app.ts#L236>

* * *

### setTheme()

名为“setTheme()”的章节

    function setTheme(theme?): Promise<void>

设置应用程序的主题。传入 `null` 或 `undefined` 以跟随系统主题。

#### 参数

名为“参数”的部分

参数| 类型
---|---
`主题`?| `null` | [`Theme`](/reference/javascript/api/namespacewindow/#theme-2)

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

#### 示例

标题为“Example”的章节

    import { setTheme } from '@tauri-apps/api/app';

    await setTheme('dark');

平台特定

  * **iOS / Android:** 不支持。

#### 始于

名为“起始版本”的部分

2.0.0

**源文件** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/app.ts#L219>

* * *

### show()

名为“show()”的章节

    function show(): Promise<void>

在 macOS 上显示应用程序。此函数不会自动聚焦任何特定的应用程序窗口。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

#### 示例

标题为“Example”的章节

    import { show } from '@tauri-apps/api/app';

    await show();

#### 始于

名为“起始版本”的部分

1.2.0

**源文件** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/app.ts#L128>