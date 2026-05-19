# OpenClaw 状态检测集成

## 背景

桌宠需要感知 OpenClaw Gateway 的运行状态：
- Gateway 运行中 → 显示 "working" 工作动画
- Gateway 关闭 → 显示 "idle" 空闲动画

## 技术方案

### 曾尝试的方案：WebSocket

最初参考 KKClaw 的 WebSocket 认证流程实现了客户端，但遇到 `DEVICE_AUTH_SIGNATURE_INVALID` 错误。经过深入调试（包括用 Python 验证签名算法）确认签名正确，问题可能出在 Gateway 的认证实现上。

### 最终方案：HTTP 轮询

改用 **HTTP 轮询** 方式，参考 KKClaw 的 `checkConnection()` 实现。KKClaw 本身并不依赖 WebSocket 接收 lifecycle 事件，而是通过定时轮询检测 Gateway 是否存活。

### 原理

```
┌─────────────────┐     每5秒 HTTP GET      ┌──────────────────┐
│   MeoClaw   │ ──────────────────────→ │  OpenClaw Gateway │
│   (Rust 端)      │  http://127.0.0.1:18789 │   (HTTP 服务)     │
└─────────────────┘ ←────────────────────── └──────────────────┘
         │
         │ 可达 → 写 "working" 到 /tmp/meo_anim_state.txt
         │ 不可达 → 写 "idle" 到 /tmp/meo_anim_state.txt
         ↓
┌──────────────────────────────────────────────────────────────┐
│                    文件监控 & 动画切换                         │
│   notify 监控文件变化 → switch_animation → 前端切换动画         │
└──────────────────────────────────────────────────────────────┘
```

## 实现细节

### Cargo.toml 依赖

```toml
# === OpenClaw HTTP 客户端新增依赖 ===
reqwest = { version = "0.12", features = ["json"] }
tokio = { version = "1", features = ["full"] }
sha2 = "0.10"
hex = "0.4"
```

### 核心代码 (lib.rs)

#### HTTP 连接检测

```rust
const GATEWAY_URL: &str = "http://127.0.0.1:18789";
const POLL_INTERVAL_SECS: u64 = 5;

async fn check_openclaw_connection() -> bool {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(2))
        .build()
        .ok()?;

    client.get(GATEWAY_URL)
        .send()
        .await
        .map(|resp| resp.status().is_success())
        .unwrap_or(false)
}
```

#### 状态写入文件

```rust
const STATE_FILE: &str = "/tmp/meo_anim_state.txt";

fn write_state_to_file(state: &str) {
    if let Err(e) = fs::write(STATE_FILE, format!("{}\n", state)) {
        log::error!("[OpenClaw] Failed to write state: {}", e);
    }
}
```

#### 轮询循环

```rust
async_runtime::spawn(async move {
    let mut was_connected = false;
    loop {
        let is_connected = check_openclaw_connection().await;

        if is_connected {
            if !was_connected {
                write_state_to_file("working");
                was_connected = true;
                log::info!("[OpenClaw] Gateway connected, switched to working");
            }
            write_state_to_file("working");  // 持续写入保持活跃
        } else {
            if was_connected {
                write_state_to_file("idle");
                was_connected = false;
                log::info!("[OpenClaw] Gateway disconnected, switched to idle");
            }
        }

        tokio::time::sleep(Duration::from_secs(POLL_INTERVAL_SECS)).await;
    }
});
```

#### 文件监控（复用已有机制）

```rust
// 监听文件变化，触发动画切换
if event.kind.is_modify() {
    if let Ok(content) = fs::read_to_string(STATE_FILE) {
        let state = content.trim();
        switch_animation(state, app_handle.clone())?;
    }
}
```

## 关键设计决策

### 1. 为什么复用文件监控而不是直接 emit？

MeoClaw 已有完整的文件监控体系（用于接收外部状态切换命令）。将 OpenClaw 状态写入文件，可以统一通过现有的监控→切换流程处理，减少代码复杂度。

### 2. 为什么持续写入 "working"？

首次连接成功写入 "working" 后，每 5 秒持续写入。这样即使前端错过了一次变化，后续轮询也会补上，保证状态同步。

### 3. 为什么用 HTTP 而不是 WebSocket？

KKClaw 本身就是用 HTTP 轮询检测 Gateway 存活。WebSocket 需要复杂的认证流程（Ed25519 challenge-response），而 HTTP GET 只需判断服务是否响应即可。

## 配置

| 配置项 | 值 | 说明 |
|--------|-----|------|
| `GATEWAY_URL` | `http://127.0.0.1:18789` | OpenClaw Gateway 地址 |
| `POLL_INTERVAL_SECS` | `5` | 轮询间隔（秒） |
| `STATE_FILE` | `/tmp/meo_anim_state.txt` | 状态文件路径 |

## 测试方法

### 1. 启动应用

```bash
npm run tauri dev
```

### 2. 观察日志

```bash
# 实时查看日志
tail -f /tmp/meow_debug.log
```

### 3. 测试状态切换

```bash
# OpenClaw 运行时 → 应显示 working
# 关闭 OpenClaw → 应显示 idle
```

### 4. 验证日志输出

```
[OpenClaw] Wrote state: idle           # 启动时
[OpenClaw] Gateway connected, switched to working  # 检测到
[Rust] switch_animation: working      # 动画切换
```

## 相关文档

- [KKClaw 源码参考](https://github.com/aieos-xxx/kkclaw) - `openclaw-client.js` 的 `checkConnection()` 方法
- [测试流程-文件监控方式](./测试流程-文件监控方式.md) - 文件监控机制的详细说明
