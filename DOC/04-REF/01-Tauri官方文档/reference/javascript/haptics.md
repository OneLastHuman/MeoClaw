# @tauri-apps/plugin-haptics

_Source: https://v2.tauri.org.cn/reference/javascript/haptics/_

## 类型别名 (Type Aliases)

名为“类型别名”的部分

### ImpactFeedbackStyle

“ImpactFeedbackStyle” 章节

    type ImpactFeedbackStyle:

      | "light"

      | "medium"

      | "heavy"

      | "soft"

      | "rigid";

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/haptics/guest-js/bindings.ts#L76>

* * *

### NotificationFeedbackType

“NotificationFeedbackType” 章节

    type NotificationFeedbackType: "success" | "warning" | "error";

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/haptics/guest-js/bindings.ts#L82>

## 函数

名为“函数”的部分

### impactFeedback()

“impactFeedback()” 章节

    function impactFeedback(style): Promise<Result<null, never>>

#### 参数

名为“参数”的部分

参数| 类型
---|---
`style`| [`ImpactFeedbackStyle`](/reference/javascript/haptics/#impactfeedbackstyle)

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`Result`<`null`, `never`>>

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/haptics/guest-js/index.ts#L11>

* * *

### notificationFeedback()

“notificationFeedback()” 章节

    function notificationFeedback(type): Promise<Result<null, never>>

#### 参数

名为“参数”的部分

参数| 类型
---|---
`type`| [`NotificationFeedbackType`](/reference/javascript/haptics/#notificationfeedbacktype)

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`Result`<`null`, `never`>>

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/haptics/guest-js/index.ts#L12>

* * *

### selectionFeedback()

“selectionFeedback()” 章节

    function selectionFeedback(): Promise<Result<null, never>>

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`Result`<`null`, `never`>>

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/haptics/guest-js/index.ts#L13>

* * *

### vibrate()

“vibrate()” 章节

    function vibrate(duration): Promise<Result<null, never>>

#### 参数

名为“参数”的部分

参数| 类型
---|---
`duration`| `数字`

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`Result`<`null`, `never`>>

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/haptics/guest-js/index.ts#L10>