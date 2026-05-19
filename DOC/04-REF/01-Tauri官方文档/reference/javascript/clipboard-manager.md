# @tauri-apps/plugin-clipboard-manager

_Source: https://v2.tauri.org.cn/reference/javascript/clipboard-manager/_

读取和写入系统剪贴板。

## 函数

名为“函数”的部分

### clear()

“clear()” 章节

    function clear(): Promise<void>

清除剪贴板。

平台特定

  * **Android：** 仅在 SDK 28+ 上受支持。对于较旧的版本，我们会改为向剪贴板写入一个空字符串。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

#### 示例

标题为“Example”的章节

    import { clear } from '@tauri-apps/plugin-clipboard-manager';

    await clear();

#### 始于

名为“起始版本”的部分

2.0.0

**源文件** ： <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/clipboard-manager/guest-js/index.ts#L147>

* * *

### readImage()

“readImage()” 章节

    function readImage(): Promise<Image>

以 Uint8Array 图像格式获取剪贴板内容。

平台特定

  * **Android / iOS：** 不支持。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`Image`>

#### 示例

标题为“Example”的章节

    import { readImage } from '@tauri-apps/plugin-clipboard-manager';

    const clipboardImage = await readImage();

    const blob = new Blob([await clipboardImage.rgba()], { type: 'image' })

    const url = URL.createObjectURL(blob)

#### 始于

名为“起始版本”的部分

2.0.0

**源文件** ： <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/clipboard-manager/guest-js/index.ts#L99>

* * *

### readText()

“readText()” 章节

    function readText(): Promise<string>

以纯文本格式获取剪贴板内容。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`string`>

#### 示例

标题为“Example”的章节

    import { readText } from '@tauri-apps/plugin-clipboard-manager';

    const clipboardText = await readText();

#### 始于

名为“起始版本”的部分

2.0.0

**源文件** ： <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/clipboard-manager/guest-js/index.ts#L46>

* * *

### writeHtml()

“writeHtml()” 章节

    function writeHtml(html, altText?): Promise<void>

  * 写入 HTML，或回退到将提供的纯文本写入剪贴板。

平台特定

  * **Android / iOS：** 不支持。

#### 参数

名为“参数”的部分

参数| 类型
---|---
`html`| `string`
`altText`?| `string`

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

#### 示例

标题为“Example”的章节

    import { writeHtml } from '@tauri-apps/plugin-clipboard-manager';

    await writeHtml('<h1>Tauri is awesome!</h1>', 'plaintext');

    // The following will write "<h1>Tauri is awesome</h1>" as plain text

    await writeHtml('<h1>Tauri is awesome!</h1>', '<h1>Tauri is awesome</h1>');

    // we can read html data only as a string so there's just readText(), no readHtml()

    assert(await readText(), '<h1>Tauri is awesome!</h1>');

#### 始于

名为“起始版本”的部分

2.0.0

**源文件** ： <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/clipboard-manager/guest-js/index.ts#L126>

* * *

### writeImage()

“writeImage()” 章节

    function writeImage(image): Promise<void>

将图像缓冲区写入剪贴板。

平台特定

  * **Android / iOS：** 不支持。

#### 参数

名为“参数”的部分

参数| 类型
---|---
`image`| | `string` | `number`[] | [`ArrayBuffer`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer) | [`Uint8Array`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array) | `Image`

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

#### 示例

标题为“Example”的章节

    import { writeImage } from '@tauri-apps/plugin-clipboard-manager';

    const buffer = [

      // A red pixel

      255, 0, 0, 255,

     // A green pixel

      0, 255, 0, 255,

    ];

    await writeImage(buffer);

#### 始于

名为“起始版本”的部分

2.0.0

**源文件** ： <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/clipboard-manager/guest-js/index.ts#L74>

* * *

### writeText()

“writeText()” 章节

    function writeText(text, opts?): Promise<void>

将纯文本写入剪贴板。

#### 参数

名为“参数”的部分

参数| 类型
---|---
`text`| `string`
`opts`?| `对象`
`opts.label`?| `string`

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

#### 示例

标题为“Example”的章节

    import { writeText, readText } from '@tauri-apps/plugin-clipboard-manager';

    await writeText('Tauri is awesome!');

    assert(await readText(), 'Tauri is awesome!');

#### 始于

名为“起始版本”的部分

2.0.0

**源文件** ： <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/clipboard-manager/guest-js/index.ts#L27>