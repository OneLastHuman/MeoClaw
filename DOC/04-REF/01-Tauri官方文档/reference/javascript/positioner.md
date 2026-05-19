# @tauri-apps/plugin-positioner

_Source: https://v2.tauri.org.cn/reference/javascript/positioner/_

## 枚举

标题为“枚举”的章节

### Position

标题为“Position”的章节

众所周知的窗口位置。

#### 枚举成员

标题为“枚举成员”的章节

##### BottomCenter

标题为“BottomCenter”的章节

    BottomCenter: 5;

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/positioner/guest-js/index.ts#L18>

##### BottomLeft

标题为“BottomLeft”的章节

    BottomLeft: 2;

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/positioner/guest-js/index.ts#L15>

##### BottomRight

标题为“BottomRight”的章节

    BottomRight: 3;

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/positioner/guest-js/index.ts#L16>

##### Center

标题为“Center”的章节

    Center: 8;

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/positioner/guest-js/index.ts#L21>

##### LeftCenter

标题为“LeftCenter”的章节

    LeftCenter: 6;

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/positioner/guest-js/index.ts#L19>

##### RightCenter

标题为“RightCenter”的章节

    RightCenter: 7;

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/positioner/guest-js/index.ts#L20>

##### TopCenter

标题为“TopCenter”的章节

    TopCenter: 4;

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/positioner/guest-js/index.ts#L17>

##### TopLeft

标题为“TopLeft”的章节

    TopLeft: 0;

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/positioner/guest-js/index.ts#L13>

##### TopRight

标题为“TopRight”的章节

    TopRight: 1;

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/positioner/guest-js/index.ts#L14>

##### TrayBottomCenter

标题为“TrayBottomCenter”的章节

    TrayBottomCenter: 14;

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/positioner/guest-js/index.ts#L27>

##### TrayBottomLeft

标题为“TrayBottomLeft”的章节

    TrayBottomLeft: 10;

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/positioner/guest-js/index.ts#L23>

##### TrayBottomRight

标题为“TrayBottomRight”的章节

    TrayBottomRight: 12;

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/positioner/guest-js/index.ts#L25>

##### TrayCenter

标题为“TrayCenter”的章节

    TrayCenter: 13;

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/positioner/guest-js/index.ts#L26>

##### TrayLeft

标题为“TrayLeft”的章节

    TrayLeft: 9;

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/positioner/guest-js/index.ts#L22>

##### TrayRight

标题为“TrayRight”的章节

    TrayRight: 11;

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/positioner/guest-js/index.ts#L24>

## 函数

名为“函数”的部分

### handleIconState()

标题为“handleIconState()”的章节

    function handleIconState(event): Promise<void>

#### 参数

名为“参数”的部分

参数| 类型
---|---
`event`| `TrayIconEvent`

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/positioner/guest-js/index.ts#L55>

* * *

### moveWindow()

标题为“moveWindow()”的章节

    function moveWindow(to): Promise<void>

使用 `WindowExt.move_window()` 将 `Window` 移动到指定 [Position](/reference/javascript/positioner/#position)。所有位置均相对于**当前** 屏幕。

#### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`to`| [`Position`](/reference/javascript/positioner/#position)| 要移动到的 [Position](/reference/javascript/positioner/#position)。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/positioner/guest-js/index.ts#L36>

* * *

### moveWindowConstrained()

标题为“moveWindowConstrained()”的章节

    function moveWindowConstrained(to): Promise<void>

使用 `WindowExt.move_window_constrained()` 将 `Window` 移动到指定 [Position](/reference/javascript/positioner/#position)。

对于托盘图标位置，此移动操作会将窗口限制在屏幕尺寸内。

#### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`to`| [`Position`](/reference/javascript/positioner/#position)| 要移动到的（托盘）[Position](/reference/javascript/positioner/#position)。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/positioner/guest-js/index.ts#L49>