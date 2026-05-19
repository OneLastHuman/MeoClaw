# dpi

_Source: https://v2.tauri.org.cn/reference/javascript/api/namespacedpi/_

## 类

名为“类”的部分

### LogicalPosition

标题为“LogicalPosition”的章节

以逻辑像素表示的坐标。有关逻辑像素的解释，请参见 [`LogicalSize`](/reference/javascript/api/namespacedpi/#logicalsize) 的描述。

#### 始于

名为“起始版本”的部分

2.0.0

#### 构造函数

名为“构造函数”的部分

##### new LogicalPosition()

标题为“new LogicalPosition()”的章节

    new LogicalPosition(x, y): LogicalPosition

###### 参数

名为“参数”的部分

参数| 类型
---|---
`x`| `数字`
`y`| `数字`

###### 返回

名为“返回值”的部分

[`LogicalPosition`](/reference/javascript/api/namespacedpi/#logicalposition)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L219>

##### new LogicalPosition()

标题为“new LogicalPosition()”的章节

    new LogicalPosition(object): LogicalPosition

###### 参数

名为“参数”的部分

参数| 类型
---|---
`对象`| `对象`
`object.Logical`| `对象`
`object.Logical.x`| `数字`
`object.Logical.y`| `数字`

###### 返回

名为“返回值”的部分

[`LogicalPosition`](/reference/javascript/api/namespacedpi/#logicalposition)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L220>

##### new LogicalPosition()

标题为“new LogicalPosition()”的章节

    new LogicalPosition(object): LogicalPosition

###### 参数

名为“参数”的部分

参数| 类型
---|---
`对象`| `对象`
`object.x`| `数字`
`object.y`| `数字`

###### 返回

名为“返回值”的部分

[`LogicalPosition`](/reference/javascript/api/namespacedpi/#logicalposition)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L221>

#### 属性

名为“属性”的部分

属性| 修饰符| 类型| 默认值| 定义于
---|---|---|---|---
`type`| `readonly`| `"Logical"`| `'Logical'`| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L215>
`x`| `public`| `数字`| `未定义`| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L216>
`y`| `public`| `数字`| `未定义`| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L217>

#### 方法

名为“方法”的部分

##### __TAURI_TO_IPC_KEY__()

标题为“__TAURI_TO_IPC_KEY__()”的章节

    __TAURI_TO_IPC_KEY__(): object

###### 返回

名为“返回值”的部分

`对象`

名称| 类型| 定义于
---|---|---
`x`| `数字`| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L263>
`y`| `数字`| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L264>

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L261>

##### toJSON()

标题为“toJSON()”的章节

    toJSON(): object

###### 返回

名为“返回值”的部分

`对象`

名称| 类型| 定义于
---|---|---
`x`| `数字`| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L263>
`y`| `数字`| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L264>

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L268>

##### toPhysical()

标题为“toPhysical()”的章节

    toPhysical(scaleFactor): PhysicalPosition

将逻辑坐标转换为物理坐标。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`缩放因子`| `数字`

###### 返回

名为“返回值”的部分

[`PhysicalPosition`](/reference/javascript/api/namespacedpi/#physicalposition)

###### 示例

标题为“Example”的章节

    import { LogicalPosition } from '@tauri-apps/api/dpi';

    import { getCurrentWindow } from '@tauri-apps/api/window';

    const appWindow = getCurrentWindow();

    const factor = await appWindow.scaleFactor();

    const position = new LogicalPosition(400, 500);

    const physical = position.toPhysical(factor);

###### 始于

名为“起始版本”的部分

2.0.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L257>

* * *

### LogicalSize

标题为“LogicalSize”的章节

以逻辑像素表示的尺寸。逻辑像素根据窗口的 DPI 缩放比例进行调整。大多数浏览器 API（例如 `MouseEvent` 的 `clientX`）返回的都是逻辑像素。

有关基于逻辑像素的坐标，请参见 [`LogicalPosition`](/reference/javascript/api/namespacedpi/#logicalposition)。

#### 始于

名为“起始版本”的部分

2.0.0

#### 构造函数

名为“构造函数”的部分

##### new LogicalSize()

标题为“new LogicalSize()”的章节

    new LogicalSize(width, height): LogicalSize

###### 参数

名为“参数”的部分

参数| 类型
---|---
`宽度`| `数字`
`高度`| `数字`

###### 返回

名为“返回值”的部分

[`LogicalSize`](/reference/javascript/api/namespacedpi/#logicalsize)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L21>

##### new LogicalSize()

标题为“new LogicalSize()”的章节

    new LogicalSize(object): LogicalSize

###### 参数

名为“参数”的部分

参数| 类型
---|---
`对象`| `对象`
`object.Logical`| `对象`
`object.Logical.height`| `数字`
`object.Logical.width`| `数字`

###### 返回

名为“返回值”的部分

[`LogicalSize`](/reference/javascript/api/namespacedpi/#logicalsize)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L22>

##### new LogicalSize()

标题为“new LogicalSize()”的章节

    new LogicalSize(object): LogicalSize

###### 参数

名为“参数”的部分

参数| 类型
---|---
`对象`| `对象`
`object.height`| `数字`
`object.width`| `数字`

###### 返回

名为“返回值”的部分

[`LogicalSize`](/reference/javascript/api/namespacedpi/#logicalsize)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L23>

#### 属性

名为“属性”的部分

属性| 修饰符| 类型| 默认值| 定义于
---|---|---|---|---
`height`| `public`| `数字`| `未定义`| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L19>
`type`| `readonly`| `"Logical"`| `'Logical'`| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L17>
`width`| `public`| `数字`| `未定义`| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L18>

#### 方法

名为“方法”的部分

##### __TAURI_TO_IPC_KEY__()

标题为“__TAURI_TO_IPC_KEY__()”的章节

    __TAURI_TO_IPC_KEY__(): object

###### 返回

名为“返回值”的部分

`对象`

名称| 类型| 定义于
---|---|---
`高度`| `数字`| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L66>
`宽度`| `数字`| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L65>

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L63>

##### toJSON()

标题为“toJSON()”的章节

    toJSON(): object

###### 返回

名为“返回值”的部分

`对象`

名称| 类型| 定义于
---|---|---
`高度`| `数字`| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L66>
`宽度`| `数字`| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L65>

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L70>

##### toPhysical()

标题为“toPhysical()”的章节

    toPhysical(scaleFactor): PhysicalSize

将逻辑尺寸转换为物理尺寸。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`缩放因子`| `数字`

###### 返回

名为“返回值”的部分

[`PhysicalSize`](/reference/javascript/api/namespacedpi/#physicalsize)

###### 示例

标题为“Example”的章节

    import { LogicalSize } from '@tauri-apps/api/dpi';

    import { getCurrentWindow } from '@tauri-apps/api/window';

    const appWindow = getCurrentWindow();

    const factor = await appWindow.scaleFactor();

    const size = new LogicalSize(400, 500);

    const physical = size.toPhysical(factor);

###### 始于

名为“起始版本”的部分

2.0.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L59>

* * *

### PhysicalPosition

标题为“PhysicalPosition”的章节

以物理像素表示的坐标。

有关物理像素的解释，请参见 [`PhysicalSize`](/reference/javascript/api/namespacedpi/#physicalsize) 的描述。

#### 始于

名为“起始版本”的部分

2.0.0

#### 构造函数

名为“构造函数”的部分

##### new PhysicalPosition()

标题为“new PhysicalPosition()”的章节

    new PhysicalPosition(x, y): PhysicalPosition

###### 参数

名为“参数”的部分

参数| 类型
---|---
`x`| `数字`
`y`| `数字`

###### 返回

名为“返回值”的部分

[`PhysicalPosition`](/reference/javascript/api/namespacedpi/#physicalposition)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L286>

##### new PhysicalPosition()

标题为“new PhysicalPosition()”的章节

    new PhysicalPosition(object): PhysicalPosition

###### 参数

名为“参数”的部分

参数| 类型
---|---
`对象`| `对象`
`object.Physical`| `对象`
`object.Physical.x`| `数字`
`object.Physical.y`| `数字`

###### 返回

名为“返回值”的部分

[`PhysicalPosition`](/reference/javascript/api/namespacedpi/#physicalposition)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L287>

##### new PhysicalPosition()

标题为“new PhysicalPosition()”的章节

    new PhysicalPosition(object): PhysicalPosition

###### 参数

名为“参数”的部分

参数| 类型
---|---
`对象`| `对象`
`object.x`| `数字`
`object.y`| `数字`

###### 返回

名为“返回值”的部分

[`PhysicalPosition`](/reference/javascript/api/namespacedpi/#physicalposition)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L288>

#### 属性

名为“属性”的部分

属性| 修饰符| 类型| 默认值| 定义于
---|---|---|---|---
`type`| `readonly`| `"Physical"`| `'Physical'`| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L282>
`x`| `public`| `数字`| `未定义`| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L283>
`y`| `public`| `数字`| `未定义`| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L284>

#### 方法

名为“方法”的部分

##### __TAURI_TO_IPC_KEY__()

标题为“__TAURI_TO_IPC_KEY__()”的章节

    __TAURI_TO_IPC_KEY__(): object

###### 返回

名为“返回值”的部分

`对象`

名称| 类型| 定义于
---|---|---
`x`| `数字`| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L330>
`y`| `数字`| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L331>

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L328>

##### toJSON()

标题为“toJSON()”的章节

    toJSON(): object

###### 返回

名为“返回值”的部分

`对象`

名称| 类型| 定义于
---|---|---
`x`| `数字`| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L330>
`y`| `数字`| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L331>

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L335>

##### toLogical()

标题为“toLogical()”的章节

    toLogical(scaleFactor): LogicalPosition

将物理坐标转换为逻辑坐标。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`缩放因子`| `数字`

###### 返回

名为“返回值”的部分

[`LogicalPosition`](/reference/javascript/api/namespacedpi/#logicalposition)

###### 示例

标题为“Example”的章节

    import { PhysicalPosition } from '@tauri-apps/api/dpi';

    import { getCurrentWindow } from '@tauri-apps/api/window';

    const appWindow = getCurrentWindow();

    const factor = await appWindow.scaleFactor();

    const position = new PhysicalPosition(400, 500);

    const physical = position.toLogical(factor);

###### 始于

名为“起始版本”的部分

2.0.0

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L324>

* * *

### PhysicalSize

标题为“PhysicalSize”的章节

以物理像素表示的尺寸。

物理像素代表实际的屏幕像素，且与 DPI 无关。对于高 DPI 窗口，这意味着窗口屏幕上的任何点在逻辑像素 [`LogicalSize`](/reference/javascript/api/namespacedpi/#logicalsize) 中都会有不同的位置。

有关基于物理像素的坐标，请参见 [`PhysicalPosition`](/reference/javascript/api/namespacedpi/#physicalposition)。

#### 始于

名为“起始版本”的部分

2.0.0

#### 构造函数

名为“构造函数”的部分

##### new PhysicalSize()

标题为“new PhysicalSize()”的章节

    new PhysicalSize(width, height): PhysicalSize

###### 参数

名为“参数”的部分

参数| 类型
---|---
`宽度`| `数字`
`高度`| `数字`

###### 返回

名为“返回值”的部分

[`PhysicalSize`](/reference/javascript/api/namespacedpi/#physicalsize)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L92>

##### new PhysicalSize()

标题为“new PhysicalSize()”的章节

    new PhysicalSize(object): PhysicalSize

###### 参数

名为“参数”的部分

参数| 类型
---|---
`对象`| `对象`
`object.Physical`| `对象`
`object.Physical.height`| `数字`
`object.Physical.width`| `数字`

###### 返回

名为“返回值”的部分

[`PhysicalSize`](/reference/javascript/api/namespacedpi/#physicalsize)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L93>

##### new PhysicalSize()

标题为“new PhysicalSize()”的章节

    new PhysicalSize(object): PhysicalSize

###### 参数

名为“参数”的部分

参数| 类型
---|---
`对象`| `对象`
`object.height`| `数字`
`object.width`| `数字`

###### 返回

名为“返回值”的部分

[`PhysicalSize`](/reference/javascript/api/namespacedpi/#physicalsize)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L94>

#### 属性

名为“属性”的部分

属性| 修饰符| 类型| 默认值| 定义于
---|---|---|---|---
`height`| `public`| `数字`| `未定义`| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L90>
`type`| `readonly`| `"Physical"`| `'Physical'`| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L88>
`width`| `public`| `数字`| `未定义`| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L89>

#### 方法

名为“方法”的部分

##### __TAURI_TO_IPC_KEY__()

标题为“__TAURI_TO_IPC_KEY__()”的章节

    __TAURI_TO_IPC_KEY__(): object

###### 返回

名为“返回值”的部分

`对象`

名称| 类型| 定义于
---|---|---
`高度`| `数字`| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L133>
`宽度`| `数字`| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L132>

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L130>

##### toJSON()

标题为“toJSON()”的章节

    toJSON(): object

###### 返回

名为“返回值”的部分

`对象`

名称| 类型| 定义于
---|---|---
`高度`| `数字`| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L133>
`宽度`| `数字`| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L132>

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L137>

##### toLogical()

标题为“toLogical()”的章节

    toLogical(scaleFactor): LogicalSize

将物理尺寸转换为逻辑尺寸。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`缩放因子`| `数字`

###### 返回

名为“返回值”的部分

[`LogicalSize`](/reference/javascript/api/namespacedpi/#logicalsize)

###### 示例

标题为“Example”的章节

    import { getCurrentWindow } from '@tauri-apps/api/window';

    const appWindow = getCurrentWindow();

    const factor = await appWindow.scaleFactor();

    const size = await appWindow.innerSize(); // PhysicalSize

    const logical = size.toLogical(factor);

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L126>

* * *

### Position

标题为“Position”的章节

以物理像素或逻辑像素表示的坐标。

此类型实际上是 [`LogicalSize`](/reference/javascript/api/namespacedpi/#logicalsize) 和 [`PhysicalSize`](/reference/javascript/api/namespacedpi/#physicalsize) 的联合类型。在 Rust 中将 `tauri::Position` 用作命令参数时，它非常有用，因为此类会自动序列化为有效格式，以便正确反序列化为 `tauri::Position`。

因此，无需使用

    import { invoke } from '@tauri-apps/api/core';

    import { LogicalPosition, PhysicalPosition } from '@tauri-apps/api/dpi';

    const position: LogicalPosition | PhysicalPosition = someFunction(); // where someFunction returns either LogicalPosition or PhysicalPosition

    const validPosition = position instanceof LogicalPosition

      ? { Logical: { x: position.x, y: position.y } }

      : { Physical: { x: position.x, y: position.y } }

    await invoke("do_something_with_position", { position: validPosition });

您可以直接使用 [`Position`](/reference/javascript/api/namespacedpi/#position)

    import { invoke } from '@tauri-apps/api/core';

    import { LogicalPosition, PhysicalPosition, Position } from '@tauri-apps/api/dpi';

    const position: LogicalPosition | PhysicalPosition = someFunction(); // where someFunction returns either LogicalPosition or PhysicalPosition

    const validPosition = new Position(position);

    await invoke("do_something_with_position", { position: validPosition });

#### 始于

名为“起始版本”的部分

2.1.0

#### 构造函数

名为“构造函数”的部分

##### new Position()

标题为“new Position()”的章节

    new Position(position): Position

###### 参数

名为“参数”的部分

参数| 类型
---|---
`位置`| [`LogicalPosition`](/reference/javascript/api/namespacedpi/#logicalposition) | [`PhysicalPosition`](/reference/javascript/api/namespacedpi/#physicalposition)

###### 返回

名为“返回值”的部分

[`Position`](/reference/javascript/api/namespacedpi/#position)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L375>

#### 属性

名为“属性”的部分

属性| 类型| 定义于
---|---|---
`position`| [`LogicalPosition`](/reference/javascript/api/namespacedpi/#logicalposition) | [`PhysicalPosition`](/reference/javascript/api/namespacedpi/#physicalposition)| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L373>

#### 方法

名为“方法”的部分

##### __TAURI_TO_IPC_KEY__()

标题为“__TAURI_TO_IPC_KEY__()”的章节

    __TAURI_TO_IPC_KEY__(): object

###### 返回

名为“返回值”的部分

`对象`

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L391>

##### toJSON()

标题为“toJSON()”的章节

    toJSON(): object

###### 返回

名为“返回值”的部分

`对象`

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L400>

##### toLogical()

标题为“toLogical()”的章节

    toLogical(scaleFactor): LogicalPosition

###### 参数

名为“参数”的部分

参数| 类型
---|---
`缩放因子`| `数字`

###### 返回

名为“返回值”的部分

[`LogicalPosition`](/reference/javascript/api/namespacedpi/#logicalposition)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L379>

##### toPhysical()

标题为“toPhysical()”的章节

    toPhysical(scaleFactor): PhysicalPosition

###### 参数

名为“参数”的部分

参数| 类型
---|---
`缩放因子`| `数字`

###### 返回

名为“返回值”的部分

[`PhysicalPosition`](/reference/javascript/api/namespacedpi/#physicalposition)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L385>

* * *

### Size

标题为“Size”的章节

以物理像素或逻辑像素表示的尺寸。

此类型实际上是 [`LogicalSize`](/reference/javascript/api/namespacedpi/#logicalsize) 和 [`PhysicalSize`](/reference/javascript/api/namespacedpi/#physicalsize) 的联合类型。在 Rust 中将 `tauri::Size` 用作命令参数时，它非常有用，因为此类会自动序列化为有效格式，以便正确反序列化为 `tauri::Size`。

因此，无需使用

    import { invoke } from '@tauri-apps/api/core';

    import { LogicalSize, PhysicalSize } from '@tauri-apps/api/dpi';

    const size: LogicalSize | PhysicalSize = someFunction(); // where someFunction returns either LogicalSize or PhysicalSize

    const validSize = size instanceof LogicalSize

      ? { Logical: { width: size.width, height: size.height } }

      : { Physical: { width: size.width, height: size.height } }

    await invoke("do_something_with_size", { size: validSize });

您可以直接使用 [`Size`](/reference/javascript/api/namespacedpi/#size)

    import { invoke } from '@tauri-apps/api/core';

    import { LogicalSize, PhysicalSize, Size } from '@tauri-apps/api/dpi';

    const size: LogicalSize | PhysicalSize = someFunction(); // where someFunction returns either LogicalSize or PhysicalSize

    const validSize = new Size(size);

    await invoke("do_something_with_size", { size: validSize });

#### 始于

名为“起始版本”的部分

2.1.0

#### 构造函数

名为“构造函数”的部分

##### new Size()

标题为“new Size()”的章节

    new Size(size): Size

###### 参数

名为“参数”的部分

参数| 类型
---|---
`尺寸`| [`LogicalSize`](/reference/javascript/api/namespacedpi/#logicalsize) | [`PhysicalSize`](/reference/javascript/api/namespacedpi/#physicalsize)

###### 返回

名为“返回值”的部分

[`Size`](/reference/javascript/api/namespacedpi/#size)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L177>

#### 属性

名为“属性”的部分

属性| 类型| 定义于
---|---|---
`size`| [`LogicalSize`](/reference/javascript/api/namespacedpi/#logicalsize) | [`PhysicalSize`](/reference/javascript/api/namespacedpi/#physicalsize)| **源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L175>

#### 方法

名为“方法”的部分

##### __TAURI_TO_IPC_KEY__()

标题为“__TAURI_TO_IPC_KEY__()”的章节

    __TAURI_TO_IPC_KEY__(): object

###### 返回

名为“返回值”的部分

`对象`

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L193>

##### toJSON()

标题为“toJSON()”的章节

    toJSON(): object

###### 返回

名为“返回值”的部分

`对象`

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L202>

##### toLogical()

标题为“toLogical()”的章节

    toLogical(scaleFactor): LogicalSize

###### 参数

名为“参数”的部分

参数| 类型
---|---
`缩放因子`| `数字`

###### 返回

名为“返回值”的部分

[`LogicalSize`](/reference/javascript/api/namespacedpi/#logicalsize)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L181>

##### toPhysical()

标题为“toPhysical()”的章节

    toPhysical(scaleFactor): PhysicalSize

###### 参数

名为“参数”的部分

参数| 类型
---|---
`缩放因子`| `数字`

###### 返回

名为“返回值”的部分

[`PhysicalSize`](/reference/javascript/api/namespacedpi/#physicalsize)

**源码** : <https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/dpi.ts#L187>