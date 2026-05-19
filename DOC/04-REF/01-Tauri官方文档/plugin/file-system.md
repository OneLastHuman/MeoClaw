# 文件系统

_Source: https://v2.tauri.org.cn/plugin/file-system/_

[ GitHub](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/fs) [ npm ](https://npmjs.net.cn/package/@tauri-apps/plugin-fs) [ crates.io ](https://crates.io/crates/tauri-plugin-fs)

API 参考 [ ](/reference/javascript/fs/) [ ](https://docs.rs/tauri-plugin-fs)

访问文件系统。

在 Rust 端使用 std::fs 或 tokio::fs

如果您想通过 Rust 操作文件/目录，请使用传统的 Rust 库（std::fs、tokio::fs 等）。

## 支持的平台

标题为“支持的平台”的章节

_此插件需要 Rust 版本至少为 **1.77.2**_

平台 | 级别 | 备注
---|---|---
windows |  |  通过 MSI 或 NSIS 在 `perMachine` 和 `both` 模式下安装的应用程序，在 `$RESOURCES` 文件夹中进行写操作需要管理员权限
linux |  |  没有对 `$RESOURCES` 文件夹的写入权限
macos |  |  没有对 `$RESOURCES` 文件夹的写入权限
android |  |  默认情况下访问仅限于应用程序文件夹
ios |  |  默认情况下访问仅限于应用程序文件夹

## 设置

标题为“设置”的章节

安装 fs 插件以开始使用。

  * 自动
  * 手动

使用你的项目包管理器添加依赖项

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri add fs

    yarn run tauri add fs

    pnpm tauri add fs

    deno task tauri add fs

    bun tauri add fs

    cargo tauri add fs

  1. 在 `src-tauri` 文件夹中运行以下命令，将插件添加到项目的 `Cargo.toml` 依赖项中

         cargo add tauri-plugin-fs

  2. 修改 `lib.rs` 以初始化插件

src-tauri/src/lib.rs

         #[cfg_attr(mobile, tauri::mobile_entry_point)]

         pub fn run() {

           tauri::Builder::default()

             .plugin(tauri_plugin_fs::init())

             .run(tauri::generate_context!())

             .expect("error while running tauri application");

         }

  3. 使用您首选的 JavaScript 包管理器安装 JavaScript 访客绑定

     * npm
     * yarn
     * pnpm
     * deno
     * bun

    npm install @tauri-apps/plugin-fs

    yarn add @tauri-apps/plugin-fs

    pnpm add @tauri-apps/plugin-fs

    deno add npm:@tauri-apps/plugin-fs

    bun add @tauri-apps/plugin-fs

## 配置

名为“配置”的章节

### Android

标题为“Android”的章节

使用音频 (audio)、缓存 (cache)、文档 (documents)、下载 (downloads)、图片 (picture)、公共 (public) 或视频 (video) 目录时，您的应用程序必须具有外部存储访问权限。

将以下权限包含在 `gen/android/app/src/main/AndroidManifest.xml` 文件中的 `manifest` 标签内

    <uses-permission android:name="android.permission.READ_EXTERNAL_STORAGE"/>

    <uses-permission android:name="android.permission.WRITE_EXTERNAL_STORAGE" />

### iOS

名为“iOS”的章节

Apple 要求应用程序开发者指定 API 使用的批准理由，以增强用户隐私。

您必须在 `src-tauri/gen/apple` 文件夹中创建一个 `PrivacyInfo.xcprivacy` 文件，并包含必需的 [NSPrivacyAccessedAPICategoryFileTimestamp](https://developer.apple.com/documentation/bundleresources/privacy_manifest_files/describing_use_of_required_reason_api#4278393) 键和推荐的 [C617.1](https://developer.apple.com/documentation/bundleresources/privacy_manifest_files/describing_use_of_required_reason_api#4278393) 使用理由。

    <?xml version="1.0" encoding="UTF-8"?>

    <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">

    <plist version="1.0">

      <dict>

        <key>NSPrivacyAccessedAPITypes</key>

        <array>

          <dict>

            <key>NSPrivacyAccessedAPIType</key>

            <string>NSPrivacyAccessedAPICategoryFileTimestamp</string>

            <key>NSPrivacyAccessedAPITypeReasons</key>

            <array>

              <string>C617.1</string>

            </array>

          </dict>

        </array>

      </dict>

    </plist>

## 用法

名为“用法”的章节

fs 插件在 JavaScript 和 Rust 中均可用。

不同的 API

虽然此插件在前端提供了文件操作 API，但在后端它仅提供更改某些资源（文件、目录等）权限的方法。

在 Rust 端，您可以使用传统的文件操作库：[std::fs](https://doc.rust-lang.net.cn/std/fs/struct.File.html)、[tokio::fs](https://docs.rs/tokio/latest/tokio/fs/index.html) 或其他库。

  * JavaScript
  * Rust

    import { exists, BaseDirectory } from '@tauri-apps/plugin-fs';

    // when using `"withGlobalTauri": true`, you may use

    // const { exists, BaseDirectory } = window.__TAURI__.fs;

    // Check if the `$APPDATA/avatar.png` file exists

    await exists('avatar.png', { baseDir: BaseDirectory.AppData });

src-tauri/src/lib.rs

    use tauri_plugin_fs::FsExt;

    #[cfg_attr(mobile, tauri::mobile_entry_point)]

    pub fn run() {

      tauri::Builder::default()

          .plugin(tauri_plugin_fs::init())

          .setup(|app| {

              // allowed the given directory

              let scope = app.fs_scope();

              scope.allow_directory("/path/to/directory", false);

              dbg!(scope.allowed());

              Ok(())

           })

           .run(tauri::generate_context!())

           .expect("error while running tauri application");

    }

## 安全

名为“安全”的章节

该模块防止路径遍历，不允许使用父目录访问器（例如，不允许使用 “/usr/path/to/../file” 或 ”../path/to/file” 路径）。使用此 API 访问的路径必须相对于某个[基础目录](/reference/javascript/api/namespacepath/#basedirectory)，或者是使用[路径 API](/reference/javascript/api/namespacepath/) 创建的。

更多信息请参见 [@tauri-apps/plugin-fs - 安全性](/reference/javascript/fs/#security)。

## 路径

标题为“路径”的章节

文件系统插件提供了两种操作路径的方法：[基础目录](/reference/javascript/api/namespacepath/#basedirectory)和[路径 API](/reference/javascript/api/namespacepath/)。

  * 基础目录

每个 API 都有一个选项参数，允许您定义一个作为操作工作目录的 [baseDir](/reference/javascript/api/namespacepath/#basedirectory)。

        import { readFile } from '@tauri-apps/plugin-fs';

        const contents = await readFile('avatars/tauri.png', {

          baseDir: BaseDirectory.Home,

        });

在上面的示例中，由于我们使用了**主目录 (Home)** 基础目录，因此读取的是 ~/avatars/tauri.png 文件。

  * 路径 API

或者，您可以使用路径 API 执行路径操作。

        import { readFile } from '@tauri-apps/plugin-fs';

        import * as path from '@tauri-apps/api/path';

        const home = await path.homeDir();

        const contents = await readFile(await path.join(home, 'avatars/tauri.png'));

## 文件

标题为“文件”的章节

### 创建

标题为“创建”的章节

创建文件并返回其句柄。如果文件已存在，它将被截断。

    import { create, BaseDirectory } from '@tauri-apps/plugin-fs';

    const file = await create('foo/bar.txt', { baseDir: BaseDirectory.AppData });

    await file.write(new TextEncoder().encode('Hello world'));

    await file.close();

注意

文件操作完成后，请务必调用 `file.close()`。

### 写入

标题为“写入”的章节

该插件提供单独的 API 来写入文本文件和二进制文件，以提高性能。

  * 文本文件

        import { writeTextFile, BaseDirectory } from '@tauri-apps/plugin-fs';

        const contents = JSON.stringify({ notifications: true });

        await writeTextFile('config.json', contents, {

          baseDir: BaseDirectory.AppConfig,

        });

  * 二进制文件

        import { writeFile, BaseDirectory } from '@tauri-apps/plugin-fs';

        const contents = new Uint8Array(); // fill a byte array

        await writeFile('config', contents, {

          baseDir: BaseDirectory.AppConfig,

        });

### 打开

标题为“打开”的章节

打开文件并返回其句柄。通过此 API，您可以更好地控制文件的打开方式（只读模式、只写模式、追加而非覆盖、仅在不存在时创建等）。

注意

文件操作完成后，请务必调用 `file.close()`。

  * 只读

这是默认模式。

        import { open, BaseDirectory } from '@tauri-apps/plugin-fs';

        const file = await open('foo/bar.txt', {

          read: true,

          baseDir: BaseDirectory.AppData,

        });

        const stat = await file.stat();

        const buf = new Uint8Array(stat.size);

        await file.read(buf);

        const textContents = new TextDecoder().decode(buf);

        await file.close();

  * 只写

        import { open, BaseDirectory } from '@tauri-apps/plugin-fs';

        const file = await open('foo/bar.txt', {

          write: true,

          baseDir: BaseDirectory.AppData,

        });

        await file.write(new TextEncoder().encode('Hello world'));

        await file.close();

默认情况下，任何 `file.write()` 调用都会截断文件。请参阅以下示例了解如何追加到现有内容。

  * 追加

        import { open, BaseDirectory } from '@tauri-apps/plugin-fs';

        const file = await open('foo/bar.txt', {

          append: true,

          baseDir: BaseDirectory.AppData,

        });

        await file.write(new TextEncoder().encode('world'));

        await file.close();

请注意，`{ append: true }` 与 `{ write: true, append: true }` 的效果相同。

  * 截断

当设置了 `truncate` 选项且文件已存在时，它将被截断为 0 长度。

        import { open, BaseDirectory } from '@tauri-apps/plugin-fs';

        const file = await open('foo/bar.txt', {

          write: true,

          truncate: true,

          baseDir: BaseDirectory.AppData,

        });

        await file.write(new TextEncoder().encode('world'));

        await file.close();

此选项需要 `write` 为 `true`。

如果您想使用多个 `file.write()` 调用重写现有文件，可以将其与 `append` 选项结合使用。

  * 创建

默认情况下，`open` API 仅打开现有文件。要在文件不存在时创建它，并在已存在时打开它，请将 `create` 设置为 `true`

        import { open, BaseDirectory } from '@tauri-apps/plugin-fs';

        const file = await open('foo/bar.txt', {

          write: true,

          create: true,

          baseDir: BaseDirectory.AppData,

        });

        await file.write(new TextEncoder().encode('world'));

        await file.close();

为了创建文件，还必须将 `write` 或 `append` 设置为 `true`。

如果要在文件已存在时失败，请参阅 `createNew`。

  * createNew

`createNew` 的工作方式与 `create` 类似，但如果文件已存在，则会失败。

        import { open, BaseDirectory } from '@tauri-apps/plugin-fs';

        const file = await open('foo/bar.txt', {

          write: true,

          createNew: true,

          baseDir: BaseDirectory.AppData,

        });

        await file.write(new TextEncoder().encode('world'));

        await file.close();

为了创建文件，还必须将 `write` 设置为 `true`。

### 读取

标题为“读取”的章节

该插件提供单独的 API 来读取文本文件和二进制文件，以提高性能。

  * 文本文件

        import { readTextFile, BaseDirectory } from '@tauri-apps/plugin-fs';

        const configToml = await readTextFile('config.toml', {

          baseDir: BaseDirectory.AppConfig,

        });

如果文件较大，您可以使用 `readTextFileLines` API 流式读取其行

        import { readTextFileLines, BaseDirectory } from '@tauri-apps/plugin-fs';

        const lines = await readTextFileLines('app.logs', {

          baseDir: BaseDirectory.AppLog,

        });

        for await (const line of lines) {

          console.log(line);

        }

  * 二进制文件

        import { readFile, BaseDirectory } from '@tauri-apps/plugin-fs';

        const icon = await readFile('icon.png', {

          baseDir: BaseDirectory.Resources,

        });

### 移除

标题为“移除”的章节

调用 `remove()` 删除文件。如果文件不存在，则会返回错误。

    import { remove, BaseDirectory } from '@tauri-apps/plugin-fs';

    await remove('user.db', { baseDir: BaseDirectory.AppLocalData });

### 复制

标题为“复制”的章节

`copyFile` 函数接收源路径和目标路径。请注意，您必须分别配置每个基础目录。

    import { copyFile, BaseDirectory } from '@tauri-apps/plugin-fs';

    await copyFile('user.db', 'user.db.bk', {

      fromPathBaseDir: BaseDirectory.AppLocalData,

      toPathBaseDir: BaseDirectory.Temp,

    });

在上面的示例中，<app-local-data>/user.db 文件被复制到 $TMPDIR/user.db.bk。

### 存在

标题为“存在”的章节

使用 `exists()` 函数检查文件是否存在

    import { exists, BaseDirectory } from '@tauri-apps/plugin-fs';

    const tokenExists = await exists('token', {

      baseDir: BaseDirectory.AppLocalData,

    });

### 元数据

标题为“元数据”的章节

可以通过 `stat` 和 `lstat` 函数检索文件元数据。`stat` 会跟随符号链接（如果其指向的实际文件未被作用域允许，则返回错误），而 `lstat` 不会跟随符号链接，而是返回符号链接本身的信息。

    import { stat, BaseDirectory } from '@tauri-apps/plugin-fs';

    const metadata = await stat('app.db', {

      baseDir: BaseDirectory.AppLocalData,

    });

### 重命名

标题为“重命名”的章节

`rename` 函数接收源路径和目标路径。请注意，您必须分别配置每个基础目录。

    import { rename, BaseDirectory } from '@tauri-apps/plugin-fs';

    await rename('user.db.bk', 'user.db', {

      fromPathBaseDir: BaseDirectory.AppLocalData,

      toPathBaseDir: BaseDirectory.Temp,

    });

在上面的示例中，<app-local-data>/user.db.bk 文件被重命名为 $TMPDIR/user.db。

### 截断

标题为“截断”的章节

截断或扩展指定文件以达到指定的文件长度（默认为 0）。

  * 截断为 0 长度

    import { truncate } from '@tauri-apps/plugin-fs';

    await truncate('my_file.txt', 0, { baseDir: BaseDirectory.AppLocalData });

  * 截断为特定长度

    import {

      truncate,

      readTextFile,

      writeTextFile,

      BaseDirectory,

    } from '@tauri-apps/plugin-fs';

    const filePath = 'file.txt';

    await writeTextFile(filePath, 'Hello World', {

      baseDir: BaseDirectory.AppLocalData,

    });

    await truncate(filePath, 7, {

      baseDir: BaseDirectory.AppLocalData,

    });

    const data = await readTextFile(filePath, {

      baseDir: BaseDirectory.AppLocalData,

    });

    console.log(data); // "Hello W"

## 目录

标题为“目录”的章节

### 创建

标题为“创建”的章节

要创建目录，请调用 `mkdir` 函数

    import { mkdir, BaseDirectory } from '@tauri-apps/plugin-fs';

    await mkdir('images', {

      baseDir: BaseDirectory.AppLocalData,

    });

### 读取

标题为“读取”的章节

`readDir` 函数可递归列出目录条目

    import { readDir, BaseDirectory } from '@tauri-apps/plugin-fs';

    const entries = await readDir('users', { baseDir: BaseDirectory.AppLocalData });

### 移除

标题为“移除”的章节

调用 `remove()` 删除目录。如果目录不存在，则会返回错误。

    import { remove, BaseDirectory } from '@tauri-apps/plugin-fs';

    await remove('images', { baseDir: BaseDirectory.AppLocalData });

如果目录不为空，则必须将 `recursive` 选项设置为 `true`

    import { remove, BaseDirectory } from '@tauri-apps/plugin-fs';

    await remove('images', {

      baseDir: BaseDirectory.AppLocalData,

      recursive: true,

    });

### 存在

标题为“存在”的章节

使用 `exists()` 函数检查目录是否存在

    import { exists, BaseDirectory } from '@tauri-apps/plugin-fs';

    const tokenExists = await exists('images', {

      baseDir: BaseDirectory.AppLocalData,

    });

### 元数据

标题为“元数据”的章节

可以通过 `stat` 和 `lstat` 函数检索目录元数据。`stat` 会跟随符号链接（如果其指向的实际文件未被作用域允许，则返回错误），而 `lstat` 不会跟随符号链接，而是返回符号链接本身的信息。

    import { stat, BaseDirectory } from '@tauri-apps/plugin-fs';

    const metadata = await stat('databases', {

      baseDir: BaseDirectory.AppLocalData,

    });

## 监视变更

标题为“监视变更”的章节

要监视目录或文件的更改，请使用 `watch` 或 `watchImmediate` 函数。

  * watch

`watch` 是去抖动 (debounced) 的，因此它仅在一定延迟后发出事件

        import { watch, BaseDirectory } from '@tauri-apps/plugin-fs';

        await watch(

          'app.log',

          (event) => {

            console.log('app.log event', event);

          },

          {

            baseDir: BaseDirectory.AppLog,

            delayMs: 500,

          }

        );

  * watchImmediate

`watchImmediate` 会立即通知侦听器事件发生

        import { watchImmediate, BaseDirectory } from '@tauri-apps/plugin-fs';

        await watchImmediate(

          'logs',

          (event) => {

            console.log('logs directory event', event);

          },

          {

            baseDir: BaseDirectory.AppLog,

            recursive: true,

          }

        );

默认情况下，对目录的监视操作不是递归的。设置 `recursive` 选项为 `true` 可递归监视所有子目录的更改。

注意

监视函数需要 `watch` 功能标志

src-tauri/Cargo.toml

    [dependencies]

    tauri-plugin-fs = { version = "2.0.0", features = ["watch"] }

## Permissions

名为“权限”的章节

默认情况下，所有潜在危险的插件命令和范围都被阻止，无法访问。您必须修改 `capabilities` 配置中的权限才能启用这些功能。

有关更多信息，请参阅[功能概述](/security/capabilities/)，并参阅[分步指南](/learn/security/using-plugin-permissions/)以使用插件权限。

src-tauri/capabilities/default.json

    {

      "$schema": "../gen/schemas/desktop-schema.json",

      "identifier": "main-capability",

      "description": "Capability for the main window",

      "windows": ["main"],

      "permissions": [

        "fs:default",

        {

          "identifier": "fs:allow-exists",

          "allow": [{ "path": "$APPDATA/*" }]

        }

      ]

    }

## 默认权限

此权限集描述了 `fs` 插件默认启用或禁用的文件系统访问权限。

#### 已授予权限

此默认权限集启用了对特定于应用程序的目录（AppConfig、AppData、AppLocalData、AppCache、AppLog）以及其中创建的所有文件和子目录的读取访问权限。这些目录的位置取决于运行应用程序的操作系统。

通常，这些目录需要由应用程序在运行时手动创建，然后才能访问其中的文件或文件夹。

因此，也允许通过 `mkdir` 命令创建所有这些文件夹。

#### 拒绝的权限

此默认权限集默认防止访问 Tauri 应用程序的关键组件。在 Windows 上，拒绝访问 webview 数据文件夹。

#### 此默认权限集包括以下内容

  * `create-app-specific-dirs`
  * `read-app-specific-dirs-recursive`
  * `deny-default`

## 权限表

标识符 | 描述
---|---
`fs:allow-app-read-recursive` |  允许对完整的应用程序文件夹、文件和子目录进行完全递归读取访问。
`fs:allow-app-write-recursive` |  允许对完整的应用程序文件夹、文件和子目录进行完全递归写入访问。
`fs:allow-app-read` |  允许对应用程序文件夹进行非递归读取访问。
`fs:allow-app-write` |  允许对应用程序文件夹进行非递归写入访问。
`fs:allow-app-meta-recursive` |  允许对应用程序文件夹的元数据进行完全递归读取访问，包括文件列表和统计信息。
`fs:allow-app-meta` |  允许对应用程序文件夹的元数据进行非递归读取访问，包括文件列表和统计信息。
`fs:scope-app-recursive` |  此范围允许递归访问完整的应用程序文件夹，包括子目录和文件。
`fs:scope-app` |  此范围允许访问所有文件并列出应用程序文件夹中顶级目录的内容。
`fs:scope-app-index` |  此范围允许列出应用程序目录中的所有文件和文件夹。
`fs:allow-appcache-read-recursive` |  允许对完整的 `$APPCACHE` 文件夹、文件和子目录进行完全递归读取访问。
`fs:allow-appcache-write-recursive` |  允许对完整的 `$APPCACHE` 文件夹、文件和子目录进行完全递归写入访问。
`fs:allow-appcache-read` |  允许对 `$APPCACHE` 文件夹进行非递归读取访问。
`fs:allow-appcache-write` |  允许对 `$APPCACHE` 文件夹进行非递归写入访问。
`fs:allow-appcache-meta-recursive` |  允许对 `$APPCACHE` 文件夹的元数据进行完全递归读取访问，包括文件列表和统计信息。
`fs:allow-appcache-meta` |  允许对 `$APPCACHE` 文件夹的元数据进行非递归读取访问，包括文件列表和统计信息。
`fs:scope-appcache-recursive` |  此范围允许递归访问完整的 `$APPCACHE` 文件夹，包括子目录和文件。
`fs:scope-appcache` |  此范围允许访问所有文件并列出 `$APPCACHE` 文件夹中顶级目录的内容。
`fs:scope-appcache-index` |  此范围允许列出 `$APPCACHE` 文件夹中的所有文件和文件夹。
`fs:allow-appconfig-read-recursive` |  允许对完整的 `$APPCONFIG` 文件夹、文件和子目录进行完全递归读取访问。
`fs:allow-appconfig-write-recursive` |  允许对完整的 `$APPCONFIG` 文件夹、文件和子目录进行完全递归写入访问。
`fs:allow-appconfig-read` |  允许对 `$APPCONFIG` 文件夹进行非递归读取访问。
`fs:allow-appconfig-write` |  允许对 `$APPCONFIG` 文件夹进行非递归写入访问。
`fs:allow-appconfig-meta-recursive` |  允许对 `$APPCONFIG` 文件夹的元数据进行完全递归读取访问，包括文件列表和统计信息。
`fs:allow-appconfig-meta` |  允许对 `$APPCONFIG` 文件夹的元数据进行非递归读取访问，包括文件列表和统计信息。
`fs:scope-appconfig-recursive` |  此范围允许递归访问完整的 `$APPCONFIG` 文件夹，包括子目录和文件。
`fs:scope-appconfig` |  此范围允许访问所有文件并列出 `$APPCONFIG` 文件夹中顶级目录的内容。
`fs:scope-appconfig-index` |  此范围允许列出 `$APPCONFIG` 文件夹中的所有文件和文件夹。
`fs:allow-appdata-read-recursive` |  允许对完整的 `$APPDATA` 文件夹、文件和子目录进行完全递归读取访问。
`fs:allow-appdata-write-recursive` |  允许对完整的 `$APPDATA` 文件夹、文件和子目录进行完全递归写入访问。
`fs:allow-appdata-read` |  允许对 `$APPDATA` 文件夹进行非递归读取访问。
`fs:allow-appdata-write` |  允许对 `$APPDATA` 文件夹进行非递归写入访问。
`fs:allow-appdata-meta-recursive` |  允许对 `$APPDATA` 文件夹的元数据进行完全递归读取访问，包括文件列表和统计信息。
`fs:allow-appdata-meta` |  允许对 `$APPDATA` 文件夹的元数据进行非递归读取访问，包括文件列表和统计信息。
`fs:scope-appdata-recursive` |  此范围允许递归访问完整的 `$APPDATA` 文件夹，包括子目录和文件。
`fs:scope-appdata` |  此范围允许访问所有文件并列出 `$APPDATA` 文件夹中顶级目录的内容。
`fs:scope-appdata-index` |  此范围允许列出 `$APPDATA` 文件夹中的所有文件和文件夹。
`fs:allow-applocaldata-read-recursive` |  允许对完整的 `$APPLOCALDATA` 文件夹、文件和子目录进行完全递归读取访问。
`fs:allow-applocaldata-write-recursive` |  允许对完整的 `$APPLOCALDATA` 文件夹、文件和子目录进行完全递归写入访问。
`fs:allow-applocaldata-read` |  允许对 `$APPLOCALDATA` 文件夹进行非递归读取访问。
`fs:allow-applocaldata-write` |  允许对 `$APPLOCALDATA` 文件夹进行非递归写入访问。
`fs:allow-applocaldata-meta-recursive` |  允许对 `$APPLOCALDATA` 文件夹的元数据进行完全递归读取访问，包括文件列表和统计信息。
`fs:allow-applocaldata-meta` |  允许对 `$APPLOCALDATA` 文件夹的元数据进行非递归读取访问，包括文件列表和统计信息。
`fs:scope-applocaldata-recursive` |  此范围允许递归访问完整的 `$APPLOCALDATA` 文件夹，包括子目录和文件。
`fs:scope-applocaldata` |  此范围允许访问所有文件并列出 `$APPLOCALDATA` 文件夹中顶级目录的内容。
`fs:scope-applocaldata-index` |  此范围允许列出 `$APPLOCALDATA` 文件夹中的所有文件和文件夹。
`fs:allow-applog-read-recursive` |  允许对完整的 `$APPLOG` 文件夹、文件和子目录进行完全递归读取访问。
`fs:allow-applog-write-recursive` |  允许对完整的 `$APPLOG` 文件夹、文件和子目录进行完全递归写入访问。
`fs:allow-applog-read` |  允许对 `$APPLOG` 文件夹进行非递归读取访问。
`fs:allow-applog-write` |  允许对 `$APPLOG` 文件夹进行非递归写入访问。
`fs:allow-applog-meta-recursive` |  允许对 `$APPLOG` 文件夹的元数据进行完全递归读取访问，包括文件列表和统计信息。
`fs:allow-applog-meta` |  允许对 `$APPLOG` 文件夹的元数据进行非递归读取访问，包括文件列表和统计信息。
`fs:scope-applog-recursive` |  此范围允许递归访问完整的 `$APPLOG` 文件夹，包括子目录和文件。
`fs:scope-applog` |  此范围允许访问所有文件并列出 `$APPLOG` 文件夹中顶级目录的内容。
`fs:scope-applog-index` |  此范围允许列出 `$APPLOG` 文件夹中的所有文件和文件夹。
`fs:allow-audio-read-recursive` |  允许对完整的 `$AUDIO` 文件夹、文件和子目录进行完全递归读取访问。
`fs:allow-audio-write-recursive` |  允许对完整的 `$AUDIO` 文件夹、文件和子目录进行完全递归写入访问。
`fs:allow-audio-read` |  允许对 `$AUDIO` 文件夹进行非递归读取访问。
`fs:allow-audio-write` |  允许对 `$AUDIO` 文件夹进行非递归写入访问。
`fs:allow-audio-meta-recursive` |  允许对 `$AUDIO` 文件夹的元数据进行完全递归读取访问，包括文件列表和统计信息。
`fs:allow-audio-meta` |  允许对 `$AUDIO` 文件夹的元数据进行非递归读取访问，包括文件列表和统计信息。
`fs:scope-audio-recursive` |  此范围允许递归访问完整的 `$AUDIO` 文件夹，包括子目录和文件。
`fs:scope-audio` |  此范围允许访问所有文件并列出 `$AUDIO` 文件夹中顶级目录的内容。
`fs:scope-audio-index` |  此范围允许列出 `$AUDIO` 文件夹中的所有文件和文件夹。
`fs:allow-cache-read-recursive` |  允许对完整的 `$CACHE` 文件夹、文件和子目录进行完全递归读取访问。
`fs:allow-cache-write-recursive` |  允许对完整的 `$CACHE` 文件夹、文件和子目录进行完全递归写入访问。
`fs:allow-cache-read` |  允许对 `$CACHE` 文件夹进行非递归读取访问。
`fs:allow-cache-write` |  允许对 `$CACHE` 文件夹进行非递归写入访问。
`fs:allow-cache-meta-recursive` |  允许对 `$CACHE` 文件夹的元数据进行完全递归读取访问，包括文件列表和统计信息。
`fs:allow-cache-meta` |  允许对 `$CACHE` 文件夹的元数据进行非递归读取访问，包括文件列表和统计信息。
`fs:scope-cache-recursive` |  此范围允许递归访问完整的 `$CACHE` 文件夹，包括子目录和文件。
`fs:scope-cache` |  此范围允许访问所有文件并列出 `$CACHE` 文件夹中顶级目录的内容。
`fs:scope-cache-index` |  此范围允许列出 `$CACHE` 文件夹中的所有文件和文件夹。
`fs:allow-config-read-recursive` |  允许对完整的 `$CONFIG` 文件夹、文件和子目录进行完全递归读取访问。
`fs:allow-config-write-recursive` |  允许对完整的 `$CONFIG` 文件夹、文件和子目录进行完全递归写入访问。
`fs:allow-config-read` |  允许对 `$CONFIG` 文件夹进行非递归读取访问。
`fs:allow-config-write` |  允许对 `$CONFIG` 文件夹进行非递归写入访问。
`fs:allow-config-meta-recursive` |  允许对 `$CONFIG` 文件夹的元数据进行完全递归读取访问，包括文件列表和统计信息。
`fs:allow-config-meta` |  允许对 `$CONFIG` 文件夹的元数据进行非递归读取访问，包括文件列表和统计信息。
`fs:scope-config-recursive` |  此范围允许递归访问完整的 `$CONFIG` 文件夹，包括子目录和文件。
`fs:scope-config` |  此范围允许访问所有文件并列出 `$CONFIG` 文件夹中顶级目录的内容。
`fs:scope-config-index` |  此范围允许列出 `$CONFIG` 文件夹中的所有文件和文件夹。
`fs:allow-data-read-recursive` |  允许对完整的 `$DATA` 文件夹、文件和子目录进行完全递归读取访问。
`fs:allow-data-write-recursive` |  允许对完整的 `$DATA` 文件夹、文件和子目录进行完全递归写入访问。
`fs:allow-data-read` |  允许对 `$DATA` 文件夹进行非递归读取访问。
`fs:allow-data-write` |  允许对 `$DATA` 文件夹进行非递归写入访问。
`fs:allow-data-meta-recursive` |  允许对 `$DATA` 文件夹的元数据进行完全递归读取访问，包括文件列表和统计信息。
`fs:allow-data-meta` |  允许对 `$DATA` 文件夹的元数据进行非递归读取访问，包括文件列表和统计信息。
`fs:scope-data-recursive` |  此范围允许递归访问完整的 `$DATA` 文件夹，包括子目录和文件。
`fs:scope-data` |  此范围允许访问所有文件并列出 `$DATA` 文件夹中顶级目录的内容。
`fs:scope-data-index` |  此范围允许列出 `$DATA` 文件夹中的所有文件和文件夹。
`fs:allow-desktop-read-recursive` |  允许对完整的 `$DESKTOP` 文件夹、文件和子目录进行完全递归读取访问。
`fs:allow-desktop-write-recursive` |  允许对完整的 `$DESKTOP` 文件夹、文件和子目录进行完全递归写入访问。
`fs:allow-desktop-read` |  允许对 `$DESKTOP` 文件夹进行非递归读取访问。
`fs:allow-desktop-write` |  允许对 `$DESKTOP` 文件夹进行非递归写入访问。
`fs:allow-desktop-meta-recursive` |  允许对 `$DESKTOP` 文件夹的元数据进行完全递归读取访问，包括文件列表和统计信息。
`fs:allow-desktop-meta` |  允许对 `$DESKTOP` 文件夹的元数据进行非递归读取访问，包括文件列表和统计信息。
`fs:scope-desktop-recursive` |  此范围允许递归访问完整的 `$DESKTOP` 文件夹，包括子目录和文件。
`fs:scope-desktop` |  此范围允许访问所有文件并列出 `$DESKTOP` 文件夹中顶级目录的内容。
`fs:scope-desktop-index` |  此范围允许列出 `$DESKTOP` 文件夹中的所有文件和文件夹。
`fs:allow-document-read-recursive` |  允许对完整的 `$DOCUMENT` 文件夹、文件和子目录进行完全递归读取访问。
`fs:allow-document-write-recursive` |  允许对完整的 `$DOCUMENT` 文件夹、文件和子目录进行完全递归写入访问。
`fs:allow-document-read` |  允许对 `$DOCUMENT` 文件夹进行非递归读取访问。
`fs:allow-document-write` |  允许对 `$DOCUMENT` 文件夹进行非递归写入访问。
`fs:allow-document-meta-recursive` |  允许对 `$DOCUMENT` 文件夹的元数据进行完全递归读取访问，包括文件列表和统计信息。
`fs:allow-document-meta` |  允许对 `$DOCUMENT` 文件夹的元数据进行非递归读取访问，包括文件列表和统计信息。
`fs:scope-document-recursive` |  此范围允许递归访问完整的 `$DOCUMENT` 文件夹，包括子目录和文件。
`fs:scope-document` |  此范围允许访问所有文件并列出 `$DOCUMENT` 文件夹中顶级目录的内容。
`fs:scope-document-index` |  此范围允许列出 `$DOCUMENT` 文件夹中的所有文件和文件夹。
`fs:allow-download-read-recursive` |  允许对完整的 `$DOWNLOAD` 文件夹、文件和子目录进行完全递归读取访问。
`fs:allow-download-write-recursive` |  允许对完整的 `$DOWNLOAD` 文件夹、文件和子目录进行完全递归写入访问。
`fs:allow-download-read` |  允许对 `$DOWNLOAD` 文件夹进行非递归读取访问。
`fs:allow-download-write` |  允许对 `$DOWNLOAD` 文件夹进行非递归写入访问。
`fs:allow-download-meta-recursive` |  允许对 `$DOWNLOAD` 文件夹的元数据进行完全递归读取访问，包括文件列表和统计信息。
`fs:allow-download-meta` |  允许对 `$DOWNLOAD` 文件夹的元数据进行非递归读取访问，包括文件列表和统计信息。
`fs:scope-download-recursive` |  此范围允许递归访问完整的 `$DOWNLOAD` 文件夹，包括子目录和文件。
`fs:scope-download` |  此范围允许访问所有文件并列出 `$DOWNLOAD` 文件夹中顶级目录的内容。
`fs:scope-download-index` |  此范围允许列出 `$DOWNLOAD` 文件夹中的所有文件和文件夹。
`fs:allow-exe-read-recursive` |  允许对完整的 `$EXE` 文件夹、文件和子目录进行完全递归读取访问。
`fs:allow-exe-write-recursive` |  允许对完整的 `$EXE` 文件夹、文件和子目录进行完全递归写入访问。
`fs:allow-exe-read` |  允许对 `$EXE` 文件夹进行非递归读取访问。
`fs:allow-exe-write` |  允许对 `$EXE` 文件夹进行非递归写入访问。
`fs:allow-exe-meta-recursive` |  允许对 `$EXE` 文件夹的元数据进行完全递归读取访问，包括文件列表和统计信息。
`fs:allow-exe-meta` |  允许对 `$EXE` 文件夹的元数据进行非递归读取访问，包括文件列表和统计信息。
`fs:scope-exe-recursive` |  此范围允许递归访问完整的 `$EXE` 文件夹，包括子目录和文件。
`fs:scope-exe` |  此范围允许访问所有文件并列出 `$EXE` 文件夹中顶级目录的内容。
`fs:scope-exe-index` |  此范围允许列出 `$EXE` 文件夹中的所有文件和文件夹。
`fs:allow-font-read-recursive` |  允许对完整的 `$FONT` 文件夹、文件和子目录进行完全递归读取访问。
`fs:allow-font-write-recursive` |  允许对完整的 `$FONT` 文件夹、文件和子目录进行完全递归写入访问。
`fs:allow-font-read` |  允许对 `$FONT` 文件夹进行非递归读取访问。
`fs:allow-font-write` |  允许对 `$FONT` 文件夹进行非递归写入访问。
`fs:allow-font-meta-recursive` |  允许对 `$FONT` 文件夹的元数据进行完全递归读取访问，包括文件列表和统计信息。
`fs:allow-font-meta` |  允许对 `$FONT` 文件夹的元数据进行非递归读取访问，包括文件列表和统计信息。
`fs:scope-font-recursive` |  此范围允许递归访问完整的 `$FONT` 文件夹，包括子目录和文件。
`fs:scope-font` |  此范围允许访问所有文件并列出 `$FONT` 文件夹中顶级目录的内容。
`fs:scope-font-index` |  此范围允许列出 `$FONT` 文件夹中的所有文件和文件夹。
`fs:allow-home-read-recursive` |  允许对完整的 `$HOME` 文件夹、文件和子目录进行完全递归读取访问。
`fs:allow-home-write-recursive` |  允许对完整的 `$HOME` 文件夹、文件和子目录进行完全递归写入访问。
`fs:allow-home-read` |  允许对 `$HOME` 文件夹进行非递归读取访问。
`fs:allow-home-write` |  允许对 `$HOME` 文件夹进行非递归写入访问。
`fs:allow-home-meta-recursive` |  允许对 `$HOME` 文件夹的元数据进行完全递归读取访问，包括文件列表和统计信息。
`fs:allow-home-meta` |  允许对 `$HOME` 文件夹的元数据进行非递归读取访问，包括文件列表和统计信息。
`fs:scope-home-recursive` |  此范围允许递归访问完整的 `$HOME` 文件夹，包括子目录和文件。
`fs:scope-home` |  此范围允许访问所有文件并列出 `$HOME` 文件夹中顶级目录的内容。
`fs:scope-home-index` |  此范围允许列出 `$HOME` 文件夹中的所有文件和文件夹。
`fs:allow-localdata-read-recursive` |  允许对完整的 `$LOCALDATA` 文件夹、文件和子目录进行完全递归读取访问。
`fs:allow-localdata-write-recursive` |  允许对完整的 `$LOCALDATA` 文件夹、文件和子目录进行完全递归写入访问。
`fs:allow-localdata-read` |  允许对 `$LOCALDATA` 文件夹进行非递归读取访问。
`fs:allow-localdata-write` |  允许对 `$LOCALDATA` 文件夹进行非递归写入访问。
`fs:allow-localdata-meta-recursive` |  允许对 `$LOCALDATA` 文件夹的元数据进行完全递归读取访问，包括文件列表和统计信息。
`fs:allow-localdata-meta` |  允许对 `$LOCALDATA` 文件夹的元数据进行非递归读取访问，包括文件列表和统计信息。
`fs:scope-localdata-recursive` |  此范围允许递归访问完整的 `$LOCALDATA` 文件夹，包括子目录和文件。
`fs:scope-localdata` |  此范围允许访问所有文件并列出 `$LOCALDATA` 文件夹中顶级目录的内容。
`fs:scope-localdata-index` |  此范围允许列出 `$LOCALDATA` 文件夹中的所有文件和文件夹。
`fs:allow-log-read-recursive` |  允许对完整的 `$LOG` 文件夹、文件和子目录进行完全递归读取访问。
`fs:allow-log-write-recursive` |  允许对完整的 `$LOG` 文件夹、文件和子目录进行完全递归写入访问。
`fs:allow-log-read` |  允许对 `$LOG` 文件夹进行非递归读取访问。
`fs:allow-log-write` |  允许对 `$LOG` 文件夹进行非递归写入访问。
`fs:allow-log-meta-recursive` |  允许对 `$LOG` 文件夹的元数据进行完全递归读取访问，包括文件列表和统计信息。
`fs:allow-log-meta` |  允许对 `$LOG` 文件夹的元数据进行非递归读取访问，包括文件列表和统计信息。
`fs:scope-log-recursive` |  此范围允许递归访问完整的 `$LOG` 文件夹，包括子目录和文件。
`fs:scope-log` |  此范围允许访问所有文件并列出 `$LOG` 文件夹中顶级目录的内容。
`fs:scope-log-index` |  此范围允许列出 `$LOG` 文件夹中的所有文件和文件夹。
`fs:allow-picture-read-recursive` |  允许对完整的 `$PICTURE` 文件夹、文件和子目录进行完全递归读取访问。
`fs:allow-picture-write-recursive` |  允许对完整的 `$PICTURE` 文件夹、文件和子目录进行完全递归写入访问。
`fs:allow-picture-read` |  允许对 `$PICTURE` 文件夹进行非递归读取访问。
`fs:allow-picture-write` |  允许对 `$PICTURE` 文件夹进行非递归写入访问。
`fs:allow-picture-meta-recursive` |  允许对 `$PICTURE` 文件夹的元数据进行完全递归读取访问，包括文件列表和统计信息。
`fs:allow-picture-meta` |  允许对 `$PICTURE` 文件夹的元数据进行非递归读取访问，包括文件列表和统计信息。
`fs:scope-picture-recursive` |  此范围允许递归访问完整的 `$PICTURE` 文件夹，包括子目录和文件。
`fs:scope-picture` |  此范围允许访问所有文件并列出 `$PICTURE` 文件夹中顶级目录的内容。
`fs:scope-picture-index` |  此范围允许列出 `$PICTURE` 文件夹中的所有文件和文件夹。
`fs:allow-public-read-recursive` |  允许对完整的 `$PUBLIC` 文件夹、文件和子目录进行完全递归读取访问。
`fs:allow-public-write-recursive` |  允许对完整的 `$PUBLIC` 文件夹、文件和子目录进行完全递归写入访问。
`fs:allow-public-read` |  允许对 `$PUBLIC` 文件夹进行非递归读取访问。
`fs:allow-public-write` |  允许对 `$PUBLIC` 文件夹进行非递归写入访问。
`fs:allow-public-meta-recursive` |  允许对 `$PUBLIC` 文件夹的元数据进行完全递归读取访问，包括文件列表和统计信息。
`fs:allow-public-meta` |  允许对 `$PUBLIC` 文件夹的元数据进行非递归读取访问，包括文件列表和统计信息。
`fs:scope-public-recursive` |  此范围允许递归访问完整的 `$PUBLIC` 文件夹，包括子目录和文件。
`fs:scope-public` |  此范围允许访问所有文件并列出 `$PUBLIC` 文件夹中顶级目录的内容。
`fs:scope-public-index` |  此范围允许列出 `$PUBLIC` 文件夹中的所有文件和文件夹。
`fs:allow-resource-read-recursive` |  允许对完整的 `$RESOURCE` 文件夹、文件和子目录进行完全递归读取访问。
`fs:allow-resource-write-recursive` |  允许对完整的 `$RESOURCE` 文件夹、文件和子目录进行完全递归写入访问。
`fs:allow-resource-read` |  允许对 `$RESOURCE` 文件夹进行非递归读取访问。
`fs:allow-resource-write` |  允许对 `$RESOURCE` 文件夹进行非递归写入访问。
`fs:allow-resource-meta-recursive` |  允许对 `$RESOURCE` 文件夹的元数据进行完全递归读取访问，包括文件列表和统计信息。
`fs:allow-resource-meta` |  允许对 `$RESOURCE` 文件夹的元数据进行非递归读取访问，包括文件列表和统计信息。
`fs:scope-resource-recursive` |  此范围允许递归访问完整的 `$RESOURCE` 文件夹，包括子目录和文件。
`fs:scope-resource` |  此范围允许访问所有文件并列出 `$RESOURCE` 文件夹中顶级目录的内容。
`fs:scope-resource-index` |  此范围允许列出 `$RESOURCE` 文件夹中的所有文件和文件夹。
`fs:allow-runtime-read-recursive` |  允许对完整的 `$RUNTIME` 文件夹、文件和子目录进行完全递归读取访问。
`fs:allow-runtime-write-recursive` |  允许对完整的 `$RUNTIME` 文件夹、文件和子目录进行完全递归写入访问。
`fs:allow-runtime-read` |  允许对 `$RUNTIME` 文件夹进行非递归读取访问。
`fs:allow-runtime-write` |  允许对 `$RUNTIME` 文件夹进行非递归写入访问。
`fs:allow-runtime-meta-recursive` |  允许对 `$RUNTIME` 文件夹的元数据进行完全递归读取访问，包括文件列表和统计信息。
`fs:allow-runtime-meta` |  允许对 `$RUNTIME` 文件夹的元数据进行非递归读取访问，包括文件列表和统计信息。
`fs:scope-runtime-recursive` |  此范围允许递归访问完整的 `$RUNTIME` 文件夹，包括子目录和文件。
`fs:scope-runtime` |  此范围允许访问所有文件并列出 `$RUNTIME` 文件夹中顶级目录的内容。
`fs:scope-runtime-index` |  此范围允许列出 `$RUNTIME` 文件夹中的所有文件和文件夹。
`fs:allow-temp-read-recursive` |  允许对完整的 `$TEMP` 文件夹、文件和子目录进行完全递归读取访问。
`fs:allow-temp-write-recursive` |  允许对完整的 `$TEMP` 文件夹、文件和子目录进行完全递归写入访问。
`fs:allow-temp-read` |  允许对 `$TEMP` 文件夹进行非递归读取访问。
`fs:allow-temp-write` |  允许对 `$TEMP` 文件夹进行非递归写入访问。
`fs:allow-temp-meta-recursive` |  允许对 `$TEMP` 文件夹的元数据进行完全递归读取访问，包括文件列表和统计信息。
`fs:allow-temp-meta` |  允许对 `$TEMP` 文件夹的元数据进行非递归读取访问，包括文件列表和统计信息。
`fs:scope-temp-recursive` |  此范围允许递归访问完整的 `$TEMP` 文件夹，包括子目录和文件。
`fs:scope-temp` |  此范围允许访问所有文件并列出 `$TEMP` 文件夹中顶级目录的内容。
`fs:scope-temp-index` |  此范围允许列出 `$TEMP` 文件夹中的所有文件和文件夹。
`fs:allow-template-read-recursive` |  允许对完整的 `$TEMPLATE` 文件夹、文件和子目录进行完全递归读取访问。
`fs:allow-template-write-recursive` |  允许对完整的 `$TEMPLATE` 文件夹、文件和子目录进行完全递归写入访问。
`fs:allow-template-read` |  允许对 `$TEMPLATE` 文件夹进行非递归读取访问。
`fs:allow-template-write` |  允许对 `$TEMPLATE` 文件夹进行非递归写入访问。
`fs:allow-template-meta-recursive` |  允许对 `$TEMPLATE` 文件夹的元数据进行完全递归读取访问，包括文件列表和统计信息。
`fs:allow-template-meta` |  允许对 `$TEMPLATE` 文件夹的元数据进行非递归读取访问，包括文件列表和统计信息。
`fs:scope-template-recursive` |  此范围允许递归访问完整的 `$TEMPLATE` 文件夹，包括子目录和文件。
`fs:scope-template` |  此范围允许访问所有文件并列出 `$TEMPLATE` 文件夹中顶级目录的内容。
`fs:scope-template-index` |  此范围允许列出 `$TEMPLATE` 文件夹中的所有文件和文件夹。
`fs:allow-video-read-recursive` |  允许对完整的 `$VIDEO` 文件夹、文件和子目录进行完全递归读取访问。
`fs:allow-video-write-recursive` |  允许对完整的 `$VIDEO` 文件夹、文件和子目录进行完全递归写入访问。
`fs:allow-video-read` |  允许对 `$VIDEO` 文件夹进行非递归读取访问。
`fs:allow-video-write` |  允许对 `$VIDEO` 文件夹进行非递归写入访问。
`fs:allow-video-meta-recursive` |  允许对 `$VIDEO` 文件夹的元数据进行完全递归读取访问，包括文件列表和统计信息。
`fs:allow-video-meta` |  允许对 `$VIDEO` 文件夹的元数据进行非递归读取访问，包括文件列表和统计信息。
`fs:scope-video-recursive` |  此范围允许递归访问完整的 `$VIDEO` 文件夹，包括子目录和文件。
`fs:scope-video` |  此范围允许访问所有文件并列出 `$VIDEO` 文件夹中顶级目录的内容。
`fs:scope-video-index` |  此范围允许列出 `$VIDEO` 文件夹中的所有文件和文件夹。
`fs:allow-copy-file` |  启用 copy_file 命令，无需任何预配置的作用域。
`fs:deny-copy-file` |  拒绝 copy_file 命令，无需任何预配置的作用域。
`fs:allow-create` |  启用 create 命令，无需任何预配置的作用域。
`fs:deny-create` |  拒绝 create 命令，无需任何预配置的作用域。
`fs:allow-exists` |  启用 exists 命令，无需任何预配置的作用域。
`fs:deny-exists` |  拒绝 exists 命令，无需任何预配置的作用域。
`fs:allow-fstat` |  启用 fstat 命令，无需任何预配置的作用域。
`fs:deny-fstat` |  拒绝 fstat 命令，无需任何预配置的作用域。
`fs:allow-ftruncate` |  启用 ftruncate 命令，无需任何预配置的作用域。
`fs:deny-ftruncate` |  拒绝 ftruncate 命令，无需任何预配置的作用域。
`fs:allow-lstat` |  启用 lstat 命令，无需任何预配置的作用域。
`fs:deny-lstat` |  拒绝 lstat 命令，无需任何预配置的作用域。
`fs:allow-mkdir` |  启用 mkdir 命令，无需任何预配置的作用域。
`fs:deny-mkdir` |  拒绝 mkdir 命令，无需任何预配置的作用域。
`fs:allow-open` |  启用打开命令，不带任何预配置范围。
`fs:deny-open` |  禁用打开命令，不带任何预配置范围。
`fs:allow-read` |  启用 read 命令，无需任何预配置的作用域。
`fs:deny-read` |  拒绝 read 命令，无需任何预配置的作用域。
`fs:allow-read-dir` |  启用 read_dir 命令，无需任何预配置的作用域。
`fs:deny-read-dir` |  拒绝 read_dir 命令，无需任何预配置的作用域。
`fs:allow-read-file` |  启用 read_file 命令，无需任何预配置的作用域。
`fs:deny-read-file` |  拒绝 read_file 命令，无需任何预配置的作用域。
`fs:allow-read-text-file` |  启用 read_text_file 命令，无需任何预配置的作用域。
`fs:deny-read-text-file` |  拒绝 read_text_file 命令，无需任何预配置的作用域。
`fs:allow-read-text-file-lines` |  启用 read_text_file_lines 命令，无需任何预配置的作用域。
`fs:deny-read-text-file-lines` |  拒绝 read_text_file_lines 命令，无需任何预配置的作用域。
`fs:allow-read-text-file-lines-next` |  启用 read_text_file_lines_next 命令，无需任何预配置的作用域。
`fs:deny-read-text-file-lines-next` |  拒绝 read_text_file_lines_next 命令，无需任何预配置的作用域。
`fs:allow-remove` |  启用 remove 命令，无需任何预配置的作用域。
`fs:deny-remove` |  拒绝 remove 命令，无需任何预配置的作用域。
`fs:allow-rename` |  启用 rename 命令，无需任何预配置的作用域。
`fs:deny-rename` |  拒绝 rename 命令，无需任何预配置的作用域。
`fs:allow-seek` |  启用 seek 命令，无需任何预配置的作用域。
`fs:deny-seek` |  拒绝 seek 命令，无需任何预配置的作用域。
`fs:allow-size` |  启用 size 命令，无需任何预配置的作用域。
`fs:deny-size` |  拒绝 size 命令，无需任何预配置的作用域。
`fs:allow-start-accessing-security-scoped-resource` |  启用 start_accessing_security_scoped_resource 命令，无需任何预配置的作用域。
`fs:deny-start-accessing-security-scoped-resource` |  拒绝 start_accessing_security_scoped_resource 命令，无需任何预配置的作用域。
`fs:allow-stat` |  启用 stat 命令，无需任何预配置的作用域。
`fs:deny-stat` |  拒绝 stat 命令，无需任何预配置的作用域。
`fs:allow-stop-accessing-security-scoped-resource` |  在没有预配置范围的情况下启用 stop_accessing_security_scoped_resource 命令。
`fs:deny-stop-accessing-security-scoped-resource` |  在没有预配置范围的情况下禁止 stop_accessing_security_scoped_resource 命令。
`fs:allow-truncate` |  在没有预配置范围的情况下启用 truncate 命令。
`fs:deny-truncate` |  在没有预配置范围的情况下禁止 truncate 命令。
`fs:allow-unwatch` |  在没有预配置范围的情况下启用 unwatch 命令。
`fs:deny-unwatch` |  在没有预配置范围的情况下禁止 unwatch 命令。
`fs:allow-watch` |  在没有预配置范围的情况下启用 watch 命令。
`fs:deny-watch` |  在没有预配置范围的情况下禁止 watch 命令。
`fs:allow-write` |  在没有预配置范围的情况下启用 write 命令。
`fs:deny-write` |  在没有预配置范围的情况下禁止 write 命令。
`fs:allow-write-file` |  在没有预配置范围的情况下启用 write_file 命令。
`fs:deny-write-file` |  在没有预配置范围的情况下禁止 write_file 命令。
`fs:allow-write-text-file` |  在没有预配置范围的情况下启用 write_text_file 命令。
`fs:deny-write-text-file` |  在没有预配置范围的情况下禁止 write_text_file 命令。
`fs:create-app-specific-dirs` |  该权限允许创建特定于应用程序的目录。
`fs:deny-default` |  默认情况下，禁止访问危险的 Tauri 相关文件和文件夹。
`fs:deny-webview-data-linux` |  这会禁止在 Linux 上读取 `$APPLOCALDATA` 文件夹，因为 Webview 数据和配置值存储在此处。允许访问可能导致敏感信息泄露，应慎重考虑。
`fs:deny-webview-data-windows` |  这会禁止在 Windows 上读取 `$APPLOCALDATA/EBWebView` 文件夹，因为 Webview 数据和配置值存储在此处。允许访问可能导致敏感信息泄露，应慎重考虑。
`fs:read-all` |  在没有预配置可访问路径的情况下，启用所有与读取相关的命令。
`fs:read-app-specific-dirs-recursive` |  该权限允许对应用程序特定的基目录进行递归读取。
`fs:read-dirs` |  在没有预配置可访问路径的情况下，启用目录读取和文件元数据相关命令。
`fs:read-files` |  在没有预配置可访问路径的情况下，启用所有与文件读取相关的命令。
`fs:read-meta` |  在没有预配置可访问路径的情况下，启用所有与索引或元数据相关的命令。
`fs:scope` |  一个可用于修改全局范围的空权限。

## 示例

    {
      "identifier": "read-documents",
      "windows": ["main"],
      "permissions": [
        "fs:allow-read",
        {
          "identifier": "fs:scope",
          "allow": [
            "$APPDATA/documents/**/*"
          ],
          "deny": [
            "$APPDATA/documents/secret.txt"
          ]
        }
      ]
    }

`fs:write-all` |  在没有预配置可访问路径的情况下，启用所有与写入相关的命令。
`fs:write-files` |  在没有预配置可访问路径的情况下，启用所有与文件写入相关的命令。

### 范围

标题为“范围 (Scopes)”的章节

此插件权限包含用于定义允许或明确拒绝哪些路径的范围。有关范围的更多信息，请参阅 [命令范围](/security/scope/)。

每个 `allow`（允许）或 `deny`（拒绝）范围都必须包含一个列表，列出所有应被允许或拒绝的路径。范围条目的格式为 `{ path: string }`。

注意

`deny`（拒绝）优先于 `allow`（允许），因此如果一个路径被范围所拒绝，即使它被另一个范围所允许，它在运行时也会被阻止。

范围条目可以使用 `$<path>` 变量来引用常见的系统路径，例如主目录、应用程序资源目录和配置目录。下表列出了您可以引用的所有常见路径。

路径| 变量
---|---
[appConfigDir](https://v2.tauri.org.cn/reference/javascript/api/namespacepath/#appconfigdir)| $APPCONFIG
[appDataDir](https://v2.tauri.org.cn/reference/javascript/api/namespacepath/#appdatadir)| $APPDATA
[appLocalDataDir](https://v2.tauri.org.cn/reference/javascript/api/namespacepath/#appLocaldatadir)| $APPLOCALDATA
[appcacheDir](https://v2.tauri.org.cn/reference/javascript/api/namespacepath/#appcachedir)| $APPCACHE
[applogDir](https://v2.tauri.org.cn/reference/javascript/api/namespacepath/#applogdir)| $APPLOG
[audioDir](https://v2.tauri.org.cn/reference/javascript/api/namespacepath/#audiodir)| $AUDIO
[cacheDir](https://v2.tauri.org.cn/reference/javascript/api/namespacepath/#cachedir)| $CACHE
[configDir](https://v2.tauri.org.cn/reference/javascript/api/namespacepath/#configdir)| $CONFIG
[dataDir](https://v2.tauri.org.cn/reference/javascript/api/namespacepath/#datadir)| $DATA
[localDataDir](https://v2.tauri.org.cn/reference/javascript/api/namespacepath/#localdatadir)| $LOCALDATA
[desktopDir](https://v2.tauri.org.cn/reference/javascript/api/namespacepath/#desktopdir)| $DESKTOP
[documentDir](https://v2.tauri.org.cn/reference/javascript/api/namespacepath/#documentdir)| $DOCUMENT
[downloadDir](https://v2.tauri.org.cn/reference/javascript/api/namespacepath/#downloaddir)| $DOWNLOAD
[executableDir](https://v2.tauri.org.cn/reference/javascript/api/namespacepath/#executabledir)| $EXE
[fontDir](https://v2.tauri.org.cn/reference/javascript/api/namespacepath/#fontdir)| $FONT
[homeDir](https://v2.tauri.org.cn/reference/javascript/api/namespacepath/#homedir)| $HOME
[pictureDir](https://v2.tauri.org.cn/reference/javascript/api/namespacepath/#picturedir)| $PICTURE
[publicDir](https://v2.tauri.org.cn/reference/javascript/api/namespacepath/#publicdir)| $PUBLIC
[runtimeDir](https://v2.tauri.org.cn/reference/javascript/api/namespacepath/#runtimedir)| $RUNTIME
[templateDir](https://v2.tauri.org.cn/reference/javascript/api/namespacepath/#templatedir)| $TEMPLATE
[videoDir](https://v2.tauri.org.cn/reference/javascript/api/namespacepath/#videodir)| $VIDEO
[resourceDir](https://v2.tauri.org.cn/reference/javascript/api/namespacepath/#resourcedir)| $RESOURCE
[tempDir](https://v2.tauri.org.cn/reference/javascript/api/namespacepath/#tempdir)| $TEMP

#### 示例

名为“示例”的章节

  * 全局范围

要将范围应用于任何 `fs` 命令，请使用 `fs:scope` 权限。

src-tauri/capabilities/default.json

    {

      "$schema": "../gen/schemas/desktop-schema.json",

      "identifier": "main-capability",

      "description": "Capability for the main window",

      "windows": ["main"],

      "permissions": [

        {

          "identifier": "fs:scope",

          "allow": [{ "path": "$APPDATA" }, { "path": "$APPDATA/**/*" }]

        }

      ]

    }

要将范围应用于特定的 `fs` 命令，请使用对象形式的权限 `{ "identifier": string, "allow"?: [], "deny"?: [] }`。

src-tauri/capabilities/default.json

    {

      "$schema": "../gen/schemas/desktop-schema.json",

      "identifier": "main-capability",

      "description": "Capability for the main window",

      "windows": ["main"],

      "permissions": [

        {

          "identifier": "fs:allow-rename",

          "allow": [{ "path": "$HOME/**/*" }]

        },

        {

          "identifier": "fs:allow-rename",

          "deny": [{ "path": "$HOME/.config/**/*" }]

        },

        {

          "identifier": "fs:allow-exists",

          "allow": [{ "path": "$APPDATA/*" }]

        }

      ]

    }

在上面的示例中，您可以使用 `exists` API 来使用任何 `$APPDATA` 子路径（不包括子目录），以及 `rename`。

提示

如果您尝试在基于 Unix 的系统上访问点文件（例如 `.gitignore`）或点文件夹（例如 `.ssh`），则需要指定完整路径 `/home/user/.ssh/example` 或点文件夹路径组件之后的 glob `/home/user/.ssh/*`。

如果这在您的用例中不起作用，那么您可以配置该插件将任何组件视为有效的路径字面量。

"src-tauri/tauri.conf.json

     "plugins": {

        "fs": {

          "requireLiteralLeadingDot": false

        }

      }