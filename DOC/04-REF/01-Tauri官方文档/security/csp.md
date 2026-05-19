# 内容安全策略 (CSP)

_Source: https://v2.tauri.org.cn/security/csp/_

Tauri 会限制 HTML 页面的[内容安全策略 (CSP)](https://mdn.org.cn/en-US/docs/Web/HTTP/CSP)。这可用于减少或防止跨站脚本攻击 (XSS) 等常见基于 Web 的漏洞的影响。

本地脚本会进行哈希处理，样式和外部脚本则使用加密随机数 (nonce) 进行引用，从而防止加载未经允许的内容。

注意

避免加载 CDN 提供的脚本等远程内容，因为它们引入了攻击媒介。通常，任何不受信任的文件都可能引入新的、隐蔽的攻击媒介。

只有在 Tauri 配置文件中设置时，CSP 保护才会启用。你应该尽可能地对其进行严格限制，只允许 Webview 加载来自你信任且最好是由你拥有的主机的资源。在编译时，Tauri 会自动将其随机数和哈希值附加到捆绑代码和资源的相应 CSP 属性中，因此你只需要关注应用程序特有的配置。

这是一个摘自 Tauri [`api`](https://github.com/tauri-apps/tauri/blob/dev/examples/api/src-tauri/tauri.conf.json#L22) 示例的 CSP 配置示例，但每位应用程序开发者都需要根据自己的应用需求对其进行调整。

tauri/examples/api/src-tauri/tauri.conf.json

      "csp": {

            "default-src": "'self' customprotocol: asset:",

            "connect-src": "ipc: http://ipc.localhost",

            "font-src": ["https://fonts.gstatic.com"],

            "img-src": "'self' asset: http://asset.localhost blob: data:",

            "style-src": "'unsafe-inline' 'self' https://fonts.googleapis.ac.cn"

          },

提示

当使用 Rust 开发前端，或者如果你的前端以其他方式使用 WebAssembly 时，请记住将 `'wasm-unsafe-eval'` 添加为 `script-src`。

有关此保护的更多信息，请参阅 [`script-src`](https://mdn.org.cn/en-US/docs/Web/HTTP/Headers/Content-Security-Policy/script-src)、[`style-src`](https://mdn.org.cn/en-US/docs/Web/HTTP/Headers/Content-Security-Policy/style-src) 和 [CSP 源 (Sources)](https://mdn.org.cn/en-US/docs/Web/HTTP/Headers/Content-Security-Policy/Sources#sources)。