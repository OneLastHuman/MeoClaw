# @tauri-apps/plugin-notification

_Source: https://v2.tauri.org.cn/reference/javascript/notification/_

向用户发送 Toast 通知（短暂且自动过期的操作系统窗口元素）。也可以与 Notification Web API 一起使用。

## 枚举

标题为“枚举”的章节

### 重要性

标题为“重要性”的部分

#### 枚举成员

标题为“枚举成员”的章节

##### 默认

标题为“默认”的部分

    Default: 3;

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L293>

##### 高 (High)

标题为“高”的部分

    High: 4;

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L294>

##### 低 (Low)

标题为“低”的部分

    Low: 2;

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L292>

##### 最小 (Min)

标题为“最小”的部分

    Min: 1;

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L291>

##### 无

标题为“None”的章节

    None: 0;

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L290>

* * *

### 定时循环 (ScheduleEvery)

标题为“定时循环”的部分

#### 枚举成员

标题为“枚举成员”的章节

##### 天 (Day)

标题为“天”的部分

    Day: "day";

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L165>

##### 小时 (Hour)

标题为“小时”的部分

    Hour: "hour";

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L166>

##### 分钟 (Minute)

标题为“分钟”的部分

    Minute: "minute";

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L167>

##### 月 (Month)

标题为“月”的部分

    Month: "month";

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L162>

##### 秒 (Second)

标题为“秒”的部分

    Second: "second";

iOS 不支持。

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L171>

##### 两周 (TwoWeeks)

标题为“两周”的部分

    TwoWeeks: "twoWeeks";

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L163>

##### 周 (Week)

标题为“周”的部分

    Week: "week";

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L164>

##### 年 (Year)

标题为“年”的部分

    Year: "year";

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L161>

* * *

### 可见性

标题为“可见性”的部分

#### 枚举成员

标题为“枚举成员”的章节

##### 私密 (Private)

标题为“私密”的部分

    Private: 0;

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L299>

##### Public

标题为“公开”的部分

    Public: 1;

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L300>

##### 秘密 (Secret)

标题为“秘密”的部分

    Secret: -1;

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L298>

## 类

名为“类”的部分

### 调度 (Schedule)

标题为“调度”的部分

#### 构造函数

名为“构造函数”的部分

##### new Schedule()

标题为“new Schedule()”的部分

    new Schedule(): Schedule

###### 返回

名为“返回值”的部分

[`调度 (Schedule)`](/reference/javascript/notification/#schedule)

#### 属性

名为“属性”的部分

属性| 类型| 定义于
---|---|---
`at`| `undefined` | `object`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L175>
`every`| `undefined` | `object`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L190>
`interval`| `undefined` | `object`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L183>

#### 方法

名为“方法”的部分

##### at()

标题为“at()”的部分

    static at(

       date,

       repeating,

       allowWhileIdle): Schedule

###### 参数

名为“参数”的部分

参数| 类型| 默认值
---|---|---
`日期 (date)`| [`Date`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Date)| `未定义`
`重复 (repeating)`| `布尔值 (boolean)`| `false`
`空闲时允许 (allowWhileIdle)`| `布尔值 (boolean)`| `false`

###### 返回

名为“返回值”的部分

[`调度 (Schedule)`](/reference/javascript/notification/#schedule)

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L198>

##### every()

标题为“every()”的部分

    static every(

       kind,

       count,

       allowWhileIdle): Schedule

###### 参数

名为“参数”的部分

参数| 类型| 默认值
---|---|---
`类型 (kind)`| [`定时循环 (ScheduleEvery)`](/reference/javascript/notification/#scheduleevery)| `未定义`
`count`| `数字`| `未定义`
`空闲时允许 (allowWhileIdle)`| `布尔值 (boolean)`| `false`

###### 返回

名为“返回值”的部分

[`调度 (Schedule)`](/reference/javascript/notification/#schedule)

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L217>

##### interval()

标题为“interval()”的部分

    static interval(interval, allowWhileIdle): Schedule

###### 参数

名为“参数”的部分

参数| 类型| 默认值
---|---|---
`间隔 (interval)`| [`调度间隔 (ScheduleInterval)`](/reference/javascript/notification/#scheduleinterval)| `未定义`
`空闲时允许 (allowWhileIdle)`| `布尔值 (boolean)`| `false`

###### 返回

名为“返回值”的部分

[`调度 (Schedule)`](/reference/javascript/notification/#schedule)

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L206>

## 接口

名为“接口”的部分

### 操作 (Action)

标题为“操作”的部分

#### 属性

名为“属性”的部分

属性| 类型| 定义于
---|---|---
`destructive?`| `布尔值 (boolean)`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L245>
`foreground?`| `布尔值 (boolean)`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L244>
`id`| `string`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L241>
`input?`| `布尔值 (boolean)`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L246>
`inputButtonTitle?`| `string`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L247>
`inputPlaceholder?`| `string`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L248>
`requiresAuthentication?`| `布尔值 (boolean)`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L243>
`title`| `string`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L242>

* * *

### 操作类型 (ActionType)

标题为“操作类型”的部分

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
`actions`| [`操作 (Action)`](/reference/javascript/notification/#action)[]| 关联操作列表| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L259>
`allowInCarPlay?`| `布尔值 (boolean)`| -| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L262>
`customDismissAction?`| `布尔值 (boolean)`| -| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L261>
`hiddenPreviewsBodyPlaceholder?`| `string`| -| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L260>
`hiddenPreviewsShowSubtitle?`| `布尔值 (boolean)`| -| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L264>
`hiddenPreviewsShowTitle?`| `布尔值 (boolean)`| -| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L263>
`id`| `string`| 此操作类型的标识符| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L255>

* * *

### 活动通知 (ActiveNotification)

标题为“活动通知”的部分

#### 属性

名为“属性”的部分

属性| 类型| 定义于
---|---|---
`actionTypeId?`| `string`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L284>
`attachments`| [`附件 (Attachment)`](/reference/javascript/notification/#attachment)[]| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L283>
`body?`| `string`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L278>
`data`| [`Record`](https://typescript.net.cn/docs/handbook/utility-types.html#recordkeys-type)<`string`, `string`>| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L281>
`extra`| [`Record`](https://typescript.net.cn/docs/handbook/utility-types.html#recordkeys-type)<`string`, `unknown`>| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L282>
`group?`| `string`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L279>
`groupSummary`| `布尔值 (boolean)`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L280>
`id`| `数字`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L275>
`schedule?`| [`调度 (Schedule)`](/reference/javascript/notification/#schedule)| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L285>
`sound?`| `string`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L286>
`tag?`| `string`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L276>
`title?`| `string`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L277>

* * *

### 附件 (Attachment)

标题为“附件”的部分

通知的附件。

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
`id`| `string`| 附件标识符。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L235>
`url`| `string`| 附件 URL。接受 `asset` 和 `file` 协议。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L237>

* * *

### 频道 (Channel)

标题为“频道”的部分

#### 属性

名为“属性”的部分

属性| 类型| 定义于
---|---|---
`description?`| `string`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L306>
`id`| `string`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L304>
`importance?`| [`重要性`](/reference/javascript/notification/#importance)| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L311>
`lightColor?`| `string`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L309>
`lights?`| `布尔值 (boolean)`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L308>
`name`| `string`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L305>
`sound?`| `string`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L307>
`vibration?`| `布尔值 (boolean)`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L310>
`visibility?`| [`可见性`](/reference/javascript/notification/#visibility)| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L312>

* * *

### 选项

选项章节

发送通知的选项。

#### 始于

名为“起始版本”的部分

2.0.0

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
`actionTypeId?`| `string`| 为此通知定义操作类型。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L62>
`attachments?`| [`附件 (Attachment)`](/reference/javascript/notification/#attachment)[]| 通知附件。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L110>
`autoCancel?`| `布尔值 (boolean)`| 当用户点击通知时，自动取消该通知。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L126>
`body?`| `string`| 可选的通知正文。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L44>
`channelId?`| `string`| 发送此通知的 [频道](/reference/javascript/notification/#channel) 标识符。如果频道不存在，通知将不会触发。请确保使用 listChannels 和 [createChannel](/reference/javascript/notification/#createchannel) 确保频道存在。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L36>
`extra?`| [`Record`](https://typescript.net.cn/docs/handbook/utility-types.html#recordkeys-type)<`string`, `unknown`>| 要存储在通知中的额外有效载荷。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L114>
`group?`| `string`| 用于对多个通知进行分组的标识符。<https://developer.apple.com/documentation/usernotifications/unmutablenotificationcontent/1649872-threadidentifier>| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L68>
`groupSummary?`| `布尔值 (boolean)`| 指示系统此通知是 Android 上某一组通知的摘要。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L72>
`icon?`| `string`| 通知图标。在 Android 上，图标必须放置在应用程序的 `res/drawable` 文件夹中。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L96>
`iconColor?`| `string`| Android 上的图标颜色。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L106>
`id?`| `数字`| 用于稍后引用此对象的通知标识符。必须是 32 位整数。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L29>
`inboxLines?`| `string`[]| 要添加到通知中的行列表。将通知样式更改为收件箱。不能与 `largeBody` 一起使用。最多支持 5 行。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L90>
`largeBody?`| `string`| 多行文本。将通知样式更改为大文本。不能与 `inboxLines` 一起使用。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L54>
`largeIcon?`| `string`| 通知大图标（Android）。图标必须放置在应用程序的 `res/drawable` 文件夹中。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L102>
`number?`| `数字`| 设置此通知在 Android 上代表的项目数量。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L138>
`ongoing?`| `布尔值 (boolean)`| 如果为 true，用户在 Android 上无法取消该通知。应用程序服务必须管理该通知的取消。它通常用于指示后台任务正在挂起（例如文件下载）或用户正在进行某项操作（例如正在播放音乐）。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L122>
`schedule?`| [`调度 (Schedule)`](/reference/javascript/notification/#schedule)| 安排此通知在以后的时间或固定间隔内触发。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L48>
`silent?`| `布尔值 (boolean)`| 将 iOS 上的通知显示更改为静默（无标记、无声音、不列出）。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L130>
`sound?`| `string`| 通知的声音资源名称或文件路径。特定平台的行为：- 在 macOS 上：使用系统声音（例如“Ping”、“Blow”）或应用包中的声音文件 - 在 Linux 上：使用 XDG 主题声音（例如“message-new-instant”）或文件路径 - 在 Windows 上：使用声音文件的文件路径（.wav 格式） - 在移动设备上：使用资源名称| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L82>
`summary?`| `string`| 使用 `largeBody`、`inboxLines` 或 `groupSummary` 的通知详细文本。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L58>
`title`| `string`| 通知标题。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L40>
`visibility?`| [`可见性`](/reference/javascript/notification/#visibility)| 通知可见性。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L134>

* * *

### 待处理通知 (PendingNotification)

标题为“待处理通知”的部分

#### 属性

名为“属性”的部分

属性| 类型| 定义于
---|---|---
`body?`| `string`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L270>
`id`| `数字`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L268>
`schedule`| [`调度 (Schedule)`](/reference/javascript/notification/#schedule)| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L271>
`title?`| `string`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L269>

* * *

### 调度间隔 (ScheduleInterval)

标题为“调度间隔”的部分

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
`day?`| `数字`| -| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L144>
`hour?`| `数字`| -| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L155>
`minute?`| `数字`| -| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L156>
`month?`| `数字`| -| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L143>
`second?`| `数字`| -| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L157>
`weekday?`| `数字`| 1 - 周日, 2 - 周一, 3 - 周二, 4 - 周三, 5 - 周四, 6 - 周五, 7 - 周六| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L154>
`year?`| `数字`| -| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L142>

## 类型别名 (Type Aliases)

名为“类型别名”的部分

### PermissionState

名为“PermissionState”的部分

    type PermissionState: "granted" | "denied" | "prompt" | "prompt-with-rationale";

**来源** : 未定义

## 函数

名为“函数”的部分

### active()

标题为“active()”的部分

    function active(): Promise<ActiveNotification[]>

检索活动通知列表。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`ActiveNotification`](/reference/javascript/notification/#activenotification)[]>

解析为活动通知列表的 Promise。

#### 示例

标题为“Example”的章节

    import { active } from '@tauri-apps/plugin-notification';

    const activeNotifications = await active();

#### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L465>

* * *

### cancel()

标题为“cancel()”的部分

    function cancel(notifications): Promise<void>

取消具有给定标识符列表的待处理通知。

#### 参数

名为“参数”的部分

参数| 类型
---|---
`notifications`| `数字`[]

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

#### 示例

标题为“Example”的章节

    import { cancel } from '@tauri-apps/plugin-notification';

    await cancel([-34234, 23432, 4311]);

#### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L431>

* * *

### cancelAll()

标题为“cancelAll()”的部分

    function cancelAll(): Promise<void>

取消所有待处理的通知。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

#### 示例

标题为“Example”的章节

    import { cancelAll } from '@tauri-apps/plugin-notification';

    await cancelAll();

#### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L448>

* * *

### channels()

标题为“channels()”的部分

    function channels(): Promise<Channel[]>

检索通知频道列表。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`Channel`](/reference/javascript/notification/#channel)[]>

解析为通知频道列表的 Promise。

#### 示例

标题为“Example”的章节

    import { channels } from '@tauri-apps/plugin-notification';

    const notificationChannels = await channels();

#### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L559>

* * *

### createChannel()

标题为“createChannel()”的部分

    function createChannel(channel): Promise<void>

创建一个通知频道。

#### 参数

名为“参数”的部分

参数| 类型
---|---
`频道 (channel)`| [`频道 (Channel)`](/reference/javascript/notification/#channel)

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

#### 示例

标题为“Example”的章节

    import { createChannel, Importance, Visibility } from '@tauri-apps/plugin-notification';

    await createChannel({

      id: 'new-messages',

      name: 'New Messages',

      lights: true,

      vibration: true,

      importance: Importance.Default,

      visibility: Visibility.Private

    });

#### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L525>

* * *

### isPermissionGranted()

标题为“isPermissionGranted()”的部分

    function isPermissionGranted(): Promise<boolean>

检查发送通知的权限是否已被授予。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`boolean`>

#### 示例

标题为“Example”的章节

    import { isPermissionGranted } from '@tauri-apps/plugin-notification';

    const permissionGranted = await isPermissionGranted();

#### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L325>

* * *

### onAction()

标题为“onAction()”的部分

    function onAction(cb): Promise<PluginListener>

#### 参数

名为“参数”的部分

参数| 类型
---|---
`cb`| (`notification`) => `void`

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`PluginListener`>

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L569>

* * *

### onNotificationReceived()

标题为“onNotificationReceived()”的部分

    function onNotificationReceived(cb): Promise<PluginListener>

#### 参数

名为“参数”的部分

参数| 类型
---|---
`cb`| (`notification`) => `void`

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`PluginListener`>

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L563>

* * *

### pending()

标题为“pending()”的部分

    function pending(): Promise<PendingNotification[]>

检索待处理通知列表。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`PendingNotification`](/reference/javascript/notification/#pendingnotification)[]>

解析为待处理通知列表的 Promise。

#### 示例

标题为“Example”的章节

    import { pending } from '@tauri-apps/plugin-notification';

    const pendingNotifications = await pending();

#### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L414>

* * *

### registerActionTypes()

标题为“registerActionTypes()”的部分

    function registerActionTypes(types): Promise<void>

注册用户点击通知时执行的操作。

#### 参数

名为“参数”的部分

参数| 类型
---|---
`类型 (types)`| [`操作类型 (ActionType)`](/reference/javascript/notification/#actiontype)[]

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

#### 示例

标题为“Example”的章节

    import { registerActionTypes } from '@tauri-apps/plugin-notification';

    await registerActionTypes([{

      id: 'tauri',

      actions: [{

        id: 'my-action',

        title: 'Settings'

      }]

    }])

#### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L397>

* * *

### removeActive()

标题为“removeActive()”的部分

    function removeActive(notifications): Promise<void>

删除具有给定标识符列表的活动通知。

#### 参数

名为“参数”的部分

参数| 类型
---|---
`notifications`| `对象`[]

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

#### 示例

标题为“Example”的章节

    import { cancel } from '@tauri-apps/plugin-notification';

    await cancel([-34234, 23432, 4311])

#### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L482>

* * *

### removeAllActive()

标题为“removeAllActive()”的部分

    function removeAllActive(): Promise<void>

删除所有活动通知。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

#### 示例

标题为“Example”的章节

    import { removeAllActive } from '@tauri-apps/plugin-notification';

    await removeAllActive()

#### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L501>

* * *

### removeChannel()

标题为“removeChannel()”的部分

    function removeChannel(id): Promise<void>

删除具有给定标识符的频道。

#### 参数

名为“参数”的部分

参数| 类型
---|---
`ID`| `string`

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

#### 示例

标题为“Example”的章节

    import { removeChannel } from '@tauri-apps/plugin-notification';

    await removeChannel();

#### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L542>

* * *

### requestPermission()

标题为“requestPermission()”的部分

    function requestPermission(): Promise<NotificationPermission>

请求发送通知的权限。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`NotificationPermission`>

解析为用户是否授予权限的 Promise。

#### 示例

标题为“Example”的章节

    import { isPermissionGranted, requestPermission } from '@tauri-apps/plugin-notification';

    let permissionGranted = await isPermissionGranted();

    if (!permissionGranted) {

      const permission = await requestPermission();

      permissionGranted = permission === 'granted';

    }

#### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L348>

* * *

### sendNotification()

标题为“sendNotification()”的章节

    function sendNotification(options): void

向用户发送通知。

#### 参数

名为“参数”的部分

参数| 类型
---|---
`选项`| `string` | [`Options`](/reference/javascript/notification/#options)

#### 返回

名为“返回值”的部分

`空`

#### 示例

标题为“Example”的章节

    import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/plugin-notification';

    let permissionGranted = await isPermissionGranted();

    if (!permissionGranted) {

      const permission = await requestPermission();

      permissionGranted = permission === 'granted';

    }

    if (permissionGranted) {

      sendNotification('Tauri is awesome!');

      sendNotification({ title: 'TAURI', body: 'Tauri is awesome!' });

    }

#### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/notification/guest-js/index.ts#L370>