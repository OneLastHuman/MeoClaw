# 权限

_Source: https://v2.tauri.org.cn/reference/acl/permission/_

对命令明确权限的描述。

它可以使命令在应用程序的前端中可用。

如果定义了作用域，则可用于细粒度控制对单个或多个命令的访问。

**对象属性** :

  * 命令（commands）
  * description
  * 标识符 (必需)
  * platforms
  * 作用域（scope）
  * version

### 命令（commands）

标题为“命令”的章节

`命令`

使用此权限时允许或拒绝的命令。

默认

    {

      "allow": [],

      "deny": []

    }

### description

标题为“description”的章节

`string` | `null`

关于该权限用途的人类可读描述。Tauri 的内部约定是使用 `<h4>` 标题进行 Markdown 内容撰写，以用于 Tauri 文档生成。

### identifier

名为“identifier”的章节

`string`

权限的唯一标识符。

### platforms

标题为“platforms”的章节

`Target`[] | `null`

此权限适用的目标平台。默认情况下，所有平台都受此权限影响。

### 作用域（scope）

标题为“作用域”的章节

`范围`

使用此权限时允许或拒绝的作用域。

### version

标题为“版本”的章节

`integer` | `null` 最小值为 `1`，格式为 `uint64`

权限的版本。

## 定义

名为“定义”的章节

### 命令

标题为“命令”的章节

权限内允许和拒绝的命令。

如果两个命令在 `allow` 和 `deny` 中发生冲突，则默认应拒绝。

**对象属性** :

  * allow（允许）
  * deny（拒绝）

##### allow（允许）

标题为“allow”的章节

`string`[]

允许的命令。

**默认值** ：`[]`

##### deny（拒绝）

标题为“deny”的章节

`string`[]

被拒绝的命令，具有最高优先级。

**默认值** ：`[]`

### 数字

标题为“Number”的章节

**以下任何一种** :

  * `integer` 格式为 `int64`，表示 [`i64`]。
  * `number` 格式为 `double`，表示 [`f64`]。

一个有效的 ACL 数字。

### 范围

标题为“作用域”的章节

用于对 Tauri 命令行为进行细粒度控制的参数。

它可以是任何支持 Serde 序列化的类型，并用于允许或阻止 Tauri 命令内的某些操作。配置的作用域将传递给命令，并由命令的具体实现执行。

##### 示例

标题为“Example”的章节

    {

      "allow": [{ "path": "$HOME/**" }],

      "deny": [{ "path": "$HOME/secret.txt" }]

    }

**对象属性** :

  * allow（允许）
  * deny（拒绝）

##### allow（允许）

标题为“allow”的章节

`Value`[] | `null`

定义作用域允许范围的数据。

##### deny（拒绝）

标题为“deny”的章节

`Value`[] | `null`

定义作用域拒绝范围的数据。在验证逻辑中应优先处理此项。

### 目标

“Target” 章节

**以下其中之一** :

  * `"macOS"` macOS。
  * `"windows"` Windows。
  * `"linux"` Linux。
  * `"android"` Android。
  * `"iOS"` iOS。

平台目标。

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