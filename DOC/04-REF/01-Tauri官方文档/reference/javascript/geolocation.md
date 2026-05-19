# @tauri-apps/plugin-geolocation

_Source: https://v2.tauri.org.cn/reference/javascript/geolocation/_

## 类型别名 (Type Aliases)

名为“类型别名”的部分

### Coordinates（坐标）

名为“Coordinates”的章节

    type Coordinates: object;

#### 类型声明

名为“类型定义”的章节

名称| 类型| 描述| 定义于
---|---|---|---
`accuracy`| `数字`| 纬度和经度坐标的精度级别，单位为米。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/geolocation/guest-js/index.ts#L24>
`altitude`| `number` | `null`| 用户所在的海拔高度（如果可用）。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/geolocation/guest-js/index.ts#L33>
`altitudeAccuracy`| `number` | `null`| 海拔坐标的精度级别，单位为米（如果可用）。适用于所有 iOS 版本以及 Android 8 及以上版本。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/geolocation/guest-js/index.ts#L29>
`heading`| `number` | `null`| 用户朝向的方位角（如果可用）。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/geolocation/guest-js/index.ts#L38>
`latitude`| `数字`| 纬度，单位为十进制度数。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/geolocation/guest-js/index.ts#L16>
`longitude`| `数字`| 经度，单位为十进制度数。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/geolocation/guest-js/index.ts#L20>
`speed`| `number` | `null`| -| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/geolocation/guest-js/index.ts#L34>

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/geolocation/guest-js/index.ts#L12>

* * *

### PermissionStatus（权限状态）

名为“PermissionStatus”的章节

    type PermissionStatus: object;

#### 类型声明

名为“类型定义”的章节

名称| 类型| 描述| 定义于
---|---|---|---
`coarseLocation`| `PermissionState`| coarseLocation（粗略位置）别名的权限状态。在 Android 上，它会请求/检查 ACCESS_COARSE_LOCATION 权限。在 Android 12+ 上，用户可以在“近似位置”（ACCESS_COARSE_LOCATION）和“精确位置”（ACCESS_FINE_LOCATION）之间进行选择。在 iOS 上，它的值与 `location` 别名相同。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/geolocation/guest-js/index.ts#L59>
`location`| `PermissionState`| location（位置）别名的权限状态。在 Android 上，它会同时请求/检查 ACCESS_COARSE_LOCATION 和 ACCESS_FINE_LOCATION 权限。在 iOS 上，它会请求/检查位置权限。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/geolocation/guest-js/index.ts#L49>

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/geolocation/guest-js/index.ts#L41>

* * *

### PermissionType（权限类型）

名为“PermissionType”的章节

    type PermissionType: "location" | "coarseLocation";

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/geolocation/guest-js/index.ts#L62>

* * *

### Position

标题为“Position”的章节

    type Position: object;

#### 类型声明

名为“类型定义”的章节

名称| 类型| 描述| 定义于
---|---|---|---
`coords`| [`Coordinates（坐标）`](/reference/javascript/geolocation/#coordinates)| GPS 坐标以及数据的精度。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/geolocation/guest-js/index.ts#L72>
`timestamp`| `数字`| 这些坐标的创建时间。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/geolocation/guest-js/index.ts#L68>

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/geolocation/guest-js/index.ts#L64>

* * *

### PositionOptions（位置选项）

名为“PositionOptions”的章节

    type PositionOptions: object;

#### 类型声明

名为“类型定义”的章节

名称| 类型| 描述| 定义于
---|---|---|---
`enableHighAccuracy`| `布尔值 (boolean)`| 高精度模式（例如 GPS，如果可用）。如果用户未授予 ACCESS_FINE_LOCATION 权限（`coarseLocation` 权限），则在 Android 12+ 上此项将被忽略。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/geolocation/guest-js/index.ts#L80>
`maximumAge`| `数字`| 可接受返回的缓存位置的最大时长（以毫秒为单位）。默认值：0。在 iOS 上会被忽略。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/geolocation/guest-js/index.ts#L92>
`timeout`| `数字`| 位置更新的最大等待时间（以毫秒为单位）。在 Android 上，对于 getCurrentPosition，超时设置会被忽略。在 iOS 上也会被忽略。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/geolocation/guest-js/index.ts#L86>

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/geolocation/guest-js/index.ts#L75>

## 函数

名为“函数”的部分

### checkPermissions()

名为“checkPermissions()”的部分

    function checkPermissions(): Promise<PermissionStatus>

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`PermissionStatus`](/reference/javascript/geolocation/#permissionstatus)>

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/geolocation/guest-js/index.ts#L128>

* * *

### clearWatch()

名为“clearWatch()”的章节

    function clearWatch(channelId): Promise<void>

#### 参数

名为“参数”的部分

参数| 类型
---|---
`channelId`| `数字`

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/geolocation/guest-js/index.ts#L122>

* * *

### getCurrentPosition()

名为“getCurrentPosition()”的章节

    function getCurrentPosition(options?): Promise<Position>

#### 参数

名为“参数”的部分

参数| 类型
---|---
`选项`?| [`PositionOptions（位置选项）`](/reference/javascript/geolocation/#positionoptions)

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`Position`](/reference/javascript/geolocation/#position)>

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/geolocation/guest-js/index.ts#L114>

* * *

### requestPermissions()

名为“requestPermissions()”的部分

    function requestPermissions(permissions): Promise<PermissionStatus>

#### 参数

名为“参数”的部分

参数| 类型
---|---
`permissions`| `null` | [`PermissionType`](/reference/javascript/geolocation/#permissiontype)[]

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`PermissionStatus`](/reference/javascript/geolocation/#permissionstatus)>

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/geolocation/guest-js/index.ts#L132>

* * *

### watchPosition()

名为“watchPosition()”的章节

    function watchPosition(options, cb): Promise<number>

#### 参数

名为“参数”的部分

参数| 类型
---|---
`选项`| [`PositionOptions（位置选项）`](/reference/javascript/geolocation/#positionoptions)
`cb`| (`location`, `error`?) => `void`

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`number`>

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/geolocation/guest-js/index.ts#L95>