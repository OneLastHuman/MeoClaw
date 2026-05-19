# Qwik

_Source: https://v2.tauri.org.cn/start/frontend/qwik/_

本指南将引导你使用 Qwik Web 框架创建 Tauri 应用。了解更多关于 Qwik 的信息，请访问 <https://qwik.node.org.cn>。

## 清单

名为“检查清单”的章节

  * 使用 [SSG](https://qwik.node.org.cn/docs/guides/static-site-generation/)。Tauri 不支持基于服务器的解决方案。
  * 在 `tauri.conf.json` 中使用 `dist/` 作为 `frontendDist`。

## 示例配置

名为“示例配置”的章节

  1. ##### 创建一个新的 Qwik 应用

名为“创建一个新的 Qwik 应用”的章节

     * npm
     * yarn
     * pnpm
     * deno

    npm create qwik@latest

    cd <PROJECT>

    yarn create qwik@latest

    cd <PROJECT>

    pnpm create qwik@latest

    cd <PROJECT>

    deno run -A npm:create-qwik@latest

    cd <PROJECT>

  2. ##### 安装 `static adapter` (静态适配器)

名为“安装静态适配器”的章节

     * npm
     * yarn
     * pnpm
     * deno

    npm run qwik add static

    yarn qwik add static

    pnpm qwik add static

    deno task qwik add static

  3. ##### 将 Tauri CLI 添加到你的项目中

名为“将 Tauri CLI 添加到你的项目中”的章节

     * npm
     * yarn
     * pnpm
     * deno

    npm install -D @tauri-apps/cli@latest

    yarn add -D @tauri-apps/cli@latest

    pnpm add -D @tauri-apps/cli@latest

    deno add -D npm:@tauri-apps/cli@latest

  4. ##### 初始化一个新的 Tauri 项目

名为“初始化一个新的 Tauri 项目”的章节

     * npm
     * yarn
     * pnpm
     * deno

    npm run tauri init

    yarn tauri init

    pnpm tauri init

    deno task tauri init

  5. ##### Tauri 配置

名为“Tauri 配置”的章节

     * npm
     * yarn
     * pnpm
     * deno

tauri.conf.json

    {

      "build": {

        "devUrl": "https://:5173"

        "frontendDist": "../dist",

        "beforeDevCommand": "npm run dev",

        "beforeBuildCommand": "npm run build"

      }

    }

tauri.conf.json

    {

      "build": {

        "devUrl": "https://:5173"

        "frontendDist": "../dist",

        "beforeDevCommand": "yarn dev",

        "beforeBuildCommand": "yarn build"

      }

    }

tauri.conf.json

    {

      "build": {

        "devUrl": "https://:5173"

        "frontendDist": "../dist",

        "beforeDevCommand": "pnpm dev",

        "beforeBuildCommand": "pnpm build"

      }

    }

tauri.conf.json

    {

      "build": {

        "devUrl": "https://:5173"

        "frontendDist": "../dist",

        "beforeDevCommand": "deno task dev",

        "beforeBuildCommand": "deno task build"

      }

    }

  6. ##### 启动你的 `tauri` 应用

名为“启动你的 tauri 应用”的章节

     * npm
     * yarn
     * pnpm
     * deno

    npm run tauri dev

    yarn tauri dev

    pnpm tauri dev

    deno task tauri dev