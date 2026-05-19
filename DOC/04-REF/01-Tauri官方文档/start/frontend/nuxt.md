# Nuxt

_Source: https://v2.tauri.org.cn/start/frontend/nuxt/_

Nuxt 是一个 Vue 元框架。访问 <https://nuxtjs.org.cn> 了解更多关于 Nuxt 的信息。本指南适用于 Nuxt 4.2 及更高版本。

## 清单

名为“检查清单”的章节

  * 通过设置 `ssr: false` 使用静态站点生成（SSG）。Tauri 不支持基于服务器的解决方案。
  * 在 `tauri.conf.json` 中使用默认的 `../dist` 作为 `frontendDist`。
  * 使用 `nuxi build` 进行编译。
  * （可选）：通过在 `nuxt.config.ts` 中设置 `telemetry: false` 来禁用遥测功能。

## 示例配置

名为“示例配置”的章节

  1. ##### 更新 Tauri 配置

标题为“更新 Tauri 配置”的章节

     * npm
     * yarn
     * pnpm
     * deno

tauri.conf.json

    {

      "build": {

        "beforeDevCommand": "npm run dev",

        "beforeBuildCommand": "npm run generate",

        "devUrl": "https://:3000",

        "frontendDist": "../dist"

      }

    }

tauri.conf.json

    {

      "build": {

        "beforeDevCommand": "yarn dev",

        "beforeBuildCommand": "yarn generate",

        "devUrl": "https://:3000",

        "frontendDist": "../dist"

      }

    }

tauri.conf.json

    {

      "build": {

        "beforeDevCommand": "pnpm dev",

        "beforeBuildCommand": "pnpm generate",

        "devUrl": "https://:3000",

        "frontendDist": "../dist"

      }

    }

tauri.conf.json

    {

      "build": {

        "beforeDevCommand": "deno task dev",

        "beforeBuildCommand": "deno task generate",

        "devUrl": "https://:3000",

        "frontendDist": "../dist"

      }

    }

  2. ##### 更新 Nuxt 配置

标题为“更新 Nuxt 配置”的章节

         export default defineNuxtConfig({

           compatibilityDate: '2025-05-15',

           // (optional) Enable the Nuxt devtools

           devtools: { enabled: true },

           // Enable SSG

           ssr: false,

           // Enables the development server to be discoverable by other devices when running on iOS physical devices

           devServer: {

             host: '0',

           },

           vite: {

             // Better support for Tauri CLI output

             clearScreen: false,

             // Enable environment variables

             // Additional environment variables can be found at

             // https://v2.tauri.org.cn/reference/environment-variables/

             envPrefix: ['VITE_', 'TAURI_'],

             server: {

               // Tauri requires a consistent port

               strictPort: true,

             },

           },

           // Avoids error [unhandledRejection] EMFILE: too many open files, watch

           ignore: ['**/src-tauri/**'],

         });