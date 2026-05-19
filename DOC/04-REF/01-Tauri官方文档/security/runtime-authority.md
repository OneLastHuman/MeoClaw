# 运行时权限

_Source: https://v2.tauri.org.cn/security/runtime-authority/_

运行时权限（Runtime authority）是 Tauri Core 的一部分。它在运行时持有所有的权限（permissions）、功能（capabilities）和作用域（scopes），用于强制执行哪些窗口可以访问哪些命令，并将作用域传递给这些命令。

每当从 webview 调用 Tauri 命令时，运行时权限组件会接收到该调用请求，确认发起调用的源（origin）是否真正被允许使用所请求的命令，检查该源是否属于相关功能（capabilities），如果为该命令定义了作用域且适用，则会将这些作用域注入到调用请求中，然后再将其传递给对应的 Tauri 命令。

如果该源不允许调用此命令，运行时权限组件将拒绝该请求，因此该 Tauri 命令永远不会被执行。

![IPC Diagram](/_astro/runtime-authority.97JqQbdT_cKpwH.svg)