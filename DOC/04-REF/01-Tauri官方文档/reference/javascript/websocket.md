# @tauri-apps/plugin-websocket

_Source: https://v2.tauri.org.cn/reference/javascript/websocket/_

## 类

名为“类”的部分

### default

名为“default”的章节

#### 构造函数

名为“构造函数”的部分

##### new default()

名为“new default()”的章节

    new default(id, listeners): default

###### 参数

名为“参数”的部分

参数| 类型
---|---
`ID`| `数字`
`listeners`| [`Set`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Set)<(`arg`) => `void`>

###### 返回

名为“返回值”的部分

[`default`](/reference/javascript/websocket/#default)

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/websocket/guest-js/index.ts#L63>

#### 属性

名为“属性”的部分

属性| 类型| 定义于
---|---|---
`id`| `数字`| **来源** : [https://github.com/tauri-apps/plugins-websocket/guest-js/index.ts#L60](https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/websocket/guest-js/index.ts#L60)

#### 方法

名为“方法”的部分

##### addListener()

名为“addListener()”的章节

    addListener(cb): () => void

###### 参数

名为“参数”的部分

参数| 类型
---|---
`cb`| (`arg`) => `void`

###### 返回

名为“返回值”的部分

`函数`

###### 返回

名为“返回值”的部分

`空`

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/websocket/guest-js/index.ts#L92>

##### disconnect()

名为“disconnect()”的章节

    disconnect(): Promise<void>

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/websocket/guest-js/index.ts#L119>

##### send()

名为“send()”的章节

    send(message): Promise<void>

###### 参数

名为“参数”的部分

参数| 类型
---|---
`消息`| `string` | `number`[] | [`Message`](/reference/javascript/websocket/#message)

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/websocket/guest-js/index.ts#L100>

##### connect()

名为“connect()”的章节

    static connect(url, config?): Promise<default>

###### 参数

名为“参数”的部分

参数| 类型
---|---
`url`| `string`
`config`?| [`ConnectionConfig`](/reference/javascript/websocket/#connectionconfig)

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`default`](/reference/javascript/websocket/#default)>

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/websocket/guest-js/index.ts#L68>

## 接口

名为“接口”的部分

### CloseFrame

名为“CloseFrame”的章节

#### 属性

名为“属性”的部分

属性| 类型| 定义于
---|---|---
`code`| `数字`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/websocket/guest-js/index.ts#L48>
`reason`| `string`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/websocket/guest-js/index.ts#L49>

* * *

### ConnectionConfig

名为“ConnectionConfig”的章节

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
`acceptUnmaskedFrames?`| `布尔值 (boolean)`| 当设置为 true 时，服务器将接受并处理来自客户端的未掩码帧（unmasked frames）。根据 RFC 6455，服务器在此类情况下必须关闭与客户端的连接，但似乎有一些流行的库正在发送未掩码帧，从而忽略了 RFC 标准。此选项默认为 false，即遵循 RFC 6455 标准。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/websocket/guest-js/index.ts#L35>
`headers?`| `HeadersInit`| 额外的连接请求头。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/websocket/guest-js/index.ts#L39>
`maxFrameSize?`| `number` | `"none"`| 单个传入消息帧的最大大小。字符串“none”表示没有大小限制。此限制针对的是帧有效载荷，不包括帧头。默认值为 16 MiB，对于所有正常用例来说应该足够大，但又足够小，以防止恶意用户消耗内存。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/websocket/guest-js/index.ts#L31>
`maxMessageSize?`| `number` | `"none"`| 传入消息的最大大小。字符串“none”表示没有大小限制。默认值为 64 MiB，对于所有正常用例来说应该足够大，但又足够小，以防止恶意用户消耗内存。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/websocket/guest-js/index.ts#L27>
`maxWriteBufferSize?`| `数字`| 写缓冲区的最大大小（以字节为单位）。设置此项可以在写缓冲区因写入错误而填满时提供背压（backpressure）。默认值是无限的。注意：只有当底层流的写入失败时，写缓冲区才会超过 write_buffer_size。因此，如果您没有遇到写入错误，写缓冲区就不会填满。注意：应该始终至少为 write_buffer_size + 1 条消息，具体取决于错误处理策略，通常需要更多。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/websocket/guest-js/index.ts#L23>
`readBufferSize?`| `数字`| 读缓冲区容量。默认值为 128 KiB。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/websocket/guest-js/index.ts#L11>
`writeBufferSize?`| `数字`| 在将数据写入底层流之前，写缓冲区应达到的目标最小大小。默认值为 128 KiB。如果设置为 0，则每条消息都会被立即写入到底层流。通常允许它们稍微缓冲一下会更优化，这就是默认值的原因。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/websocket/guest-js/index.ts#L16>

* * *

### MessageKind<T, D>

名为“MessageKind<T, D>”的章节

#### 类型参数

名为“类型参数”的部分

类型参数
---
`T`
`D`

#### 属性

名为“属性”的部分

属性| 类型| 定义于
---|---|---
`data`| `D`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/websocket/guest-js/index.ts#L44>
`type`| `T`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/websocket/guest-js/index.ts#L43>

## 类型别名 (Type Aliases)

名为“类型别名”的部分

### Message

名为“Message”的章节

    type Message:

      | MessageKind<"Text", string>

      | MessageKind<"Binary", number[]>

      | MessageKind<"Ping", number[]>

      | MessageKind<"Pong", number[]>

    | MessageKind<"Close", CloseFrame | null>;

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/websocket/guest-js/index.ts#L52>