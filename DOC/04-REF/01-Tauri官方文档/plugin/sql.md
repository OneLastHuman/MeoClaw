# SQL

_Source: https://v2.tauri.org.cn/plugin/sql/_

[ GitHub](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/sql) [ npm ](https://npmjs.net.cn/package/@tauri-apps/plugin-sql) [ crates.io ](https://crates.io/crates/tauri-plugin-sql)

API 参考 [ ](/reference/javascript/sql/) [ ](https://docs.rs/tauri-plugin-sql)

该插件为前端提供了一个与 SQL 数据库通信的接口，底层使用 [sqlx](https://github.com/launchbadge/sqlx)。它支持 SQLite、MySQL 和 PostgreSQL 驱动程序，可通过 Cargo 特性启用。

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

安装 SQL 插件以开始使用。

  * 自动
  * 手动

使用你的项目包管理器添加依赖项

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri add sql

    yarn run tauri add sql

    pnpm tauri add sql

    deno task tauri add sql

    bun tauri add sql

    cargo tauri add sql

  1. 在 `src-tauri` 文件夹中运行以下命令，将插件添加到项目的 `Cargo.toml` 依赖项中

         cargo add tauri-plugin-sql

  2. 修改 `lib.rs` 以初始化插件

src-tauri/src/lib.rs

         #[cfg_attr(mobile, tauri::mobile_entry_point)]

             pub fn run() {

             tauri::Builder::default()

                 .plugin(tauri_plugin_sql::Builder::default().build())

                 .run(tauri::generate_context!())

                 .expect("error while running tauri application");

         }

  3. 使用您首选的 JavaScript 包管理器安装 JavaScript 访客绑定

     * npm
     * yarn
     * pnpm
     * deno
     * bun

    npm install @tauri-apps/plugin-sql

    yarn add @tauri-apps/plugin-sql

    pnpm add @tauri-apps/plugin-sql

    deno add npm:@tauri-apps/plugin-sql

    bun add @tauri-apps/plugin-sql

安装插件后，必须选择支持的数据库引擎。可用的引擎有 Sqlite、MySQL 和 PostgreSQL。在 `src-tauri` 文件夹中运行以下命令以启用您偏好的引擎

  * SQLite
  * MySQL
  * PostgreSQL

    cargo add tauri-plugin-sql --features sqlite

    cargo add tauri-plugin-sql --features mysql

    cargo add tauri-plugin-sql --features postgres

## 用法

名为“用法”的章节

插件的所有 API 均可通过 JavaScript guest 绑定使用

  * SQLite
  * MySQL
  * PostgreSQL

路径相对于 [`tauri::api::path::BaseDirectory::AppConfig`](https://docs.rs/tauri/2.0.0/tauri/path/enum.BaseDirectory.html#variant.AppConfig)。

    import Database from '@tauri-apps/plugin-sql';

    // when using `"withGlobalTauri": true`, you may use

    // const Database = window.__TAURI__.sql;

    const db = await Database.load('sqlite:test.db');

    await db.execute('INSERT INTO ...');

    import Database from '@tauri-apps/plugin-sql';

    // when using `"withGlobalTauri": true`, you may use

    // const Database = window.__TAURI__.sql;

    const db = await Database.load('mysql://user:password@host/test');

    await db.execute('INSERT INTO ...');

    import Database from '@tauri-apps/plugin-sql';

    // when using `"withGlobalTauri": true`, you may use

    // const Database = window.__TAURI__.sql;

    const db = await Database.load('postgres://user:password@host/test');

    await db.execute('INSERT INTO ...');

## 语法

名为“语法”的章节

我们使用 [sqlx](https://docs.rs/sqlx/latest/sqlx/) 作为底层库，并采用其查询语法。

  * SQLite
  * MySQL
  * PostgreSQL

替换查询数据时使用 “$#” 语法

    const result = await db.execute(

      'INSERT into todos (id, title, status) VALUES ($1, $2, $3)',

      [todos.id, todos.title, todos.status]

    );

    const result = await db.execute(

      'UPDATE todos SET title = $1, status = $2 WHERE id = $3',

      [todos.title, todos.status, todos.id]

    );

替换查询数据时使用 “?”

    const result = await db.execute(

      'INSERT into todos (id, title, status) VALUES (?, ?, ?)',

      [todos.id, todos.title, todos.status]

    );

    const result = await db.execute(

      'UPDATE todos SET title = ?, status = ? WHERE id = ?',

      [todos.title, todos.status, todos.id]

    );

替换查询数据时使用 “$#” 语法

    const result = await db.execute(

      'INSERT into todos (id, title, status) VALUES ($1, $2, $3)',

      [todos.id, todos.title, todos.status]

    );

    const result = await db.execute(

      'UPDATE todos SET title = $1, status = $2 WHERE id = $3',

      [todos.title, todos.status, todos.id]

    );

## 迁移

名为“迁移”的章节

此插件支持数据库迁移，允许您随时间管理数据库模式的演变。

### 定义迁移

名为“定义迁移”的章节

迁移是在 Rust 中使用 [`Migration`](https://docs.rs/tauri-plugin-sql/latest/tauri_plugin_sql/struct.Migration.html) 结构体定义的。每次迁移都应包含一个唯一的版本号、描述、要执行的 SQL 以及迁移类型（Up 或 Down）。

迁移示例

    use tauri_plugin_sql::{Migration, MigrationKind};

    let migration = Migration {

        version: 1,

        description: "create_initial_tables",

        sql: "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT);",

        kind: MigrationKind::Up,

    };

或者，如果您想使用文件中的 SQL，可以使用 `include_str!` 宏将其包含进来

    use tauri_plugin_sql::{Migration, MigrationKind};

    let migration = Migration {

        version: 1,

        description: "create_initial_tables",

        sql: include_str!("../drizzle/0000_graceful_boomer.sql"),

        kind: MigrationKind::Up,

    };

### 将迁移添加到插件构建器

名为“将迁移添加到插件构建器”的章节

迁移通过插件提供的 [`Builder`](https://docs.rs/tauri-plugin-sql/latest/tauri_plugin_sql/struct.Builder.html) 结构体进行注册。使用 `add_migrations` 方法将您的迁移添加到特定数据库连接的插件中。

添加迁移示例

src-tauri/src/main.rs

    use tauri_plugin_sql::{Builder, Migration, MigrationKind};

    fn main() {

        let migrations = vec![

            // Define your migrations here

            Migration {

                version: 1,

                description: "create_initial_tables",

                sql: "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT);",

                kind: MigrationKind::Up,

            }

        ];

        tauri::Builder::default()

            .plugin(

                tauri_plugin_sql::Builder::default()

                    .add_migrations("sqlite:mydatabase.db", migrations)

                    .build(),

            )

            ...

    }

### 应用迁移

名为“应用迁移”的章节

要在插件初始化时应用迁移，请将连接字符串添加到 `tauri.conf.json` 文件中

src-tauri/tauri.conf.json

    {

      "plugins": {

        "sql": {

          "preload": ["sqlite:mydatabase.db"]

        }

      }

    }

或者，客户端的 `load()` 方法也会为给定的连接字符串运行迁移

    import Database from '@tauri-apps/plugin-sql';

    const db = await Database.load('sqlite:mydatabase.db');

确保迁移定义顺序正确，并且可以安全地多次运行。

注意

所有迁移都在事务中执行，以确保原子性。如果任何迁移失败，整个事务将回滚，使数据库保持在一致状态。

### 迁移管理

名为“迁移管理”的章节

  * **版本控制** ：每次迁移必须有一个唯一的版本号。这对于确保迁移按正确顺序应用至关重要。
  * **幂等性** ：编写迁移时，确保它们可以安全地重新运行，而不会导致错误或意外后果。
  * **测试** ：彻底测试迁移，以确保它们按预期工作，并且不会损害数据库的完整性。

## Permissions

名为“权限”的章节

默认情况下，所有潜在危险的插件命令和范围都被阻止，无法访问。您必须修改 `capabilities` 配置中的权限才能启用这些功能。

有关更多信息，请参阅[功能概述](/security/capabilities/)，并参阅[分步指南](/learn/security/using-plugin-permissions/)以使用插件权限。

src-tauri/capabilities/default.json

    {

      "permissions": [

        ...,

        "sql:default",

        "sql:allow-execute",

      ]

    }

## 默认权限

### 默认权限

此权限集配置了 SQL 插件可用的数据库操作类型。

### 已授予权限

启用了所有与读取相关的操作。还允许加载或关闭连接。

#### 此默认权限集包括以下内容

  * `allow-close`
  * `allow-load`
  * `allow-select`

## 权限表

标识符 | 描述
---|---
`sql:allow-close` |  启用 close 命令，无需任何预配置的作用域。
`sql:deny-close` |  禁用 close 命令，无需任何预配置的作用域。
`sql:allow-execute` |  启用 execute 命令，无需任何预配置的作用域。
`sql:deny-execute` |  禁用 execute 命令，无需任何预配置的作用域。
`sql:allow-load` |  启用 load 命令，无需任何预配置的作用域。
`sql:deny-load` |  禁用 load 命令，无需任何预配置的作用域。
`sql:allow-select` |  启用 select 命令，无需任何预配置的作用域。
`sql:deny-select` |  禁用 select 命令，无需任何预配置的作用域。