# 应用图标

_Source: https://v2.tauri.org.cn/develop/icons/_

Tauri 自带了一套基于其徽标的默认图标。这并不是你发布应用时想要使用的。为了解决这个常见问题，Tauri 提供了 `icon` 命令，它会读取一个输入文件（默认为 `"./app-icon.png"`）并创建各平台所需的所有图标。

关于文件类型的说明

  * `icon.icns` = macOS
  * `icon.ico` = Windows
  * `*.png` = Linux
  * `Square*Logo.png` 和 `StoreLogo.png` = 目前未使用，但适用于 AppX/MS Store 目标。

某些图标类型可能会在上述平台之外使用（尤其是 `png`）。因此，我们建议即使你打算只为部分平台构建，也应包含所有图标。

## 命令用法

标题为“命令用法”的章节

  * npm
  * yarn
  * pnpm
  * deno
  * cargo

    npm run tauri icon

    yarn tauri icon

    pnpm tauri icon

    deno task tauri icon

    cargo tauri icon

终端窗口

    > pnpm tauri icon --help

    Generate various icons for all major platforms

    Usage: pnpm run tauri icon [OPTIONS] [INPUT]

    Arguments:

      [INPUT]  Path to the source icon (squared PNG or SVG file with transparency) [default: ./app-icon.png]

    Options:

      -o, --output <OUTPUT>        Output directory. Default: 'icons' directory next to the tauri.conf.json file

      -v, --verbose...             Enables verbose logging

      -p, --png <PNG>              Custom PNG icon sizes to generate. When set, the default icons are not generated

          --ios-color <IOS_COLOR>  The background color of the iOS icon - string as defined in the W3C's CSS Color Module Level 4 <https://www.w3.org/TR/css-color-4/> [default: #fff]

      -h, --help                   Print help

      -V, --version                Print version

**桌面** 图标默认会放在你的 `src-tauri/icons` 文件夹中，并在构建应用时自动包含在内。如果你想从其他位置读取图标，可以编辑 `tauri.conf.json` 文件的相应部分。

    {

      "bundle": {

        "icon": [

          "icons/32x32.png",

          "icons/128x128.png",

          "icons/128x128@2x.png",

          "icons/icon.icns",

          "icons/icon.ico"

        ]

      }

    }

**移动端** 图标将直接放置到 Xcode 和 Android Studio 项目中！

## 手动创建图标

标题为“手动创建图标”的章节

如果你更喜欢自己构建这些图标（例如，为了在小尺寸下拥有更简洁的设计，或者因为不想依赖 CLI 内部的图像缩放功能），你必须确保图标符合某些要求：

  * `icon.icns`：[`icns`](https://en.wikipedia.org/wiki/Apple_Icon_Image_format) 文件所需的图层尺寸和名称在 [Tauri 仓库中](https://github.com/tauri-apps/tauri/blob/1.x/tooling/cli/src/helpers/icns.json)有详细描述。
  * `icon.ico`：[`ico`](https://en.wikipedia.org/wiki/ICO_\(file_format\)) 文件必须包含 16、24、32、48、64 和 256 像素的图层。为了在 _开发期间_ 获得最佳显示效果，32px 图层应作为第一图层。
  * `png`：PNG 图标的要求是：宽度 == 高度，RGBA（RGB + 透明度），每像素 32 位（每通道 8 位）。桌面上常用的预期尺寸为 32、128、256 和 512 像素。我们建议至少匹配 `tauri icon` 的输出：`32x32.png`、`128x128.png`、`128x128@2x.png` 和 `icon.png`。

### Android

标题为“Android”的章节

在 Android 上，你需要符合相同要求但尺寸不同的 PNG 图标。它们也需要直接放置在 Android Studio 项目中。

  * `src-tauri/gen/android/app/src/main/res/`
    * `mipmap-hdpi/`
      * `ic_launcher.png` 和 `ic_launcher_round.png`：49x49px
      * `ic_launcher_foreground.png`：162x162px
    * `mipmap-mdpi/`
      * `ic_launcher.png` 和 `ic_launcher_round.png`：48x48px
      * `ic_launcher_foreground.png`：108x108px
    * `mipmap-xhdpi/`
      * `ic_launcher.png` 和 `ic_launcher_round.png`：96x96px
      * `ic_launcher_foreground.png`：216x216px
    * `mipmap-xxhdpi/`
      * `ic_launcher.png` 和 `ic_launcher_round.png`：144x144px
      * `ic_launcher_foreground.png`：324x324px
    * `mipmap-xxxhdpi/`
      * `ic_launcher.png` 和 `ic_launcher_round.png`：192x192px
      * `ic_launcher_foreground.png`：432x432px

如果无法使用 `tauri icon`，我们建议查看 Android Studio 的 [Image Asset Studio](https://developer.android.com.cn/studio/write/create-app-icons)。

### iOS

名为“iOS”的章节

在 iOS 上，你需要符合相同要求但**不带透明度** 且尺寸不同的 PNG 图标。它们也需要直接放置在 Xcode 项目的 `src-tauri/gen/apple/Assets.xcassets/AppIcon.appiconset/` 中。预期需要以下图标：

  * 20px，包含 1x, 2x, 3x，以及一个额外图标
  * 29px，包含 1x, 2x, 3x，以及一个额外图标
  * 40px，包含 1x, 2x, 3x，以及一个额外图标
  * 60px，包含 2x, 3x
  * 76px，包含 1x, 2x
  * 83.5px，包含 2x
  * 512px，包含 2x，保存为 `AppIcon-512@2x.png`

文件名格式为 `AppIcon-{size}x{size}@{scaling}{extra}.png`。对于 20px 图标，这意味着你需要 20x20、40x40 和 60x60 尺寸的图标，命名为 `AppIcon-20x20@1x.png`、`AppIcon-20x20@2x.png`、`AppIcon-20x20@3x.png`，并将 `2x` 额外保存为 `AppIcon-20x20@2x-1.png`（“额外图标”）。