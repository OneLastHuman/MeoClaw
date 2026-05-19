# @tauri-apps/plugin-deep-link

_Source: https://v2.tauri.org.cn/reference/javascript/deep-link/_

## 函数

名为“函数”的部分

### getCurrent()

名为“getCurrent()”的章节

    function getCurrent(): Promise<string[] | null>

获取触发深度链接的当前 URL。在应用加载时使用此方法，以检查您的应用是否通过深度链接启动。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`string`[] | `null`>

#### 示例

标题为“Example”的章节

    import { getCurrent } from '@tauri-apps/plugin-deep-link';

    const urls = await getCurrent();

#### \- **Windows / Linux** : 此函数读取命令行参数并检查是否只有一个值，该值必须是方案与配置值之一匹配的 URL。

标题为“- Windows / Linux: 此函数读取命令行参数并检查是否只有一个值，该值必须是方案与配置值之一匹配的 URL。”的章节

请注意，当使用 [`Self::register`] 动态注册深度链接方案时，您必须手动检查参数。此外，深度链接可能是作为命令行参数提供的，因此您应该检查其格式是否符合您的预期。

#### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/deep-link/guest-js/index.ts#L23>

* * *

### isRegistered()

标题为“isRegistered()”的章节

    function isRegistered(protocol): Promise<boolean>

检查应用是否是指定协议的默认处理程序。

#### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`protocol`| `string`| 不带 `://` 的协议名称。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`boolean`>

#### 示例

标题为“Example”的章节

    import { isRegistered } from '@tauri-apps/plugin-deep-link';

    await isRegistered("my-scheme");

#### \- **macOS / Android / iOS** : 不支持。

标题为“- macOS / Android / iOS: 不支持。”的章节

#### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/deep-link/guest-js/index.ts#L80>

* * *

### onOpenUrl()

标题为“onOpenUrl()”的章节

    function onOpenUrl(handler): Promise<UnlistenFn>

`deep-link://new-url` 事件的辅助函数，用于在应用运行时每次触发协议时运行一个函数。在应用加载时使用 `getCurrent` 来检查您的应用是否通过深度链接启动。

#### 参数

名为“参数”的部分

参数| 类型
---|---
`处理程序`| (`urls`) => `void`

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`UnlistenFn`>

#### 示例

标题为“Example”的章节

    import { onOpenUrl } from '@tauri-apps/plugin-deep-link';

    await onOpenUrl((urls) => { console.log(urls) });

#### \- **Windows / Linux** : 若没有单实例插件，则不支持。操作系统将启动一个新的应用程序实例，并将 URL 作为命令行参数传递。

标题为“- Windows / Linux: 若没有单实例插件，则不支持。操作系统将启动一个新的应用程序实例，并将 URL 作为命令行参数传递。”的章节

#### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/deep-link/guest-js/index.ts#L99>

* * *

### register()

标题为“register()”的章节

    function register(protocol): Promise<null>

将应用注册为指定协议的默认处理程序。

#### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`protocol`| `string`| 不带 `://` 的协议名称。例如，如果您希望您的应用处理 `tauri://` 链接，请使用 `tauri` 作为协议来调用此方法。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`null`>

#### 示例

标题为“Example”的章节

    import { register } from '@tauri-apps/plugin-deep-link';

    await register("my-scheme");

#### \- **macOS / Android / iOS** : 不支持。

标题为“- macOS / Android / iOS: 不支持。”的章节

#### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/deep-link/guest-js/index.ts#L42>

* * *

### unregister()

名为“unregister()”的部分

    function unregister(protocol): Promise<null>

取消注册应用作为指定协议的默认处理程序。

#### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`protocol`| `string`| 不带 `://` 的协议名称。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`null`>

#### 示例

标题为“Example”的章节

    import { unregister } from '@tauri-apps/plugin-deep-link';

    await unregister("my-scheme");

#### \- **macOS / Linux / Android / iOS** : 不支持。

标题为“- macOS / Linux / Android / iOS: 不支持。”的章节

#### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/deep-link/guest-js/index.ts#L61>