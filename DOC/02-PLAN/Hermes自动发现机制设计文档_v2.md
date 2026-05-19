# Hermes 自动发现机制设计方案 v2

> 创建时间：2026/04/23
> 状态：✅ 已实现（Layer 1-3）
> 替代：~ `DOC/功能/Hermes自动发现机制设计文档.md`（v1）

## 实现状态

| Layer | 功能 | 状态 | 说明 |
|-------|------|------|------|
| Layer 1 | 配置文件发现 | ✅ 已实现 | 读取 `~/.hermes/.env` |
| Layer 2 | 进程扫描 | ✅ 已实现 | 跨平台 pgrep/tasklist |
| Layer 3 | 端口探测 | ✅ 已实现 | 8642/9119 端口，2s 超时 |
| Layer 4 | mDNS 局域网发现 | ⏳ 预留接口 | stub 返回空列表，后续通过 feature flag 启用 |

**已知问题**：SSE 解析存在 bug，发送消息后会卡在 "working" 状态，不会收到回复。curl 测试确认 Hermes API 本身工作正常。

---

## 一、背景与目标

**背景**：MeoClaw 是一个跨平台桌面应用，需要连接 Hermes 后端处理消息。当前只支持连接本机 Hermes，换设备或连其他机器无法发现。

**目标**：让 MeoClaw 打开后，**零配置**自动找到局域网内正在运行的 Hermes 后端。

---

## 二、现有架构问题（v1）

| 问题 | 说明 | 严重程度 |
|------|------|---------|
| `HermesConfig` 结构体重复定义 | `hermes_discovery.rs` 和 `manager.rs` 各有一份，字段完全不同 | 🔴 编译错误 |
| `HermesClient` 硬编码 localhost | Layer 1-4 发现的结果被忽略，实际发请求到 `127.0.0.1:8642` | 🔴 功能不工作 |
| Layer 2 进程扫描仅 macOS/Linux | `pgrep` 命令在 Windows 上不存在 | 🔴 跨平台失效 |
| Layer 4 用 ARP 缓存，macOS 需 sudo | 普通用户无法读完整 ARP 表，触发权限错误 | 🔴 macOS 失败 |
| 多实例存储架构未定义 | `BackendConfig.hermes` 是单个对象，不是数组 | 🟡 架构缺失 |
| 远程 Hermes 无 API Key 获取方式 | Layer 4 发现的远程实例无法读对方 `.env` | 🟡 功能缺失 |
| 缺少超时设计 | 端口探测无超时，失败时整体卡住 | 🟡 性能问题 |
| `/health` 认证问题未考虑 | 认证中间件可能让健康检查返回 401 | 🟡 误判风险 |

---

## 三、升级方案：4 层自动发现 + mDNS 广播

### 3.1 核心变更：Layer 4 改用 mDNS/Bonjour

| 方案 | 安全性 | 跨平台 | 需要 sudo | 实时性 |
|------|--------|--------|----------|--------|
| 读 ARP 缓存 | ✅ 安全（只读） | ❌ macOS 需 sudo | ⚠️ macOS 需 sudo | ❌ 只记录已通信设备 |
| **mDNS 广播（Bonjour）** | ✅ 安全 | ✅ 全部平台 | ❌ 不需要 | ✅ 实时发现新设备 |

**mDNS（Bonjour）** 是零配置网络标准，Hermes 后端注册一个服务（如 `_hermes._tcp.local`），局域网内所有设备无需 ARP 即可发现。

### 3.2 发现流程

```
MeoClaw 启动
    ↓
┌─────────────────────────────────────────┐
│ Layer 1: 本机配置文件                     │
│ 读取 ~/.hermes/.env                      │
│ 获取用户配置的端口、地址、API Key          │
└─────────────────────────────────────────┘
    ↓ 找不到（enabled=false 或无 .env）
┌─────────────────────────────────────────┐
│ Layer 2: 本机进程扫描                     │
│ 检查 Hermes 进程是否在本机运行             │
│ 在运行 → 复用 Layer 1 的配置              │
└─────────────────────────────────────────┘
    ↓ 找不到
┌─────────────────────────────────────────┐
│ Layer 3: 本机端口探测                     │
│ 尝试本机 8642、9119 端口（带超时）         │
│ 用 HTTP GET /health 验证                 │
│ 额外尝试验证端点（见 §六.3）              │
└─────────────────────────────────────────┘
    ↓ 找不到（或者需要连其他机器）
┌─────────────────────────────────────────┐
│ Layer 4: mDNS 局域网发现  ← 替换 ARP      │
│ 查询 _hermes._tcp.local 服务             │
│ 或查询 _http._tcp.local + 端口过滤       │
│ 用 /health 验证每个候选                  │
└─────────────────────────────────────────┘
```

---

## 四、统一的 HermesConfig 结构

> 解决 v1 中 `hermes_discovery.rs` 和 `manager.rs` 两份 `HermesConfig` 冲突问题。

### 4.1 唯一定义（放到 `backend/hermes_discovery.rs`）

```rust
// backend/hermes_discovery.rs

/// Hermes 实例标识符（用于多实例管理）
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HermesInstanceId {
    pub host: String,   // "192.168.1.100" 或 "localhost"
    pub port: u16,      // 8642
}

impl HermesInstanceId {
    /// 用于 UI 显示
    pub fn display_name(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

/// Hermes 配置（统一结构，消除重复定义）
#[derive(Debug, Clone)]
pub struct HermesConfig {
    pub id: HermesInstanceId,
    pub endpoint: String,           // "http://192.168.1.100:8642"
    pub api_key: Option<String>,    // 从 .env 读取，远程实例为 None
    pub source: DiscoverySource,     // 来自哪个 Layer
    pub verified: bool,              // 是否通过 /health 验证
}

/// 发现来源
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiscoverySource {
    /// Layer 1: 本机 .env 配置文件
    ConfigFile,
    /// Layer 2: 本机进程扫描
    ProcessScan,
    /// Layer 3: 本机端口探测
    PortProbe,
    /// Layer 4: mDNS 局域网广播
    MdnsDiscovery,
}

impl std::fmt::Display for DiscoverySource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiscoverySource::ConfigFile   => write!(f, "配置文件"),
            DiscoverySource::ProcessScan  => write!(f, "进程扫描"),
            DiscoverySource::PortProbe    => write!(f, "端口探测"),
            DiscoverySource::MdnsDiscovery => write!(f, "局域网发现"),
        }
    }
}
```

### 4.2 迁移 plan

1. **删除** `manager.rs` 中的 `HermesConfig` 定义（lines 71-75）
2. **修改** `manager.rs` 的 `BackendConfig` 使用 `backend::HermesConfig`：

```rust
// backend/manager.rs
use super::hermes_discovery::HermesConfig;

// BackendConfig 改为持有一个已验证的配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendConfig {
    pub selected: ActiveBackend,
    pub openclaw: OpenClawConfig,
    // ✅ 统一使用 hermes_discovery 中的 HermesConfig
    pub hermes: Option<HermesConfig>,
    // ✅ 新增：所有发现的实例（用于 UI 切换）
    pub discovered_instances: Vec<HermesConfig>,
}
```

---

## 五、Layer 4 详细设计：mDNS/Bonjour 实现

### 5.1 跨平台库选择

| 库 | 平台 | 依赖 | 推荐 |
|----|------|------|------|
| `bon跨` (bon跨) | Unix/macOS/Windows | 无额外依赖 | ✅ 首选 |
| `dns_sd` | macOS/iOS | 系统库 | macOS 原生 |
| 手写 UDP 239.255.255.250:1900 | 全部 | 无 | ⚠️ 兜底方案 |

推荐使用 **`bon跨`**（跨平台 Bonjour 库），`Cargo.toml` 添加：

```toml
[dependencies]
bon跨 = "0.5"
```

### 5.2 mDNS 查询流程

```rust
// Layer 4: mDNS 局域网发现
pub fn discover_via_mdns(timeout_secs: u64) -> Vec<HermesConfig> {
    let mut results = Vec::new();

    // 方法 A：查询 Hermes 专用服务名
    // Hermes 后端需要注册 _hermes._tcp.local 服务（可选增强）
    let hermes_services = bon跨::query::<bon跨::TxtRecord>("_hermes._tcp.local.");

    // 方法 B（更通用）：查询所有 HTTP 服务，过滤端口
    let http_services = bon跨::query::<bon跨::TxtRecord>("_http._tcp.local.");
    let candidate_services: Vec<_> = http_services
        .into_iter()
        .filter(|svc| matches!(svc.port, 8642 | 9119 | 8080 | 8000 | 3000 | 80))
        .collect();

    // 对每个候选发 /health 验证
    for service in hermes_services.into_iter().chain(candidate_services) {
        let host = service.hostname.trim_end_matches('.');
        let url = format!("http://{}:{}/health", host, service.port);

        if let Ok(true) = verify_hermes_health(&url) {
            results.push(HermesConfig {
                id: HermesInstanceId {
                    host: host.to_string(),
                    port: service.port,
                },
                endpoint: format!("http://{}:{}", host, service.port),
                api_key: None,  // 远程实例无法获取 API Key，见 §六.2
                source: DiscoverySource::MdnsDiscovery,
                verified: true,
            });
        }
    }

    results
}
```

### 5.3 Hermes 后端需要注册 mDNS 服务（非必须）

> 如果 Hermes 后端实现了 mDNS 服务注册，Layer 4 更可靠。以下是可选的 Hermes 端实现：

```rust
// Hermes 端注册 mDNS 服务（hermes gateway 侧，非 MeoClaw 侧）
use bon跨::Service;

let service = Service::new("_hermes._tcp", 8642)
    .with_txt_record("version", env!("CARGO_PKG_VERSION"))
    .with_txt_record("api_version", "v1");

service.register();
```

---

## 六、修复的关键问题

### 6.1 修复 HermesClient 硬编码（🔴 Critical）

> 确保发现的结果实际被使用。

```rust
// hermes_client.rs - HermesClient::new() 不再自己 discover
pub struct HermesClient {
    // ✅ 不再从 .env 读 endpoint，从 BackendManager 传入
    endpoint: String,
    api_key: Option<String>,
    client: Client,
}

impl HermesClient {
    /// 创建时直接接收已发现的配置，不再自己 discover
    pub fn from_config(config: &HermesConfig) -> Self {
        Self {
            endpoint: config.endpoint.clone(),
            api_key: config.api_key.clone(),
            client: Client::new(),
        }
    }

    /// 发送消息，使用动态 endpoint
    fn send_message_internal(&self, text: &str) -> Result<Box<dyn Stream>, Box<dyn Error + Send + Sync>> {
        // ✅ 修复：使用 self.endpoint 而不是硬编码的 127.0.0.1:8642
        let url = format!("{}/v1/chat/completions", self.endpoint);
        let mut request = self.client.post(&url).json(&request_body);
        // ...
    }
}
```

**BackendManager 调用改造**：

```rust
// manager.rs - 初始化时执行发现，结果传给 HermesClient
pub fn new() -> Self {
    let discovered = HermesDiscovery::discover_all();  // 返回 Vec<HermesConfig>

    // 优先使用 Layer 1-3 发现的本地实例（有 API Key）
    let primary = discovered.iter()
        .find(|c| c.source != DiscoverySource::MdnsDiscovery && c.api_key.is_some())
        .cloned()
        .or_else(|| discovered.first().cloned());

    let mut config = Self::load_config().unwrap_or_default();
    if let Some(ref primary) = primary {
        config.hermes = Some(primary.clone());
    }
    config.discovered_instances = discovered;

    let manager = Self {
        config: Arc::new(Mutex::new(config)),
        clients: Arc::new(Mutex::new(HashMap::new())),
    };

    // 注册 HermesClient，传入已发现的 endpoint
    if let Some(ref hermes_cfg) = manager.config.lock().unwrap().hermes {
        manager.register_client(Box::new(HermesClient::from_config(hermes_cfg)));
    }

    manager
}
```

### 6.2 远程 Hermes 的 API Key 处理

Layer 4 发现的远程 Hermes 实例**无法读取对方的 `.env`**。两种处理方式：

| 方案 | 实现 | 优点 | 缺点 |
|------|------|------|------|
| **A. 无 Key 也允许连** | 远程实例不带 API Key 也能连接（依赖 Hermes 端配置） | 零配置 | 如果 Hermes 有认证中间件会失败 |
| **B. 本机配全局 Key** | ~/.hermes/.env 存一个"通用 Key"，发给所有 Hermes | 简单 | 需要 Hermes 支持通用 Key |
| **C. mDNS TXT 传 Key 指纹** | Hermes 注册时把 Key 的 SHA256 指纹放 TXT record | 安全 | 需要 Hermes 端配合 |

**推荐方案 A + B 组合**：
1. 优先用本机 `.env` 的 Key（Layer 1-3 发现）
2. Layer 4 发现的远程实例，用本机 `.env` 的 Key 尝试
3. 都失败则在 UI 提示用户手动输入 Key

### 6.3 修复 `/health` 认证误判

有些 Hermes 部署有全局认证中间件，`/health` 也需要 Auth header。

```rust
/// 验证 Hermes 实例（尝试多种端点和认证方式）
fn verify_hermes_health(url: &str) -> Result<bool, DiscoveryError> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(2))  // ✅ 加 2 秒超时
        .build()?;

    // 1. 尝试无认证 /health
    if client.get(&format!("{}/health", url)).send()?.status().is_success() {
        return Ok(true);
    }

    // 2. 尝试带 Authorization /health
    if let Some(key) = load_local_api_key() {
        if client.get(&format!("{}/health", url))
            .header("Authorization", format!("Bearer {}", key))
            .send()?.status().is_success() {
            return Ok(true);
        }
    }

    // 3. 尝试备用端点 /api/health
    for path in &["/api/health", "/api/status", "/status"] {
        if client.get(&format!("{}{}", url.trim_end_matches("/health"), path))
            .send()?.status().is_success() {
            return Ok(true);
        }
    }

    Err(DiscoveryError::HealthCheckFailed)
}
```

### 6.4 进程扫描跨平台实现

```rust
fn check_hermes_process_running() -> bool {
    #[cfg(target_os = "macos")]
    {
        Command::new("pgrep").args(["-f", "hermes"]).output()
            .map(|o| o.status.success()).unwrap_or(false)
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("pgrep").args(["-f", "hermes"]).output()
            .map(|o| o.status.success()).unwrap_or(false)
    }

    #[cfg(target_os = "windows")]
    {
        Command::new("tasklist")
            .args(["/FI", "IMAGENAME eq *hermes*"])
            .output()
            .map(|o| o.status.success() && String::from_utf8_lossy(&o.stdout).contains("hermes"))
            .unwrap_or(false)
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        false  // 其他平台跳过进程扫描
    }
}
```

---

## 七、多实例管理架构

### 7.1 存储结构

```rust
// discovered_instances.json（独立文件，Layer 4 发现结果）
[
  {
    "id": { "host": "192.168.1.100", "port": 8642 },
    "endpoint": "http://192.168.1.100:8642",
    "api_key": null,
    "source": "mdns",
    "verified": true,
    "last_seen": "2026-04-23T10:30:00Z",
    "hostname": "johns-macbook.local"
  }
]
```

### 7.2 切换逻辑

```
用户切换到其他 Hermes 实例
    ↓
BackendManager.switch_instance(instance_id)
    ↓
1. 更新 config.hermes = 选中实例
2. 重新注册 HermesClient（用新 endpoint）
3. 触发健康检测，失败则自动回退
```

### 7.3 自动切换

```
启动时或定时（每 60 秒）后台扫描
    ↓
发现更优实例（同网段 + 有 API Key）
    ↓
如果当前实例不健康 → 自动切换
如果当前实例健康 → 仅记录，不切换
```

---

## 八、涉及修改的文件

| 文件 | 操作 | 修改内容 |
|------|------|---------|
| `src-tauri/src/backend/hermes_discovery.rs` | 修改 | 新增 `HermesInstanceId`、`DiscoverySource`、Layer 4 mDNS、`discover_all()`、`verify_hermes_health()` |
| `src-tauri/src/backend/hermes_client.rs` | 修改 | 移除硬编码 endpoint，改为从 config 构造；新增 `from_config()` |
| `src-tauri/src/backend/manager.rs` | 修改 | 删除重复 `HermesConfig`，引入统一定义；新增 `discovered_instances`、`switch_instance()` |
| `src-tauri/src/lib.rs` | 修改 | 初始化时调用 `discover_all()` |
| `src/options.ts` | 修改 | 显示多 Hermes 选项；切换按钮 |
| `Cargo.toml` | 修改 | 新增 `bon跨 = "0.5"` 依赖 |
| `DOC/功能/Hermes后端集成.md` | 修改 | 更新配置发现相关章节 |

---

## 九、启动时序图

```
MeoClaw 启动
    ↓
Layer 1: 读 ~/.hermes/.env
    ↓
Layer 2: 扫 Hermes 进程（跨平台 pgrep/tasklist）
    ↓
Layer 3: 试 localhost:8642/9119 + /health 验证（2s 超时）
    ↓
Layer 4: mDNS 查询 _http._tcp.local + 端口过滤 + /health 验证
    ↓
收集所有发现的实例 → Vec<HermesConfig>
    ↓
优先选择：同网段有 API Key 的实例
    ↓
注册 HermesClient → 开始处理消息
    ↓
后台：每 60s 重新扫描，发现更优实例则自动切换
```

---

## 十、性能预估

| 步骤 | 时间 | 说明 |
|------|------|------|
| Layer 1-2 | < 50ms | 读文件 + 进程扫描 |
| Layer 3（2 个端口） | ~4s max | 每个端口 2s 超时 |
| Layer 4（mDNS） | ~3-5s | mDNS 查询 + 每个候选 2s 超时，最多 10 个候选 |
| **总计（悲观）** | **< 10s** | 全部在后台静默执行 |

mDNS 比 ARP 扫描更快，无需逐个 IP 探测。

---

## 十一、实现 Checklist

- [x] **统一 HermesConfig 定义**（消除重复 struct）✅
- [x] **修复 HermesClient 硬编码**（使用动态 endpoint）✅
- [ ] **Layer 4 mDNS 发现实现**（预留接口，stub 返回空列表）
- [x] **进程扫描跨平台（Windows tasklist）** ✅
- [x] **添加 2s 超时的 health 验证** ✅
- [x] **/health 认证 header 降级尝试** ✅
- [x] **多实例 discovered_instances 存储** ✅
- [x] **BackendManager.switch_hermes_instance() 实现** ✅
- [ ] **后台定时重新扫描（60s 间隔）**
- [ ] **前端多后端选择 UI（options.ts）**
- [ ] **文档更新（Hermes后端集成.md）**
- [ ] **SSE 解析 bug 修复**（消息卡在 working 状态）

---

## 十二、已实现的代码结构

### 12.1 核心类型（hermes_discovery.rs）

```rust
// Hermes 实例唯一标识符
pub struct HermesInstanceId {
    pub host: String,   // "192.168.1.100" 或 "localhost"
    pub port: u16,      // 8642
}

// 发现来源枚举
pub enum DiscoverySource {
    ConfigFile,    // Layer 1: 配置文件
    ProcessScan,   // Layer 2: 进程扫描
    PortProbe,     // Layer 3: 端口探测
    MdnsDiscovery, // Layer 4: mDNS（预留）
}

// 统一配置结构
pub struct HermesConfig {
    pub id: HermesInstanceId,
    pub endpoint: String,        // "http://192.168.1.100:8642"
    pub api_key: Option<String>, // 从 .env 读取
    pub source: DiscoverySource,
    pub verified: bool,          // 是否通过 /health 验证
}
```

### 12.2 BackendManager 新增方法

```rust
// 发现所有 Hermes 实例
pub fn discover_hermes_all(&self) -> Vec<HermesConfig>

// 切换到指定实例
pub fn switch_hermes_instance(&self, instance_id: &HermesInstanceId) -> Result<(), String>

// 获取已发现的实例列表
pub fn get_discovered_instances(&self) -> Vec<HermesConfig>
```

### 12.3 HermesClient 新增构造方法

```rust
// 从发现配置创建客户端（修复硬编码问题）
pub fn from_config(config: &HermesConfig) -> Self
```

---

## 十三、附录：替代方案对比

### Layer 4 替代方案

| 方案 | 实现难度 | 可靠性 | 跨平台 | 推荐场景 |
|------|---------|--------|--------|---------|
| **mDNS/Bonjour** | 中等 | 高 | ✅ 全部 | **首选**，需要 Hermes 配合注册服务 |
| SSDP（UPnP） | 低 | 中 | ✅ 全部 | 备选，无需服务端修改 |
| 读取 ARP 缓存 | 低 | 低 | ⚠️ macOS 需 sudo | 不推荐 |
| 扫描整个局域网段 | 低 | 低 | ✅ 全部 | 临时调试用，不适合生产 |
