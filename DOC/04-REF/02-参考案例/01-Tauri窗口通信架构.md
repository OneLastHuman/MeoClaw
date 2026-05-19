# Tauri 多窗口通信架构参考

## 概述

在 Tauri 桌面应用中，多窗口通信是一个核心架构问题。每个窗口有独立 HTML，但共享 Rust 后端。这与 Web 前端 SPA 架构完全不同。

## 通信模式对比

| 模式 | 适用场景 | 特点 |
|------|----------|------|
| **Event 系统 (emit/listen)** | 通知、状态更新 | 轻量、异步、无返回值 |
| **Commands (invoke)** | 请求-响应、数据获取 | 有返回值、可返回错误 |
| **Channels** | 流式数据、进度更新 | 适合大量数据或流式场景 |
| **Rust 后端转发** | 复杂多窗口协调 | 统一总线、安全验证 |

---

## 1. Event 系统（emit / listen）

### 原理

- `emit` 发送事件 → `listen` 监听接收
- 事件负载必须是 JSON 序列化数据
- 支持全局广播或定向发送

### Rust → Frontend（全局事件）

```rust
// src-tauri/src/lib.rs
use tauri::{AppHandle, Emitter};

#[tauri::command]
fn download(app: AppHandle, url: String) {
    app.emit("download-started", &url).unwrap();
    for progress in [1, 15, 50, 80, 100] {
        app.emit("download-progress", progress).unwrap();
    }
    app.emit("download-finished", &url).unwrap();
}
```

### Rust → Frontend（定向事件）

```rust
// 发送到指定窗口
app.emit_to("window-label", "event-name", payload).unwrap();

// 条件过滤发送
use tauri::{Emitter, EventTarget};
app.emit_filter("open-file", path, |target| {
    match target {
        EventTarget::WebviewWindow { label } => label == "main" || label == "file-viewer",
        _ => false,
    }
}).unwrap();
```

### Frontend → Rust

```typescript
// 监听全局事件
import { listen } from '@tauri-apps/api/event';

const unlisten = await listen<{ url: string }>('download-started', (event) => {
    console.log('Download started:', event.payload.url);
});

// 清理监听器
unlisten();

// 只监听一次
import { once } from '@tauri-apps/api/event';
once('ready', (event) => { /* ... */ });
```

### Frontend → Frontend（窗口间通信）

```typescript
// 窗口 A 发送
import { emit } from '@tauri-apps/api/event';
await emit('global-message', { data: 'Hello from Window A!' });

// 窗口 B 接收
import { listen } from '@tauri-apps/api/event';
const unlisten = await listen('global-message', (event) => {
    console.log('Received:', event.payload.data);
});
```

---

## 2. Commands (invoke)

### 基本用法

```rust
// src-tauri/src/lib.rs
#[tauri::command]
fn get_file_content(path: String) -> Result<String, String> {
    std::fs::read_to_string(path)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn read_file_async(path: String) -> Result<Vec<u8>, String> {
    tokio::fs::read(&path).await.map_err(|e| e.to_string())
}
```

```typescript
// Frontend 调用
import { invoke } from '@tauri-apps/api/core';

const content = await invoke<string>('get_file_content', { path: '/path/to/file' });
```

### 错误处理

```rust
#[tauri::command]
fn login(user: String, password: String) -> Result<String, String> {
    if user == "admin" && password == "secret" {
        Ok("logged_in".to_string())
    } else {
        Err("invalid credentials".to_string())
    }
}
```

```typescript
try {
    const result = await invoke<string>('login', { user: 'admin', password: 'wrong' });
} catch (error) {
    console.error('Login failed:', error);
}
```

---

## 3. Channels（流式数据）

适合下载进度、子进程输出、WebSocket 消息等场景。

### Rust 端

```rust
use tauri::{AppHandle, ipc::Channel};
use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(tag = "event", content = "data")]
enum DownloadEvent {
    Started { url: String, size: usize },
    Progress { bytes: usize },
    Finished { path: String },
}

#[tauri::command]
async fn download(url: String, channel: Channel<DownloadEvent>) {
    channel.send(DownloadEvent::Started { url: url.clone(), size: 1000 }).unwrap();
    
    for i in 0..10 {
        channel.send(DownloadEvent::Progress { bytes: i * 100 }).unwrap();
    }
    
    channel.send(DownloadEvent::Finished { path: "/tmp/file".to_string() }).unwrap();
}
```

### Frontend 端

```typescript
import { invoke, Channel } from '@tauri-apps/api/core';

const channel = new Channel<DownloadEvent>();
channel.onmessage = (message) => {
    switch (message.event) {
        case 'Started':
            console.log('Download started, size:', message.data.size);
            break;
        case 'Progress':
            console.log('Progress:', message.data.bytes);
            break;
        case 'Finished':
            console.log('Download finished:', message.data.path);
            break;
    }
};

await invoke('download', { url: 'https://example.com/file', channel });
```

---

## 4. Rust 后端作为消息总线

对于复杂的多窗口协调，让 Rust 作为中央消息路由器。

### 架构

```
Window A  -->  Rust Backend  -->  Window B
              (消息总线)
                   |
              Window C
```

### 实现示例

```rust
// src-tauri/src/lib.rs
#[tauri::command]
fn route_message(from: String, to: String, event: String, payload: serde_json::Value) {
    // 消息路由逻辑
    app.emit_to(&to, &event, payload).unwrap();
}

// 窗口注册
#[tauri::command]
fn register_window(label: String, app: AppHandle) {
    app.emit_to("main", "window-registered", label).unwrap();
}
```

---

## 5. tauri-store 跨窗口状态同步

第三方插件，适合需要跨窗口共享和持久化状态的场景。

### 特点

- 将状态保存到磁盘
- 跨多窗口同步
- 支持防抖（debounce）和节流（throttle）
- 可从 JavaScript 和 Rust 访问

### 使用示例

```typescript
import { create } from 'zustand';
import { createTauriStore } from '@tauri-store/zustand';

const useStore = create((set) => ({
    theme: 'dark',
    setTheme: (theme) => set({ theme }),
}));

const tauriHandler = createTauriStore('app-store', useStore);
```

---

## 6. Window Event 监听

窗口移动、缩放等事件的监听。

### Rust 端（推荐方式）

```rust
use tauri::{Manager, WindowEvent};

tauri::Builder::default()
    .setup(|app| {
        if let Some(window) = app.get_webview_window("main") {
            window.on_window_event(|event| {
                match event {
                    WindowEvent::Moved(position) => {
                        println!("Window moved to: {:?}", position);
                    }
                    WindowEvent::Resized(size) => {
                        println!("Window resized to: {:?}", size);
                    }
                    _ => {}
                }
            });
        }
        Ok(())
    })
```

### Frontend 端

```typescript
import { getCurrentWindow } from '@tauri-apps/api/window';

const win = getCurrentWindow();
win.onMoved((position) => {
    console.log('Moved to:', position.x, position.y);
});

win.onResized((size) => {
    console.log('Resized to:', size.width, size.height);
});
```

---

## 7. 备选方案：单窗口透明层叠

**Gemini 建议**：若主窗口和气泡窗口需要实时同步运动，两窗口会导致 IPC 延迟。最佳方案是**合并为单个透明窗口**，内部用 CSS 布局两个区域。

### 优势

| 优势 | 说明 |
|------|------|
| **零延迟** | DOM 元素移动自然同步，无 IPC 开销 |
| **简化状态** | 不需要跨窗口同步状态 |
| **统一坐标** | 同一坐标系，无转换烦恼 |

### 实现

```json
// tauri.conf.json
{
  "windows": [{
    "title": "Desktop Pet",
    "width": 560,
    "height": 560,
    "decorations": false,
    "transparent": true,
    "resizable": false
  }]
}
```

```css
/* 前端布局：猫 + 气泡在同一窗口 */
body {
  background: transparent;
  overflow: hidden;
}

.pet-container {
  position: relative;
  width: 180px;
  height: 180px;
}

.bubble-container {
  position: absolute;
  left: 180px;     /* 猫右侧 */
  top: -380px;     /* 猫上方（向上展开） */
  width: 380px;
  height: 560px;
}

/* 拖拽区域：整个 body */
[data-tauri-drag-region] {
  cursor: move;
}
```

### 点击穿透

透明区域需要穿透到桌面，用 `set_ignore_cursor_events`：

```rust
// Rust: 仅在宠物实体区域启用鼠标事件
#[tauri::command]
fn set_clickable(window: tauri::Window, enabled: bool) {
    window.set_ignore_cursor_events(!enabled).unwrap();
}
```

---

## 8. 屏幕空间感知

宠物需感知屏幕边界、任务栏、多显示器。

### 获取显示器信息

```rust
#[derive(serde::Serialize)]
struct MonitorInfo {
    name: String,
    x: i32, y: i32,
    width: u32, height: u32,
    is_primary: bool,
}

#[tauri::command]
fn get_monitors(app: AppHandle) -> Vec<MonitorInfo> {
    app.available_monitors()
        .unwrap()
        .map(|m| MonitorInfo {
            name: m.name().unwrap_or_default(),
            x: m.position().x,
            y: m.position().y,
            width: m.size().width,
            height: m.size().height,
            is_primary: m.is_primary(),
        })
        .collect()
}
```

### 边界检测

```rust
fn clamp_to_screen(x: i32, y: i32, w: u32, h: u32) -> (i32, i32) {
    let monitors = app.available_monitors().unwrap();
    
    for m in monitors {
        let pos = m.position();
        let size = m.size();
        
        if x >= pos.x 
        && x + w as i32 <= pos.x + size.width as i32
        && y >= pos.y
        && y + h as i32 <= pos.y + size.height as i32 {
            return (x, y);  // 在显示器内
        }
    }
    
    // 回归主显示器
    let primary = monitors.find(|m| m.is_primary()).unwrap();
    (primary.position().x, primary.position().y)
}
```

---

## 9. Alpha Hit-Test（精细点击穿透）

仅宠物实体区域响应点击，透明区域穿透。

### 原理

根据 Canvas Alpha 通道值决定是否启用鼠标事件：

```typescript
// 前端：检测鼠标位置是否为实体
function isPetHit(canvas: HTMLCanvasElement, x: number, y: number): boolean {
    const ctx = canvas.getContext('2d')!;
    const pixel = ctx.getImageData(x, y, 1, 1).data;
    return pixel[3] > 64;  // alpha > 64 认为是非透明区域
}

// 鼠标移动时动态切换
canvas.addEventListener('mousemove', (e) => {
    const rect = canvas.getBoundingClientRect();
    const hit = isPetHit(canvas, e.clientX - rect.left, e.clientY - rect.top);
    invoke('set_clickable', { enabled: hit });
});
```

---

## 参考资料

- [Tauri 官方文档 - Inter-Process Communication](https://tauri.app/develop/inter-process-communication/)
- [掘金 - Tauri 多窗口之间通信方案](https://juejin.cn/post/7485201976050188323)
- [博客园 - tauri 中的通信](https://www.cnblogs.com/marilol/p/18530647)
- [tauri-store 插件](https://tb.dev.br/tauri-store/)
- [Tauri Window State 插件](https://tauri.app/plugin/window-state/)
