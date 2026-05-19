# 嵌入外部二进制文件

_Source: https://v2.tauri.org.cn/develop/sidecar/_

你可能需要嵌入外部二进制文件来为应用程序添加额外功能，或者避免用户安装额外的依赖（例如 Node.js 或 Python）。我们将这种二进制文件称为 `sidecar`（边车）。

二进制文件是指使用任何编程语言编写的可执行文件。常见的用例是使用 `pyinstaller` 打包的 Python CLI 应用程序或 API 服务器。

要打包你选择的二进制文件，可以在 `tauri.conf.json` 的 `tauri > bundle` 对象中添加 `externalBin` 属性。`externalBin` 配置需要一个字符串列表，指向二进制文件的绝对路径或相对路径。

以下是一个 Tauri 配置片段，展示了 sidecar 的配置方式：

src-tauri/tauri.conf.json

    {

      "bundle": {

        "externalBin": [

          "/absolute/path/to/sidecar",

          "../relative/path/to/binary",

          "binaries/my-sidecar"

        ]

      }

    }

注意

相对路径是相对于位于 `src-tauri` 目录下的 `tauri.conf.json` 文件而言的。因此 `binaries/my-sidecar` 代表 `<项目根目录>/src-tauri/binaries/my-sidecar`。

为了使外部二进制文件能在每个支持的架构上运行，指定路径下必须存在一个名称相同且带有 `-$TARGET_TRIPLE` 后缀的二进制文件。例如，`"externalBin": ["binaries/my-sidecar"]` 在 Linux 上需要一个 `src-tauri/binaries/my-sidecar-x86_64-unknown-linux-gnu` 可执行文件，或在搭载 Apple Silicon 的 macOS 上需要一个 `src-tauri/binaries/my-sidecar-aarch64-apple-darwin`。

你可以通过运行以下命令来查找**当前** 平台的 `-$TARGET_TRIPLE` 后缀：

终端窗口

    rustc --print host-tuple

这会直接输出你主机的目标三元组（例如 `x86_64-unknown-linux-gnu` 或 `aarch64-apple-darwin`）。

注意

`--print host-tuple` 标志是在 Rust 1.84.0 中添加的。如果你使用的是旧版本，则需要解析 `rustc -Vv` 的输出。

终端窗口

    # Unix (Linux/macOS)

    rustc -Vv | grep host | cut -f2 -d' '

    # Windows PowerShell

    rustc -Vv | Select-String "host:" | ForEach-Object {$_.Line.split(" ")[1]}

这是一个将目标三元组附加到二进制文件名的 Node.js 脚本：

    import { execSync } from 'child_process';

    import fs from 'fs';

    const extension = process.platform === 'win32' ? '.exe' : '';

    const targetTriple = execSync('rustc --print host-tuple').toString().trim();

    if (!targetTriple) {

      console.error('Failed to determine platform target triple');

    }

    fs.renameSync(

      `src-tauri/binaries/sidecar${extension}`,

      `src-tauri/binaries/sidecar-${targetTriple}${extension}`

    );

请注意，如果编译的目标架构与运行脚本的架构不同，此脚本将无法工作，因此仅将其作为你构建脚本的起点。

## 从 Rust 运行

标题为“从 Rust 运行”的章节

注意

请先按照 [shell 插件指南](/plugin/shell/)正确设置并初始化该插件。如果不初始化和配置插件，该示例将无法运行。

在 Rust 端，导入 `tauri_plugin_shell::ShellExt` trait 并在 `AppHandle` 上调用 `shell().sidecar()` 函数：

    use tauri_plugin_shell::ShellExt;

    use tauri_plugin_shell::process::CommandEvent;

    use tauri::Emitter;

    let sidecar_command = app.shell().sidecar("my-sidecar").unwrap();

    let (mut rx, mut child) = sidecar_command

      .spawn()

      .expect("Failed to spawn sidecar");

    tauri::async_runtime::spawn(async move {

      // read events such as stdout

      while let Some(event) = rx.recv().await {

        if let CommandEvent::Stdout(line_bytes) = event {

          let line = String::from_utf8_lossy(&line_bytes);

          app

            .emit("message", Some(format!("'{}'", line)))

            .expect("failed to emit event");

          // write to stdin

          child.write("message from Rust\n".as_bytes()).unwrap();

        }

      }

    });

注意

`sidecar()` 函数只需文件名，**不要** 传入 `externalBin` 数组中配置的完整路径。

给定以下配置：

src-tauri/tauri.conf.json

    {

      "bundle": {

        "externalBin": ["binaries/app", "my-sidecar", "../scripts/sidecar"]

      }

    }

执行 sidecar 的正确方法是调用 `app.shell().sidecar(name)`，其中 `name` 为 `"app"`、`"my-sidecar"` 或 `"sidecar"`，而不是 `"binaries/app"`。

你可以将此代码放入 Tauri 命令中以便轻松传递 `AppHandle`，或者在构建脚本中存储对 `AppHandle` 的引用，以便在应用程序的其他地方访问它。

## 从 JavaScript 运行

标题为“从 JavaScript 运行”的章节

运行 sidecar 时，Tauri 要求你授予 sidecar 权限以在子进程上执行 `execute` 或 `spawn` 方法。要授予此权限，请前往 `<项目根目录>/src-tauri/capabilities/default.json` 文件并将以下部分添加到 `permissions` 数组中。不要忘记根据之前提到的相对路径为你的 sidecar 命名。

src-tauri/capabilities/default.json

    {

      "permissions": [

        "core:default",

        {

          "identifier": "shell:allow-execute",

          "allow": [

            {

              "name": "binaries/app",

              "sidecar": true

            }

          ]

        }

      ]

    }

注意

使用 `shell:allow-execute` 标识符是因为 sidecar 的子进程将通过 `command.execute()` 方法启动。若要使用 `command.spawn()` 运行它，你需要将标识符更改为 `shell:allow-spawn`，或者在数组中添加另一个与上述结构相同但标识符设为 `shell:allow-spawn` 的条目。

在 JavaScript 代码中，从 `@tauri-apps/plugin-shell` 模块导入 `Command` 类并使用 `sidecar` 静态方法。

    import { Command } from '@tauri-apps/plugin-shell';

    const command = Command.sidecar('binaries/my-sidecar');

    const output = await command.execute();

注意

提供给 `Command.sidecar` 的字符串必须与 `externalBin` 配置数组中定义的字符串之一匹配。

## 传递参数

标题为“传递参数”的章节

你可以像运行普通 [Command](https://doc.rust-lang.net.cn/std/process/struct.Command.html) 一样，将参数传递给 Sidecar 命令。

参数可以是**静态的** （例如 `-o` 或 `serve`）或**动态的** （例如 `<file_path>` 或 `localhost:<PORT>`）。设为 `true` 将允许将任何参数传递给命令。`false` 将禁用所有参数。如果未设置 `true` 或 `false`，则需要按调用时的确切顺序定义参数。静态参数按原样定义，而动态参数可以使用正则表达式定义。

首先，在 `src-tauri/capabilities/default.json` 中定义需要传递给 sidecar 命令的参数：

src-tauri/capabilities/default.json

    {

      "$schema": "../gen/schemas/desktop-schema.json",

      "identifier": "default",

      "description": "Capability for the main window",

      "windows": ["main"],

      "permissions": [

        "core:default",

        {

          "identifier": "shell:allow-execute",

          "allow": [

            {

              "args": [

                "arg1",

                "-a",

                "--arg2",

                {

                  "validator": "\\S+"

                }

              ],

              "name": "binaries/my-sidecar",

              "sidecar": true

            }

          ]

        }

      ]

    }

注意

如果你是从 Tauri v1 迁移过来的，Tauri v2 CLI 中的 `migrate` 命令应该已经为你处理了这些。阅读 [自动迁移](/start/migrate/from-tauri-1/#automated-migration)了解更多信息。

然后，要调用 sidecar 命令，只需将**所有** 参数作为一个数组传入即可。

在 Rust 中

    use tauri_plugin_shell::ShellExt;

    #[tauri::command]

    async fn call_my_sidecar(app: tauri::AppHandle) {

      let sidecar_command = app

        .shell()

        .sidecar("my-sidecar")

        .unwrap()

        .args(["arg1", "-a", "--arg2", "any-string-that-matches-the-validator"]);

      let (mut _rx, mut _child) = sidecar_command.spawn().unwrap();

    }

在 JavaScript 中

    import { Command } from '@tauri-apps/plugin-shell';

    // notice that the args array matches EXACTLY what is specified in `capabilities/default.json`.

    const command = Command.sidecar('binaries/my-sidecar', [

      'arg1',

      '-a',

      '--arg2',

      'any-string-that-matches-the-validator',

    ]);

    const output = await command.execute();