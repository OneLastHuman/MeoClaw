# HTTP 头

_Source: https://v2.tauri.org.cn/security/http-headers/_

[ 自 `2.1.0` 版本起](https://v2.tauri.org.cn/release/tauri/v2.1.0)

在配置中定义的头部会随响应一同发送给 webview。这不包含 IPC 消息和错误响应。更具体地说，通过 `get_response` 函数在 [crates/tauri/src/protocol/tauri.rs ↗](https://github.com/tauri-apps/tauri/blob/8e8312bb8201ccc609e4bbc1a990bdc314daa00f/crates/tauri/src/protocol/tauri.rs#L103) 中发送的每一个响应都将包含这些头部。

### 头部名称

标题为“头部名称”的章节

头部名称限制为

  * [Access-Control-Allow-Credentials ↗](https://mdn.org.cn/en-US/docs/Web/HTTP/Reference/Headers/Access-Control-Allow-Credentials)
  * [Access-Control-Allow-Headers ↗](https://mdn.org.cn/en-US/docs/Web/HTTP/Reference/Headers/Access-Control-Allow-Headers)
  * [Access-Control-Allow-Methods ↗](https://mdn.org.cn/en-US/docs/Web/HTTP/Reference/Headers/Access-Control-Allow-Methods)
  * [Access-Control-Expose-Headers ↗](https://mdn.org.cn/en-US/docs/Web/HTTP/Reference/Headers/Access-Control-Expose-Headers)
  * [Access-Control-Max-Age ↗](https://mdn.org.cn/en-US/docs/Web/HTTP/Reference/Headers/Access-Control-Max-Age)
  * [Cross-Origin-Embedder-Policy ↗](https://mdn.org.cn/en-US/docs/Web/HTTP/Reference/Headers/Cross-Origin-Embedder-Policy)
  * [Cross-Origin-Opener-Policy ↗](https://mdn.org.cn/en-US/docs/Web/HTTP/Reference/Headers/Cross-Origin-Opener-Policy)
  * [Cross-Origin-Resource-Policy ↗](https://mdn.org.cn/en-US/docs/Web/HTTP/Reference/Headers/Cross-Origin-Resource-Policy)
  * [Permissions-Policy ↗](https://mdn.org.cn/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy)
  * [Service-Worker-Allowed ↗](https://mdn.org.cn/en-US/docs/Web/HTTP/Reference/Headers/Service-Worker-Allowed)
  * [Timing-Allow-Origin ↗](https://mdn.org.cn/en-US/docs/Web/HTTP/Reference/Headers/Timing-Allow-Origin)
  * [X-Content-Type-Options ↗](https://mdn.org.cn/en-US/docs/Web/HTTP/Reference/Headers/X-Content-Type-Options)
  * Tauri-Custom-Header

注意

`Tauri-Custom-Header` 不适用于生产环境。

注意

[内容安全策略 (CSP)](../csp/) 不在此处定义。

### 如何配置头部

标题为“如何配置头部”的章节

  * 使用字符串
  * 使用字符串数组
  * 使用对象/键值对，其中值必须是字符串
  * 使用 null

头部值在实际响应中总是被转换为字符串。根据配置文件的编写方式，某些头部值需要进行组合。以下是组合创建的规则：

  * `string`：在结果头部值中保持不变
  * `array`：各项通过 `, ` 连接以形成结果头部值
  * `key-value`：各项由“键 + 空格 + 值”组成。然后各项通过 `; ` 连接以形成结果头部值
  * `null`：该头部将被忽略

### 示例

标题为“Example”的章节

src-tauri/tauri.conf.json

    {

     //...

      "app":{

        //...

        "security": {

          //...

          "headers": {

            "Cross-Origin-Opener-Policy": "same-origin",

            "Cross-Origin-Embedder-Policy": "require-corp",

            "Timing-Allow-Origin": [

              "https://mdn.org.cn",

              "https://example.com",

            ],

            "X-Content-Type-Options": null, // gets ignored

            "Access-Control-Expose-Headers": "Tauri-Custom-Header",

            "Tauri-Custom-Header": {

              "key1": "'value1' 'value2'",

              "key2": "'value3'"

            }

          },

          // notice how the CSP is not defined under headers

          "csp": "default-src 'self'; connect-src ipc: http://ipc.localhost",

        }

      }

    }

注意

`Tauri-Custom-Header` 不适用于生产环境。对于测试：请记住相应地设置 `Access-Control-Expose-Headers`。

在此示例中，`Cross-Origin-Opener-Policy` 和 `Cross-Origin-Embedder-Policy` 被设置为允许使用 [`SharedArrayBuffer ↗`](https://mdn.org.cn/en-US/docs/Web/JavaScript/Reference/Global_Objects/SharedArrayBuffer)。`Timing-Allow-Origin` 允许从所列网站加载的脚本通过 [资源计时 API (Resource Timing API) ↗](https://mdn.org.cn/en-US/docs/Web/API/Performance_API/Resource_timing) 访问详细的网络计时数据。

对于 helloworld 示例，此配置的结果为

    access-control-allow-origin:  http://tauri.localhost

    access-control-expose-headers: Tauri-Custom-Header

    content-security-policy: default-src 'self'; connect-src ipc: http://ipc.localhost; script-src 'self' 'sha256-Wjjrs6qinmnr+tOry8x8PPwI77eGpUFR3EEGZktjJNs='

    content-type: text/html

    cross-origin-embedder-policy: require-corp

    cross-origin-opener-policy: same-origin

    tauri-custom-header: key1 'value1' 'value2'; key2 'value3'

    timing-allow-origin: https://mdn.org.cn, https://example.com

### 框架

标题为“框架”的章节

某些开发环境需要额外的设置来模拟生产环境。

注意

为了让头部在这些框架中生效，你可能需要在框架的配置文件（用于开发模式）和 Tauri 的配置文件（用于构建模式）中同时定义它们。这是因为：

  * 框架在构建时不会包含其配置文件中定义的头部。
  * Tauri 无法将头部注入到框架的开发服务器中——它只能将头部注入到最终的构建输出中。

#### JavaScript/TypeScript

标题为“JavaScript/TypeScript”的章节

对于运行构建工具 **Vite** 的设置（包括 **Qwik、React、Solid、Svelte 和 Vue** ），请将所需的头部添加到 `vite.config.ts` 中。

vite.config.ts

    import { defineConfig } from 'vite';

    export default defineConfig({

      // ...

      server: {

          // ...

          headers: {

            'Cross-Origin-Opener-Policy': 'same-origin',

            'Cross-Origin-Embedder-Policy': 'require-corp',

            'Timing-Allow-Origin': 'https://mdn.org.cn, https://example.com',

            'Access-Control-Expose-Headers': 'Tauri-Custom-Header',

            'Tauri-Custom-Header': "key1 'value1' 'value2'; key2 'value3'"

          },

        },

    })

有时 `vite.config.ts` 会被集成到框架的配置文件中，但设置方式保持不变。对于 **Angular** ，请将其添加到 `angular.json` 中。

angular.json

    {

      //...

      "projects":{

        //...

        "insert-project-name":{

          //...

          "architect":{

            //...

            "serve":{

              //...

              "options":{

                //...

                "headers":{

                  "Cross-Origin-Opener-Policy": "same-origin",

                  "Cross-Origin-Embedder-Policy": "require-corp",

                  "Timing-Allow-Origin": "https://mdn.org.cn, https://example.com",

                  "Access-Control-Expose-Headers": "Tauri-Custom-Header",

                  "Tauri-Custom-Header": "key1 'value1' 'value2'; key2 'value3'"

                }

              }

            }

          }

        }

      }

    }

对于 **Nuxt** ，请添加到 `nuxt.config.ts` 中。

nuxt.config.ts

    export default defineNuxtConfig({

      //...

      vite: {

        //...

        server: {

          //...

          headers:{

            'Cross-Origin-Opener-Policy': 'same-origin',

            'Cross-Origin-Embedder-Policy': 'require-corp',

            'Timing-Allow-Origin': 'https://mdn.org.cn, https://example.com',

            'Access-Control-Expose-Headers': 'Tauri-Custom-Header',

            'Tauri-Custom-Header': "key1 'value1' 'value2'; key2 'value3'"

          }

        },

      },

    });

**Next.js** 不依赖于 **Vite** ，因此方法有所不同。点击[此处 ↗](https://nextjs.net.cn/docs/pages/api-reference/next-config-js/headers)阅读更多相关信息。头部在 `next.config.js` 中定义。

next.config.js

    module.exports = {

      //...

      async headers() {

        return [

          {

            source: '/*',

            headers: [

              {

                key: 'Cross-Origin-Opener-Policy',

                value: 'same-origin',

              },

              {

                key: 'Cross-Origin-Embedder-Policy',

                value: 'require-corp',

              },

              {

                key: 'Timing-Allow-Origin',

                value: 'https://mdn.org.cn, https://example.com',

              },

              {

                key: 'Access-Control-Expose-Headers',

                value: 'Tauri-Custom-Header',

              },

              {

                key: 'Tauri-Custom-Header',

                value: "key1 'value1' 'value2'; key2 'value3'",

              },

            ],

          },

        ]

      },

    }

#### Rust

名为“Rust”的章节

对于 **Yew** 和 **Leptos** ，请将头部添加到 `Trunk.toml` 中。

Trunk.toml

    #...

    [serve]

    #...

    headers = {

      "Cross-Origin-Opener-Policy" = "same-origin",

      "Cross-Origin-Embedder-Policy" = "require-corp",

      "Timing-Allow-Origin" = "https://mdn.org.cn, https://example.com",

      "Access-Control-Expose-Headers" = "Tauri-Custom-Header",

      "Tauri-Custom-Header" = "key1 'value1' 'value2'; key2 'value3'"

    }