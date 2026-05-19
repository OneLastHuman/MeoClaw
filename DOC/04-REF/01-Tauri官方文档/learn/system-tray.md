# 系统托盘

_Source: https://v2.tauri.org.cn/learn/system-tray/_

Tauri 允许您为应用程序创建和自定义系统托盘。这可以通过提供对常用操作的快速访问来增强用户体验。

## 配置

名为“配置”的章节

首先，更新您的 `Cargo.toml` 以包含系统托盘所需的特性（feature）。

src-tauri/Cargo.toml

    tauri = { version = "2.0.0", features = [ "tray-icon" ] }

## 用法

名为“用法”的章节

托盘 API 可在 JavaScript 和 Rust 中使用。

### 创建托盘图标

标题为“创建托盘图标”的章节

  * JavaScript
  * Rust

使用 [`TrayIcon.new`](/reference/javascript/api/namespacetray/#new) 静态函数来创建一个新的托盘图标

    import { TrayIcon } from '@tauri-apps/api/tray';

    const options = {

      // here you can add a tray menu, title, tooltip, event handler, etc

    };

    const tray = await TrayIcon.new(options);

有关自定义选项的更多信息，请参阅 [`TrayIconOptions`](/reference/javascript/api/namespacetray/#trayiconoptions)。

    use tauri::tray::TrayIconBuilder;

    tauri::Builder::default()

        .setup(|app| {

            let tray = TrayIconBuilder::new().build(app)?;

            Ok(())

        })

有关自定义选项的更多信息，请参阅 [`TrayIconBuilder`](https://docs.rs/tauri/2.0.0/tauri/tray/struct.TrayIconBuilder.html)。

### 更改托盘图标

标题为“更改托盘图标”的章节

在创建托盘时，您可以使用应用程序图标作为托盘图标

  * JavaScript
  * Rust

    import { TrayIcon } from '@tauri-apps/api/tray';

    import { defaultWindowIcon } from '@tauri-apps/api/app';

    const options = {

      icon: await defaultWindowIcon(),

    };

    const tray = await TrayIcon.new(options);

    let tray = TrayIconBuilder::new()

      .icon(app.default_window_icon().unwrap().clone())

      .build(app)?;

### 添加菜单

标题为“添加菜单”的章节

要附加一个在点击托盘时显示的菜单，可以使用 `menu` 选项。

注意

默认情况下，菜单会在左键和右键点击时显示。

若要防止菜单在左键点击时弹出，请调用 Rust 函数 [`menu_on_left_click(false)`](https://docs.rs/tauri/2.0.0/tauri/tray/struct.TrayIconBuilder.html#method.menu_on_left_click)，或者将 JavaScript 选项 [`menuOnLeftClick`](/reference/javascript/api/namespacetray/#properties-1) 设置为 `false`。

  * JavaScript
  * Rust

    import { TrayIcon } from '@tauri-apps/api/tray';

    import { Menu } from '@tauri-apps/api/menu';

    const menu = await Menu.new({

      items: [

        {

          id: 'quit',

          text: 'Quit',

        },

      ],

    });

    const options = {

      menu,

      menuOnLeftClick: true,

    };

    const tray = await TrayIcon.new(options);

    use tauri::{

      menu::{Menu, MenuItem},

      tray::TrayIconBuilder,

    };

    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&quit_i])?;

    let tray = TrayIconBuilder::new()

      .menu(&menu)

      .menu_on_left_click(true)

      .build(app)?;

#### 监听菜单事件

标题为“监听菜单事件”的章节

  * JavaScript
  * Rust

在 JavaScript 中，您可以直接将菜单点击事件监听器附加到菜单项上。

  * 使用共享菜单点击处理程序

        import { Menu } from '@tauri-apps/api/menu';

        function onTrayMenuClick(itemId) {

          // itemId === 'quit'

        }

        const menu = await Menu.new({

          items: [

            {

              id: 'quit',

              text: 'Quit',

              action: onTrayMenuClick,

            },

          ],

        });

  * 使用专用的菜单点击处理程序

        import { Menu } from '@tauri-apps/api/menu';

        const menu = await Menu.new({

          items: [

            {

              id: 'quit',

              text: 'Quit',

              action: () => {

                console.log('quit pressed');

              },

            },

          ],

        });

使用 [`TrayIconBuilder::on_menu_event`](https://docs.rs/tauri/2.0.0/tauri/tray/struct.TrayIconBuilder.html#method.on_menu_event) 方法来附加托盘菜单点击事件监听器。

    use tauri::tray::TrayIconBuilder;

    TrayIconBuilder::new()

      .on_menu_event(|app, event| match event.id.as_ref() {

        "quit" => {

          println!("quit menu item was clicked");

          app.exit(0);

        }

        _ => {

          println!("menu item {:?} not handled", event.id);

        }

      })

### 监听托盘事件

标题为“监听托盘事件”的章节

托盘图标会针对以下鼠标事件发出通知：

  * Click（点击）：当光标接收到单次左键、右键或中键点击时触发，包含有关是否释放了鼠标按键的信息。
  * Double click（双击）：当光标接收到双次左键、右键或中键点击时触发。
  * Enter（进入）：当光标进入托盘图标区域时触发。
  * Move（移动）：当光标在托盘图标区域内移动时触发。
  * Leave（离开）：当光标离开托盘图标区域时触发。

注意

Linux：不支持。即使图标显示且右键点击时仍会显示上下文菜单，也不会触发此事件。

  * JavaScript
  * Rust

    import { TrayIcon } from '@tauri-apps/api/tray';

    const options = {

      action: (event) => {

        switch (event.type) {

          case 'Click':

            console.log(

              `mouse ${event.button} button pressed, state: ${event.buttonState}`

            );

            break;

          case 'DoubleClick':

            console.log(`mouse ${event.button} button pressed`);

            break;

          case 'Enter':

            console.log(

              `mouse hovered tray at ${event.rect.position.x}, ${event.rect.position.y}`

            );

            break;

          case 'Move':

            console.log(

              `mouse moved on tray at ${event.rect.position.x}, ${event.rect.position.y}`

            );

            break;

          case 'Leave':

            console.log(

              `mouse left tray at ${event.rect.position.x}, ${event.rect.position.y}`

            );

            break;

        }

      },

    };

    const tray = await TrayIcon.new(options);

有关事件负载（payload）的更多信息，请参阅 [`TrayIconEvent`](/reference/javascript/api/namespacetray/#trayiconevent)。

    use tauri::{

        Manager,

        tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent}

    };

    TrayIconBuilder::new()

      .on_tray_icon_event(|tray, event| match event {

        TrayIconEvent::Click {

          button: MouseButton::Left,

          button_state: MouseButtonState::Up,

          ..

        } => {

          println!("left click pressed and released");

          // in this example, let's show and focus the main window when the tray is clicked

          let app = tray.app_handle();

          if let Some(window) = app.get_webview_window("main") {

            let _ = window.unminimize();

            let _ = window.show();

            let _ = window.set_focus();

          }

        }

        _ => {

          println!("unhandled event {event:?}");

        }

      })

有关事件类型的更多信息，请参阅 [`TrayIconEvent`](https://docs.rs/tauri/2.0.0/tauri/tray/enum.TrayIconEvent.html)。

有关创建菜单的详细信息（包括菜单项、子菜单和动态更新），请参阅 [窗口菜单 (Window Menu)](/learn/window-menu/) 文档。