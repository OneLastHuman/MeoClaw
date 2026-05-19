# 范围

_Source: https://v2.tauri.org.cn/reference/acl/scope/_

用于对 Tauri 命令的行为进行细粒度控制的参数。

它可以是任何支持 Serde 序列化的类型，并用于允许或禁止 Tauri 命令内部的某些操作。配置的作用域会传递给命令，并由命令的具体实现强制执行。

### 示例

标题为“Example”的章节

    {

      "allow": [{ "path": "$HOME/**" }],

      "deny": [{ "path": "$HOME/secret.txt" }]

    }

**对象属性** :

  * allow
  * deny

### allow

名为“allow”的章节

`Value`[] | `null`

定义作用域内允许操作的数据。

### deny

名为“deny”的章节

`Value`[] | `null`

定义作用域内禁止操作的数据。验证逻辑应优先处理此项。

## 定义

名为“定义”的章节

### 数字

标题为“Number”的章节

**以下任何一种** :

  * `integer` 格式为 `int64`，表示 [`i64`]。
  * `number` 格式为 `double`，表示 [`f64`]。

一个有效的 ACL 数字。

### 值

“Value” 章节

**以下任何一种** :

  * `null` 表示一个空 JSON 值。
  * `boolean` 表示一个 [`bool`] 值。
  * `Number` 表示一个有效的 ACL [`Number`] 值。
  * `string` 表示一个 [`String`] 值。
  * `Value`[] 表示其他 [`Value`] 值的列表。
  * 表示从 [`String`] 键到 [`Value`] 值的映射。**允许附加属性** ：`Value`

所有支持的 ACL 值。