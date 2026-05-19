# 在 VS Code 中调试

_Source: https://v2.tauri.org.cn/develop/debug/vscode/_

本指南将引导您设置 VS Code，以便调试 [Tauri 应用的核心进程](/concept/process-model/#the-core-process)。

## 所有平台使用 vscode-lldb 扩展

章节标题“所有平台使用 vscode-lldb 扩展”

### 先决条件

标题为“前提条件”的章节

安装 [`vscode-lldb`](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) 扩展。

### 配置 launch.json

章节标题“配置 launch.json”

创建一个 `.vscode/launch.json` 文件并将以下 JSON 内容粘贴进去

.vscode/launch.json

    {

      // Use IntelliSense to learn about possible attributes.

      // Hover to view descriptions of existing attributes.

      // For more information, visit: https://go.microsoft.com/fwlink/?linkid=830387

      "version": "0.2.0",

      "configurations": [

        {

          "type": "lldb",

          "request": "launch",

          "name": "Tauri Development Debug",

          "cargo": {

            "args": [

              "build",

              "--manifest-path=./src-tauri/Cargo.toml",

              "--no-default-features"

            ]

          },

          // task for the `beforeDevCommand` if used, must be configured in `.vscode/tasks.json`

          "preLaunchTask": "ui:dev"

        },

        {

          "type": "lldb",

          "request": "launch",

          "name": "Tauri Production Debug",

          "cargo": {

            "args": ["build", "--release", "--manifest-path=./src-tauri/Cargo.toml"]

          },

          // task for the `beforeBuildCommand` if used, must be configured in `.vscode/tasks.json`

          "preLaunchTask": "ui:build"

        }

      ]

    }

这直接使用 `cargo` 来构建 Rust 应用程序，并以开发和生产模式加载它。

请注意，它没有使用 Tauri CLI，因此不会执行特定的 CLI 功能。`beforeDevCommand` 和 `beforeBuildCommand` 脚本必须提前执行，或配置为 `preLaunchTask` 字段中的任务。以下是一个示例 `.vscode/tasks.json` 文件，其中包含两个任务：一个用于启动开发服务器的 `beforeDevCommand`，另一个用于 `beforeBuildCommand`

.vscode/tasks.json

    {

      // See https://go.microsoft.com/fwlink/?LinkId=733558

      // for the documentation about the tasks.json format

      "version": "2.0.0",

      "tasks": [

        {

          "label": "ui:dev",

          "type": "shell",

          // `dev` keeps running in the background

          // ideally you should also configure a `problemMatcher`

          // see https://vscode.js.cn/docs/editor/tasks#_can-a-background-task-be-used-as-a-prelaunchtask-in-launchjson

          "isBackground": true,

          // change this to your `beforeDevCommand`:

          "command": "yarn",

          "args": ["dev"]

        },

        {

          "label": "ui:build",

          "type": "shell",

          // change this to your `beforeBuildCommand`:

          "command": "yarn",

          "args": ["build"]

        }

      ]

    }

现在，您可以在 `src-tauri/src/main.rs` 或任何其他 Rust 文件中设置断点，并按 `F5` 开始调试。

## 在 Windows 上使用 Visual Studio Windows 调试器

章节标题“在 Windows 上使用 Visual Studio Windows 调试器”

Visual Studio Windows 调试器是一个仅限 Windows 的调试器，它通常比 [`vscode-lldb`](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) 更快，并对某些 Rust 功能（如枚举）有更好的支持。

### 先决条件

标题为“前提条件”的章节

安装 [C/C++](https://marketplace.visualstudio.com/items?itemName=ms-vscode.cpptools) 扩展，并按照 <https://vscode.js.cn/docs/cpp/config-msvc#_prerequisites> 安装 Visual Studio Windows 调试器。

### 配置 launch.json 和 tasks.json

章节标题“配置 launch.json 和 tasks.json”

.vscode/launch.json

    {

      // Use IntelliSense to learn about possible attributes.

      // Hover to view descriptions of existing attributes.

      // For more information, visit: https://go.microsoft.com/fwlink/?linkid=830387

      "version": "0.2.0",

      "configurations": [

        {

          "name": "Launch App Debug",

          "type": "cppvsdbg",

          "request": "launch",

          // change the exe name to your actual exe name

          // (to debug release builds, change `target/debug` to `release/debug`)

          "program": "${workspaceRoot}/src-tauri/target/debug/your-app-name-here.exe",

          "cwd": "${workspaceRoot}",

          "preLaunchTask": "ui:dev"

        }

      ]

    }

请注意，它没有使用 Tauri CLI，因此不会执行特定的 CLI 功能。`tasks.json` 与 `lldb` 的使用方法相同，除了您需要添加一个配置组，并在 `launch.json` 中将您的 `preLaunchTask` 指向它，以便在启动前始终进行编译。

以下是一个将运行开发服务器（相当于 `beforeDevCommand`）和编译（`cargo build`）作为一个组运行的示例。要使用它，请将 `launch.json` 中的 `preLaunchTask` 配置更改为 `dev`（或您为该组指定的任何名称）。

.vscode/tasks.json

    {

      // See https://go.microsoft.com/fwlink/?LinkId=733558

      // for the documentation about the tasks.json format

      "version": "2.0.0",

      "tasks": [

        {

          "label": "build:debug",

          "type": "cargo",

          "command": "build",

          "options": {

            "cwd": "${workspaceRoot}/src-tauri"

          }

        },

        {

          "label": "ui:dev",

          "type": "shell",

          // `dev` keeps running in the background

          // ideally you should also configure a `problemMatcher`

          // see https://vscode.js.cn/docs/editor/tasks#_can-a-background-task-be-used-as-a-prelaunchtask-in-launchjson

          "isBackground": true,

          // change this to your `beforeDevCommand`:

          "command": "yarn",

          "args": ["dev"]

        },

        {

          "label": "dev",

          "dependsOn": ["build:debug", "ui:dev"],

          "group": {

            "kind": "build"

          }

        }

      ]

    }