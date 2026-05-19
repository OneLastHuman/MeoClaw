# @tauri-apps/plugin-cli

_Source: https://v2.tauri.org.cn/reference/javascript/cli/_

解析命令行界面参数。

## 接口

名为“接口”的部分

### ArgMatch

“ArgMatch” 章节

#### 始于

名为“起始版本”的部分

2.0.0

#### 属性

名为“属性”的部分

属性| 类型| 描述| 定义于
---|---|---|---
`occurrences`| `数字`| 出现次数| **源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/cli/guest-js/index.ts#L26>
`value`| `null` | `string` | `boolean` | `string`[]| 如果带值则为 string，如果为标记则为 boolean，如果接受多个值则为 string[] 或 null| **源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/cli/guest-js/index.ts#L22>

* * *

### CliMatches

“CliMatches” 章节

#### 始于

名为“起始版本”的部分

2.0.0

#### 属性

名为“属性”的部分

属性| 类型| 定义于
---|---|---
`args`| [`Record`](https://typescript.net.cn/docs/handbook/utility-types.html#recordkeys-type)<`string`, [`ArgMatch`](/reference/javascript/cli/#argmatch)>| **源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/cli/guest-js/index.ts#L41>
`subcommand`| `null` | [`SubcommandMatch`](/reference/javascript/cli/#subcommandmatch)| **源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/cli/guest-js/index.ts#L42>

* * *

### SubcommandMatch

“SubcommandMatch” 章节

#### 始于

名为“起始版本”的部分

2.0.0

#### 属性

名为“属性”的部分

属性| 类型| 定义于
---|---|---
`matches`| [`CliMatches`](/reference/javascript/cli/#climatches)| **源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/cli/guest-js/index.ts#L34>
`name`| `string`| **源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/cli/guest-js/index.ts#L33>

## 函数

名为“函数”的部分

### getMatches()

“getMatches()” 章节

    function getMatches(): Promise<CliMatches>

解析提供给当前进程的参数，并使用在 `tauri.conf.json` 中定义的 [`tauri.cli`](https://tauri.org.cn/v1/api/config/#tauriconfig.cli) 配置来获取匹配项。

#### 返回

名为“返回值”的部分

[`Promise`](https://mdn.org.cn/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`CliMatches`](/reference/javascript/cli/#climatches)>

#### 示例

标题为“Example”的章节

    import { getMatches } from '@tauri-apps/plugin-cli';

    const matches = await getMatches();

    if (matches.subcommand?.name === 'run') {

      // `./your-app run $ARGS` was executed

      const args = matches.subcommand?.matches.args

      if ('debug' in args) {

        // `./your-app run --debug` was executed

      }

    } else {

      const args = matches.args

      // `./your-app $ARGS` was executed

    }

#### 始于

名为“起始版本”的部分

2.0.0

**源码** : <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/cli/guest-js/index.ts#L66>