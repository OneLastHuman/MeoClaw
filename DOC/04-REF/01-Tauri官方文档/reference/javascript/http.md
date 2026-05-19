# @tauri-apps/plugin-http

_Source: https://v2.tauri.org.cn/reference/javascript/http/_

使用 Rust 后端进行 HTTP 请求。

## 安全

名为“安全”的章节

此 API 具有作用域配置，强制您使用 glob 模式限制可以访问的 URL。

例如，此作用域配置仅允许向 `tauri.app` 的所有子域发起 HTTP 请求，但 `https://private.tauri.app` 除外

    {

      "permissions": [

        {

          "identifier": "http:default",

          "allow": [{ "url": "https://*.tauri.app" }],

          "deny": [{ "url": "https://private.tauri.app" }]

        }

      ]

    }

尝试执行任何 URL 未在作用域中配置的 API，将由于访问被拒绝导致 Promise 被拒绝。

## 接口

名为“接口”的部分

### ClientOptions

名为“ClientOptions”的章节

用于配置执行 fetch 请求的 Rust 客户端的选项

#### 始于

名为“起始版本”的部分

2.0.0

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
`connectTimeout?`| `数字`| 超时时间（毫秒）| **源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/http/guest-js/index.ts#L82>
`danger?`| [`DangerousSettings`](/reference/javascript/http/#dangeroussettings)| 客户端危险设置的配置，例如禁用 SSL 验证。| **源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/http/guest-js/index.ts#L90>
`maxRedirections?`| `数字`| 定义客户端应跟随的最大重定向次数。如果设置为 0，则不跟随任何重定向。| **源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/http/guest-js/index.ts#L80>
`proxy?`| [`Proxy`](/reference/javascript/http/#proxy-1)| 客户端应将请求转发到的代理配置。| **源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/http/guest-js/index.ts#L86>

* * *

### DangerousSettings

名为“DangerousSettings”的章节

客户端危险设置的配置，例如禁用 SSL 验证。

#### 始于

名为“起始版本”的部分

2.3.0

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
`acceptInvalidCerts?`| `布尔值 (boolean)`| 禁用 SSL 验证。| **源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/http/guest-js/index.ts#L102>
`acceptInvalidHostnames?`| `布尔值 (boolean)`| 禁用主机名验证。| **源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/http/guest-js/index.ts#L106>

* * *

### Proxy

名为“Proxy”的章节

客户端应将请求转发到的代理配置。

#### 始于

名为“起始版本”的部分

2.0.0

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
`all?`| `string` | [`ProxyConfig`](/reference/javascript/http/#proxyconfig)| 将所有流量代理到指定的 URL。| **源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/http/guest-js/index.ts#L40>
`http?`| `string` | [`ProxyConfig`](/reference/javascript/http/#proxyconfig)| 将所有 HTTP 流量代理到指定的 URL。| **源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/http/guest-js/index.ts#L44>
`https?`| `string` | [`ProxyConfig`](/reference/javascript/http/#proxyconfig)| 将所有 HTTPS 流量代理到指定的 URL。| **源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/http/guest-js/index.ts#L48>

* * *

### ProxyConfig

名为“ProxyConfig”的章节

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
`basicAuth?`| `对象`| 使用基础认证 (Basic auth) 设置 `Proxy-Authorization` 请求头。| **源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/http/guest-js/index.ts#L59>
`basicAuth.password`| `string`| -| **源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/http/guest-js/index.ts#L61>
`basicAuth.username`| `string`| -| **源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/http/guest-js/index.ts#L60>
`noProxy?`| `string`| 用于过滤不需要进行代理的请求的配置。条目应以逗号分隔（条目之间的空格会被忽略）| **源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/http/guest-js/index.ts#L67>
`url`| `string`| 代理服务器的 URL。| **源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/http/guest-js/index.ts#L55>

## 函数

名为“函数”的部分

### fetch()

名为“fetch()”的章节

    function fetch(input, init?): Promise<Response>

从网络获取资源。它返回一个 `Promise`，解析为该 `Request` 的 `Response`，无论请求是否成功。

#### 参数

名为“参数”的部分

参数| 类型
---|---
`input`| `string` | [`URL`](https://mdn.org.cn/docs/Web/API/URL) | [`Request`](https://mdn.org.cn/docs/Web/API/Request)
`init`?| `RequestInit` & [`ClientOptions`](/reference/javascript/http/#clientoptions)

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`Response`](https://mdn.org.cn/docs/Web/API/Response)>

#### 示例

标题为“Example”的章节

    const response = await fetch("http://my.json.host/data.json");

    console.log(response.status);  // e.g. 200

    console.log(response.statusText); // e.g. "OK"

    const jsonData = await response.json();

#### 始于

名为“起始版本”的部分

2.0.0

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/http/guest-js/index.ts#L125>