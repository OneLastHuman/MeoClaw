# 移动端插件开发

_Source: https://v2.tauri.org.cn/develop/plugins/develop-mobile/_

插件开发

请确保您熟悉 [插件开发指南](/develop/plugins/) 中涵盖的概念，因为本指南中的许多概念都建立在这些基础之上。

插件可以运行使用 Kotlin（或 Java）和 Swift 编写的原生移动代码。默认插件模板包含一个使用 Kotlin 的 Android 库项目，以及一个包含示例移动命令的 Swift 包，展示了如何从 Rust 代码触发其执行。

## 初始化插件项目

标题为“初始化插件项目”的章节

按照 [插件开发指南](/develop/plugins/#initialize-plugin-project) 中的步骤初始化一个新的插件项目。

如果您已经拥有一个插件并希望为其添加 Android 或 iOS 功能，可以使用 `plugin android init` 和 `plugin ios init` 来引导移动端库项目，并指引您完成所需的更改。

默认插件模板将插件的实现分为两个独立的模块：`desktop.rs` 和 `mobile.rs`。

桌面端实现使用 Rust 代码来实现功能，而移动端实现则向原生移动代码发送消息以执行函数并获取结果。如果两种实现都需要共享逻辑，则可以在 `lib.rs` 中定义。

src/lib.rs

    use tauri::Runtime;

    impl<R: Runtime> <plugin-name><R> {

      pub fn do_something(&self) {

        // do something that is a shared implementation between desktop and mobile

      }

    }

这种实现简化了共享 API 的过程，使其既可用于命令，也可用于 Rust 代码。

### 开发 Android 插件

标题为“开发 Android 插件”的章节

Android 的 Tauri 插件定义为一个继承自 `app.tauri.plugin.Plugin` 并使用 `app.tauri.annotation.TauriPlugin` 注解的 Kotlin 类。每个使用 `app.tauri.annotation.Command` 注解的方法都可以被 Rust 或 JavaScript 调用。

Tauri 默认使用 Kotlin 进行 Android 插件实现，但如果您愿意，可以切换到 Java。生成插件后，在 Android Studio 中右键点击 Kotlin 插件类，然后从菜单中选择“Convert Kotlin file to Java file”（将 Kotlin 文件转换为 Java 文件）选项。Android Studio 将引导您完成项目迁移到 Java 的过程。

### 开发 iOS 插件

标题为“开发 iOS 插件”的章节

iOS 的 Tauri 插件定义为一个继承自 `Tauri` 包中 `Plugin` 类的 Swift 类。每个带有 `@objc` 属性和 `(_ invoke: Invoke)` 参数的函数（例如 `@objc private func download(_ invoke: Invoke) { }`）都可以被 Rust 或 JavaScript 调用。

该插件定义为一个 [Swift 包](https://swiftlang.cn/package-manager/)，以便您可以使用其包管理器来管理依赖项。

## 插件配置

标题为“插件配置”的章节

有关开发插件配置的更多详细信息，请参阅插件开发指南的 [插件配置章节](/develop/plugins/#plugin-configuration)。

移动端插件实例拥有一个用于获取插件配置的 getter 方法。

  * Android
  * iOS

    import android.app.Activity

    import android.webkit.WebView

    import app.tauri.annotation.TauriPlugin

    import app.tauri.annotation.InvokeArg

    @InvokeArg

    class Config {

        var timeout: Int? = 3000

    }

    @TauriPlugin

    class ExamplePlugin(private val activity: Activity): Plugin(activity) {

      private var timeout: Int? = 3000

      override fun load(webView: WebView) {

        getConfig(Config::class.java).let {

           this.timeout = it.timeout

        }

      }

    }

    struct Config: Decodable {

      let timeout: Int?

    }

    class ExamplePlugin: Plugin {

      var timeout: Int? = 3000

      @objc public override func load(webview: WKWebView) {

        do {

          let config = try parseConfig(Config.self)

          self.timeout = config.timeout

        } catch {}

      }

    }

## 生命周期事件

标题为“生命周期事件”的章节

插件可以挂钩到多个生命周期事件：

  * load：当插件被加载到 webview 时
  * onNewIntent：仅限 Android，当 Activity 被重新启动时

插件开发指南中还有其他的 [插件生命周期事件](/develop/plugins/#lifecycle-events)。

### load

标题为“load”的章节

  * **触发时机** ：当插件被加载到 webview 时
  * **用途** ：执行插件初始化代码

  * Android
  * iOS

    import android.app.Activity

    import android.webkit.WebView

    import app.tauri.annotation.TauriPlugin

    @TauriPlugin

    class ExamplePlugin(private val activity: Activity): Plugin(activity) {

      override fun load(webView: WebView) {

        // perform plugin setup here

      }

    }

    class ExamplePlugin: Plugin {

      @objc public override func load(webview: WKWebView) {

        let timeout = self.config["timeout"] as? Int ?? 30

      }

    }

### onNewIntent

标题为“onNewIntent”的章节

**注意** ：此事件仅在 Android 上可用。

  * **触发时机** ：当 Activity 被重新启动时。更多信息请参阅 [Activity#onNewIntent](https://developer.android.com.cn/reference/android/app/Activity#onNewIntent\(android.content.Intent\))。
  * **用途** ：处理应用程序重新启动的情况，例如点击通知或访问深层链接时。

    import android.app.Activity

    import android.content.Intent

    import app.tauri.annotation.TauriPlugin

    @TauriPlugin

    class ExamplePlugin(private val activity: Activity): Plugin(activity) {

      override fun onNewIntent(intent: Intent) {

        // handle new intent event

      }

    }

## 添加移动端命令

标题为“添加移动端命令”的章节

各自的移动端项目中有一个插件类，可以在其中定义可由 Rust 代码调用的命令。

  * Android
  * iOS

    import android.app.Activity

    import app.tauri.annotation.Command

    import app.tauri.annotation.TauriPlugin

    @TauriPlugin

    class ExamplePlugin(private val activity: Activity): Plugin(activity) {

      @Command

      fun openCamera(invoke: Invoke) {

        val ret = JSObject()

        ret.put("path", "/path/to/photo.jpg")

        invoke.resolve(ret)

      }

    }

如果您想使用 Kotlin 的 `suspend` 函数，则需要使用自定义的协程作用域。

    import android.app.Activity

    import app.tauri.annotation.Command

    import app.tauri.annotation.TauriPlugin

    // Change to Dispatchers.IO if it is intended for fetching data

    val scope = CoroutineScope(Dispatchers.Default + SupervisorJob())

    @TauriPlugin

    class ExamplePlugin(private val activity: Activity): Plugin(activity) {

      @Command

      fun openCamera(invoke: Invoke) {

        scope.launch {

          openCameraInner(invoke)

        }

      }

      private suspend fun openCameraInner(invoke: Invoke) {

        val ret = JSObject()

        ret.put("path", "/path/to/photo.jpg")

        invoke.resolve(ret)

      }

    }

注意

在 Android 上，原生命令是在主线程上调度的。执行耗时的操作会导致 UI 卡死，并可能引发“应用程序无响应”（ANR）错误。

如果您需要等待某些阻塞 IO，可以像这样启动一个协程：

    CoroutineScope(Dispatchers.IO).launch {

      val result = myLongRunningOperation()

      invoke.resolve(result)

    }

    class ExamplePlugin: Plugin {

      @objc public func openCamera(_ invoke: Invoke) throws {

        invoke.resolve(["path": "/path/to/photo.jpg"])

      }

    }

使用 [`tauri::plugin::PluginHandle`](https://docs.rs/tauri/2.0.0/tauri/plugin/struct.PluginHandle.html) 从 Rust 调用移动端命令。

    use std::path::PathBuf;

    use serde::{Deserialize, Serialize};

    use tauri::Runtime;

    #[derive(Serialize)]

    #[serde(rename_all = "camelCase")]

    pub struct CameraRequest {

      quality: usize,

      allow_edit: bool,

    }

    #[derive(Deserialize)]

    pub struct Photo {

      path: PathBuf,

    }

    impl<R: Runtime> <plugin-name;pascal-case><R> {

      pub fn open_camera(&self, payload: CameraRequest) -> crate::Result<Photo> {

        self

          .0

          .run_mobile_plugin("openCamera", payload)

          .map_err(Into::into)

      }

    }

## 命令参数

标题为“命令参数”的章节

参数被序列化为命令，并可以通过 `Invoke::parseArgs` 函数在移动端插件上进行解析，该函数接收一个描述参数对象的类。

### Android

标题为“Android”的章节

在 Android 上，参数被定义为一个使用 `@app.tauri.annotation.InvokeArg` 注解的类。内部对象也必须进行注解。

    import android.app.Activity

    import android.webkit.WebView

    import app.tauri.annotation.Command

    import app.tauri.annotation.InvokeArg

    import app.tauri.annotation.TauriPlugin

    @InvokeArg

    internal class OpenAppArgs {

      lateinit var name: String

      var timeout: Int? = null

    }

    @InvokeArg

    internal class OpenArgs {

      lateinit var requiredArg: String

      var allowEdit: Boolean = false

      var quality: Int = 100

      var app: OpenAppArgs? = null

    }

    @TauriPlugin

    class ExamplePlugin(private val activity: Activity): Plugin(activity) {

      @Command

      fun openCamera(invoke: Invoke) {

        val args = invoke.parseArgs(OpenArgs::class.java)

      }

    }

注意

可选参数定义为 `var <argumentName>: Type? = null`。

带有默认值的参数定义为 `var <argumentName>: Type = <default-value>`。

必需参数定义为 `lateinit var <argumentName>: Type`。

### iOS

名为“iOS”的章节

在 iOS 上，参数定义为一个继承自 `Decodable` 的类。内部对象也必须继承 Decodable 协议。

    class OpenAppArgs: Decodable {

      let name: String

      var timeout: Int?

    }

    class OpenArgs: Decodable {

      let requiredArg: String

      var allowEdit: Bool?

      var quality: UInt8?

      var app: OpenAppArgs?

    }

    class ExamplePlugin: Plugin {

      @objc public func openCamera(_ invoke: Invoke) throws {

        let args = try invoke.parseArgs(OpenArgs.self)

        invoke.resolve(["path": "/path/to/photo.jpg"])

      }

    }

注意

可选参数定义为 `var <argumentName>: Type?`。

**不支持** 带有默认值的参数。请使用可空类型，并在命令函数中设置默认值。

必需参数定义为 `let <argumentName>: Type`。

## 从移动端插件调用 Rust

标题为“从移动端插件调用 Rust”的章节

出于性能和可重用性的考虑，通常更倾向于使用 Rust 编写插件代码。虽然 Tauri 没有直接提供从插件代码调用 Rust 的机制，但在 Android 上使用 JNI 以及在 iOS 上使用 FFI，允许插件即使在应用程序 WebView 挂起时也能调用共享代码。

### Android

标题为“Android”的章节

在插件的 `Cargo.toml` 中，将 `jni` crate 添加为依赖项。

    [target.'cfg(target_os = "android")'.dependencies]

    jni = "0.21"

静态加载应用程序库并在 Kotlin 代码中定义原生函数。在此示例中，Kotlin 类为 `com.example.HelloWorld`，我们需要在 Rust 端引用完整的包名。

    private const val TAG = "MyPlugin"

    init {

      try {

        // Load the native library (libapp_lib.so)

        // This is the shared library built by Cargo with crate-type = ["cdylib"]

        System.loadLibrary("app_lib")

        Log.d(TAG, "Successfully loaded libapp_lib.so")

      } catch (e: UnsatisfiedLinkError) {

        Log.e(TAG, "Failed to load libapp_lib.so", e)

        throw e

      }

    }

    external fun helloWorld(name: String): String?

然后在插件的 Rust 代码中，定义 JNI 将查找的函数。函数格式为 `Java_package_class_method`，因此对于上面的类，它变为 `Java_com_example_HelloWorld_helloWorld`，以便被我们的 `helloWorld` 方法调用。

    #[cfg(target_os = "android")]

    #[no_mangle]

    pub extern "system" fn Java_com_example_HelloWorld_helloWorld(

        mut env: JNIEnv,

        _class: JClass,

        name: JString,

    ) -> jstring {

        log::debug!("Calling JNI Hello World!");

        let result = format!("Hello, {}!", name);

        match env.new_string(result) {

            Ok(jstr) => jstr.into_raw(),

            Err(e) => {

                log::error!("Failed to create JString: {}", e);

                std::ptr::null_mut()

            }

        }

    }

### iOS

名为“iOS”的章节

iOS 仅使用标准 C FFI，因此不需要任何新的依赖项。在您的 Swift 代码中添加 hook，以及任何必要的清理工作。这些函数可以命名为任何有效名称，但必须使用 `@_silgen_name(FFI_FUNC)` 进行注解，其中 `FFI_FUNC` 是要从 Rust 调用的函数名称。

    @_silgen_name("hello_world_ffi")

    private static func helloWorldFFI(_ name: UnsafePointer<CChar>) -> UnsafeMutablePointer<CChar>?

    @_silgen_name("free_hello_result_ffi")

    private static func freeHelloResult(_ result: UnsafeMutablePointer<CChar>)

    static func helloWorld(name: String) -> String? {

      // Call Rust FFI

      let resultPtr = name.withCString({ helloWorldFFI($0) })

      // Convert C string to Swift String

      let result = String(cString: resultPtr)

      // Free the C string

      freeHelloResult(resultPtr)

      return result

    }

然后，实现 Rust 端。此处的 `extern` 函数必须与 Swift 端的 `@_silgen_name` 注解相匹配。

    #[no_mangle]

    pub unsafe extern "C" fn hello_world_ffi(c_name: *const c_char) -> *mut c_char {

        let name = match CStr::from_ptr(c_name).to_str() {

            Ok(s) => s,

            Err(e) => {

                log::error!("[iOS FFI] Failed to convert C string: {}", e);

                return std::ptr::null_mut();

            }

        };

        let result = format!("Hello, {}!", name);

        match CString::new(result) {

            Ok(c_str) => c_str.into_raw(),

            Err(e) => {

                log::error!("[iOS FFI] Failed to create C string: {}", e);

                std::ptr::null_mut()

            }

        }

    }

    #[no_mangle]

    pub unsafe extern "C" fn free_hello_result_ffi(result: *mut c_char) {

        if !result.is_null() {

            drop(CString::from_raw(result));

        }

    }

## Android 16KB 内存页

标题为“Android 16KB 内存页”的章节

谷歌正在推动将 16KB 内存页作为所有新 Android 应用提交的要求。使用 NDK 28 或更高版本构建应自动生成符合此要求的包，但如果必须使用旧版 NDK 或生成的文件未进行 16KB 对齐，可以将以下内容添加到 `.cargo/config.toml`，以向 `rustc` 标记此需求：

    [target.aarch64-linux-android]

    rustflags = ["-C", "link-arg=-Wl,-z,max-page-size=16384"]

## Permissions

名为“权限”的章节

如果插件需要最终用户的权限，Tauri 会简化检查和请求权限的过程。

  * Android
  * iOS

首先定义所需的权限列表，并在代码中为每组权限定义一个别名。这是在 `TauriPlugin` 注解内部完成的。

    @TauriPlugin(

      permissions = [

        Permission(strings = [Manifest.permission.POST_NOTIFICATIONS], alias = "postNotification")

      ]

    )

    class ExamplePlugin(private val activity: Activity): Plugin(activity) { }

首先覆盖 `checkPermissions` 和 `requestPermissions` 函数。

    class ExamplePlugin: Plugin {

      @objc open func checkPermissions(_ invoke: Invoke) {

        invoke.resolve(["postNotification": "prompt"])

      }

      @objc public override func requestPermissions(_ invoke: Invoke) {

        // request permissions here

        // then resolve the request

        invoke.resolve(["postNotification": "granted"])

      }

    }

Tauri 会自动为该插件实现两个命令：`checkPermissions` 和 `requestPermissions`。这些命令可以直接从 JavaScript 或 Rust 调用。

  * JavaScript
  * Rust

    import { invoke, PermissionState } from '@tauri-apps/api/core'

    interface Permissions {

      postNotification: PermissionState

    }

    // check permission state

    const permission = await invoke<Permissions>('plugin:<plugin-name>|checkPermissions')

    if (permission.postNotification === 'prompt-with-rationale') {

      // show information to the user about why permission is needed

    }

    // request permission

    if (permission.postNotification.startsWith('prompt')) {

      const state = await invoke<Permissions>('plugin:<plugin-name>|requestPermissions', { permissions: ['postNotification'] })

    }

    use serde::{Serialize, Deserialize};

    use tauri::{plugin::PermissionState, Runtime};

    #[derive(Deserialize)]

    #[serde(rename_all = "camelCase")]

    struct PermissionResponse {

      pub post_notification: PermissionState,

    }

    #[derive(Serialize)]

    #[serde(rename_all = "camelCase")]

    struct RequestPermission {

      post_notification: bool,

    }

    impl<R: Runtime> Notification<R> {

      pub fn request_post_notification_permission(&self) -> crate::Result<PermissionState> {

        self.0

          .run_mobile_plugin::<PermissionResponse>("requestPermissions", RequestPermission { post_notification: true })

          .map(|r| r.post_notification)

          .map_err(Into::into)

      }

      pub fn check_permissions(&self) -> crate::Result<PermissionResponse> {

        self.0

          .run_mobile_plugin::<PermissionResponse>("checkPermissions", ())

          .map_err(Into::into)

      }

    }

## 插件事件

标题为“插件事件”的章节

插件可以使用 `trigger` 函数在任何时间点发出事件。

  * Android
  * iOS

    @TauriPlugin

    class ExamplePlugin(private val activity: Activity): Plugin(activity) {

        override fun load(webView: WebView) {

          trigger("load", JSObject())

        }

        override fun onNewIntent(intent: Intent) {

          // handle new intent event

          if (intent.action == Intent.ACTION_VIEW) {

            val data = intent.data.toString()

            val event = JSObject()

            event.put("data", data)

            trigger("newIntent", event)

          }

        }

        @Command

        fun openCamera(invoke: Invoke) {

          val payload = JSObject()

          payload.put("open", true)

          trigger("camera", payload)

        }

    }

    class ExamplePlugin: Plugin {

      @objc public override func load(webview: WKWebView) {

        trigger("load", data: [:])

      }

      @objc public func openCamera(_ invoke: Invoke) {

        trigger("camera", data: ["open": true])

      }

    }

然后，可以通过使用 [`addPluginListener`](/reference/javascript/api/namespacecore/#addpluginlistener) 辅助函数，从 NPM 包中调用辅助函数。

    import { addPluginListener, PluginListener } from '@tauri-apps/api/core';

    export async function onRequest(

      handler: (url: string) => void

    ): Promise<PluginListener> {

      return await addPluginListener(

        '<plugin-name>',

        'event-name',

        handler

      );

    }