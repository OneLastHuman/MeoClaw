# @tauri-apps/plugin-stronghold

_Source: https://v2.tauri.org.cn/reference/javascript/stronghold/_

## 类

名为“类”的部分

### Client

名为“Client”的章节

#### 构造函数

名为“构造函数”的部分

##### new Client()

名为“new Client()”的章节

    new Client(path, name): Client

###### 参数

名为“参数”的部分

参数| 类型
---|---
`path`| `string`
`名称 (name)`| [`ClientPath`](/reference/javascript/stronghold/#clientpath)

###### 返回

名为“返回值”的部分

[`Client`](/reference/javascript/stronghold/#client)

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L265>

#### 属性

名为“属性”的部分

属性| 类型| 定义于
---|---|---
`name`| [`ClientPath`](/reference/javascript/stronghold/#clientpath)| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L263>
`path`| `string`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L262>

#### 方法

名为“方法”的部分

##### getStore()

名为“getStore()”的章节

    getStore(): Store

###### 返回

名为“返回值”的部分

[`存储 (Store)`](/reference/javascript/stronghold/#store)

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L280>

##### getVault()

名为“getVault()”的章节

    getVault(name): Vault

按名称获取保险库（vault）。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`名称 (name)`| [`VaultPath`](/reference/javascript/stronghold/#vaultpath)|

###### 返回

名为“返回值”的部分

[`Vault`](/reference/javascript/stronghold/#vault)

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L276>

* * *

### Location

名为“Location”的章节

#### 构造函数

名为“构造函数”的部分

##### new Location()

名为“new Location()”的章节

    new Location(type, payload): Location

###### 参数

名为“参数”的部分

参数| 类型
---|---
`type`| `string`
`载荷 (payload)`| [`Record`](https://typescript.net.cn/docs/handbook/utility-types.html#recordkeys-type)<`string`, `unknown`>

###### 返回

名为“返回值”的部分

[`Location`](/reference/javascript/stronghold/#location)

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L86>

#### 属性

名为“属性”的部分

属性| 类型| 定义于
---|---|---
`payload`| [`Record`](https://typescript.net.cn/docs/handbook/utility-types.html#recordkeys-type)<`string`, `unknown`>| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L84>
`type`| `string`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L83>

#### 方法

名为“方法”的部分

##### counter()

名为“counter()”的章节

    static counter(vault, counter): Location

###### 参数

名为“参数”的部分

参数| 类型
---|---
`vault`| [`VaultPath`](/reference/javascript/stronghold/#vaultpath)
`counter`| `数字`

###### 返回

名为“返回值”的部分

[`Location`](/reference/javascript/stronghold/#location)

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L98>

##### generic()

名为“generic()”的章节

    static generic(vault, record): Location

###### 参数

名为“参数”的部分

参数| 类型
---|---
`vault`| [`VaultPath`](/reference/javascript/stronghold/#vaultpath)
`record`| [`RecordPath`](/reference/javascript/stronghold/#recordpath)

###### 返回

名为“返回值”的部分

[`Location`](/reference/javascript/stronghold/#location)

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L91>

* * *

### 存储 (Store)

名为“Store”的章节

#### 构造函数

名为“构造函数”的部分

##### new Store()

名为“new Store()”的章节

    new Store(path, client): Store

###### 参数

名为“参数”的部分

参数| 类型
---|---
`path`| `string`
`client`| [`ClientPath`](/reference/javascript/stronghold/#clientpath)

###### 返回

名为“返回值”的部分

[`存储 (Store)`](/reference/javascript/stronghold/#store)

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L289>

#### 属性

名为“属性”的部分

属性| 类型| 定义于
---|---|---
`client`| [`ClientPath`](/reference/javascript/stronghold/#clientpath)| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L287>
`path`| `string`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L286>

#### 方法

名为“方法”的部分

##### get()

名为“get()”的章节

    get(key): Promise<null | Uint8Array>

###### 参数

名为“参数”的部分

参数| 类型
---|---
`key`| [`StoreKey`](/reference/javascript/stronghold/#storekey)

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`null` | [`Uint8Array`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array)>

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L294>

##### insert()

名为“insert()”的章节

    insert(

       key,

       value,

    lifetime?): Promise<void>

###### 参数

名为“参数”的部分

参数| 类型
---|---
`key`| [`StoreKey`](/reference/javascript/stronghold/#storekey)
`value`| `数字`[]
`lifetime`?| [`Duration`](/reference/javascript/stronghold/#duration)

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L302>

##### remove()

名为“remove()”的章节

    remove(key): Promise<null | Uint8Array>

###### 参数

名为“参数”的部分

参数| 类型
---|---
`key`| [`StoreKey`](/reference/javascript/stronghold/#storekey)

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`null` | [`Uint8Array`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array)>

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L316>

* * *

### 要塞 (Stronghold)

名为“Stronghold”的章节

Stronghold 访问的表现形式。

#### 属性

名为“属性”的部分

属性| 类型| 定义于
---|---|---
`path`| `string`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L388>

#### 方法

名为“方法”的部分

##### createClient()

名为“createClient()”的章节

    createClient(client): Promise<Client>

###### 参数

名为“参数”的部分

参数| 类型
---|---
`client`| [`ClientPath`](/reference/javascript/stronghold/#clientpath)

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`Client`](/reference/javascript/stronghold/#client)>

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L428>

##### loadClient()

名为“loadClient()”的章节

    loadClient(client): Promise<Client>

###### 参数

名为“参数”的部分

参数| 类型
---|---
`client`| [`ClientPath`](/reference/javascript/stronghold/#clientpath)

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`Client`](/reference/javascript/stronghold/#client)>

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L421>

##### save()

save() 章节

    save(): Promise<void>

将 Stronghold 状态持久化到快照中。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L439>

##### unload()

名为“unload()”的章节

    unload(): Promise<void>

从缓存中移除此实例。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L415>

##### load()

名为“load()”的章节

    static load(path, password): Promise<Stronghold>

如果快照存在则加载（密码必须匹配），否则启动一个新的 Stronghold 实例。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`path`| `string`| -
`password`| `string`|

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`Stronghold`](/reference/javascript/stronghold/#stronghold)>

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L405>

* * *

### Vault

名为“Vault”的章节

一种键值存储，支持创建、更新和删除操作。它不允许直接读取数据，因此必须使用程序（procedures）来操作存储的数据，从而实现机密的安全存储。

#### 继承 (Extends)

名为“继承自”的章节

  * `ProcedureExecutor`

#### 构造函数

名为“构造函数”的部分

##### new Vault()

名为“new Vault()”的章节

    new Vault(

       path,

       client,

       name): Vault

###### 参数

名为“参数”的部分

参数| 类型
---|---
`path`| `string`
`client`| [`ClientPath`](/reference/javascript/stronghold/#clientpath)
`名称 (name)`| [`VaultPath`](/reference/javascript/stronghold/#vaultpath)

###### 返回

名为“返回值”的部分

[`Vault`](/reference/javascript/stronghold/#vault)

###### 重写 (Overrides)

名为“Overrides”的章节

`ProcedureExecutor.constructor`

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L340>

#### 属性

名为“属性”的部分

属性| 类型| 描述| 继承自 (Inherited from)| 定义于
---|---|---|---|---
`client`| [`ClientPath`](/reference/javascript/stronghold/#clientpath)| -| -| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L336>
`name`| [`VaultPath`](/reference/javascript/stronghold/#vaultpath)| 保险库名称。| -| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L338>
`path`| `string`| 保险库路径。| -| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L335>
`procedureArgs`| [`Record`](https://typescript.net.cn/docs/handbook/utility-types.html#recordkeys-type)<`string`, `unknown`>| -| `ProcedureExecutor.procedureArgs`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L107>

#### 方法

名为“方法”的部分

##### deriveSLIP10()

名为“deriveSLIP10()”的章节

    deriveSLIP10(

       chain,

       source,

       sourceLocation,

    outputLocation): Promise<Uint8Array>

使用种子或密钥派生 SLIP10 私钥。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`chain`| `数字`[]| 链路径。
`source`| `"Seed"` | `"Key"`| 源类型，可以是“Seed”（种子）或“Key”（密钥）。
`sourceLocation`| [`Location`](/reference/javascript/stronghold/#location)| 源位置，必须是之前调用 `generateSLIP10Seed` 或 `deriveSLIP10` 时的 `outputLocation`。
`outputLocation`| [`Location`](/reference/javascript/stronghold/#location)| 存储私钥的记录位置。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`Uint8Array`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array)>

###### 继承自 (Inherited from)

名为“继承自”的章节

`ProcedureExecutor.deriveSLIP10`

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L145>

##### generateBIP39()

名为“generateBIP39()”的章节

    generateBIP39(outputLocation, passphrase?): Promise<Uint8Array>

生成 BIP39 种子。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`outputLocation`| [`Location`](/reference/javascript/stronghold/#location)| 存储 BIP39 种子的记录位置。
`passphrase`?| `string`| 可选的助记词口令。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`Uint8Array`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array)>

###### 继承自 (Inherited from)

名为“继承自”的章节

`ProcedureExecutor.generateBIP39`

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L200>

##### generateSLIP10Seed()

名为“generateSLIP10Seed()”的章节

    generateSLIP10Seed(outputLocation, sizeBytes?): Promise<Uint8Array>

为指定位置生成 SLIP10 种子。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`outputLocation`| [`Location`](/reference/javascript/stronghold/#location)| 存储种子的记录位置。
`sizeBytes`?| `数字`| SLIP10 种子的大小（字节）。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`Uint8Array`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array)>

###### 继承自 (Inherited from)

名为“继承自”的章节

`ProcedureExecutor.generateSLIP10Seed`

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L120>

##### getEd25519PublicKey()

名为“getEd25519PublicKey()”的章节

    getEd25519PublicKey(privateKeyLocation): Promise<Uint8Array>

获取 SLIP10 私钥的 Ed25519 公钥。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`privateKeyLocation`| [`Location`](/reference/javascript/stronghold/#location)| 私钥的位置。必须是之前调用 `deriveSLIP10` 时的 `outputLocation`。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`Uint8Array`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array)>

一个解析为公钥十六进制字符串的 Promise。

###### 始于

名为“起始版本”的部分

2.0.0

###### 继承自 (Inherited from)

名为“继承自”的章节

`ProcedureExecutor.getEd25519PublicKey`

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L223>

##### insert()

名为“insert()”的章节

    insert(recordPath, secret): Promise<void>

将记录插入此保险库。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`recordPath`| [`RecordPath`](/reference/javascript/stronghold/#recordpath)
`secret`| `数字`[]

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L358>

##### recoverBIP39()

名为“recoverBIP39()”的章节

    recoverBIP39(

       mnemonic,

       outputLocation,

    passphrase?): Promise<Uint8Array>

存储 BIP39 助记词。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`mnemonic`| `string`| 助记词字符串。
`outputLocation`| [`Location`](/reference/javascript/stronghold/#location)| 存储 BIP39 助记词的记录位置。
`passphrase`?| `string`| 可选的助记词口令。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`Uint8Array`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array)>

###### 继承自 (Inherited from)

名为“继承自”的章节

`ProcedureExecutor.recoverBIP39`

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L175>

##### remove()

名为“remove()”的章节

    remove(location): Promise<void>

从保险库中删除一条记录。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`location`| [`Location`](/reference/javascript/stronghold/#location)| 记录位置。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L374>

##### signEd25519()

名为“signEd25519()”的章节

    signEd25519(privateKeyLocation, msg): Promise<Uint8Array>

使用私钥创建 Ed25519 签名。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`privateKeyLocation`| [`Location`](/reference/javascript/stronghold/#location)| 存储私钥的记录位置。必须是之前调用 `deriveSLIP10` 时的 `outputLocation`。
`msg`| `string`| 要签名的消息。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`Uint8Array`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array)>

一个解析为签名十六进制字符串的 Promise。

###### 始于

名为“起始版本”的部分

2.0.0

###### 继承自 (Inherited from)

名为“继承自”的章节

`ProcedureExecutor.signEd25519`

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L244>

## 接口

名为“接口”的部分

### AddressInfo

名为“AddressInfo”的章节

#### 属性

名为“属性”的部分

属性| 类型| 定义于
---|---|---
`peers`| [`Map`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Map)<`string`, [`PeerAddress`](/reference/javascript/stronghold/#peeraddress)>| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L43>
`relays`| `string`[]| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L44>

* * *

### ClientAccess

名为“ClientAccess”的章节

#### 属性

名为“属性”的部分

属性| 类型| 定义于
---|---|---
`cloneVaultDefault?`| `布尔值 (boolean)`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L52>
`cloneVaultExceptions?`| [`Map`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Map)<[`VaultPath`](/reference/javascript/stronghold/#vaultpath), `boolean`>| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L53>
`readStore?`| `布尔值 (boolean)`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L54>
`useVaultDefault?`| `布尔值 (boolean)`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L48>
`useVaultExceptions?`| [`Map`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Map)<[`VaultPath`](/reference/javascript/stronghold/#vaultpath), `boolean`>| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L49>
`writeStore?`| `布尔值 (boolean)`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L55>
`writeVaultDefault?`| `布尔值 (boolean)`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L50>
`writeVaultExceptions?`| [`Map`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Map)<[`VaultPath`](/reference/javascript/stronghold/#vaultpath), `boolean`>| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L51>

* * *

### ConnectionLimits

名为“ConnectionLimits”的章节

#### 属性

名为“属性”的部分

属性| 类型| 定义于
---|---|---
`maxEstablishedIncoming?`| `数字`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L31>
`maxEstablishedOutgoing?`| `数字`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L32>
`maxEstablishedPerPeer?`| `数字`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L33>
`maxEstablishedTotal?`| `数字`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L34>
`maxPendingIncoming?`| `数字`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L29>
`maxPendingOutgoing?`| `数字`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L30>

* * *

### Duration

名为“Duration”的章节

持续时间定义。

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
`nanos`| `数字`| 此持续时间的分数部分，以纳秒为单位。必须大于或等于 0 且小于 1e+9（一秒中的最大纳秒数）。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L79>
`secs`| `数字`| 此持续时间包含的整秒数。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L77>

* * *

### NetworkConfig

名为“NetworkConfig”的章节

#### 属性

名为“属性”的部分

属性| 类型| 定义于
---|---|---
`addresses?`| [`AddressInfo`](/reference/javascript/stronghold/#addressinfo)| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L69>
`connectionTimeout?`| [`Duration`](/reference/javascript/stronghold/#duration)| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L65>
`connectionsLimit?`| [`ConnectionLimits`](/reference/javascript/stronghold/#connectionlimits)| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L66>
`enableMdns?`| `布尔值 (boolean)`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L67>
`enableRelay?`| `布尔值 (boolean)`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L68>
`peerPermissions?`| [`Map`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Map)<`string`, [`Permissions`](/reference/javascript/stronghold/#permissions)>| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L70>
`permissionsDefault?`| [`Permissions`](/reference/javascript/stronghold/#permissions)| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L71>
`requestTimeout?`| [`Duration`](/reference/javascript/stronghold/#duration)| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L64>

* * *

### PeerAddress

名为“PeerAddress”的章节

#### 属性

名为“属性”的部分

属性| 类型| 定义于
---|---|---
`known`| `string`[]| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L38>
`use_relay_fallback`| `布尔值 (boolean)`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L39>

* * *

### Permissions

名为“权限”的章节

#### 属性

名为“属性”的部分

属性| 类型| 定义于
---|---|---
`default?`| [`ClientAccess`](/reference/javascript/stronghold/#clientaccess)| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L59>
`exceptions?`| [`Map`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Map)<[`VaultPath`](/reference/javascript/stronghold/#vaultpath), [`ClientAccess`](/reference/javascript/stronghold/#clientaccess)>| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L60>

## 类型别名 (Type Aliases)

名为“类型别名”的部分

### ClientPath

名为“ClientPath”的章节

    type ClientPath: string | Iterable<number> | ArrayLike<number> | ArrayBuffer;

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L7>

* * *

### RecordPath

名为“RecordPath”的章节

    type RecordPath: string | Iterable<number> | ArrayLike<number> | ArrayBuffer;

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L17>

* * *

### StoreKey

名为“StoreKey”的章节

    type StoreKey: string | Iterable<number> | ArrayLike<number> | ArrayBuffer;

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L22>

* * *

### VaultPath

名为“VaultPath”的章节

    type VaultPath: string | Iterable<number> | ArrayLike<number> | ArrayBuffer;

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/stronghold/guest-js/index.ts#L12>