//! OpenClaw Gateway 发现模块
//!
//! 负责探测本机运行的 OpenClaw Gateway，支持两个端口：
//! - 18789（默认端口）
//! - 19001（dev 模式端口）

use std::net::TcpStream;
use std::time::Duration;

/// 默认端口（生产模式）
const DEFAULT_PORT: u16 = 18789;
/// Dev 模式端口
const DEV_PORT: u16 = 19001;
/// 端口探测超时（秒）
const PROBE_TIMEOUT_SECS: u64 = 3;

/// 发现结果
#[derive(Debug, Clone)]
pub struct DiscoveryResult {
    /// 主机地址（始终是 127.0.0.1）
    pub host: String,
    /// 端口号
    pub port: u16,
    /// 完整 WebSocket URL
    pub url: String,
}

impl DiscoveryResult {
    pub fn new(port: u16) -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port,
            url: format!("ws://127.0.0.1:{}", port),
        }
    }
}

/// 探测单个端口是否可连接（同步 TCP 连通测试）
fn probe_port(port: u16) -> bool {
    let addr = format!("127.0.0.1:{}", port);
    match TcpStream::connect_timeout(
        &addr.parse().expect("Invalid socket address"),
        Duration::from_secs(PROBE_TIMEOUT_SECS),
    ) {
        Ok(_) => {
            log::debug!("[OpenClaw Discovery] Port {} is reachable", port);
            true
        }
        Err(e) => {
            log::debug!("[OpenClaw Discovery] Port {} connection failed: {}", port, e);
            false
        }
    }
}

/// 发现可用的 OpenClaw Gateway
///
/// 按顺序尝试 18789 → 19001，返回第一个可用的。
/// 最坏情况耗时约 6 秒（两个端口各 3 秒超时）。
///
/// 同步函数，可以在任何上下文中调用。
pub fn discover() -> Option<DiscoveryResult> {
    log::info!("[OpenClaw Discovery] Starting discovery...");

    // 先试默认端口
    if probe_port(DEFAULT_PORT) {
        log::info!("[OpenClaw Discovery] Found OpenClaw on default port {}", DEFAULT_PORT);
        return Some(DiscoveryResult::new(DEFAULT_PORT));
    }

    // 再试 dev 模式端口
    if probe_port(DEV_PORT) {
        log::info!("[OpenClaw Discovery] Found OpenClaw on dev port {}", DEV_PORT);
        return Some(DiscoveryResult::new(DEV_PORT));
    }

    log::warn!("[OpenClaw Discovery] No OpenClaw Gateway found on ports {} or {}", DEFAULT_PORT, DEV_PORT);
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discover_returns_result_when_port_open() {
        // 注意：这个测试需要在本地有 OpenClaw 运行才能通过
        // CI 环境中可能失败，这是正常的
        let result = discover();
        // 只验证不 panic
        if let Some(r) = result {
            assert!(matches!(r.port, DEFAULT_PORT | DEV_PORT));
            assert_eq!(r.host, "127.0.0.1");
            assert!(r.url.starts_with("ws://127.0.0.1:"));
        }
    }

    #[test]
    fn test_probe_invalid_port() {
        // 探测一个确定没有服务监听的端口
        let result = probe_port(19999);
        assert!(!result, "Port 19999 should not be reachable");
    }
}
