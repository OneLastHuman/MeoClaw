//! Hermes 后端客户端适配器
//!
//! 实现 BackendClient trait，使其符合多后端架构
//!
//! Hermes 使用 HTTP + SSE (Server-Sent Events) 模式进行通信

use std::collections::HashMap;
use std::error::Error;

use tokio::sync::mpsc;
use reqwest::Client;

use super::client_trait::{BackendClient, Stream, StreamResponse};
use super::hermes_discovery::{HermesDiscovery, HermesConfig, DiscoverySource};

// ============================================================================
// HermesStream 实现
// ============================================================================

/// Hermes 流式响应包装器
///
/// 将 SSE (Server-Sent Events) 响应桥接为 Stream trait
///
/// SSE 格式示例:
///   data: {"choices":[{"delta":{"content":"Hello"}}]}
///   data: {"choices":[{"delta":{"content":" world"}}]}
///   data: [DONE]
pub struct HermesStream {
    receiver: mpsc::Receiver<Result<StreamResponse, Box<dyn Error + Send + Sync>>>,
}

impl HermesStream {
    /// 创建新的 HermesStream
    pub fn new(
        receiver: mpsc::Receiver<Result<StreamResponse, Box<dyn Error + Send + Sync>>>,
    ) -> Self {
        Self { receiver }
    }
}

impl Stream for HermesStream {
    fn poll_next(
        &mut self,
    ) -> Option<Result<StreamResponse, Box<dyn Error + Send + Sync>>> {
        match self.receiver.blocking_recv() {
            Some(result) => Some(result),
            None => None,
        }
    }
}

// ============================================================================
// HermesClient 实现
// ============================================================================

/// Hermes 客户端适配器 - 实现 BackendClient trait
///
/// 包装 Hermes HTTP API，提供符合 trait 接口的实现
///
/// Hermes 使用 HTTP + SSE 模式，比 OpenClaw 的 WebSocket 回调模式更简单
pub struct HermesClient {
    endpoint: String,
    api_key: Option<String>,
    client: Client,
}

impl HermesClient {
    /// 从发现配置创建客户端
    pub fn from_config(config: &HermesConfig) -> Self {
        log::info!("[HermesClient] from_config() - endpoint: {}, api_key present: {}",
            config.endpoint, config.api_key.is_some());

        Self {
            endpoint: config.endpoint.clone(),
            api_key: config.api_key.clone(),
            client: reqwest::Client::new(),
        }
    }

    /// 创建新的 Hermes 客户端适配器（兼容旧接口，默认 localhost）
    pub fn new() -> Self {
        let config = HermesDiscovery::discover();
        let endpoint = config
            .as_ref()
            .map(|c| c.endpoint.clone())
            .unwrap_or_else(|| "http://127.0.0.1:8642".to_string());

        let api_key = config.and_then(|c| c.api_key);

        log::info!("[HermesClient] new() - endpoint: {}, api_key present: {}", endpoint, api_key.is_some());

        Self {
            endpoint,
            api_key,
            client: reqwest::Client::new(),
        }
    }

    /// Hermes 健康检测实现
    /// HTTP GET /health，如果失败则降级为 TCP 连接检测
    fn check_health_internal(&self) -> bool {
        // 尝试 HTTP GET 健康检测 (使用 blocking client 避免 async 上下文问题)
        let health_url = format!("{}/health", self.endpoint);
        if let Ok(response) = reqwest::blocking::get(&health_url) {
            if response.status().is_success() {
                return true;
            }
        }

        // 如果 HTTP 失败，尝试 TCP 连接检测作为降级方案
        // 从 endpoint 中解析端口
        let port = self.endpoint.split(':').last().unwrap_or("8642");
        if std::net::TcpStream::connect(&format!("127.0.0.1:{}", port)).is_ok() {
            return true;
        }

        false
    }

    /// 发送消息并返回流式响应
    ///
    /// 使用 HTTP POST 发送消息到 Hermes API，处理 SSE 格式响应
    fn send_message_internal(
        &self,
        text: &str,
    ) -> Result<Box<dyn Stream>, Box<dyn Error + Send + Sync>> {
        log::info!("[HermesClient] send_message_internal called with: {}", text);
        let (response_tx, response_rx) = mpsc::channel(100);
        let message_text = text.to_string();
        let client = self.client.clone();
        let api_key = self.api_key.clone();
        let endpoint = self.endpoint.clone();

        log::info!("[HermesClient] api_key present: {}", api_key.is_some());

        // 异步处理 HTTP/SSE 请求
        tokio::spawn(async move {
            log::info!("[HermesClient] Async task started for: {}", message_text);
            let request_body = serde_json::json!({
                "model": "assistant",
                "messages": [{"role": "user", "content": &message_text}],
                "stream": true
            });

            let mut request = client
                .post(format!("{}/v1/chat/completions", endpoint))
                .json(&request_body);

            // 添加 Authorization header（如果 API key 存在）
            if let Some(key) = &api_key {
                log::info!("[HermesClient] Adding Authorization header");
                request = request.header("Authorization", format!("Bearer {}", key));
            }

            let result = request.send().await;
            log::info!("[HermesClient] HTTP request result: {:?}", result);

            match result {
                Ok(resp) => {
                    log::info!("[HermesClient] Response status: {}", resp.status());
                    // SSE 响应是一个字节流
                    let mut stream = resp.bytes_stream();

                    while let Some(chunk_result) = futures_util::StreamExt::next(&mut stream).await {
                        match chunk_result {
                            Ok(bytes) => {
                                let bytes_vec: Vec<u8> = bytes.to_vec();
                                log::info!("[HermesClient] Received {} bytes from SSE stream", bytes_vec.len());
                                if let Ok(text) = String::from_utf8(bytes_vec) {
                                    log::info!("[HermesClient] SSE chunk text: {:?}", text);
                                    // 解析 SSE 格式: data: {...}
                                    Self::parse_sse_events(&text, &response_tx).await;
                                }
                            }
                            Err(e) => {
                                let _ = response_tx
                                    .send(Err(Box::new(e) as Box<dyn Error + Send + Sync>))
                                    .await;
                                return;
                            }
                        }
                    }
                }
                Err(e) => {
                    let _ = response_tx
                        .send(Err(Box::new(e) as Box<dyn Error + Send + Sync>))
                        .await;
                }
            }
        });

        Ok(Box::new(HermesStream::new(response_rx)))
    }

    /// 解析 SSE 事件数据
    ///
    /// SSE 格式: `data: {"choices":[{"delta":{"content":"..."}}]}`
    /// 结束标记: `data: [DONE]`
    async fn parse_sse_events(
        text: &str,
        response_tx: &mpsc::Sender<Result<StreamResponse, Box<dyn Error + Send + Sync>>>,
    ) {
        for line in text.lines() {
            let line = line.trim();

            // 跳过空行
            if line.is_empty() {
                continue;
            }

            // 检查是否是 data: 前缀
            if let Some(data) = line.strip_prefix("data: ") {
                // 检查是否是结束标记
                if data == "[DONE]" {
                    let _ = response_tx
                        .send(Ok(StreamResponse {
                            text_delta: String::new(),
                            tool_event: None,
                            is_final: true,
                        }))
                        .await;
                    return;
                }

                // 解析 JSON 数据
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(data) {
                    // 检查是否是最终响应（finish_reason === "stop"）
                    let is_final = json["choices"][0]["finish_reason"]
                        .as_str()
                        .map(|s| s == "stop")
                        .unwrap_or(false);

                    // 提取 delta.content
                    if let Some(content) = json["choices"][0]["delta"]["content"].as_str() {
                        let _ = response_tx
                            .send(Ok(StreamResponse {
                                text_delta: content.to_string(),
                                tool_event: None,
                                is_final,
                            }))
                            .await;
                    } else if is_final {
                        // 如果没有 content 但 is_final 为 true，发送一个空 delta 表示结束
                        let _ = response_tx
                            .send(Ok(StreamResponse {
                                text_delta: String::new(),
                                tool_event: None,
                                is_final: true,
                            }))
                            .await;
                    }
                }
            }
        }
    }
}

impl BackendClient for HermesClient {
    fn name(&self) -> &str {
        "hermes"
    }

    fn check_health(&self) -> bool {
        self.check_health_internal()
    }

    fn send_message(&self, text: &str) -> Result<Box<dyn Stream>, Box<dyn Error + Send + Sync>> {
        self.send_message_internal(text)
    }

    fn endpoint(&self) -> &str {
        &self.endpoint
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

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::mpsc;

    #[test]
    fn test_hermes_client_name() {
        let client = HermesClient::new();
        assert_eq!(client.name(), "hermes");
    }

    #[test]
    fn test_hermes_client_endpoint() {
        let client = HermesClient::new();
        assert_eq!(client.endpoint(), "http://127.0.0.1:8642");
    }

    #[test]
    fn test_hermes_client_auth_headers() {
        let client = HermesClient::new();
        assert!(client.auth_headers().is_empty());
    }

    #[test]
    fn test_hermes_client_check_health_returns_true_when_available() {
        let client = HermesClient::new();
        // When Hermes API server is running, health check should return true
        // Note: This test requires Hermes API server to be running with API_SERVER_ENABLED=true
        assert!(client.check_health(), "Hermes API server should be available at discovered endpoint");
    }

    #[test]
    fn test_hermes_client_health_check_uses_correct_endpoint() {
        let client = HermesClient::new();
        // Verify the endpoint is correctly set
        assert_eq!(client.endpoint(), "http://127.0.0.1:8642");
        // Health check endpoint should be {base}/health
        let health_url = format!("{}/health", client.endpoint());
        assert_eq!(health_url, "http://127.0.0.1:8642/health");
    }

    #[tokio::test]
    async fn test_hermes_client_send_message_returns_stream() {
        // Use from_config to avoid network discovery in test context
        let config = HermesConfig::new("127.0.0.1", 8642, DiscoverySource::PortProbe);
        let client = HermesClient::from_config(&config);
        // send_message should return Ok with a HermesStream
        // The actual HTTP request is spawned async, so it won't fail immediately
        let result = client.send_message("test message");
        assert!(result.is_ok());
    }

    #[test]
    fn test_hermes_stream_implements_stream_trait() {
        // Create a channel
        let (tx, rx) = mpsc::channel::<Result<StreamResponse, Box<dyn Error + Send + Sync>>>(10);

        // Create HermesStream with the receiver
        let mut stream = HermesStream::new(rx);

        // Send a test response
        let response = Ok(StreamResponse {
            text_delta: "Hello".to_string(),
            tool_event: None,
            is_final: false,
        });

        // Use try_send which doesn't block
        let _ = tx.try_send(response);

        // poll_next should return the response
        let result = stream.poll_next();
        assert!(result.is_some());
        let result = result.unwrap();
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.text_delta, "Hello");
        assert!(!response.is_final);
    }

    #[test]
    fn test_hermes_stream_returns_none_when_channel_closed() {
        // Create a channel and immediately drop the sender
        let (_, rx) = mpsc::channel::<Result<StreamResponse, Box<dyn Error + Send + Sync>>>(10);
        let mut stream = HermesStream::new(rx);

        // poll_next should return None after channel closes
        let result = stream.poll_next();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_parse_sse_events_single_message() {
        let (tx, mut rx) = mpsc::channel::<Result<StreamResponse, Box<dyn Error + Send + Sync>>>(10);

        let sse_data = r#"data: {"choices":[{"delta":{"content":"Hello"}}]}"#;
        HermesClient::parse_sse_events(sse_data, &tx).await;

        // Receive and verify
        let result = rx.recv().await.unwrap();
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.text_delta, "Hello");
        assert!(!response.is_final);
    }

    #[tokio::test]
    async fn test_parse_sse_events_done_marker() {
        let (tx, mut rx) = mpsc::channel::<Result<StreamResponse, Box<dyn Error + Send + Sync>>>(10);

        let sse_data = "data: [DONE]";
        HermesClient::parse_sse_events(sse_data, &tx).await;

        // Receive and verify
        let result = rx.recv().await.unwrap();
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.text_delta.is_empty());
        assert!(response.is_final);
    }

    #[tokio::test]
    async fn test_parse_sse_events_multiple_lines() {
        let (tx, mut rx) = mpsc::channel::<Result<StreamResponse, Box<dyn Error + Send + Sync>>>(10);

        let sse_data = r#"data: {"choices":[{"delta":{"content":"Hello"}}]}
data: {"choices":[{"delta":{"content":" world"}}]}
data: [DONE]"#;
        HermesClient::parse_sse_events(sse_data, &tx).await;

        // Receive first message
        let result = rx.recv().await.unwrap();
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.text_delta, "Hello");
        assert!(!response.is_final);

        // Receive second message
        let result = rx.recv().await.unwrap();
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.text_delta, " world");
        assert!(!response.is_final);

        // Receive done marker
        let result = rx.recv().await.unwrap();
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.is_final);
    }

    #[tokio::test]
    async fn test_parse_sse_events_skips_empty_lines() {
        let (tx, mut rx) = mpsc::channel::<Result<StreamResponse, Box<dyn Error + Send + Sync>>>(10);

        let sse_data = "data: {\"choices\":[{\"delta\":{\"content\":\"A\"}}]}\n\ndata: {\"choices\":[{\"delta\":{\"content\":\"B\"}}]}";
        HermesClient::parse_sse_events(sse_data, &tx).await;

        // Should receive exactly 2 messages, not affected by empty lines
        let result1 = rx.recv().await.unwrap().unwrap();
        let result2 = rx.recv().await.unwrap().unwrap();
        assert_eq!(result1.text_delta, "A");
        assert_eq!(result2.text_delta, "B");
    }
}