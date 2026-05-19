# SvelteKit

_Source: https://v2.tauri.org.cn/start/frontend/sveltekit/_

SvelteKit 是 Svelte 的元框架。在 <https://svelte.net.cn/> 了解更多关于 SvelteKit 的信息。本指南适用于 SvelteKit 2.20.4 / Svelte 5.25.8 版本。

## 清单

名为“检查清单”的章节

  * 通过 `static-adapter` 使用 [SSG](https://svelte.net.cn/docs/kit/adapter-static) 和 [SPA](https://svelte.net.cn/docs/kit/single-page-apps) 模式。Tauri 不支持基于服务器的解决方案。
  * 如果在使用 SSG **且开启预渲染（prerendering）** 时，请注意 `load` 函数在应用构建过程中将无法访问 Tauri API。建议使用 SPA 模式（不开启预渲染），因为此时 load 函数仅在具备 Tauri API 访问权限的 webview 中运行。
  * 在 `tauri.conf.json` 中将 `build/` 设置为 `frontendDist`。

## 示例配置

名为“示例配置”的章节

  1. ##### 安装 `@sveltejs/adapter-static`

标题为“安装 @sveltejs/adapter-static”的章节

     * npm
     * yarn
     * pnpm
     * deno

    npm install --save-dev @sveltejs/adapter-static

    yarn add -D @sveltejs/adapter-static

    pnpm add -D @sveltejs/adapter-static

    deno add -D npm:@sveltejs/adapter-static

  2. ##### 更新 Tauri 配置

标题为“更新 Tauri 配置”的章节

     * npm
     * yarn
     * pnpm
     * deno

tauri.conf.json

    {

      "build": {

        "beforeDevCommand": "npm run dev",

        "beforeBuildCommand": "npm run build",

        "devUrl": "https://:5173",

        "frontendDist": "../build"

      }

    }

tauri.conf.json

    {

      "build": {

        "beforeDevCommand": "yarn dev",

        "beforeBuildCommand": "yarn build",

        "devUrl": "https://:5173",

        "frontendDist": "../build"

      }

    }

tauri.conf.json

    {

      "build": {

        "beforeDevCommand": "pnpm dev",

        "beforeBuildCommand": "pnpm build",

        "devUrl": "https://:5173",

        "frontendDist": "../build"

      }

    }

tauri.conf.json

    {

      "build": {

        "beforeDevCommand": "deno task dev",

        "beforeBuildCommand": "deno task build",

        "devUrl": "https://:5173",

        "frontendDist": "../build"

      }

    }

  3. ##### 更新 SvelteKit 配置

标题为“更新 SvelteKit 配置：”的章节

svelte.config.js

         import adapter from '@sveltejs/adapter-static';

         import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

         /** @type {import('@sveltejs/kit').Config} */

         const config = {

           // Consult https://svelte.net.cn/docs/kit/integrations#preprocessors

           // for more information about preprocessors

           preprocess: vitePreprocess(),

           kit: {

             adapter: adapter({

               fallback: 'index.html',

             }),

           },

         };

         export default config;

  4. ##### 禁用 SSR

标题为“禁用 SSR”的章节

最后，我们需要通过添加一个根目录下的 `+layout.ts` 文件（如果你不使用 TypeScript，则为 `+layout.js`）来禁用 SSR，文件内容如下：

src/routes/+layout.ts

         export const ssr = false;

请注意，`static-adapter` 并不强制要求你为整个应用禁用 SSR，但这样做可以让你使用依赖于全局 window 对象（如 Tauri 的 API）的接口，而无需进行 [客户端检查](https://svelte.net.cn/docs/kit/faq#how-do-i-use-x-with-sveltekit-how-do-i-use-a-client-side-only-library-that-depends-on-document-or-window)。

此外，如果你更倾向于使用静态网站生成（SSG）而非单页应用（SPA）模式，可以根据 [适配器文档](https://svelte.net.cn/docs/kit/adapter-static) 修改适配器配置和 `+layout.ts`。