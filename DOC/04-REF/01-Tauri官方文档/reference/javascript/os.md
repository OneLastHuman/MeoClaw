# @tauri-apps/plugin-os

_Source: https://v2.tauri.org.cn/reference/javascript/os/_

提供与操作系统相关的实用方法和属性。

## 类型别名 (Type Aliases)

名为“类型别名”的部分

### Arch

名为“Arch”的章节

    type Arch:

      | "x86"

      | "x86_64"

      | "arm"

      | "aarch64"

      | "mips"

      | "mips64"

      | "powerpc"

      | "powerpc64"

      | "riscv64"

      | "s390x"

      | "sparc64";

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/os/guest-js/index.ts#L42>

* * *

### Family

名为“Family”的章节

    type Family: "unix" | "windows";

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/os/guest-js/index.ts#L97>

* * *

### OsType

名为“OsType”的章节

    type OsType:

      | "linux"

      | "windows"

      | "macos"

      | "ios"

      | "android";

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/os/guest-js/index.ts#L40>

* * *

### 平台

名为“Platform”的章节

    type Platform:

      | "linux"

      | "macos"

      | "ios"

      | "freebsd"

      | "dragonfly"

      | "netbsd"

      | "openbsd"

      | "solaris"

      | "android"

      | "windows";

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/os/guest-js/index.ts#L28>

## 函数

名为“函数”的部分

### arch()

名为“arch()”的章节

    function arch(): Arch

返回当前的操作系统架构。可能的值为 `'x86'`, `'x86_64'`, `'arm'`, `'aarch64'`, `'mips'`, `'mips64'`, `'powerpc'`, `'powerpc64'`, `'riscv64'`, `'s390x'`, `'sparc64'`。

#### 返回

名为“返回值”的部分

[`Arch`](/reference/javascript/os/#arch)

#### 示例

标题为“Example”的章节

    import { arch } from '@tauri-apps/plugin-os';

    const archName = arch();

#### 始于

名为“起始版本”的部分

2.0.0

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/os/guest-js/index.ts#L138>

* * *

### eol()

名为“eol()”的章节

    function eol(): string

返回特定于操作系统的行尾标记。

  * `\n` (POSIX 系统)
  * `\r\n` (Windows 系统)

#### 返回

名为“返回值”的部分

`string`

#### 始于

名为“起始版本”的部分

2.0.0

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/os/guest-js/index.ts#L62>

* * *

### exeExtension()

名为“exeExtension()”的章节

    function exeExtension(): string

返回该平台上可执行二进制文件的文件扩展名（如果有）。可能的值为 `'exe'` 或 `''`（空字符串）。

#### 返回

名为“返回值”的部分

`string`

#### 示例

标题为“Example”的章节

    import { exeExtension } from '@tauri-apps/plugin-os';

    const exeExt = exeExtension();

#### 始于

名为“起始版本”的部分

2.0.0

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/os/guest-js/index.ts#L152>

* * *

### family()

名为“family()”的章节

    function family(): Family

返回当前操作系统族。可能的值为 `'unix'`, `'windows'`。

#### 返回

名为“返回值”的部分

[`Family`](/reference/javascript/os/#family)

#### 示例

标题为“Example”的章节

    import { family } from '@tauri-apps/plugin-os';

    const family = family();

#### 始于

名为“起始版本”的部分

2.0.0

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/os/guest-js/index.ts#L109>

* * *

### hostname()

名为“hostname()”的章节

    function hostname(): Promise<string | null>

返回操作系统的主机名。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`string` | `null`>

#### 示例

标题为“Example”的章节

    import { hostname } from '@tauri-apps/plugin-os';

    const hostname = await hostname();

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/os/guest-js/index.ts#L181>

* * *

### locale()

名为“locale()”的章节

    function locale(): Promise<string | null>

返回包含 `BCP-47` 语言标记的字符串。如果无法获取区域设置，则返回 `null`。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`string` | `null`>

#### 示例

标题为“Example”的章节

    import { locale } from '@tauri-apps/plugin-os';

    const locale = await locale();

    if (locale) {

       // use the locale string here

    }

#### 始于

名为“起始版本”的部分

2.0.0

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/os/guest-js/index.ts#L169>

* * *

### platform()

名为“platform()”的章节

    function platform(): Platform

返回描述当前使用的具体操作系统的字符串。该值在编译时设置。可能的值为 `'linux'`, `'macos'`, `'ios'`, `'freebsd'`, `'dragonfly'`, `'netbsd'`, `'openbsd'`, `'solaris'`, `'android'`, `'windows'`

#### 返回

名为“返回值”的部分

[`平台`](/reference/javascript/os/#platform)

#### 示例

标题为“Example”的章节

    import { platform } from '@tauri-apps/plugin-os';

    const platformName = platform();

#### 始于

名为“起始版本”的部分

2.0.0

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/os/guest-js/index.ts#L79>

* * *

### type()

名为“type()”的章节

    function type(): OsType

返回当前的操作系统类型。Linux 上返回 `'linux'`，macOS 上返回 `'macos'`，Windows 上返回 `'windows'`，iOS 上返回 `'ios'`，Android 上返回 `'android'`。

#### 返回

名为“返回值”的部分

[`OsType`](/reference/javascript/os/#ostype)

#### 示例

标题为“Example”的章节

    import { type } from '@tauri-apps/plugin-os';

    const osType = type();

#### 始于

名为“起始版本”的部分

2.0.0

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/os/guest-js/index.ts#L123>

* * *

### version()

名为“version()”的章节

    function version(): string

返回当前操作系统的版本。

#### 返回

名为“返回值”的部分

`string`

#### 示例

标题为“Example”的章节

    import { version } from '@tauri-apps/plugin-os';

    const osVersion = version();

#### 始于

名为“起始版本”的部分

2.0.0

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/os/guest-js/index.ts#L93>