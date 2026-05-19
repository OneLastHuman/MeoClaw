# 应用大小

_Source: https://v2.tauri.org.cn/concept/size/_

虽然 Tauri 默认提供的二进制文件已经非常精简，但挑战极限也未尝不可。以下是一些实现最优构建结果的技巧和建议。

## Cargo 配置

名为“Cargo 配置”的章节

对项目进行的最简单且与前端框架无关的体积优化方法之一，就是为其添加一个 Cargo 配置文件。

根据你使用的是 Rust 稳定版（stable）还是夜间版（nightly）工具链，可用的选项会略有不同。除非你是高级用户，否则建议保持使用稳定版工具链。

  * 稳定版 (Stable)
  * 夜间版 (Nightly)

src-tauri/Cargo.toml

    [profile.dev]

    incremental = true # Compile your binary in smaller steps.

    [profile.release]

    codegen-units = 1 # Allows LLVM to perform better optimization.

    lto = true # Enables link-time-optimizations.

    opt-level = "s" # Prioritizes small binary size. Use `3` if you prefer speed.

    panic = "abort" # Higher performance by disabling panic handlers.

    strip = true # Ensures debug symbols are removed.

src-tauri/Cargo.toml

    [profile.dev]

    incremental = true # Compile your binary in smaller steps.

    rustflags = ["-Zthreads=8"] # Better compile performance.

    [profile.release]

    codegen-units = 1 # Allows LLVM to perform better optimization.

    lto = true # Enables link-time-optimizations.

    opt-level = "s" # Prioritizes small binary size. Use `3` if you prefer speed.

    panic = "abort" # Higher performance by disabling panic handlers.

    strip = true # Ensures debug symbols are removed.

    trim-paths = "all" # Removes potentially privileged information from your binaries.

    rustflags = ["-Cdebuginfo=0", "-Zthreads=8"] # Better compile performance.

### 参考

标题为“参考”的章节

注意

这并不是所有可用选项的完整参考，仅列出了我们希望你额外关注的内容。

  * [incremental:](https://doc.rust-lang.net.cn/cargo/reference/profiles.html#incremental) 分步编译二进制文件。
  * [codegen-units:](https://doc.rust-lang.net.cn/cargo/reference/profiles.html#codegen-units) 加快编译速度，但会牺牲一定的编译时优化。
  * [lto:](https://doc.rust-lang.net.cn/cargo/reference/profiles.html#lto) 启用链接时优化。
  * [opt-level:](https://doc.rust-lang.net.cn/cargo/reference/profiles.html#opt-level) 决定编译器的优化重心。使用 `3` 来优化性能，使用 `z` 来优化体积，使用 `s` 则介于两者之间。
  * [panic:](https://doc.rust-lang.net.cn/cargo/reference/profiles.html#panic) 通过移除 panic 展开（unwinding）来减小体积。
  * [strip:](https://doc.rust-lang.net.cn/cargo/reference/profiles.html#strip) 从二进制文件中剥离符号或调试信息。
  * [rpath:](https://doc.rust-lang.net.cn/cargo/reference/profiles.html#rpath) 通过在二进制文件中硬编码信息，协助查找二进制文件所需的动态库。
  * [trim-paths:](https://rust-lang.github.io/rfcs/3127-trim-paths.html) 从二进制文件中移除潜在的特权信息。
  * [rustflags:](https://doc.rust-lang.net.cn/nightly/cargo/reference/unstable.html#profile-rustflags-option) 按配置文件设置 Rust 编译器标志。
    * `-Cdebuginfo=0`: 是否在构建中包含调试信息符号。
    * `-Zthreads=8`: 增加编译过程中使用的线程数。

## 移除未使用的命令

名为“移除未使用的命令”的章节

在 Pull Request [`feat: add a new option to remove unused commands`](https://github.com/tauri-apps/tauri/pull/12890) 中，我们在 Tauri 配置文件中增加了一个新选项

tauri.conf.json

    {

      "build": {

        "removeUnusedCommands": true

      }

    }

用于移除在能力文件（ACL）中从未被允许的命令，这样你就不必为未使用的功能付费。

提示

为了最大化此功能的效果，请仅在 ACL 中包含你实际使用的命令，而不是使用 `default`s。

注意

此功能需要 `tauri@2.4`、`tauri-build@2.1`、`tauri-plugin@2.1` 和 `tauri-cli@2.4`。

注意

这不会考虑在运行时动态添加的 ACL，因此在使用此功能时请务必注意。

其底层工作原理是什么？

`tauri-cli` 会通过环境变量与 `tauri-build` 以及 `tauri`、`tauri-plugin` 的构建脚本进行通信，让它们从 ACL 中生成一份允许命令的列表，随后 `generate_handler` 宏将使用该列表来移除未使用的命令。

内部细节是，该环境变量目前名为 `REMOVE_UNUSED_COMMANDS`，它被设置为项目的目录（通常是 `src-tauri` 目录），构建脚本会利用它来查找能力文件。虽然不建议这样做，但如果你无法或不想使用 `tauri-cli` 来实现此功能，你依然可以手动设置该环境变量（**请注意，由于这是实现细节，我们不保证其稳定性** ）。