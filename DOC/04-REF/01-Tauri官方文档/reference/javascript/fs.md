# @tauri-apps/plugin-fs

_Source: https://v2.tauri.org.cn/reference/javascript/fs/_

访问文件系统。

## iOS 安全范围资源

标题为“iOS 安全范围资源”的章节

在 iOS 上，当访问文件 URL 时，`fs` 插件会自动管理对安全范围资源的访问。这是访问应用沙箱外部文件（例如通过文件选择器）所必需的。

## 示例

标题为“Example”的章节

    import { open } from '@tauri-apps/plugin-fs';

    const file = await open('file:///path/to/file.txt');

    await file.close();

## 安全

名为“安全”的章节

此模块可防止路径遍历，不允许使用父目录访问器（即不允许使用 “/usr/path/to/../file” 或 ”../path/to/file” 路径）。使用此 API 访问的路径必须是相对于某个 [基础目录 (base directories)](/reference/javascript/fs/#basedirectory) 的路径，或者是使用 [path API](https://v2.tauri.org.cn/reference/javascript/api/namespacepath/) 创建的。

该 API 具有一个作用域配置，强制要求您使用 glob 模式来限制可访问的路径。

作用域配置是一个描述允许访问的文件/目录路径的 glob 模式数组。例如，此作用域配置允许**所有** 启用的 `fs` API（仅）访问 [`$APPDATA` 目录](https://v2.tauri.org.cn/reference/javascript/api/namespacepath/#appdatadir) 下的 _databases_ 目录中的文件。

    {

      "permissions": [

        {

          "identifier": "fs:scope",

          "allow": [{ "path": "$APPDATA/databases/*" }]

        }

      ]

    }

作用域也可以通过使用 API 的标识符而不是 `fs:scope` 来应用于特定的 `fs` API。

    {

      "permissions": [

        {

          "identifier": "fs:allow-exists",

          "allow": [{ "path": "$APPDATA/databases/*" }]

        }

      ]

    }

请注意 `$APPDATA` 变量的使用。该值在运行时注入，解析为 [应用数据目录](https://v2.tauri.org.cn/reference/javascript/api/namespacepath/#appdatadir)。

可用的变量有：[`$APPCONFIG`](https://v2.tauri.org.cn/reference/javascript/api/namespacepath/#appconfigdir), [`$APPDATA`](https://v2.tauri.org.cn/reference/javascript/api/namespacepath/#appdatadir), [`$APPLOCALDATA`](https://v2.tauri.org.cn/reference/javascript/api/namespacepath/#applocaldatadir), [`$APPCACHE`](https://v2.tauri.org.cn/reference/javascript/api/namespacepath/#appcachedir), [`$APPLOG`](https://v2.tauri.org.cn/reference/javascript/api/namespacepath/#applogdir), [`$AUDIO`](https://v2.tauri.org.cn/reference/javascript/api/namespacepath/#audiodir), [`$CACHE`](https://v2.tauri.org.cn/reference/javascript/api/namespacepath/#cachedir), [`$CONFIG`](https://v2.tauri.org.cn/reference/javascript/api/namespacepath/#configdir), [`$DATA`](https://v2.tauri.org.cn/reference/javascript/api/namespacepath/#datadir), [`$LOCALDATA`](https://v2.tauri.org.cn/reference/javascript/api/namespacepath/#localdatadir), [`$DESKTOP`](https://v2.tauri.org.cn/reference/javascript/api/namespacepath/#desktopdir), [`$DOCUMENT`](https://v2.tauri.org.cn/reference/javascript/api/namespacepath/#documentdir), [`$DOWNLOAD`](https://v2.tauri.org.cn/reference/javascript/api/namespacepath/#downloaddir), [`$EXE`](https://v2.tauri.org.cn/reference/javascript/api/namespacepath/#executabledir), [`$FONT`](https://v2.tauri.org.cn/reference/javascript/api/namespacepath/#fontdir), [`$HOME`](https://v2.tauri.org.cn/reference/javascript/api/namespacepath/#homedir), [`$PICTURE`](https://v2.tauri.org.cn/reference/javascript/api/namespacepath/#picturedir), [`$PUBLIC`](https://v2.tauri.org.cn/reference/javascript/api/namespacepath/#publicdir), [`$RUNTIME`](https://v2.tauri.org.cn/reference/javascript/api/namespacepath/#runtimedir), [`$TEMPLATE`](https://v2.tauri.org.cn/reference/javascript/api/namespacepath/#templatedir), [`$VIDEO`](https://v2.tauri.org.cn/reference/javascript/api/namespacepath/#videodir), [`$RESOURCE`](https://v2.tauri.org.cn/reference/javascript/api/namespacepath/#resourcedir), [`$TEMP`](https://v2.tauri.org.cn/reference/javascript/api/namespacepath/#tempdir)。

尝试使用未在作用域中配置的 URL 执行任何 API，都会因为拒绝访问而导致 Promise 被拒绝 (rejection)。

## 枚举

标题为“枚举”的章节

### BaseDirectory (基础目录)

标题为“BaseDirectory”的章节

#### 始于

名为“起始版本”的部分

2.0.0

#### 枚举成员

标题为“枚举成员”的章节

##### AppCache (应用缓存目录)

标题为“AppCache”的章节

    AppCache: 16;

###### 查看

标题为“参见”的章节

有关更多信息，请参阅 appCacheDir。

**来源** : 未定义

##### AppConfig

名为“AppConfig”的章节

    AppConfig: 13;

###### 查看

标题为“参见”的章节

有关更多信息，请参阅 appConfigDir。

**来源** : 未定义

##### AppData (应用数据目录)

标题为“AppData”的章节

    AppData: 14;

###### 查看

标题为“参见”的章节

有关更多信息，请参阅 appDataDir。

**来源** : 未定义

##### AppLocalData (应用本地数据目录)

标题为“AppLocalData”的章节

    AppLocalData: 15;

###### 查看

标题为“参见”的章节

有关更多信息，请参阅 appLocalDataDir。

**来源** : 未定义

##### AppLog (应用日志目录)

标题为“AppLog”的章节

    AppLog: 17;

###### 查看

标题为“参见”的章节

有关更多信息，请参阅 appLogDir。

**来源** : 未定义

##### Audio (音频目录)

标题为“Audio”的章节

    Audio: 1;

###### 查看

标题为“参见”的章节

有关更多信息，请参阅 audioDir。

**来源** : 未定义

##### Cache (缓存目录)

标题为“Cache”的章节

    Cache: 2;

###### 查看

标题为“参见”的章节

有关更多信息，请参阅 cacheDir。

**来源** : 未定义

##### Config

标题为“配置”的章节

    Config: 3;

###### 查看

标题为“参见”的章节

有关更多信息，请参阅 configDir。

**来源** : 未定义

##### Data (数据目录)

标题为“Data”的章节

    Data: 4;

###### 查看

标题为“参见”的章节

有关更多信息，请参阅 dataDir。

**来源** : 未定义

##### Desktop

标题为“Desktop”的章节

    Desktop: 18;

###### 查看

标题为“参见”的章节

有关更多信息，请参阅 desktopDir。

**来源** : 未定义

##### Document (文档目录)

标题为“Document”的章节

    Document: 6;

###### 查看

标题为“参见”的章节

有关更多信息，请参阅 documentDir。

**来源** : 未定义

##### Download (下载目录)

标题为“Download”的章节

    Download: 7;

###### 查看

标题为“参见”的章节

有关更多信息，请参阅 downloadDir。

**来源** : 未定义

##### Executable (可执行文件目录)

标题为“Executable”的章节

    Executable: 19;

###### 查看

标题为“参见”的章节

有关更多信息，请参阅 executableDir。

**来源** : 未定义

##### Font (字体目录)

标题为“Font”的章节

    Font: 20;

###### 查看

标题为“参见”的章节

有关更多信息，请参阅 fontDir。

**来源** : 未定义

##### Home

标题为“Home”的章节

    Home: 21;

###### 查看

标题为“参见”的章节

有关更多信息，请参阅 homeDir。

**来源** : 未定义

##### LocalData (本地数据目录)

标题为“LocalData”的章节

    LocalData: 5;

###### 查看

标题为“参见”的章节

有关更多信息，请参阅 localDataDir。

**来源** : 未定义

##### Picture (图片目录)

标题为“Picture”的章节

    Picture: 8;

###### 查看

标题为“参见”的章节

有关更多信息，请参阅 pictureDir。

**来源** : 未定义

##### Public

标题为“公开”的部分

    Public: 9;

###### 查看

标题为“参见”的章节

有关更多信息，请参阅 publicDir。

**来源** : 未定义

##### Resource

名为“Resource”的部分

    Resource: 11;

###### 查看

标题为“参见”的章节

有关更多信息，请参阅 resourceDir。

**来源** : 未定义

##### Runtime (运行时目录)

标题为“Runtime”的章节

    Runtime: 22;

###### 查看

标题为“参见”的章节

有关更多信息，请参阅 runtimeDir。

**来源** : 未定义

##### Temp (临时目录)

标题为“Temp”的章节

    Temp: 12;

###### 查看

标题为“参见”的章节

有关更多信息，请参阅 tempDir。

**来源** : 未定义

##### Template (模板目录)

标题为“Template”的章节

    Template: 23;

###### 查看

标题为“参见”的章节

有关更多信息，请参阅 templateDir。

**来源** : 未定义

##### Video (视频目录)

标题为“Video”的章节

    Video: 10;

###### 查看

标题为“参见”的章节

有关更多信息，请参阅 videoDir。

**来源** : 未定义

* * *

### SeekMode (寻址模式)

标题为“SeekMode”的章节

#### 枚举成员

标题为“枚举成员”的章节

##### Current (当前位置)

标题为“Current”的章节

    Current: 1;

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L93>

##### End (末尾)

标题为“End”的章节

    End: 2;

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L94>

##### Start (起始位置)

标题为“Start”的章节

    Start: 0;

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L92>

## 类

名为“类”的部分

### FileHandle (文件句柄)

标题为“FileHandle”的章节

用于读取和写入文件的 Tauri 抽象层。

#### 始于

名为“起始版本”的部分

2.0.0

#### 继承 (Extends)

名为“继承自”的章节

  * `Resource`

#### 构造函数

名为“构造函数”的部分

##### new FileHandle()

标题为“new FileHandle()”的章节

    new FileHandle(rid): FileHandle

###### 参数

名为“参数”的部分

参数| 类型
---|---
`rid`| `数字`

###### 返回

名为“返回值”的部分

[`FileHandle (文件句柄)`](/reference/javascript/fs/#filehandle)

###### 继承自 (Inherited from)

名为“继承自”的章节

`Resource.constructor`

**来源** : 未定义

#### 访问器

名为“访问器”的部分

##### rid

名为“rid”的部分

    get rid(): number

###### 返回

名为“返回值”的部分

`数字`

###### 继承自 (Inherited from)

名为“继承自”的章节

`Resource.rid`

**来源** : 未定义

#### 方法

名为“方法”的部分

##### close()

名为“close()”的部分

    close(): Promise<void>

销毁并从内存中清理此资源。**您不应再对此对象调用任何方法，并应放弃对它的任何引用。**

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

###### 继承自 (Inherited from)

名为“继承自”的章节

`Resource.close`

**来源** : 未定义

##### read()

标题为“read()”的章节

    read(buffer): Promise<null | number>

读取最多 `p.byteLength` 字节到 `p` 中。它将解析为读取的字节数（`0` < `n` <= `p.byteLength`），如果遇到任何错误则拒绝。即使 `read()` 解析为 `n` < `p.byteLength`，它在调用过程中也可能使用整个 `p` 作为暂存空间。如果部分数据可用但少于 `p.byteLength` 字节，`read()` 通常会解析为可用数据量，而不是等待更多数据。

当 `read()` 遇到文件结束条件时，它会解析为 EOF（`null`）。

当 `read()` 遇到错误时，它会以错误拒绝。

调用者应始终在考虑 EOF（`null`）之前处理返回的 `n` > `0` 字节。这样做可以正确处理在读取部分字节后发生的 I/O 错误，并正确处理两种允许的 EOF 行为。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`buffer`| [`Uint8Array`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array)

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`null` | `number`>

###### 示例

标题为“Example”的章节

    import { open, BaseDirectory } from "@tauri-apps/plugin-fs"

    // if "$APPCONFIG/foo/bar.txt" contains the text "hello world":

    const file = await open("foo/bar.txt", { baseDir: BaseDirectory.AppConfig });

    const buf = new Uint8Array(100);

    const numberOfBytesRead = await file.read(buf); // 11 bytes

    const text = new TextDecoder().decode(buf);  // "hello world"

    await file.close();

###### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L327>

##### seek()

标题为“seek()”的章节

    seek(offset, whence): Promise<number>

Seek 会将下一次 `read()` 或 `write()` 的偏移量设置为 offset，并根据 `whence` 进行解释：`Start` 表示相对于文件起始处，`Current` 表示相对于当前偏移量，`End` 表示相对于文件末尾。Seek 解析为相对于文件起始处的新偏移量。

定位到文件起始之前的偏移量是错误的。定位到任何正偏移量是合法的，但随后对底层对象进行 I/O 操作的行为取决于实现。它会返回光标位置的数值。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`offset`| `数字`
`whence`| [`SeekMode (寻址模式)`](/reference/javascript/fs/#seekmode)

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`number`>

###### 示例

标题为“Example”的章节

    import { open, SeekMode, BaseDirectory } from '@tauri-apps/plugin-fs';

    // Given hello.txt pointing to file with "Hello world", which is 11 bytes long:

    const file = await open('hello.txt', { read: true, write: true, truncate: true, create: true, baseDir: BaseDirectory.AppLocalData });

    await file.write(new TextEncoder().encode("Hello world"));

    // Seek 6 bytes from the start of the file

    console.log(await file.seek(6, SeekMode.Start)); // "6"

    // Seek 2 more bytes from the current position

    console.log(await file.seek(2, SeekMode.Current)); // "8"

    // Seek backwards 2 bytes from the end of the file

    console.log(await file.seek(-2, SeekMode.End)); // "9" (e.g. 11-2)

    await file.close();

###### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L382>

##### stat()

标题为“stat()”的章节

    stat(): Promise<FileInfo>

返回此文件的 [`FileInfo`](/reference/javascript/fs/#fileinfo)。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`FileInfo`](/reference/javascript/fs/#fileinfo)>

###### 示例

标题为“Example”的章节

    import { open, BaseDirectory } from '@tauri-apps/plugin-fs';

    const file = await open("file.txt", { read: true, baseDir: BaseDirectory.AppLocalData });

    const fileInfo = await file.stat();

    console.log(fileInfo.isFile); // true

    await file.close();

###### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L404>

##### truncate()

标题为“truncate()”的章节

    truncate(len?): Promise<void>

截断或扩展此文件，以达到指定的 `len`。如果未指定 `len`，则截断整个文件内容。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`len`?| `数字`

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

###### 示例

标题为“Example”的章节

    import { open, BaseDirectory } from '@tauri-apps/plugin-fs';

    // truncate the entire file

    const file = await open("my_file.txt", { read: true, write: true, create: true, baseDir: BaseDirectory.AppLocalData });

    await file.truncate();

    // truncate part of the file

    const file = await open("my_file.txt", { read: true, write: true, create: true, baseDir: BaseDirectory.AppLocalData });

    await file.write(new TextEncoder().encode("Hello World"));

    await file.truncate(7);

    const data = new Uint8Array(32);

    await file.read(data);

    console.log(new TextDecoder().decode(data)); // Hello W

    await file.close();

###### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L436>

##### write()

名为“write()”的章节

    write(data): Promise<number>

从 `data` 写入 `data.byteLength` 字节到基础数据流中。它会解析为从 `data` 写入的字节数（`0` <= `n` <= `data.byteLength`），或者以导致写入提前停止的错误拒绝。如果 `write()` 解析为 `n` < `data.byteLength`，则必须以非空错误拒绝。`write()` 不得修改切片数据，即使是临时修改。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`data`| [`Uint8Array`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array)

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`number`>

###### 示例

标题为“Example”的章节

    import { open, write, BaseDirectory } from '@tauri-apps/plugin-fs';

    const encoder = new TextEncoder();

    const data = encoder.encode("Hello world");

    const file = await open("bar.txt", { write: true, baseDir: BaseDirectory.AppLocalData });

    const bytesWritten = await file.write(data); // 11

    await file.close();

###### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L463>

## 接口

名为“接口”的部分

### CopyFileOptions (复制文件选项)

标题为“CopyFileOptions”的章节

#### 始于

名为“起始版本”的部分

2.0.0

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
`fromPathBaseDir?`| [`BaseDirectory (基础目录)`](/reference/javascript/fs/#basedirectory)| `fromPath` 的基础目录。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L599>
`toPathBaseDir?`| [`BaseDirectory (基础目录)`](/reference/javascript/fs/#basedirectory)| `toPath` 的基础目录。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L601>

* * *

### CreateOptions (创建选项)

标题为“CreateOptions”的章节

#### 始于

名为“起始版本”的部分

2.0.0

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
`baseDir?`| [`BaseDirectory (基础目录)`](/reference/javascript/fs/#basedirectory)| `path` 的基础目录。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L476>

* * *

### DebouncedWatchOptions (防抖监视选项)

标题为“DebouncedWatchOptions”的章节

#### 始于

名为“起始版本”的部分

2.0.0

#### 继承 (Extends)

名为“继承自”的章节

  * [`WatchOptions (监视选项)`](/reference/javascript/fs/#watchoptions)

#### 属性

名为“属性”的部分

属性| 类型| 描述| 继承自 (Inherited from)| 定义于
---|---|---|---|---
`baseDir?`| [`BaseDirectory (基础目录)`](/reference/javascript/fs/#basedirectory)| `path` 的基础目录。| [`WatchOptions`](/reference/javascript/fs/#watchoptions).[`baseDir`](/reference/javascript/fs/#basedir-10)| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L1195>
`delayMs?`| `数字`| 防抖延迟时间。| -| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L1203>
`recursive?`| `布尔值 (boolean)`| 递归监视目录。| [`WatchOptions`](/reference/javascript/fs/#watchoptions).[`recursive`](/reference/javascript/fs/#recursive-3)| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L1193>

* * *

### DirEntry (目录条目)

标题为“DirEntry”的章节

磁盘条目，可以是文件、目录或符号链接。

这是 [`readDir`](/reference/javascript/fs/#readdir) 的结果。

#### 始于

名为“起始版本”的部分

2.0.0

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
`isDirectory`| `布尔值 (boolean)`| 指定此条目是否为目录。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L690>
`isFile`| `布尔值 (boolean)`| 指定此条目是否为文件。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L692>
`isSymlink`| `布尔值 (boolean)`| 指定此条目是否为符号链接。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L694>
`name`| `string`| 条目名称（带扩展名的文件名或目录名）。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L688>

* * *

### ExistsOptions (存在性检查选项)

标题为“ExistsOptions”的章节

#### 始于

名为“起始版本”的部分

2.0.0

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
`baseDir?`| [`BaseDirectory (基础目录)`](/reference/javascript/fs/#basedirectory)| `path` 的基础目录。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L1160>

* * *

### FileInfo (文件信息)

标题为“FileInfo”的章节

FileInfo 描述了一个文件，由 `stat`、`lstat` 或 `fstat` 返回。

#### 始于

名为“起始版本”的部分

2.0.0

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
`atime`| `null` | [`Date`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Date)| 文件的最后访问时间。这对应于 Unix 上 `stat` 的 `atime` 字段以及 Windows 上 `ftLastAccessTime`。此信息并非在所有平台上都可用。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L133>
`birthtime`| `null` | [`Date`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Date)| 文件的创建时间。这对应于 Mac/BSD 上 `stat` 的 `birthtime` 字段以及 Windows 上 `ftCreationTime`。此信息并非在所有平台上都可用。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L139>
`blksize`| `null` | `number`| 文件系统 I/O 的块大小。特定于平台 - **Windows:** 不支持。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L216>
`blocks`| `null` | `number`| 分配给文件的块数（以 512 字节为单位）。特定于平台 - **Windows:** 不支持。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L224>
`dev`| `null` | `number`| 包含该文件的设备 ID。特定于平台 - **Windows:** 不支持。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L159>
`fileAttributes`| `null` | `number`| 此字段包含文件或目录的文件系统属性信息。有关可能的值及其说明，请参阅 Windows 开发中心中的 [文件属性常量](https://docs.microsoft.com/en-us/windows/win32/fileio/file-attribute-constants)。特定于平台 - **macOS / Linux / Android / iOS:** 不支持。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L151>
`gid`| `null` | `number`| 该文件所有者的组 ID。特定于平台 - **Windows:** 不支持。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L200>
`ino`| `null` | `number`| Inode 编号。特定于平台 - **Windows:** 不支持。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L167>
`isDirectory`| `布尔值 (boolean)`| 如果这是常规目录的信息，则为 true。与 `FileInfo.isFile` 和 `FileInfo.isSymlink` 互斥。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L112>
`isFile`| `布尔值 (boolean)`| 如果这是常规文件的信息，则为 true。与 `FileInfo.isDirectory` 和 `FileInfo.isSymlink` 互斥。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L107>
`isSymlink`| `布尔值 (boolean)`| 如果这是符号链接的信息，则为 true。与 `FileInfo.isFile` 和 `FileInfo.isDirectory` 互斥。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L117>
`mode`| `null` | `number`| 包含此文件/目录的标准 Unix 权限的底层原始 `st_mode` 位。特定于平台 - **Windows:** 不支持。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L176>
`mtime`| `null` | [`Date`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Date)| 文件的最后修改时间。这对应于 Linux/Mac OS 上 `stat` 的 `mtime` 字段以及 Windows 上 `ftLastWriteTime`。此信息并非在所有平台上都可用。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L127>
`nlink`| `null` | `number`| 指向此文件的硬链接数。特定于平台 - **Windows:** 不支持。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L184>
`rdev`| `null` | `number`| 此文件的设备 ID。特定于平台 - **Windows:** 不支持。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L208>
`readonly`| `布尔值 (boolean)`| 该文件是否为只读（不可写）。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L141>
`size`| `数字`| 文件大小，以字节为单位。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L121>
`uid`| `null` | `number`| 该文件所有者的用户 ID。特定于平台 - **Windows:** 不支持。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L192>

* * *

### MkdirOptions (创建目录选项)

标题为“MkdirOptions”的章节

#### 始于

名为“起始版本”的部分

2.0.0

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
`baseDir?`| [`BaseDirectory (基础目录)`](/reference/javascript/fs/#basedirectory)| `path` 的基础目录。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L644>
`mode?`| `数字`| 创建目录时使用的权限（默认为 `0o777`，在进程的 umask 之前）。在 Windows 上会被忽略。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L638>
`recursive?`| `布尔值 (boolean)`| 默认为 `false`。如果设置为 `true`，则意味着任何中间目录也会被创建（类似于 shell 命令 `mkdir -p`）。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L642>

* * *

### OpenOptions (打开选项)

标题为“OpenOptions”的章节

#### 始于

名为“起始版本”的部分

2.0.0

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
`append?`| `布尔值 (boolean)`| 设置追加模式的选项。此选项为 `true` 时，表示写入将追加到文件末尾，而不是覆盖之前的内容。请注意，设置 `{ write: true, append: true }` 与仅设置 `{ append: true }` 的效果相同。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L531>
`baseDir?`| [`BaseDirectory (基础目录)`](/reference/javascript/fs/#basedirectory)| `path` 的基础目录。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L559>
`create?`| `布尔值 (boolean)`| 设置允许在指定路径不存在文件时创建新文件的选项。使用时需要写或追加权限。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L544>
`createNew?`| `布尔值 (boolean)`| 默认为 `false`。如果设置为 `true`，则目标位置不允许存在任何文件、目录或符号链接。使用时需要写或追加权限。当 createNew 设置为 `true` 时，create 和 truncate 会被忽略。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L551>
`mode?`| `数字`| 如果创建文件时使用的权限（默认为 `0o666`，在进程的 umask 之前）。在 Windows 上会被忽略。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L557>
`read?`| `布尔值 (boolean)`| 设置读取访问权限的选项。此选项为 `true` 时，表示文件在打开时应该是可读的。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L517>
`truncate?`| `布尔值 (boolean)`| 设置截断之前文件的选项。如果以设置此选项成功打开文件，它将把现有文件截断为 `0` 大小。文件必须以写访问权限打开才能使截断工作。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L538>
`write?`| `布尔值 (boolean)`| 设置写入访问权限的选项。此选项为 `true` 时，表示文件在打开时应该是可写的。如果文件已存在，对它的任何写调用都将默认覆盖其内容，而不截断它。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L524>

* * *

### ReadDirOptions (读取目录选项)

标题为“ReadDirOptions”的章节

#### 始于

名为“起始版本”的部分

2.0.0

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
`baseDir?`| [`BaseDirectory (基础目录)`](/reference/javascript/fs/#basedirectory)| `path` 的基础目录。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L676>

* * *

### ReadFileOptions (读取文件选项)

标题为“ReadFileOptions”的章节

#### 始于

名为“起始版本”的部分

2.0.0

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
`baseDir?`| [`BaseDirectory (基础目录)`](/reference/javascript/fs/#basedirectory)| `path` 的基础目录。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L738>
`encoding?`| `string`| 读取文本文件时使用的文本编码。默认为 ‘utf-8’。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L740>

* * *

### RemoveOptions (移除选项)

标题为“RemoveOptions”的章节

#### 始于

名为“起始版本”的部分

2.0.0

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
`baseDir?`| [`BaseDirectory (基础目录)`](/reference/javascript/fs/#basedirectory)| `path` 的基础目录。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L882>
`recursive?`| `布尔值 (boolean)`| 默认为 `false`。如果设置为 `true`，即使路径是非空目录，也会被删除。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L880>

* * *

### RenameOptions (重命名选项)

标题为“RenameOptions”的章节

#### 始于

名为“起始版本”的部分

2.0.0

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
`newPathBaseDir?`| [`BaseDirectory (基础目录)`](/reference/javascript/fs/#basedirectory)| `newPath` 的基础目录。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L918>
`oldPathBaseDir?`| [`BaseDirectory (基础目录)`](/reference/javascript/fs/#basedirectory)| `oldPath` 的基础目录。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L916>

* * *

### StatOptions (状态选项)

标题为“StatOptions”的章节

#### 始于

名为“起始版本”的部分

2.0.0

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
`baseDir?`| [`BaseDirectory (基础目录)`](/reference/javascript/fs/#basedirectory)| `path` 的基础目录。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L960>

* * *

### TruncateOptions (截断选项)

名为“TruncateOptions”的章节

#### 始于

名为“起始版本”的部分

2.0.0

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
`baseDir?`| [`BaseDirectory (基础目录)`](/reference/javascript/fs/#basedirectory)| `path` 的基础目录。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L1019>

* * *

### WatchEvent (监视事件)

名为“WatchEvent”的章节

#### 始于

名为“起始版本”的部分

2.0.0

#### 属性

名为“属性”的部分

属性| 类型| 定义于
---|---|---
`attrs`| `未知`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L1212>
`paths`| `string`[]| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L1211>
`type`| [`WatchEventKind (监视事件类型)`](/reference/javascript/fs/#watcheventkind)| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L1210>

* * *

### WatchOptions (监视选项)

名为“WatchOptions”的章节

#### 始于

名为“起始版本”的部分

2.0.0

#### 扩展自

名为“扩展”的部分

  * [`DebouncedWatchOptions (防抖监视选项)`](/reference/javascript/fs/#debouncedwatchoptions)

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
`baseDir?`| [`BaseDirectory (基础目录)`](/reference/javascript/fs/#basedirectory)| `path` 的基础目录。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L1195>
`recursive?`| `布尔值 (boolean)`| 递归监视目录。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L1193>

* * *

### WriteFileOptions (写入文件选项)

名为“WriteFileOptions”的章节

#### 始于

名为“起始版本”的部分

2.0.0

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
`append?`| `布尔值 (boolean)`| 默认为 `false`。如果设置为 `true`，则会追加到文件末尾，而不是覆盖之前的内容。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L1063>
`baseDir?`| [`BaseDirectory (基础目录)`](/reference/javascript/fs/#basedirectory)| `path` 的基础目录。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L1071>
`create?`| `布尔值 (boolean)`| 设置是否允许在指定路径下创建新文件（如果该路径尚不存在）（默认为 `true`）。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L1065>
`createNew?`| `布尔值 (boolean)`| 设置创建新文件的选项，如果文件已存在则会失败。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L1067>
`mode?`| `数字`| 文件权限。在 Windows 上会被忽略。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L1069>

## 类型别名 (Type Aliases)

名为“类型别名”的部分

### UnwatchFn() (停止监视函数)

名为“UnwatchFn()”的章节

    type UnwatchFn: () => void;

#### 返回

名为“返回值”的部分

`空`

#### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L1277>

* * *

### WatchEventKind (监视事件类型)

名为“WatchEventKind”的章节

    type WatchEventKind:

      | "any"

      | object

      | object

      | object

      | object

      | "other";

#### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L1218>

* * *

### WatchEventKindAccess (访问事件类型)

名为“WatchEventKindAccess”的章节

    type WatchEventKindAccess: object | object | object | object;

#### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L1229>

* * *

### WatchEventKindCreate (创建事件类型)

名为“WatchEventKindCreate”的章节

    type WatchEventKindCreate: object | object | object | object;

#### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L1238>

* * *

### WatchEventKindModify (修改事件类型)

名为“WatchEventKindModify”的章节

    type WatchEventKindModify:

      | object

      | object

      | object

      | object

      | object;

#### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L1247>

* * *

### WatchEventKindRemove (移除事件类型)

名为“WatchEventKindRemove”的章节

    type WatchEventKindRemove: object | object | object | object;

#### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L1267>

## 函数

名为“函数”的部分

### copyFile()

名为“copyFile()”的章节

    function copyFile(

       fromPath,

       toPath,

    options?): Promise<void>

将一个文件的内容和权限复制到另一个指定路径。默认情况下，如果需要则创建一个新文件，否则覆盖现有文件。

#### 参数

名为“参数”的部分

参数| 类型
---|---
`fromPath`| `string` | [`URL`](https://mdn.org.cn/docs/Web/API/URL)
`toPath`| `string` | [`URL`](https://mdn.org.cn/docs/Web/API/URL)
`选项`?| [`CopyFileOptions (复制文件选项)`](/reference/javascript/fs/#copyfileoptions)

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

#### 示例

标题为“Example”的章节

    import { copyFile, BaseDirectory } from '@tauri-apps/plugin-fs';

    await copyFile('app.conf', 'app.conf.bk', { fromPathBaseDir: BaseDirectory.AppConfig, toPathBaseDir: BaseDirectory.AppConfig });

#### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L614>

* * *

### create()

名为“create()”的章节

    function create(path, options?): Promise<FileHandle>

如果文件不存在则创建一个文件，或者截断现有文件，并解析为一个 [`FileHandle`](/reference/javascript/fs/#filehandle) 实例。

#### 参数

名为“参数”的部分

参数| 类型
---|---
`path`| `string` | [`URL`](https://mdn.org.cn/docs/Web/API/URL)
`选项`?| [`CreateOptions (创建选项)`](/reference/javascript/fs/#createoptions)

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`FileHandle`](/reference/javascript/fs/#filehandle)>

#### 示例

标题为“Example”的章节

    import { create, BaseDirectory } from "@tauri-apps/plugin-fs"

    const file = await create("foo/bar.txt", { baseDir: BaseDirectory.AppConfig });

    await file.write(new TextEncoder().encode("Hello world"));

    await file.close();

#### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L493>

* * *

### exists()

名为“exists()”的章节

    function exists(path, options?): Promise<boolean>

检查路径是否存在。

#### 参数

名为“参数”的部分

参数| 类型
---|---
`path`| `string` | [`URL`](https://mdn.org.cn/docs/Web/API/URL)
`选项`?| [`ExistsOptions (存在性检查选项)`](/reference/javascript/fs/#existsoptions)

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`boolean`>

#### 示例

标题为“Example”的章节

    import { exists, BaseDirectory } from '@tauri-apps/plugin-fs';

    // Check if the `$APPDATA/avatar.png` file exists

    await exists('avatar.png', { baseDir: BaseDirectory.AppData });

#### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L1174>

* * *

### lstat()

名为“lstat()”的章节

    function lstat(path, options?): Promise<FileInfo>

解析为指定 `path` 的 [`FileInfo`](/reference/javascript/fs/#fileinfo)。如果 `path` 是符号链接，则会返回该符号链接本身的信息，而不是其指向的目标信息。

#### 参数

名为“参数”的部分

参数| 类型
---|---
`path`| `string` | [`URL`](https://mdn.org.cn/docs/Web/API/URL)
`选项`?| [`StatOptions (状态选项)`](/reference/javascript/fs/#statoptions)

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`FileInfo`](/reference/javascript/fs/#fileinfo)>

#### 示例

标题为“Example”的章节

    import { lstat, BaseDirectory } from '@tauri-apps/plugin-fs';

    const fileInfo = await lstat("hello.txt", { baseDir: BaseDirectory.AppLocalData });

    console.log(fileInfo.isFile); // true

#### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L1002>

* * *

### mkdir()

名为“mkdir()”的章节

    function mkdir(path, options?): Promise<void>

在指定路径下创建一个新目录。

#### 参数

名为“参数”的部分

参数| 类型
---|---
`path`| `string` | [`URL`](https://mdn.org.cn/docs/Web/API/URL)
`选项`?| [`MkdirOptions (创建目录选项)`](/reference/javascript/fs/#mkdiroptions)

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

#### 示例

标题为“Example”的章节

    import { mkdir, BaseDirectory } from '@tauri-apps/plugin-fs';

    await mkdir('users', { baseDir: BaseDirectory.AppLocalData });

#### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L657>

* * *

### open()

名为“open()”的章节

    function open(path, options?): Promise<FileHandle>

打开文件并解析为一个 [`FileHandle`](/reference/javascript/fs/#filehandle) 实例。如果使用 `create` 或 `createNew` 打开选项，则文件无需预先存在。调用者有责任在完成使用后关闭文件。

#### 参数

名为“参数”的部分

参数| 类型
---|---
`path`| `string` | [`URL`](https://mdn.org.cn/docs/Web/API/URL)
`选项`?| [`OpenOptions (打开选项)`](/reference/javascript/fs/#openoptions)

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`FileHandle`](/reference/javascript/fs/#filehandle)>

#### 示例

标题为“Example”的章节

    import { open, BaseDirectory } from "@tauri-apps/plugin-fs"

    const file = await open("foo/bar.txt", { read: true, write: true, baseDir: BaseDirectory.AppLocalData });

    // Do work with file

    await file.close();

#### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L578>

* * *

### readDir()

名为“readDir()”的章节

    function readDir(path, options?): Promise<DirEntry[]>

读取路径给定的目录并返回一个 `DirEntry` 数组。

#### 参数

名为“参数”的部分

参数| 类型
---|---
`path`| `string` | [`URL`](https://mdn.org.cn/docs/Web/API/URL)
`选项`?| [`ReadDirOptions (读取目录选项)`](/reference/javascript/fs/#readdiroptions)

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`DirEntry`](/reference/javascript/fs/#direntry)[]>

#### 示例

标题为“Example”的章节

    import { readDir, BaseDirectory } from '@tauri-apps/plugin-fs';

    import { join } from '@tauri-apps/api/path';

    const dir = "users"

    const entries = await readDir('users', { baseDir: BaseDirectory.AppLocalData });

    processEntriesRecursively(dir, entries);

    async function processEntriesRecursively(parent, entries) {

      for (const entry of entries) {

        console.log(`Entry: ${entry.name}`);

        if (entry.isDirectory) {

           const dir = await join(parent, entry.name);

          processEntriesRecursively(dir, await readDir(dir, { baseDir: BaseDirectory.AppLocalData }))

        }

      }

    }

#### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L719>

* * *

### readFile()

名为“readFile()”的章节

    function readFile(path, options?): Promise<Uint8Array>

读取文件的全部内容并解析为字节数组。如果需要，可以使用 TextDecoder 将字节转换为字符串。

#### 参数

名为“参数”的部分

参数| 类型
---|---
`path`| `string` | [`URL`](https://mdn.org.cn/docs/Web/API/URL)
`选项`?| [`ReadFileOptions (读取文件选项)`](/reference/javascript/fs/#readfileoptions)

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`Uint8Array`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array)>

#### 示例

标题为“Example”的章节

    import { readFile, BaseDirectory } from '@tauri-apps/plugin-fs';

    const contents = await readFile('avatar.png', { baseDir: BaseDirectory.Resource });

#### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L754>

* * *

### readTextFile()

名为“readTextFile()”的章节

    function readTextFile(path, options?): Promise<string>

读取文件的全部内容并作为字符串返回，使用指定的编码（默认：UTF-8）。

#### 参数

名为“参数”的部分

参数| 类型
---|---
`path`| `string` | [`URL`](https://mdn.org.cn/docs/Web/API/URL)
`选项`?| [`ReadFileOptions (读取文件选项)`](/reference/javascript/fs/#readfileoptions)

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`string`>

#### 示例

标题为“Example”的章节

    import { readTextFile, BaseDirectory } from '@tauri-apps/plugin-fs';

    const contents = await readTextFile('app.conf', { baseDir: BaseDirectory.AppConfig });

#### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L780>

* * *

### readTextFileLines()

名为“readTextFileLines()”的章节

    function readTextFileLines(path, options?): Promise<AsyncIterableIterator<string>>

返回一个针对文件行内容的异步迭代器（AsyncIterableIterator），使用指定的编码解码（默认：UTF-8）。

#### 参数

名为“参数”的部分

参数| 类型
---|---
`path`| `string` | [`URL`](https://mdn.org.cn/docs/Web/API/URL)
`选项`?| [`ReadFileOptions (读取文件选项)`](/reference/javascript/fs/#readfileoptions)

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`AsyncIterableIterator`<`string`>>

#### 示例

标题为“Example”的章节

    import { readTextFileLines, BaseDirectory } from '@tauri-apps/plugin-fs';

    const lines = await readTextFileLines('app.conf', { baseDir: BaseDirectory.AppConfig });

    for await (const line of lines) {

      console.log(line);

    }

您也可以调用 AsyncIterableIterator.next 来推进迭代器，以便在需要时懒加载地读取下一行。

#### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L813>

* * *

### remove()

名为“remove()”的章节

    function remove(path, options?): Promise<void>

移除指定的文件或目录。如果目录不为空且 `recursive` 选项未设置为 true，则 Promise 将被拒绝。

#### 参数

名为“参数”的部分

参数| 类型
---|---
`path`| `string` | [`URL`](https://mdn.org.cn/docs/Web/API/URL)
`选项`?| [`RemoveOptions (移除选项)`](/reference/javascript/fs/#removeoptions)

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

#### 示例

标题为“Example”的章节

    import { remove, BaseDirectory } from '@tauri-apps/plugin-fs';

    await remove('users/file.txt', { baseDir: BaseDirectory.AppLocalData });

    await remove('users', { baseDir: BaseDirectory.AppLocalData });

#### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L897>

* * *

### rename()

名为“rename()”的章节

    function rename(

       oldPath,

       newPath,

    options?): Promise<void>

将 oldpath 重命名（移动）为 newpath。路径可以是文件或目录。如果 newpath 已存在且不是目录，rename() 将替换它。当 oldpath 和 newpath 位于不同目录时，可能会应用特定于操作系统的限制。

在 Unix 上，此操作不会在任何路径上跟随符号链接。

#### 参数

名为“参数”的部分

参数| 类型
---|---
`oldPath`| `string` | [`URL`](https://mdn.org.cn/docs/Web/API/URL)
`newPath`| `string` | [`URL`](https://mdn.org.cn/docs/Web/API/URL)
`选项`?| [`RenameOptions (重命名选项)`](/reference/javascript/fs/#renameoptions)

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

#### 示例

标题为“Example”的章节

    import { rename, BaseDirectory } from '@tauri-apps/plugin-fs';

    await rename('avatar.png', 'deleted.png', { oldPathBaseDir: BaseDirectory.App, newPathBaseDir: BaseDirectory.AppLocalData });

#### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L936>

* * *

### size()

名为“size()”的章节

    function size(path): Promise<number>

获取文件或目录的大小。对于文件，也可以使用 `stat` 函数。

如果 `path` 是一个目录，此函数将递归地遍历 `path` 内部的每个文件和每个目录，因此在大型目录上使用时会非常耗时。

#### 参数

名为“参数”的部分

参数| 类型
---|---
`path`| `string` | [`URL`](https://mdn.org.cn/docs/Web/API/URL)

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`number`>

#### 示例

标题为“Example”的章节

    import { size, BaseDirectory } from '@tauri-apps/plugin-fs';

    // Get the size of the `$APPDATA/tauri` directory.

    const dirSize = await size('tauri', { baseDir: BaseDirectory.AppData });

    console.log(dirSize); // 1024

#### 始于

名为“起始版本”的部分

2.1.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L1359>

* * *

### startAccessingSecurityScopedResource()

名为“startAccessingSecurityScopedResource()”的章节

    function startAccessingSecurityScopedResource(path): Promise<void>

开始为给定的文件 URL 访问受安全范围限制（security-scoped）的资源。当您访问使用此类 URL（例如，来自文件选择器）打开的文件时，应调用此函数。

请注意，在 iOS 上，插件会自动管理对受安全范围限制资源的访问，因此除非您希望手动管理作用域，否则无需调用此函数。

当您完成访问资源后，必须调用 [`stopAccessingSecurityScopedResource`](/reference/javascript/fs/#stopaccessingsecurityscopedresource)。

平台特定

  * **iOS:** 开始访问受安全范围限制的资源。
  * **其他平台:** 不做任何操作。

#### 参数

名为“参数”的部分

参数| 类型
---|---
`path`| `string` | [`URL`](https://mdn.org.cn/docs/Web/API/URL)

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

#### 示例

标题为“Example”的章节

    import { startAccessingSecurityScopedResource } from '@tauri-apps/plugin-fs';

    const filePath = 'file:///path/to/file.txt';

    await startAccessingSecurityScopedResource(filePath);

    // ... use the resource ...

#### 始于

名为“起始版本”的部分

2.5.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L1395>

* * *

### stat()

标题为“stat()”的章节

    function stat(path, options?): Promise<FileInfo>

解析为指定 `path` 的 [`FileInfo`](/reference/javascript/fs/#fileinfo)。始终会跟随符号链接，但如果符号链接指向作用域外的路径，则会拒绝操作。

#### 参数

名为“参数”的部分

参数| 类型
---|---
`path`| `string` | [`URL`](https://mdn.org.cn/docs/Web/API/URL)
`选项`?| [`StatOptions (状态选项)`](/reference/javascript/fs/#statoptions)

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`FileInfo`](/reference/javascript/fs/#fileinfo)>

#### 示例

标题为“Example”的章节

    import { stat, BaseDirectory } from '@tauri-apps/plugin-fs';

    const fileInfo = await stat("hello.txt", { baseDir: BaseDirectory.AppLocalData });

    console.log(fileInfo.isFile); // true

#### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L976>

* * *

### stopAccessingSecurityScopedResource()

名为“stopAccessingSecurityScopedResource()”的章节

    function stopAccessingSecurityScopedResource(path): Promise<void>

停止访问给定的文件 URL 的受安全范围限制的资源。当您使用手动跟踪（通过 [`startAccessingSecurityScopedResource`](/reference/javascript/fs/#startaccessingsecurityscopedresource)）并在使用完使用安全范围 URL（例如，来自文件选择器）打开的文件后，应调用此函数。

平台特定

  * **iOS:** 停止访问受安全范围限制的资源。
  * **其他平台:** 不做任何操作。

#### 参数

名为“参数”的部分

参数| 类型
---|---
`path`| `string` | [`URL`](https://mdn.org.cn/docs/Web/API/URL)

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

#### 示例

标题为“Example”的章节

    import { stopAccessingSecurityScopedResource } from '@tauri-apps/plugin-fs';

    const filePath = 'file:///path/to/file.txt';

    await startAccessingSecurityScopedResource(filePath);

    // ... use the resource ...

    // when you're done with the resource:

    await stopAccessingSecurityScopedResource(filePath);

#### 始于

名为“起始版本”的部分

2.5.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L1430>

* * *

### truncate()

标题为“truncate()”的章节

    function truncate(

       path,

       len?,

    options?): Promise<void>

截断或扩展指定文件以达到指定的 `len`。如果 `len` 为 `0` 或未指定，则会清空整个文件内容。

#### 参数

名为“参数”的部分

参数| 类型
---|---
`path`| `string` | [`URL`](https://mdn.org.cn/docs/Web/API/URL)
`len`?| `数字`
`选项`?| [`TruncateOptions (截断选项)`](/reference/javascript/fs/#truncateoptions)

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

#### 示例

标题为“Example”的章节

    import { truncate, readTextFile, writeTextFile, BaseDirectory } from '@tauri-apps/plugin-fs';

    // truncate the entire file

    await truncate("my_file.txt", 0, { baseDir: BaseDirectory.AppLocalData });

    // truncate part of the file

    const filePath = "file.txt";

    await writeTextFile(filePath, "Hello World", { baseDir: BaseDirectory.AppLocalData });

    await truncate(filePath, 7, { baseDir: BaseDirectory.AppLocalData });

    const data = await readTextFile(filePath, { baseDir: BaseDirectory.AppLocalData });

    console.log(data);  // "Hello W"

#### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L1042>

* * *

### watch()

名为“watch()”的章节

    function watch(

       paths,

       cb,

    options?): Promise<UnwatchFn>

监控文件或目录的变更（会有延迟）。

#### 参数

名为“参数”的部分

参数| 类型
---|---
`paths`| `string` | [`URL`](https://mdn.org.cn/docs/Web/API/URL) | `string`[] | [`URL`](https://mdn.org.cn/docs/Web/API/URL)[]
`cb`| (`event`) => `void`
`选项`?| [`DebouncedWatchOptions (防抖监视选项)`](/reference/javascript/fs/#debouncedwatchoptions)

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`UnwatchFn`](/reference/javascript/fs/#unwatchfn)>

#### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L1316>

* * *

### watchImmediate()

名为“watchImmediate()”的章节

    function watchImmediate(

       paths,

       cb,

    options?): Promise<UnwatchFn>

监控文件或目录的变更。

#### 参数

名为“参数”的部分

参数| 类型
---|---
`paths`| `string` | [`URL`](https://mdn.org.cn/docs/Web/API/URL) | `string`[] | [`URL`](https://mdn.org.cn/docs/Web/API/URL)[]
`cb`| (`event`) => `void`
`选项`?| [`WatchOptions (监视选项)`](/reference/javascript/fs/#watchoptions)

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`UnwatchFn`](/reference/javascript/fs/#unwatchfn)>

#### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L1333>

* * *

### writeFile()

名为“writeFile()”的章节

    function writeFile(

       path,

       data,

    options?): Promise<void>

将 `data` 写入给定 `path`。默认情况下，如果需要则创建一个新文件，否则覆盖现有文件。

#### 参数

名为“参数”的部分

参数| 类型
---|---
`path`| `string` | [`URL`](https://mdn.org.cn/docs/Web/API/URL)
`data`| [`Uint8Array`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array) | [`ReadableStream`](https://mdn.org.cn/docs/Web/API/ReadableStream)<[`Uint8Array`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array)>
`选项`?| [`WriteFileOptions (写入文件选项)`](/reference/javascript/fs/#writefileoptions)

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

#### 示例

标题为“Example”的章节

    import { writeFile, BaseDirectory } from '@tauri-apps/plugin-fs';

    let encoder = new TextEncoder();

    let data = encoder.encode("Hello World");

    await writeFile('file.txt', data, { baseDir: BaseDirectory.AppLocalData });

#### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L1087>

* * *

### writeTextFile()

名为“writeTextFile()”的章节

    function writeTextFile(

       path,

       data,

    options?): Promise<void>

将 UTF-8 字符串 `data` 写入给定 `path`。默认情况下，如果需要则创建一个新文件，否则覆盖现有文件。

#### 参数

名为“参数”的部分

参数| 类型
---|---
`path`| `string` | [`URL`](https://mdn.org.cn/docs/Web/API/URL)
`data`| `string`
`选项`?| [`WriteFileOptions (写入文件选项)`](/reference/javascript/fs/#writefileoptions)

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

#### 示例

标题为“Example”的章节

    import { writeTextFile, BaseDirectory } from '@tauri-apps/plugin-fs';

    await writeTextFile('file.txt', "Hello world", { baseDir: BaseDirectory.AppLocalData });

#### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/fs/guest-js/index.ts#L1136>