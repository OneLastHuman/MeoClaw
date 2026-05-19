# @tauri-apps/plugin-store

_Source: https://v2.tauri.org.cn/reference/javascript/store/_

## 类

名为“类”的部分

### LazyStore

标题为“LazyStore”的章节

由后端层持久化的惰性加载键值存储。

#### 实现

标题为“实现”的章节

  * `IStore`

#### 构造函数

名为“构造函数”的部分

##### new LazyStore()

标题为“new LazyStore()”的章节

    new LazyStore(path, options?): LazyStore

请注意，如果已经有其他地方创建了存储，则选项不会生效。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`path`| `string`| 在 `app_data_dir` 中保存存储的路径。
`选项`?| [`StoreOptions`](/reference/javascript/store/#storeoptions)| 存储配置选项。

###### 返回

名为“返回值”的部分

[`LazyStore`](/reference/javascript/store/#lazystore)

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L104>

#### 方法

名为“方法”的部分

##### clear()

标题为“clear()”的章节

    clear(): Promise<void>

清除存储，删除所有键值对。

注意：要清除存储并将其重置为 `默认` 值，请使用 [`reset`](/reference/javascript/store/#reset)。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

###### 实现自

标题为“实现自”的章节

`IStore.clear`

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L132>

##### close()

名为“close()”的部分

    close(): Promise<void>

关闭存储并从内存中清除此资源。**你不应再调用此对象上的任何方法，并应丢弃对其的所有引用。**

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

###### 实现自

标题为“实现自”的章节

`IStore.close`

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L177>

##### delete()

标题为“delete()”的章节

    delete(key): Promise<boolean>

从存储中移除一个键值对。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`key`| `string`|

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`boolean`>

###### 实现自

标题为“实现自”的章节

`IStore.delete`

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L128>

##### entries()

标题为“entries()”的章节

    entries<T>(): Promise<[string, T][]>

返回存储中所有条目的列表。

###### 类型参数

名为“类型参数”的部分

类型参数
---
`T`

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`string`, `T`][]>

###### 实现自

标题为“实现自”的章节

`IStore.entries`

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L148>

##### get()

名为“get()”的章节

    get<T>(key): Promise<undefined | T>

返回给定 `key` 的值，如果该键不存在，则返回 `undefined`。

###### 类型参数

名为“类型参数”的部分

类型参数
---
`T`

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`key`| `string`|

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`undefined` | `T`>

###### 实现自

标题为“实现自”的章节

`IStore.get`

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L120>

##### has()

标题为“has()”的章节

    has(key): Promise<boolean>

如果给定的 `key` 存在于存储中，则返回 `true`。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`key`| `string`|

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`boolean`>

###### 实现自

标题为“实现自”的章节

`IStore.has`

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L124>

##### init()

标题为“init()”的章节

    init(): Promise<void>

如果尚未加载，则初始化/加载存储。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L112>

##### keys()

标题为“keys()”的章节

    keys(): Promise<string[]>

返回存储中所有键的列表。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`string`[]>

###### 实现自

标题为“实现自”的章节

`IStore.keys`

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L140>

##### length()

标题为“length()”的章节

    length(): Promise<number>

返回存储中的键值对数量。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`number`>

###### 实现自

标题为“实现自”的章节

`IStore.length`

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L152>

##### onChange()

标题为“onChange()”的章节

    onChange<T>(cb): Promise<UnlistenFn>

监听存储的变化。

###### 类型参数

名为“类型参数”的部分

类型参数
---
`T`

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`cb`| (`key`, `value`) => `void`|

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`UnlistenFn`>

一个解析为取消监听事件函数的 Promise。

###### 始于

名为“起始版本”的部分

2.0.0

###### 实现自

标题为“实现自”的章节

`IStore.onChange`

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L171>

##### onKeyChange()

标题为“onKeyChange()”的章节

    onKeyChange<T>(key, cb): Promise<UnlistenFn>

监听存储键的变化。

###### 类型参数

名为“类型参数”的部分

类型参数
---
`T`

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`key`| `string`|
`cb`| (`value`) => `void`|

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`UnlistenFn`>

一个解析为取消监听事件函数的 Promise。

###### 始于

名为“起始版本”的部分

2.0.0

###### 实现自

标题为“实现自”的章节

`IStore.onKeyChange`

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L164>

##### reload()

标题为“reload()”的章节

    reload(options?): Promise<void>

尝试将存储 `path` 下的磁盘状态加载到内存中。

如果用户编辑了磁盘上的状态，并且你想同步这些更改，此方法很有用。

注意

  * 此方法加载数据并将其与当前存储合并。在 v3 中，此行为将更改为先重置为默认值，然后再与磁盘状态合并。要使存储与磁盘状态完全匹配，请将 [`ignoreDefaults`](/reference/javascript/store/#reloadoptions) 设置为 `true`。
  * 此方法不会触发更改事件。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`选项`?| [`ReloadOptions`](/reference/javascript/store/#reloadoptions)

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

###### 实现自

标题为“实现自”的章节

`IStore.reload`

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L156>

##### reset()

标题为“reset()”的章节

    reset(): Promise<void>

将存储重置为其 `默认` 值。

如果没有设置默认值，此方法的行为与 [`clear`](/reference/javascript/store/#clear) 相同。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

###### 实现自

标题为“实现自”的章节

`IStore.reset`

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L136>

##### save()

save() 章节

    save(): Promise<void>

将存储保存到磁盘上的 `path`。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

###### 实现自

标题为“实现自”的章节

`IStore.save`

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L160>

##### set()

标题为“set()”的章节

    set(key, value): Promise<void>

将键值对插入到存储中。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`key`| `string`|
`value`| `未知`|

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

###### 实现自

标题为“实现自”的章节

`IStore.set`

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L116>

##### values()

标题为“values()”的章节

    values<T>(): Promise<T[]>

返回存储中所有值的列表。

###### 类型参数

名为“类型参数”的部分

类型参数
---
`T`

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`T`[]>

###### 实现自

标题为“实现自”的章节

`IStore.values`

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L144>

* * *

### 存储 (Store)

标题为“Store”的章节

由后端层持久化的键值存储。

#### 继承 (Extends)

名为“继承自”的章节

  * `Resource`

#### 实现

标题为“实现”的章节

  * `IStore`

#### 访问器

名为“访问器”的部分

##### rid

名为“rid”的部分

    get rid(): number

###### 返回

名为“返回值”的部分

`数字`

###### 继承自 (Inherited from)

名为“继承自”的章节

`Resource.rid`

**来源** : 未定义

#### 方法

名为“方法”的部分

##### clear()

标题为“clear()”的章节

    clear(): Promise<void>

清除存储，删除所有键值对。

注意：要清除存储并将其重置为 `默认` 值，请使用 [`reset`](/reference/javascript/store/#reset-1)。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

###### 实现自

标题为“实现自”的章节

`IStore.clear`

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L267>

##### close()

名为“close()”的部分

    close(): Promise<void>

销毁并从内存中清理此资源。**您不应再对此对象调用任何方法，并应放弃对它的任何引用。**

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

###### 实现自

标题为“实现自”的章节

`IStore.close`

###### 继承自 (Inherited from)

名为“继承自”的章节

`Resource.close`

**来源** : 未定义

##### delete()

标题为“delete()”的章节

    delete(key): Promise<boolean>

从存储中移除一个键值对。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`key`| `string`|

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`boolean`>

###### 实现自

标题为“实现自”的章节

`IStore.delete`

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L260>

##### entries()

标题为“entries()”的章节

    entries<T>(): Promise<[string, T][]>

返回存储中所有条目的列表。

###### 类型参数

名为“类型参数”的部分

类型参数
---
`T`

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`string`, `T`][]>

###### 实现自

标题为“实现自”的章节

`IStore.entries`

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L283>

##### get()

名为“get()”的章节

    get<T>(key): Promise<undefined | T>

返回给定 `key` 的值，如果该键不存在，则返回 `undefined`。

###### 类型参数

名为“类型参数”的部分

类型参数
---
`T`

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`key`| `string`|

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`undefined` | `T`>

###### 实现自

标题为“实现自”的章节

`IStore.get`

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L245>

##### has()

标题为“has()”的章节

    has(key): Promise<boolean>

如果给定的 `key` 存在于存储中，则返回 `true`。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`key`| `string`|

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`boolean`>

###### 实现自

标题为“实现自”的章节

`IStore.has`

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L253>

##### keys()

标题为“keys()”的章节

    keys(): Promise<string[]>

返回存储中所有键的列表。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`string`[]>

###### 实现自

标题为“实现自”的章节

`IStore.keys`

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L275>

##### length()

标题为“length()”的章节

    length(): Promise<number>

返回存储中的键值对数量。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`number`>

###### 实现自

标题为“实现自”的章节

`IStore.length`

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L287>

##### onChange()

标题为“onChange()”的章节

    onChange<T>(cb): Promise<UnlistenFn>

监听存储的变化。

###### 类型参数

名为“类型参数”的部分

类型参数
---
`T`

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`cb`| (`key`, `value`) => `void`|

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`UnlistenFn`>

一个解析为取消监听事件函数的 Promise。

###### 始于

名为“起始版本”的部分

2.0.0

###### 实现自

标题为“实现自”的章节

`IStore.onChange`

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L310>

##### onKeyChange()

标题为“onKeyChange()”的章节

    onKeyChange<T>(key, cb): Promise<UnlistenFn>

监听存储键的变化。

###### 类型参数

名为“类型参数”的部分

类型参数
---
`T`

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`key`| `string`|
`cb`| (`value`) => `void`|

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`UnlistenFn`>

一个解析为取消监听事件函数的 Promise。

###### 始于

名为“起始版本”的部分

2.0.0

###### 实现自

标题为“实现自”的章节

`IStore.onKeyChange`

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L299>

##### reload()

标题为“reload()”的章节

    reload(options?): Promise<void>

尝试将存储 `path` 下的磁盘状态加载到内存中。

如果用户编辑了磁盘上的状态，并且你想同步这些更改，此方法很有用。

注意

  * 此方法加载数据并将其与当前存储合并。在 v3 中，此行为将更改为先重置为默认值，然后再与磁盘状态合并。要使存储与磁盘状态完全匹配，请将 [`ignoreDefaults`](/reference/javascript/store/#reloadoptions) 设置为 `true`。
  * 此方法不会触发更改事件。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`选项`?| [`ReloadOptions`](/reference/javascript/store/#reloadoptions)

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

###### 实现自

标题为“实现自”的章节

`IStore.reload`

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L291>

##### reset()

标题为“reset()”的章节

    reset(): Promise<void>

将存储重置为其 `默认` 值。

如果没有设置默认值，此方法的行为与 [`clear`](/reference/javascript/store/#clear-1) 相同。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

###### 实现自

标题为“实现自”的章节

`IStore.reset`

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L271>

##### save()

save() 章节

    save(): Promise<void>

将存储保存到磁盘上的 `path`。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

###### 实现自

标题为“实现自”的章节

`IStore.save`

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L295>

##### set()

标题为“set()”的章节

    set(key, value): Promise<void>

将键值对插入到存储中。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`key`| `string`|
`value`| `未知`|

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

###### 实现自

标题为“实现自”的章节

`IStore.set`

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L237>

##### values()

标题为“values()”的章节

    values<T>(): Promise<T[]>

返回存储中所有值的列表。

###### 类型参数

名为“类型参数”的部分

类型参数
---
`T`

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`T`[]>

###### 实现自

标题为“实现自”的章节

`IStore.values`

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L279>

##### get()

名为“get()”的章节

    static get(path): Promise<null | Store>

获取一个已经加载的存储。

如果存储未加载，则返回 `null`。在这种情况下，你必须 [加载](/reference/javascript/store/#load) 它。

当你知道存储已经加载并且只需要访问其实例时，此函数更有用。否则，请优先使用 [Store.load](/reference/javascript/store/#load)。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`path`| `string`| 存储的路径。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`null` | [`Store`](/reference/javascript/store/#store)>

###### 示例

标题为“Example”的章节

    import { Store } from '@tauri-apps/api/store';

    let store = await Store.get('store.json');

    if (!store) {

      store = await Store.load('store.json');

    }

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L231>

##### load()

标题为“load()”的章节

    static load(path, options?): Promise<Store>

创建一个新的存储，或加载带有该路径的现有存储。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`path`| `string`| 在 `app_data_dir` 中保存存储的路径。
`选项`?| [`StoreOptions`](/reference/javascript/store/#storeoptions)| 存储配置选项。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`Store`](/reference/javascript/store/#store)>

###### 示例

标题为“Example”的章节

    import { Store } from '@tauri-apps/api/store';

    const store = await Store.load('store.json');

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L204>

## 类型别名 (Type Aliases)

名为“类型别名”的部分

### ReloadOptions

标题为“ReloadOptions”的章节

    type ReloadOptions: object;

IStore.reload 的选项

#### 类型声明

名为“类型定义”的章节

名称| 类型| 描述| 定义于
---|---|---|---
`ignoreDefaults`| `布尔值 (boolean)`| 为了使存储与磁盘上的状态完全匹配，忽略默认值。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L461>

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L457>

* * *

### StoreOptions

标题为“StoreOptions”的章节

    type StoreOptions: object;

创建存储的选项。

#### 类型声明

名为“类型定义”的章节

名称| 类型| 描述| 定义于
---|---|---|---
`autoSave`| `boolean` | `number`| 修改时自动保存，并指定防抖时长（以毫秒为单位），默认值为 100ms，传入 `false` 可禁用它。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L28>
`createNew`| `布尔值 (boolean)`| 即使存储已经存在，也强制创建一个带有默认值的新存储。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L40>
`defaults`| `对象`| 存储的默认值。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L24>
`deserializeFnName`| `string`| 在 Rust 端插件构建器中注册的反序列化函数名称。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L36>
`overrideDefaults`| `布尔值 (boolean)`| 创建存储时，如果磁盘上存在存储状态，则用磁盘上的状态覆盖它，忽略默认值。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L44>
`serializeFnName`| `string`| 在 Rust 端插件构建器中注册的序列化函数名称。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L32>

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L20>

## 函数

名为“函数”的部分

### getStore()

标题为“getStore()”的章节

    function getStore(path): Promise<Store | null>

获取一个已经加载的存储。

如果存储未加载，则返回 `null`。在这种情况下，你必须 [加载](/reference/javascript/store/#load) 它。

当你知道存储已经加载并且只需要访问其实例时，此函数更有用。否则，请优先使用 [Store.load](/reference/javascript/store/#load)。

#### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`path`| `string`| 存储的路径。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`Store`](/reference/javascript/store/#store) | `null`>

#### 示例

标题为“Example”的章节

    import { getStore } from '@tauri-apps/api/store';

    const store = await getStore('store.json');

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L82>

* * *

### load()

标题为“load()”的章节

    function load(path, options?): Promise<Store>

创建一个新的存储，或加载带有该路径的现有存储。

#### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`path`| `string`| 在 `app_data_dir` 中保存存储的路径。
`选项`?| [`StoreOptions`](/reference/javascript/store/#storeoptions)| 存储配置选项。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`Store`](/reference/javascript/store/#store)>

#### 示例

标题为“Example”的章节

    import { Store } from '@tauri-apps/api/store';

    const store = await Store.load('store.json');

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/guest-js/index.ts#L59>