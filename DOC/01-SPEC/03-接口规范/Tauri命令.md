# Tauri 命令

本文档列出所有 `#[tauri::command]` 定义及其详细信息。

## 命令列表

### 1. switch_animation

**位置**: `src-tauri/src/lib.rs:188`

**签名**:
```rust
#[tauri::command]
fn switch_animation(state: &str, app: AppHandle) -> Result<(), String>
```

**参数**:
| 参数 | 类型 | 说明 |
|-----|------|------|
| `state` | `&str` | 目标动画状态字符串 |
| `app` | `AppHandle` | Tauri 应用句柄 |

**返回值**: `Result<(), String>`

**功能**: 切换桌宠动画状态，验证状态有效性后通过 `app.emit()` 发送 `switch-animation` 事件。

**验证状态**:
- `idle`, `shock`, `EnterInput`, `startworking`, `working`, `workingPreview`
- `EnterReceiving`, `Receiving`, `received`, `Response`

**前端调用**:
```typescript
// 目前通过文件监控自动触发，较少直接调用
```

---

### 2. send_to_openclaw

**位置**: `src-tauri/src/lib.rs:203`

**签名**:
```rust
#[tauri::command]
async fn send_to_openclaw(message: String, attachments: Vec<Attachment>, app: AppHandle) -> Result<(), String>
```

**参数**:
| 参数 | 类型 | 说明 |
|-----|------|------|
| `message` | `String` | 发送的消息内容 |
| `attachments` | `Vec<Attachment>` | 附件列表 |
| `app` | `AppHandle` | Tauri 应用句柄 |

**返回值**: `Result<(), String>`

**功能**: 发送消息到 OpenClaw WebSocket 会话。

**前端调用** (`src/menu/contextMenu.ts`):
```typescript
await invoke('send_to_openclaw', { message, attachments: [] });
// 菜单命令: /new, /stop, /status
```

---

### 3. send_to_hermes

**位置**: `src-tauri/src/lib.rs:213`

**签名**:
```rust
#[tauri::command]
async fn send_to_hermes(message: String, app: AppHandle) -> Result<(), String>
```

**参数**:
| 参数 | 类型 | 说明 |
|-----|------|------|
| `message` | `String` | 发送的消息内容 |
| `app` | `AppHandle` | Tauri 应用句柄 |

**返回值**: `Result<(), String>`

**功能**: 发送消息到 Hermes 后端，启动异步任务处理流式响应。

**注意**: 此命令直接调用 Hermes HTTP API 并处理 SSE 流。

---

### 4. send_message

**位置**: `src-tauri/src/lib.rs:231`

**签名**:
```rust
#[tauri::command]
async fn send_message(message: String, attachments: Vec<Attachment>, app: AppHandle) -> Result<(), String>
```

**参数**:
| 参数 | 类型 | 说明 |
|-----|------|------|
| `message` | `String` | 发送的消息内容 |
| `attachments` | `Vec<Attachment>` | 附件列表 |
| `app` | `AppHandle` | Tauri 应用句柄 |

**返回值**: `Result<(), String>`

**功能**: 统一消息路由命令，根据当前后端自动选择 OpenClaw 或 Hermes。

**路由逻辑**:
```
当前后端 = "hermes" → send_to_hermes
当前后端 = 其他    → send_to_openclaw
```

**前端调用** (`src/components/DesktopPet.vue`):
```typescript
await invoke('send_message', { message: fullMessage, attachments: [] });
```

---

### 5. test_openclaw

**位置**: `src-tauri/src/lib.rs:408`

**签名**:
```rust
#[tauri::command]
async fn test_openclaw(app: AppHandle) -> Result<String, String>
```

**功能**: 测试 OpenClaw 连接，发送测试消息。

**返回值**: `"Test passed!"` on success

---

### 6. test_bubble

**位置**: `src-tauri/src/lib.rs:418`

**签名**:
```rust
#[tauri::command]
async fn test_bubble(app: AppHandle) -> Result<String, String>
```

**功能**: 测试气泡事件发送，发送包含两个工具调用的测试气泡。

---

### 7. test_response_bubble

**位置**: `src-tauri/src/lib.rs:448`

**签名**:
```rust
#[tauri::command]
async fn test_response_bubble(app: AppHandle) -> Result<String, String>
```

**功能**: 测试响应气泡显示，包含 Markdown 格式的长文本。

---

### 8. test_hermes

**位置**: `src-tauri/src/lib.rs:466`

**签名**:
```rust
#[tauri::command]
async fn test_hermes() -> Result<String, String>
```

**功能**: 直接测试 Hermes HTTP API 连接，不通过 `BackendManager`。

**返回值**: `format!("Hermes status: {}, body len: {}", status, body.len())`

---

### 9. update_bubble

**位置**: `src-tauri/src/lib.rs:513`

**签名**:
```rust
#[tauri::command]
fn update_bubble(tools: Vec<ToolCall>, app: AppHandle) -> Result<(), String>
```

**参数**:
| 参数 | 类型 | 说明 |
|-----|------|------|
| `tools` | `Vec<ToolCall>` | 工具调用列表 |
| `app` | `AppHandle` | Tauri 应用句柄 |

**功能**: 手动更新工具气泡，发送 `update-bubble` 事件。

---

### 10. show_bubble_window

**位置**: `src-tauri/src/lib.rs:524`

**签名**:
```rust
#[tauri::command]
async fn show_bubble_window(main_x: i32, main_y: i32, app: AppHandle) -> Result<(), String>
```

**参数**:
| 参数 | 类型 | 说明 |
|-----|------|------|
| `main_x` | `i32` | 主窗口 X 坐标 |
| `main_y` | `i32` | 主窗口 Y 坐标 |
| `app` | `AppHandle` | Tauri 应用句柄 |

**功能**: 显示气泡窗口，位置为气泡左下角对齐猫咪右上角。

**窗口尺寸**: 380x560

**位置计算**:
```
bubble_x = main_x + 180
bubble_y = main_y - 560
```

---

### 11. hide_bubble_window

**位置**: `src-tauri/src/lib.rs:566`

**签名**:
```rust
#[tauri::command]
async fn hide_bubble_window(app: AppHandle) -> Result<(), String>
```

**功能**: 隐藏气泡窗口（不销毁，只隐藏）。

---

### 12. update_bubble_content

**位置**: `src-tauri/src/lib.rs:575`

**签名**:
```rust
#[tauri::command]
async fn update_bubble_content(text: String, app: AppHandle) -> Result<(), String>
```

**参数**:
| 参数 | 类型 | 说明 |
|-----|------|------|
| `text` | `String` | 气泡文本内容 |
| `app` | `AppHandle` | Tauri 应用句柄 |

**功能**: 更新气泡内容，发送 `bubble-update` 事件。

**注意**: 此命令发送的事件名为 `bubble-update`，与其他气泡事件 `update-bubble` 不同。

---

### 13. move_bubble_window

**位置**: `src-tauri/src/lib.rs:610`

**签名**:
```rust
#[tauri::command]
async fn move_bubble_window(main_x: i32, main_y: i32, app: AppHandle) -> Result<(), String>
```

**功能**: 移动气泡窗口跟随主窗口移动。

---

### 14. enter_response_mode

**位置**: `src-tauri/src/lib.rs:625`

**签名**:
```rust
#[tauri::command]
async fn enter_response_mode(app: AppHandle) -> Result<(), String>
```

**功能**: 进入响应模式，扩展主窗口以显示响应内容。

**窗口变化**:
```
尺寸: 360x360 → 1120x1480
保持猫咪 Y 坐标不变
```

**算法**: 先 resize（macOS 会从当前位置向下扩展），再移动窗口到目标 Y。

---

### 15. exit_response_mode

**位置**: `src-tauri/src/lib.rs:662`

**签名**:
```rust
#[tauri::command]
async fn exit_response_mode(app: AppHandle) -> Result<(), String>
```

**功能**: 退出响应模式，恢复主窗口尺寸。

**窗口变化**:
```
尺寸: 1120x1480 → 360x360
保持猫咪 Y 坐标不变
```

**算法**: 先 resize（macOS 会从当前位置向上收缩），再移动窗口到目标 Y。

---

### 16. switch_backend

**位置**: `src-tauri/src/lib.rs:697`

**签名**:
```rust
#[tauri::command]
fn switch_backend(backend: String, state: tauri::State<AppState>) -> Result<(), String>
```

**参数**:
| 参数 | 类型 | 说明 |
|-----|------|------|
| `backend` | `String` | 目标后端名称 (`openclaw` 或 `hermes`) |
| `state` | `tauri::State<AppState>` | 应用状态 |

**功能**: 切换当前使用的 AI 后端。

**前端调用** (`src/options.ts`):
```typescript
await invoke('switch_backend', { backend });
```

---

### 17. get_current_backend

**位置**: `src-tauri/src/lib.rs:703`

**签名**:
```rust
#[tauri::command]
fn get_current_backend(state: tauri::State<AppState>) -> Option<String>
```

**返回值**: 当前后端名称或 `None`

**前端调用** (`src/options.ts`):
```typescript
const backend = await invoke<string | null>('get_current_backend');
```

---

### 18. check_backend_health

**位置**: `src-tauri/src/lib.rs:774`

**签名**:
```rust
#[tauri::command]
fn check_backend_health(state: tauri::State<AppState>) -> backend::BackendHealth
```

**返回值**: `BackendHealth` 结构体（仅当前选中后端）

```rust
struct BackendHealth {
    available: bool,      // 是否可用
    backend: String,       // 后端名称
    endpoint: String,      // 端点地址
    error: Option<String>, // 错误信息
}
```

---

### 19. check_all_backends_health

**位置**: `src-tauri/src/lib.rs:760`

**签名**:
```rust
#[tauri::command]
fn check_all_backends_health(state: tauri::State<AppState>) -> Vec<backend::BackendHealth>
```

**返回值**: `Vec<BackendHealth>` — 所有已注册后端的健康状态列表

**前端调用** (`src/options.ts`):
```typescript
const healthList = await invoke<BackendHealth[]>('check_all_backends_health');
healthList.forEach(health => {
  // 更新每个后端的指示器
});
```

---

## 命令注册

所有命令在 `lib.rs:805` 的 `invoke_handler` 中注册：

```rust
.invoke_handler(tauri::generate_handler![
    switch_animation,
    update_bubble,
    send_to_openclaw,
    send_to_hermes,
    send_message,
    test_openclaw,
    test_bubble,
    test_response_bubble,
    test_hermes,
    show_bubble_window,
    hide_bubble_window,
    update_bubble_content,
    move_bubble_window,
    enter_response_mode,
    exit_response_mode,
    switch_backend,
    get_current_backend,
    check_backend_health,
    check_all_backends_health
])
```
