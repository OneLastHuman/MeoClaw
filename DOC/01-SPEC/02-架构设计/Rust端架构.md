# Rust 端架构

## 模块结构

```
src-tauri/src/
├── main.rs              # 程序入口，调用 lib::run()
├── lib.rs               # 主库，包含所有 Tauri 命令和初始化逻辑
├── backend/
│   ├── mod.rs           # 后端模块导出
│   ├── manager.rs      # BackendManager 多后端管理
│   ├── client_trait.rs # BackendClient trait 定义
│   ├── openclaw_client.rs  # OpenClaw 适配器
│   ├── hermes_client.rs    # Hermes HTTP 客户端
│   └── hermes_discovery.rs # Hermes 服务发现
└── openclaw/
    ├── mod.rs           # OpenClaw WebSocket 客户端
    ├── client.rs        # 客户端实现
    ├── protocol.rs      # 协议处理
    └── auth.rs          # 认证逻辑
```

## 核心组件

### lib.rs - 主入口

**职责**:
- Tauri 应用初始化
- 命令处理 (`switch_animation`, `send_message`, `update_bubble` 等)
- 窗口管理 (main, bubble, options)
- 系统托盘设置
- 文件监控 (状态文件、气泡内容)
- OpenClaw WebSocket 生命周期管理

**关键命令**:

| 命令 | 用途 |
|------|------|
| `switch_animation` | 切换动画状态 |
| `send_message` | 统一消息路由 (自动选择后端) |
| `send_to_openclaw` | 发送消息到 OpenClaw |
| `send_to_hermes` | 发送消息到 Hermes |
| `update_bubble` | 更新工具调用气泡 |
| `show_bubble_window` / `hide_bubble_window` | 控制气泡窗口 |
| `enter_response_mode` / `exit_response_mode` | Response 状态窗口调整 |
| `switch_backend` | 切换 AI 后端 |
| `check_backend_health` | 健康检查（当前选中后端） |
| `check_all_backends_health` | 健康检查（所有已注册后端） |

### backend/manager.rs - 后端管理器

**职责**:
- 统一管理多个后端客户端
- 后端切换
- 配置持久化

**核心结构**:

```rust
pub struct BackendManager {
    config: Arc<Mutex<BackendConfig>>,      // 配置
    clients: Arc<Mutex<HashMap<String, Box<dyn BackendClient>>>>,  // 客户端实例
}
```

**配置结构**:

```rust
pub struct BackendConfig {
    pub selected: ActiveBackend,             // 当前选中后端
    pub openclaw: OpenClawConfig,            // OpenClaw 配置
    pub hermes: LegacyHermesConfig,          // Hermes 配置
    pub discovered_instances: Vec<HermesConfig>,  // 发现的 Hermes 实例
}
```

### backend/client_trait.rs - 后端客户端 trait

定义 `BackendClient` 接口，所有后端实现需实现此 trait：

```rust
pub trait BackendClient {
    fn name(&self) -> &str;
    fn send_message(&self, message: &str) -> Result<Box<dyn Stream>, String>;
    fn check_health(&self) -> bool;
}
```

### backend/openclaw_client.rs - OpenClaw 适配器

将 OpenClaw WebSocket 客户端适配为 `BackendClient` trait。

### backend/hermes_client.rs - Hermes 客户端

HTTP/SSE 方式的 Hermes 后端实现。

### backend/hermes_discovery.rs - Hermes 服务发现

4 层发现机制:
1. 环境变量
2. 配置文件
3. mDNS 发现
4. 默认地址

### openclaw/ - OpenClaw WebSocket 客户端

**职责**:
- WebSocket 连接管理
- 会话管理
- 工具调用回调
- 生命周期事件回调
- HTTP 轮询降级

**关键结构**:

```rust
pub struct OpenClawClient {
    // WebSocket 连接
    // 回调处理器
    // 状态管理
}
```

## 状态管理

### AppState

```rust
pub struct AppState {
    pub backend_manager: Arc<Mutex<BackendManager>>,
}
```

通过 `tauri::State<AppState>` 在命令间共享。

### 文件状态

| 文件 | 用途 |
|------|------|
| `/tmp/meo_anim_state.txt` | 当前动画状态 |
| `/tmp/bubble_content.txt` | 气泡显示内容 |

## 依赖

核心依赖 (Cargo.toml):

| 依赖 | 版本 | 用途 |
|------|------|------|
| tauri | 2 | 框架 |
| tauri-plugin-log | 2 | 日志 |
| serde/serde_json | 1 | 序列化 |
| notify | 7 | 文件监控 |
| tokio | 1 | 异步运行时 |
| reqwest | 0.12 | HTTP 客户端 |
| tokio-tungstenite | 0.26 | WebSocket |
