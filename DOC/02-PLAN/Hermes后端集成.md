# Hermes 后端集成

> 创建时间：2026/04/22
> 状态：已完成 ✅

---

## 目录

1. [系统架构](#系统架构)
2. [技术实现](#技术实现)
3. [配置说明](#配置说明)
4. [API 接口](#api-接口)
5. [问题排查](#问题排查)
6. [踩坑经验](#踩坑经验)

---

## 系统架构

### 整体链路

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│ Frontend (Vue)                                                                 │
│ DesktopPet.vue: onEnterPress() → invoke('send_message')                       │
└─────────────────────────────────────────────────────────────────────────────────┘
                                    ↓ Tauri IPC
┌─────────────────────────────────────────────────────────────────────────────────┐
│ Tauri Command Layer (lib.rs)                                                   │
│ send_message() → 根据 current_backend 路由                                        │
└─────────────────────────────────────────────────────────────────────────────────┘
                                    ↓
┌─────────────────────────────────────────────────────────────────────────────────┐
│ Hermes Backend                                                                 │
│ HTTP POST /v1/chat/completions → Streaming SSE Response                        │
│ http://127.0.0.1:8642                                                          │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### 消息流程

1. **Frontend** 调用 `invoke('send_message', { message, attachments })`
2. **Rust** 接收后查询 `BackendManager.current()` 获取当前后端
3. 若为 `hermes`，在 `tokio::spawn` 中直接处理 HTTP 请求
4. Hermes API 返回 SSE 流式响应
5. Rust 解析 SSE，逐步累积文本
6. 收到最终响应时：
   - 写入状态文件 `/tmp/meo_anim_state.txt`
   - 发送 `update-response-bubble` 事件（气泡更新）
   - 发送 `switch-animation` 事件（动画切换）

---

## 技术实现

### 核心代码结构

#### 1. Hermes 配置发现 (`hermes_discovery.rs`)

```rust
pub struct HermesDiscovery;

impl HermesDiscovery {
    // 三级发现机制：
    // 1. Layer 1: 从 ~/.hermes/.env 读取
    // 2. Layer 2: 进程扫描
    // 3. Layer 3: 默认端口尝试验证
    pub fn discover() -> Option<HermesConfig> { ... }
}
```

**HermesConfig 结构**：
```rust
pub struct HermesConfig {
    pub endpoint: String,   // e.g., "http://127.0.0.1:8642"
    pub api_key: Option<String>,
    pub enabled: bool,
}
```

#### 2. Hermes HTTP 响应处理 (`lib.rs`)

**关键设计**：直接在 async task 中处理 HTTP 请求和 SSE 响应，**不使用** `Stream` trait。

```rust
match current_backend.as_str() {
    "hermes" => {
        let message_text = message.clone();
        let app_for_task = app.clone();
        tokio::spawn(async move {
            // 直接构造 HTTP 请求
            let config = backend::HermesDiscovery::discover();
            let client = reqwest::Client::new();
            // ... 发送请求 ...
            
            // 处理 SSE 流
            let mut stream = resp.bytes_stream();
            while let Some(chunk_result) = futures_util::StreamExt::next(&mut stream).await {
                // 解析 SSE 格式: data: {...}
                // 累积文本
                // 最终响应时写入状态文件和发送事件
            }
        });
    }
}
```

#### 3. 状态切换机制

**OpenClaw** 和 **Hermes** 共用同一套状态切换机制：

```
最终响应到达
    ↓
fs::write(STATE_FILE, "Response\n")  →  触发文件监控器
    ↓
emit("switch-animation", "Response")  →  Frontend 切换动画状态
    ↓
emit("update-response-bubble", { text })  →  显示回复气泡
```

**状态文件**：`/tmp/meo_anim_state.txt`

### Hermes API 调用格式

**请求**：
```json
POST http://127.0.0.1:8642/v1/chat/completions
Authorization: Bearer {API_SERVER_KEY}
Content-Type: application/json

{
    "model": "assistant",
    "messages": [{"role": "user", "content": "消息内容"}],
    "stream": true
}
```

**SSE 响应格式**：
```
data: {"id":"...","object":"chat.completion.chunk","choices":[{"index":0,"delta":{"role":"assistant"},"finish_reason":null}]}

data: {"id":"...","object":"chat.completion.chunk","choices":[{"index":0,"delta":{"content":"Hello"},"finish_reason":null}]}

data: {"id":"...","object":"chat.completion.chunk","choices":[{"index":0,"delta":{},"finish_reason":"stop"}],"usage":{...}}

data: [DONE]
```

---

## 配置说明

### Hermes 端配置 (`~/.hermes/.env`)

```bash
API_SERVER_ENABLED=true
API_SERVER_KEY=meo-claw-secret-key-2024
API_SERVER_PORT=8642
API_SERVER_HOST=127.0.0.1
```

### 验证 Hermes 配置

```bash
# 健康检查
curl http://127.0.0.1:8642/health

# 验证 API Key
curl -X POST http://127.0.0.1:8642/v1/chat/completions \
  -H "Authorization: Bearer meo-claw-secret-key-2024" \
  -H "Content-Type: application/json" \
  -d '{"model":"assistant","messages":[{"role":"user","content":"hi"}],"stream":false}'
```

---

## API 接口

### Tauri Commands

| 命令 | 参数 | 说明 |
|------|------|------|
| `send_message` | `message: String, attachments: Vec<Attachment>` | 统一消息发送，根据后端自动路由 |
| `send_to_hermes` | `message: String` | 直接发送到 Hermes（已不推荐使用） |
| `test_hermes` | 无 | 测试 Hermes API 连接 |

### Frontend Events

| 事件名 | Payload | 说明 |
|--------|---------|------|
| `update-response-bubble` | `{ text: String }` | 更新回复气泡内容 |
| `switch-animation` | `String` (状态名) | 切换动画状态 |
| `update-bubble` | `{ tools: [...] }` | 更新工具气泡 |

### Working Bubble 支持

Hermes 支持工具执行时的 Working Bubble 显示：

- **开始处理**：发送 `update-bubble` 显示 "思考中..."
- **工具进度**：解析 SSE `event: hermes.tool.progress` 事件，更新 bubble 显示工具信息
- **处理完成**：发送 `update-bubble { tools: [] }` 清空 bubble

Hermes 工具事件格式：
```
event: hermes.tool.progress
data: {"tool": "terminal", "emoji": "💻", "label": "ls"}
```

详细说明见：[Hermes-Working-Bubble修复方案.md](./Hermes-Working-Bubble修复方案.md)

### 状态值

| 状态 | 说明 |
|------|------|
| `idle` | 空闲 |
| `working` | 处理中 |
| `Response` | 收到回复 |

---

## 问题排查

### 排查步骤

1. **检查 Hermes 是否运行**
   ```bash
   curl http://127.0.0.1:8642/health
   ```

2. **检查 API Key**
   ```bash
   grep API_SERVER_KEY ~/.hermes/.env
   ```

3. **检查 MeoClaw 日志**
   ```bash
   tail -100 /tmp/meo_debug.log
   ```

4. **检查状态文件**
   ```bash
   cat /tmp/meo_anim_state.txt
   ```

### 常见问题

| 问题 | 原因 | 解决方案 |
|------|------|---------|
| Hermes 返回 401 | API Key 不正确 | 确认 `~/.hermes/.env` 中的 `API_SERVER_KEY` |
| 前端一直 working | 状态文件未写入 | 检查 `handle_hermes_stream` 是否调用 `fs::write` |
| 流提前结束 | `poll_next` 使用 `try_recv` | 见踩坑经验 |
| Hermes 无响应 | Hermes 未启动 | 启动 `hermes gateway` |

---

## 踩坑经验

### 问题 1：Stream trait 与 async 上下文不兼容（致命）

**现象**：前端一直卡在 working 状态，Hermes 后台已收到并回复消息。

**根因**：最初使用 `Stream` trait 包装 Hermes 响应，但 `poll_next()` 是同步方法，在 async 上下文中调用会死锁：

```rust
// 错误的实现 - HermesStream poll_next
impl Stream for HermesStream {
    fn poll_next(&mut self) -> Option<Result<StreamResponse, ...>> {
        match self.receiver.try_recv() {  // 问题1: try_recv 非阻塞，数据未到就返回 None
            Ok(result) => Some(result),
            Err(_) => None,
        }
    }
}
```

**修复**：直接在 async task 中处理 HTTP 请求，不使用 `Stream` trait。

**教训**：Rust 的 `Stream` trait（`poll_next` 返回 `Option`）是给同步上下文用的，在 async 代码中应该直接用 `tokio::sync::mpsc::Receiver.recv().await`。

---

### 问题 2：try_recv vs blocking_recv 混淆

**现象**：`poll_next` 使用 `try_recv()` 导致流立即结束。

**根因**：
- `try_recv()`：非阻塞，立即返回。channel 有数据就返回数据，没有就返回 `Err`。
- `blocking_recv()`：阻塞等待，直到有数据或 channel 关闭。

在异步上下文中，消息通过网络传输到达需要时间，`try_recv()` 会立即返回 `None`，误判为流已结束。

**教训**：处理异步流时，不要用 `try_recv()`，应该用 await 等待数据到达。

---

### 问题 3：状态切换缺少两个必要步骤

**现象**：Hermes 回复了，但前端没有切换到 Response 状态。

**根因**：对比 OpenClaw 的 lifecycle 回调，Hermes 处理缺少：
1. `fs::write(STATE_FILE, "Response\n")` - 写入状态文件
2. `emit("switch-animation", "Response")` - 发送状态切换事件

**修复**：在收到最终响应时，同时执行这两个操作。

**教训**：状态切换需要三方协同：Rust 写状态文件 → 文件监控器检测 → Frontend 接收事件。两者缺一不可。

---

### 问题 4：API Key 值与文档不符

**现象**：按照文档中的 `change-me-local-dev` 测试失败。

**根因**：Hermes 实际使用的 API Key 是 `meo-claw-secret-key-2024`，不是文档中的默认值。

**教训**：不要假设文档值正确，必须以实际配置文件 `~/.hermes/.env` 为准。

---

### 问题 5：SSE 格式解析的边界情况

**现象**：某些 Hermes 响应中的首个 chunk 只有 `role` 字段，没有 `content`。

**SSE 示例**：
```
data: {"choices":[{"delta":{"role":"assistant"},"finish_reason":null}]}  // 只有 role，无 content
data: {"choices":[{"delta":{"content":"Hello"},"finish_reason":null}]}  // 有 content
```

**原代码问题**：
```rust
if let Some(content) = json["choices"][0]["delta"]["content"].as_str() {
    // 发送 content
}
// 首个 chunk 没有 content，这里什么都没做，但 finish_reason 此时可能是 null
```

**修复**：正确判断 `finish_reason === "stop"` 作为最终响应的依据，而不是依赖 content 是否存在。

**教训**：解析 SSE 时，`finish_reason === "stop"` 才是最终响应的可靠标志，不能依赖 content 字段是否存在。

---

## 相关文件索引

| 文件 | 说明 |
|------|------|
| `src-tauri/src/backend/hermes_discovery.rs` | Hermes 配置发现 |
| `src-tauri/src/backend/hermes_client.rs` | Hermes 客户端（当前未使用 Stream 模式） |
| `src-tauri/src/lib.rs` | 命令路由和 Hermes 响应处理 |
| `src-tauri/src/backend/manager.rs` | 后端管理器 |
| `DOC/功能/Hermes后端集成调试记录-2026-04-21.md` | 旧调试记录 |

---

## 总结

Hermes 后端集成的核心是：**在 tokio async task 中直接处理 HTTP SSE 响应，避免使用同步的 Stream trait**。

关键设计点：
1. **不要在 async 代码中混用同步 Stream trait**
2. **状态切换需要同时：写状态文件 + 发事件**
3. **以实际配置文件为准，不要假设文档值**
4. **SSE 解析用 `finish_reason === "stop"` 判断结束**
