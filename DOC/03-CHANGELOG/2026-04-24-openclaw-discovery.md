## OpenClaw 本机发现机制 (2026-04-24)

### 功能描述
- 自动探测 OpenClaw Gateway 运行端口（18789 → 19001）
- 不再硬编码 `ws://127.0.0.1:18789`
- 重连时重新探测端口（支持用户切换端口场景）

### 修复内容

#### 问题：block_on panic 导致应用崩溃
- 错误信息: `Cannot drop a runtime in a context where blocking is not allowed`
- 根本原因: 在 async 上下文中使用 `tokio::runtime::Handle::current().block_on()`

#### 修复内容

1. **src-tauri/src/openclaw/discovery.rs**（新增）
   - 使用同步 `std::net::TcpStream::connect_timeout()` 进行端口探测
   - 避免依赖 Tokio runtime 的 `block_on`
   - 探测端口: 18789（默认）→ 19001（dev 模式）
   - 每个端口 3 秒超时，最坏情况 6 秒

2. **src-tauri/src/openclaw/client.rs**
   - 移除硬编码 `GATEWAY_URL` 常量
   - `OpenClawClient::new()` 调用同步 `discover()` 进行端口发现
   - `connect_and_run()` 重连时重新探测端口

3. **src-tauri/src/openclaw/mod.rs**
   - 导出 `discovery` 模块和 `DiscoveryResult`

### 涉及文件

| 文件 | 操作 | 说明 |
|------|------|------|
| `src-tauri/src/openclaw/discovery.rs` | 新增 | 端口发现模块 |
| `src-tauri/src/openclaw/mod.rs` | 修改 | 导出 discovery |
| `src-tauri/src/openclaw/client.rs` | 修改 | 使用发现机制 |

### 测试验证
- [x] 应用启动成功，discovery 正确找到 18789 端口
- [x] OpenClaw Gateway 连接认证成功
- [x] 消息发送/响应正常
