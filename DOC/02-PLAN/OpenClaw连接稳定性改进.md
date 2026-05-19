# OpenClaw 连接稳定性改进

## 功能概述

改进桌宠与 OpenClaw Gateway 的连接稳定性，实现：
- WebSocket 断开时自动重连（指数退避策略）
- WebSocket 断开期间自动切换到 HTTP 轮询作为备胎
- WebSocket 重连成功后自动切回并停止 HTTP 轮询
- 前端可感知 Gateway 连接状态变化

---

## 架构流程

```
正常情况：桌宠 ←──WebSocket──→ Gateway（快）
    ↓
WebSocket 断开（网络波动/进程重启）
    ↓
自动重连（指数退避：1s → 2s → 4s → ... 最大30s）
    ↓
重连期间：桌宠 ──HTTP轮询──→ Gateway（慢但稳定）
    ↓
重连成功：桌宠 ←──WebSocket──→ Gateway（自动切回）
```

---

## 关键文件

| 文件 | 作用 |
|------|------|
| `src-tauri/src/openclaw/client.rs` | WebSocket 客户端，含自动重连逻辑 |
| `src-tauri/src/lib.rs` | HTTP 轮询协调，连接状态事件 |

---

## 重连配置

```rust
const RECONNECT_INITIAL_DELAY_MS: u64 = 1000;  // 初始 1 秒
const RECONNECT_MAX_DELAY_MS: u64 = 30000;     // 最大 30 秒
const RECONNECT_MAX_RETRIES: u32 = 10;        // 最多重试 10 次
```

---

## 前端事件

| 事件名 | 触发时机 |
|--------|---------|
| `gateway-connected` | WebSocket 连接成功 |
| `gateway-disconnected` | WebSocket 连接断开 |
| `gateway-reconnected` | HTTP 轮询检测到 Gateway 恢复 |

---

## 验证方法

1. 启动桌宠，确认连接 Gateway 成功
2. `pkill -f meo-claw` 杀掉桌宠进程
3. 观察 Gateway Web UI 是否仍显示 live（Gateway 不受影响）
4. 重启桌宠，确认自动重连成功

---

## 相关文档

- `OpenClaw状态同步.md` - OpenClaw 连接和状态同步基础
- `OpenClaw 工具气泡集成.md` - 工具气泡显示
