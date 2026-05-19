# 隔离模式

_Source: https://v2.tauri.org.cn/concept/inter-process-communication/isolation/_

隔离模式是一种通过 JavaScript 拦截并修改前端发送给 Tauri Core 的 Tauri API 消息的方法。由隔离模式注入的安全 JavaScript 代码被称为“隔离应用程序”。

## 原因

标题为“原因”的章节

隔离模式的目的是为开发者提供一种机制，帮助保护其应用程序免受前端对 Tauri Core 的不必要或恶意调用。隔离模式的需求源于运行在前端的不可信内容所带来的威胁，这在拥有大量依赖项的应用程序中是一个常见问题。请参阅 [安全：威胁模型](/security/lifecycle/)，了解应用程序可能面临的多种威胁源列表。

上述所描述的隔离模式旨在解决的最大威胁模型是“开发威胁”。不仅许多前端构建工具由数十（甚至数百）个通常嵌套很深的依赖项组成，复杂的应用程序也可能拥有大量（通常同样嵌套很深）被打包到最终输出中的依赖项。

## 何时使用

标题为“何时使用”的章节

Tauri 强烈建议在任何可以使用的场景下采用隔离模式。由于隔离应用程序会拦截来自前端的 _**所有**_ 消息，因此它 _总是_ 可以被使用。

Tauri 还强烈建议您在每次使用外部 Tauri API 时锁定您的应用程序。作为开发者，您可以利用安全的隔离应用程序来尝试验证 IPC 输入，确保它们在预期的参数范围内。例如，您可能希望检查文件读取或写入调用是否未尝试访问应用程序预期位置之外的路径。另一个例子是确保 Tauri API 的 HTTP 请求调用仅将 Origin 标头设置为应用程序预期的值。

话虽如此，由于它会拦截来自前端的 _**所有**_ 消息，因此即使是像 [事件 (Events)](/reference/javascript/api/namespaceevent/) 这样常驻的 API 也能正常工作。由于某些事件可能会导致您的 Rust 代码执行操作，因此对它们也可以使用相同的验证技术。

## 如何使用

标题为“如何使用”的章节

隔离模式的核心在于在您的前端和 Tauri Core 之间注入一个安全的应用程序，以拦截并修改传入的 IPC 消息。它通过利用 `<iframe>` 的沙箱功能，在主前端应用程序旁安全地运行 JavaScript。Tauri 在加载页面时强制执行隔离模式，强制所有指向 Tauri Core 的 IPC 调用首先路由通过沙箱化的隔离应用程序。一旦消息准备好传递给 Tauri Core，它就会使用浏览器提供的 [SubtleCrypto](https://mdn.org.cn/en-US/docs/Web/API/SubtleCrypto) 实现进行加密，并传回主前端应用程序。到达那里后，它被直接传递给 Tauri Core，随后在内部解密并像往常一样读取。

为了确保不会有人手动读取特定版本应用程序的密钥并利用该密钥在消息加密后进行修改，每次运行应用程序时都会生成新的密钥。

### IPC 消息的近似处理步骤

标题为“IPC 消息的近似处理步骤”的章节

为了便于理解，以下是采用隔离模式时，IPC 消息发送至 Tauri Core 的近似步骤列表：

  1. Tauri 的 IPC 处理程序接收消息
  2. IPC 处理程序 -> 隔离应用程序
  3. `[沙箱]` 隔离应用程序钩子 (hook) 运行并可能修改该消息
  4. `[沙箱]` 消息使用运行时生成的密钥通过 AES-GCM 进行加密
  5. `[加密]` 隔离应用程序 -> IPC 处理程序
  6. `[加密]` IPC 处理程序 -> Tauri Core

_注：箭头 (- >) 表示消息传递过程。_

### 性能影响

标题为“性能影响”的章节

由于消息确实会进行加密，与 [Brownfield 模式](/concept/inter-process-communication/brownfield/) 相比，会有额外的开销，即使安全隔离应用程序未执行任何操作。除了对性能敏感的应用程序（通常维护着少量且经过仔细管理的依赖项以保持性能）外，大多数应用程序应该不会感知到加密/解密 IPC 消息带来的运行成本，因为这些消息相对较小，且 AES-GCM 速度相对较快。如果您不熟悉 AES-GCM，只需了解它是 [SubtleCrypto](https://mdn.org.cn/en-US/docs/Web/API/SubtleCrypto) 中唯一包含的认证模式算法，并且您可能每天都在通过 [TLS](https://en.wikipedia.org/wiki/Transport_Layer_Security) 在后台使用它。

此外，Tauri 应用程序每次启动时都会生成一个加密安全的密钥。如果系统拥有足够的熵来立即返回足量的随机数，那么通常不会感觉到延迟；这在桌面环境中非常常见。如果是在无头 (headless) 环境中运行以进行某些 [WebDriver 集成测试](/develop/tests/webdriver/)，而您的操作系统未包含熵生成服务，则您可能需要安装如 `haveged` 之类的服务。Linux 5.6（2020 年 3 月）现在包含使用投机执行 (speculative execution) 的熵生成功能。

### 局限性

标题为“限制”的章节

隔离模式由于平台差异存在一些限制。最显著的限制是由于外部文件在 Windows 的沙箱 `<iframe>` 内无法正确加载。因此，我们在构建时实现了一个简单的脚本内联步骤，将相对于隔离应用程序的脚本内容以内联方式注入。这意味着典型的打包或简单的文件包含（如 `<script src="index.js"></script>`）仍然可以正常工作，但像 ES Modules 这样的较新机制将 _无法_ 成功加载。

## 建议

标题为“建议”的章节

由于隔离应用程序的意义在于防御开发威胁，我们强烈建议保持隔离应用程序尽可能简单。您不仅应努力将隔离应用程序的依赖项保持在最低限度，还应考虑将所需的构建步骤保持在最低限度。这样，除了您的前端应用程序外，您就不必担心针对隔离应用程序的供应链攻击。

## 创建隔离应用程序

标题为“创建隔离应用程序”的章节

在此示例中，我们将创建一个简单的“Hello World”式隔离应用程序，并将其关联到一个假设现有的 Tauri 应用程序中。它不会对通过它的消息进行任何验证，只会将内容打印到 WebView 控制台。

为了本示例，假设我们位于与 `tauri.conf.json` 同一目录下。现有的 Tauri 应用程序将其 `frontendDist` 设置为 `../dist`。

`../dist-isolation/index.html`:

    <!doctype html>

    <html lang="en">

      <head>

        <meta charset="UTF-8" />

        <title>Isolation Secure Script</title>

      </head>

      <body>

        <script src="index.js"></script>

      </body>

    </html>

`../dist-isolation/index.js`:

    window.__TAURI_ISOLATION_HOOK__ = (payload) => {

      // let's not verify or modify anything, just print the content from the hook

      console.log('hook', payload);

      return payload;

    };

现在，我们要做的就是设置我们的 `tauri.conf.json` 配置 以使用隔离模式，并从 [Brownfield 模式](/concept/inter-process-communication/brownfield/) 完成向隔离模式的引导。

## 配置

名为“配置”的章节

假设我们的主要前端 `frontendDist` 设置为 `../dist`。我们将隔离应用程序输出到 `../dist-isolation`。

    {

      "build": {

        "frontendDist": "../dist"

      },

      "app": {

        "security": {

          "pattern": {

            "use": "isolation",

            "options": {

              "dir": "../dist-isolation"

            }

          }

        }

      }

    }