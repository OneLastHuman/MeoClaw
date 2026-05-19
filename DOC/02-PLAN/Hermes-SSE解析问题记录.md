# Hermes SSE 解析问题记录

> 创建时间：2026/04/23
> 状态：未修复
> 影响功能：Hermes 后端消息发送

---

## 一、问题现象

**用户操作**：在前端输入消息并发送

**预期结果**：Hermes 后端返回回复，消息气泡显示内容

**实际结果**：消息一直显示 "working" 状态，没有收到任何回复

---

## 二、问题根因

### 2.1 直接原因

Hermes 后端返回的是 **SSE（Server-Sent Events）** 格式的数据流。

当前代码在解析 SSE 时，可能没有正确处理数据格式，导致消息无法正确显示。

### 2.2 Hermes 返回的数据格式

```json
data: {"choices":[{"delta":{"content":"Hello"}}]}
data: {"choices":[{"delta":{"content":" world"}}]}
data: [DONE]
```

每行以 `data: ` 开头，后面是 JSON 数据。最后一行 `data: [DONE]` 表示消息结束。

---

## 三、排查过程

| 步骤 | 操作 | 结果 |
|------|------|------|
| 1 | curl 测试 Hermes API | ✅ API 本身正常工作，能收到完整回复 |
| 2 | 检查网络连接 | ✅ 可以连接到 Hermes |
| 3 | 检查日志 | ⚠️ 日志显示收到了数据，但解析失败 |

---

## 四、技术细节

### 4.1 涉及的代码文件

| 文件 | 功能 |
|------|------|
| `src-tauri/src/backend/hermes_client.rs` | Hermes 客户端，SSE 解析逻辑 |
| `src-tauri/src/backend/hermes_discovery.rs` | Hermes 发现机制 |

### 4.2 SSE 解析关键代码位置

```rust
// hermes_client.rs 第 199-255 行
async fn parse_sse_events(text: &str, response_tx: &mpsc::Sender<...>) {
    for line in text.lines() {
        let line = line.trim();
        if let Some(data) = line.strip_prefix("data: ") {
            // 解析 JSON
            // ...
        }
    }
}
```

---

## 五、修复方向

### 5.1 可能的原因

1. **JSON 解析位置不准确** - `json["choices"][0]["delta"]["content"]` 路径可能不对
2. **多行数据处理** - 一个 chunk 可能包含多行 SSE 数据，需要逐行处理
3. **字符编码问题** - 中文或其他特殊字符可能导致解析失败
4. **异步处理时序** - tokio spawn 的异步任务和 channel 通信可能有时序问题

### 5.2 建议的修复方向

1. 在日志中完整打印收到的原始 SSE 数据
2. 逐字符调试 JSON 解析路径
3. 检查是否有多行数据合并在一个 chunk 中的情况
4. 添加更详细的错误日志

---

## 六、复现条件

| 条件 | 说明 |
|------|------|
| Hermes 后端运行 | 需要启动 Hermes API 服务 |
| API Key 正确 | ~/.hermes/.env 中的 API_SERVER_KEY 与 Hermes 配置一致 |
| 网络通畅 | 本地 127.0.0.1:8642 可访问 |

---

## 七、相关文档

- `DOC/功能/Hermes自动发现机制设计文档_v2.md` - Hermes 发现机制
- `DOC/Hermes配置与文件说明.md` - Hermes 配置说明
