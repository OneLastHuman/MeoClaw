# 存储 (Store)

_Source: https://v2.tauri.org.cn/plugin/store/_

[ GitHub](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/store) [ npm ](https://npmjs.net.cn/package/@tauri-apps/plugin-store) [ crates.io ](https://crates.io/crates/tauri-plugin-store)

API 参考 [ ](/reference/javascript/store/) [ ](https://docs.rs/tauri-plugin-store)

此插件提供了一个持久化的键值存储。这是处理应用程序状态的众多选项之一。有关其他选项的更多信息，请参阅[状态管理概述](/develop/state-management/)。

此存储允许您将状态持久化到文件中，该文件可以按需保存和加载，包括在应用程序重启之间。请注意，此过程是异步的，需要在您的代码中进行处理。它既可以在 Webview 中使用，也可以在 Rust 中使用。

## 支持的平台

标题为“支持的平台”的章节

_此插件需要 Rust 版本至少为 **1.77.2**_

平台 | 级别 | 备注
---|---|---
windows |  |
linux |  |
macos |  |
android |  |
ios |  |

## 设置

标题为“设置”的章节

安装存储插件以开始使用。

  * 自动
  * 手动

使用你的项目包管理器添加依赖项

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri add store

    yarn run tauri add store

    pnpm tauri add store

    deno task tauri add store

    bun tauri add store

    cargo tauri add store

  1. 在 `src-tauri` 文件夹中运行以下命令，将插件添加到项目的 `Cargo.toml` 依赖项中

         cargo add tauri-plugin-store

  2. 修改 `lib.rs` 以初始化插件

src-tauri/src/lib.rs

         #[cfg_attr(mobile, tauri::mobile_entry_point)]

         pub fn run() {

             tauri::Builder::default()

                 .plugin(tauri_plugin_store::Builder::new().build())

                 .run(tauri::generate_context!())

                 .expect("error while running tauri application");

         }

  3. 使用您首选的 JavaScript 包管理器安装 JavaScript 访客绑定

     * npm
     * yarn
     * pnpm
     * deno
     * bun

    npm install @tauri-apps/plugin-store

    yarn add @tauri-apps/plugin-store

    pnpm add @tauri-apps/plugin-store

    deno add npm:@tauri-apps/plugin-store

    bun add @tauri-apps/plugin-store

## 用法

名为“用法”的章节

  * JavaScript
  * Rust

    import { load } from '@tauri-apps/plugin-store';

    // when using `"withGlobalTauri": true`, you may use

    // const { load } = window.__TAURI__.store;

    // Create a new store or load the existing one,

    // note that the options will be ignored if a `Store` with that path has already been created

    const store = await load('store.json', { autoSave: false });

    // Set a value.

    await store.set('some-key', { value: 5 });

    // Get a value.

    const val = await store.get<{ value: number }>('some-key');

    console.log(val); // { value: 5 }

    // You can manually save the store after making changes.

    // Otherwise, it will save upon graceful exit

    // And if you set `autoSave` to a number or left empty,

    // it will save the changes to disk after a debounce delay, 100ms by default.

    await store.save();

src-tauri/src/lib.rs

    use tauri::Wry;

    use tauri_plugin_store::StoreExt;

    use serde_json::json;

    #[cfg_attr(mobile, tauri::mobile_entry_point)]

    pub fn run() {

        tauri::Builder::default()

            .plugin(tauri_plugin_store::Builder::default().build())

            .setup(|app| {

                // Create a new store or load the existing one

                // this also put the store in the app's resource table

                // so your following `store` calls (from both Rust and JS)

                // will reuse the same store.

                let store = app.store("store.json")?;

                // Note that values must be serde_json::Value instances,

                // otherwise, they will not be compatible with the JavaScript bindings.

                store.set("some-key", json!({ "value": 5 }));

                // Get a value from the store.

                let value = store.get("some-key").expect("Failed to get value from store");

                println!("{}", value); // {"value":5}

                // Remove the store from the resource table

                store.close_resource();

                Ok(())

            })

            .run(tauri::generate_context!())

            .expect("error while running tauri application");

    }

### LazyStore

名为“LazyStore”的章节

还有一个高级 JavaScript API `LazyStore`，它只在首次访问时加载存储。

    import { LazyStore } from '@tauri-apps/plugin-store';

    const store = new LazyStore('settings.json');

## 从 v1 和 v2 beta/rc 版本迁移

名为“从 v1 和 v2 beta/rc 版本迁移”的章节

  * JavaScript
  * Rust

    import { Store } from '@tauri-apps/plugin-store';

    import { LazyStore } from '@tauri-apps/plugin-store';

    with_store(app.handle().clone(), stores, path, |store| {

        store.insert("some-key".to_string(), json!({ "value": 5 }))?;

        Ok(())

    });

    let store = app.store(path)?;

    store.set("some-key".to_string(), json!({ "value": 5 }));

## Permissions

名为“权限”的章节

默认情况下，所有潜在危险的插件命令和范围都被阻止，无法访问。您必须修改 `capabilities` 配置中的权限才能启用这些功能。

有关更多信息，请参阅[功能概述](/security/capabilities/)，并参阅[分步指南](/learn/security/using-plugin-permissions/)以使用插件权限。

src-tauri/capabilities/default.json

    {

      "permissions": [

        ...,

        "store:default",

      ]

    }

## 默认权限

此权限集配置了存储插件中可用的操作类型。

#### 已授予权限

默认情况下，所有操作均已启用。

#### 此默认权限集包括以下内容

  * `allow-load`
  * `allow-get-store`
  * `allow-set`
  * `allow-get`
  * `allow-has`
  * `allow-delete`
  * `allow-clear`
  * `allow-reset`
  * `allow-keys`
  * `allow-values`
  * `allow-entries`
  * `allow-length`
  * `allow-reload`
  * `允许保存`

## 权限表

标识符 | 描述
---|---
`store:allow-clear` |  在没有预先配置范围的情况下启用 clear 命令。
`store:deny-clear` |  在没有预先配置范围的情况下禁止 clear 命令。
`store:allow-delete` |  在没有预先配置范围的情况下启用 delete 命令。
`store:deny-delete` |  在没有预先配置范围的情况下禁止 delete 命令。
`store:allow-entries` |  在没有预先配置范围的情况下启用 entries 命令。
`store:deny-entries` |  在没有预先配置范围的情况下禁止 entries 命令。
`store:allow-get` |  在没有预先配置范围的情况下启用 get 命令。
`store:deny-get` |  在没有预先配置范围的情况下禁止 get 命令。
`store:allow-get-store` |  在没有预先配置范围的情况下启用 get_store 命令。
`store:deny-get-store` |  在没有预先配置范围的情况下禁止 get_store 命令。
`store:allow-has` |  在没有预先配置范围的情况下启用 has 命令。
`store:deny-has` |  在没有预先配置范围的情况下禁止 has 命令。
`store:allow-keys` |  在没有预先配置范围的情况下启用 keys 命令。
`store:deny-keys` |  在没有预先配置范围的情况下禁止 keys 命令。
`store:allow-length` |  在没有预先配置范围的情况下启用 length 命令。
`store:deny-length` |  在没有预先配置范围的情况下禁止 length 命令。
`store:allow-load` |  在没有预先配置范围的情况下启用 load 命令。
`store:deny-load` |  在没有预先配置范围的情况下禁止 load 命令。
`store:allow-reload` |  在没有预先配置范围的情况下启用 reload 命令。
`store:deny-reload` |  在没有预先配置范围的情况下禁止 reload 命令。
`store:allow-reset` |  在没有预先配置范围的情况下启用 reset 命令。
`store:deny-reset` |  在没有预先配置范围的情况下禁止 reset 命令。
`store:allow-save` |  启用保存命令，不带任何预配置范围。
`store:deny-save` |  禁用保存命令，不带任何预配置范围。
`store:allow-set` |  在没有预先配置范围的情况下启用 set 命令。
`store:deny-set` |  在没有预先配置范围的情况下禁止 set 命令。
`store:allow-values` |  在没有预先配置范围的情况下启用 values 命令。
`store:deny-values` |  在没有预先配置范围的情况下禁止 values 命令。