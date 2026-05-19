# Hermes 发现机制

## 概述

Hermes 采用 4 层发现机制，自动探测本地或局域网内的 Hermes 实例。

## 发现层级

| 层级 | 来源 | 优先级 |
|------|------|--------|
| Layer 1 | 本机 .env 配置文件 | 最高 |
| Layer 2 | 本机进程扫描 | 高 |
| Layer 3 | 本机端口探测 | 中 |
| Layer 4 | mDNS 局域网广播 | 低 |

## 发现来源枚举

```rust
pub enum DiscoverySource {
    ConfigFile,     // Layer 1: 本机 .env 配置文件
    ProcessScan,   // Layer 2: 本机进程扫描
    PortProbe,     // Layer 3: 本机端口探测
    MdnsDiscovery, // Layer 4: mDNS 局域网广播
}
```

## HermesConfig 结构

```rust
pub struct HermesConfig {
    pub id: HermesInstanceId,          // 实例标识符
    pub endpoint: String,               // "http://192.168.1.100:8642"
    pub api_key: Option<String>,       // 从 .env 读取
    pub source: DiscoverySource,        // 发现来源
    pub verified: bool,                 // 是否通过 /health 验证
}
```

## HermesInstanceId 结构

```rust
pub struct HermesInstanceId {
    pub host: String,  // "192.168.1.100" 或 "localhost"
    pub port: u16,    // 8642
}
```

## 各层发现详解

### Layer 1: 配置文件 (ConfigFile)

从 `~/.hermes/.env` 读取配置：

```
HERMES_HOST=127.0.0.1
HERMES_PORT=8642
HERMES_API_KEY=your-api-key
```

### Layer 2: 进程扫描 (ProcessScan)

扫描本地运行的 Hermes 进程，获取监听地址。

### Layer 3: 端口探测 (PortProbe)

探测本地常见端口：
- 8642 (Hermes 默认)
- 8643, 8644 (备选)

### Layer 4: mDNS 广播 (MdnsDiscovery)

通过 mDNS 发送局域网广播，发现远程 Hermes 实例。

## 发现流程

```
discover_all()
    │
    ├─► Layer 1: from_env_config() → Option<HermesConfig>
    │
    ├─► Layer 2: scan_processes() → Vec<HermesConfig>
    │
    ├─► Layer 3: probe_ports() → Vec<HermesConfig>
    │
    └─► Layer 4: mdns_discovery() → Vec<HermesConfig>
              │
              └─► 对每个候选执行 health_check()
```

## 健康检查

每个发现的实例都会通过 `/health` 端点验证可用性：

```
GET http://{host}:{port}/health
```

## 代码位置

| 文件 | 说明 |
|------|------|
| `src-tauri/src/backend/hermes_discovery.rs` | 发现机制实现 |
| `src-tauri/src/backend/hermes_client.rs` | Hermes 客户端实现 |

## 配置更新

发现结果会更新到 `BackendConfig.discovered_instances`，可通过 UI 选择切换。
