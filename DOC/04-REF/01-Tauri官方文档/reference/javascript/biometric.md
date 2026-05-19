# @tauri-apps/plugin-biometric

_Source: https://v2.tauri.org.cn/reference/javascript/biometric/_

## 枚举

标题为“枚举”的章节

### BiometryType（生物识别类型）

名为“BiometryType”的章节

#### 枚举成员

标题为“枚举成员”的章节

##### FaceID

名为“FaceID”的章节

    FaceID: 2;

**源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/biometric/guest-js/index.ts#L12>

##### Iris（虹膜）

名为“Iris”的章节

    Iris: 3;

**源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/biometric/guest-js/index.ts#L14>

##### 无

标题为“None”的章节

    None: 0;

**源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/biometric/guest-js/index.ts#L8>

##### TouchID

名为“TouchID”的章节

    TouchID: 1;

**源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/biometric/guest-js/index.ts#L10>

## 接口

名为“接口”的部分

### AuthOptions（认证选项）

名为“AuthOptions”的章节

#### 属性

名为“属性”的部分

属性| 类型| 定义于
---|---|---
`allowDeviceCredential?`| `布尔值 (boolean)`| **源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/biometric/guest-js/index.ts#L36>
`cancelTitle?`| `string`| **源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/biometric/guest-js/index.ts#L37>
`confirmationRequired?`| `布尔值 (boolean)`| **源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/biometric/guest-js/index.ts#L45>
`fallbackTitle?`| `string`| **源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/biometric/guest-js/index.ts#L40>
`maxAttemps?`| `数字`| **源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/biometric/guest-js/index.ts#L46>
`subtitle?`| `string`| **源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/biometric/guest-js/index.ts#L44>
`title?`| `string`| **源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/biometric/guest-js/index.ts#L43>

* * *

### Status（状态）

名为“Status”的章节

#### 属性

名为“属性”的部分

属性| 类型| 定义于
---|---|---
`biometryType`| [`BiometryType（生物识别类型）`](/reference/javascript/biometric/#biometrytype)| **源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/biometric/guest-js/index.ts#L19>
`error?`| `string`| **源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/biometric/guest-js/index.ts#L20>
`errorCode?`| | `"appCancel"` | `"authenticationFailed"` | `"invalidContext"` | `"notInteractive"` | `"passcodeNotSet"` | `"systemCancel"` | `"userCancel"` | `"userFallback"` | `"biometryLockout"` | `"biometryNotAvailable"` | `"biometryNotEnrolled"`| **源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/biometric/guest-js/index.ts#L21>
`isAvailable`| `布尔值 (boolean)`| **源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/biometric/guest-js/index.ts#L18>

## 函数

名为“函数”的部分

### authenticate()（进行认证）

名为“authenticate()”的章节

    function authenticate(reason, options?): Promise<void>

提示用户使用系统界面（TouchID、FaceID 或 Android 虹膜）进行身份验证。如果身份验证失败，则拒绝。

    import { authenticate } from "@tauri-apps/plugin-biometric";

    await authenticate('Open your wallet');

#### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`reason（理由）`| `string`|
`选项`?| [`AuthOptions（认证选项）`](/reference/javascript/biometric/#authoptions)|

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

**源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/biometric/guest-js/index.ts#L69>

* * *

### checkStatus()（检查状态）

名为“checkStatus()”的章节

    function checkStatus(): Promise<Status>

检查生物识别认证是否可用。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`Status`](/reference/javascript/biometric/#status)>

解析为一个包含生物识别状态所有信息的对象的 Promise。

**源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/biometric/guest-js/index.ts#L53>