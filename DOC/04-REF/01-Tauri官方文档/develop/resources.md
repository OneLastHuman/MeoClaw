# 嵌入附加文件

_Source: https://v2.tauri.org.cn/develop/resources/_

你可能需要在应用程序包中包含一些不直接属于你的前端（即 `frontendDist`）或者太大而无法直接嵌入到二进制文件中的额外文件。我们将这些文件称为 `resources`（资源）。

## 配置

名为“配置”的章节

要打包你选择的文件，请在 `tauri.conf.json` 文件中将 `resources` 属性添加到 `bundle` 对象中。

包含文件列表

  * 语法
  * 说明

tauri.conf.json

    {

      "bundle": {

        "resources": [

          "./path/to/some-file.txt",

          "/absolute/path/to/textfile.txt",

          "../relative/path/to/jsonfile.json",

          "some-folder/",

          "resources/**/*.md"

        ]

      }

    }

tauri.conf.json5

    {

      "bundle": {

        "resources": [

          // Will be placed to `$RESOURCE/path/to/some-file.txt`

          "./path/to/some-file.txt",

          // The root in an absolute path will be replaced by `_root_`,

          // so `textfile.txt` will be placed to `$RESOURCE/_root_/absolute/path/to/textfile.txt`

          "/absolute/path/to/textfile.txt",

          // `..` in a relative path will be replaced by `_up_`,

          // so `jsonfile.json` will be placed to `$RESOURCE/_up_/relative/path/to/textfile.txt`,

          "../relative/path/to/jsonfile.json",

          // If the path is a directory, the entire directory will be copied to the `$RESOURCE` directory,

          // preserving the original structures, for example:

          //   - `some-folder/file.txt`                   -> `$RESOURCE/some-folder/file.txt`

          //   - `some-folder/another-folder/config.json` -> `$RESOURCE/some-folder/another-folder/config.json`

          // This is the same as `some-folder/**/*`

          "some-folder/",

          // You can also include multiple files at once through glob patterns.

          // All the `.md` files inside `resources` will be placed to `$RESOURCE/resources/`,

          // preserving their original directory structures, for example:

          //   - `resources/index.md`      -> `$RESOURCE/resources/index.md`

          //   - `resources/docs/setup.md` -> `$RESOURCE/resources/docs/setup.md`

          "resources/**/*.md"

        ]

      }

    }

打包后的文件将位于 `$RESOURCES/` 目录下，并保留原始的目录结构，例如：`./path/to/some-file.txt` -> `$RESOURCE/path/to/some-file.txt`

若要精细控制文件的复制目标位置，请改用映射（map）方式

  * 语法
  * 说明

tauri.conf.json

    {

      "bundle": {

        "resources": {

          "/absolute/path/to/textfile.txt": "resources/textfile.txt",

          "relative/path/to/jsonfile.json": "resources/jsonfile.json",

          "resources/": "",

          "docs/**/*md": "website-docs/"

        }

      }

    }

tauri.conf.json5

    {

      "bundle": {

        "resources": {

          // `textfile.txt` will be placed to `$RESOURCE/resources/textfile.txt`

          "/absolute/path/to/textfile.txt": "resources/textfile.txt",

          // `jsonfile.json` will be placed to `$RESOURCE/resources/jsonfile.json`

          "relative/path/to/jsonfile.json": "resources/jsonfile.json",

          // Copy the entire directory to `$RESOURCE`, preserving the original structures,

          // the target is "" which means it will be placed directly in the resource directory `$RESOURCE`, for example:

          //   - `resources/file.txt`                -> `$RESOURCE/file.txt`

          //   - `resources/some-folder/config.json` -> `$RESOURCE/some-folder/config.json`

          "resources/": "",

          // When using glob patterns, the behavior is different from the list one,

          // all the matching files will be placed to the target directory without preserving the original file structures

          // for example:

          //   - `docs/index.md`         -> `$RESOURCE/website-docs/index.md`

          //   - `docs/plugins/setup.md` -> `$RESOURCE/website-docs/setup.md`

          "docs/**/*md": "website-docs/"

        }

      }

    }

要了解 `$RESOURCE` 在各平台上的解析位置，请参阅 [`resource_dir`](https://docs.rs/tauri/latest/tauri/path/struct.PathResolver.html#method.resource_dir) 的文档。

源路径语法

在以下说明中，“目标资源目录”指的是对象表示法中冒号后的值，或者数组表示法中原始文件路径的重构路径。

  * `"dir/file.txt"`：将 `file.txt` 文件复制到目标资源目录中。
  * `"dir/"`：将所有文件**和目录** _递归_ 复制到目标资源目录中。如果你想保留文件和目录的文件系统结构，请使用此项。
  * `"dir/*"`：将 `dir` 目录下的所有文件 _非递归地_ （子目录将被忽略）复制到目标资源目录中。
  * `"dir/**"`：会抛出错误，因为 `**` 仅匹配目录，因此找不到任何文件。
  * `"dir/**/*"`：将 `dir` 目录下的所有文件 _递归地_ （包括 `dir/` 下的所有文件以及所有子目录中的所有文件）复制到目标资源目录中。
  * `"dir/**/**"`：会抛出错误，因为 `**` 仅匹配目录，因此找不到任何文件。

## 解析资源文件路径

名为“解析资源文件路径”的章节

要解析资源文件的路径，请使用以下 API，而不是手动计算路径：

  * Rust
  * JavaScript

在 Rust 端，你需要一个 [`PathResolver`](https://docs.rs/tauri/latest/tauri/path/struct.PathResolver.html) 实例，你可以从 [`App`](https://docs.rs/tauri/latest/tauri/struct.App.html) 和 [`AppHandle`](https://docs.rs/tauri/latest/tauri/struct.AppHandle.html) 获取它，然后调用 [`PathResolver::resolve`](https://docs.rs/tauri/latest/tauri/path/struct.PathResolver.html#method.resolve)。

    tauri::Builder::default()

      .setup(|app| {

        let resource_path = app.path().resolve("lang/de.json", BaseDirectory::Resource)?;

        Ok(())

      })

在命令（Command）中使用：

    #[tauri::command]

    fn hello(handle: tauri::AppHandle) {

      let resource_path = handle.path().resolve("lang/de.json", BaseDirectory::Resource)?;

    }

若要在 JavaScript 中解析路径，请使用 [`resolveResource`](https://tauri.org.cn/reference/javascript/api/namespacepath/#resolveresource)。

    import { resolveResource } from '@tauri-apps/api/path';

    const resourcePath = await resolveResource('lang/de.json');

### 路径语法

名为“路径语法”的章节

API 调用中的路径可以是普通的相对路径（如 `folder/json_file.json`，解析为 `$RESOURCE/folder/json_file.json`），也可以是类似 `../relative/folder/toml_file.toml` 的路径（解析为 `$RESOURCE/_up_/relative/folder/toml_file.toml`）。这些 API 使用与在 `tauri.conf.json > bundle > resources` 中编写时相同的规则，例如：

tauri.conf.json

    {

      "bundle": {

        "resources": ["folder/json_file.json", "../relative/folder/toml_file.toml"]

      }

    }

    let json_path = app.path().resolve("folder/json_file.json", BaseDirectory::Resource)?;

    let toml_path = app.path().resolve("../relative/folder/toml_file.toml", BaseDirectory::Resource)?;

### Android

标题为“Android”的章节

目前，资源在 APK 中作为资产（assets）存储，因此这些 API 的返回值并不是普通的文件系统路径。我们在此处使用了一个特殊的 URI 前缀 `asset:///`，它可以配合 [`fs` 插件](/plugin/file-system/)使用。通过该插件，你可以使用 [`FsExt::fs`](https://docs.rs/tauri-plugin-fs/latest/tauri_plugin_fs/trait.FsExt.html#tymethod.fs) 读取文件，如下所示：

    let resource_path = app.path().resolve("lang/de.json", BaseDirectory::Resource).unwrap();

    let json = app.fs().read_to_string(&resource_path);

如果你希望或必须将资源文件放在实际的文件系统中，请通过 [`fs` 插件](/plugin/file-system/)手动复制内容。

## 读取资源文件

名为“读取资源文件”的章节

在此示例中，我们想要捆绑额外的 i18n JSON 文件，如下所示：

    .

    ├── src-tauri/

    │   ├── tauri.conf.json

    │   ├── lang/

    │   │   ├── de.json

    │   │   └── en.json

    │   └── ...

    └── ...

tauri.conf.json

    {

      "bundle": {

        "resources": ["lang/*"]

      }

    }

lang/de.json

    {

      "hello": "Guten Tag!",

      "bye": "Auf Wiedersehen!"

    }

### Rust

名为“Rust”的章节

在 Rust 端，你需要一个 [`PathResolver`](https://docs.rs/tauri/latest/tauri/path/struct.PathResolver.html) 实例，你可以从 [`App`](https://docs.rs/tauri/latest/tauri/struct.App.html) 和 [`AppHandle`](https://docs.rs/tauri/latest/tauri/struct.AppHandle.html) 获取它。

    tauri::Builder::default()

      .setup(|app| {

        // The path specified must follow the same syntax as defined in

        // `tauri.conf.json > bundle > resources`

        let resource_path = app.path().resolve("lang/de.json", BaseDirectory::Resource)?;

        let json = std::fs::read_to_string(&resource_path).unwrap();

        // Or when dealing with Android, use the file system plugin instead

        // let json = app.fs().read_to_string(&resource_path);

        let lang_de: serde_json::Value = serde_json::from_str(json).unwrap();

        // This will print 'Guten Tag!' to the terminal

        println!("{}", lang_de.get("hello").unwrap());

        Ok(())

      })

    #[tauri::command]

    fn hello(handle: tauri::AppHandle) -> String {

        let resource_path = handle.path().resolve("lang/de.json", BaseDirectory::Resource)?;

        let json = std::fs::read_to_string(&resource_path).unwrap();

        // Or when dealing with Android, use the file system plugin instead

        // let json = handle.fs().read_to_string(&resource_path);

        let lang_de: serde_json::Value = serde_json::from_str(json).unwrap();

        lang_de.get("hello").unwrap()

    }

### JavaScript

名为“JavaScript”的章节

对于 JavaScript 端，你可以使用类似上述的命令并通过 `await invoke('hello')` 调用，或者使用 [`fs` 插件](/plugin/file-system/)访问这些文件。

使用 [`fs` 插件](/plugin/file-system/)时，除了[基本设置](/plugin/file-system/#setup)外，你还需要配置访问控制列表（ACL）以启用所需的插件 API，以及授予访问 `$RESOURCE` 文件夹的权限。

src-tauri/capabilities/default.json

    {

      "$schema": "../gen/schemas/desktop-schema.json",

      "identifier": "main-capability",

      "description": "Capability for the main window",

      "windows": ["main"],

      "permissions": [

        "core:default",

        "fs:allow-read-text-file",

        "fs:allow-resource-read-recursive"

      ]

    }

注意

此处我们使用 `fs:allow-resource-read-recursive` 来允许对整个 `$RESOURCE` 文件夹、文件及子目录进行完全递归读取访问。更多信息请阅读 [范围权限 (Scope Permissions)](/plugin/file-system/#scopes) 以了解其他选项，或查看 [范围 (Scopes)](/plugin/file-system/#scopes) 以获取更精细的控制。

    import { resolveResource } from '@tauri-apps/api/path';

    import { readTextFile } from '@tauri-apps/plugin-fs';

    const resourcePath = await resolveResource('lang/de.json');

    const langDe = JSON.parse(await readTextFile(resourcePath));

    console.log(langDe.hello); // This will print 'Guten Tag!' to the devtools console

## Permissions

名为“权限”的章节

由于我们在使用列表时将相对路径中的 `../` 替换为 `_up_`，将绝对路径中的根目录替换为 `_root_`，这些文件将位于资源目录内的子文件夹中。为了在 Tauri 的 [权限系统](/security/capabilities/)中允许这些路径，请使用 `$RESOURCE/**/*` 来允许对这些文件的递归访问。

### 示例

名为“示例”的章节

对于这样打包的文件：

tauri.conf.json

    {

      "bundle": {

        "resources": ["../relative/path/to/jsonfile.json"]

      }

    }

配合 [`fs` 插件](/plugin/file-system/)使用：

src-tauri/capabilities/default.json

    {

      "$schema": "../gen/schemas/desktop-schema.json",

      "identifier": "main-capability",

      "description": "Capability for the main window",

      "windows": ["main"],

      "permissions": [

        "core:default",

        "fs:allow-stat",

        "fs:allow-read-text-file",

        "fs:allow-resource-read-recursive",

        {

          "identifier": "fs:scope",

          "allow": ["$RESOURCE/**/*"],

          "deny": ["$RESOURCE/secret.txt"]

        }

      ]

    }

配合 [`opener` 插件](/plugin/opener/)使用：

src-tauri/capabilities/default.json

    {

      "$schema": "../gen/schemas/desktop-schema.json",

      "identifier": "main-capability",

      "description": "Capability for the main window",

      "windows": ["main"],

      "permissions": [

        "core:default",

        {

          "identifier": "opener:allow-open-path",

          "allow": [

            {

              "path": "$RESOURCE/**/*"

            }

          ]

        }

      ]

    }