# @tauri-apps/plugin-window-state

_Source: https://v2.tauri.org.cn/reference/javascript/window-state/_

## 枚举

标题为“枚举”的章节

### StateFlags (状态标志)

“StateFlags” 章节

#### 枚举成员

标题为“枚举成员”的章节

##### ALL

“ALL” 章节

    ALL: 63;

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/window-state/guest-js/index.ts#L15>

##### DECORATIONS

“DECORATIONS” 章节

    DECORATIONS: 16;

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/window-state/guest-js/index.ts#L13>

##### FULLSCREEN

“FULLSCREEN” 章节

    FULLSCREEN: 32;

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/window-state/guest-js/index.ts#L14>

##### MAXIMIZED

“MAXIMIZED” 章节

    MAXIMIZED: 4;

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/window-state/guest-js/index.ts#L11>

##### POSITION

“POSITION” 章节

    POSITION: 2;

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/window-state/guest-js/index.ts#L10>

##### SIZE

“SIZE” 章节

    SIZE: 1;

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/window-state/guest-js/index.ts#L9>

##### VISIBLE

“VISIBLE” 章节

    VISIBLE: 8;

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/window-state/guest-js/index.ts#L12>

## 函数

名为“函数”的部分

### filename()

“filename()” 章节

    function filename(): Promise<string>

获取用于存储窗口状态的文件名称。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`string`>

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/window-state/guest-js/index.ts#L44>

* * *

### restoreState()

“restoreState()” 章节

    function restoreState(label, flags?): Promise<void>

从磁盘恢复指定窗口的状态。

#### 参数

名为“参数”的部分

参数| 类型
---|---
`标签`| `string`
`flags (标志)`?| [`StateFlags (状态标志)`](/reference/javascript/window-state/#stateflags)

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/window-state/guest-js/index.ts#L28>

* * *

### restoreStateCurrent()

“restoreStateCurrent()” 章节

    function restoreStateCurrent(flags?): Promise<void>

从磁盘恢复当前窗口的状态。

#### 参数

名为“参数”的部分

参数| 类型
---|---
`flags (标志)`?| [`StateFlags (状态标志)`](/reference/javascript/window-state/#stateflags)

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/window-state/guest-js/index.ts#L38>

* * *

### saveWindowState()

“saveWindowState()” 章节

    function saveWindowState(flags?): Promise<void>

将所有已打开窗口的状态保存到磁盘。

#### 参数

名为“参数”的部分

参数| 类型
---|---
`flags (标志)`?| [`StateFlags (状态标志)`](/reference/javascript/window-state/#stateflags)

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/window-state/guest-js/index.ts#L21>