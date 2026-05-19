//! OpenClaw 后端客户端适配器
//!
//! 实现 BackendClient trait，使其符合多后端架构

use std::collections::HashMap;
use std::error::Error;
use tokio::sync::mpsc;

use super::client_trait::{BackendClient, Stream, StreamResponse, ToolEvent, ToolPhase};

// ============================================================================
// Stream 实现
// ============================================================================

/// OpenClaw 流式响应实现
pub struct OpenClawStream {
    receiver: mpsc::Receiver<Result<StreamResponse, Box<dyn Error + Send + Sync>>>,
}

impl OpenClawStream {
    pub fn new(receiver: mpsc::Receiver<Result<StreamResponse, Box<dyn Error + Send + Sync>>>) -> Self {
        Self { receiver }
    }
}

impl Stream for OpenClawStream {
    fn poll_next(&mut self) -> Option<Result<StreamResponse, Box<dyn Error + Send + Sync>>> {
        match self.receiver.blocking_recv() {
            Some(result) => Some(result),
            None => None,
        }
    }
}

// ============================================================================
// OpenClawClientAdapter 实现
// ============================================================================

/// OpenClaw 客户端适配器 - 实现 BackendClient trait
///
/// 包装 OpenClawClient，提供符合 trait 接口的实现
///
/// 注意：OpenClaw 使用持久连接和回调模式，而 BackendClient trait 使用 Stream 模式。
/// 这个适配器通过 channel 桥接两种模式。
///
/// 当前实现返回模拟响应，实际的 WebSocket 通信需要完整的异步客户端。
pub struct OpenClawClientAdapter {
    endpoint: String,
}

impl OpenClawClientAdapter {
    /// 创建新的 OpenClaw 客户端适配器
    pub fn new() -> Self {
        Self {
            endpoint: "ws://127.0.0.1:18789".to_string(),
        }
    }

    /// 通过 TCP 连接检测判断 OpenClaw Gateway 是否运行
    fn check_health_internal(&self) -> bool {
        std::net::TcpStream::connect("127.0.0.1:18789").is_ok()
    }
}

impl BackendClient for OpenClawClientAdapter {
    fn name(&self) -> &str {
        "openclaw"
    }

    fn check_health(&self) -> bool {
        self.check_health_internal()
    }

    fn send_message(&self, text: &str) -> Result<Box<dyn Stream>, Box<dyn Error + Send + Sync>> {
        // 创建返回用的 channel
        let (response_tx, response_rx) = mpsc::channel::<Result<StreamResponse, Box<dyn Error + Send + Sync>>>(100);

        let message_text = text.to_string();

        // Spawn async task to handle the message
        tokio::spawn(async move {
            // 模拟收到响应（实际实现需要通过 WebSocket 通信）
            // 这里发送一个模拟的最终响应
            let response_text = format!("[OpenClaw: {}]", &message_text[..message_text.len().min(30)]);
            let _ = response_tx.send(Ok(StreamResponse {
                text_delta: response_text,
                tool_event: None,
                is_final: true,
            })).await;
        });

        Ok(Box::new(OpenClawStream::new(response_rx)))
    }

    fn endpoint(&self) -> &str {
        &self.endpoint
    }

    fn auth_headers(&self) -> HashMap<String, String> {
        HashMap::new()
    }
}

impl Default for OpenClawClientAdapter {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 工具类型转换
// ============================================================================

/// 将 OpenClaw 的工具阶段转换为统一的 ToolPhase
pub fn convert_tool_phase(phase: &str) -> ToolPhase {
    match phase {
        "start" | "started" => ToolPhase::Started,
        "update" | "progress" => ToolPhase::Progress,
        "end" | "result" => ToolPhase::Result,
        "error" => ToolPhase::Error,
        _ => ToolPhase::Progress,
    }
}

/// 将 OpenClaw 工具事件转换为统一的 ToolEvent
pub fn convert_tool_event(
    name: String,
    phase: String,
    args: Option<serde_json::Value>,
    preview: Option<String>,
) -> ToolEvent {
    ToolEvent {
        name,
        phase: convert_tool_phase(&phase),
        args,
        preview,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_phase_conversion() {
        assert_eq!(convert_tool_phase("start"), ToolPhase::Started);
        assert_eq!(convert_tool_phase("update"), ToolPhase::Progress);
        assert_eq!(convert_tool_phase("end"), ToolPhase::Result);
        assert_eq!(convert_tool_phase("error"), ToolPhase::Error);
    }

    #[test]
    fn test_client_adapter_name() {
        let client = OpenClawClientAdapter::new();
        assert_eq!(client.name(), "openclaw");
    }

    #[test]
    fn test_client_adapter_endpoint() {
        let client = OpenClawClientAdapter::new();
        assert_eq!(client.endpoint(), "ws://127.0.0.1:18789");
    }

    #[test]
    fn test_client_adapter_auth_headers() {
        let client = OpenClawClientAdapter::new();
        assert!(client.auth_headers().is_empty());
    }
}
