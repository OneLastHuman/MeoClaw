# @tauri-apps/plugin-updater

_Source: https://v2.tauri.org.cn/reference/javascript/updater/_

## 类

名为“类”的部分

### Update

名为“Update”的章节

#### 继承 (Extends)

名为“继承自”的章节

  * `Resource`

#### 构造函数

名为“构造函数”的部分

##### new Update()

名为“new Update()”的章节

    new Update(metadata): Update

###### 参数

名为“参数”的部分

参数| 类型
---|---
`metadata`| `UpdateMetadata`

###### 返回

名为“返回值”的部分

[`Update`](/reference/javascript/updater/#update)

###### Overrides

名为“Overrides”的章节

`Resource.constructor`

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/updater/guest-js/index.ts#L69>

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
~~`available`~~| `布尔值 (boolean)`| **已弃用** 该属性始终为 true，使用 [`check`](/reference/javascript/updater/#check) 时，请改为检查返回值是否为 `null`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/updater/guest-js/index.ts#L61>
`body?`| `string`| -| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/updater/guest-js/index.ts#L65>
`currentVersion`| `string`| -| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/updater/guest-js/index.ts#L62>
`date?`| `string`| -| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/updater/guest-js/index.ts#L64>
`rawJson`| [`Record`](https://typescript.net.cn/docs/handbook/utility-types.html#recordkeys-type)<`string`, `unknown`>| -| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/updater/guest-js/index.ts#L66>
`version`| `string`| -| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/updater/guest-js/index.ts#L63>

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

`Resource.rid`

**来源** : 未定义

#### 方法

名为“方法”的部分

##### close()

名为“close()”的部分

    close(): Promise<void>

销毁并从内存中清理此资源。**您不应再对此对象调用任何方法，并应放弃对它的任何引用。**

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

###### Overrides

名为“Overrides”的章节

`Resource.close`

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/updater/guest-js/index.ts#L129>

##### download()

名为“download()”的章节

    download(onEvent?, options?): Promise<void>

下载更新包

###### 参数

名为“参数”的部分

参数| 类型
---|---
`onEvent`?| (`progress`) => `void`
`选项`?| [`DownloadOptions`](/reference/javascript/updater/#downloadoptions)

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/updater/guest-js/index.ts#L80>

##### downloadAndInstall()

名为“downloadAndInstall()”的章节

    downloadAndInstall(onEvent?, options?): Promise<void>

下载更新包并进行安装

###### 参数

名为“参数”的部分

参数| 类型
---|---
`onEvent`?| (`progress`) => `void`
`选项`?| [`DownloadOptions`](/reference/javascript/updater/#downloadoptions)

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/updater/guest-js/index.ts#L113>

##### install()

名为“install()”的章节

    install(): Promise<void>

安装已下载的更新包

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/updater/guest-js/index.ts#L98>

## 接口

名为“接口”的部分

### CheckOptions

名为“CheckOptions”的章节

检查更新时使用的选项

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
`allowDowngrades?`| `布尔值 (boolean)`| 允许降级到旧版本（通过不检查当前版本是否大于可用版本）。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/updater/guest-js/index.ts#L28>
`headers?`| `HeadersInit`| 请求头| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/updater/guest-js/index.ts#L12>
`proxy?`| `string`| 用于检查和下载更新的代理 URL。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/updater/guest-js/index.ts#L20>
`target?`| `string`| 运行中应用程序的目标标识符。这会发送到后端。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/updater/guest-js/index.ts#L24>
`timeout?`| `数字`| 超时时间（毫秒）| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/updater/guest-js/index.ts#L16>

* * *

### DownloadOptions

名为“DownloadOptions”的章节

下载更新时使用的选项

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
`headers?`| `HeadersInit`| 请求头| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/updater/guest-js/index.ts#L36>
`timeout?`| `数字`| 超时时间（毫秒）| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/updater/guest-js/index.ts#L40>

## 类型别名 (Type Aliases)

名为“类型别名”的部分

### DownloadEvent

名为“DownloadEvent”的章节

    type DownloadEvent: object | object | object;

更新程序下载事件

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/updater/guest-js/index.ts#L53>

## 函数

名为“函数”的部分

### check()

名为“check()”的章节

    function check(options?): Promise<Update | null>

检查更新，如果没有可用更新，则解析为 `null`

#### 参数

名为“参数”的部分

参数| 类型
---|---
`选项`?| [`CheckOptions`](/reference/javascript/updater/#checkoptions)

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`Update`](/reference/javascript/updater/#update) | `null`>

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/updater/guest-js/index.ts#L136>