# 窗口菜单

_Source: https://v2.tauri.org.cn/learn/window-menu/_

原生应用程序菜单可以附加到窗口或系统托盘上。仅适用于桌面端。

## 创建基础级菜单

标题为“创建基础级菜单”的章节

若要创建基础级原生窗口菜单并附加到窗口，你可以创建各种类型的菜单项，包括基本项、复选框项和分隔符。

  * JavaScript
  * Rust

使用 [`Menu.new`](https://v2.tauri.org.cn/reference/javascript/api/namespacemenu/#new-2) 静态函数来创建窗口菜单。

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

        {

          id: 'check_item',

          text: 'Check Item',

          checked: true,

        },

        {

          item: 'Separator',

        },

        {

          id: 'disabled_item',

          text: 'Disabled Item',

          enabled: false,

        },

        {

          id: 'status',

          text: 'Status: Processing...',

        },

      ],

    });

    // If a window was not created with an explicit menu or had one set explicitly,

    // this menu will be assigned to it.

    menu.setAsAppMenu().then(async (res) => {

      console.log('menu set success', res);

      // Update individual menu item text

      const statusItem = await menu.get('status');

      if (statusItem) {

        await statusItem.setText('Status: Ready');

      }

    });

    use tauri::menu::MenuBuilder;

    fn main() {

        tauri::Builder::default()

            .setup(|app| {

                let menu = MenuBuilder::new(app)

                    .text("open", "Open")

                    .text("close", "Close")

                    .check("check_item", "Check Item")

                    .separator()

                    .text("disabled_item", "Disabled Item")

                    .text("status", "Status: Processing...")

                    .build()?;

                app.set_menu(menu.clone())?;

                // Update individual menu item text

                menu

                    .get("status")

                    .unwrap()

                    .as_menuitem_unchecked()

                    .set_text("Status: Ready")?;

                Ok(())

            })

            .run(tauri::generate_context!());

    }

## 监听自定义菜单项事件

标题为“监听自定义菜单项事件”的章节

每个自定义菜单项在被点击时都会触发一个事件。请使用 `on_menu_event` API 来处理它们。

  * JavaScript
  * Rust

    import { Menu } from '@tauri-apps/api/menu';

    const menu = await Menu.new({

      items: [

        {

          id: 'Open',

          text: 'open',

          action: () => {

            console.log('open pressed');

          },

        },

        {

          id: 'Close',

          text: 'close',

          action: () => {

            console.log('close pressed');

          },

        },

      ],

    });

    await menu.setAsAppMenu();

    #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

    use tauri::menu::{MenuBuilder};

    fn main() {

      tauri::Builder::default()

            .setup(|app| {

                let menu = MenuBuilder::new(app)

                    .text("open", "Open")

                    .text("close", "Close")

                    .build()?;

                app.set_menu(menu)?;

                app.on_menu_event(move |app_handle: &tauri::AppHandle, event| {

                    println!("menu event: {:?}", event.id());

                    match event.id().0.as_str() {

                        "open" => {

                            println!("open event");

                        }

                        "close" => {

                            println!("close event");

                        }

                        _ => {

                            println!("unexpected menu event");

                        }

                    }

                });

                Ok(())

            })

    }

## 创建多级菜单

标题为“创建多级菜单”的章节

多级菜单允许你将菜单项归类到诸如“文件”、“编辑”等类别下。在 Windows 或 Linux 上，这些菜单将作为应用程序窗口的一部分出现；在 macOS 上，则会显示在菜单栏中。

**注意：** 在 macOS 上使用子菜单时，所有项目必须归类在某个子菜单下。顶级项目将被忽略。此外，无论 `text` 标签内容为何，第一个子菜单都会默认放置在应用程序的“关于”菜单下。你应该包含一个子菜单作为第一个条目（例如一个“关于”子菜单）来填充此位置。

注意

Tauri 2.8.0 版本起支持子菜单图标。

  * JavaScript
  * Rust

    import { Menu, MenuItem, Submenu } from '@tauri-apps/api/menu';

    // Will become the application submenu on MacOS

    const aboutSubmenu = await Submenu.new({

      text: 'About',

      items: [

        await MenuItem.new({

          id: 'quit',

          text: 'Quit',

          action: () => {

            console.log('Quit pressed');

          },

        }),

      ],

    });

    const fileSubmenu = await Submenu.new({

      text: 'File',

      icon: 'folder', // Optional: Add an icon to the submenu

      items: [

        await MenuItem.new({

          id: 'new',

          text: 'New',

          action: () => {

            console.log('New clicked');

          },

        }),

        await MenuItem.new({

          id: 'open',

          text: 'Open',

          action: () => {

            console.log('Open clicked');

          },

        }),

        await MenuItem.new({

          id: 'save_as',

          text: 'Save As...',

          action: () => {

            console.log('Save As clicked');

          },

        }),

      ],

    });

    const editSubmenu = await Submenu.new({

      text: 'Edit',

      items: [

        await MenuItem.new({

          id: 'undo',

          text: 'Undo',

          action: () => {

            console.log('Undo clicked');

          },

        }),

        await MenuItem.new({

          id: 'redo',

          text: 'Redo',

          action: () => {

            console.log('Redo clicked');

          },

        }),

      ],

    });

    const menu = await Menu.new({

      items: [aboutSubmenu, fileSubmenu, editSubmenu],

    });

    menu.setAsAppMenu();

    // You can also update the submenu icon dynamically

    fileSubmenu.setIcon('document');

    // Or set a native icon (only one type applies per platform)

    fileSubmenu.setNativeIcon('NSFolder');

    use tauri::{

        image::Image,

        menu::{CheckMenuItemBuilder, IconMenuItemBuilder, MenuBuilder, SubmenuBuilder},

    };

    fn main() {

        tauri::Builder::default()

            .setup(|app| {

                let menu_image = Image::from_bytes(include_bytes!("../icons/menu.png")).unwrap();

                let file_menu = SubmenuBuilder::new(app, "File")

                    .submenu_icon(menu_image) // Optional: Add an icon to the submenu

                    .text("open", "Open")

                    .text("quit", "Quit")

                    .build()?;

                let lang_str = "en";

                let check_sub_item_1 = CheckMenuItemBuilder::new("English")

                    .id("en")

                    .checked(lang_str == "en")

                    .build(app)?;

                let check_sub_item_2 = CheckMenuItemBuilder::new("Chinese")

                    .id("en")

                    .checked(lang_str == "en")

                    .enabled(false)

                    .build(app)?;

                // Load icon from path

                let icon_image = Image::from_bytes(include_bytes!("../icons/icon.png")).unwrap();

                let icon_item = IconMenuItemBuilder::new("icon")

                    .icon(icon_image)

                    .build(app)?;

                let other_item = SubmenuBuilder::new(app, "language")

                    .item(&check_sub_item_1)

                    .item(&check_sub_item_2)

                    .build()?;

                let menu = MenuBuilder::new(app)

                    .items(&[&file_menu, &other_item, &icon_item])

                    .build()?;

                app.set_menu(menu)?;

                let menu_image_update =

                    Image::from_bytes(include_bytes!("../icons/menu_update.png")).unwrap();

                // You can also update the submenu icon dynamically

                file_menu.set_icon(Some(menu_image_update))?;

                // Or set a native icon (only one type applies per platform)

                file_menu.set_native_icon(Some(tauri::menu::NativeIcon::Folder))?;

                Ok(())

            })

            .run(tauri::generate_context!());

    }

请注意，你需要启用 `image-ico` 或 `image-png` 特性才能使用此 API。

src-tauri/Cargo.toml

    [dependencies]

    tauri = { version = "...", features = ["...", "image-png"] }

## 创建预定义菜单

标题为“创建预定义菜单”的章节

若要使用由操作系统或 Tauri 预定义行为的内置（原生）菜单项。

  * JavaScript
  * Rust

    import { Menu, PredefinedMenuItem } from '@tauri-apps/api/menu';

    const copy = await PredefinedMenuItem.new({

      text: 'copy-text',

      item: 'Copy',

    });

    const separator = await PredefinedMenuItem.new({

      text: 'separator-text',

      item: 'Separator',

    });

    const undo = await PredefinedMenuItem.new({

      text: 'undo-text',

      item: 'Undo',

    });

    const redo = await PredefinedMenuItem.new({

      text: 'redo-text',

      item: 'Redo',

    });

    const cut = await PredefinedMenuItem.new({

      text: 'cut-text',

      item: 'Cut',

    });

    const paste = await PredefinedMenuItem.new({

      text: 'paste-text',

      item: 'Paste',

    });

    const select_all = await PredefinedMenuItem.new({

      text: 'select_all-text',

      item: 'SelectAll',

    });

    const menu = await Menu.new({

      items: [copy, separator, undo, redo, cut, paste, select_all],

    });

    await menu.setAsAppMenu();

    #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

    use tauri::menu::{MenuBuilder, PredefinedMenuItem};

    fn main() {

      tauri::Builder::default()

            .setup(|app| {

          let menu = MenuBuilder::new(app)

                    .copy()

                    .separator()

                    .undo()

                    .redo()

                    .cut()

                    .paste()

                    .select_all()

                    .item(&PredefinedMenuItem::copy(app, Some("custom text"))?)

                    .build()?;

                app.set_menu(menu)?;

                Ok(())

            })

    }

有关更多预设功能，请参考 [`PredefinedMenuItem`](https://docs.rs/tauri/latest/tauri/menu/struct.PredefinedMenuItem.html) 的文档。

提示

菜单构建器具有专门的方法来添加每个预定义菜单项，因此你可以调用 `.copy()` 而不必使用 `.item(&PredefinedMenuItem::copy(app, None)?)`。

## 更改菜单状态

标题为“更改菜单状态”的章节

如果你想更改菜单的状态（如文本、图标或复选状态），可以再次调用 `set_menu`。

  * JavaScript
  * Rust

    import {

      Menu,

      CheckMenuItem,

      IconMenuItem,

      MenuItem,

    } from '@tauri-apps/api/menu';

    import { Image } from '@tauri-apps/api/image';

    let currentLanguage = 'en';

    const check_sub_item_en = await CheckMenuItem.new({

      id: 'en',

      text: 'English',

      checked: currentLanguage === 'en',

      action: () => {

        currentLanguage = 'en';

        check_sub_item_en.setChecked(currentLanguage === 'en');

        check_sub_item_zh.setChecked(currentLanguage === 'cn');

        console.log('English pressed');

      },

    });

    const check_sub_item_zh = await CheckMenuItem.new({

      id: 'zh',

      text: 'Chinese',

      checked: currentLanguage === 'zh',

      action: () => {

        currentLanguage = 'zh';

        check_sub_item_en.setChecked(currentLanguage === 'en');

        check_sub_item_zh.setChecked(currentLanguage === 'zh');

        check_sub_item_zh.setAccelerator('Ctrl+L');

        console.log('Chinese pressed');

      },

    });

    // Load icon from path

    const icon = await Image.fromPath('../src/icon.png');

    const icon2 = await Image.fromPath('../src/icon-2.png');

    const icon_item = await IconMenuItem.new({

      id: 'icon_item',

      text: 'Icon Item',

      icon: icon,

      action: () => {

        icon_item.setIcon(icon2);

        console.log('icon pressed');

      },

    });

    const text_item = await MenuItem.new({

      id: 'text_item',

      text: 'Text Item',

      action: () => {

        text_item.setText('Text Item Changed');

        console.log('text pressed');

      },

    });

    const menu = await Menu.new({

      items: [

        {

          id: 'change menu',

          text: 'change_menu',

          items: [text_item, check_sub_item_en, check_sub_item_zh, icon_item],

        },

      ],

    });

    await menu.setAsAppMenu();

    // change-menu-status

    #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

    use tauri::{

        image::Image,

        menu::{CheckMenuItemBuilder, IconMenuItem, MenuBuilder, MenuItem, SubmenuBuilder},

    };

    fn main() {

        tauri::Builder::default()

            .setup(|app| {

                let check_sub_item_en = CheckMenuItemBuilder::with_id("en", "EN")

                    .checked(true)

                    .build(app)?;

                let check_sub_item_zh = CheckMenuItemBuilder::with_id("zh", "ZH")

                    .checked(false)

                    .build(app)?;

                let text_menu = MenuItem::with_id(

                    app,

                    "change_text",

                    &"Change menu".to_string(),

                    true,

                    Some("Ctrl+Z"),

                )

                .unwrap();

                let icon_menu = IconMenuItem::with_id(

                    app,

                    "change_icon",

                    &"Change icon menu",

                    true,

                    Some(Image::from_bytes(include_bytes!("../icons/icon.png")).unwrap()),

                    Some("Ctrl+F"),

                )

                .unwrap();

                let menu_item = SubmenuBuilder::new(app, "Change menu")

                    .item(&text_menu)

                    .item(&icon_menu)

                    .items(&[&check_sub_item_en, &check_sub_item_zh])

                    .build()?;

                let menu = MenuBuilder::new(app).items(&[&menu_item]).build()?;

                app.set_menu(menu)?;

                app.on_menu_event(move |_app_handle: &tauri::AppHandle, event| {

                    match event.id().0.as_str() {

                        "change_text" => {

                            text_menu

                                .set_text("changed menu text")

                                .expect("Change text error");

                            text_menu

                                .set_text("changed menu text")

                                .expect("Change text error");

                        }

                        "change_icon" => {

                            icon_menu

                                .set_text("changed menu-icon text")

                                .expect("Change text error");

                            icon_menu

                                .set_icon(Some(

                                    Image::from_bytes(include_bytes!("../icons/icon-2.png")).unwrap(),

                                ))

                                .expect("Change icon error");

                        }

                        "en" | "zh" => {

                            check_sub_item_en

                                .set_checked(event.id().0.as_str() == "en")

                                .expect("Change check error");

                            check_sub_item_zh

                                .set_checked(event.id().0.as_str() == "zh")

                                .expect("Change check error");

                            check_sub_item_zh.set_accelerator(Some("Ctrl+L"))

                            .expect("Change accelerator error");

                        }

                        _ => {

                            println!("unexpected menu event");

                        }

                    }

                });

                Ok(())

            })

            .run(tauri::generate_context!())

            .expect("error while running tauri application");

    }