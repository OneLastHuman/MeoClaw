## Hermes 后端修复 (2026-04-24)

### 问题 1：阻塞 I/O Panic
- Hermes 后端发送消息后，进程崩溃并停留在 "working" 状态
- 错误信息: `Cannot drop a runtime in a context where blocking is not allowed`

### 根本原因 1
- `HermesDiscovery::discover()` 内部使用 `reqwest::blocking::Client` 执行阻塞 I/O
- 在 `tokio::spawn` (async context) 中调用阻塞 I/O 导致 panic

### 修复内容 1
1. **src-tauri/src/lib.rs**
   - 使用 `tokio::task::spawn_blocking` 包装 `HermesDiscovery::discover()` 调用
   - 避免在 async 上下文中执行阻塞 I/O
   - 添加 `finish_reason === "stop"` 检测，正确处理最终响应
   - 添加详细日志便于后续调试

### 问题 2：工具调用事件丢失
- Hermes 工具调用（tool call）没有正常返回对应的气泡消息
- 桌宠只显示 "思考中..." bubble，工具执行时无反馈

### 根本原因 2
- commit 0c7dee3 重构 SSE 解析逻辑时误删了 `hermes.tool.progress` 事件处理
- 移除了 `current_event` 变量来追踪 event 类型
- SSE 解析只处理 `data: ` 开头的行，忽略了 `event: xxx` 行

### 修复内容 2
1. **src-tauri/src/lib.rs**
   - 恢复 `current_event` 变量追踪 event 类型
   - 恢复 `hermes.tool.progress` 事件的处理分支
   - 解析 `event: hermes.tool.progress` + `data: {...}` 格式
   - 发送 `update-bubble` 事件显示工具调用气泡

### 测试验证
- Hermes API 健康检查正常
- 发送消息后能正确接收响应
- 桌宠动画状态能正确切换到 Response
- 工具调用时正确显示工具名称和状态气泡
