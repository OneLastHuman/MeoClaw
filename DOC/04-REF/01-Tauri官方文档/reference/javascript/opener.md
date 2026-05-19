# @tauri-apps/plugin-opener

_Source: https://v2.tauri.org.cn/reference/javascript/opener/_

使用默认应用程序打开文件和 URL。

## 安全

名为“安全”的章节

此 API 具有作用域配置，强制您限制要打开的文件和 URL。

### 限制对 open | `open` API 的访问

标题为“限制对 open | open API 的访问”的部分

在配置对象中，`open: true` 表示 open API 可以与任何 URL 一起使用，因为参数是使用 `^((mailto:\w+)|(tel:\w+)|(https?://\w+)).+` 正则表达式进行验证的。您可以将布尔值更改为字符串来修改该正则表达式，例如 `open: ^https://github.com/`。

## 函数

名为“函数”的部分

### openPath()

标题为“openPath()”的部分

    function openPath(path, openWith?): Promise<void>

使用系统的默认应用程序打开路径，或者使用 openWith 指定的应用程序。

#### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`path`| `string`| 要打开的路径。
`openWith`?| `string`| 用于打开路径的应用程序。如果未指定，则默认为指定路径类型的系统默认应用程序。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

#### 示例

标题为“Example”的章节

    import { openPath } from '@tauri-apps/plugin-opener';

    // opens a file using the default program:

    await openPath('/path/to/file');

    // opens a file using `vlc` command on Windows.

    await openPath('C:/path/to/file', 'vlc');

#### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/opener/guest-js/index.ts#L71>

* * *

### openUrl()

标题为“openUrl()”的部分

    function openUrl(url, openWith?): Promise<void>

使用系统的默认应用程序打开 URL，或者使用 openWith 指定的应用程序。

#### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`url`| `string` | [`URL`](https://mdn.org.cn/docs/Web/API/URL)| 要打开的 URL。
`openWith`?| `string`| 用于打开 URL 的应用程序。如果未指定，则默认为指定 URL 类型的系统默认应用程序。在移动端上，`openWith` 可以提供为 `inAppBrowser`，以便在应用内浏览器中打开 URL。否则，它将在系统默认浏览器中打开 URL。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

#### 示例

标题为“Example”的章节

    import { openUrl } from '@tauri-apps/plugin-opener';

    // opens the given URL on the default browser:

    await openUrl('https://github.com/tauri-apps/tauri');

    // opens the given URL using `firefox`:

    await openUrl('https://github.com/tauri-apps/tauri', 'firefox');

#### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/opener/guest-js/index.ts#L42>

* * *

### revealItemInDir()

标题为“revealItemInDir()”的部分

    function revealItemInDir(path): Promise<void>

使用系统的默认文件管理器显示路径。

平台特定

  * **Android / iOS:** 不支持。

#### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`path`| `string` | `string`[]| 要显示的路径。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

#### 示例

标题为“Example”的章节

    import { revealItemInDir } from '@tauri-apps/plugin-opener';

    await revealItemInDir('/path/to/file');

    await revealItemInDir([ '/path/to/file', '/path/to/another/file' ]);

#### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/opener/guest-js/index.ts#L96>