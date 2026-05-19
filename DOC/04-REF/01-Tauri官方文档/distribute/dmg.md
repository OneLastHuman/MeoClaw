# DMG

_Source: https://v2.tauri.org.cn/distribute/dmg/_

DMG (Apple Disk Image) 格式是一种常见的 macOS 安装程序文件，它将您的 [应用程序包 (App Bundle)](/distribute/macos-application-bundle/) 封装在一个用户友好的安装窗口中。

安装窗口包含您的应用程序图标和“应用程序”文件夹图标。用户通常只需将应用程序图标拖动到“应用程序”文件夹图标上即可完成安装。这是 macOS 应用程序在 App Store 之外分发时最常见的安装方式。

本指南仅涵盖在 App Store 之外使用 DMG 格式分发应用程序的详细信息。有关 macOS 分发选项和配置的更多信息，请参阅 [应用程序包分发指南](/distribute/macos-application-bundle/)。若要通过 App Store 分发您的 macOS 应用程序，请参阅 [App Store 分发指南](/distribute/app-store/)。

要为您的应用程序创建 Apple 磁盘映像，您可以使用 Tauri CLI 并在 Mac 电脑上运行 `tauri build` 命令。

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri build -- --bundles dmg

    yarn tauri build --bundles dmg

    pnpm tauri build --bundles dmg

    deno task tauri build --bundles dmg

    bun tauri build --bundles dmg

    cargo tauri build --bundles dmg

![Standard DMG window](/_astro/standard-dmg-light.DwnO_utB_2qN4sD.webp) ![Standard DMG window](/_astro/standard-dmg-dark.DDFg0R9E_Z1Ofxfz.webp)

注意

macOS 和 Linux 上的 GUI 应用程序不会从您的 shell 配置文件（`.bashrc`、`.bash_profile`、`.zshrc` 等）中继承 `$PATH`。请查看 Tauri 的 [fix-path-env-rs](https://github.com/tauri-apps/fix-path-env-rs) crate 以修复此问题。

## 窗口背景

标题为“窗口背景”的章节

您可以使用 [`tauri.conf.json > bundle > macOS > dmg > background`] 配置选项为 DMG 安装窗口设置自定义背景图像。

tauri.conf.json

    {

      "bundle": {

        "macOS": {

          "dmg": {

            "background": "./images/"

          }

        }

      }

    }

例如，您的 DMG 背景图像可以包含一个箭头，以提示用户将其拖动到“应用程序”文件夹中。

## 窗口大小和位置

标题为“窗口大小和位置”的章节

默认窗口大小为 660x400。如果您需要不同的尺寸来适配自定义背景图像，请设置 [`tauri.conf.json > bundle > macOS > dmg > windowSize`] 配置。

tauri.conf.json

    {

      "bundle": {

        "macOS": {

          "dmg": {

            "windowSize": {

              "width": 800,

              "height": 600

            }

          }

        }

      }

    }

此外，您可以通过 [`tauri.conf.json > bundle > macOS > dmg > windowPosition`] 设置初始窗口位置。

tauri.conf.json

    {

      "bundle": {

        "macOS": {

          "dmg": {

            "windowPosition": {

              "x": 400,

              "y": 400

            }

          }

        }

      }

    }

## 图标位置

标题为“图标位置”的章节

您可以使用 [appPosition](/reference/config/#appposition) 和 [applicationFolderPosition](/reference/config/#applicationfolderposition) 配置值分别更改应用程序图标和“应用程序”文件夹图标的位置。

tauri.conf.json

    {

      "bundle": {

        "macOS": {

          "dmg": {

            "appPosition": {

              "x": 180,

              "y": 220

            },

            "applicationFolderPosition": {

              "x": 480,

              "y": 220

            }

          }

        }

      }

    }

注意

由于已知问题，在 CI/CD 平台上创建 DMG 时，图标大小和位置设置不会生效。更多信息请参阅 [tauri-apps/tauri#1731](https://github.com/tauri-apps/tauri/issues/1731)。