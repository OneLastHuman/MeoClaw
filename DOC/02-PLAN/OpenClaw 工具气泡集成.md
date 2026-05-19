# OpenClaw 工具气泡集成

## 功能概述

桌宠在 `working` 状态时，通过 WebSocket 连接 OpenClaw Gateway，监听工具调用事件，实时显示当前正在执行的工具名称。工具执行完成后进入 `Response` 状态，显示 OpenClaw 的最终回答。

### 效果
- 进入 `working` 状态后，右上角显示气泡
- 工具调用时，气泡显示具体工具名和参数（如 `执行：curl -s "wttr.in...`）
- 工具执行完成后切换到 `Response` 状态，气泡显示 OpenClaw 的最终回答
- Response 气泡持续显示回答内容，直到用户操作退出 Response 状态

---

## 架构流程

```
用户输入文字
       ↓
DesktopPet.vue → invoke('send_to_openclaw')
       ↓
Rust lib.rs → send_to_openclaw command
       ↓
client.sessions_send() 通过 mpsc channel 发送到 WebSocket
       ↓
OpenClaw Gateway → session.message 事件
       ↓
client.handle_message() 解析事件
       ↓
tool_callback / lifecycle_callback 触发
       ↓
app.emit('update-bubble') → DesktopPet.vue
       ↓
气泡更新显示
```

### 两种事件类型

| 事件类型 | 说明 |
|---------|------|
| `session.tool` | 工具调用事件，包含 name/phase/args |
| `agent` (stream: lifecycle) | 会话生命周期事件，phase: start/end |

---

## 关键文件

| 文件 | 作用 |
|------|------|
| `src-tauri/src/openclaw/client.rs` | WebSocket 客户端，连接 OpenClaw Gateway |
| `src-tauri/src/openclaw/protocol.rs` | 协议消息结构体定义 |
| `src-tauri/src/openclaw/auth.rs` | 设备认证和签名 |
| `src-tauri/src/lib.rs` | Tauri 命令和回调设置 |
| `src/components/DesktopPet.vue` | 前端状态管理和气泡显示 |

---

## 协议实现细节

### WebSocket 连接流程

1. 连接 `ws://127.0.0.1:18789`
2. 接收 `connect.challenge` 事件，获取 nonce
3. 使用设备私钥签名认证请求
4. 发送 `connect` 请求，订阅 `sessions.subscribe` 和 `sessions.messages.subscribe`
5. 进入主事件循环

### 订阅 session messages

```rust
// 注意：必须使用 "key" 而不是 "sessionKey"
let msg_subscribe_req = WsMessage::sessions_messages_subscribe("sub-2", &session_key);
```

### sessions.send 格式

```rust
let params = serde_json::json!({
    "key": session_info.key,  // 不是 "sessionKey"！
    "message": message
});
```

---

## 踩过的坑

### Bug 1: 字符串截断导致 Panic

**问题**：日志中打印原始消息时使用字节截断，UTF-8 中文字符被截断导致 panic：
```
byte index 500 is not a char boundary; it is inside '日'
```

**位置**：`client.rs:175`
```rust
// 错误代码
log::info!("[OpenClaw] RAW WS MSG: {}", &text[..text.len().min(500)]);

// 正确代码（使用日志框架的截断）
log::info!("[OpenClaw] RAW WS MSG: {:.500}", text);
```

---

### Bug 2: session.tool 事件不在 session.message 中

**问题**：工具调用事件是通过 `session.tool` 事件类型接收的，而不是嵌套在 `session.message` 的 payload 里。

**日志证据**：
```json
{"type":"event","event":"session.tool","payload":{...}}
```

**处理方式**：在 `handle_message` 中单独处理 `session.tool` 事件：
```rust
} else if event == "session.tool" {
    if let Ok(tool_event) = serde_json::from_value::<ToolEvent>(payload.clone()) {
        let tool_name = tool_event.data.name.clone().unwrap_or_default();
        let phase = tool_event.data.phase.clone();
        let args = tool_event.data.args.clone().unwrap_or(serde_json::Value::Null);
        // 调用 tool 回调
    }
}
```

---

### Bug 3: lifecycle 事件是 agent 类型，不是 session.message

**问题**：会话生命周期的 start/end 事件是通过 `agent` 事件（stream: lifecycle）接收的：
```json
{"type":"event","event":"agent","payload":{"stream":"lifecycle","data":{"phase":"start",...}}}
```

之前只处理了 `session.message` 中的 lifecycle，导致状态切换无法触发。

**处理方式**：添加 `AgentEvent` 结构体和处理逻辑：
```rust
} else if event == "agent" {
    if let Ok(agent_event) = serde_json::from_value::<AgentEvent>(payload.clone()) {
        if agent_event.stream == "lifecycle" {
            if let Some(phase) = &agent_event.data.phase {
                // 调用 lifecycle 回调
            }
        }
    }
}
```

---

### Bug 4: sessions.messages.subscribe 参数错误

**问题**：使用 `sessionKey` 作为参数名导致 INVALID_REQUEST 错误：
```json
{"error": "unexpected property 'sessionKey'"}
```

**正确参数**：
```rust
params: serde_json::json!({ "key": session_key })
```

---

### Bug 5: emit 时监听器未注册

**问题**：在 `onMounted` 中先调用 `invoke('test_bubble')`，后注册监听器，导致事件丢失。

**正确顺序**：
```typescript
onMounted(async () => {
  // 1. 先注册监听器
  unlistenBubble = await listen<BubblePayload>('update-bubble', (event) => {
    // ...
  });

  // 2. 再调用会触发事件的代码
  await invoke('test_bubble');
});
```

---

### Bug 6: 前端显示循环气泡而非真实数据

**问题**：`working` 状态无条件显示气泡，没有区分是否有真实工具调用数据。

**解决**：添加 `hasRealBubbleData` 标志：
```typescript
const showBubble = computed(() => animState.value === 'working' && hasRealBubbleData.value);
const hasRealBubbleData = ref(false);

// 监听器收到真实数据后设置为 true
if (event.payload.texts && event.payload.texts.length > 0) {
    bubbleTexts.value = event.payload.texts;
    hasRealBubbleData.value = true;
}
```

---

## 调试方法

### 日志追踪

启动应用并记录日志：
```bash
npm run tauri dev 2>&1 | tee /tmp/meo_debug.log &
```

实时查看：
```bash
tail -f /tmp/meo_debug.log
```

### 关键日志过滤

```bash
# 查看工具事件和气泡发射
grep -E "(session\.tool|Tool|Agent lifecycle|emit.*bubble|Bubble text)" /tmp/meo_debug.log

# 查看 WebSocket 原始消息（注意：修复后使用 {:.500} 截断）
grep "RAW WS MSG" /tmp/meo_debug.log

# 查看会话发送
grep "sessions.send" /tmp/meo_debug.log
```

### 测试 Rust → Frontend emit 链路

使用 `test_bubble` 命令验证 emit 是否正常工作：
```rust
// 在 lib.rs 中
#[tauri::command]
async fn test_bubble(app: AppHandle) -> Result<String, String> {
    let payload = BubblePayload {
        texts: vec!["测试：exec ···".to_string()],
        interval: Some(2000),
    };
    match app.emit("update-bubble", payload) {
        Ok(_) => Ok("Bubble sent!".to_string()),
        Err(e) => Err(format!("Emit failed: {}", e)),
    }
}
```

---

## 工具气泡格式化

Rust 端根据工具名称格式化气泡文案：

```rust
fn format_tool_bubble(name: &str, args: &Value) -> String {
    match name {
        "read" => {
            if let Some(path) = args.get("path").and_then(|v| v.as_str()) {
                let short = Path::new(path)
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_else(|| path.to_string());
                format!("查看：{} ···", short)
            } else {
                "查看中···".to_string()
            }
        }
        "exec" => {
            if let Some(cmd) = args.get("command").and_then(|v| v.as_str()) {
                let short = if cmd.len() > 20 {
                    format!("{}...", &cmd[..20])
                } else {
                    cmd.to_string()
                };
                format!("执行：{} ···", short)
            } else {
                "执行中···".to_string()
            }
        }
        // ... 其他工具
        _ => format!("{}中···", name),
    }
}
```

---

## OpenClaw 事件流详解

一次完整的工具调用会话事件序列：

```
1. agent (stream: lifecycle)     → phase: "start"        # 会话开始，清空回答累积
2. session.tool (phase: start)   → name: "read"          # 工具开始
3. session.tool (phase: result)  → name: "read"          # 工具结果
4. agent (stream: assistant)     → delta: "部分文本..."   # 回答文本（持续）
5. session.tool (phase: start)   → name: "exec"          # 下一个工具
6. session.tool (phase: update) → name: "exec"          # 执行中
7. agent (stream: assistant)     → delta: "更多文本..."   # 回答文本（持续）
8. session.tool (phase: result)  → name: "exec"          # 执行完成
9. agent (stream: assistant)     → delta: "最终回答..."   # 完整回答
10. agent (stream: lifecycle)     → phase: "end"          # 会话结束，切换 Response
```

**注意**：
- `agent (stream: assistant)` 事件在工具执行期间持续发送，携带回答的 `delta`
- 多个 delta 需要累积才能得到完整回答
- lifecycle end 时切换到 Response 状态，而不是 idle

---

## 状态机转换

```
idle ──────────────────────────────────────────────────
  │                                                           │
  │ 用户输入文字，invoke('send_to_openclaw')                  │
  ↓                                                           │
working                                                      │
  │                                                           │
  │ 收到 lifecycle start (agent event)                       │
  │ 工具调用 → 显示工具气泡                                   │
  │ 收到 assistant delta → 累积回答文本                       │
  │                                                           │
  │ 收到 lifecycle end (agent event)                         │
  ↓                                                           │
Response                                                     │
  │                                                           │
  │ assistant delta 持续更新 responseBubbleText               │
  │                                                           │
  │ 用户操作（如点击）→ 切换到 idle                          │
  ↓                                                           │
idle ◀────────────────────────────────────────────────────
```

---

## Response 气泡实现

### 前端数据结构

```typescript
// Response 气泡状态
const showResponseBubble = computed(() => animState.value === 'Response');
const responseBubbleText = ref('思考中...'); // 初始占位

// 监听最终回答事件
unlistenResponseBubble = await listen<ResponseBubblePayload>('update-response-bubble', (event) => {
  responseBubbleText.value = event.payload.text;
});
```

### Rust 端 response 回调

```rust
// 设置 response 回调（用于显示最终回答气泡）
let cb_app3 = app_handle.clone();
client.set_response_callback(Arc::new(move |full_text: String| {
    log::info!("[OpenClaw] Response text received: {:.100}", full_text);

    // 发送 response 气泡更新事件
    let payload = ResponseBubblePayload { text: full_text };
    match cb_app3.emit("update-response-bubble", payload) {
        Ok(_) => log::info!("[OpenClaw] Emit response bubble SUCCESS"),
        Err(e) => log::error!("[OpenClaw] Failed to emit response bubble: {}", e),
    }
}));
```

### Lifecycle end 行为变更

之前 lifecycle end 时写入 `"idle"`，现在改为写入 `"Response"`：

```rust
let state = if event.is_start() {
    "working"
} else if event.is_end() || event.is_error() {
    "Response"  // 不再是 idle
} else {
    return;
};
```

### assistant 事件处理

`client.rs` 中处理 `stream == "assistant"` 事件，累积 delta：

```rust
} else if agent_event.stream == "assistant" {
    if let Some(delta) = &agent_event.data.delta {
        if !delta.is_empty() {
            // 累积 delta 到完整回答
            let mut acc = self.accumulated_response.write().await;
            acc.push_str(delta);

            // 调用 response 回调（带累积的完整文本）
            let callback = self.response_callback.read().await;
            if let Some(cb) = callback.as_ref() {
                let acc = self.accumulated_response.read().await;
                cb(acc.clone());
            }
        }
    }
}
```

---

## 相关文档

- `Working状态气泡.md` - 气泡基础实现（静态循环版本）
- `OpenClaw状态检测集成.md` - OpenClaw 连接和状态同步
- `Response状态与窗口位置调整.md` - Response 窗口处理
