# Next.js

_Source: https://v2.tauri.org.cn/start/frontend/nextjs/_

Next.js 是一个 React 元框架。欲了解更多关于 Next.js 的信息，请访问 <https://nextjs.net.cn>。本指南适用于 Next.js 14.2.3 版本。

## 清单

名为“检查清单”的章节

  * 通过设置 `output: 'export'` 使用静态导出。Tauri 不支持基于服务器的解决方案。
  * 在 `tauri.conf.json` 中将 `out` 目录用作 `frontendDist`。

## 示例配置

名为“示例配置”的章节

  1. ##### 更新 Tauri 配置

标题为“更新 Tauri 配置”的章节

     * npm
     * yarn
     * pnpm
     * deno

src-tauri/tauri.conf.json

    {

      "build": {

        "beforeDevCommand": "npm run dev",

        "beforeBuildCommand": "npm run build",

        "devUrl": "https://:3000",

        "frontendDist": "../out"

      }

    }

src-tauri/tauri.conf.json

    {

      "build": {

        "beforeDevCommand": "yarn dev",

        "beforeBuildCommand": "yarn build",

        "devUrl": "https://:3000",

        "frontendDist": "../out"

      }

    }

src-tauri/tauri.conf.json

    {

      "build": {

        "beforeDevCommand": "pnpm dev",

        "beforeBuildCommand": "pnpm build",

        "devUrl": "https://:3000",

        "frontendDist": "../out"

      }

    }

src-tauri/tauri.conf.json

    {

      "build": {

        "beforeDevCommand": "deno task dev",

        "beforeBuildCommand": "deno task build",

        "devUrl": "https://:3000",

        "frontendDist": "../out"

      }

    }

  2. ##### 更新 Next.js 配置

标题为“更新 Next.js 配置”的部分

next.conf.mjs

         const isProd = process.env.NODE_ENV === 'production';

         const internalHost = process.env.TAURI_DEV_HOST || 'localhost';

         /** @type {import('next').NextConfig} */

         const nextConfig = {

           // Ensure Next.js uses SSG instead of SSR

           // https://nextjs.net.cn/docs/pages/building-your-application/deploying/static-exports

           output: 'export',

           // Note: This feature is required to use the Next.js Image component in SSG mode.

           // See https://nextjs.net.cn/docs/messages/export-image-api for different workarounds.

           images: {

             unoptimized: true,

           },

           // Configure assetPrefix or else the server won't properly resolve your assets.

           assetPrefix: isProd ? undefined : `http://${internalHost}:3000`,

         };

         export default nextConfig;

  3. ##### 更新 package.json 配置

标题为“更新 package.json 配置”的部分

         "scripts": {

           "dev": "next dev",

           "build": "next build",

           "start": "next start",

           "lint": "next lint",

           "tauri": "tauri"

         }