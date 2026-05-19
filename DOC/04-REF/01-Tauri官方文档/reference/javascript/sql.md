# @tauri-apps/plugin-sql

_Source: https://v2.tauri.org.cn/reference/javascript/sql/_

## 类

名为“类”的部分

### default

名为“default”的章节

**Database**

`Database` 类是与 SQL 插件的 Rust 端进行通信的主要接口。

#### 构造函数

名为“构造函数”的部分

##### new default()

名为“new default()”的章节

    new default(path): default

###### 参数

名为“参数”的部分

参数| 类型
---|---
`path`| `string`

###### 返回

名为“返回值”的部分

[`default`](/reference/javascript/sql/#default)

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/sql/guest-js/index.ts#L29>

#### 属性

名为“属性”的部分

属性| 类型| 定义于
---|---|---
`path`| `string`| **源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/sql/guest-js/index.ts#L28>

#### 方法

名为“方法”的部分

##### close()

名为“close()”的部分

    close(db?): Promise<boolean>

**close**

关闭数据库连接池。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`db`?| `string`| 如果您正在管理多个数据库，可以选填数据库名称。否则，所有数据库连接池都将在作用域内。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`boolean`>

###### 示例

标题为“Example”的章节

    const success = await db.close()

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/sql/guest-js/index.ts#L162>

##### execute()

名为“execute()”的章节

    execute(query, bindValues?): Promise<QueryResult>

**execute**

将 SQL 表达式传递给数据库执行。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`query`| `string`
`bindValues`?| `未知`[]

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`QueryResult`](/reference/javascript/sql/#queryresult)>

###### 示例

标题为“Example”的章节

    // for sqlite & postgres

    // INSERT example

    const result = await db.execute(

       "INSERT into todos (id, title, status) VALUES ($1, $2, $3)",

       [ todos.id, todos.title, todos.status ]

    );

    // UPDATE example

    const result = await db.execute(

       "UPDATE todos SET title = $1, completed = $2 WHERE id = $3",

       [ todos.title, todos.status, todos.id ]

    );

    // for mysql

    // INSERT example

    const result = await db.execute(

       "INSERT into todos (id, title, status) VALUES (?, ?, ?)",

       [ todos.id, todos.title, todos.status ]

    );

    // UPDATE example

    const result = await db.execute(

       "UPDATE todos SET title = ?, completed = ? WHERE id = ?",

       [ todos.title, todos.status, todos.id ]

    );

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/sql/guest-js/index.ts#L108>

##### select()

名为“select()”的章节

    select<T>(query, bindValues?): Promise<T>

**select**

将 SELECT 查询传递给数据库执行。

###### 类型参数

名为“类型参数”的部分

类型参数
---
`T`

###### 参数

名为“参数”的部分

参数| 类型
---|---
`query`| `string`
`bindValues`?| `未知`[]

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`T`>

###### 示例

标题为“Example”的章节

    // for sqlite & postgres

    const result = await db.select(

       "SELECT * from todos WHERE id = $1", [ id ]

    );

    // for mysql

    const result = await db.select(

       "SELECT * from todos WHERE id = ?", [ id ]

    );

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/sql/guest-js/index.ts#L141>

##### get()

名为“get()”的章节

    static get(path): default

**get**

一个静态初始化程序，它同步返回 Database 类的一个实例，同时将实际的数据库连接推迟到第一次调用或在数据库上进行选择时。

# Sqlite

名为“Sqlite”的章节

路径是相对于 `tauri::path::BaseDirectory::App` 的，并且必须以 `sqlite:` 开头。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`path`| `string`

###### 返回

名为“返回值”的部分

[`default`](/reference/javascript/sql/#default)

###### 示例

标题为“Example”的章节

    const db = Database.get("sqlite:test.db");

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/sql/guest-js/index.ts#L72>

##### load()

名为“load()”的章节

    static load(path): Promise<default>

**load**

一个静态初始化程序，它连接到基础数据库，并在建立数据库连接后返回一个 `Database` 实例。

# Sqlite

名为“Sqlite”的章节

路径是相对于 `tauri::path::BaseDirectory::App` 的，并且必须以 `sqlite:` 开头。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`path`| `string`

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`default`](/reference/javascript/sql/#default)>

###### 示例

标题为“Example”的章节

    const db = await Database.load("sqlite:test.db");

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/sql/guest-js/index.ts#L48>

## 接口

名为“接口”的部分

### QueryResult

名为“QueryResult”的章节

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
`lastInsertId?`| `数字`| 最后插入的 `id`。此值不适用于 Postgres 数据库。如果 Postgres 需要最后插入的 id，则必须使用 `select` 函数，并带有 `RETURNING` 子句（例如：`INSERT INTO todos (title) VALUES ($1) RETURNING id`）。| **源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/sql/guest-js/index.ts#L18>
`rowsAffected`| `数字`| 受查询影响的行数。| **源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/sql/guest-js/index.ts#L9>