# Vite

_Source: https://v2.tauri.org.cn/start/frontend/vite/_

Vite 是一个旨在为现代 Web 项目提供更快、更精简开发体验的构建工具。本指南适用于 Vite 5.4.8 及以上版本。

## 清单

名为“检查清单”的章节

  * 在 `src-tauri/tauri.conf.json` 中将 `frontendDist` 设置为 `../dist`。
  * 在 iOS 真机上运行时，使用 `process.env.TAURI_DEV_HOST` 作为开发服务器的主机 IP。

## 示例配置

标题为“示例配置”的章节

  1. ##### 更新 Tauri 配置

标题为“更新 Tauri 配置”的章节

假设你在 `package.json` 中有以下 `dev` 和 `build` 脚本

         {

           "scripts": {

             "dev": "vite",

             "build": "tsc && vite build",

             "preview": "vite preview",

             "tauri": "tauri"

           }

         }

你可以配置 Tauri CLI 来使用你的 Vite 开发服务器和 dist 文件夹，并结合钩子自动运行 Vite 脚本

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

        "frontendDist": "../dist"

      }

    }

tauri.conf.json

    {

      "build": {

        "beforeDevCommand": "yarn dev",

        "beforeBuildCommand": "yarn build",

        "devUrl": "https://:5173",

        "frontendDist": "../dist"

      }

    }

tauri.conf.json

    {

      "build": {

        "beforeDevCommand": "pnpm dev",

        "beforeBuildCommand": "pnpm build",

        "devUrl": "https://:5173",

        "frontendDist": "../dist"

      }

    }

tauri.conf.json

    {

      "build": {

        "beforeDevCommand": "deno task dev",

        "beforeBuildCommand": "deno task build",

        "devUrl": "https://:5173",

        "frontendDist": "../dist"

      }

    }

  2. ##### 更新 Vite 配置

标题为“更新 Vite 配置：”的章节

vite.config.js

         import { defineConfig } from 'vite';

         const host = process.env.TAURI_DEV_HOST;

         export default defineConfig({

           // prevent vite from obscuring rust errors

           clearScreen: false,

           server: {

             // make sure this port matches the devUrl port in tauri.conf.json file

             port: 5173,

             // Tauri expects a fixed port, fail if that port is not available

             strictPort: true,

             // if the host Tauri is expecting is set, use it

             host: host || false,

             hmr: host

               ? {

                   protocol: 'ws',

                   host,

                   port: 1421,

                 }

               : undefined,

             watch: {

               // tell vite to ignore watching `src-tauri`

               ignored: ['**/src-tauri/**'],

             },

           },

           // Env variables starting with the item of `envPrefix` will be exposed in tauri's source code through `import.meta.env`.

           envPrefix: ['VITE_', 'TAURI_ENV_*'],

           build: {

             // Tauri uses Chromium on Windows and WebKit on macOS and Linux

             target:

               process.env.TAURI_ENV_PLATFORM == 'windows'

                 ? 'chrome105'

                 : 'safari13',

             // don't minify for debug builds

             minify: !process.env.TAURI_ENV_DEBUG ? 'esbuild' : false,

             // produce sourcemaps for debug builds

             sourcemap: !!process.env.TAURI_ENV_DEBUG,

           },

         });