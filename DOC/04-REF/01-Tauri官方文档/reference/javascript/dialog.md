# @tauri-apps/plugin-dialog

_Source: https://v2.tauri.org.cn/reference/javascript/dialog/_

## 接口

名为“接口”的部分

### ConfirmDialogOptions（确认对话框选项）

ConfirmDialogOptions 章节

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
`cancelLabel?`| `string`| 取消按钮的标签。| **源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L293>
`kind?`| `"info"` | `"warning"` | `"error"`| 对话框类型。默认为 `info`。| **源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L289>
`okLabel?`| `string`| 确认按钮的标签。| **源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L291>
`title?`| `string`| 对话框标题。默认为应用程序名称。| **源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L287>

* * *

### DialogFilter（对话框筛选器）

DialogFilter 章节

文件对话框的扩展名筛选器。

#### 始于

名为“起始版本”的部分

2.0.0

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
`extensions`| `string`[]| 要筛选的扩展名，不需要 `.` 前缀。**注意：** 移动平台有不同的筛选 API，可能不支持扩展名。iOS：文档选择器支持扩展名，但媒体选择器不支持。Android：不支持扩展名。在这些平台上，MIME 类型是筛选文件的主要方式。这意味着此处标记为 `extensions` 的字符串值也可以是 MIME 类型。保留 `extensions` 属性名是为了向后兼容，但未来可能会修改以明确扩展名筛选和 MIME 类型筛选之间的区别。**示例** `extensions: ['svg', 'png']`| **源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L32>
`name`| `string`| 筛选器名称。| **源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L14>

* * *

### MessageDialogOptions（消息对话框选项）

MessageDialogOptions 章节

#### 始于

名为“起始版本”的部分

2.0.0

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
`buttons?`| [`MessageDialogButtons（消息对话框按钮）`](/reference/javascript/dialog/#messagedialogbuttons)| 对话框按钮。**示例** `// 使用系统默认按钮文本 await message('Hello World!', { buttons: 'Ok' }) await message('Hello World!', { buttons: 'OkCancel' }) // 或使用自定义按钮文本 await message('Hello World!', { buttons: { ok: 'Yes!' } }) await message('Take on the task?', { buttons: { ok: 'Accept', cancel: 'Cancel' } }) await message('Show the file content?', { buttons: { yes: 'Show content', no: 'Show in folder', cancel: 'Cancel' } })` **自版本** 2.4.0| **源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L259>
`kind?`| `"info"` | `"warning"` | `"error"`| 对话框类型。默认为 `info`。| **源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L230>
~~`okLabel?`~~| `string`| 确定按钮的标签。**已弃用** ，请改用 [`MessageDialogOptions.buttons`](/reference/javascript/dialog/#buttons)。| **源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L236>
`title?`| `string`| 对话框标题。默认为应用程序名称。| **源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L228>

* * *

### OpenDialogOptions（打开对话框选项）

OpenDialogOptions 章节

打开对话框的选项。

#### 始于

名为“起始版本”的部分

2.0.0

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
`canCreateDirectories?`| `布尔值 (boolean)`| 是否允许在对话框中创建目录。默认启用。**仅限 macOS**| **源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L71>
`defaultPath?`| `string`| 初始目录或文件路径。如果是目录路径，对话框界面将跳转到该文件夹。如果不是现有目录，文件名将被设置为对话框的文件名输入框内容，并将对话框设置为父文件夹。在移动端，文件名始终用于对话框的文件名输入框。如果未提供，Android 将使用 `(invalid).txt` 作为默认文件名。| **源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L60>
`directory?`| `布尔值 (boolean)`| 对话框是否用于选择目录。| **源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L64>
`fileAccessMode?`| [`FileAccessMode（文件访问模式）`](/reference/javascript/dialog/#fileaccessmode-1)| 对话框的文件访问模式。如果未提供，则使用 `copy`，这与此选项引入之前的 [`open`](/reference/javascript/dialog/#open) 方法行为一致。**用法** 如果使用 [`: 'copy'`](/reference/javascript/dialog/#fileaccessmode) 打开文件，它将被复制到应用程序的沙盒中。这意味着文件可以被自由读取、编辑、删除或执行其他任何操作，因为文件现在归应用程序所有。这也意味着如果该文件不需要保留在应用程序沙盒中，调用者有责任删除该文件。如果使用 [`: 'scoped'`](/reference/javascript/dialog/#fileaccessmode) 打开文件，文件将保留在原位置，并且安全范围访问将由系统自动管理。**注意** 这主要用于 iOS 或 macOS 上的文档选择器，配合 [安全范围资源](https://developer.apple.com/documentation/foundation/nsurl/startaccessingsecurityscopedresource\(\)) 使用。为什么仅限文档选择器，而不包括图像或视频选择器？iOS 上的图像和视频选择器与文档选择器行为不同，它们返回的是 [NSItemProvider](https://developer.apple.com/documentation/foundation/nsitemprovider) 对象，而不是文件 URL。这些对象是临时的（仅在选择器的回调函数中可用），在回调之外无法访问。因此，对于图像和视频选择器，访问文件的唯一方法是将其复制到应用程序的沙盒中，API 返回的正是此 URL。这意味着无法为图像或视频选择器使用 `scoped` 模式。如果使用了图像或视频选择器，始终会使用 `copy`。| **源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L103>
`filters?`| [`DialogFilter（对话框筛选器）`](/reference/javascript/dialog/#dialogfilter)[]| 对话框的筛选器。在移动平台上：A) 如果 [`pickerMode`](/reference/javascript/dialog/#pickermode) 设置为 `media`、`image` 或 `video`，或者 B) 如果筛选器**仅** 包含图像或视频 MIME 类型，则将显示媒体选择器。否则，将显示文档选择器。| **源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L51>
`multiple?`| `布尔值 (boolean)`| 对话框是否允许进行多选。| **源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L62>
`pickerMode?`| [`PickerMode（选择器模式）`](/reference/javascript/dialog/#pickermode-1)| 对话框的首选模式。这主要针对移动平台（iOS 和 Android），它们拥有独立的文件选择器和媒体选择器。如果未提供，对话框将根据 [`filters`](/reference/javascript/dialog/#filters) 的 MIME 类型或扩展名自动选择最佳模式。在桌面上，此选项会被忽略。| **源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L78>
`recursive?`| `布尔值 (boolean)`| 如果 `directory` 为 true，表示稍后将递归读取该目录。定义是否允许子目录进入作用域。| **源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L69>
`title?`| `string`| 对话框窗口的标题（仅限桌面）。| **源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L42>

* * *

### SaveDialogOptions（保存对话框选项）

SaveDialogOptions 章节

保存对话框的选项。

#### 始于

名为“起始版本”的部分

2.0.0

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
`canCreateDirectories?`| `布尔值 (boolean)`| 是否允许在对话框中创建目录。默认启用。**仅限 macOS**| **源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L126>
`defaultPath?`| `string`| 初始目录或文件路径。如果是目录路径，对话框界面将跳转到该文件夹。如果不是现有目录，文件名将被设置为对话框的文件名输入框内容，并将对话框设置为父文件夹。在移动端，文件名始终用于对话框的文件名输入框。如果未提供，Android 将使用 `(invalid).txt` 作为默认文件名。| **源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L124>
`filters?`| [`DialogFilter（对话框筛选器）`](/reference/javascript/dialog/#dialogfilter)[]| 对话框的筛选器。| **源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L115>
`title?`| `string`| 对话框窗口的标题（仅限桌面）。| **源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L113>

## 类型别名 (Type Aliases)

名为“类型别名”的部分

### FileAccessMode（文件访问模式）

FileAccessMode 章节

    type FileAccessMode: "copy" | "scoped";

对话框的文件访问模式。

  * `copy`：将选定文件复制/移动到应用程序沙盒中；不需要范围访问权限。
  * `scoped`：文件保持在原地；安全范围访问由系统自动管理。

**注意：** 此选项仅在 iOS 14 及更高版本上支持。在 iOS 13 及以下版本中，该参数会被忽略。

**源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L147>

* * *

### MessageDialogButtons（消息对话框按钮）

MessageDialogButtons 章节

    type MessageDialogButtons: MessageDialogDefaultButtons | MessageDialogCustomButtons;

消息对话框的按钮。

#### 始于

名为“起始版本”的部分

2.4.0

**源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L219>

* * *

### MessageDialogButtonsOk（消息对话框“确定”按钮）

MessageDialogButtonsOk 章节

    type MessageDialogButtonsOk: object & BanExcept<"ok">;

消息对话框的“确定”按钮。

#### 类型声明

名为“类型定义”的章节

名称| 类型| 描述| 定义于
---|---|---|---
`ok（确定）`| `string`| “确定”按钮。| **源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L201>

#### 始于

名为“起始版本”的部分

2.4.0

**源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L199>

* * *

### MessageDialogButtonsOkCancel（消息对话框“确定/取消”按钮）

MessageDialogButtonsOkCancel 章节

    type MessageDialogButtonsOkCancel: object & BanExcept<"ok" | "cancel">;

消息对话框的“确定”和“取消”按钮。

#### 类型声明

名为“类型定义”的章节

名称| 类型| 描述| 定义于
---|---|---|---
`cancel（取消）`| `string`| “取消”按钮。| **源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L191>
`ok（确定）`| `string`| “确定”按钮。| **源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L189>

#### 始于

名为“起始版本”的部分

2.4.0

**源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L187>

* * *

### MessageDialogButtonsYesNoCancel（消息对话框“是/否/取消”按钮）

MessageDialogButtonsYesNoCancel 章节

    type MessageDialogButtonsYesNoCancel: object & BanExcept<"yes" | "no" | "cancel">;

消息对话框的“是”、“否”和“取消”按钮。

#### 类型声明

名为“类型定义”的章节

名称| 类型| 描述| 定义于
---|---|---|---
`cancel（取消）`| `string`| “取消”按钮。| **源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L179>
`no（否）`| `string`| “否”按钮。| **源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L177>
`yes（是）`| `string`| “是”按钮。| **源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L175>

#### 始于

名为“起始版本”的部分

2.4.0

**源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L173>

* * *

### MessageDialogCustomButtons（消息对话框自定义按钮）

MessageDialogCustomButtons 章节

    type MessageDialogCustomButtons: MessageDialogButtonsYesNoCancel | MessageDialogButtonsOkCancel | MessageDialogButtonsOk;

消息对话框的自定义按钮。

#### 始于

名为“起始版本”的部分

2.4.0

**源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L209>

* * *

### MessageDialogDefaultButtons（消息对话框默认按钮）

MessageDialogDefaultButtons 章节

    type MessageDialogDefaultButtons: "Ok" | "OkCancel" | "YesNo" | "YesNoCancel";

消息对话框的默认按钮。

#### 始于

名为“起始版本”的部分

2.4.0

**源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L154>

* * *

### MessageDialogResult（消息对话框结果）

MessageDialogResult 章节

    type MessageDialogResult:

      | "Yes"

      | "No"

      | "Ok"

      | "Cancel"

      | string & object;

消息对话框的结果。

如果对话框具有自定义按钮，则结果为字符串；否则，它将是默认按钮之一。

#### 始于

名为“起始版本”的部分

2.4.0

**源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L406>

* * *

### OpenDialogReturn<T>（打开对话框返回值）

OpenDialogReturn<T> 章节

    type OpenDialogReturn<T>: T["directory"] extends true ? T["multiple"] extends true ? string[] | null : string | null : T["multiple"] extends true ? string[] | null : string | null;

#### 类型参数

名为“类型参数”的部分

类型参数
---
`T` _继承自_ [`OpenDialogOptions`](/reference/javascript/dialog/#opendialogoptions)

**源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L296>

* * *

### PickerMode（选择器模式）

PickerMode 章节

    type PickerMode: "document" | "media" | "image" | "video";

对话框的首选模式。这主要针对移动平台（iOS 和 Android），它们拥有独立的文件选择器和媒体选择器。在桌面上，此选项会被忽略。如果未提供，对话框将根据筛选器的 MIME 类型或扩展名自动选择最佳模式。

**注意：** 此选项仅在 iOS 14 及更高版本上支持。在 iOS 13 及以下版本中，该参数会被忽略。

**源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L137>

## 函数

名为“函数”的部分

### ask()（询问）

ask() 章节

    function ask(message, options?): Promise<boolean>

显示一个带有“是”和“否”按钮的询问对话框。

这是一个便捷包装器，等同于 `await message('msg', { buttons: 'YesNo' }) === 'Yes'`

#### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`消息`| `string`| 要显示的消息。
`选项`?| `string` | [`ConfirmDialogOptions`](/reference/javascript/dialog/#confirmdialogoptions)| 对话框选项。如果是字符串，则表示对话框标题。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`boolean`>

一个 Promise，解析为布尔值，指示是否点击了“是”。

#### 示例

标题为“Example”的章节

    import { ask } from '@tauri-apps/plugin-dialog';

    const yes = await ask('Are you sure?', 'Tauri');

    const yes2 = await ask('This action cannot be reverted. Are you sure?', { title: 'Tauri', kind: 'warning' });

#### 始于

名为“起始版本”的部分

2.0.0

**源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L467>

* * *

### confirm()（确认）

confirm() 章节

    function confirm(message, options?): Promise<boolean>

显示一个带有“确定”和“取消”按钮的询问对话框。

这是一个便捷包装器，等同于 `await message('msg', { buttons: 'OkCancel' }) === 'Ok'`

#### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`消息`| `string`| 要显示的消息。
`选项`?| `string` | [`ConfirmDialogOptions`](/reference/javascript/dialog/#confirmdialogoptions)| 对话框选项。如果是字符串，则表示对话框标题。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`boolean`>

一个 Promise，解析为布尔值，指示是否点击了“确定”。

#### 示例

标题为“Example”的章节

    import { confirm } from '@tauri-apps/plugin-dialog';

    const confirmed = await confirm('Are you sure?', 'Tauri');

    const confirmed2 = await confirm('This action cannot be reverted. Are you sure?', { title: 'Tauri', kind: 'warning' });

#### 始于

名为“起始版本”的部分

2.0.0

**源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L504>

* * *

### message()（消息）

message() 章节

    function message(message, options?): Promise<MessageDialogResult>

显示一个带有“确定”按钮的消息对话框。

#### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`消息`| `string`| 要显示的消息。
`选项`?| `string` | [`MessageDialogOptions`](/reference/javascript/dialog/#messagedialogoptions)| 对话框选项。如果是字符串，则表示对话框标题。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`MessageDialogResult`](/reference/javascript/dialog/#messagedialogresult)>

表示操作成功或失败的 Promise。

#### 示例

标题为“Example”的章节

    import { message } from '@tauri-apps/plugin-dialog';

    await message('Tauri is awesome', 'Tauri');

    await message('File not found', { title: 'Tauri', kind: 'error' });

#### 始于

名为“起始版本”的部分

2.0.0

**源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L437>

* * *

### open()

名为“open()”的章节

    function open<T>(options): Promise<OpenDialogReturn<T>>

打开文件/目录选择对话框。

选定的路径会被添加到文件系统和资源协议的访问作用域中。当安全性比本 API 的易用性更重要时，请优先编写专用的命令。

注意，作用域更改不会持久保存，因此当应用程序重启时，这些值会被清除。可以使用 [tauri-plugin-persisted-scope](https://github.com/tauri-apps/tauri-plugin-persisted-scope) 将其保存到文件系统中。

#### 类型参数

名为“类型参数”的部分

类型参数
---
`T` _继承自_ [`OpenDialogOptions`](/reference/javascript/dialog/#opendialogoptions)

#### 参数

名为“参数”的部分

参数| 类型
---|---
`选项`| `T`

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`OpenDialogReturn`](/reference/javascript/dialog/#opendialogreturnt)<`T`>>

一个解析为所选路径的 Promise。

#### 示例

名为“示例”的章节

    import { open } from '@tauri-apps/plugin-dialog';

    // Open a selection dialog for image files

    const selected = await open({

      multiple: true,

      filters: [{

        name: 'Image',

        extensions: ['png', 'jpeg']

      }]

    });

    if (Array.isArray(selected)) {

      // user selected multiple files

    } else if (selected === null) {

      // user cancelled the selection

    } else {

      // user selected a single file

    }

    import { open } from '@tauri-apps/plugin-dialog';

    import { appDir } from '@tauri-apps/api/path';

    // Open a selection dialog for directories

    const selected = await open({

      directory: true,

      multiple: true,

      defaultPath: await appDir(),

    });

    if (Array.isArray(selected)) {

      // user selected multiple directories

    } else if (selected === null) {

      // user cancelled the selection

    } else {

      // user selected a single directory

    }

#### 始于

名为“起始版本”的部分

2.0.0

**源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L356>

* * *

### save()

save() 章节

    function save(options): Promise<string | null>

打开文件/目录保存对话框。

选定的路径会被添加到文件系统和资源协议的访问作用域中。当安全性比本 API 的易用性更重要时，请优先编写专用的命令。

注意，作用域更改不会持久保存，因此当应用程序重启时，这些值会被清除。可以使用 [tauri-plugin-persisted-scope](https://github.com/tauri-apps/tauri-plugin-persisted-scope) 将其保存到文件系统中。

#### 参数

名为“参数”的部分

参数| 类型
---|---
`选项`| [`SaveDialogOptions（保存对话框选项）`](/reference/javascript/dialog/#savedialogoptions)

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`string` | `null`>

一个解析为所选路径的 Promise。

#### 示例

标题为“Example”的章节

    import { save } from '@tauri-apps/plugin-dialog';

    const filePath = await save({

      filters: [{

        name: 'Image',

        extensions: ['png', 'jpeg']

      }]

    });

#### 始于

名为“起始版本”的部分

2.0.0

**源文件** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L390>