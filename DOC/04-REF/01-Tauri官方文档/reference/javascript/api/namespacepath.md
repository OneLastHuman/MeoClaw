# path

_Source: https://v2.tauri.org.cn/reference/javascript/api/namespacepath/_

path 模块提供了用于处理文件和目录路径的实用工具。

当 `tauri.conf.json` 中的 [`app.withGlobalTauri`](https://v2.tauri.org.cn/reference/config/#withglobaltauri) 设置为 `true` 时，也可以通过 `window.__TAURI__.path` 访问此包。

建议仅允许使用您用到的 API，以实现最优的 bundle 大小和安全性。

## 枚举

标题为“枚举”的章节

### BaseDirectory (基础目录)

名为“BaseDirectory”的章节

#### 始于

名为“起始版本”的部分

2.0.0

#### 枚举成员

标题为“枚举成员”的章节

##### AppCache (应用缓存目录)

名为“AppCache”的章节

    AppCache: 16;

###### 查看

标题为“参见”的章节

[appCacheDir](/reference/javascript/api/namespacepath/#appcachedir) 了解更多信息。

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L83>

##### AppConfig

名为“AppConfig”的章节

    AppConfig: 13;

###### 查看

标题为“参见”的章节

[appConfigDir](/reference/javascript/api/namespacepath/#appconfigdir) 了解更多信息。

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L71>

##### AppData (应用数据目录)

名为“AppData”的章节

    AppData: 14;

###### 查看

标题为“参见”的章节

[appDataDir](/reference/javascript/api/namespacepath/#appdatadir) 了解更多信息。

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L75>

##### AppLocalData (应用本地数据目录)

名为“AppLocalData”的章节

    AppLocalData: 15;

###### 查看

标题为“参见”的章节

[appLocalDataDir](/reference/javascript/api/namespacepath/#applocaldatadir) 了解更多信息。

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L79>

##### AppLog (应用日志目录)

名为“AppLog”的章节

    AppLog: 17;

###### 查看

标题为“参见”的章节

[appLogDir](/reference/javascript/api/namespacepath/#applogdir) 了解更多信息。

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L87>

##### Audio (音频目录)

名为“Audio”的章节

    Audio: 1;

###### 查看

标题为“参见”的章节

[audioDir](/reference/javascript/api/namespacepath/#audiodir) 了解更多信息。

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L23>

##### Cache (缓存目录)

名为“Cache”的章节

    Cache: 2;

###### 查看

标题为“参见”的章节

[cacheDir](/reference/javascript/api/namespacepath/#cachedir) 了解更多信息。

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L27>

##### Config

标题为“配置”的章节

    Config: 3;

###### 查看

标题为“参见”的章节

[configDir](/reference/javascript/api/namespacepath/#configdir) 了解更多信息。

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L31>

##### Data (数据目录)

名为“Data”的章节

    Data: 4;

###### 查看

标题为“参见”的章节

[dataDir](/reference/javascript/api/namespacepath/#datadir) 了解更多信息。

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L35>

##### Desktop

名为“Desktop”的章节

    Desktop: 18;

###### 查看

标题为“参见”的章节

[desktopDir](/reference/javascript/api/namespacepath/#desktopdir) 了解更多信息。

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L91>

##### Document (文档目录)

名为“Document”的章节

    Document: 6;

###### 查看

标题为“参见”的章节

[documentDir](/reference/javascript/api/namespacepath/#documentdir) 了解更多信息。

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L43>

##### Download (下载目录)

名为“Download”的章节

    Download: 7;

###### 查看

标题为“参见”的章节

[downloadDir](/reference/javascript/api/namespacepath/#downloaddir) 了解更多信息。

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L47>

##### Executable (可执行文件目录)

名为“Executable”的章节

    Executable: 19;

###### 查看

标题为“参见”的章节

[executableDir](/reference/javascript/api/namespacepath/#executabledir) 了解更多信息。

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L95>

##### Font (字体目录)

名为“Font”的章节

    Font: 20;

###### 查看

标题为“参见”的章节

[fontDir](/reference/javascript/api/namespacepath/#fontdir) 了解更多信息。

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L99>

##### Home

名为“Home”的章节

    Home: 21;

###### 查看

标题为“参见”的章节

[homeDir](/reference/javascript/api/namespacepath/#homedir) 了解更多信息。

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L103>

##### LocalData (本地数据目录)

名为“LocalData”的章节

    LocalData: 5;

###### 查看

标题为“参见”的章节

[localDataDir](/reference/javascript/api/namespacepath/#localdatadir) 了解更多信息。

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L39>

##### Picture (图片目录)

名为“Picture”的章节

    Picture: 8;

###### 查看

标题为“参见”的章节

[pictureDir](/reference/javascript/api/namespacepath/#picturedir) 了解更多信息。

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L51>

##### Public

标题为“公开”的部分

    Public: 9;

###### 查看

标题为“参见”的章节

[publicDir](/reference/javascript/api/namespacepath/#publicdir) 了解更多信息。

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L55>

##### Resource

名为“Resource”的部分

    Resource: 11;

###### 查看

标题为“参见”的章节

[resourceDir](/reference/javascript/api/namespacepath/#resourcedir) 了解更多信息。

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L63>

##### Runtime (运行时目录)

名为“Runtime”的章节

    Runtime: 22;

###### 查看

标题为“参见”的章节

[runtimeDir](/reference/javascript/api/namespacepath/#runtimedir) 了解更多信息。

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L107>

##### Temp (临时目录)

名为“Temp”的章节

    Temp: 12;

###### 查看

标题为“参见”的章节

[tempDir](/reference/javascript/api/namespacepath/#tempdir) 了解更多信息。

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L67>

##### Template (模板目录)

名为“Template”的章节

    Template: 23;

###### 查看

标题为“参见”的章节

[templateDir](/reference/javascript/api/namespacepath/#templatedir) 了解更多信息。

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L111>

##### Video (视频目录)

名为“Video”的章节

    Video: 10;

###### 查看

标题为“参见”的章节

[videoDir](/reference/javascript/api/namespacepath/#videodir) 了解更多信息。

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L59>

## 函数

名为“函数”的部分

### appCacheDir()

名为“appCacheDir()”的章节

    function appCacheDir(): Promise<string>

返回应用缓存文件的建议目录路径。解析为 `${cacheDir}/${bundleIdentifier}`，其中 `bundleIdentifier` 是在 `tauri.conf.json` 中配置的 [`identifier`](https://v2.tauri.org.cn/reference/config/#identifier) 值。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`string`>

#### 示例

标题为“Example”的章节

    import { appCacheDir } from '@tauri-apps/api/path';

    const appCacheDirPath = await appCacheDir();

#### 始于

名为“起始版本”的部分

1.2.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L176>

* * *

### appConfigDir()

名为“appConfigDir()”的章节

    function appConfigDir(): Promise<string>

返回应用配置文件建议目录的路径。解析为 `${configDir}/${bundleIdentifier}`，其中 `bundleIdentifier` 是在 `tauri.conf.json` 中配置的 [`identifier`](https://v2.tauri.org.cn/reference/config/#identifier) 值。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`string`>

#### 示例

标题为“Example”的章节

    import { appConfigDir } from '@tauri-apps/api/path';

    const appConfigDirPath = await appConfigDir();

#### 始于

名为“起始版本”的部分

1.2.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L125>

* * *

### appDataDir()

名为“appDataDir()”的章节

    function appDataDir(): Promise<string>

返回应用数据文件建议目录的路径。解析为 `${dataDir}/${bundleIdentifier}`，其中 `bundleIdentifier` 是在 `tauri.conf.json` 中配置的 [`identifier`](https://v2.tauri.org.cn/reference/config/#identifier) 值。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`string`>

#### 示例

标题为“Example”的章节

    import { appDataDir } from '@tauri-apps/api/path';

    const appDataDirPath = await appDataDir();

#### 始于

名为“起始版本”的部分

1.2.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L142>

* * *

### appLocalDataDir()

名为“appLocalDataDir()”的章节

    function appLocalDataDir(): Promise<string>

返回应用本地数据文件建议目录的路径。解析为 `${localDataDir}/${bundleIdentifier}`，其中 `bundleIdentifier` 是在 `tauri.conf.json` 中配置的 [`identifier`](https://v2.tauri.org.cn/reference/config/#identifier) 值。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`string`>

#### 示例

标题为“Example”的章节

    import { appLocalDataDir } from '@tauri-apps/api/path';

    const appLocalDataDirPath = await appLocalDataDir();

#### 始于

名为“起始版本”的部分

1.2.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L159>

* * *

### appLogDir()

名为“appLogDir()”的章节

    function appLogDir(): Promise<string>

返回应用日志文件建议目录的路径。

平台特定

  * **Linux:** 解析为 `${configDir}/${bundleIdentifier}/logs`。
  * **macOS:** 解析为 `${homeDir}/Library/Logs/{bundleIdentifier}`
  * **Windows:** 解析为 `${configDir}/${bundleIdentifier}/logs`。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`string`>

#### 示例

标题为“Example”的章节

    import { appLogDir } from '@tauri-apps/api/path';

    const appLogDirPath = await appLogDir();

#### 始于

名为“起始版本”的部分

1.2.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L604>

* * *

### audioDir()

名为“audioDir()”的章节

    function audioDir(): Promise<string>

返回用户音频目录的路径。

平台特定

  * **Linux:** 解析为 [`xdg-user-dirs`](https://www.freedesktop.org/wiki/Software/xdg-user-dirs/) 的 `XDG_MUSIC_DIR`。
  * **macOS:** 解析为 `$HOME/Music`。
  * **Windows:** 解析为 `{FOLDERID_Music}`。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`string`>

#### 示例

标题为“Example”的章节

    import { audioDir } from '@tauri-apps/api/path';

    const audioDirPath = await audioDir();

#### 始于

名为“起始版本”的部分

1.0.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L198>

* * *

### basename()

名为“basename()”的章节

    function basename(path, ext?): Promise<string>

返回 `path` 的最后一部分。尾随的目录分隔符将被忽略。

#### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`path`| `string`| -
`ext`?| `string`| 从返回路径中移除的可选文件扩展名。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`string`>

#### 示例

标题为“Example”的章节

    import { basename } from '@tauri-apps/api/path';

    const base = await basename('path/to/app.conf');

    assert(base === 'app.conf');

#### 始于

名为“起始版本”的部分

1.0.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L734>

* * *

### cacheDir()

名为“cacheDir()”的章节

    function cacheDir(): Promise<string>

返回用户缓存目录的路径。

平台特定

  * **Linux:** 解析为 `$XDG_CACHE_HOME` 或 `$HOME/.cache`。
  * **macOS:** 解析为 `$HOME/Library/Caches`。
  * **Windows:** 解析为 `{FOLDERID_LocalAppData}`。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`string`>

#### 示例

标题为“Example”的章节

    import { cacheDir } from '@tauri-apps/api/path';

    const cacheDirPath = await cacheDir();

#### 始于

名为“起始版本”的部分

1.0.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L220>

* * *

### configDir()

名为“configDir()”的章节

    function configDir(): Promise<string>

返回用户配置目录的路径。

平台特定

  * **Linux:** 解析为 `$XDG_CONFIG_HOME` 或 `$HOME/.config`。
  * **macOS:** 解析为 `$HOME/Library/Application Support`。
  * **Windows:** 解析为 `{FOLDERID_RoamingAppData}`。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`string`>

#### 示例

标题为“Example”的章节

    import { configDir } from '@tauri-apps/api/path';

    const configDirPath = await configDir();

#### 始于

名为“起始版本”的部分

1.0.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L242>

* * *

### dataDir()

名为“dataDir()”的章节

    function dataDir(): Promise<string>

返回用户数据目录的路径。

平台特定

  * **Linux:** 解析为 `$XDG_DATA_HOME` 或 `$HOME/.local/share`。
  * **macOS:** 解析为 `$HOME/Library/Application Support`。
  * **Windows:** 解析为 `{FOLDERID_RoamingAppData}`。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`string`>

#### 示例

标题为“Example”的章节

    import { dataDir } from '@tauri-apps/api/path';

    const dataDirPath = await dataDir();

#### 始于

名为“起始版本”的部分

1.0.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L264>

* * *

### delimiter()

名为“delimiter()”的章节

    function delimiter(): string

返回平台特定的路径段分隔符

  * Windows 上为 `;`
  * POSIX 上为 `:`

#### 返回

名为“返回值”的部分

`string`

#### 始于

名为“起始版本”的部分

2.0.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L644>

* * *

### desktopDir()

名为“desktopDir()”的章节

    function desktopDir(): Promise<string>

返回用户桌面目录的路径。

平台特定

  * **Linux:** 解析为 [`xdg-user-dirs`](https://www.freedesktop.org/wiki/Software/xdg-user-dirs/) 的 `XDG_DESKTOP_DIR`。
  * **macOS:** 解析为 `$HOME/Desktop`。
  * **Windows:** 解析为 `{FOLDERID_Desktop}`。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`string`>

#### 示例

标题为“Example”的章节

    import { desktopDir } from '@tauri-apps/api/path';

    const desktopPath = await desktopDir();

#### 始于

名为“起始版本”的部分

1.0.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L286>

* * *

### dirname()

名为“dirname()”的章节

    function dirname(path): Promise<string>

返回给定 `path` 的父目录。尾随的目录分隔符将被忽略。

#### 参数

名为“参数”的部分

参数| 类型
---|---
`path`| `string`

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`string`>

#### 示例

标题为“Example”的章节

    import { dirname } from '@tauri-apps/api/path';

    const dir = await dirname('/path/to/somedir/');

    assert(dir === '/path/to');

#### 始于

名为“起始版本”的部分

1.0.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L703>

* * *

### documentDir()

名为“documentDir()”的章节

    function documentDir(): Promise<string>

返回用户文档目录的路径。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`string`>

#### 示例

标题为“Example”的章节

    import { documentDir } from '@tauri-apps/api/path';

    const documentDirPath = await documentDir();

平台特定

  * **Linux:** 解析为 [`xdg-user-dirs`](https://www.freedesktop.org/wiki/Software/xdg-user-dirs/) 的 `XDG_DOCUMENTS_DIR`。
  * **macOS:** 解析为 `$HOME/Documents`。
  * **Windows:** 解析为 `{FOLDERID_Documents}`。

#### 始于

名为“起始版本”的部分

1.0.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L308>

* * *

### downloadDir()

名为“downloadDir()”的章节

    function downloadDir(): Promise<string>

返回用户下载目录的路径。

平台特定

  * **Linux** : 解析为 [`xdg-user-dirs`](https://www.freedesktop.org/wiki/Software/xdg-user-dirs/) 的 `XDG_DOWNLOAD_DIR`。
  * **macOS** : 解析为 `$HOME/Downloads`。
  * **Windows** : 解析为 `{FOLDERID_Downloads}`。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`string`>

#### 示例

标题为“Example”的章节

    import { downloadDir } from '@tauri-apps/api/path';

    const downloadDirPath = await downloadDir();

#### 始于

名为“起始版本”的部分

1.0.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L330>

* * *

### executableDir()

名为“executableDir()”的章节

    function executableDir(): Promise<string>

返回用户可执行文件目录的路径。

平台特定

  * **Linux:** 解析为 `$XDG_BIN_HOME/../bin` 或 `$XDG_DATA_HOME/../bin` 或 `$HOME/.local/bin`。
  * **macOS:** 不支持。
  * **Windows:** 不支持。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`string`>

#### 示例

标题为“Example”的章节

    import { executableDir } from '@tauri-apps/api/path';

    const executableDirPath = await executableDir();

#### 始于

名为“起始版本”的部分

1.0.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L352>

* * *

### extname()

名为“extname()”的章节

    function extname(path): Promise<string>

返回 `path` 的扩展名。

#### 参数

名为“参数”的部分

参数| 类型
---|---
`path`| `string`

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`string`>

#### 示例

标题为“Example”的章节

    import { extname } from '@tauri-apps/api/path';

    const ext = await extname('/path/to/file.html');

    assert(ext === 'html');

#### 始于

名为“起始版本”的部分

1.0.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L718>

* * *

### fontDir()

名为“fontDir()”的章节

    function fontDir(): Promise<string>

返回用户字体目录的路径。

平台特定

  * **Linux:** 解析为 `$XDG_DATA_HOME/fonts` 或 `$HOME/.local/share/fonts`。
  * **macOS:** 解析为 `$HOME/Library/Fonts`。
  * **Windows:** 不支持。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`string`>

#### 示例

标题为“Example”的章节

    import { fontDir } from '@tauri-apps/api/path';

    const fontDirPath = await fontDir();

#### 始于

名为“起始版本”的部分

1.0.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L374>

* * *

### homeDir()

名为“homeDir()”的章节

    function homeDir(): Promise<string>

返回用户主目录的路径。

平台特定

  * **Linux:** 解析为 `$HOME`。
  * **macOS:** 解析为 `$HOME`。
  * **Windows:** 解析为 `{FOLDERID_Profile}`。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`string`>

#### 示例

标题为“Example”的章节

    import { homeDir } from '@tauri-apps/api/path';

    const homeDirPath = await homeDir();

#### 始于

名为“起始版本”的部分

1.0.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L396>

* * *

### isAbsolute()

名为“isAbsolute()”的章节

    function isAbsolute(path): Promise<boolean>

返回路径是否为绝对路径。

#### 参数

名为“参数”的部分

参数| 类型
---|---
`path`| `string`

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`boolean`>

#### 示例

标题为“Example”的章节

    import { isAbsolute } from '@tauri-apps/api/path';

    assert(await isAbsolute('/home/tauri'));

#### 始于

名为“起始版本”的部分

1.0.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L748>

* * *

### join()

名为“join()”的章节

    function join(...paths): Promise<string>

使用平台特定的分隔符连接所有给定的 `path` 段，然后规范化生成的路径。

#### 参数

名为“参数”的部分

参数| 类型
---|---
…`paths`| `string`[]

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`string`>

#### 示例

标题为“Example”的章节

    import { join, appDataDir } from '@tauri-apps/api/path';

    const appDataDirPath = await appDataDir();

    const path = await join(appDataDirPath, 'users', 'tauri', 'avatar.png');

#### 始于

名为“起始版本”的部分

1.0.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L688>

* * *

### localDataDir()

名为“localDataDir()”的章节

    function localDataDir(): Promise<string>

返回用户本地数据目录的路径。

平台特定

  * **Linux:** 解析为 `$XDG_DATA_HOME` 或 `$HOME/.local/share`。
  * **macOS:** 解析为 `$HOME/Library/Application Support`。
  * **Windows:** 解析为 `{FOLDERID_LocalAppData}`。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`string`>

#### 示例

标题为“Example”的章节

    import { localDataDir } from '@tauri-apps/api/path';

    const localDataDirPath = await localDataDir();

#### 始于

名为“起始版本”的部分

1.0.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L418>

* * *

### normalize()

名为“normalize()”的章节

    function normalize(path): Promise<string>

规范化给定的 `path`，解析 `'..'` 和 `'.'` 段，并解析符号链接。

#### 参数

名为“参数”的部分

参数| 类型
---|---
`path`| `string`

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`string`>

#### 示例

标题为“Example”的章节

    import { normalize, appDataDir } from '@tauri-apps/api/path';

    const appDataDirPath = await appDataDir();

    const path = await normalize(`${appDataDirPath}/../users/tauri/avatar.png`);

#### 始于

名为“起始版本”的部分

1.0.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L673>

* * *

### pictureDir()

名为“pictureDir()”的章节

    function pictureDir(): Promise<string>

返回用户图片目录的路径。

平台特定

  * **Linux:** 解析为 [`xdg-user-dirs`](https://www.freedesktop.org/wiki/Software/xdg-user-dirs/) 的 `XDG_PICTURES_DIR`。
  * **macOS:** 解析为 `$HOME/Pictures`。
  * **Windows:** 解析为 `{FOLDERID_Pictures}`。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`string`>

#### 示例

标题为“Example”的章节

    import { pictureDir } from '@tauri-apps/api/path';

    const pictureDirPath = await pictureDir();

#### 始于

名为“起始版本”的部分

1.0.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L440>

* * *

### publicDir()

名为“publicDir()”的章节

    function publicDir(): Promise<string>

返回用户公共目录的路径。

平台特定

  * **Linux:** 解析为 [`xdg-user-dirs`](https://www.freedesktop.org/wiki/Software/xdg-user-dirs/) 的 `XDG_PUBLICSHARE_DIR`。
  * **macOS:** 解析为 `$HOME/Public`。
  * **Windows:** 解析为 `{FOLDERID_Public}`。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`string`>

#### 示例

标题为“Example”的章节

    import { publicDir } from '@tauri-apps/api/path';

    const publicDirPath = await publicDir();

#### 始于

名为“起始版本”的部分

1.0.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L462>

* * *

### resolve()

名为“resolve()”的章节

    function resolve(...paths): Promise<string>

将一系列 `paths` 或路径段解析为绝对路径。

#### 参数

名为“参数”的部分

参数| 类型
---|---
…`paths`| `string`[]

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`string`>

#### 示例

标题为“Example”的章节

    import { resolve, appDataDir } from '@tauri-apps/api/path';

    const appDataDirPath = await appDataDir();

    const path = await resolve(appDataDirPath, '..', 'users', 'tauri', 'avatar.png');

#### 始于

名为“起始版本”的部分

1.0.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L658>

* * *

### resolveResource()

名为“resolveResource()”的章节

    function resolveResource(resourcePath): Promise<string>

解析资源文件的路径。

#### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`resourcePath`| `string`| 资源的路径。必须遵循 `tauri.conf.json > bundle > resources` 中定义的相同语法，即保留子文件夹和父目录组件（`../`）。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`string`>

资源的完整路径。

#### 示例

标题为“Example”的章节

    import { resolveResource } from '@tauri-apps/api/path';

    const resourcePath = await resolveResource('script.sh');

#### 始于

名为“起始版本”的部分

1.0.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L515>

* * *

### resourceDir()

名为“resourceDir()”的章节

    function resourceDir(): Promise<string>

返回应用程序资源目录的路径。要解析资源路径，请参阅 [`resolveResource`](/reference/javascript/api/namespacepath/#resolveresource)。

## 平台特定

名为“平台特定”的章节

尽管我们提供了该函数解析到的确切路径，但这并不是合约，将来可能会发生变化。

  * **Windows:** 解析为包含主可执行文件的目录。
  * **Linux:** 在 AppImage 中运行时，`APPDIR` 变量将被设置为应用的挂载位置，资源目录将为 `${APPDIR}/usr/lib/${exe_name}`。如果不是在 AppImage 中运行，路径则为 `/usr/lib/${exe_name}`。当从 `src-tauri/target/(debug|release)/` 运行应用时，路径为 `${exe_dir}/../lib/${exe_name}`。
  * **macOS:** 解析为 `${exe_dir}/../Resources`（位于 .app 内）。
  * **iOS:** 解析为 `${exe_dir}/assets`。
  * **Android:** 目前资源作为资产存储在 APK 中，因此它不是一个普通的文件系统路径，这里我们返回一个特殊的 URI 前缀 `asset:///`，该前缀可以与 [文件系统插件](https://tauri.org.cn/plugin/file-system/) 配合使用。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`string`>

#### 示例

标题为“Example”的章节

    import { resourceDir } from '@tauri-apps/api/path';

    const resourceDirPath = await resourceDir();

#### 始于

名为“起始版本”的部分

1.0.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L495>

* * *

### runtimeDir()

名为“runtimeDir()”的章节

    function runtimeDir(): Promise<string>

返回用户运行时目录的路径。

平台特定

  * **Linux:** 解析为 `$XDG_RUNTIME_DIR`。
  * **macOS:** 不支持。
  * **Windows:** 不支持。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`string`>

#### 示例

标题为“Example”的章节

    import { runtimeDir } from '@tauri-apps/api/path';

    const runtimeDirPath = await runtimeDir();

#### 始于

名为“起始版本”的部分

1.0.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L538>

* * *

### sep()

名为“sep()”的章节

    function sep(): string

返回平台特定的路径段分隔符

  * Windows 上为 `\\`
  * POSIX 上为 `/`

#### 返回

名为“返回值”的部分

`string`

#### 始于

名为“起始版本”的部分

2.0.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L633>

* * *

### tempDir()

名为“tempDir()”的章节

    function tempDir(): Promise<string>

返回一个临时目录。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`string`>

#### 示例

标题为“Example”的章节

    import { tempDir } from '@tauri-apps/api/path';

    const temp = await tempDir();

#### 始于

名为“起始版本”的部分

2.0.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L620>

* * *

### templateDir()

名为“templateDir()”的章节

    function templateDir(): Promise<string>

返回用户模板目录的路径。

平台特定

  * **Linux:** 解析为 [`xdg-user-dirs`](https://www.freedesktop.org/wiki/Software/xdg-user-dirs/) 的 `XDG_TEMPLATES_DIR`。
  * **macOS:** 不支持。
  * **Windows:** 解析为 `{FOLDERID_Templates}`。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`string`>

#### 示例

标题为“Example”的章节

    import { templateDir } from '@tauri-apps/api/path';

    const templateDirPath = await templateDir();

#### 始于

名为“起始版本”的部分

1.0.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L560>

* * *

### videoDir()

名为“videoDir()”的章节

    function videoDir(): Promise<string>

返回用户视频目录的路径。

平台特定

  * **Linux:** 解析为 [`xdg-user-dirs`](https://www.freedesktop.org/wiki/Software/xdg-user-dirs/) 的 `XDG_VIDEOS_DIR`。
  * **macOS:** 解析为 `$HOME/Movies`。
  * **Windows:** 解析为 `{FOLDERID_Videos}`。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`string`>

#### 示例

标题为“Example”的章节

    import { videoDir } from '@tauri-apps/api/path';

    const videoDirPath = await videoDir();

#### 始于

名为“起始版本”的部分

1.0.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/path.ts#L582>