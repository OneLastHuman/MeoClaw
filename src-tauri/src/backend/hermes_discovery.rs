use std::path::PathBuf;
use std::process::Command;
use std::fs;
use std::time::Duration;
use serde::{Serialize, Deserialize};

/// Hermes 实例标识符（用于多实例管理）
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

/// 发现来源
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

/// Hermes 配置（统一结构，消除重复定义）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HermesConfig {
    pub id: HermesInstanceId,
    pub endpoint: String,           // "http://192.168.1.100:8642"
    pub api_key: Option<String>,    // 从 .env 读取，远程实例为 None
    pub source: DiscoverySource,     // 来自哪个 Layer
    pub verified: bool,              // 是否通过 /health 验证
}

impl HermesConfig {
    /// 创建新配置
    pub fn new(host: &str, port: u16, source: DiscoverySource) -> Self {
        Self {
            id: HermesInstanceId {
                host: host.to_string(),
                port,
            },
            endpoint: format!("http://{}:{}", host, port),
            api_key: None,
            source,
            verified: false,
        }
    }

    /// 转换为用于 BackendManager 的旧格式（兼容性）
    pub fn to_legacy_config(&self) -> LegacyHermesConfig {
        LegacyHermesConfig {
            endpoint: self.endpoint.clone(),
            health_endpoint: format!("{}/health", self.endpoint),
        }
    }
}

/// 向后兼容的 HermesConfig（供 BackendManager 使用）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyHermesConfig {
    pub endpoint: String,
    pub health_endpoint: String,
}

/// 三级发现机制
pub struct HermesDiscovery;

impl HermesDiscovery {
    /// Layer 1: 从 ~/.hermes/.env 读取配置
    pub fn from_env_config() -> Option<HermesConfig> {
        let hermes_home = dirs::home_dir()?.join(".hermes");
        let env_path = hermes_home.join(".env");

        if !env_path.exists() {
            log::info!("[HermesDiscovery] .env file not found at {:?}", env_path);
            return None;
        }

        let content = fs::read_to_string(&env_path).ok()?;
        let env_vars = parse_env_file(&content);

        // 检查 API_SERVER_ENABLED 或 API_SERVER_KEY
        let enabled = env_vars.get("API_SERVER_ENABLED")
            .map(|v| is_truthy(v))
            .unwrap_or(false)
            || env_vars.contains_key("API_SERVER_KEY");

        log::info!("[HermesDiscovery] from_env_config - API_SERVER_ENABLED: {:?}, API_SERVER_KEY present: {}", env_vars.get("API_SERVER_ENABLED"), env_vars.contains_key("API_SERVER_KEY"));

        if !enabled {
            log::info!("[HermesDiscovery] Hermes not enabled");
            return None;
        }

        let host = env_vars.get("API_SERVER_HOST")
            .cloned()
            .unwrap_or_else(|| "127.0.0.1".to_string());

        let port = env_vars.get("API_SERVER_PORT")
            .and_then(|v| v.parse().ok())
            .unwrap_or(8642);

        let api_key = env_vars.get("API_SERVER_KEY").cloned();
        log::info!("[HermesDiscovery] Discovered endpoint: http://{}:{}, api_key present: {}", host, port, api_key.is_some());

        let mut config = HermesConfig::new(&host, port, DiscoverySource::ConfigFile);
        config.api_key = api_key;
        config.verified = true; // 本地配置默认已验证

        Some(config)
    }

    /// Layer 2: 进程扫描发现
    pub fn discover_via_process() -> Option<HermesConfig> {
        // 1. 找 hermes CLI 路径
        let _hermes_path = find_hermes_cli()?;

        // 2. 检查进程是否运行
        let is_running = check_hermes_process_running();
        if !is_running {
            return None;
        }

        // 3. 读取 hermes 配置目录的 .env
        if let Some(home) = dirs::home_dir() {
            let env_path = home.join(".hermes").join(".env");
            if env_path.exists() {
                // 复用 Layer 1 的逻辑
                if let Some(mut config) = Self::from_env_config() {
                    config.source = DiscoverySource::ProcessScan;
                    return Some(config);
                }
            }
        }

        None
    }

    /// Layer 3: 默认端口尝试验证（带超时）
    pub fn try_default_ports() -> Option<HermesConfig> {
        let ports = [8642, 9119];

        for port in ports {
            let host = "127.0.0.1";
            let url = format!("http://{}:{}/health", host, port);

            if verify_hermes_health(&url, None).is_ok() {
                let mut config = HermesConfig::new(host, port, DiscoverySource::PortProbe);
                config.verified = true;
                return Some(config);
            }

            // 也尝试 Dashboard API
            let status_url = format!("http://{}:{}/api/status", host, port);
            if let Ok(true) = verify_hermes_health(&status_url, None) {
                let mut config = HermesConfig::new(host, port, DiscoverySource::PortProbe);
                config.verified = true;
                return Some(config);
            }
        }

        None
    }

    /// Layer 4: mDNS 局域网发现
    ///
    /// 注意：mDNS/Bonjour 实现需要添加 bonjour crate 依赖
    /// 当前版本返回空列表，后续通过 feature flag 启用
    pub fn discover_via_mdns(_timeout_secs: u64) -> Vec<HermesConfig> {
        log::info!("[HermesDiscovery] Layer 4 mDNS discovery not yet implemented, skipping");
        Vec::new()
    }

    /// 主入口：级联检测，返回所有发现的实例
    pub fn discover_all() -> Vec<HermesConfig> {
        let mut results: Vec<HermesConfig> = Vec::new();
        let mut seen_endpoints: std::collections::HashSet<String> = std::collections::HashSet::new();

        // Layer 1: 配置文件
        if let Some(config) = Self::from_env_config() {
            let endpoint = config.endpoint.clone();
            if !seen_endpoints.contains(&endpoint) {
                seen_endpoints.insert(endpoint);
                results.push(config);
            }
        }

        // Layer 2: 进程扫描
        if let Some(config) = Self::discover_via_process() {
            let endpoint = config.endpoint.clone();
            if !seen_endpoints.contains(&endpoint) {
                seen_endpoints.insert(endpoint);
                results.push(config);
            }
        }

        // Layer 3: 端口探测
        if let Some(config) = Self::try_default_ports() {
            let endpoint = config.endpoint.clone();
            if !seen_endpoints.contains(&endpoint) {
                seen_endpoints.insert(endpoint);
                results.push(config);
            }
        }

        // Layer 4: mDNS
        let mdns_results = Self::discover_via_mdns(3);
        for config in mdns_results {
            let endpoint = config.endpoint.clone();
            if !seen_endpoints.contains(&endpoint) {
                seen_endpoints.insert(endpoint);
                results.push(config);
            }
        }

        log::info!("[HermesDiscovery] discover_all found {} instances", results.len());
        results
    }

    /// 简单入口：返回第一个发现的配置（兼容旧接口）
    pub fn discover() -> Option<HermesConfig> {
        Self::discover_all().into_iter().next()
    }

    /// 验证 Hermes 实例，返回是否健康
    pub fn verify_hermes(url: &str) -> Result<bool, DiscoveryError> {
        verify_hermes_health(url, None)
    }
}

/// 验证 Hermes 实例（尝试多种端点和认证方式）
fn verify_hermes_health(url: &str, api_key: Option<&str>) -> Result<bool, DiscoveryError> {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(1))
        .build()
        .map_err(|_| DiscoveryError::NetworkError)?;

    // 1. 尝试无认证 /health
    let health_url = if url.ends_with("/health") {
        url.to_string()
    } else {
        format!("{}/health", url.trim_end_matches('/'))
    };

    if let Ok(response) = client.get(&health_url).send() {
        if response.status().is_success() {
            return Ok(true);
        }
    }

    // 2. 尝试带 Authorization /health
    if let Some(key) = api_key {
        let alt_url = if health_url.ends_with("/health") {
            &health_url[..health_url.len() - 7]
        } else {
            url
        };
        let probe_url = format!("{}/health", alt_url.trim_end_matches('/'));

        if client.get(&probe_url)
            .header("Authorization", format!("Bearer {}", key))
            .send()
            .map(|r| r.status().is_success())
            .unwrap_or(false)
        {
            return Ok(true);
        }
    }

    // 3. 尝试备用端点
    let base = health_url.trim_end_matches("/health");
    for path in &["/api/health", "/api/status", "/status"] {
        let probe_url = format!("{}{}", base, path);
        if let Ok(response) = client.get(&probe_url).send() {
            if response.status().is_success() {
                return Ok(true);
            }
        }
    }

    Err(DiscoveryError::HealthCheckFailed)
}

/// 发现错误
#[derive(Debug)]
pub enum DiscoveryError {
    NetworkError,
    HealthCheckFailed,
    ParseError,
}

/// 辅助函数
fn parse_env_file(content: &str) -> std::collections::HashMap<String, String> {
    let mut map = std::collections::HashMap::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(pos) = line.find('=') {
            let key = line[..pos].trim().to_string();
            let mut value = line[pos + 1..].trim().to_string();
            // 去掉引号
            if (value.starts_with('"') && value.ends_with('"'))
                || (value.starts_with('\'') && value.ends_with('\''))
            {
                value = value[1..value.len() - 1].to_string();
            }
            map.insert(key, value);
        }
    }
    map
}

fn is_truthy(value: &str) -> bool {
    matches!(value.to_lowercase().as_str(), "1" | "true" | "yes" | "on")
}

fn find_hermes_cli() -> Option<PathBuf> {
    // Mac/Linux 用 `which`, Windows 用 `where`
    let which_cmd = if cfg!(target_os = "windows") { "where" } else { "which" };

    let output = Command::new(which_cmd)
        .arg("hermes")
        .output()
        .ok()?;

    if output.status.success() {
        let path = String::from_utf8_lossy(&output.stdout);
        let path = path.trim();
        if !path.is_empty() {
            return Some(PathBuf::from(path));
        }
    }

    // 尝试常见路径
    if let Some(home) = dirs::home_dir() {
        let local_bin = if cfg!(target_os = "windows") {
            home.join(".local").join("bin").join("hermes.exe")
        } else {
            home.join(".local").join("bin").join("hermes")
        };
        if local_bin.exists() {
            return Some(local_bin);
        }
    }

    None
}

/// 跨平台进程扫描
fn check_hermes_process_running() -> bool {
    #[cfg(target_os = "macos")]
    {
        Command::new("pgrep")
            .args(["-f", "hermes"])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("pgrep")
            .args(["-f", "hermes"])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    #[cfg(target_os = "windows")]
    {
        Command::new("tasklist")
            .args(["/FI", "IMAGENAME eq *hermes*"])
            .output()
            .map(|o| {
                o.status.success() && String::from_utf8_lossy(&o.stdout).contains("hermes")
            })
            .unwrap_or(false)
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hermes_instance_id_display_name() {
        let id = HermesInstanceId {
            host: "localhost".to_string(),
            port: 8642,
        };
        assert_eq!(id.display_name(), "localhost:8642");
    }

    #[test]
    fn test_hermes_config_new() {
        let config = HermesConfig::new("127.0.0.1", 8642, DiscoverySource::PortProbe);
        assert_eq!(config.endpoint, "http://127.0.0.1:8642");
        assert_eq!(config.source, DiscoverySource::PortProbe);
        assert!(!config.verified);
    }

    #[test]
    fn test_parse_env_file() {
        let content = r#"
# comment
API_SERVER_HOST=192.168.1.100
API_SERVER_PORT=8642
API_SERVER_KEY="secret_key"
"#;
        let map = parse_env_file(content);
        assert_eq!(map.get("API_SERVER_HOST"), Some(&"192.168.1.100".to_string()));
        assert_eq!(map.get("API_SERVER_PORT"), Some(&"8642".to_string()));
        assert_eq!(map.get("API_SERVER_KEY"), Some(&"secret_key".to_string()));
    }

    #[test]
    fn test_is_truthy() {
        assert!(is_truthy("true"));
        assert!(is_truthy("True"));
        assert!(is_truthy("TRUE"));
        assert!(is_truthy("1"));
        assert!(is_truthy("yes"));
        assert!(is_truthy("on"));
        assert!(!is_truthy("false"));
        assert!(!is_truthy("0"));
    }
}
