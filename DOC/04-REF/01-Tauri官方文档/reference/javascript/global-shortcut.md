# @tauri-apps/plugin-global-shortcut

_Source: https://v2.tauri.org.cn/reference/javascript/global-shortcut/_

注册全局快捷键。

## 接口

名为“接口”的部分

### ShortcutEvent

Section titled “ShortcutEvent”

#### 属性

名为“属性”的部分

属性| 类型| 定义于
---|---|---
`id`| `数字`| **源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/global-shortcut/guest-js/index.ts#L15>
`shortcut`| `string`| **源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/global-shortcut/guest-js/index.ts#L14>
`state`| `"Released"` | `"Pressed"`| **源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/global-shortcut/guest-js/index.ts#L16>

## 类型别名 (Type Aliases)

名为“类型别名”的部分

### ShortcutHandler()

Section titled “ShortcutHandler()”

    type ShortcutHandler: (event) => void;

#### 参数

名为“参数”的部分

参数| 类型
---|---
`event`| [`ShortcutEvent`](/reference/javascript/global-shortcut/#shortcutevent)

#### 返回

名为“返回值”的部分

`空`

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/global-shortcut/guest-js/index.ts#L19>

## 函数

名为“函数”的部分

### isRegistered()

Section titled “isRegistered()”

    function isRegistered(shortcut): Promise<boolean>

确定给定的快捷键是否由本应用程序注册。

如果快捷键是由其他应用程序注册的，此方法仍将返回 `false`。

#### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`shortcut`| `string`| 快捷键定义，修饰键和按键之间用 “+” 分隔，例如 CmdOrControl+Q

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`boolean`>

#### 示例

标题为“Example”的章节

    import { isRegistered } from '@tauri-apps/plugin-global-shortcut';

    const isRegistered = await isRegistered('CommandOrControl+P');

#### 始于

名为“起始版本”的部分

2.0.0

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/global-shortcut/guest-js/index.ts#L117>

* * *

### register()

Section titled “register()”

    function register(shortcuts, handler): Promise<void>

注册一个全局快捷键或一组快捷键。

当用户按下任何已注册的快捷键时，处理程序将被调用。

如果快捷键已被其他应用程序占用，则处理程序将不会触发。请确保快捷键尽可能唯一，同时考虑用户体验。

#### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`shortcuts`| `string` | `string`[]| -
`处理程序`| [`ShortcutHandler`](/reference/javascript/global-shortcut/#shortcuthandler)| 快捷键处理回调 - 以触发的快捷键作为参数

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

#### 示例

标题为“Example”的章节

    import { register } from '@tauri-apps/plugin-global-shortcut';

    // register a single hotkey

    await register('CommandOrControl+Shift+C', (event) => {

      if (event.state === "Pressed") {

          console.log('Shortcut triggered');

      }

    });

    // or register multiple hotkeys at once

    await register(['CommandOrControl+Shift+C', 'Alt+A'], (event) => {

      console.log(`Shortcut ${event.shortcut} triggered`);

    });

#### 始于

名为“起始版本”的部分

2.0.0

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/global-shortcut/guest-js/index.ts#L51>

* * *

### unregister()

名为“unregister()”的部分

    function unregister(shortcuts): Promise<void>

注销一个全局快捷键或一组快捷键。

#### 参数

名为“参数”的部分

参数| 类型
---|---
`shortcuts`| `string` | `string`[]

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

#### 示例

标题为“Example”的章节

    import { unregister } from '@tauri-apps/plugin-global-shortcut';

    // unregister a single hotkey

    await unregister('CmdOrControl+Space');

    // or unregister multiple hotkeys at the same time

    await unregister(['CmdOrControl+Space', 'Alt+A']);

#### 始于

名为“起始版本”的部分

2.0.0

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/global-shortcut/guest-js/index.ts#L82>

* * *

### unregisterAll()

Section titled “unregisterAll()”

    function unregisterAll(): Promise<void>

注销所有全局快捷键。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

#### 示例

标题为“Example”的章节

    import { unregisterAll } from '@tauri-apps/plugin-global-shortcut';

    await unregisterAll();

#### 始于

名为“起始版本”的部分

2.0.0

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/global-shortcut/guest-js/index.ts#L98>