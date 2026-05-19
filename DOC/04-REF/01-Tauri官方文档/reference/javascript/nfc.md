# @tauri-apps/plugin-nfc

_Source: https://v2.tauri.org.cn/reference/javascript/nfc/_

## 枚举

标题为“枚举”的章节

### NFCTypeNameFormat

名为“NFCTypeNameFormat”的章节

#### 枚举成员

标题为“枚举成员”的章节

##### AbsoluteURI

名为“AbsoluteURI”的章节

    AbsoluteURI: 3;

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/nfc/guest-js/index.ts#L84>

##### Empty

名为“Empty”的章节

    Empty: 0;

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/nfc/guest-js/index.ts#L81>

##### Media

名为“Media”的章节

    Media: 2;

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/nfc/guest-js/index.ts#L83>

##### NfcExternal

名为“NfcExternal”的章节

    NfcExternal: 4;

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/nfc/guest-js/index.ts#L85>

##### NfcWellKnown

名为“NfcWellKnown”的章节

    NfcWellKnown: 1;

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/nfc/guest-js/index.ts#L82>

##### Unchanged

名为“Unchanged”的章节

    Unchanged: 6;

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/nfc/guest-js/index.ts#L87>

##### Unknown

名为“Unknown”的章节

    Unknown: 5;

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/nfc/guest-js/index.ts#L86>

* * *

### TechKind

名为“TechKind”的章节

#### 枚举成员

标题为“枚举成员”的章节

##### IsoDep

名为“IsoDep”的章节

    IsoDep: 0;

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/nfc/guest-js/index.ts#L17>

##### MifareClassic

名为“MifareClassic”的章节

    MifareClassic: 1;

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/nfc/guest-js/index.ts#L18>

##### MifareUltralight

名为“MifareUltralight”的章节

    MifareUltralight: 2;

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/nfc/guest-js/index.ts#L19>

##### Ndef

名为“Ndef”的章节

    Ndef: 3;

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/nfc/guest-js/index.ts#L20>

##### NdefFormatable

名为“NdefFormatable”的章节

    NdefFormatable: 4;

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/nfc/guest-js/index.ts#L21>

##### NfcA

名为“NfcA”的章节

    NfcA: 5;

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/nfc/guest-js/index.ts#L22>

##### NfcB

名为“NfcB”的章节

    NfcB: 6;

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/nfc/guest-js/index.ts#L23>

##### NfcBarcode

名为“NfcBarcode”的章节

    NfcBarcode: 7;

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/nfc/guest-js/index.ts#L24>

##### NfcF

名为“NfcF”的章节

    NfcF: 8;

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/nfc/guest-js/index.ts#L25>

##### NfcV

名为“NfcV”的章节

    NfcV: 9;

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/nfc/guest-js/index.ts#L26>

## 接口

名为“接口”的部分

### NFCRecord

名为“NFCRecord”的章节

#### 属性

名为“属性”的部分

属性| 类型| 定义于
---|---|---
`format`| [`NFCTypeNameFormat`](/reference/javascript/nfc/#nfctypenameformat)| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/nfc/guest-js/index.ts#L104>
`id`| `数字`[]| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/nfc/guest-js/index.ts#L106>
`kind`| `数字`[]| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/nfc/guest-js/index.ts#L105>
`payload`| `数字`[]| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/nfc/guest-js/index.ts#L107>

* * *

### ScanOptions

名为“ScanOptions”的章节

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
`keepSessionAlive?`| `布尔值 (boolean)`| -| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/nfc/guest-js/index.ts#L63>
`message?`| `string`| 在 UI 中显示的消息。仅限 iOS。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/nfc/guest-js/index.ts#L65>
`successMessage?`| `string`| 当消息读取完毕时在 UI 中显示的消息。仅限 iOS。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/nfc/guest-js/index.ts#L67>

* * *

### Tag

名为“Tag”的章节

#### 属性

名为“属性”的部分

属性| 类型| 定义于
---|---|---
`id`| `数字`[]| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/nfc/guest-js/index.ts#L98>
`kind`| `string`[]| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/nfc/guest-js/index.ts#L99>
`records`| [`TagRecord`](/reference/javascript/nfc/#tagrecord)[]| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/nfc/guest-js/index.ts#L100>

* * *

### TagRecord

名为“TagRecord”的章节

#### 属性

名为“属性”的部分

属性| 类型| 定义于
---|---|---
`id`| `数字`[]| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/nfc/guest-js/index.ts#L93>
`kind`| `数字`[]| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/nfc/guest-js/index.ts#L92>
`payload`| `数字`[]| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/nfc/guest-js/index.ts#L94>
`tnf`| [`NFCTypeNameFormat`](/reference/javascript/nfc/#nfctypenameformat)| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/nfc/guest-js/index.ts#L91>

* * *

### UriFilter

名为“UriFilter”的章节

#### 属性

名为“属性”的部分

属性| 类型| 定义于
---|---|---
`host?`| `string`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/nfc/guest-js/index.ts#L12>
`pathPrefix?`| `string`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/nfc/guest-js/index.ts#L13>
`scheme?`| `string`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/nfc/guest-js/index.ts#L11>

* * *

### WriteOptions

名为“WriteOptions”的章节

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
`kind?`| [`ScanKind`](/reference/javascript/nfc/#scankind)| -| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/nfc/guest-js/index.ts#L71>
`message?`| `string`| 读取标签时在 UI 中显示的消息。仅限 iOS。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/nfc/guest-js/index.ts#L73>
`successMessage?`| `string`| 当消息写入完毕时在 UI 中显示的消息。仅限 iOS。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/nfc/guest-js/index.ts#L77>
`successfulReadMessage?`| `string`| 标签读取完毕时在 UI 中显示的消息。仅限 iOS。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/nfc/guest-js/index.ts#L75>

## 类型别名 (Type Aliases)

名为“类型别名”的部分

### ScanKind

名为“ScanKind”的章节

    type ScanKind: object | object;

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/nfc/guest-js/index.ts#L29>

## Variables

名为“Variables”的章节

### RTD_TEXT

名为“RTD_TEXT”的章节

    const RTD_TEXT: number[];

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/nfc/guest-js/index.ts#L7>

* * *

### RTD_URI

名为“RTD_URI”的章节

    const RTD_URI: number[];

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/nfc/guest-js/index.ts#L8>

## 函数

名为“函数”的部分

### isAvailable()

名为“isAvailable()”的章节

    function isAvailable(): Promise<boolean>

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`boolean`>

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/nfc/guest-js/index.ts#L271>

* * *

### record()

名为“record()”的章节

    function record(

       format,

       kind,

       id,

       payload): NFCRecord

#### 参数

名为“参数”的部分

参数| 类型
---|---
`format`| [`NFCTypeNameFormat`](/reference/javascript/nfc/#nfctypenameformat)
`kind`| `string` | `number`[]
`ID`| `string` | `number`[]
`载荷 (payload)`| `string` | `number`[]

#### 返回

名为“返回值”的部分

[`NFCRecord`](/reference/javascript/nfc/#nfcrecord)

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/nfc/guest-js/index.ts#L110>

* * *

### scan()

名为“scan()”的章节

    function scan(kind, options?): Promise<Tag>

扫描 NFC 标签。

    import { scan } from "@tauri-apps/plugin-nfc";

    await scan({ type: "tag" });

更多信息，请参阅 <https://developer.android.com.cn/develop/connectivity/nfc/nfc#ndef>。

#### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`kind`| [`ScanKind`](/reference/javascript/nfc/#scankind)|
`选项`?| [`ScanOptions`](/reference/javascript/nfc/#scanoptions)|

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`Tag`](/reference/javascript/nfc/#tag)>

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/nfc/guest-js/index.ts#L231>

* * *

### textRecord()

名为“textRecord()”的章节

    function textRecord(

       text,

       id?,

       language?): NFCRecord

#### 参数

名为“参数”的部分

参数| 类型| 默认值
---|---|---
`text`| `string`| `未定义`
`ID`?| `string` | `number`[]| `未定义`
`language`?| `string`| `'en'`

#### 返回

名为“返回值”的部分

[`NFCRecord`](/reference/javascript/nfc/#nfcrecord)

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/nfc/guest-js/index.ts#L130>

* * *

### uriRecord()

名为“uriRecord()”的章节

    function uriRecord(uri, id?): NFCRecord

#### 参数

名为“参数”的部分

参数| 类型
---|---
`uri`| `string`
`ID`?| `string` | `number`[]

#### 返回

名为“返回值”的部分

[`NFCRecord`](/reference/javascript/nfc/#nfcrecord)

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/nfc/guest-js/index.ts#L203>

* * *

### write()

名为“write()”的章节

    function write(records, options?): Promise<void>

写入 NFC 标签。

    import { uriRecord, write } from "@tauri-apps/plugin-nfc";

    await write([uriRecord("https://tauri.org.cn")], { kind: { type: "ndef" } });

如果您之前没有将 [ScanOptions.keepSessionAlive](/reference/javascript/nfc/#keepsessionalive) 设置为 true 来调用 [scan](/reference/javascript/nfc/#scan)，它将首先扫描标签，然后对其进行写入。

#### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`records`| [`NFCRecord`](/reference/javascript/nfc/#nfcrecord)[]|
`选项`?| [`WriteOptions`](/reference/javascript/nfc/#writeoptions)|

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/nfc/guest-js/index.ts#L256>