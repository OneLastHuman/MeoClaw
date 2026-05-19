# Hermes 后端集成调试记录

> 创建时间：2026/04/21
> 状态：进行中 - 502 Bad Gateway 问题未解决

---

## 问题现象

切换到 Hermes 后端后，发送消息返回 **502 Bad Gateway**，前端一直处于 working 状态没有回复。

---

## 已验证正常的部分

| 组件 | 状态 | 说明 |
|------|------|------|
| Hermes API (`/health`) | ✅ 正常 | `curl http://127.0.0.1:8642/health` 返回 200 |
| Hermes API 非 streaming | ✅ 正常 | `curl POST /v1/chat/completions stream=false` 正常返回 |
| Hermes API streaming | ✅ 正常 | `curl POST /v1/chat/completions stream=true` 正常返回 SSE |
| curl 请求格式 | ✅ 正常 | 使用 Bearer token 认证 |
| Hermes 配置发现 | ✅ 正常 | `API_SERVER_ENABLED=true`, `API_SERVER_KEY` 正确读取 |

---

## Rust 代码修改记录

### 1. HermesStream poll_next 实现修改

**文件**: `src-tauri/src/backend/hermes_client.rs`

**问题**: 原始代码使用 `blocking_recv()`

```rust
// 修改前 - 导致 panic
impl Stream for HermesStream {
    fn poll_next(&mut self) -> Option<Result<StreamResponse, Box<dyn Error + Send + Sync>>> {
        match self.receiver.blocking_recv() {
            Some(result) => Some(result),
            None => None,
        }
    }
}
```

**错误**: `tokio::sync::mpsc::Receiver` 不支持 `blocking_recv()`，会导致 panic:
```
thread 'tokio-rt-worker' (526367) panicked at src/backend/hermes_client.rs:45:29
```

**修改后**:

```rust
// 修改后 - 使用 try_recv
impl Stream for HermesStream {
    fn poll_next(&mut self) -> Option<Result<StreamResponse, Box<dyn Error + Send + Sync>>> {
        match self.receiver.try_recv() {
            Ok(result) => Some(result),
            Err(_) => None,
        }
    }
}
```

**问题**: `try_recv()` 是非阻塞的，在异步上下文中会立即返回 `None`，导致流立即结束。

---

### 2. 添加详细日志

**文件**: `src-tauri/src/backend/hermes_client.rs`

添加了以下日志点：
- `[HermesClient] send_message_internal called`
- `[HermesClient] api_key present`
- `[HermesClient] Async task started for: {message}`
- `[HermesClient] Adding Authorization header`
- `[HermesClient] HTTP request result`
- `[HermesClient] Response status`
- `[HermesClient] Received N bytes from SSE stream`
- `[HermesClient] SSE chunk text`

---

## 当前日志输出

```
[HermesClient] send_message_internal called with: What?
[HermesClient] api_key present: true
[Router] send_message via Hermes: What? (attachments: 0)
[HermesClient] Async task started for: What?
[Hermes] stream ended
[HermesClient] Adding Authorization header
[HermesClient] HTTP request result: Ok(Response { url: "...", status: 502, ... })
[HermesClient] Response status: 502 Bad Gateway
```

**关键发现**:
1. `[Hermes] stream ended` 在 `Adding Authorization header` 之前出现 - 说明 `handle_hermes_stream` 的 `poll_next()` 立即返回了 None
2. HTTP 请求发出了但返回 502

---

## 错误假设排除

| 假设 | 验证结果 |
|------|----------|
| Hermes API 本身故障 | ❌ 排除 - curl 正常 |
| API key 错误 | ❌ 排除 - 日志显示 `api_key present: true` |
| URL 错误 | ❌ 排除 - URL 正确 |
| 请求 body 格式错误 | ❌ 排除 - curl 相同格式正常 |
| ActiveBackend 大小写 | ❌ 排除 - 反序列化使用 `to_lowercase()` |
| 前端没有调用正确命令 | ❌ 排除 - 日志显示 `[Router] send_message via Hermes` |

---

## 待解决问题

**502 Bad Gateway** - Hermes 收到了请求但处理失败

curl 和 Rust 使用相同 URL、相同 header、相同 body，但结果不同。可能原因：

1. **异步处理时序问题**: `try_recv()` 非阻塞导致流提前结束
2. **HTTP 客户端差异**: reqwest vs curl 的行为差异
3. **Streaming 处理逻辑问题**: `poll_next` 实现不适合异步上下文

---

## 下一步调试方向

1. 先用非 streaming 模式测试 Rust 代码是否能成功调用 Hermes
2. 检查 Hermes 端日志确认是否收到请求
3. 考虑使用 `tokio::sync::mpsc::Receiver` 的正确异步接收方式

---

## 相关文件

- `src-tauri/src/backend/hermes_client.rs` - Hermes 客户端实现
- `src-tauri/src/backend/hermes_discovery.rs` - 配置发现
- `src-tauri/src/backend/manager.rs` - 后端管理器
- `src-tauri/src/lib.rs` - 命令路由

---

## Hermes API 文档参考

文档位置: `~/.hermes/hermes-agent/website/docs/user-guide/features/api-server.md`

**关键信息**:
- 端点: `POST /v1/chat/completions`
- 认证: `Authorization: Bearer {API_SERVER_KEY}`
- Streaming: 设置 `"stream": true`，返回 SSE
- Model 字段: 必须是 `"hermes-agent"` 或 profile 名称
