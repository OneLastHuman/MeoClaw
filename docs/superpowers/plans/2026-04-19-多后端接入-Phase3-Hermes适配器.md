# 多后端接入 - Phase 3: Hermes 适配器实现

> **Status:** Ready to Start

**Goal:** 实现 Hermes 客户端适配器，使其符合 BackendClient trait 接口

**Architecture:** Hermes 使用 HTTP + SSE (Server-Sent Events) 模式，比 OpenClaw 的回调模式更简单

**Tech Stack:** Rust (Tauri v2), reqwest, futures-util, tokio

---

## 文件结构

```
src-tauri/src/
├── backend/
│   ├── hermes_client.rs  # ✅ 占位已创建，待实现
│   └── mod.rs              # ✅ 已更新导出
```

---

## 与 OpenClaw 的模式对比

| 特性 | OpenClaw | Hermes |
|------|----------|--------|
| 连接类型 | WebSocket (持久连接) | HTTP + SSE (短连接) |
| 通信模式 | 回调模式 | 请求-响应 + 流式事件 |
| API 风格 | 双向 WebSocket | REST + SSE |
| 复杂度 | 高 (回调到 Stream 桥接) | 中 (SSE本身就是流) |

---

## 实现状态

### 📋 Task 1: HermesClient 结构定义

**Files:**
- `src-tauri/src/backend/hermes_client.rs`

**实现内容:**

```rust
pub struct HermesClient {
    endpoint: String,
    client: reqwest::Client,
}

impl HermesClient {
    pub fn new() -> Self {
        Self {
            endpoint: "http://127.0.0.1:8642".to_string(),
            client: reqwest::Client::new(),
        }
    }
}
```

---

### 📋 Task 2: 实现 BackendClient trait

**实现内容:**

```rust
impl BackendClient for HermesClient {
    fn name(&self) -> &str { "hermes" }

    fn check_health(&self) -> bool {
        // HTTP GET /health 检测
    }

    fn send_message(&self, text: &str) -> Result<Box<dyn Stream>, ...> {
        // POST /v1/chat/completions
        // 解析 SSE 响应
        // 返回 HermesStream
    }

    fn endpoint(&self) -> &str { &self.endpoint }

    fn auth_headers(&self) -> HashMap<String, String> { HashMap::new() }
}
```

---

### 📋 Task 3: HermesStream 实现

**实现内容:**

```rust
pub struct HermesStream {
    reader: BufReader<TcpStream>,
}

impl Stream for HermesStream {
    fn poll_next(&mut self) -> Option<Result<StreamResponse, ...>> {
        // 读取 SSE 事件
        // 解析 data: {...} 格式
        // 转换为 StreamResponse
    }
}
```

---

### 📋 Task 4: 单元测试

**测试用例:**

```rust
#[test] fn test_hermes_client_name() { ... }
#[test] fn test_hermes_client_endpoint() { ... }
#[test] fn test_hermes_client_auth_headers() { ... }
#[test] fn test_sse_parsing() { ... }
```

---

## 验收标准

- [ ] HermesClient 实现 BackendClient trait
- [ ] name() 返回 "hermes"
- [ ] endpoint() 返回 "http://127.0.0.1:8642"
- [ ] check_health() 通过 HTTP 健康检测
- [ ] send_message() 返回实现了 Stream 的对象
- [ ] 单元测试全部通过
- [ ] 与 BackendManager 集成测试通过

---

## Phase 2 遗留问题

**`send_message()` 返回模拟响应**

Phase 2 的 OpenClaw 适配器因为模式差异，暂时返回模拟响应。Phase 3 实现后，可以：
1. 在 Hermes 上验证 Stream 实现的正确性
2. 评估是否需要重构 OpenClaw 适配器以支持真实 WebSocket

---

## Self-Review 检查清单

- [ ] HermesClient 实现 BackendClient trait
- [ ] name() 返回 "hermes"
- [ ] endpoint() 返回 "http://127.0.0.1:8642"
- [ ] check_health() 通过 HTTP 连接检测
- [ ] auth_headers() 返回空 HashMap
- [ ] send_message() 返回实现了 Stream 的对象
- [ ] 单元测试全部通过

---

## Execution Options

**1. Subagent-Driven (recommended)** - 适合复杂任务，多个子任务可以并行执行

**2. Inline Execution** - Execute tasks in this session

Which approach?

---

## Task 拆分 (Subagent-Driven)

### Task 3.1: HermesClient 基础结构
- 实现 HermesClient 结构体
- 实现 BackendClient trait 基本部分

### Task 3.2: HermesStream SSE 流式响应
- 实现 HermesStream
- 实现 SSE 解析逻辑
- 实现 Stream trait

### Task 3.3: 健康检测与单元测试
- 实现 check_health()
- 编写单元测试
- 验证编译通过

### Task 3.4: 与 BackendManager 集成
- 注册 HermesClient 到 BackendManager
- 运行集成测试
