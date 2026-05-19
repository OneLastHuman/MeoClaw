# 在 Neovim 中调试

_Source: https://v2.tauri.org.cn/develop/debug/neovim/_

有许多插件可用于在 Neovim 中调试 Rust 代码。本指南将向您展示如何设置 `nvim-dap` 以及一些额外的插件来调试 Tauri 应用程序。

### 先决条件

标题为“前提条件”的章节

`nvim-dap` 扩展需要 `codelldb` 二进制文件。从 <https://github.com/vadimcn/codelldb/releases> 下载适用于您系统的版本并解压。我们稍后会在 `nvim-dap` 配置中指定其路径。

### 配置 nvim-dap

标题为“配置 nvim-dap”的章节

安装 [`nvim-dap`](https://github.com/mfussenegger/nvim-dap) 和 [`nvim-dap-ui`](https://github.com/rcarriga/nvim-dap-ui) 插件。请按照其 GitHub 页面提供的说明进行操作，或直接使用您喜欢的插件管理器。请注意，`nvim-dap-ui` 需要 `nvim-nio` 插件。

接下来，在您的 Neovim 配置中设置该插件

init.lua

    local dap = require("dap")

    dap.adapters.codelldb = {

      type = 'server',

      port = "${port}",

      executable = {

        -- Change this to your path!

        command = '/opt/codelldb/adapter/codelldb',

        args = {"--port", "${port}"},

      }

    }

    dap.configurations.rust= {

      {

        name = "Launch file",

        type = "codelldb",

        request = "launch",

        program = function()

          return vim.fn.input('Path to executable: ', vim.fn.getcwd() .. '/target/debug/', 'file')

        end,

        cwd = '${workspaceFolder}',

        stopOnEntry = false

      },

    }

此设置将在您每次启动调试器时，要求您指定想要调试的 Tauri 应用程序二进制文件路径。

（可选）您可以设置 `nvim-dap-ui` 插件，以便在每次调试会话开始和停止时自动切换调试器视图

init.lua

    local dapui = require("dapui")

    dapui.setup()

    dap.listeners.before.attach.dapui_config = function()

      dapui.open()

    end

    dap.listeners.before.launch.dapui_config = function()

      dapui.open()

    end

    dap.listeners.before.event_terminated.dapui_config = function()

      dapui.close()

    end

    dap.listeners.before.event_exited.dapui_config = function()

      dapui.close()

    end

最后，您可以更改编辑器中显示断点的默认方式

init.lua

    vim.fn.sign_define('DapBreakpoint',{ text ='🟥', texthl ='', linehl ='', numhl =''})

    vim.fn.sign_define('DapStopped',{ text ='▶️', texthl ='', linehl ='', numhl =''})

### 启动开发服务器

标题为“启动开发服务器”的章节

由于我们没有使用 Tauri CLI 来启动应用程序，因此开发服务器不会自动启动。要从 Neovim 控制开发服务器的状态，您可以使用 [overseer](https://github.com/stevearc/overseer.nvim/tree/master) 插件。

控制后台运行任务的最佳方式是使用 [VS Code 风格的任务](https://github.com/stevearc/overseer.nvim/blob/master/doc/guides.md#vs-code-tasks)配置。为此，请在项目目录中创建一个 `.vscode/tasks.json` 文件。

您可以在下方找到使用 `trunk` 的项目的任务配置示例。

.vscode/tasks.json

    {

      "version": "2.0.0",

      "tasks": [

        {

          "type": "process",

          "label": "dev server",

          "command": "trunk",

          "args": ["serve"],

          "isBackground": true,

          "presentation": {

            "revealProblems": "onProblem"

          },

          "problemMatcher": {

            "pattern": {

              "regexp": "^error:.*",

              "file": 1,

              "line": 2

            },

            "background": {

              "activeOnStart": false,

              "beginsPattern": ".*Rebuilding.*",

              "endsPattern": ".*server listening at:.*"

            }

          }

        }

      ]

    }

### 示例快捷键绑定

标题为“示例快捷键绑定”的章节

您可以在下方找到用于启动和控制调试会话的示例快捷键绑定。

init.lua

    vim.keymap.set('n', '<F5>', function() dap.continue() end)

    vim.keymap.set('n', '<F6>', function() dap.disconnect({ terminateDebuggee = true }) end)

    vim.keymap.set('n', '<F10>', function() dap.step_over() end)

    vim.keymap.set('n', '<F11>', function() dap.step_into() end)

    vim.keymap.set('n', '<F12>', function() dap.step_out() end)

    vim.keymap.set('n', '<Leader>b', function() dap.toggle_breakpoint() end)

    vim.keymap.set('n', '<Leader>o', function() overseer.toggle() end)

    vim.keymap.set('n', '<Leader>R', function() overseer.run_template() end)