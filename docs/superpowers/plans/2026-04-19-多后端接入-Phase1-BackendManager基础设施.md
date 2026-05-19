# 多后端接入 - Phase 1: BackendManager 基础设施

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 建立后端抽象框架，包括 BackendClient trait、BackendManager 核心结构、配置持久化

**Architecture:** 采用 trait 对象模式，通过 BackendManager 统一管理不同后端客户端，支持运行时切换

**Tech Stack:** Rust (Tauri v2), serde_json, tauri-plugin-store

---

## 文件结构

```
src-tauri/src/
├── backend/
│   ├── mod.rs              # 导出 BackendManager, trait
│   ├── trait.rs           # BackendClient trait, ToolEvent, StreamResponse, ToolPhase
│   ├── manager.rs         # BackendManager, BackendConfig, BackendHealth, ActiveBackend
│   ├── openclaw_client.rs # 占位结构，后续 Phase 2 实现
│   └── hermes_client.rs   # 占位结构，后续 Phase 3 实现
```

---

## Task 1: 创建 BackendClient trait

**Files:**
- Create: `src-tauri/src/backend/trait.rs`

- [ ] **Step 1: 创建 trait.rs 文件**

```rust
use std::collections::HashMap;
use std::error::Error;
use std::io;
use serde::{Deserialize, Serialize};

/// 工具事件的执行阶段
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ToolPhase {
    Started,
    Progress,
    Result,
    Error,
}

/// 工具事件结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolEvent {
    pub name: String,
    pub phase: ToolPhase,
    pub args: Option<serde_json::Value>,
    pub preview: Option<String>,
}

/// 流式响应结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamResponse {
    pub text_delta: String,
    pub tool_event: Option<ToolEvent>,
    pub is_final: bool,
}

/// Stream trait 用于返回流式数据
pub trait Stream: Send + io::Read {
    fn poll_next(&mut self) -> Option<Result<StreamResponse, Box<dyn Error + Send + Sync>>>;
}

/// BackendClient trait - 所有后端客户端必须实现此接口
pub trait BackendClient: Send + Sync {
    /// 返回后端名称
    fn name(&self) -> &str;

    /// 健康检测
    fn check_health(&self) -> bool;

    /// 发送消息，返回流式响应
    fn send_message(&self, text: &str) -> Result<Box<dyn Stream>, Box<dyn Error + Send + Sync>>;

    /// 获取端点地址
    fn endpoint(&self) -> &str;

    /// 获取认证头
    fn auth_headers(&self) -> HashMap<String, String>;
}
```

- [ ] **Step 2: 验证编译**

Run: `cd /Users/hue/Documents/pet/MeoClaw && cargo check --package meo-claw 2>&1`
Expected: 编译成功，无错误

- [ ] **Step 3: 提交**

```bash
git add src-tauri/src/backend/trait.rs
git commit -m "feat(multi-backend): add BackendClient trait and types"
```

---

## Task 2: 创建 BackendManager 核心结构

**Files:**
- Create: `src-tauri/src/backend/manager.rs`
- Modify: `src-tauri/src/backend/mod.rs`

- [ ] **Step 1: 创建 manager.rs**

```rust
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};

use super::trait::{BackendClient, BackendHealth};

/// 活跃后端枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ActiveBackend {
    OpenClaw,
    Hermes,
}

impl ActiveBackend {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "openclaw" => Some(ActiveBackend::OpenClaw),
            "hermes" => Some(ActiveBackend::Hermes),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            ActiveBackend::OpenClaw => "openclaw",
            ActiveBackend::Hermes => "hermes",
        }
    }
}

/// OpenClaw 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenClawConfig {
    pub endpoint: String,
    pub health_endpoint: String,
}

/// Hermes 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HermesConfig {
    pub endpoint: String,
    pub health_endpoint: String,
}

/// 后端配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendConfig {
    pub selected: ActiveBackend,
    pub openclaw: OpenClawConfig,
    pub hermes: HermesConfig,
}

impl Default for BackendConfig {
    fn default() -> Self {
        Self {
            selected: ActiveBackend::OpenClaw,
            openclaw: OpenClawConfig {
                endpoint: "ws://127.0.0.1:18789".to_string(),
                health_endpoint: "http://127.0.0.1:18789".to_string(),
            },
            hermes: HermesConfig {
                endpoint: "http://127.0.0.1:8642".to_string(),
                health_endpoint: "http://127.0.0.1:8642/health".to_string(),
            },
        }
    }
}

/// BackendManager - 统一管理所有后端客户端
pub struct BackendManager {
    config: Arc<Mutex<BackendConfig>>,
    clients: HashMap<String, Box<dyn BackendClient>>,
}

impl BackendManager {
    pub fn new() -> Self {
        Self {
            config: Arc::new(Mutex::new(BackendConfig::default())),
            clients: HashMap::new(),
        }
    }

    /// 注册后端客户端
    pub fn register_client(&mut self, client: Box<dyn BackendClient>) {
        let name = client.name().to_string();
        self.clients.insert(name, client);
    }

    /// 切换活跃后端
    pub fn switch(&self, backend: &str) -> Result<(), String> {
        let active = ActiveBackend::from_str(backend)
            .ok_or_else(|| format!("Unknown backend: {}", backend))?;

        let mut config = self.config.lock().map_err(|e| e.to_string())?;
        config.selected = active;
        Ok(())
    }

    /// 获取当前活跃后端
    pub fn current(&self) -> Option<String> {
        self.config.lock().ok()
            .map(|c| c.selected.as_str().to_string())
    }

    /// 获取当前后端配置
    pub fn config(&self) -> Arc<Mutex<BackendConfig>> {
        Arc::clone(&self.config)
    }

    /// 检测当前后端健康状态
    pub fn check_health(&self) -> BackendHealth {
        let config = match self.config.lock() {
            Ok(c) => c,
            Err(e) => return BackendHealth {
                available: false,
                backend: "unknown".to_string(),
                endpoint: "".to_string(),
                error: Some(e.to_string()),
            },
        };

        let client_name = config.selected.as_str();
        let endpoint = match config.selected {
            ActiveBackend::OpenClaw => &config.openclaw.endpoint,
            ActiveBackend::Hermes => &config.hermes.endpoint,
        };

        drop(config);

        if let Some(client) = self.clients.get(client_name) {
            let available = client.check_health();
            BackendHealth {
                available,
                backend: client_name.to_string(),
                endpoint: endpoint.clone(),
                error: if available { None } else { Some("Health check failed".to_string()) },
            }
        } else {
            BackendHealth {
                available: false,
                backend: client_name.to_string(),
                endpoint: endpoint.clone(),
                error: Some(format!("Client '{}' not registered", client_name)),
            }
        }
    }
}

impl Default for BackendManager {
    fn default() -> Self {
        Self::new()
    }
}
```

- [ ] **Step 2: 创建 mod.rs**

```rust
pub mod trait;
pub mod manager;
pub mod openclaw_client;
pub mod hermes_client;

pub use trait::{BackendClient, StreamResponse, StreamResponse, ToolEvent, ToolPhase};
pub use manager::{ActiveBackend, BackendConfig, BackendManager, HermesConfig, OpenClawConfig};
```

- [ ] **Step 3: 验证编译**

Run: `cargo check --package meo-claw 2>&1`
Expected: 编译成功，当前仅有 unused warnings（openclaw_client, hermes_client 占位）

- [ ] **Step 4: 提交**

```bash
git add src-tauri/src/backend/
git commit -m "feat(multi-backend): add BackendManager core structure"
```

---

## Task 3: 创建占位客户端结构

**Files:**
- Create: `src-tauri/src/backend/openclaw_client.rs`
- Create: `src-tauri/src/backend/hermes_client.rs`

- [ ] **Step 1: 创建 openclaw_client.rs 占位结构**

```rust
use std::collections::HashMap;
use std::error::Error;
use crate::backend::trait::{BackendClient, Stream};

/// OpenClaw 客户端占位结构
/// Phase 2 将实现完整的 WebSocket 功能
pub struct OpenClawClient;

impl OpenClawClient {
    pub fn new() -> Self {
        Self
    }
}

impl BackendClient for OpenClawClient {
    fn name(&self) -> &str {
        "openclaw"
    }

    fn check_health(&self) -> bool {
        false // Phase 2 实现
    }

    fn send_message(&self, _text: &str) -> Result<Box<dyn Stream>, Box<dyn Error + Send + Sync>> {
        Err("Not implemented".into())
    }

    fn endpoint(&self) -> &str {
        "ws://127.0.0.1:18789"
    }

    fn auth_headers(&self) -> HashMap<String, String> {
        HashMap::new()
    }
}

impl Default for OpenClawClient {
    fn default() -> Self {
        Self::new()
    }
}
```

- [ ] **Step 2: 创建 hermes_client.rs 占位结构**

```rust
use std::collections::HashMap;
use std::error::Error;
use crate::backend::trait::{BackendClient, Stream};

/// Hermes 客户端占位结构
/// Phase 3 将实现完整的 HTTP/SSE 功能
pub struct HermesClient;

impl HermesClient {
    pub fn new() -> Self {
        Self
    }
}

impl BackendClient for HermesClient {
    fn name(&self) -> &str {
        "hermes"
    }

    fn check_health(&self) -> bool {
        false // Phase 3 实现
    }

    fn send_message(&self, _text: &str) -> Result<Box<dyn Stream>, Box<dyn Error + Send + Sync>> {
        Err("Not implemented".into())
    }

    fn endpoint(&self) -> &str {
        "http://127.0.0.1:8642"
    }

    fn auth_headers(&self) -> HashMap<String, String> {
        HashMap::new()
    }
}

impl Default for HermesClient {
    fn default() -> Self {
        Self::new()
    }
}
```

- [ ] **Step 3: 验证编译**

Run: `cargo check --package meo-claw 2>&1`
Expected: 编译成功，无错误

- [ ] **Step 4: 提交**

```bash
git add src-tauri/src/backend/openclaw_client.rs src-tauri/src/backend/hermes_client.rs
git commit -m "feat(multi-backend): add placeholder client structures"
```

---

## Task 4: 集成 BackendManager 到 Tauri 命令

**Files:**
- Modify: `src-tauri/src/lib.rs`
- Modify: `src-tauri/src/commands.rs` (如存在)

- [ ] **Step 1: 读取 lib.rs 了解当前结构**

Run: `head -100 src-tauri/src/lib.rs`

- [ ] **Step 2: 在 lib.rs 中添加 BackendManager 集成**

在 lib.rs 顶部添加:
```rust
mod backend;
use backend::BackendManager;
```

在 lib.rs 中添加 Tauri state:
```rust
#[tauri::state]
struct AppState {
    backend_manager: BackendManager,
}
```

在 main.rs 或 lib.rs 的 tauri::build! 之后添加命令处理（如果 commands 模块不存在则创建）:
```rust
#[tauri::command]
fn switch_backend(backend: String, state: tauri::State<AppState>) -> Result<(), String> {
    state.backend_manager.switch(&backend)
}

#[tauri::command]
fn get_current_backend(state: tauri::State<AppState>) -> Option<String> {
    state.backend_manager.current()
}

#[tauri::command]
fn check_backend_health(state: tauri::State<AppState>) -> backend::BackendHealth {
    state.backend_manager.check_health()
}
```

- [ ] **Step 3: 验证编译**

Run: `cargo check --package meo-claw 2>&1`
Expected: 编译成功

- [ ] **Step 4: 提交**

```bash
git add src-tauri/src/lib.rs
git commit -m "feat(multi-backend): integrate BackendManager with Tauri commands"
```

---

## Task 5: 添加配置持久化

**Files:**
- Modify: `src-tauri/src/backend/manager.rs` (添加 save/load 方法)
- Modify: `src-tauri/src/lib.rs` (集成 tauri-plugin-store)

- [ ] **Step 1: 添加持久化方法到 BackendConfig**

在 `BackendConfig` 实现块中添加:

```rust
impl BackendConfig {
    /// 从文件加载配置
    pub fn load(path: &std::path::Path) -> Result<Self, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| e.to_string())?;
        serde_json::from_str(&content)
            .map_err(|e| e.to_string())
    }

    /// 保存配置到文件
    pub fn save(&self, path: &std::path::Path) -> Result<(), String> {
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| e.to_string())?;
        std::fs::write(path, content)
            .map_err(|e| e.to_string())
    }
}
```

- [ ] **Step 2: 验证编译**

Run: `cargo check --package meo-claw 2>&1`
Expected: 编译成功

- [ ] **Step 3: 提交**

```bash
git add src-tauri/src/backend/manager.rs
git commit -m "feat(multi-backend): add BackendConfig persistence"
```

---

## Self-Review 检查清单

- [ ] BackendClient trait 定义完整（name, check_health, send_message, endpoint, auth_headers）
- [ ] ToolEvent, ToolPhase, StreamResponse 类型定义完整
- [ ] BackendManager 支持 register_client, switch, current, check_health
- [ ] BackendConfig 支持 save/load 持久化
- [ ] OpenClawClient 和 HermesClient 占位结构已创建
- [ ] Tauri 命令已注册 switch_backend, get_current_backend, check_backend_health
- [ ] 所有任务已提交

---

**Phase 1 完成。下一步：Phase 2 - OpenClaw 适配器实现**

---

**Execution Options:**

**1. Subagent-Driven (recommended)** - I dispatch a fresh subagent per task, review between tasks, fast iteration

**2. Inline Execution** - Execute tasks in this session using executing-plans, batch execution with checkpoints

Which approach?
