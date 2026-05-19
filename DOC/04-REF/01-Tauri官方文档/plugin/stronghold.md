# 要塞 (Stronghold)

_Source: https://v2.tauri.org.cn/plugin/stronghold/_

[ GitHub](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/stronghold) [ npm ](https://npmjs.net.cn/package/@tauri-apps/plugin-stronghold) [ crates.io ](https://crates.io/crates/tauri-plugin-stronghold)

API 参考 [ ](/reference/javascript/stronghold/) [ ](https://docs.rs/tauri-plugin-stronghold)

使用 [IOTA Stronghold](https://github.com/iotaledger/stronghold.rs) 密钥管理引擎存储密钥和机密。

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

安装 Stronghold 插件以开始使用。

  * 自动
  * 手动

使用你的项目包管理器添加依赖项

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri add stronghold

    yarn run tauri add stronghold

    pnpm tauri add stronghold

    deno task tauri add stronghold

    bun tauri add stronghold

    cargo tauri add stronghold

  1. 在 `src-tauri` 文件夹中运行以下命令，将插件添加到项目的 `Cargo.toml` 依赖项中

         cargo add tauri-plugin-stronghold

  2. 修改 `lib.rs` 以初始化插件

src-tauri/src/lib.rs

         #[cfg_attr(mobile, tauri::mobile_entry_point)]

         pub fn run() {

             tauri::Builder::default()

                 .plugin(tauri_plugin_stronghold::Builder::new(|password| {}).build())

                 .run(tauri::generate_context!())

                 .expect("error while running tauri application");

         }

  3. 使用您首选的 JavaScript 包管理器安装 JavaScript 访客绑定

     * npm
     * yarn
     * pnpm
     * deno
     * bun

    npm install @tauri-apps/plugin-stronghold

    yarn add @tauri-apps/plugin-stronghold

    pnpm add @tauri-apps/plugin-stronghold

    deno add npm:@tauri-apps/plugin-stronghold

    bun add @tauri-apps/plugin-stronghold

由于存在一个 [上游 Bug](https://github.com/tauri-apps/plugins-workspace/issues/2048)，我们还建议您将其添加到您的 `Cargo.toml` 文件中

    [profile.dev.package.scrypt]

    opt-level = 3

## 用法

名为“用法”的章节

该插件必须使用密码哈希函数进行初始化，该函数接收密码字符串并必须返回一个由其派生的 32 字节哈希值。

### 使用 argon2 密码哈希函数初始化

标题为“使用 argon2 密码哈希函数初始化”的章节

Stronghold 插件提供了一个使用 [argon2](https://docs.rs/rust-argon2/latest/argon2/) 算法的默认哈希函数。

src-tauri/src/lib.rs

    use tauri::Manager;

    pub fn run() {

        tauri::Builder::default()

            .setup(|app| {

                let salt_path = app

                    .path()

                    .app_local_data_dir()

                    .expect("could not resolve app local data path")

                    .join("salt.txt");

                app.handle().plugin(tauri_plugin_stronghold::Builder::with_argon2(&salt_path).build())?;

                Ok(())

            })

            .run(tauri::generate_context!())

            .expect("error while running tauri application");

    }

### 使用自定义密码哈希函数初始化

标题为“使用自定义密码哈希函数初始化”的章节

或者，您可以使用 `tauri_plugin_stronghold::Builder::new` 构造函数提供您自己的哈希算法。

注意

密码哈希必须正好包含 32 个字节。这是 Stronghold 的一项要求。

src-tauri/src/lib.rs

    pub fn run() {

        tauri::Builder::default()

            .plugin(

                tauri_plugin_stronghold::Builder::new(|password| {

                    // Hash the password here with e.g. argon2, blake2b or any other secure algorithm

                    // Here is an example implementation using the `rust-argon2` crate for hashing the password

                    use argon2::{hash_raw, Config, Variant, Version};

                    let config = Config {

                        lanes: 4,

                        mem_cost: 10_000,

                        time_cost: 10,

                        variant: Variant::Argon2id,

                        version: Version::Version13,

                        ..Default::default()

                    };

                    let salt = "your-salt".as_bytes();

                    let key = hash_raw(password.as_ref(), salt, &config).expect("failed to hash password");

                    key.to_vec()

                })

                .build(),

            )

            .run(tauri::generate_context!())

            .expect("error while running tauri application");

    }

### 从 JavaScript 使用

标题为“从 JavaScript 使用”的章节

Stronghold 插件可在 JavaScript 中使用。

    import { Client, Stronghold } from '@tauri-apps/plugin-stronghold';

    // when using `"withGlobalTauri": true`, you may use

    // const { Client, Stronghold } = window.__TAURI__.stronghold;

    import { appDataDir } from '@tauri-apps/api/path';

    // when using `"withGlobalTauri": true`, you may use

    // const { appDataDir } = window.__TAURI__.path;

    const initStronghold = async () => {

      const vaultPath = `${await appDataDir()}/vault.hold`;

      const vaultPassword = 'vault password';

      const stronghold = await Stronghold.load(vaultPath, vaultPassword);

      let client: Client;

      const clientName = 'name your client';

      try {

        client = await stronghold.loadClient(clientName);

      } catch {

        client = await stronghold.createClient(clientName);

      }

      return {

        stronghold,

        client,

      };

    };

    // Insert a record to the store

    async function insertRecord(store: any, key: string, value: string) {

      const data = Array.from(new TextEncoder().encode(value));

      await store.insert(key, data);

    }

    // Read a record from store

    async function getRecord(store: any, key: string): Promise<string> {

      const data = await store.get(key);

      return new TextDecoder().decode(new Uint8Array(data));

    }

    const { stronghold, client } = await initStronghold();

    const store = client.getStore();

    const key = 'my_key';

    // Insert a record to the store

    insertRecord(store, key, 'secret value');

    // Read a record from store

    const value = await getRecord(store, key);

    console.log(value); // 'secret value'

    // Save your updates

    await stronghold.save();

    // Remove a record from store

    await store.remove(key);

## Permissions

名为“权限”的章节

默认情况下，所有潜在危险的插件命令和范围都被阻止，无法访问。您必须修改 `capabilities` 配置中的权限才能启用这些功能。

有关更多信息，请参阅[功能概述](/security/capabilities/)，并参阅[分步指南](/learn/security/using-plugin-permissions/)以使用插件权限。

src-tauri/capabilities/default.json

    {

      ...,

      "permissions": [

        "stronghold:default",

      ]

    }

## 默认权限

此权限集配置了 Stronghold 插件中可用的操作类型。

#### 已授予权限

所有非破坏性操作默认启用。

#### 此默认权限集包括以下内容

  * `allow-create-client`
  * `allow-get-store-record`
  * `allow-initialize`
  * `allow-execute-procedure`
  * `allow-load-client`
  * `allow-save-secret`
  * `allow-save-store-record`
  * `允许保存`

## 权限表

标识符 | 描述
---|---
`stronghold:allow-create-client` |  启用 create_client 命令，无需任何预配置的作用域。
`stronghold:deny-create-client` |  禁用 create_client 命令，无需任何预配置的作用域。
`stronghold:allow-destroy` |  启用 destroy 命令，无需任何预配置的作用域。
`stronghold:deny-destroy` |  禁用 destroy 命令，无需任何预配置的作用域。
`stronghold:allow-execute-procedure` |  启用 execute_procedure 命令，无需任何预配置的作用域。
`stronghold:deny-execute-procedure` |  禁用 execute_procedure 命令，无需任何预配置的作用域。
`stronghold:allow-get-store-record` |  启用 get_store_record 命令，无需任何预配置的作用域。
`stronghold:deny-get-store-record` |  禁用 get_store_record 命令，无需任何预配置的作用域。
`stronghold:allow-initialize` |  启用 initialize 命令，无需任何预配置的作用域。
`stronghold:deny-initialize` |  禁用 initialize 命令，无需任何预配置的作用域。
`stronghold:allow-load-client` |  启用 load_client 命令，无需任何预配置的作用域。
`stronghold:deny-load-client` |  禁用 load_client 命令，无需任何预配置的作用域。
`stronghold:allow-remove-secret` |  启用 remove_secret 命令，无需任何预配置的作用域。
`stronghold:deny-remove-secret` |  禁用 remove_secret 命令，无需任何预配置的作用域。
`stronghold:allow-remove-store-record` |  启用 remove_store_record 命令，无需任何预配置的作用域。
`stronghold:deny-remove-store-record` |  禁用 remove_store_record 命令，无需任何预配置的作用域。
`stronghold:allow-save` |  启用保存命令，不带任何预配置范围。
`stronghold:deny-save` |  禁用保存命令，不带任何预配置范围。
`stronghold:allow-save-secret` |  启用 save_secret 命令，无需任何预配置的作用域。
`stronghold:deny-save-secret` |  禁用 save_secret 命令，无需任何预配置的作用域。
`stronghold:allow-save-store-record` |  启用 save_store_record 命令，无需任何预配置的作用域。
`stronghold:deny-save-store-record` |  禁用 save_store_record 命令，无需任何预配置的作用域。