# Rust 与 Frontend 通信协议设计

## 概述

在 Tauri 应用中，Rust 后端和 Web 前端之间的通信是核心架构决策。本文档参考业界最佳实践，提供通信协议设计指南。

## 通信模式总览

| 模式 | Rust → JS | JS → Rust | 典型场景 |
|------|-----------|-----------|----------|
| **Commands** | ✗ | ✓ (响应) | 查询数据、执行操作 |
| **Events** | ✓ | ✗ | 推送通知、状态变更 |
| **Channels** | ✓ (流式) | ✗ | 进度更新、文件下载 |

---

## Commands (invoke) 设计

### 设计原则

1. **命令命名**: `verb_noun` 格式
   - `get_window_position`
   - `switch_animation`
   - `update_bubble_content`

2. **参数序列化**: 使用 camelCase
   ```rust
   #[derive(serde::Serialize)]
   struct WindowPosition {
       x: i32,
       y: i32,
   }
   ```

3. **错误处理**: 返回 `Result<T, String>`
   ```rust
   #[tauri::command]
   fn risky_operation() -> Result<String, String> {
       // 成功返回 Ok(value)
       // 失败返回 Err(error_message)
   }
   ```

### 命令分组

```rust
// src-tauri/src/commands/
mod animation {
    #[tauri::command]
    pub fn switch(state: &str, app: AppHandle) -> Result<(), String> { }
    
    #[tauri::command]
    pub fn get_current_state() -> String { }
}

mod window {
    #[tauri::command]
    pub fn move_bubble(x: i32, y: i32, app: AppHandle) -> Result<(), String> { }
}

// lib.rs
invoke_handler(tauri::generate_handler![
    animation::switch,
    animation::get_current_state,
    window::move_bubble,
])
```

### 事件命名规范

| 事件名 | 方向 | 说明 |
|--------|------|------|
| `animation:switch` | Rust → JS | 动画切换通知 |
| `animation:frame` | Rust → JS | 帧更新（可选） |
| `bubble:update` | Rust → JS | 气泡内容更新 |
| `window:moved` | Rust → JS | 窗口位置变更 |

---

## Events 设计

### Payload 结构

```rust
// 结构化事件 payload
#[derive(Clone, serde::Serialize)]
#[serde(tag = "type", content = "data")]
enum AnimationEvent {
    Switched { from: String, to: String },
    Completed { state: String },
}

#[derive(Clone, serde::Serialize)]
struct BubbleUpdate {
    texts: Vec<String>,
    interval: Option<u64>,
}
```

### 前端监听

```typescript
import { listen } from '@tauri-apps/api/event';

// 类型化监听
type AnimationEvent = 
    | { type: 'Switched'; data: { from: string; to: string } }
    | { type: 'Completed'; data: { state: string } };

listen<AnimationEvent>('animation:event', (event) => {
    switch (event.payload.type) {
        case 'Switched':
            console.log(`Animation: ${event.payload.data.from} → ${event.payload.data.to}`);
            break;
        case 'Completed':
            console.log(`Animation completed: ${event.payload.data.state}`);
            break;
    }
});
```

---

## 状态同步策略

### 方案一：事件驱动同步

```
状态变更 → Rust emit → Frontend listen → 更新本地状态
```

```rust
fn set_animation_state(state: &str, app: &AppHandle) -> Result<(), String> {
    // 更新 Rust 状态
    update_internal_state(state)?;
    
    // 发送事件通知
    app.emit("animation:switch", state)
        .map_err(|e| e.to_string())?;
    
    Ok(())
}
```

### 方案二：前端轮询 + Rust 查询

```typescript
// 前端定期查询状态
setInterval(async () => {
    const state = await invoke<string>('get_animation_state');
    if (state !== currentState) {
        currentState = state;
        updateUI(state);
    }
}, 500);
```

### 方案三：tauri-store 跨窗口同步

```typescript
import { createTauriStore } from '@tauri-store/zustand';

// 创建共享 store
const store = createTauriStore('app-state', useAppStore);

// 启动同步
await store.start();
```

---

## 类型安全

### Rust 端定义枚举

```rust
#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AnimState {
    Idle,
    Shock,
    EnterInput,
    StartWorking,
    Working,
    EnterReceiving,
    Receiving,
    Received,
    Response,
}

impl AnimState {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "idle" => Some(AnimState::Idle),
            "shock" => Some(AnimState::Shock),
            // ...
            _ => None,
        }
    }
}
```

### TypeScript 端定义

```typescript
export type AnimState = 
    | 'idle' 
    | 'shock' 
    | 'EnterInput' 
    | 'startworking' 
    | 'working' 
    | 'EnterReceiving' 
    | 'Receiving' 
    | 'received' 
    | 'Response';

// 类型守卫
function isAnimState(value: string): value is AnimState {
    return ['idle', 'shock', 'EnterInput', 'startworking', 'working', 
            'EnterReceiving', 'Receiving', 'received', 'Response'].includes(value);
}

// 使用
if (isAnimState(event.payload)) {
    switchAnim(event.payload);
}
```

---

## 统一 API 层

### 前端 API 封装

```typescript
// api/animations.ts
export const animationApi = {
    async switch(state: AnimState): Promise<void> {
        await invoke('switch_animation', { state });
    },
    
    async getState(): Promise<AnimState> {
        return invoke<AnimState>('get_animation_state');
    },
};

export const windowApi = {
    async showBubble(x: number, y: number): Promise<void> {
        await invoke('show_bubble_window', { mainX: x, mainY: y });
    },
    
    async hideBubble(): Promise<void> {
        await invoke('hide_bubble_window');
    },
    
    async moveBubble(mainX: number, mainY: number): Promise<void> {
        await invoke('move_bubble_window', { mainX, mainY });
    },
};
```

### 使用示例

```typescript
// 在组件中使用
import { animationApi, windowApi } from '@/api';

async function onDragEnd(position: { x: number; y: number }) {
    // 更新气泡位置
    await windowApi.moveBubble(position.x, position.y);
}

async function onDoubleClick() {
    // 切换动画
    await animationApi.switch('EnterInput');
}
```

---

## 错误处理模式

### Rust 端

```rust
#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error("Invalid animation state: {0}")]
    InvalidState(String),
    
    #[error("Window not found: {0}")]
    WindowNotFound(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

// 实现 Serialize 以返回前端
impl serde::Serialize for CommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[tauri::command]
fn switch_animation(state: &str, app: AppHandle) -> Result<(), CommandError> {
    let anim_state = AnimState::from_str(state)
        .ok_or_else(|| CommandError::InvalidState(state.to_string()))?;
    
    // 执行切换
    app.emit("switch-animation", state)
        .map_err(|e| CommandError::Io(std::io::Error::new(ErrorKind::Other, e.to_string())))?;
    
    Ok(())
}
```

### 前端端

```typescript
type CommandError = {
    message: string;
    code?: string;
};

async function safeSwitch(state: AnimState) {
    try {
        await animationApi.switch(state);
    } catch (error) {
        if (isCommandError(error)) {
            console.error('Animation switch failed:', error.message);
            // 显示错误给用户
        }
    }
}
```

---

## 协议文档示例

### 命令列表

| 命令 | 参数 | 返回值 | 说明 |
|------|------|--------|------|
| `switch_animation` | `state: string` | `void` | 切换动画状态 |
| `get_animation_state` | - | `string` | 获取当前状态 |
| `show_bubble_window` | `mainX: i32, mainY: i32` | `void` | 显示气泡窗口 |
| `hide_bubble_window` | - | `void` | 隐藏气泡窗口 |
| `update_bubble_content` | `text: string` | `void` | 更新气泡内容 |

### 事件列表

| 事件 | Payload | 说明 |
|------|---------|------|
| `switch-animation` | `string` | 动画状态切换通知 |
| `update-bubble` | `{ texts: string[], interval?: number }` | 气泡内容更新 |
| `bubble-update` | `{ text: string }` | 气泡文本更新 |
| `window:moved` | `{ x: number, y: number }` | 主窗口移动 |

## 8. TS-RS 类型自动生成

Rust 和 TypeScript 类型手动同步容易出错。用 TS-RS 自动生成。

### 安装

```bash
# Cargo.toml
ts-rs = "7"
```

### Rust 端定义

```rust
use ts_rs::TS;

#[derive(Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum AnimState {
    Idle,
    Shock,
    Working,
    #[serde(rename = "enterInput")]
    EnterInput,
}

#[derive(Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct AnimationConfig {
    pub name: String,
    pub total_frames: u32,
    pub interval: u32,
}
```

### 生成 TypeScript

```bash
cargo build
# 自动在 src/ 目录生成 .ts 文件
```

### 生成结果

```typescript
// generated/anim_state.ts
export enum AnimState {
    Idle = 'Idle',
    Shock = 'Shock',
    Working = 'Working',
    EnterInput = 'enterInput',
}

export interface AnimationConfig {
    name: string;
    total_frames: number;
    interval: number;
}
```

### 使用

```typescript
import { AnimState, AnimationConfig } from '../generated/anim_state';

const config: AnimationConfig = {
    name: 'idle',
    total_frames: 16,
    interval: 100
};

await invoke('switch_animation', { state: AnimState.Idle });
```

---

## 参考资料

- [Tauri Commands 文档](https://tauri.app/develop/calling-rust/)
- [Tauri Events 文档](https://tauri.app/develop/events/)
- [Tauri Channels](https://tauri.app/develop/channels/)
- [TS-RS GitHub](https://github.com/Aleph-Alpha/ts-rs)
- [REST API 设计最佳实践](https://restfulapi.net/)
