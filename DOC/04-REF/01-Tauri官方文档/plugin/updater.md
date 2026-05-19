# 更新器

_Source: https://v2.tauri.org.cn/plugin/updater/_

[ GitHub](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/updater) [ npm ](https://npmjs.net.cn/package/@tauri-apps/plugin-updater) [ crates.io ](https://crates.io/crates/tauri-plugin-updater)

API 参考 [ ](/reference/javascript/updater/) [ ](https://docs.rs/tauri-plugin-updater)

通过更新服务器或静态 JSON 文件自动更新您的 Tauri 应用。

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

安装 Tauri 更新器插件即可开始。

  * 自动
  * 手动

使用你的项目包管理器添加依赖项

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri add updater

    yarn run tauri add updater

    pnpm tauri add updater

    deno task tauri add updater

    bun tauri add updater

    cargo tauri add updater

  1. 在 `src-tauri` 文件夹中运行以下命令，将插件添加到项目的 `Cargo.toml` 依赖项中

         cargo add tauri-plugin-updater --target 'cfg(any(target_os = "macos", windows, target_os = "linux"))'

  2. 修改 `lib.rs` 以初始化插件

lib.rs

         #[cfg_attr(mobile, tauri::mobile_entry_point)]

         pub fn run() {

             tauri::Builder::default()

                 .setup(|app| {

                     #[cfg(desktop)]

                     app.handle().plugin(tauri_plugin_updater::Builder::new().build());

                     Ok(())

                 })

                 .run(tauri::generate_context!())

                 .expect("error while running tauri application");

         }

  3. 您可以使用您偏好的 JavaScript 包管理器安装 JavaScript Guest 绑定

     * npm
     * yarn
     * pnpm
     * deno
     * bun

    npm install @tauri-apps/plugin-updater

    yarn add @tauri-apps/plugin-updater

    pnpm add @tauri-apps/plugin-updater

    deno add npm:@tauri-apps/plugin-updater

    bun add @tauri-apps/plugin-updater

## 签署更新

名为“签署更新”的章节

Tauri 的更新器需要签名来验证更新是否来自受信任的来源。此功能无法禁用。

要签署您的更新，您需要两把密钥

  1. 公钥：设置在 `tauri.conf.json` 中，用于在安装前验证构件。只要您的私钥安全，此公钥可以安全地上传和共享。
  2. 私钥：用于签署您的安装程序文件。您绝不应该与任何人共享此密钥。此外，如果您丢失了此密钥，您将无法向已经安装该应用的用户发布新的更新。请务必将此密钥存储在安全的地方！

为了生成密钥，Tauri CLI 提供了 `signer generate` 命令。您可以运行它在主目录中创建密钥。

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri signer generate -- -w ~/.tauri/myapp.key

    yarn tauri signer generate -w ~/.tauri/myapp.key

    pnpm tauri signer generate -w ~/.tauri/myapp.key

    deno task tauri signer generate -w ~/.tauri/myapp.key

    bunx tauri signer generate -w ~/.tauri/myapp.key

    cargo tauri signer generate -w ~/.tauri/myapp.key

### 构建

名为“构建”的章节

在构建更新构件时，您需要在环境变量中拥有上面生成的私钥。`.env` 文件是 _不起作用_ 的！

  * Mac/Linux
  * Windows

    export TAURI_SIGNING_PRIVATE_KEY="Path or content of your private key"

    # optionally also add a password

    export TAURI_SIGNING_PRIVATE_KEY_PASSWORD=""

在 `PowerShell` 中运行此命令

    $env:TAURI_SIGNING_PRIVATE_KEY="Path or content of your private key"

    <# optionally also add a password #>

    $env:TAURI_SIGNING_PRIVATE_KEY_PASSWORD=""

之后，您可以像往常一样运行 Tauri build，Tauri 将生成更新包及其签名。生成的文件取决于下面配置的 [`createUpdaterArtifacts`](/reference/config/#createupdaterartifacts) 配置值。

  * v2
  * v1 兼容

    {

      "bundle": {

        "createUpdaterArtifacts": true

      }

    }

在 Linux 上，Tauri 将在 `target/release/bundle/appimage/` 文件夹内创建一个普通的 AppImage

  * `myapp.AppImage` \- 标准应用包。它将被更新器重用。
  * `myapp.AppImage.sig` \- 更新包的签名。

在 macOS 上，Tauri 将在 target/release/bundle/macos/ 文件夹内从应用程序包创建一个 .tar.gz 压缩包

  * `myapp.app` \- 标准应用包。
  * `myapp.app.tar.gz` \- 更新包。
  * `myapp.app.tar.gz.sig` \- 更新包的签名。

在 Windows 上，Tauri 将在 target/release/bundle/msi/ 和 target/release/bundle/nsis 文件夹内创建普通的 MSI 和 NSIS 安装程序

  * `myapp-setup.exe` \- 标准应用包。它将被更新器重用。
  * `myapp-setup.exe.sig` \- 更新包的签名。
  * `myapp.msi` \- 标准应用包。它将被更新器重用。
  * `myapp.msi.sig` \- 更新包的签名。

    {

      "bundle": {

        "createUpdaterArtifacts": "v1Compatible"

      }

    }

在 Linux 上，Tauri 将在 `target/release/bundle/appimage/` 文件夹内从 AppImage 创建一个 .tar.gz 压缩包

  * `myapp.AppImage` \- 标准应用包。
  * `myapp.AppImage.tar.gz` \- 更新包。
  * `myapp.AppImage.tar.gz.sig` \- 更新包的签名。

在 macOS 上，Tauri 将在 target/release/bundle/macos/ 文件夹内从应用程序包创建一个 .tar.gz 压缩包

  * `myapp.app` \- 标准应用包。
  * `myapp.app.tar.gz` \- 更新包。
  * `myapp.app.tar.gz.sig` \- 更新包的签名。

在 Windows 上，Tauri 将在 target/release/bundle/msi/ 和 target/release/bundle/nsis 文件夹内从 MSI 和 NSIS 安装程序创建 .zip 压缩包

  * `myapp-setup.exe` \- 标准应用包。
  * `myapp-setup.nsis.zip` \- 更新包。
  * `myapp-setup.nsis.zip.sig` \- 更新包的签名。
  * `myapp.msi` \- 标准应用包。
  * `myapp.msi.zip` \- 更新包。
  * `myapp.msi.zip.sig` \- 更新包的签名。

## Tauri 配置

名为“Tauri 配置”的章节

按照此格式设置 `tauri.conf.json`，更新器即可开始工作。

密钥| 描述
---|---
`createUpdaterArtifacts`| 将其设置为 `true` 会告诉 Tauri 的应用打包器创建更新器构件。如果您正在将应用从旧版 Tauri 迁移，请将其设置为 `"v1Compatible"`。**此设置将在 v3 中移除** ，因此请确保在所有用户都迁移到 v2 后将其更改为 `true`。
`pubkey`| 这必须是上述步骤中从 Tauri CLI 生成的公钥。它**不能** 是文件路径！
`endpoints`| 这必须是一个以字符串形式表示的端点 URL 数组。生产模式强制要求 TLS。只有当返回非 2XX 状态码时，Tauri 才会继续尝试下一个 URL！
`dangerousInsecureTransportProtocol`| 将其设置为 `true` 允许更新器接受非 HTTPS 端点。请谨慎使用此配置！

每个更新器 URL 都可以包含以下动态变量，允许您在服务器端确定是否有可用更新。

  * `{{current_version}}`：请求更新的应用版本。
  * `{{target}}`：操作系统名称（`linux`、`windows` 或 `darwin` 之一）。
  * `{{arch}}`：机器架构（`x86_64`、`i686`、`aarch64` 或 `armv7` 之一）。

tauri.conf.json

    {

      "bundle": {

        "createUpdaterArtifacts": true

      },

      "plugins": {

        "updater": {

          "pubkey": "CONTENT FROM PUBLICKEY.PEM",

          "endpoints": [

            "https://releases.myapp.com/{{target}}/{{arch}}/{{current_version}}",

            // or a static github json file

            "https://github.com/user/repo/releases/latest/download/latest.json"

          ]

        }

      }

    }

提示

不支持自定义变量，但您可以定义一个 自定义 `{{target}}`。

### Windows 上的 `installMode`

名为“Windows 上的 installMode”的章节

在 Windows 上，有一个额外的可选 `"installMode"` 配置来改变更新的安装方式。

tauri.conf.json

    {

      "plugins": {

        "updater": {

          "windows": {

            "installMode": "passive"

          }

        }

      }

    }

  * `"passive"`：将显示一个带有进度条的小窗口。更新将在无需任何用户交互的情况下安装。通常推荐使用，也是默认模式。
  * `"basicUi"`：将显示一个基本的用户界面，需要用户交互才能完成安装。
  * `"quiet"`：不会向用户提供进度反馈。在此模式下，安装程序本身无法请求管理员权限，因此它仅适用于用户级安装，或者当您的应用本身已以管理员权限运行时。通常不推荐。

## 服务器支持

名为“服务器支持”的章节

更新器插件有两种使用方式。可以使用动态更新服务器，也可以使用静态 JSON 文件（用于 S3 或 GitHub Gists 等服务）。

### 静态 JSON 文件

名为“静态 JSON 文件”的章节

使用静态文件时，只需返回一个包含必要信息的 JSON 即可。

密钥| 描述
---|---
`version`| 必须是有效的 [SemVer](https://semver.org/)，可以有或没有前导 `v`，意味着 `1.0.0` 和 `v1.0.0` 都是有效的。
`notes`| 关于更新的备注。
`pub_date`| 如果提供，日期必须按照 [RFC 3339](https://datatracker.ietf.org/doc/html/rfc3339) 格式化。
`platforms`| 每个平台键采用 `OS-ARCH` 格式，其中 `OS` 为 `linux`、`darwin` 或 `windows` 之一，`ARCH` 为 `x86_64`、`aarch64`、`i686` 或 `armv7` 之一。
`signature`| 生成的 `.sig` 文件的内容，可能会随每次构建而变化。路径或 URL 是不起作用的！

注意

当使用 自定义目标 时，提供的目标字符串将与 `platforms` 键匹配，而不是默认的 `OS-ARCH` 值。

必需的键是 `"version"`、`"platforms.[target].url"` 和 `"platforms.[target].signature"`；其余为可选。

    {

      "version": "",

      "notes": "",

      "pub_date": "",

      "platforms": {

        "linux-x86_64": {

          "signature": "",

          "url": ""

        },

        "windows-x86_64": {

          "signature": "",

          "url": ""

        },

        "darwin-x86_64": {

          "signature": "",

          "url": ""

        }

      }

    }

注意，Tauri 会在检查版本字段之前验证整个文件，因此请确保所有现有的平台配置都是有效且完整的。

提示

[Tauri Action](https://github.com/tauri-apps/tauri-action) 可以为您生成静态 JSON 文件，以便在 GitHub Releases 等 CDN 上使用。

### 动态更新服务器

名为“动态更新服务器”的章节

使用动态更新服务器时，Tauri 将遵循服务器的指示。要禁用内部版本检查，您可以重写 [插件的版本比较逻辑](https://docs.rs/tauri-plugin-updater/latest/tauri_plugin_updater/struct.UpdaterBuilder.html#method.version_comparator)，这将安装服务器发送的版本（如果您需要回滚应用，这将非常有用）。

您的服务器可以使用上面定义的 `endpoint` URL 中的变量来确定是否需要更新。如果您需要更多数据，可以按需在 Rust 中包含额外的 [请求头](https://docs.rs/tauri-plugin-updater/latest/tauri_plugin_updater/struct.UpdaterBuilder.html#method.header)。

如果没有任何更新，您的服务器应返回状态码 [`204 No Content`](https://datatracker.ietf.org/doc/html/rfc2616#section-10.2.5)。

如果需要更新，您的服务器应返回状态码 [`200 OK`](http://tools.ietf.org/html/rfc2616#section-10.2.1) 以及此格式的 JSON 响应

密钥| 描述
---|---
`version`| 这必须是有效的 [SemVer](https://semver.org/)，可以有或没有前导 `v`，意味着 `1.0.0` 和 `v1.0.0` 都是有效的。
`notes`| 关于更新的备注。
`pub_date`| 如果提供，日期必须按照 [RFC 3339](https://datatracker.ietf.org/doc/html/rfc3339) 格式化。
`url`| 这必须是更新包的有效 URL。
`signature`| 生成的 `.sig` 文件的内容，可能会随每次构建而变化。路径或 URL 是不起作用的！

必需的键是 `"url"`、`"version"` 和 `"signature"`；其余为可选。

    {

      "version": "",

      "pub_date": "",

      "url": "",

      "signature": "",

      "notes": ""

    }

提示

CrabNebula 是 Tauri 的官方合作伙伴，提供动态更新服务器。欲了解更多信息，请参阅 [使用 CrabNebula Cloud 分发](/distribute/crabnebula-cloud/) 文档。

## 检查更新

名为“检查更新”的章节

默认的 API 用于检查和安装更新，利用已配置的端点，并可由 JavaScript 和 Rust 代码访问。

  * JavaScript
  * Rust

    import { check } from '@tauri-apps/plugin-updater';

    import { relaunch } from '@tauri-apps/plugin-process';

    const update = await check();

    if (update) {

      console.log(

        `found update ${update.version} from ${update.date} with notes ${update.body}`

      );

      let downloaded = 0;

      let contentLength = 0;

      // alternatively we could also call update.download() and update.install() separately

      await update.downloadAndInstall((event) => {

        switch (event.event) {

          case 'Started':

            contentLength = event.data.contentLength;

            console.log(`started downloading ${event.data.contentLength} bytes`);

            break;

          case 'Progress':

            downloaded += event.data.chunkLength;

            console.log(`downloaded ${downloaded} from ${contentLength}`);

            break;

          case 'Finished':

            console.log('download finished');

            break;

        }

      });

      console.log('update installed');

      await relaunch();

    }

欲了解更多信息，请参阅 [JavaScript API 文档](/reference/javascript/updater/)。

src-tauri/src/lib.rs

    use tauri_plugin_updater::UpdaterExt;

    pub fn run() {

      tauri::Builder::default()

        .setup(|app| {

          let handle = app.handle().clone();

          tauri::async_runtime::spawn(async move {

            update(handle).await.unwrap();

          });

          Ok(())

        })

        .run(tauri::generate_context!())

        .unwrap();

    }

    async fn update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {

      if let Some(update) = app.updater()?.check().await? {

        let mut downloaded = 0;

        // alternatively we could also call update.download() and update.install() separately

        update

          .download_and_install(

            |chunk_length, content_length| {

              downloaded += chunk_length;

              println!("downloaded {downloaded} from {content_length:?}");

            },

            || {

              println!("download finished");

            },

          )

          .await?;

        println!("update installed");

        app.restart();

      }

      Ok(())

    }

提示

若要向前端通知下载进度，请考虑使用带 [通道 (channel)](/develop/calling-frontend/#channels) 的命令。

更新器命令

    #[cfg(desktop)]

    mod app_updates {

        use std::sync::Mutex;

        use serde::Serialize;

        use tauri::{ipc::Channel, AppHandle, State};

        use tauri_plugin_updater::{Update, UpdaterExt};

        #[derive(Debug, thiserror::Error)]

        pub enum Error {

            #[error(transparent)]

            Updater(#[from] tauri_plugin_updater::Error),

            #[error("there is no pending update")]

            NoPendingUpdate,

        }

        impl Serialize for Error {

            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>

            where

                S: serde::Serializer,

            {

                serializer.serialize_str(self.to_string().as_str())

            }

        }

        type Result<T> = std::result::Result<T, Error>;

        #[derive(Clone, Serialize)]

        #[serde(tag = "event", content = "data")]

        pub enum DownloadEvent {

            #[serde(rename_all = "camelCase")]

            Started {

                content_length: Option<u64>,

            },

            #[serde(rename_all = "camelCase")]

            Progress {

                chunk_length: usize,

            },

            Finished,

        }

        #[derive(Serialize)]

        #[serde(rename_all = "camelCase")]

        pub struct UpdateMetadata {

            version: String,

            current_version: String,

        }

        #[tauri::command]

        pub async fn fetch_update(

            app: AppHandle,

            pending_update: State<'_, PendingUpdate>,

        ) -> Result<Option<UpdateMetadata>> {

            let channel = "stable";

            let url = url::Url::parse(&format!(

                "https://cdn.myupdater.com/{{{{target}}}}-{{{{arch}}}}/{{{{current_version}}}}?channel={channel}",

            )).expect("invalid URL");

          let update = app

              .updater_builder()

              .endpoints(vec![url])?

              .build()?

              .check()

              .await?;

          let update_metadata = update.as_ref().map(|update| UpdateMetadata {

              version: update.version.clone(),

              current_version: update.current_version.clone(),

          });

          *pending_update.0.lock().unwrap() = update;

          Ok(update_metadata)

        }

        #[tauri::command]

        pub async fn install_update(pending_update: State<'_, PendingUpdate>, on_event: Channel<DownloadEvent>) -> Result<()> {

            let Some(update) = pending_update.0.lock().unwrap().take() else {

                return Err(Error::NoPendingUpdate);

            };

            let started = false;

            update

                .download_and_install(

                    |chunk_length, content_length| {

                        if !started {

                            let _ = on_event.send(DownloadEvent::Started { content_length });

                            started = true;

                        }

                        let _ = on_event.send(DownloadEvent::Progress { chunk_length });

                    },

                    || {

                        let _ = on_event.send(DownloadEvent::Finished);

                    },

                )

                .await?;

            Ok(())

        }

        struct PendingUpdate(Mutex<Option<Update>>);

    }

    #[cfg_attr(mobile, tauri::mobile_entry_point)]

    pub fn run() {

        tauri::Builder::default()

            .plugin(tauri_plugin_process::init())

            .setup(|app| {

                #[cfg(desktop)]

                {

                    app.handle().plugin(tauri_plugin_updater::Builder::new().build());

                    app.manage(app_updates::PendingUpdate(Mutex::new(None)));

                }

                Ok(())

            })

            .invoke_handler(tauri::generate_handler![

                #[cfg(desktop)]

                app_updates::fetch_update,

                #[cfg(desktop)]

                app_updates::install_update

            ])

    }

欲了解更多信息，请参阅 [Rust API 文档](https://docs.rs/tauri-plugin-updater)。

注意，不需要在安装更新后立即重启应用。您可以选择如何处理更新，可以是等待用户手动重启应用，或者提示他们选择重启时间。

注意

在 Windows 上，由于 Windows 安装程序的限制，执行安装步骤时应用程序会自动退出。

在检查和下载更新时，可以定义自定义请求超时时间、代理和请求头。

  * JavaScript
  * Rust

    import { check } from '@tauri-apps/plugin-updater';

    const update = await check({

      proxy: '<proxy url>',

      timeout: 30000 /* milliseconds */,

      headers: {

        Authorization: 'Bearer <token>',

      },

    });

    use tauri_plugin_updater::UpdaterExt;

    let update = app

      .updater_builder()

      .timeout(std::time::Duration::from_secs(30))

      .proxy("<proxy-url>".parse().expect("invalid URL"))

      .header("Authorization", "Bearer <token>")

      .build()?

      .check()

      .await?;

### 运行时配置

名为“运行时配置”的章节

更新器 API 还允许在运行时配置更新器，以获得更大的灵活性。出于安全原因，某些 API 仅适用于 Rust。

#### 端点

名为“端点”的章节

在运行时设置应请求以检查更新的 URL 允许实现更动态的更新，例如独立发布通道。

    use tauri_plugin_updater::UpdaterExt;

    let channel = if beta { "beta" } else { "stable" };

    let update_url = format!("https://{channel}.myserver.com/{{{{target}}}}-{{{{arch}}}}/{{{{current_version}}}}");

    let update = app

      .updater_builder()

      .endpoints(vec![update_url])?

      .build()?

      .check()

      .await?;

提示

注意，当使用 format!() 插值更新 URL 时，您需要对变量进行双重转义，例如 `{{{{target}}}}`。

#### 公钥

名为“公钥”的章节

在运行时设置公钥对于实现密钥轮换逻辑非常有用。它可以通过插件构建器或更新器构建器进行设置。

    tauri_plugin_updater::Builder::new().pubkey("<your public key>").build()

    use tauri_plugin_updater::UpdaterExt;

    let update = app

      .updater_builder()

      .pubkey("<your public key>")

      .build()?

      .check()

      .await?;

#### 自定义目标

名为“自定义目标”的章节

默认情况下，更新器允许您使用 `{{target}}` 和 `{{arch}}` 变量来确定必须交付哪个更新资源。如果您需要关于更新的更多信息（例如在分发 macOS Universal 二进制选项或拥有更多构建类型时），您可以设置自定义目标。

  * JavaScript
  * Rust

    import { check } from '@tauri-apps/plugin-updater';

    const update = await check({

      target: 'macos-universal',

    });

自定义目标可以通过插件构建器或更新器构建器进行设置。

    tauri_plugin_updater::Builder::new().target("macos-universal").build()

    use tauri_plugin_updater::UpdaterExt;

    let update = app

      .updater_builder()

      .target("macos-universal")

      .build()?

      .check()

      .await?;

提示

默认的 `$target-$arch` 键可以使用 `tauri_plugin_updater::target()` 获取，它返回一个 `Option<String>`；当当前平台不支持更新器时，它为 `None`。

注意

  * 使用自定义目标时，可能仅将其用于确定更新平台会更容易，这样您就可以移除 `{{arch}}` 变量。
  * 所提供的目标值就是在使用 静态 JSON 文件 时与平台键匹配的键。

#### 允许降级

名为“允许降级”的章节

默认情况下，Tauri 会检查更新版本是否大于当前应用版本，以验证是否应该更新。要允许降级，您必须使用更新器构建器的 `version_comparator` API。

    use tauri_plugin_updater::UpdaterExt;

    let update = app

      .updater_builder()

      .version_comparator(|current, update| {

        // default comparison: `update.version > current`

        update.version != current

      })

      .build()?

      .check()

      .await?;

#### Windows 退出前钩子

名为“Windows 退出前钩子”的章节

由于 Windows 安装程序的限制，Tauri 会在 Windows 上安装更新前自动退出您的应用程序。要在发生这种情况之前执行操作，请使用 `on_before_exit` 函数。

    use tauri_plugin_updater::UpdaterExt;

    let update = app

      .updater_builder()

      .on_before_exit(|| {

        println!("app is about to exit on Windows!");

      })

      .build()?

      .check()

      .await?;

注意

如果构建器值未设置，则使用 配置 中的值作为回退。

## Permissions

名为“权限”的章节

默认情况下，所有潜在危险的插件命令和范围都被阻止，无法访问。您必须修改 `capabilities` 配置中的权限才能启用这些功能。

有关更多信息，请参阅[功能概述](/security/capabilities/)，并参阅[分步指南](/learn/security/using-plugin-permissions/)以使用插件权限。

src-tauri/capabilities/default.json

    {

      "permissions": [

        ...,

        "updater:default",

      ]

    }

## 默认权限

此权限集配置了哪些更新器功能会暴露给前端。

#### 已授予权限

启用了从检查更新到安装更新的完整工作流。

#### 此默认权限集包括以下内容

  * `allow-check`
  * `allow-download`
  * `allow-install`
  * `allow-download-and-install`

## 权限表

标识符 | 描述
---|---
`updater:allow-check` |  启用 check 命令，无需任何预配置的作用域。
`updater:deny-check` |  禁用 check 命令，无需任何预配置的作用域。
`updater:allow-download` |  启用 download 命令，无需任何预配置的作用域。
`updater:deny-download` |  禁用 download 命令，无需任何预配置的作用域。
`updater:allow-download-and-install` |  启用 download_and_install 命令，无需任何预配置的作用域。
`updater:deny-download-and-install` |  禁用 download_and_install 命令，无需任何预配置的作用域。
`updater:allow-install` |  启用 install 命令，无需任何预配置的作用域。
`updater:deny-install` |  禁用 install 命令，无需任何预配置的作用域。