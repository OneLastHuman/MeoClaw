# 更新依赖

_Source: https://v2.tauri.org.cn/develop/updating-dependencies/_

## 更新 npm 软件包

名为“更新 npm 软件包”的章节

如果您正在使用 `tauri` 软件包

  * npm
  * yarn
  * pnpm

    npm install @tauri-apps/cli@latest @tauri-apps/api@latest

    yarn up @tauri-apps/cli @tauri-apps/api

    pnpm update @tauri-apps/cli @tauri-apps/api --latest

您也可以在命令行中检测 Tauri 的最新版本，使用：

  * npm
  * yarn
  * pnpm

    npm outdated @tauri-apps/cli

    yarn outdated @tauri-apps/cli

    pnpm outdated @tauri-apps/cli

## 更新 Cargo 软件包

名为“更新 Cargo 软件包”的章节

您可以使用 [`cargo outdated`](https://github.com/kbknapp/cargo-outdated) 检查过时的软件包，也可以在 crates.io 页面上查看：[tauri](https://crates.io/crates/tauri/versions) / [tauri-build](https://crates.io/crates/tauri-build/versions)。

前往 `src-tauri/Cargo.toml` 并将 `tauri` 和 `tauri-build` 修改为

    [build-dependencies]

    tauri-build = "%version%"

    [dependencies]

    tauri = { version = "%version%" }

其中 `%version%` 是上述对应的版本号。

然后执行以下操作

终端窗口

    cd src-tauri

    cargo update

或者，您可以运行 [cargo-edit](https://github.com/killercup/cargo-edit) 提供的 `cargo upgrade` 命令，它会自动完成所有这些工作。

## 同步 npm 软件包和 Cargo Crates 版本

名为“同步 npm 软件包和 Cargo Crates 版本”的章节

由于 JavaScript API 依赖于后端的 Rust 代码，添加新功能需要同时升级这两端以确保兼容性。请确保您已同步 npm 软件包 `@tauri-apps/api` 和 Cargo crate `tauri` 的次要版本（minor version）。

对于插件，我们可能会在补丁版本（patch releases）中引入此类更改，因此我们会同时更新 npm 软件包和 Cargo crate 的版本。您需要保持版本精确同步，例如，您需要使用 npm 软件包 `@tauri-apps/plugin-fs` 和 Cargo crate `tauri-plugin-fs` 的相同版本（例如 `2.2.1`）。