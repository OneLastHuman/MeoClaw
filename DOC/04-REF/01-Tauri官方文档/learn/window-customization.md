# 窗口自定义

_Source: https://v2.tauri.org.cn/learn/window-customization/_

Tauri 提供了许多选项来定制应用程序窗口的外观和风格。你可以创建自定义标题栏、使用透明窗口、设置尺寸约束等等。

## 配置

名为“配置”的章节

有三种方法可以更改窗口配置

  * [通过 tauri.conf.json](/reference/config/#windowconfig)
  * [通过 JavaScript API](/reference/javascript/api/namespacewindow/#window)
  * [通过 Rust 代码中的 Window](https://docs.rs/tauri/2.0.0/tauri/window/struct.Window.html)

## 用法

名为“用法”的章节

  * 创建自定义标题栏
  * （macOS）透明标题栏与自定义窗口背景色

### 创建自定义标题栏

名为“创建自定义标题栏”的章节

这些窗口功能的一个常见用途是创建自定义标题栏。这个简短的教程将引导你完成这一过程。

注意

对于 macOS，使用自定义标题栏会失去一些系统提供的功能，例如[移动或对齐窗口](https://support.apple.com/guide/mac-help/work-with-app-windows-mchlp2469/mac)。另一种既能自定义标题栏又保留原生功能的方法是使标题栏透明并设置窗口背景色。请参阅用法（macOS）透明标题栏与自定义窗口背景色。

#### tauri.conf.json

名为“tauri.conf.json”的章节

在 `tauri.conf.json` 中将 `decorations` 设置为 `false`

tauri.conf.json

    "tauri": {

      "windows": [

        {

          "decorations": false

        }

      ]

    }

#### Permissions

名为“权限”的章节

在 capability 文件中添加窗口权限。

默认情况下，所有插件命令均被拦截且无法访问。你必须在 `capabilities` 配置中定义权限列表。

有关更多信息，请参阅[功能概述](/security/capabilities/)，并参阅[分步指南](/learn/security/using-plugin-permissions/)以使用插件权限。

src-tauri/capabilities/default.json

    {

      "$schema": "../gen/schemas/desktop-schema.json",

      "identifier": "main-capability",

      "description": "Capability for the main window",

      "windows": ["main"],

      "permissions": ["core:window:default", "core:window:allow-start-dragging"]

    }

权限| 描述
---|---
`core:window:default`| 插件的默认权限，`window:allow-start-dragging` 除外。
`core:window:allow-close`| 启用 close 命令，无需任何预先配置的范围。
`core:window:allow-minimize`| 启用 minimize 命令，无需任何预先配置的范围。
`core:window:allow-start-dragging`| 启用 start_dragging 命令，无需任何预先配置的范围。
`core:window:allow-toggle-maximize`| 启用 toggle_maximize 命令，无需任何预先配置的范围。
`core:window:allow-internal-toggle-maximize`| 启用 internal_toggle_maximize 命令，无需任何预先配置的范围。

#### CSS

名为“CSS”的章节

添加此 CSS 示例以将其保持在屏幕顶部并为按钮设置样式

    .titlebar {

      height: 30px;

      background: #329ea3;

      user-select: none;

      display: grid;

      grid-template-columns: auto max-content;

      position: fixed;

      top: 0;

      left: 0;

      right: 0;

    }

    .titlebar > .controls {

      display: flex;

    }

    .titlebar button {

      appearance: none;

      padding: 0;

      margin: 0;

      border: none;

      display: inline-flex;

      justify-content: center;

      align-items: center;

      width: 30px;

      background-color: transparent;

    }

    .titlebar button:hover {

      background: #5bbec3;

    }

#### HTML

名为“HTML”的章节

将其放在 `<body>` 标签的顶部

    <div class="titlebar">

      <div data-tauri-drag-region></div>

      <div class="controls">

        <button id="titlebar-minimize" title="minimize">

          <!-- https://api.iconify.design/mdi:window-minimize.svg -->

          <svg

            xmlns="http://www.w3.org/2000/svg"

            width="24"

            height="24"

            viewBox="0 0 24 24"

          >

            <path fill="currentColor" d="M19 13H5v-2h14z" />

          </svg>

        </button>

        <button id="titlebar-maximize" title="maximize">

          <!-- https://api.iconify.design/mdi:window-maximize.svg -->

          <svg

            xmlns="http://www.w3.org/2000/svg"

            width="24"

            height="24"

            viewBox="0 0 24 24"

          >

            <path fill="currentColor" d="M4 4h16v16H4zm2 4v10h12V8z" />

          </svg>

        </button>

        <button id="titlebar-close" title="close">

          <!-- https://api.iconify.design/mdi:close.svg -->

          <svg

            xmlns="http://www.w3.org/2000/svg"

            width="24"

            height="24"

            viewBox="0 0 24 24"

          >

            <path

              fill="currentColor"

              d="M13.46 12L19 17.54V19h-1.46L12 13.46L6.46 19H5v-1.46L10.54 12L5 6.46V5h1.46L12 10.54L17.54 5H19v1.46z"

            />

          </svg>

        </button>

      </div>

    </div>

请注意，你可能需要将其余内容向下移动，以免被标题栏覆盖。

提示

在 Windows 上，如果你只需要一个不需要自定义交互的标题栏，可以使用

    *[data-tauri-drag-region] {

      app-region: drag;

    }

来使标题栏支持触摸和触控笔输入

#### JavaScript

名为“JavaScript”的章节

使用此代码片段使按钮生效

    import { getCurrentWindow } from '@tauri-apps/api/window';

    // when using `"withGlobalTauri": true`, you may use

    // const { getCurrentWindow } = window.__TAURI__.window;

    const appWindow = getCurrentWindow();

    document

      .getElementById('titlebar-minimize')

      ?.addEventListener('click', () => appWindow.minimize());

    document

      .getElementById('titlebar-maximize')

      ?.addEventListener('click', () => appWindow.toggleMaximize());

    document

      .getElementById('titlebar-close')

      ?.addEventListener('click', () => appWindow.close());

请注意，如果你使用的是基于 Rust 的前端，则可以将上述代码复制到 `index.html` 文件的 `<script>` 元素中。

注意

`data-tauri-drag-region` 仅对直接应用它的元素有效。如果你希望拖拽行为也应用于子元素，你需要分别为每个子元素添加该属性。

保留此行为是为了让按钮和输入框等交互元素能够正常工作。

### 手动实现 `data-tauri-drag-region`

名为“手动实现 data-tauri-drag-region”的章节

对于自定义拖拽行为的使用场景，你可以手动添加一个带有 `window.startDragging` 的事件监听器，而不是使用 `data-tauri-drag-region`。

#### HTML

名为“HTML”的章节

基于上一节的代码，我们移除 `data-tauri-drag-region` 并添加一个 `id`

    <div data-tauri-drag-region class="titlebar">

      <div id="titlebar" class="titlebar">

        <!-- ... -->

      </div>

    </div>

#### JavaScript

名为“JavaScript”的章节

为标题栏元素添加事件监听器

    // ...

    document.getElementById('titlebar')?.addEventListener('mousedown', (e) => {

      if (e.buttons === 1) {

        // Primary (left) button

        e.detail === 2

          ? appWindow.toggleMaximize() // Maximize on double click

          : appWindow.startDragging(); // Else start dragging

      }

    });

### （macOS）透明标题栏与自定义窗口背景色

名为“（macOS）透明标题栏与自定义窗口背景色”的章节

我们将从 Rust 端创建主窗口并更改其背景颜色。

从 `tauri.conf.json` 文件中移除主窗口配置

tauri.conf.json

    "tauri": {

      "windows": [

        {

          "title": "Transparent Titlebar Window",

          "width": 800,

          "height": 600

        }

      ],

    }

将 `cocoa` crate 添加到依赖项中，以便我们可以使用它调用 macOS 原生 API

src-tauri/Cargo.toml

    [target."cfg(target_os = \"macos\")".dependencies]

    cocoa = "0.26"

创建主窗口并更改其背景颜色

src-tauri/src/lib.rs

    use tauri::{TitleBarStyle, WebviewUrl, WebviewWindowBuilder};

    pub fn run() {

      tauri::Builder::default()

        .setup(|app| {

          let win_builder =

            WebviewWindowBuilder::new(app, "main", WebviewUrl::default())

              .title("Transparent Titlebar Window")

              .inner_size(800.0, 600.0);

          // set transparent title bar only when building for macOS

          #[cfg(target_os = "macos")]

          let win_builder = win_builder.title_bar_style(TitleBarStyle::Transparent);

          let window = win_builder.build().unwrap();

          // set background color only when building for macOS

          #[cfg(target_os = "macos")]

          {

            use cocoa::appkit::{NSColor, NSWindow};

            use cocoa::base::{id, nil};

            let ns_window = window.ns_window().unwrap() as id;

            unsafe {

              let bg_color = NSColor::colorWithRed_green_blue_alpha_(

                  nil,

                  50.0 / 255.0,

                  158.0 / 255.0,

                  163.5 / 255.0,

                  1.0,

              );

              ns_window.setBackgroundColor_(bg_color);

            }

          }

          Ok(())

        })

        .run(tauri::generate_context!())

        .expect("error while running tauri application");

    }