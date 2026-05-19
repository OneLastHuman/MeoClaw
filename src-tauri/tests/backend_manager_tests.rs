//! BackendManager 单元测试

use meo_claw_lib::backend::{
    ActiveBackend, BackendConfig, BackendManager, HermesClient,
};

/// 测试 ActiveBackend::from_str
#[test]
fn test_active_backend_from_str() {
    assert_eq!(ActiveBackend::from_str("openclaw"), Some(ActiveBackend::OpenClaw));
    assert_eq!(ActiveBackend::from_str("hermes"), Some(ActiveBackend::Hermes));
    assert_eq!(ActiveBackend::from_str("invalid"), None);
}

/// 测试 ActiveBackend::as_str
#[test]
fn test_active_backend_as_str() {
    assert_eq!(ActiveBackend::OpenClaw.as_str(), "openclaw");
    assert_eq!(ActiveBackend::Hermes.as_str(), "hermes");
}

/// 测试 BackendConfig Default
#[test]
fn test_backend_config_default() {
    let config = BackendConfig::default();
    assert_eq!(config.selected, ActiveBackend::OpenClaw);
    assert_eq!(config.openclaw.endpoint, "ws://127.0.0.1:18789");
    assert_eq!(config.hermes.endpoint, "http://127.0.0.1:8642");
}

/// 测试 BackendManager::new
#[test]
fn test_backend_manager_new() {
    let manager = BackendManager::new();
    // 默认应该是 openclaw，除非用户已经选择了 hermes
    let current = manager.current();
    assert!(current.is_some(), "current should return Some");
    assert!(current == Some("openclaw".to_string()) || current == Some("hermes".to_string()),
        "current should be openclaw or hermes, got {:?}", current);
}

/// 测试 BackendManager::switch
#[test]
fn test_backend_manager_switch() {
    let manager = BackendManager::new();

    // 切换到 hermes
    manager.switch("hermes").unwrap();
    assert_eq!(manager.current(), Some("hermes".to_string()));

    // 切换回 openclaw
    manager.switch("openclaw").unwrap();
    assert_eq!(manager.current(), Some("openclaw".to_string()));

    // 切换到无效后端
    assert!(manager.switch("invalid").is_err());
}

/// 测试 BackendManager::get_selected_backend
#[test]
fn test_backend_manager_get_selected_backend() {
    let manager = BackendManager::new();

    manager.switch("hermes").unwrap();
    assert_eq!(manager.get_selected_backend(), Some(ActiveBackend::Hermes));

    manager.switch("openclaw").unwrap();
    assert_eq!(manager.get_selected_backend(), Some(ActiveBackend::OpenClaw));
}

/// 测试 BackendConfig JSON 序列化/反序列化
#[test]
fn test_backend_config_json_roundtrip() {
    let config = BackendConfig::default();
    let json = serde_json::to_string(&config).unwrap();
    let deserialized: BackendConfig = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.selected, config.selected);
    assert_eq!(deserialized.openclaw.endpoint, config.openclaw.endpoint);
    assert_eq!(deserialized.hermes.endpoint, config.hermes.endpoint);
}

/// 测试 HermesClient 注册到 BackendManager
#[test]
fn test_hermes_client_registration() {
    let manager = BackendManager::new();

    // HermesClient 应该能够注册
    let result = manager.register_client(Box::new(HermesClient::new()));
    assert!(result.is_ok(), "HermesClient registration should succeed");
}

/// 测试 BackendManager 切换到 Hermes
#[test]
fn test_backend_manager_switch_to_hermes() {
    let manager = BackendManager::new();

    // 确保 switch 功能正常工作，不管初始状态是什么
    let initial = manager.current();

    // 切换到 hermes
    manager.switch("hermes").unwrap();
    assert_eq!(manager.current(), Some("hermes".to_string()));

    // 切换回 openclaw
    manager.switch("openclaw").unwrap();
    assert_eq!(manager.current(), Some("openclaw".to_string()));
}

/// 测试 BackendManager 健康检测
#[test]
fn test_backend_manager_check_health() {
    let manager = BackendManager::new();

    // 健康检测应该返回 BackendHealth 结构
    let health = manager.check_health();
    assert!(health.backend == "openclaw" || health.backend == "hermes");
}

/// 测试 HermesClient 可以通过 BackendManager 被调用
#[test]
fn test_backend_manager_with_hermes_client() {
    let manager = BackendManager::new();
    manager.register_client(Box::new(HermesClient::new())).unwrap();

    // 切换到 hermes
    manager.switch("hermes").unwrap();
    assert_eq!(manager.current(), Some("hermes".to_string()));
}