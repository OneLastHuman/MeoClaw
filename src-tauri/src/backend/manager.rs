use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex};
use serde::{de::Visitor, Deserialize, Deserializer, Serialize};

use super::client_trait::{BackendClient, Stream};
use super::hermes_discovery::{HermesDiscovery, HermesConfig, HermesInstanceId};
pub use super::hermes_discovery::LegacyHermesConfig;

/// 活跃后端枚举 - 支持大小写不敏感的 JSON 反序列化
#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum ActiveBackend {
    OpenClaw,
    Hermes,
}

/// 自定义反序列化，支持 "openclaw"/"OpenClaw"/"OPENCLAW" 等变体
impl<'de> Deserialize<'de> for ActiveBackend {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ActiveBackendVisitor;

        impl<'de> Visitor<'de> for ActiveBackendVisitor {
            type Value = ActiveBackend;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("\"openclaw\" or \"hermes\" (case-insensitive)")
            }

            fn visit_str<E>(self, value: &str) -> Result<ActiveBackend, E>
            where
                E: serde::de::Error,
            {
                match value.to_lowercase().as_str() {
                    "openclaw" => Ok(ActiveBackend::OpenClaw),
                    "hermes" => Ok(ActiveBackend::Hermes),
                    _ => Err(E::invalid_value(serde::de::Unexpected::Str(value), &self)),
                }
            }
        }

        deserializer.deserialize_str(ActiveBackendVisitor)
    }
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

/// 后端配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendConfig {
    pub selected: ActiveBackend,
    pub openclaw: OpenClawConfig,
    /// 当前选中的 Hermes 实例（用于兼容旧接口）
    pub hermes: LegacyHermesConfig,
    /// 所有发现的 Hermes 实例（用于 UI 切换）
    #[serde(default)]
    pub discovered_instances: Vec<HermesConfig>,
}

impl Default for BackendConfig {
    fn default() -> Self {
        Self {
            selected: ActiveBackend::OpenClaw,
            openclaw: OpenClawConfig {
                endpoint: "ws://127.0.0.1:18789".to_string(),
                health_endpoint: "http://127.0.0.1:18789".to_string(),
            },
            hermes: LegacyHermesConfig {
                endpoint: "http://127.0.0.1:8642".to_string(),
                health_endpoint: "http://127.0.0.1:8642/health".to_string(),
            },
            discovered_instances: Vec::new(),
        }
    }
}

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

/// BackendManager - 统一管理所有后端客户端
pub struct BackendManager {
    config: Arc<Mutex<BackendConfig>>,
    clients: Arc<Mutex<HashMap<String, Box<dyn BackendClient>>>>,
}

impl BackendManager {
    /// 获取默认配置文件路径
    fn default_config_path() -> std::path::PathBuf {
        dirs::data_local_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join("meo-claw")
            .join("backend-config.json")
    }

    /// 从文件加载配置
    fn load_config() -> Result<BackendConfig, String> {
        let path = Self::default_config_path();
        if path.exists() {
            BackendConfig::load(&path)
        } else {
            Err("Config file does not exist".to_string())
        }
    }

    /// 保存配置到文件
    fn save_config(&self) -> Result<(), String> {
        let config = self.config.lock().map_err(|e| e.to_string())?;
        let path = Self::default_config_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        config.save(&path)
    }

    pub fn new() -> Self {
        let config = Self::load_config().unwrap_or_default();
        Self {
            config: Arc::new(Mutex::new(config)),
            clients: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// 注册后端客户端
    pub fn register_client(&self, client: Box<dyn BackendClient>) -> Result<(), String> {
        let name = client.name().to_string();
        let mut clients = self.clients.lock().map_err(|e| e.to_string())?;
        clients.insert(name, client);
        Ok(())
    }

    /// 获取后端客户端并发送消息，返回流
    ///
    /// 这个方法在锁内调用 send_message，然后将流返回给调用者
    /// 调用者可以在锁外处理流
    pub fn send_message_to_client(
        &self,
        name: &str,
        message: &str,
    ) -> Result<Box<dyn Stream>, String> {
        let clients = self.clients.lock().map_err(|e| e.to_string())?;
        let client = clients
            .get(name)
            .ok_or_else(|| format!("Client '{}' not found", name))?;
        client.send_message(message).map_err(|e| e.to_string())
    }

    /// 切换活跃后端
    pub fn switch(&self, backend: &str) -> Result<(), String> {
        let active = ActiveBackend::from_str(backend)
            .ok_or_else(|| format!("Unknown backend: {}", backend))?;

        let mut config = self.config.lock().map_err(|e| e.to_string())?;
        config.selected = active;
        drop(config);

        self.save_config()?;
        Ok(())
    }

    /// 获取当前活跃后端
    pub fn current(&self) -> Option<String> {
        self.config.lock().ok()
            .map(|c| c.selected.as_str().to_string())
    }

    /// 获取选中的后端
    pub fn get_selected_backend(&self) -> Option<ActiveBackend> {
        self.config.lock().ok().map(|c| c.selected.clone())
    }

    /// 检测所有已注册后端健康状态
    pub fn check_all_health(&self) -> Vec<BackendHealth> {
        let clients = match self.clients.lock() {
            Ok(c) => c,
            Err(e) => return vec![BackendHealth {
                available: false,
                backend: "unknown".to_string(),
                endpoint: "".to_string(),
                error: Some(e.to_string()),
            }],
        };
        clients.iter().map(|(name, client)| {
            let available = client.check_health();
            BackendHealth {
                available,
                backend: name.clone(),
                endpoint: client.endpoint().to_string(),
                error: if available { None } else { Some("Health check failed".to_string()) },
            }
        }).collect()
    }

    /// 检测当前后端健康状态
    pub fn check_health(&self) -> BackendHealth {
        let (client_name, endpoint) = {
            let config = match self.config.lock() {
                Ok(c) => c,
                Err(e) => return BackendHealth {
                    available: false,
                    backend: "unknown".to_string(),
                    endpoint: "".to_string(),
                    error: Some(e.to_string()),
                },
            };
            let name = config.selected.as_str().to_string();
            let ep = match config.selected {
                ActiveBackend::OpenClaw => config.openclaw.endpoint.clone(),
                ActiveBackend::Hermes => config.hermes.endpoint.clone(),
            };
            (name, ep)
        }; // config lock dropped here

        let health = match self.clients.lock() {
            Ok(clients) => {
                match clients.get(&client_name) {
                    Some(c) => {
                        let available = c.check_health();
                        BackendHealth {
                            available,
                            backend: client_name,
                            endpoint,
                            error: if available { None } else { Some("Health check failed".to_string()) },
                        }
                    }
                    None => BackendHealth {
                        available: false,
                        backend: client_name.clone(),
                        endpoint,
                        error: Some(format!("Client '{}' not registered", client_name)),
                    },
                }
            }
            Err(e) => BackendHealth {
                available: false,
                backend: client_name,
                endpoint,
                error: Some(e.to_string()),
            },
        };
        health
    }

    /// 发现所有 Hermes 实例（4 层发现）
    pub fn discover_hermes_all(&self) -> Vec<HermesConfig> {
        let instances = HermesDiscovery::discover_all();

        // 更新配置中的发现列表
        if let Ok(mut config) = self.config.lock() {
            config.discovered_instances = instances.clone();
        }

        instances
    }

    /// 切换到指定的 Hermes 实例
    pub fn switch_hermes_instance(&self, instance_id: &HermesInstanceId) -> Result<(), String> {
        let config = self.config.lock().map_err(|e| e.to_string())?;

        // 从已发现的实例中找目标
        let target = config.discovered_instances.iter()
            .find(|i| i.id == *instance_id)
            .ok_or_else(|| format!("Hermes instance not found: {:?}", instance_id))?;

        // 更新当前选中的 hermes 配置（使用 LegacyHermesConfig 保持兼容）
        let new_hermes = LegacyHermesConfig {
            endpoint: target.endpoint.clone(),
            health_endpoint: format!("{}/health", target.endpoint),
        };

        drop(config);

        if let Ok(mut config) = self.config.lock() {
            config.hermes = new_hermes;
        }

        self.save_config()?;
        log::info!("[BackendManager] Switched to Hermes instance: {}", instance_id.display_name());
        Ok(())
    }

    /// 获取所有发现的 Hermes 实例
    pub fn get_discovered_instances(&self) -> Vec<HermesConfig> {
        self.config.lock()
            .ok()
            .map(|c| c.discovered_instances.clone())
            .unwrap_or_default()
    }
}

impl Default for BackendManager {
    fn default() -> Self {
        Self::new()
    }
}

/// 后端健康状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendHealth {
    pub available: bool,
    pub backend: String,
    pub endpoint: String,
    pub error: Option<String>,
}