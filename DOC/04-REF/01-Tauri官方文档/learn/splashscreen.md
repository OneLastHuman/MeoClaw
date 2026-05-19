# 启动画面

_Source: https://v2.tauri.org.cn/learn/splashscreen/_

在本实验中，我们将实现一个基础的 Tauri 应用启动画面功能。实现过程非常直观：启动画面本质上就是一个新窗口，用于在应用程序执行繁重的初始化任务时显示某些内容，并在任务完成后将其关闭。

## 先决条件

标题为“前提条件”的章节

创建一个实验应用

如果你不是高级用户，**强烈建议**使用此处提供的选项和框架。这只是一个实验，完成后你可以删除该项目。

  * Bash
  * PowerShell
  * Fish
  * npm
  * Yarn
  * pnpm
  * deno
  * bun
  * Cargo

    sh <(curl https://create.tauri.app/sh)

    irm https://create.tauri.app/ps | iex

    sh (curl -sSL https://create.tauri.app/sh | psub)

    npm create tauri-app@latest

    yarn create tauri-app

    pnpm create tauri-app

    deno run -A npm:create-tauri-app

    bun create tauri-app

    cargo install create-tauri-app --locked

    cargo create-tauri-app

  * 项目名称: `splashscreen-lab`
  * 选择前端开发语言: `Typescript / Javascript`
  * 选择您的包管理器：`pnpm`
  * 选择您的 UI 模板：`Vanilla`
  * 选择 UI 风格: `Typescript`

## 步骤

名为“步骤”的章节

  1. ##### 安装依赖并运行项目

名为“安装依赖并运行项目”的章节

在开始开发任何项目之前，构建并运行初始模板非常重要，这可以验证你的环境设置是否按预期工作。

显示解决方案

         # Make sure you're in the right directory

         cd splashscreen-lab

         # Install dependencies

         pnpm install

         # Build and run the app

         pnpm tauri dev

![Successful run of the created template app.](/_astro/step_1._Ugp-O91_ZSTOi5.webp)

  2. ##### 在 `tauri.conf.json` 中注册新窗口

名为“在 tauri.conf.json 中注册新窗口”的章节

添加新窗口最简单的方法是直接在 `tauri.conf.json` 中进行配置。你也可以在启动时动态创建它们，但为了简单起见，我们直接注册即可。确保你有一个标签为 `main` 的窗口（创建时隐藏），以及一个标签为 `splashscreen` 的窗口（创建时直接显示）。你可以保持其他所有选项为默认值，或者根据个人喜好进行调整。

显示解决方案

src-tauri/tauri.conf.json

         {

             "windows": [

                 {

                     "label": "main",

                     "visible": false

                 },

                 {

                     "label": "splashscreen",

                     "url": "/splashscreen"

                 }

             ]

         }

  3. ##### 创建一个新页面作为启动画面

名为“创建一个新页面作为启动画面”的章节

在开始之前，你需要准备一些内容来展示。如何开发新页面取决于你选择的框架，大多数框架都有处理页面导航的“路由器”概念，在 Tauri 中应该能像平常一样工作，在这种情况下，你只需创建一个新的启动画面页面即可。或者，按照我们要做的，创建一个新的 `splashscreen.html` 文件来承载内容。

这里的关键点是，你可以导航到 `/splashscreen` URL 并显示你想要的启动画面内容。完成此步骤后，尝试再次运行该应用程序！

显示解决方案

/splashscreen.html

         <!doctype html>

         <html lang="en">

         <head>

             <meta charset="UTF-8" />

             <link rel="stylesheet" href="/src/styles.css" />

             <meta name="viewport" content="width=device-width, initial-scale=1.0" />

             <title>Tauri App</title>

         </head>

         <body>

             <div class="container">

                 <h1>Tauri used Splash!</h1>

                 <div class="row">

                     <h5>It was super effective!</h5>

                 </div>

             </div>

         </body>

         </html>

![The splashscreen we just created.](/_astro/step_3.Cjc3aTae_Z1m6TK7.webp)

  4. ##### 开始一些初始化任务

名为“开始一些初始化任务”的章节

由于启动画面通常用于隐藏繁重的初始化任务，我们来模拟一些耗时的任务，一部分在前端，一部分在后端。

为了在前端模拟耗时操作，我们将使用一个简单的 `setTimeout` 函数。

在后端模拟耗时操作最简单的方法是使用 Tokio crate，这是 Tauri 在后端用于提供异步运行时的 Rust crate。虽然 Tauri 提供了运行时，但有些工具并没有从该库中重新导出，因此我们需要将该 crate 添加到我们的项目中以使用它们。这在 Rust 生态系统中是一种非常常见的做法。

不要在异步函数中使用 `std::thread::sleep`，因为它们是在并发环境中协同运行的，而不是并行运行。这意味着如果你挂起了线程而不是 Tokio 任务，将会导致在该线程上调度运行的所有任务都无法执行，从而导致你的应用程序冻结。

显示解决方案

         # Run this command where the `Cargo.toml` file is

         cd src-tauri

         # Add the Tokio crate

         cargo add tokio -F time

         # Optionally go back to the top folder to keep developing

         # `tauri dev` can figure out where to run automatically

         cd ..

src/main.ts

         // These contents can be copy-pasted below the existing code, don't replace the entire file!!

         // Utility function to implement a sleep function in TypeScript

         function sleep(seconds: number): Promise<void> {

             return new Promise(resolve => setTimeout(resolve, seconds * 1000));

         }

         // Setup function

         async function setup() {

             // Fake perform some really heavy setup task

             console.log('Performing really heavy frontend setup task...')

             await sleep(3);

             console.log('Frontend setup task complete!')

             // Set the frontend task as being completed

             invoke('set_complete', {task: 'frontend'})

         }

         // Effectively a JavaScript main function

         window.addEventListener("DOMContentLoaded", () => {

             setup()

         });

/src-tauri/src/lib.rs

         // Import functionalities we'll be using

         use std::sync::Mutex;

         use tauri::async_runtime::spawn;

         use tauri::{AppHandle, Manager, State};

         use tokio::time::{sleep, Duration};

         // Create a struct we'll use to track the completion of

         // setup related tasks

         struct SetupState {

             frontend_task: bool,

             backend_task: bool,

         }

         // Our main entrypoint in a version 2 mobile compatible app

         #[cfg_attr(mobile, tauri::mobile_entry_point)]

         pub fn run() {

             // Don't write code before Tauri starts, write it in the

             // setup hook instead!

             tauri::Builder::default()

                 // Register a `State` to be managed by Tauri

                 // We need write access to it so we wrap it in a `Mutex`

                 .manage(Mutex::new(SetupState {

                     frontend_task: false,

                     backend_task: false,

                 }))

                 // Add a command we can use to check

                 .invoke_handler(tauri::generate_handler![greet, set_complete])

                 // Use the setup hook to execute setup related tasks

                 // Runs before the main loop, so no windows are yet created

                 .setup(|app| {

                     // Spawn setup as a non-blocking task so the windows can be

                     // created and ran while it executes

                     spawn(setup(app.handle().clone()));

                     // The hook expects an Ok result

                     Ok(())

                 })

                 // Run the app

                 .run(tauri::generate_context!())

                 .expect("error while running tauri application");

         }

         #[tauri::command]

         fn greet(name: String) -> String {

             format!("Hello {name} from Rust!")

         }

         // A custom task for setting the state of a setup task

         #[tauri::command]

         async fn set_complete(

             app: AppHandle,

             state: State<'_, Mutex<SetupState>>,

             task: String,

         ) -> Result<(), ()> {

             // Lock the state without write access

             let mut state_lock = state.lock().unwrap();

             match task.as_str() {

                 "frontend" => state_lock.frontend_task = true,

                 "backend" => state_lock.backend_task = true,

                 _ => panic!("invalid task completed!"),

             }

             // Check if both tasks are completed

             if state_lock.backend_task && state_lock.frontend_task {

                 // Setup is complete, we can close the splashscreen

                 // and unhide the main window!

                 let splash_window = app.get_webview_window("splashscreen").unwrap();

                 let main_window = app.get_webview_window("main").unwrap();

                 splash_window.close().unwrap();

                 main_window.show().unwrap();

             }

             Ok(())

         }

         // An async function that does some heavy setup task

         async fn setup(app: AppHandle) -> Result<(), ()> {

             // Fake performing some heavy action for 3 seconds

             println!("Performing really heavy backend setup task...");

             sleep(Duration::from_secs(3)).await;

             println!("Backend setup task completed!");

             // Set the backend task as being completed

             // Commands can be ran as regular functions as long as you take

             // care of the input arguments yourself

             set_complete(

                 app.clone(),

                 app.state::<Mutex<SetupState>>(),

                 "backend".to_string(),

             )

             .await?;

             Ok(())

         }

  5. ##### 运行应用程序

名为“运行应用程序”的章节

现在，你应该能看到启动画面窗口弹出，前端和后端将各自执行 3 秒的耗时任务，之后启动画面消失，主窗口显示出来！

## 讨论

名为“讨论”的章节

##### 是否应该使用启动画面？

名为“是否应该使用启动画面？”的章节

通常来说，使用启动画面是一种妥协，因为它承认你的应用程序加载速度不够快，以至于需要用启动画面来掩盖。事实上，更好的做法是直接进入主窗口，并在角落显示一个小型的加载动画，告知用户后台仍在进行初始化任务。

不过话虽如此，将其作为一种视觉风格选择也可以，或者你可能确实有某种特殊要求，使得在执行某些任务之前无法启动应用程序。使用启动画面绝非“错误”，只是它往往不是必要的，并且可能会让用户觉得你的应用程序优化得不够好。