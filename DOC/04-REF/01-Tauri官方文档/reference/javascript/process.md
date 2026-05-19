# @tauri-apps/plugin-process

_Source: https://v2.tauri.org.cn/reference/javascript/process/_

对当前进程执行操作。

## 函数

名为“函数”的部分

### exit()

名为“exit()”的章节

    function exit(code): Promise<void>

使用给定的 `exitCode` 立即退出。

#### 参数

名为“参数”的部分

参数| 类型| 默认值| 描述
---|---|---|---
`code`| `数字`| `0`| 要使用的退出代码。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

#### 示例

标题为“Example”的章节

    import { exit } from '@tauri-apps/plugin-process';

    await exit(1);

#### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/process/guest-js/index.ts#L25>

* * *

### relaunch()

名为“relaunch()”的章节

    function relaunch(): Promise<void>

退出应用程序的当前实例，然后重新启动它。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`>

表示操作成功或失败的 Promise。

#### 示例

标题为“Example”的章节

    import { relaunch } from '@tauri-apps/plugin-process';

    await relaunch();

#### 始于

名为“起始版本”的部分

2.0.0

**来源** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/process/guest-js/index.ts#L41>