# @tauri-apps/plugin-barcode-scanner

_Source: https://v2.tauri.org.cn/reference/javascript/barcode-scanner/_

## 枚举

标题为“枚举”的章节

### 格式

标题为“格式”的章节

#### 枚举成员

标题为“枚举成员”的章节

##### Aztec

标题为“Aztec”的章节

    Aztec: "AZTEC";

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L30>

##### Codabar

标题为“Codabar”的章节

    Codabar: "CODABAR";

iOS 不支持。

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L28>

##### Code128

标题为“Code128”的章节

    Code128: "CODE_128";

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L24>

##### Code39

标题为“Code39”的章节

    Code39: "CODE_39";

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L22>

##### Code93

标题为“Code93”的章节

    Code93: "CODE_93";

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L23>

##### DataMatrix

标题为“DataMatrix”的章节

    DataMatrix: "DATA_MATRIX";

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L31>

##### EAN13

标题为“EAN13”的章节

    EAN13: "EAN_13";

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L21>

##### EAN8

标题为“EAN8”的章节

    EAN8: "EAN_8";

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L20>

##### GS1DataBar

标题为“GS1DataBar”的章节

    GS1DataBar: "GS1_DATA_BAR";

Android 不支持。需要 iOS 15.4+

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L36>

##### GS1DataBarExpanded

标题为“GS1DataBarExpanded”的章节

    GS1DataBarExpanded: "GS1_DATA_BAR_EXPANDED";

Android 不支持。需要 iOS 15.4+

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L44>

##### GS1DataBarLimited

标题为“GS1DataBarLimited”的章节

    GS1DataBarLimited: "GS1_DATA_BAR_LIMITED";

Android 不支持。需要 iOS 15.4+

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L40>

##### ITF

标题为“ITF”的章节

    ITF: "ITF";

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L29>

##### PDF417

标题为“PDF417”的章节

    PDF417: "PDF_417";

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L32>

##### QRCode

标题为“QRCode”的章节

    QRCode: "QR_CODE";

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L14>

##### UPC_A

标题为“UPC_A”的章节

    UPC_A: "UPC_A";

iOS 不支持。

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L18>

##### UPC_E

标题为“UPC_E”的章节

    UPC_E: "UPC_E";

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L19>

## 接口

名为“接口”的部分

### ScanOptions

标题为“ScanOptions”的章节

#### 属性

名为“属性”的部分

属性| 类型| 定义于
---|---|---
`cameraDirection?`| `"back"` | `"front"`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L48>
`formats?`| [`格式`](/reference/javascript/barcode-scanner/#format)[]| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L49>
`windowed?`| `布尔值 (boolean)`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L50>

* * *

### Scanned

标题为“Scanned”的章节

#### 属性

名为“属性”的部分

属性| 类型| 定义于
---|---|---
`bounds`| `未知`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L56>
`content`| `string`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L54>
`format`| [`格式`](/reference/javascript/barcode-scanner/#format)| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L55>

## 类型别名 (Type Aliases)

名为“类型别名”的部分

### PermissionState

名为“PermissionState”的部分

    type PermissionState: "granted" | "denied" | "prompt" | "prompt-with-rationale";

**来源** : 未定义

## 函数

名为“函数”的部分

### cancel()

标题为“cancel()”的章节

    function cancel(): Promise<void>

取消当前的扫描流程。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L70>

* * *

### checkPermissions()

名为“checkPermissions()”的部分

    function checkPermissions(): Promise<PermissionState>

获取权限状态。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`PermissionState`>

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L77>

* * *

### openAppSettings()

标题为“openAppSettings()”的章节

    function openAppSettings(): Promise<void>

打开应用设置。如果权限被拒绝且用户必须手动启用，则非常有用。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L95>

* * *

### requestPermissions()

名为“requestPermissions()”的部分

    function requestPermissions(): Promise<PermissionState>

请求使用相机的权限。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`PermissionState`>

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L86>

* * *

### scan()

标题为“scan()”的章节

    function scan(options?): Promise<Scanned>

开始扫描。

#### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`选项`?| [`ScanOptions`](/reference/javascript/barcode-scanner/#scanoptions)|

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`Scanned`](/reference/javascript/barcode-scanner/#scanned)>

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/barcode-scanner/guest-js/index.ts#L63>