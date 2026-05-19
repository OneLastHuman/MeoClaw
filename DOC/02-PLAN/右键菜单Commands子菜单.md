# 右键菜单 Commands 子菜单

## 需求
- 右键菜单新增 **Commands** 子菜单
- 包含 `/new`、`/stop`、`/status` 三个命令选项
- 选中后向 OpenClaw 发送对应命令，响应内容显示在 Response 气泡中

---

## 实现

### 1. 前端：右键菜单子菜单

修改 `src/menu/contextMenu.ts`：

```typescript
import { Menu, MenuItem, Submenu } from '@tauri-apps/api/menu';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { invoke } from '@tauri-apps/api/core';

// OpenClaw 命令子菜单
const newCmdItem = await MenuItem.new({
  text: '/new',
  async action() {
    console.log('[MENU] /new clicked');
    await invoke('send_to_openclaw', { message: '/new', attachments: [] });
  },
});

const stopCmdItem = await MenuItem.new({
  text: '/stop',
  async action() {
    console.log('[MENU] /stop clicked');
    await invoke('send_to_openclaw', { message: '/stop', attachments: [] });
  },
});

const statusCmdItem = await MenuItem.new({
  text: '/status',
  async action() {
    console.log('[MENU] /status clicked');
    await invoke('send_to_openclaw', { message: '/status', attachments: [] });
  },
});

const commandsSubmenu = await Submenu.new({
  text: 'Commands',
  items: [newCmdItem, stopCmdItem, statusCmdItem],
});

const menu = await Menu.new({
  items: [shockItem, startWorkingItem, workingItem, workingPreviewItem, enterReceivingItem, receivingItem, responseItem, commandsSubmenu],
});
```

### 2. 后端：处理 chat 事件

OpenClaw 对 `/status`、`/stop` 等命令的响应走的是 `chat` 事件，而非 `agent` 事件。需要新增 `ChatEvent` 结构体并处理。

#### 2.1 新增协议结构 (`src-tauri/src/openclaw/protocol.rs`)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessageContent {
    #[serde(rename = "type")]
    pub content_type: String,  // "text"
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: Vec<ChatMessageContent>,
    pub timestamp: Option<i64>,
    #[serde(rename = "stopReason")]
    pub stop_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatEvent {
    #[serde(rename = "runId")]
    pub run_id: String,
    #[serde(rename = "sessionKey")]
    pub session_key: String,
    pub state: String,
    pub message: ChatMessage,
}
```

#### 2.2 处理 chat 事件 (`src-tauri/src/openclaw/client.rs`)

```rust
} else if event == "chat" {
    // 处理 chat 事件（如 /status、/stop 等命令的返回内容）
    if let Ok(chat_event) = serde_json::from_value::<ChatEvent>(payload.clone()) {
        log::info!("[OpenClaw] Chat event: state={}", chat_event.state);
        // 从 message.content 中提取文本
        for content in chat_event.message.content {
            if content.content_type == "text" && !content.text.is_empty() {
                log::info!("[OpenClaw] Chat text: {:.200}", content.text);
                let callback = self.response_callback.read().await;
                if let Some(cb) = callback.as_ref() {
                    cb(content.text);
                }
            }
        }
        // chat 事件的 final state 触发 lifecycle end 回调，切换到 Response 状态
        if chat_event.state == "final" {
            let callback = self.lifecycle_callback.read().await;
            if let Some(cb) = callback.as_ref() {
                let lifecycle = LifecycleEvent {
                    run_id: Some(chat_event.run_id.clone()),
                    session_key: Some(chat_event.session_key.clone()),
                    session_id: None,
                    phase: "end".to_string(),
                    error: None,
                    summary: None,
                };
                cb(lifecycle);
            }
        }
    }
}
```

### 3. 事件流

```
用户点击 /status
  → invoke('send_to_openclaw', '/status')
  → Rust: client.sessions_send('/status')
  → OpenClaw WebSocket 返回 chat 事件
  → Rust: handle_message 解析 event == "chat"
  → 调用 response_callback (显示气泡文本)
  → 调用 lifecycle_callback(phase="end") → 前端切换到 Response 状态
  → 前端: showResponseBubble = true → 气泡显示
```

---

## 文件结构

```
src/
├── menu/
│   └── contextMenu.ts          # 右键菜单（修改：新增 Submenu 和 invoke 调用）
src-tauri/src/
├── openclaw/
│   ├── client.rs               # WebSocket 客户端（修改：处理 chat 事件）
│   └── protocol.rs             # 协议结构体（修改：新增 ChatEvent 等）
```

---

## 调试日志

```bash
# 查看 chat 事件
grep "\[OpenClaw\] Chat" /tmp/meo_debug.log

# 查看 response 气泡发送
grep "Emit response bubble" /tmp/meo_debug.log

# 查看状态切换
grep "switch-animation" /tmp/meo_debug.log
```
