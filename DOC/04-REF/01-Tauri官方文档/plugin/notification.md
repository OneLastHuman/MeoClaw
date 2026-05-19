# 通知

_Source: https://v2.tauri.org.cn/plugin/notification/_

[ GitHub](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/notification) [ npm ](https://npmjs.net.cn/package/@tauri-apps/plugin-notification) [ crates.io ](https://crates.io/crates/tauri-plugin-notification)

API 参考 [ ](/reference/javascript/notification/) [ ](https://docs.rs/tauri-plugin-notification)

使用通知插件向您的用户发送原生通知。

## 支持的平台

标题为“支持的平台”的章节

_此插件需要 Rust 版本至少为 **1.77.2**_

平台 | 级别 | 备注
---|---|---
windows |  |  仅适用于已安装的应用。在开发阶段会显示 PowerShell 的名称和图标。
linux |  |
macos |  |
android |  |
ios |  |

## 设置

标题为“设置”的章节

安装通知插件以开始使用。

  * 自动
  * 手动

使用你的项目包管理器添加依赖项

  * npm
  * yarn
  * pnpm
  * deno
  * bun
  * cargo

    npm run tauri add notification

    yarn run tauri add notification

    pnpm tauri add notification

    deno task tauri add notification

    bun tauri add notification

    cargo tauri add notification

  1. 在 `src-tauri` 文件夹中运行以下命令，将插件添加到项目的 `Cargo.toml` 依赖项中

         cargo add tauri-plugin-notification

  2. 修改 `lib.rs` 以初始化插件

src-tauri/src/lib.rs

         #[cfg_attr(mobile, tauri::mobile_entry_point)]

         pub fn run() {

             tauri::Builder::default()

                 .plugin(tauri_plugin_notification::init())

                 .run(tauri::generate_context!())

                 .expect("error while running tauri application");

         }

  3. 如果您想在 JavaScript 中使用通知，请同时安装 npm 包。

     * npm
     * yarn
     * pnpm
     * deno
     * bun

    npm install @tauri-apps/plugin-notification

    yarn add @tauri-apps/plugin-notification

    pnpm add @tauri-apps/plugin-notification

    bun add npm:@tauri-apps/plugin-notification

    bun add @tauri-apps/plugin-notification

## 用法

名为“用法”的章节

以下是一些如何使用通知插件的示例。

  * 向用户发送通知
  * 向通知添加操作
  * 向通知添加附件
  * 在特定频道中发送通知

通知插件同时支持 JavaScript 和 Rust。

### 发送通知

名为“发送通知”的章节

按照以下步骤发送通知

  1. 检查是否已获得许可

  2. 若未获得许可则请求权限

  3. 发送通知

  * JavaScript
  * Rust

    import {

      isPermissionGranted,

      requestPermission,

      sendNotification,

    } from '@tauri-apps/plugin-notification';

    // when using `"withGlobalTauri": true`, you may use

    // const { isPermissionGranted, requestPermission, sendNotification, } = window.__TAURI__.notification;

    // Do you have permission to send a notification?

    let permissionGranted = await isPermissionGranted();

    // If not we need to request it

    if (!permissionGranted) {

      const permission = await requestPermission();

      permissionGranted = permission === 'granted';

    }

    // Once permission has been granted we can send the notification

    if (permissionGranted) {

      sendNotification({ title: 'Tauri', body: 'Tauri is awesome!' });

    }

    tauri::Builder::default()

        .plugin(tauri_plugin_notification::init())

        .setup(|app| {

            use tauri_plugin_notification::NotificationExt;

            app.notification()

                .builder()

                .title("Tauri")

                .body("Tauri is awesome")

                .show()

                .unwrap();

            Ok(())

        })

        .run(tauri::generate_context!())

        .expect("error while running tauri application");

### 操作

名为“操作”的章节

仅限移动端

操作 API 仅在移动平台上可用。

操作可为通知添加交互式按钮和输入框。使用它们可以为您的用户创建响应式体验。

#### 注册操作类型

名为“注册操作类型”的章节

注册操作类型以定义交互元素

    import { registerActionTypes } from '@tauri-apps/plugin-notification';

    await registerActionTypes([

      {

        id: 'messages',

        actions: [

          {

            id: 'reply',

            title: 'Reply',

            input: true,

            inputButtonTitle: 'Send',

            inputPlaceholder: 'Type your reply...',

          },

          {

            id: 'mark-read',

            title: 'Mark as Read',

            foreground: false,

          },

        ],

      },

    ]);

#### 操作属性

名为“操作属性”的章节

属性| 描述
---|---
`ID`| 操作的唯一标识符
`标题`| 操作按钮的显示文本
`requiresAuthentication`| 需要设备验证
`foreground`| 触发时将应用置于前台
`destructive`| 在 iOS 上以红色显示操作
`input`| 启用文本输入
`inputButtonTitle`| 输入提交按钮的文本
`inputPlaceholder`| 输入框的占位符文本

#### 监听操作

名为“监听操作”的章节

监听用户与通知操作的交互

    import { onAction } from '@tauri-apps/plugin-notification';

    await onAction((notification) => {

      console.log('Action performed:', notification);

    });

### 附件

名为“附件”的章节

附件可为通知添加媒体内容。支持情况因平台而异。

    import { sendNotification } from '@tauri-apps/plugin-notification';

    sendNotification({

      title: 'New Image',

      body: 'Check out this picture',

      attachments: [

        {

          id: 'image-1',

          url: 'asset:///notification-image.jpg',

        },

      ],

    });

#### 附件属性

名为“附件属性”的章节

属性| 描述
---|---
`ID`| 唯一标识符
`url`| 使用 asset:// 或 file:// 协议的内容 URL

注意：请在目标平台上测试附件以确保兼容性。

### 通道

标题为“通道 (Channels)”的章节

频道将通知按行为进行分类。虽然主要用于 Android，但它们在各平台上提供了一致的 API。

#### 创建频道

名为“创建频道”的章节

    import {

      createChannel,

      Importance,

      Visibility,

    } from '@tauri-apps/plugin-notification';

    await createChannel({

      id: 'messages',

      name: 'Messages',

      description: 'Notifications for new messages',

      importance: Importance.High,

      visibility: Visibility.Private,

      lights: true,

      lightColor: '#ff0000',

      vibration: true,

      sound: 'notification_sound',

    });

#### 频道属性

名为“频道属性”的章节

属性| 描述
---|---
`ID`| 唯一标识符
`名称 (name)`| 显示名称
`description`| 用途描述
`importance`| 优先级（无、最小、低、默认、高）
`visibility`| 隐私设置（秘密、私有、公共）
`lights`| 启用通知 LED（仅限 Android）
`lightColor`| LED 颜色（仅限 Android）
`vibration`| 启用震动
`sound`| 自定义声音文件名

#### 管理频道

名为“管理频道”的章节

列出现有频道

    import { channels } from '@tauri-apps/plugin-notification';

    const existingChannels = await channels();

移除频道

    import { removeChannel } from '@tauri-apps/plugin-notification';

    await removeChannel('messages');

#### 使用频道

名为“使用频道”的章节

使用频道发送通知

    import { sendNotification } from '@tauri-apps/plugin-notification';

    sendNotification({

      title: 'New Message',

      body: 'You have a new message',

      channelId: 'messages',

    });

注意：请先创建频道，再发送引用该频道的通知。无效的频道 ID 会导致通知无法显示。

## 安全注意事项

名为“安全注意事项”的章节

除了常规的用户输入清理流程外，目前没有已知的安全注意事项。

## 默认权限

此权限集用于配置默认暴露的通知功能。

#### 已授予权限

它允许使用所有与通知相关的功能。

#### 此默认权限集包括以下内容

  * `allow-is-permission-granted`
  * `allow-request-permission`
  * `allow-notify`
  * `allow-register-action-types`
  * `allow-register-listener`
  * `allow-cancel`
  * `allow-get-pending`
  * `allow-remove-active`
  * `allow-get-active`
  * `allow-check-permissions`
  * `allow-show`
  * `allow-batch`
  * `allow-list-channels`
  * `allow-delete-channel`
  * `allow-create-channel`
  * `allow-permission-state`

## 权限表

标识符 | 描述
---|---
`notification:allow-batch` |  在没有预配置范围的情况下启用 batch 命令。
`notification:deny-batch` |  在没有预配置范围的情况下禁用 batch 命令。
`notification:allow-cancel` |  在没有预配置范围的情况下启用 cancel 命令。
`notification:deny-cancel` |  在没有预配置范围的情况下禁用 cancel 命令。
`notification:allow-check-permissions` |  在没有任何预配置作用域的情况下启用 check_permissions 命令。
`notification:deny-check-permissions` |  在没有任何预配置作用域的情况下拒绝 check_permissions 命令。
`notification:allow-create-channel` |  在没有预配置范围的情况下启用 create_channel 命令。
`notification:deny-create-channel` |  在没有预配置范围的情况下禁用 create_channel 命令。
`notification:allow-delete-channel` |  在没有预配置范围的情况下启用 delete_channel 命令。
`notification:deny-delete-channel` |  在没有预配置范围的情况下禁用 delete_channel 命令。
`notification:allow-get-active` |  在没有预配置范围的情况下启用 get_active 命令。
`notification:deny-get-active` |  在没有预配置范围的情况下禁用 get_active 命令。
`notification:allow-get-pending` |  在没有预配置范围的情况下启用 get_pending 命令。
`notification:deny-get-pending` |  在没有预配置范围的情况下禁用 get_pending 命令。
`notification:allow-is-permission-granted` |  在没有预配置范围的情况下启用 is_permission_granted 命令。
`notification:deny-is-permission-granted` |  在没有预配置范围的情况下禁用 is_permission_granted 命令。
`notification:allow-list-channels` |  在没有预配置范围的情况下启用 list_channels 命令。
`notification:deny-list-channels` |  在没有预配置范围的情况下禁用 list_channels 命令。
`notification:allow-notify` |  在没有预配置范围的情况下启用 notify 命令。
`notification:deny-notify` |  在没有预配置范围的情况下禁用 notify 命令。
`notification:allow-permission-state` |  在没有预配置范围的情况下启用 permission_state 命令。
`notification:deny-permission-state` |  在没有预配置范围的情况下禁用 permission_state 命令。
`notification:allow-register-action-types` |  在没有预配置范围的情况下启用 register_action_types 命令。
`notification:deny-register-action-types` |  在没有预配置范围的情况下禁用 register_action_types 命令。
`notification:allow-register-listener` |  在没有预配置范围的情况下启用 register_listener 命令。
`notification:deny-register-listener` |  在没有预配置范围的情况下禁用 register_listener 命令。
`notification:allow-remove-active` |  在没有预配置范围的情况下启用 remove_active 命令。
`notification:deny-remove-active` |  在没有预配置范围的情况下禁用 remove_active 命令。
`notification:allow-request-permission` |  在没有预配置范围的情况下启用 request_permission 命令。
`notification:deny-request-permission` |  在没有预配置范围的情况下禁用 request_permission 命令。
`notification:allow-show` |  在没有预配置范围的情况下启用 show 命令。
`notification:deny-show` |  在没有预配置范围的情况下禁用 show 命令。