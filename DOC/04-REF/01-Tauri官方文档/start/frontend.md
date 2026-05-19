# 前端配置

_Source: https://v2.tauri.org.cn/start/frontend/_

Tauri 与前端框架无关，并且开箱即用地支持大多数前端框架。然而，有时框架需要进行一点额外的配置才能与 Tauri 集成。以下是具有推荐配置的框架列表。

如果某个框架未列出，则它可能无需额外配置即可与 Tauri 一起使用，或者尚未记录在案。欢迎通过贡献来添加可能需要额外配置的框架，以帮助 Tauri 社区中的其他人。

## 配置核对清单

标题为“配置核对清单”的章节

从概念上讲，Tauri 充当静态 Web 主机。您需要为 Tauri 提供一个包含 HTML、CSS、Javascript 以及可能的 WASM 的文件夹，这些内容可以提供给 Tauri 提供的 Webview。

以下是将前端与 Tauri 集成时所需的常见场景清单：

  * 使用静态站点生成（SSG）、单页应用（SPA）或经典的多页应用（MPA）。Tauri 不原生支持基于服务器的替代方案（例如 SSR）。
  * 对于移动开发，必须有一种开发服务器，能够在其内部 IP 上托管前端。
  * 在您的应用程序与 API 之间使用妥当的客户端-服务器关系（不要使用带有 SSR 的混合解决方案）。

## JavaScript

名为“JavaScript”的章节

对于大多数项目，我们推荐将 [Vite](https://vite.org.cn/) 用于 React、Vue、Svelte 和 Solid 等 SPA 框架，同时也适用于纯 JavaScript 或 TypeScript 项目。此处列出的大多数其他指南展示了如何使用元框架，因为它们通常是为 SSR 设计的，因此需要特殊的配置。

[ Next.js ](/start/frontend/nextjs/)

[ Nuxt ](/start/frontend/nuxt/)

[ Qwik ](/start/frontend/qwik/)

[ SvelteKit ](/start/frontend/sveltekit/)

[ Vite（推荐） ](/start/frontend/vite/)

## Rust

名为“Rust”的章节

[ Leptos ](/start/frontend/leptos/)

[ Trunk ](/start/frontend/trunk/)

框架未列出？

没有看到列出的框架？它可能无需任何额外配置即可与 Tauri 一起使用。请阅读[配置核对清单](/start/frontend/#configuration-checklist)，以查看需要检查的常见配置。