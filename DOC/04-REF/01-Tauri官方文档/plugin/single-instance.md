# 单实例

_Source: https://v2.tauri.org.cn/plugin/single-instance/_

[ GitHub](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/single-instance) [ crates.io ](https://crates.io/crates/tauri-plugin-single-instance)

API 参考 [ ](https://docs.rs/tauri-plugin-single-instance)

使用单实例插件 (Single Instance Plugin) 确保你的 Tauri 应用同一时间仅运行一个实例。

## 支持的平台

标题为“支持的平台”的章节

_此插件需要 Rust 版本至少为 **1.77.2**_

平台 | 级别 | 备注
---|---|---
windows |  |
linux |  |
macos |  |
android |  |
ios |  |

## 设置

标题为“设置”的章节

安装单实例插件以开始使用。

  * 自动
  * 手动

使用你的项目包管理器添加依赖项

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri add single-instance

    yarn run tauri add single-instance

    pnpm tauri add single-instance

    deno task tauri add single-instance

    bun tauri add single-instance

    cargo tauri add single-instance

  1. 在 `src-tauri` 文件夹中运行以下命令，将插件添加到项目的 `Cargo.toml` 依赖项中

         cargo add tauri-plugin-single-instance --target 'cfg(any(target_os = "macos", windows, target_os = "linux"))'

  2. 修改 `lib.rs` 以初始化插件

lib.rs

         #[cfg_attr(mobile, tauri::mobile_entry_point)]

         pub fn run() {

             tauri::Builder::default()

                 .setup(|app| {

                     #[cfg(desktop)]

                     app.handle().plugin(tauri_plugin_single_instance::init(|app, args, cwd| {}));

                     Ok(())

                 })

                 .run(tauri::generate_context!())

                 .expect("error while running tauri application");

         }

注意

单实例插件必须被优先注册，这样才能正常工作。这可以确保它在其他插件干扰之前运行。

## 用法

名为“用法”的章节

该插件安装并初始化后，即可立即正常工作。不过，我们也可以使用 `init()` 方法来增强其功能。

插件的 `init()` 方法接收一个闭包，当新应用实例启动但被插件关闭时，该闭包会被调用。闭包有三个参数：

  1. **`app`:** 应用程序的 [AppHandle](https://docs.rs/tauri/2.0.0/tauri/struct.AppHandle.html)。
  2. **`args`:** 用户传递给新实例的参数列表。
  3. **`cwd`:** 当前工作目录 (Current Working Directory)，表示启动新应用实例的路径。

因此，闭包应该如下所示：

    .plugin(tauri_plugin_single_instance::init(|app, args, cwd| {

      // Write your code here...

    }))

### 聚焦新实例

名为“聚焦新实例”的章节

默认情况下，当应用已在运行时启动新实例，将不会执行任何操作。如果想要在用户尝试打开新实例时聚焦已运行的窗口，请按以下方式修改回调闭包：

src-tauri/src/lib.rs

    use tauri::{AppHandle, Manager};

    pub fn run() {

        let mut builder = tauri::Builder::default();

        #[cfg(desktop)]

        {

            builder = builder.plugin(tauri_plugin_single_instance::init(|app, args, cwd| {

                let _ = app.get_webview_window("main")

                           .expect("no main window")

                           .set_focus();

            }));

        }

        builder

            .run(tauri::generate_context!())

            .expect("error while running tauri application");

    }

## 在 Snap 和 Flatpak 中的用法

名为“在 Snap 和 Flatpak 中的用法”的章节

在 Linux 上，单实例插件使用 DBus 来确保只有一个实例在运行。它通过在第一个实例运行时向 DBus 发布服务来实现这一点。随后的实例会尝试发布相同的服务，如果服务已存在，它们会向该服务发送请求以通知第一个实例，然后立即退出。

尽管这在你的应用打包为 deb、rpm 或 AppImage 时运行良好，但对于 snap 或 flatpak 包，它默认无法按预期工作，因为这些包在受限的沙盒环境中运行，如果未在打包清单中明确声明，大多数与 DBus 服务的通信都会被阻止。

以下指南展示了如何为 snap 和 flatpak 包声明所需的权限以启用单实例功能。

### 获取应用 ID

名为“获取应用 ID”的章节

单实例插件将发布一个名为 `org.{id}.SingleInstance` 的服务。

`{id}` 将来自你 `tauri.conf.json` 文件中的 `identifier`，但其中的点 (`.`) 和连字符 (`-`) 会被替换为下划线 (`_`)。

例如，如果你的 identifier 是 `net.mydomain.MyApp`

  * `net_mydomain_MyApp` 将是你的应用 `{id}`
  * `org.net_mydomain_MyApp.SingleInstance` 将是你的应用 SingleInstance 服务名称

你需要该服务名称来授权你的应用在 snap 和 flatpak 清单中使用 DBus 服务，如下所示。

### Snap

名为“Snap”的章节

在你的 snapcraft.yml 文件中，为单实例服务声明一个 plug 和一个 slot，并在你的应用声明中使用它们

snapcraft.yml

    # ...

    slots:

      single-instance:

        interface: dbus

        bus: session

        name: org.net_mydomain_MyApp.SingleInstance # Remember to change net_mydomain_MyApp to your app ID

    plugs:

      single-instance-plug:

        interface: dbus

        bus: session

        name: org.net_mydomain_MyApp.SingleInstance # Remember to change net_mydomain_MyApp to your app ID

    # .....

    apps:

      my-app:

        # ...

        plugs:

          # ....

          - single-instance-plug

        slots:

          # ...

          - single-instance

        # ....

这将允许你的应用按预期发送和接收来自 DBus 服务的请求。

### Flatpak

名为“Flatpak”的章节

在你的 flatpak 清单文件（your.app.id.yml 或 your.app.id.json）中，使用服务名称声明 `--talk-name` 和 `--own-name` 完成参数 (finish args)

net.mydomain.MyApp.yml

    # ...

    finish-args:

      - --socket=wayland

      - --socket=fallback-x11

      - --device=dri

      - --share=ipc

      # ....

      - --talk-name=org.net_mydomain_MyApp.SingleInstance # Remember to change net_mydomain_MyApp to your app ID

      - --own-name=org.net_mydomain_MyApp.SingleInstance # Remember to change net_mydomain_MyApp to your app ID

    # ...

这将允许你的应用按预期发送和接收来自 DBus 服务的请求。

## Permissions

名为“权限”的章节

由于该插件目前没有 JavaScript API，你无需配置 [capabilities](/security/capabilities/) 即可使用它。