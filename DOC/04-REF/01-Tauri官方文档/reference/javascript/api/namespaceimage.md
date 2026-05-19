# image

_Source: https://v2.tauri.org.cn/reference/javascript/api/namespaceimage/_

## 类

名为“类”的部分

### 图像

名为“Image”的章节

以行优先顺序从上到下排列的 RGBA 图像。

#### 继承 (Extends)

名为“继承自”的章节

  * [`Resource`](/reference/javascript/api/namespacecore/#resource)

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

##### rgba()

名为“rgba()”的章节

    rgba(): Promise<Uint8Array>

返回该图像的 RGBA 数据，以行优先顺序从上到下排列。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`Uint8Array`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array)>

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/image.ts#L89>

##### size()

名为“size()”的章节

    size(): Promise<ImageSize>

返回该图像的尺寸。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`ImageSize`](/reference/javascript/api/namespaceimage/#imagesize)>

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/image.ts#L96>

##### fromBytes()

名为“fromBytes()”的章节

    static fromBytes(bytes): Promise<Image>

通过推断文件格式，使用提供的字节创建新图像。如果已知格式，请优先使用 [@link Image.fromPngBytes] 或 [@link Image.fromIcoBytes]。

仅支持 `ico` 和 `png`（取决于激活的功能标志）。

请注意，使用此 API 需要 `image-ico` 或 `image-png` Cargo 功能。要启用它，请修改您的 Cargo.toml 文件

    [dependencies]

    tauri = { version = "...", features = ["...", "image-png"] }

###### 参数

名为“参数”的部分

参数| 类型
---|---
`bytes`| [`Uint8Array`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array) | `number`[] | [`ArrayBuffer`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer)

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`Image`](/reference/javascript/api/namespaceimage/#image)>

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/image.ts#L62>

##### fromPath()

名为“fromPath()”的章节

    static fromPath(path): Promise<Image>

使用提供的路径创建新图像。

仅支持 `ico` 和 `png`（取决于激活的功能标志）。

请注意，使用此 API 需要 `image-ico` 或 `image-png` Cargo 功能。要启用它，请修改您的 Cargo.toml 文件

    [dependencies]

    tauri = { version = "...", features = ["...", "image-png"] }

###### 参数

名为“参数”的部分

参数| 类型
---|---
`path`| `string`

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`Image`](/reference/javascript/api/namespaceimage/#image)>

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/image.ts#L82>

##### new()

名为“new()”的章节

    static new(

       rgba,

       width,

    height): Promise<Image>

使用 RGBA 数据（按行优先顺序从上到下）以及指定的宽度和高度创建新图像。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`rgba`| [`Uint8Array`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array) | `number`[] | [`ArrayBuffer`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer)
`宽度`| `数字`
`高度`| `数字`

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`Image`](/reference/javascript/api/namespaceimage/#image)>

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/image.ts#L37>

## 接口

名为“接口”的部分

### ImageSize

名为“ImageSize”的章节

#### 属性

名为“属性”的部分

属性| 类型| 定义于
---|---|---
`height`| `数字`| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/image.ts#L13>
`width`| `数字`| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/image.ts#L11>

## 类型别名 (Type Aliases)

名为“类型别名”的部分

### MenuIcon

名为“MenuIcon”的章节

    type MenuIcon:

      | NativeIcon

      | string

      | Image

      | Uint8Array

      | ArrayBuffer

      | number[];

一种表示可在菜单项中使用的图标的类型。

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/image.ts#L17>

## 函数

名为“函数”的部分

### transformImage()

名为“transformImage()”的章节

    function transformImage<T>(image): T

将图像从各种类型转换为 Rust 可接受的类型。

有关详细信息，请参阅 [tauri::image::JsImage](https://docs.rs/tauri/2/tauri/image/enum.JsImage.html)。请注意，API 签名尚不稳定，可能会发生变化。

#### 类型参数

名为“类型参数”的部分

类型参数
---
`T`

#### 参数

名为“参数”的部分

参数| 类型
---|---
`image`| | `null` | `string` | [`Uint8Array`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array) | `number`[] | [`ArrayBuffer`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer) | [`Image`](/reference/javascript/api/namespaceimage/#image)

#### 返回

名为“返回值”的部分

`T`

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/image.ts#L107>