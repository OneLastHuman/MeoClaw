# 从 Tauri 2.0 Beta 升级

_Source: https://v2.tauri.org.cn/start/migrate/from-tauri-2-beta/_

本指南将引导您完成将 Tauri 2.0 beta 应用程序升级到 Tauri 2.0 正式候选版（Release Candidate）的过程。

## 自动化迁移

标题为“自动化迁移”的章节

Tauri v2 CLI 包含一个 `migrate` 命令，该命令可自动化大部分过程并帮助您完成迁移。

  * npm
  * yarn
  * pnpm
  * cargo

    npm install @tauri-apps/cli@latest

    npm run tauri migrate

    yarn upgrade @tauri-apps/cli@latest

    yarn tauri migrate

    pnpm update @tauri-apps/cli@latest

    pnpm tauri migrate

    cargo install tauri-cli --version "^2.0.0" --locked

    cargo tauri migrate

了解更多关于 `migrate` 命令的信息，请参阅 [命令行界面参考](/reference/cli/#migrate)

## 重大变更

标题为“重大变更”的章节

从 beta 到候选版，我们进行了多项重大变更。这些变更既可以通过自动迁移（见上文）完成，也可以手动执行。

### Tauri 核心插件

标题为“Tauri 核心插件”的章节

我们修改了在 capabilities 中寻址 Tauri 内置插件的方式（详见 [PR #10390](https://github.com/tauri-apps/tauri/pull/10390)）。

要从最新 beta 版本迁移，您需要在 capabilities 中的所有核心权限标识符前加上 `core:` 前缀，或者切换到 `core:default` 权限并移除旧的核心插件标识符。

    ...

    "permissions": [

        "path:default",

        "event:default",

        "window:default",

        "app:default",

        "image:default",

        "resources:default",

        "menu:default",

        "tray:default",

    ]

    ...

    ...

    "permissions": [

        "core:path:default",

        "core:event:default",

        "core:window:default",

        "core:app:default",

        "core:image:default",

        "core:resources:default",

        "core:menu:default",

        "core:tray:default",

    ]

    ...

我们还添加了一个新的特殊 `core:default` 权限集，它包含了所有核心插件的默认权限，以便您可以简化 capabilities 配置中的权限样板代码。

    ...

    "permissions": [

        "core:default"

    ]

    ...

### 内置开发服务器

标题为“内置开发服务器”的章节

我们对内置开发服务器的网络暴露方式进行了更改（详见 [PR #10437](https://github.com/tauri-apps/tauri/pull/10437) 和 [PR #10456](https://github.com/tauri-apps/tauri/pull/10456)）。

内置的移动端开发服务器不再向整个网络暴露，而是直接将本地机器的流量隧道传输到设备。

目前，此改进在 iOS 设备上运行（无论是直接运行还是从 Xcode 运行）时不会自动生效。在这种情况下，我们默认使用公共网络地址作为开发服务器，但有一种解决方法：打开 Xcode 自动启动 macOS 机器与已连接 iOS 设备之间的连接，然后运行 `tauri ios dev --force-ip-prompt` 以选择 iOS 设备的 TUN 地址（以 **::2** 结尾）。

如果打算在物理 iOS 设备上运行，您的开发服务器配置需要适应这一变更。之前我们建议检查 `TAURI_ENV_PLATFORM` 环境变量是否匹配 `android` 或 `ios`，但由于现在除非使用 iOS 设备，否则我们都可以连接到 localhost，因此您应该改为检查 `TAURI_DEV_HOST` 环境变量。以下是 Vite 配置迁移的示例：

  * 2.0.0-beta

    import { defineConfig } from 'vite';

    import { svelte } from '@sveltejs/vite-plugin-svelte';

    import { internalIpV4Sync } from 'internal-ip';

    const mobile = !!/android|ios/.exec(process.env.TAURI_ENV_PLATFORM);

    export default defineConfig({

      plugins: [svelte()],

      clearScreen: false,

      server: {

        host: mobile ? '0.0.0.0' : false,

        port: 1420,

        strictPort: true,

        hmr: mobile

          ? {

              protocol: 'ws',

              host: internalIpV4Sync(),

              port: 1421,

            }

          : undefined,

      },

    });

  * 2.0.0:

    import { defineConfig } from 'vite';

    import Unocss from 'unocss/vite';

    import { svelte } from '@sveltejs/vite-plugin-svelte';

    const host = process.env.TAURI_DEV_HOST;

    export default defineConfig({

      plugins: [svelte()],

      clearScreen: false,

      server: {

        host: host || false,

        port: 1420,

        strictPort: true,

        hmr: host

          ? {

              protocol: 'ws',

              host: host,

              port: 1430,

            }

          : undefined,

      },

    });

注意

`internal-ip` NPM 包不再需要，您可以直接使用 TAURI_DEV_HOST 值。