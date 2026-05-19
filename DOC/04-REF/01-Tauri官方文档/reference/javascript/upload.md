# @tauri-apps/plugin-upload

_Source: https://v2.tauri.org.cn/reference/javascript/upload/_

## 枚举

标题为“枚举”的章节

### HttpMethod (HTTP 方法)

标题为“HttpMethod”的章节

#### 枚举成员

标题为“枚举成员”的章节

##### Patch

标题为“Patch”的章节

    Patch: "PATCH";

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/upload/guest-js/index.ts#L19>

##### Post

标题为“Post”的章节

    Post: "POST";

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/upload/guest-js/index.ts#L17>

##### Put

标题为“Put”的章节

    Put: "PUT";

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/upload/guest-js/index.ts#L18>

## 函数

名为“函数”的部分

### download()

标题为“download()”的章节

    function download(

       url,

       filePath,

       progressHandler?,

       headers?,

    body?): Promise<void>

#### 参数

名为“参数”的部分

参数| 类型
---|---
`url`| `string`
`filePath (文件路径)`| `string`
`progressHandler (进度处理器)`?| `ProgressHandler (进度处理器类型)`
`headers (请求头)`?| [`Map`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Map)<`string`, `string`>
`body (请求体)`?| `string`

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/upload/guest-js/index.ts#L53>

* * *

### upload()

标题为“upload()”的章节

    function upload(

       url,

       filePath,

       progressHandler?,

       headers?,

    method?): Promise<string>

#### 参数

名为“参数”的部分

参数| 类型
---|---
`url`| `string`
`filePath (文件路径)`| `string`
`progressHandler (进度处理器)`?| `ProgressHandler (进度处理器类型)`
`headers (请求头)`?| [`Map`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Map)<`string`, `string`>
`method (方法)`?| [`HttpMethod (HTTP 方法)`](/reference/javascript/upload/#httpmethod)

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`string`>

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/upload/guest-js/index.ts#L22>