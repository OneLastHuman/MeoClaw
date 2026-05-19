# @tauri-apps/plugin-shell

_Source: https://v2.tauri.org.cn/reference/javascript/shell/_

访问系统 shell。允许您生成子进程，并使用默认应用程序管理文件和 URL。

## 安全

名为“安全”的章节

此 API 具有作用域配置，强制您限制可使用的程序和参数。

### 限制对 [`open`](/reference/javascript/shell/#open) API 的访问

名为“限制对 open API 的访问”的章节

在配置对象中，`open: true` 表示 [open](/reference/javascript/shell/#open) API 可以用于任何 URL，因为参数是使用 `^((mailto:\w+)|(tel:\w+)|(https?://\w+)).+` 正则表达式进行验证的。您可以通过将布尔值更改为字符串来修改该正则表达式，例如 `open: ^https://github.com/`。

### 限制对 [`Command`](/reference/javascript/shell/#commando) API 的访问

名为“限制对 Command API 的访问”的章节

插件权限对象具有一个 `scope` 字段，用于定义可使用的 CLI 数组。每个 CLI 都是一个配置对象 `{ name: string, cmd: string, sidecar?: bool, args?: boolean | Arg[] }`。

  * `name`：命令的唯一标识符，传递给 [Command.create 函数](/reference/javascript/shell/#create)。如果是 sidecar，此值必须是在 `tauri.conf.json > bundle > externalBin` 中定义的值。
  * `cmd`：在此配置下执行的程序。如果是 sidecar，则忽略此值。
  * `sidecar`：对象是配置 sidecar 还是系统程序。
  * `args`：可以传递给程序的参数。默认情况下不允许任何参数。
    * `true` 表示允许任何参数列表。
    * `false` 表示不允许任何参数。
    * 或者，可以配置一个数组。每个项目要么是表示固定参数值的字符串，要么是定义验证参数值的正则表达式的 `{ validator: string }`。

#### 作用域配置示例

名为“作用域配置示例”的章节

CLI: `git commit -m "the commit message"`

功能

    {

      "permissions": [

        {

          "identifier": "shell:allow-execute",

          "allow": [

            {

              "name": "run-git-commit",

              "cmd": "git",

              "args": ["commit", "-m", { "validator": "\\S+" }]

            }

          ]

        }

      ]

    }

用法

    import { Command } from '@tauri-apps/plugin-shell'

    Command.create('run-git-commit', ['commit', '-m', 'the commit message'])

尝试使用未在 scope 中配置的程序执行任何 API，都会因为权限被拒绝而导致 promise 被拒绝。

## 类

名为“类”的部分

### Child

名为“Child”的章节

#### 始于

名为“起始版本”的部分

2.0.0

#### 构造函数

名为“构造函数”的部分

##### new Child()

名为“new Child()”的章节

    new Child(pid): Child

###### 参数

名为“参数”的部分

参数| 类型
---|---
`pid`| `数字`

###### 返回

名为“返回值”的部分

[`Child`](/reference/javascript/shell/#child)

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/shell/guest-js/index.ts#L301>

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
`pid`| `数字`| 子进程 `pid`。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/shell/guest-js/index.ts#L299>

#### 方法

名为“方法”的部分

##### kill()

名为“kill()”的章节

    kill(): Promise<void>

杀死子进程。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/shell/guest-js/index.ts#L336>

##### write()

名为“write()”的章节

    write(data): Promise<void>

向 `stdin` 写入 `data`。

###### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`data`| [`IOPayload`](/reference/javascript/shell/#iopayload) | `number`[]| 要写入的消息，可以是字符串或字节数组。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

###### 示例

标题为“Example”的章节

    import { Command } from '@tauri-apps/plugin-shell';

    const command = Command.create('node');

    const child = await command.spawn();

    await child.write('message');

    await child.write([0, 1, 2, 3, 4, 5]);

###### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/shell/guest-js/index.ts#L322>

* * *

### Command<O>

名为“Command<O>”的章节

生成子进程的入口点。它会触发 `close` 和 `error` 事件。

#### 示例

标题为“Example”的章节

    import { Command } from '@tauri-apps/plugin-shell';

    const command = Command.create('node');

    command.on('close', data => {

      console.log(`command finished with code ${data.code} and signal ${data.signal}`)

    });

    command.on('error', error => console.error(`command error: "${error}"`));

    command.stdout.on('data', line => console.log(`command stdout: "${line}"`));

    command.stderr.on('data', line => console.log(`command stderr: "${line}"`));

    const child = await command.spawn();

    console.log('pid:', child.pid);

#### 始于

名为“起始版本”的部分

2.0.0

#### 继承 (Extends)

名为“继承自”的章节

  * [`EventEmitter`](/reference/javascript/shell/#eventemittere)<[`CommandEvents`](/reference/javascript/shell/#commandevents)>

#### 类型参数

名为“类型参数”的部分

类型参数
---
`O` _继承自_ [`IOPayload`](/reference/javascript/shell/#iopayload)

#### 属性

名为“属性”的部分

属性| 修饰符| 类型| 描述| 定义于
---|---|---|---|---
`stderr`| `只读`| [`EventEmitter`](/reference/javascript/shell/#eventemittere)<[`OutputEvents`](/reference/javascript/shell/#outputeventso)<`O`>>| `stderr` 的事件发射器。触发 `data` 事件。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/shell/guest-js/index.ts#L384>
`stdout`| `只读`| [`EventEmitter`](/reference/javascript/shell/#eventemittere)<[`OutputEvents`](/reference/javascript/shell/#outputeventso)<`O`>>| `stdout` 的事件发射器。触发 `data` 事件。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/shell/guest-js/index.ts#L382>

#### 方法

名为“方法”的部分

##### addListener()

名为“addListener()”的章节

    addListener<N>(eventName, listener): this

`emitter.on(eventName, listener)` 的别名。

###### 类型参数

名为“类型参数”的部分

类型参数
---
`N` _继承自_ keyof [`CommandEvents`](/reference/javascript/shell/#commandevents)

###### 参数

名为“参数”的部分

参数| 类型
---|---
`eventName`| `N`
`listener`| (`arg`) => `void`

###### 返回

名为“返回值”的部分

`this`

###### 始于

名为“起始版本”的部分

2.0.0

###### 继承自 (Inherited from)

名为“继承自”的章节

[`EventEmitter`](/reference/javascript/shell/#eventemittere).[`addListener`](/reference/javascript/shell/#addlistener-1)

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/shell/guest-js/index.ts#L118>

##### execute()

名为“execute()”的章节

    execute(): Promise<ChildProcess<O>>

将命令作为子进程执行，等待其完成并收集所有输出。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`ChildProcess`](/reference/javascript/shell/#childprocesso)<`O`>>

一个解析为子进程输出的 promise。

###### 示例

标题为“Example”的章节

    import { Command } from '@tauri-apps/plugin-shell';

    const output = await Command.create('echo', 'message').execute();

    assert(output.code === 0);

    assert(output.signal === null);

    assert(output.stdout === 'message');

    assert(output.stderr === '');

###### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/shell/guest-js/index.ts#L530>

##### listenerCount()

名为“listenerCount()”的章节

    listenerCount<N>(eventName): number

返回正在监听名为 `eventName` 事件的侦听器数量。

###### 类型参数

名为“类型参数”的部分

类型参数
---
`N` _继承自_ keyof [`CommandEvents`](/reference/javascript/shell/#commandevents)

###### 参数

名为“参数”的部分

参数| 类型
---|---
`eventName`| `N`

###### 返回

名为“返回值”的部分

`数字`

###### 始于

名为“起始版本”的部分

2.0.0

###### 继承自 (Inherited from)

名为“继承自”的章节

[`EventEmitter`](/reference/javascript/shell/#eventemittere).[`listenerCount`](/reference/javascript/shell/#listenercount-1)

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/shell/guest-js/index.ts#L241>

##### off()

名为“off()”的章节

    off<N>(eventName, listener): this

从事件 eventName 的侦听器数组中删除所有指定的侦听器。返回对 `EventEmitter` 的引用，以便可以进行链式调用。

###### 类型参数

名为“类型参数”的部分

类型参数
---
`N` _继承自_ keyof [`CommandEvents`](/reference/javascript/shell/#commandevents)

###### 参数

名为“参数”的部分

参数| 类型
---|---
`eventName`| `N`
`listener`| (`arg`) => `void`

###### 返回

名为“返回值”的部分

`this`

###### 始于

名为“起始版本”的部分

2.0.0

###### 继承自 (Inherited from)

名为“继承自”的章节

[`EventEmitter`](/reference/javascript/shell/#eventemittere).[`off`](/reference/javascript/shell/#off-1)

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/shell/guest-js/index.ts#L186>

##### on()

名为“on()”的章节

    on<N>(eventName, listener): this

将 `listener` 函数添加到名为 `eventName` 事件的侦听器数组末尾。不会检查是否已经添加了该 `listener`。多次传递相同组合的 `eventName` 和 `listener` 将导致该 `listener` 被多次添加和调用。

返回对 `EventEmitter` 的引用，以便可以进行链式调用。

###### 类型参数

名为“类型参数”的部分

类型参数
---
`N` _继承自_ keyof [`CommandEvents`](/reference/javascript/shell/#commandevents)

###### 参数

名为“参数”的部分

参数| 类型
---|---
`eventName`| `N`
`listener`| (`arg`) => `void`

###### 返回

名为“返回值”的部分

`this`

###### 始于

名为“起始版本”的部分

2.0.0

###### 继承自 (Inherited from)

名为“继承自”的章节

[`EventEmitter`](/reference/javascript/shell/#eventemittere).[`on`](/reference/javascript/shell/#on-1)

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/shell/guest-js/index.ts#L147>

##### once()

“once()” 章节

    once<N>(eventName, listener): this

为名为 `eventName` 的事件添加一个**一次性** `listener` 函数。下次触发 `eventName` 时，此侦听器将被移除，然后调用。

返回对 `EventEmitter` 的引用，以便可以进行链式调用。

###### 类型参数

名为“类型参数”的部分

类型参数
---
`N` _继承自_ keyof [`CommandEvents`](/reference/javascript/shell/#commandevents)

###### 参数

名为“参数”的部分

参数| 类型
---|---
`eventName`| `N`
`listener`| (`arg`) => `void`

###### 返回

名为“返回值”的部分

`this`

###### 始于

名为“起始版本”的部分

2.0.0

###### 继承自 (Inherited from)

名为“继承自”的章节

[`EventEmitter`](/reference/javascript/shell/#eventemittere).[`once`](/reference/javascript/shell/#once-1)

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/shell/guest-js/index.ts#L169>

##### prependListener()

名为“prependListener()”的章节

    prependListener<N>(eventName, listener): this

将 `listener` 函数添加到名为 `eventName` 事件的侦听器数组 _开头_ 。不会检查是否已经添加了该 `listener`。多次传递相同组合的 `eventName` 和 `listener` 将导致该 `listener` 被多次添加和调用。

返回对 `EventEmitter` 的引用，以便可以进行链式调用。

###### 类型参数

名为“类型参数”的部分

类型参数
---
`N` _继承自_ keyof [`CommandEvents`](/reference/javascript/shell/#commandevents)

###### 参数

名为“参数”的部分

参数| 类型
---|---
`eventName`| `N`
`listener`| (`arg`) => `void`

###### 返回

名为“返回值”的部分

`this`

###### 始于

名为“起始版本”的部分

2.0.0

###### 继承自 (Inherited from)

名为“继承自”的章节

[`EventEmitter`](/reference/javascript/shell/#eventemittere).[`prependListener`](/reference/javascript/shell/#prependlistener-1)

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/shell/guest-js/index.ts#L258>

##### prependOnceListener()

名为“prependOnceListener()”的章节

    prependOnceListener<N>(eventName, listener): this

为名为 `eventName` 的事件添加一个**一次性** `listener` 函数，并将其放置在侦听器数组的开头。下次触发 `eventName` 时，此侦听器将被移除，然后调用。

返回对 `EventEmitter` 的引用，以便可以进行链式调用。

###### 类型参数

名为“类型参数”的部分

类型参数
---
`N` _继承自_ keyof [`CommandEvents`](/reference/javascript/shell/#commandevents)

###### 参数

名为“参数”的部分

参数| 类型
---|---
`eventName`| `N`
`listener`| (`arg`) => `void`

###### 返回

名为“返回值”的部分

`this`

###### 始于

名为“起始版本”的部分

2.0.0

###### 继承自 (Inherited from)

名为“继承自”的章节

[`EventEmitter`](/reference/javascript/shell/#eventemittere).[`prependOnceListener`](/reference/javascript/shell/#prependoncelistener-1)

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/shell/guest-js/index.ts#L280>

##### removeAllListeners()

名为“removeAllListeners()”的章节

    removeAllListeners<N>(event?): this

移除所有侦听器，或指定 eventName 的侦听器。

返回对 `EventEmitter` 的引用，以便可以进行链式调用。

###### 类型参数

名为“类型参数”的部分

类型参数
---
`N` _继承自_ keyof [`CommandEvents`](/reference/javascript/shell/#commandevents)

###### 参数

名为“参数”的部分

参数| 类型
---|---
`event`?| `N`

###### 返回

名为“返回值”的部分

`this`

###### 始于

名为“起始版本”的部分

2.0.0

###### 继承自 (Inherited from)

名为“继承自”的章节

[`EventEmitter`](/reference/javascript/shell/#eventemittere).[`removeAllListeners`](/reference/javascript/shell/#removealllisteners-1)

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/shell/guest-js/index.ts#L206>

##### removeListener()

名为“removeListener()”的章节

    removeListener<N>(eventName, listener): this

`emitter.off(eventName, listener)` 的别名。

###### 类型参数

名为“类型参数”的部分

类型参数
---
`N` _继承自_ keyof [`CommandEvents`](/reference/javascript/shell/#commandevents)

###### 参数

名为“参数”的部分

参数| 类型
---|---
`eventName`| `N`
`listener`| (`arg`) => `void`

###### 返回

名为“返回值”的部分

`this`

###### 始于

名为“起始版本”的部分

2.0.0

###### 继承自 (Inherited from)

名为“继承自”的章节

[`EventEmitter`](/reference/javascript/shell/#eventemittere).[`removeListener`](/reference/javascript/shell/#removelistener-1)

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/shell/guest-js/index.ts#L130>

##### spawn()

名为“spawn()”的章节

    spawn(): Promise<Child>

将命令作为子进程执行，并返回其句柄。

###### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`Child`](/reference/javascript/shell/#child)>

一个解析为子进程句柄的 promise。

###### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/shell/guest-js/index.ts#L479>

##### create()

名为“create()”的章节

创建一个命令以执行给定的程序。

###### 示例

标题为“Example”的章节

    import { Command } from '@tauri-apps/plugin-shell';

    const command = Command.create('my-app', ['run', 'tauri']);

    const output = await command.execute();

###### 参数

名为“参数”的章节

要执行的程序。它必须在您项目的 capabilities 中进行配置。

###### create(program, args)

名为“create(program, args)”的章节

    static create(program, args?): Command<string>

创建一个命令以执行给定的程序。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`program`| `string`
`args`?| `string` | `string`[]

###### 返回

名为“返回值”的部分

[`Command`](/reference/javascript/shell/#commando)<`string`>

###### 示例

标题为“Example”的章节

    import { Command } from '@tauri-apps/plugin-shell';

    const command = Command.create('my-app', ['run', 'tauri']);

    const output = await command.execute();

###### 参数

名为“参数”的章节

要执行的程序。它必须在您项目的 capabilities 中进行配置。

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/shell/guest-js/index.ts#L406>

###### create(program, args, options)

名为“create(program, args, options)”的章节

    static create(

       program,

       args?,

    options?): Command<Uint8Array>

创建一个命令以执行给定的程序。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`program`| `string`
`args`?| `string` | `string`[]
`选项`?| [`SpawnOptions`](/reference/javascript/shell/#spawnoptions) & `object`

###### 返回

名为“返回值”的部分

[`Command`](/reference/javascript/shell/#commando)<[`Uint8Array`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array)>

###### 示例

标题为“Example”的章节

    import { Command } from '@tauri-apps/plugin-shell';

    const command = Command.create('my-app', ['run', 'tauri']);

    const output = await command.execute();

###### 参数

名为“参数”的章节

要执行的程序。它必须在您项目的 capabilities 中进行配置。

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/shell/guest-js/index.ts#L407>

###### create(program, args, options)

名为“create(program, args, options)”的章节

    static create(

       program,

       args?,

    options?): Command<string>

创建一个命令以执行给定的程序。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`program`| `string`
`args`?| `string` | `string`[]
`选项`?| [`SpawnOptions`](/reference/javascript/shell/#spawnoptions)

###### 返回

名为“返回值”的部分

[`Command`](/reference/javascript/shell/#commando)<`string`>

###### 示例

标题为“Example”的章节

    import { Command } from '@tauri-apps/plugin-shell';

    const command = Command.create('my-app', ['run', 'tauri']);

    const output = await command.execute();

###### 参数

名为“参数”的章节

要执行的程序。它必须在您项目的 capabilities 中进行配置。

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/shell/guest-js/index.ts#L412>

##### sidecar()

名为“sidecar()”的章节

创建一个命令以执行给定的 sidecar 程序。

###### 示例

标题为“Example”的章节

    import { Command } from '@tauri-apps/plugin-shell';

    const command = Command.sidecar('my-sidecar');

    const output = await command.execute();

###### 参数

名为“参数”的章节

要执行的程序。它必须在您项目的 capabilities 中进行配置。

###### sidecar(program, args)

名为“sidecar(program, args)”的章节

    static sidecar(program, args?): Command<string>

创建一个命令以执行给定的 sidecar 程序。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`program`| `string`
`args`?| `string` | `string`[]

###### 返回

名为“返回值”的部分

[`Command`](/reference/javascript/shell/#commando)<`string`>

###### 示例

标题为“Example”的章节

    import { Command } from '@tauri-apps/plugin-shell';

    const command = Command.sidecar('my-sidecar');

    const output = await command.execute();

###### 参数

名为“参数”的章节

要执行的程序。它必须在您项目的 capabilities 中进行配置。

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/shell/guest-js/index.ts#L438>

###### sidecar(program, args, options)

名为“sidecar(program, args, options)”的章节

    static sidecar(

       program,

       args?,

    options?): Command<Uint8Array>

创建一个命令以执行给定的 sidecar 程序。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`program`| `string`
`args`?| `string` | `string`[]
`选项`?| [`SpawnOptions`](/reference/javascript/shell/#spawnoptions) & `object`

###### 返回

名为“返回值”的部分

[`Command`](/reference/javascript/shell/#commando)<[`Uint8Array`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array)>

###### 示例

标题为“Example”的章节

    import { Command } from '@tauri-apps/plugin-shell';

    const command = Command.sidecar('my-sidecar');

    const output = await command.execute();

###### 参数

名为“参数”的章节

要执行的程序。它必须在您项目的 capabilities 中进行配置。

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/shell/guest-js/index.ts#L439>

###### sidecar(program, args, options)

名为“sidecar(program, args, options)”的章节

    static sidecar(

       program,

       args?,

    options?): Command<string>

创建一个命令以执行给定的 sidecar 程序。

###### 参数

名为“参数”的部分

参数| 类型
---|---
`program`| `string`
`args`?| `string` | `string`[]
`选项`?| [`SpawnOptions`](/reference/javascript/shell/#spawnoptions)

###### 返回

名为“返回值”的部分

[`Command`](/reference/javascript/shell/#commando)<`string`>

###### 示例

标题为“Example”的章节

    import { Command } from '@tauri-apps/plugin-shell';

    const command = Command.sidecar('my-sidecar');

    const output = await command.execute();

###### 参数

名为“参数”的章节

要执行的程序。它必须在您项目的 capabilities 中进行配置。

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/shell/guest-js/index.ts#L444>

* * *

### EventEmitter<E>

名为“EventEmitter<E>”的章节

#### 始于

名为“起始版本”的部分

2.0.0

#### 扩展自

名为“扩展”的部分

  * [`Command`](/reference/javascript/shell/#commando)

#### 类型参数

名为“类型参数”的部分

类型参数
---
`E` _继承自_ [`Record`](https://typescript.net.cn/docs/handbook/utility-types.html#recordkeys-type)<`string`, `any`>

#### 构造函数

名为“构造函数”的部分

##### new EventEmitter()

名为“new EventEmitter()”的章节

    new EventEmitter<E>(): EventEmitter<E>

###### 返回

名为“返回值”的部分

[`EventEmitter`](/reference/javascript/shell/#eventemittere)<`E`>

#### 方法

名为“方法”的部分

##### addListener()

名为“addListener()”的章节

    addListener<N>(eventName, listener): this

`emitter.on(eventName, listener)` 的别名。

###### 类型参数

名为“类型参数”的部分

类型参数
---
`N` _继承自_ `string` | `number` | `symbol`

###### 参数

名为“参数”的部分

参数| 类型
---|---
`eventName`| `N`
`listener`| (`arg`) => `void`

###### 返回

名为“返回值”的部分

`this`

###### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/shell/guest-js/index.ts#L118>

##### listenerCount()

名为“listenerCount()”的章节

    listenerCount<N>(eventName): number

返回正在监听名为 `eventName` 事件的侦听器数量。

###### 类型参数

名为“类型参数”的部分

类型参数
---
`N` _继承自_ `string` | `number` | `symbol`

###### 参数

名为“参数”的部分

参数| 类型
---|---
`eventName`| `N`

###### 返回

名为“返回值”的部分

`数字`

###### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/shell/guest-js/index.ts#L241>

##### off()

名为“off()”的章节

    off<N>(eventName, listener): this

从事件 eventName 的侦听器数组中删除所有指定的侦听器。返回对 `EventEmitter` 的引用，以便可以进行链式调用。

###### 类型参数

名为“类型参数”的部分

类型参数
---
`N` _继承自_ `string` | `number` | `symbol`

###### 参数

名为“参数”的部分

参数| 类型
---|---
`eventName`| `N`
`listener`| (`arg`) => `void`

###### 返回

名为“返回值”的部分

`this`

###### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/shell/guest-js/index.ts#L186>

##### on()

名为“on()”的章节

    on<N>(eventName, listener): this

将 `listener` 函数添加到名为 `eventName` 事件的侦听器数组末尾。不会检查是否已经添加了该 `listener`。多次传递相同组合的 `eventName` 和 `listener` 将导致该 `listener` 被多次添加和调用。

返回对 `EventEmitter` 的引用，以便可以进行链式调用。

###### 类型参数

名为“类型参数”的部分

类型参数
---
`N` _继承自_ `string` | `number` | `symbol`

###### 参数

名为“参数”的部分

参数| 类型
---|---
`eventName`| `N`
`listener`| (`arg`) => `void`

###### 返回

名为“返回值”的部分

`this`

###### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/shell/guest-js/index.ts#L147>

##### once()

“once()” 章节

    once<N>(eventName, listener): this

为名为 `eventName` 的事件添加一个**一次性** `listener` 函数。下次触发 `eventName` 时，此侦听器将被移除，然后调用。

返回对 `EventEmitter` 的引用，以便可以进行链式调用。

###### 类型参数

名为“类型参数”的部分

类型参数
---
`N` _继承自_ `string` | `number` | `symbol`

###### 参数

名为“参数”的部分

参数| 类型
---|---
`eventName`| `N`
`listener`| (`arg`) => `void`

###### 返回

名为“返回值”的部分

`this`

###### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/shell/guest-js/index.ts#L169>

##### prependListener()

名为“prependListener()”的章节

    prependListener<N>(eventName, listener): this

将 `listener` 函数添加到名为 `eventName` 事件的侦听器数组 _开头_ 。不会检查是否已经添加了该 `listener`。多次传递相同组合的 `eventName` 和 `listener` 将导致该 `listener` 被多次添加和调用。

返回对 `EventEmitter` 的引用，以便可以进行链式调用。

###### 类型参数

名为“类型参数”的部分

类型参数
---
`N` _继承自_ `string` | `number` | `symbol`

###### 参数

名为“参数”的部分

参数| 类型
---|---
`eventName`| `N`
`listener`| (`arg`) => `void`

###### 返回

名为“返回值”的部分

`this`

###### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/shell/guest-js/index.ts#L258>

##### prependOnceListener()

名为“prependOnceListener()”的章节

    prependOnceListener<N>(eventName, listener): this

为名为 `eventName` 的事件添加一个**一次性** `listener` 函数，并将其放置在侦听器数组的开头。下次触发 `eventName` 时，此侦听器将被移除，然后调用。

返回对 `EventEmitter` 的引用，以便可以进行链式调用。

###### 类型参数

名为“类型参数”的部分

类型参数
---
`N` _继承自_ `string` | `number` | `symbol`

###### 参数

名为“参数”的部分

参数| 类型
---|---
`eventName`| `N`
`listener`| (`arg`) => `void`

###### 返回

名为“返回值”的部分

`this`

###### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/shell/guest-js/index.ts#L280>

##### removeAllListeners()

名为“removeAllListeners()”的章节

    removeAllListeners<N>(event?): this

移除所有侦听器，或指定 eventName 的侦听器。

返回对 `EventEmitter` 的引用，以便可以进行链式调用。

###### 类型参数

名为“类型参数”的部分

类型参数
---
`N` _继承自_ `string` | `number` | `symbol`

###### 参数

名为“参数”的部分

参数| 类型
---|---
`event`?| `N`

###### 返回

名为“返回值”的部分

`this`

###### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/shell/guest-js/index.ts#L206>

##### removeListener()

名为“removeListener()”的章节

    removeListener<N>(eventName, listener): this

`emitter.off(eventName, listener)` 的别名。

###### 类型参数

名为“类型参数”的部分

类型参数
---
`N` _继承自_ `string` | `number` | `symbol`

###### 参数

名为“参数”的部分

参数| 类型
---|---
`eventName`| `N`
`listener`| (`arg`) => `void`

###### 返回

名为“返回值”的部分

`this`

###### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/shell/guest-js/index.ts#L130>

## 接口

名为“接口”的部分

### ChildProcess<O>

名为“ChildProcess<O>”的章节

#### 始于

名为“起始版本”的部分

2.0.0

#### 类型参数

名为“类型参数”的部分

类型参数
---
`O` _继承自_ [`IOPayload`](/reference/javascript/shell/#iopayload)

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
`code`| `null` | `number`| 进程的退出代码。如果进程在 Unix 上被信号终止，则为 `null`。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/shell/guest-js/index.ts#L94>
`signal`| `null` | `number`| 如果进程被信号终止，则代表该信号。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/shell/guest-js/index.ts#L96>
`stderr`| `O`| 进程写入 `stderr` 的数据。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/shell/guest-js/index.ts#L100>
`stdout`| `O`| 进程写入 `stdout` 的数据。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/shell/guest-js/index.ts#L98>

* * *

### CommandEvents

名为“CommandEvents”的章节

#### 属性

名为“属性”的部分

属性| 类型| 定义于
---|---|---
`close`| [`TerminatedPayload`](/reference/javascript/shell/#terminatedpayload)| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/shell/guest-js/index.ts#L345>
`error`| `string`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/shell/guest-js/index.ts#L346>

* * *

### OutputEvents<O>

名为“OutputEvents<O>”的章节

#### 类型参数

名为“类型参数”的部分

类型参数
---
`O` _继承自_ [`IOPayload`](/reference/javascript/shell/#iopayload)

#### 属性

名为“属性”的部分

属性| 类型| 定义于
---|---|---
`data`| `O`| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/shell/guest-js/index.ts#L350>

* * *

### SpawnOptions

名为“SpawnOptions”的章节

#### 始于

名为“起始版本”的部分

2.0.0

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
`cwd?`| `string`| 当前工作目录。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/shell/guest-js/index.ts#L73>
`encoding?`| `string`| stdout/stderr 的字符编码 **版本** 2.0.0| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/shell/guest-js/index.ts#L81>
`env?`| [`Record`](https://typescript.net.cn/docs/handbook/utility-types.html#recordkeys-type)<`string`, `string`>| 环境变量。设置为 `null` 以清除进程环境。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/shell/guest-js/index.ts#L75>

* * *

### TerminatedPayload

名为“TerminatedPayload”的章节

`Terminated` 命令事件的负载。

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
`code`| `null` | `number`| 进程的退出代码。如果进程在 Unix 上被信号终止，则为 `null`。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/shell/guest-js/index.ts#L560>
`signal`| `null` | `number`| 如果进程被信号终止，则代表该信号。| **来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/shell/guest-js/index.ts#L562>

## 类型别名 (Type Aliases)

名为“类型别名”的部分

### IOPayload

名为“IOPayload”的章节

    type IOPayload: string | Uint8Array;

事件负载类型

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/shell/guest-js/index.ts#L566>

## 函数

名为“函数”的部分

### open()

名为“open()”的章节

    function open(path, openWith?): Promise<void>

使用系统的默认应用程序，或通过 `openWith` 指定的应用程序打开路径或 URL。

`openWith` 的值必须是 `firefox`、`google chrome`、`chromium`、`safari`、`open`、`start`、`xdg-open`、`gio`、`gnome-open`、`kde-open` 或 `wslview` 之一。

#### 参数

名为“参数”的部分

参数| 类型| 描述
---|---|---
`path`| `string`| 要打开的路径或 URL。此值与 `tauri.conf.json > plugins > shell > open` 中定义的字符串正则表达式进行匹配，该正则表达式默认为 `^((mailto:\w+)
`openWith`?| `string`| 用于打开文件或 URL 的应用程序。默认为指定路径类型的系统默认应用程序。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

#### 示例

标题为“Example”的章节

    import { open } from '@tauri-apps/plugin-shell';

    // opens the given URL on the default browser:

    await open('https://github.com/tauri-apps/tauri');

    // opens the given URL using `firefox`:

    await open('https://github.com/tauri-apps/tauri', 'firefox');

    // opens a file using the default program:

    await open('/path/to/file');

#### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/shell/guest-js/index.ts#L601>