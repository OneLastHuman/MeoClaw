# 功能

_Source: https://v2.tauri.org.cn/reference/acl/capability/_

开发者可用于隔离 IPC 层访问权限的组合与边界机制。

它控制应用程序窗口和 webview 对 Tauri 核心、应用程序或插件命令的细粒度访问。如果一个 webview 或其所属窗口未匹配到任何能力 (Capability)，则它完全无法访问 IPC 层。

这可用于根据窗口所需的系统访问权限将其进行分组，从而降低特权较低窗口中前端漏洞带来的影响。可以通过确切名称（例如 `main-window`）或 glob 模式（如 `*` 或 `admin-*`）将窗口添加到某个能力中。一个窗口可以不关联、关联一个或多个能力。

### 示例

标题为“Example”的章节

    {

      "identifier": "main-user-files-write",

      "description": "This capability allows the `main` window on macOS and Windows access to `filesystem` write related commands and `dialog` commands to enable programmatic access to files selected by the user.",

      "windows": [

        "main"

      ],

      "permissions": [

        "core:default",

        "dialog:open",

        {

          "identifier": "fs:allow-write-text-file",

          "allow": [{ "path": "$HOME/test.txt" }]

        },

      ],

      "platforms": ["macOS","windows"]

    }

**对象属性** :

  * description
  * 标识符 (必需)
  * local
  * permissions (必需)
  * platforms
  * remote
  * webviews
  * windows

### description

标题为“description”的章节

`string`

描述该能力旨在关联窗口上允许执行的操作。

它应包含对所分组的权限允许执行哪些操作的描述。

#### 示例

标题为“Example”的章节

此能力允许 `main` 窗口访问 `filesystem`（文件系统）写入相关命令和 `dialog`（对话框）命令，以实现对用户所选文件的程序化访问。

### identifier

名为“identifier”的章节

`string`

能力的标识符。

#### 示例

标题为“Example”的章节

`main-user-files-write`

### local

标题为“local”的章节

`布尔值 (boolean)`

此能力是否对本地应用 URL 启用。默认为 `true`。

**默认值** : `true`

### permissions

标题为“permissions”的章节

`PermissionEntry`[] 每项必须唯一

附加到此能力的权限列表。

必须包含插件名称作为前缀，格式为 `${plugin-name}:${permission-name}`。对于直接在应用程序本身实现的命令，只需 `${permission-name}` 即可。

#### 示例

标题为“Example”的章节

    [

      "core:default",

      "shell:allow-open",

      "dialog:open",

      {

        "identifier": "fs:allow-write-text-file",

        "allow": [{ "path": "$HOME/test.txt" }]

      }

    ]

### platforms

标题为“platforms”的章节

`Target`[] | `null`

限制此能力适用的目标平台。

默认情况下，目标为所有平台。

#### 示例

标题为“Example”的章节

`["macOS","windows"]`

### remote

标题为“remote”的章节

`CapabilityRemote` | `null`

配置可使用该能力权限的远程 URL。

此设置是可选的，默认情况下不进行设置，因为我们的默认用例是内容由本地应用程序提供。

注意

请务必了解将本地系统访问权限提供给远程源所带来的安全影响。

#### 示例

标题为“Example”的章节

    {

      "urls": ["https://*.mydomain.dev"]

    }

### webviews

标题为“webviews”的章节

`string`[]

受此能力影响的 webview 列表。可以是 glob 模式。

无论 webview 的窗口标签是否匹配 [`Self::windows`] 中的模式，该能力都将在标签匹配此列表中任何模式的所有 webview 上启用。

#### 示例

标题为“Example”的章节

`["sub-webview-one", "sub-webview-two"]`

### windows

标题为“windows”的章节

`string`[]

受此能力影响的窗口列表。可以是 glob 模式。

如果窗口标签匹配此列表中的任何模式，则该能力将在该窗口的所有 webview 上启用，而无需考虑 [`Self::webviews`] 的值。

在包含多个 webview 的窗口上，建议优先指定 [`Self::webviews`] 并省略 [`Self::windows`]，以实现细粒度的访问控制。

#### 示例

标题为“Example”的章节

`["main"]`

## 定义

名为“定义”的章节

### CapabilityRemote

标题为“CapabilityRemote”的章节

与该能力关联的远程 URL 配置。

**对象属性** :

  * urls (必需)

##### urls

标题为“urls”的章节

`string`[]

此能力所指的远程域，使用 [URLPattern 标准](https://urlpattern.spec.whatwg.org/)。

###### 示例

名为“示例”的章节

  * “https://*.mydomain.dev”：允许 mydomain.dev 的子域
  * “<https://mydomain.dev/api/>*”：允许 mydomain.dev/api 的任何子路径

### 标识符

标题为“Identifier”的章节

`string`

### 数字

标题为“Number”的章节

**以下任何一种** :

  * `integer` 格式为 `int64`，表示 [`i64`]。
  * `number` 格式为 `double`，表示 [`f64`]。

一个有效的 ACL 数字。

### PermissionEntry

标题为“PermissionEntry”的章节

**以下任何一种** :

  * `Identifier` 通过标识符引用权限或权限集。
  * 通过标识符引用权限或权限集并扩展其范围。 **对象属性** : - allow - deny - identifier (必需) ##### allow `Value`[] | `null` 定义作用域允许内容的数据。 ##### deny `Value`[] | `null` 定义作用域拒绝内容的数据。验证逻辑应优先处理此项。 ##### identifier `Identifier` 权限或权限集的标识符。

[`Capability`] 中权限值的条目可以是原始权限 [`Identifier`]，也可以是引用权限并扩展其范围的对象。

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