# @tauri-apps/api

_Source: https://v2.tauri.org.cn/reference/javascript/api/_

Tauri API 允许你与后端层进行交互。

该模块将所有其他模块暴露为一个对象，其中键为模块名称，值为模块的导出内容。

## 示例

名为“示例”的章节

    import { event, window, path } from '@tauri-apps/api'

### 原生 JS API

名为“原生 JS API”的章节

上述导入语法适用于带有打包工具的 JavaScript/TypeScript。如果你使用的是原生 JavaScript，则可以使用全局的 `window.__TAURI__` 对象。这需要启用 `app.withGlobalTauri` 配置选项。

    const { event, window: tauriWindow, path } = window.__TAURI__;

## 命名空间

名为“命名空间”的章节

  * [app](/reference/javascript/api/namespaceapp/)
  * [core](/reference/javascript/api/namespacecore/)
  * [dpi](/reference/javascript/api/namespacedpi/)
  * [event](/reference/javascript/api/namespaceevent/)
  * [image](/reference/javascript/api/namespaceimage/)
  * [menu](/reference/javascript/api/namespacemenu/)
  * [mocks](/reference/javascript/api/namespacemocks/)
  * [path](/reference/javascript/api/namespacepath/)
  * [tray](/reference/javascript/api/namespacetray/)
  * [webview](/reference/javascript/api/namespacewebview/)
  * [webviewWindow](/reference/javascript/api/namespacewebviewwindow/)
  * [window](/reference/javascript/api/namespacewindow/)