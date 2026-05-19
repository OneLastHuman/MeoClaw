# 棕色地带模式

_Source: https://v2.tauri.org.cn/concept/inter-process-communication/brownfield/_

_**这是默认模式。**_

这是将 Tauri 与现有前端项目结合使用时，最简单且最直接的模式，因为它尽可能地保持与现有前端项目的兼容性。简而言之，它不需要在现有 Web 前端所使用的技术栈之外添加任何额外内容。并非所有在现有浏览器应用程序中能运行的代码都能在无需修改的情况下直接运行。

如果您不熟悉棕地 (Brownfield) 软件开发，可以查阅 [维基百科关于棕地开发的条目](https://en.wikipedia.org/wiki/Brownfield_\(software_development\))，其中有很好的总结。对于 Tauri 而言，现有的软件环境指的是当前的浏览器支持和行为，而非遗留系统。

## 配置

名为“配置”的章节

由于棕地模式是默认模式，因此无需设置任何配置选项。若要显式设置，您可以在 `tauri.conf.json` 配置文件中使用 `app > security > pattern` 对象进行配置。

    {

      "app": {

        "security": {

          "pattern": {

            "use": "brownfield"

          }

        }

      }

    }

_**棕地模式没有额外的配置选项。**_