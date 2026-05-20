//! OpenClaw Gateway WebSocket 客户端
//!
//! 实现与 OpenClaw Gateway 的 WebSocket 连接、认证和事件监听

use crate::openclaw::auth::DeviceIdentity;
use crate::openclaw::discovery::discover;
use crate::openclaw::protocol::{AgentEvent, Attachment, ChatEvent, ConnectChallenge, LifecycleEvent, SessionInfo, SessionMessage, ToolEvent, WsMessage};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{mpsc, RwLock};
use tokio::time::timeout;
use tokio_tungstenite::{connect_async, tungstenite::Message};
// Note: For device auth, use the values from the pairing registration
// The device was paired with clientId="cli" and clientMode="cli"
const CLIENT_ID: &str = "cli";
const CLIENT_MODE: &str = "cli";
const CLIENT_VERSION: &str = "1.0.0";

fn platform() -> &'static str {
    if cfg!(target_os = "windows") { "windows" }
    else if cfg!(target_os = "macos") { "darwin" }
    else { "linux" }
}
const ROLE: &str = "operator";
const SCOPES: &[&str] = &["operator.read", "operator.write"];

/// 连接状态回调 (true=已连接, false=已断开)
pub type ConnectionStatusCallback = Arc<dyn Fn(bool) + Send + Sync>;
/// Lifecycle 事件回调
pub type LifecycleCallback = Arc<dyn Fn(LifecycleEvent) + Send + Sync>;
/// Tool 事件回调 (tool_name, phase, args)
pub type ToolCallback = Arc<dyn Fn(String, String, serde_json::Value) + Send + Sync>;
/// Response 事件回调 (完整回答文本)
pub type ResponseCallback = Arc<dyn Fn(String) + Send + Sync>;

// 重连配置
const RECONNECT_INITIAL_DELAY_MS: u64 = 1000;
const RECONNECT_MAX_DELAY_MS: u64 = 30000;
const RECONNECT_MAX_RETRIES: u32 = 10;

/// OpenClaw 客户端状态
pub struct OpenClawClient {
    identity: Arc<DeviceIdentity>,
    gateway_token: Arc<RwLock<Option<String>>>,
    gateway_url: String,  // 发现得到的 URL，不再硬编码
    connection_status_callback: RwLock<Option<ConnectionStatusCallback>>,
    lifecycle_callback: RwLock<Option<LifecycleCallback>>,
    tool_callback: RwLock<Option<ToolCallback>>,
    response_callback: RwLock<Option<ResponseCallback>>,
    accumulated_response: RwLock<String>, // 累积的回答文本
    current_session: RwLock<Option<SessionInfo>>,
    msg_tx: mpsc::Sender<String>,
}

impl OpenClawClient {
    /// 创建新客户端（不做同步发现）
    ///
    /// 使用默认 URL 快速创建，实际的端口发现和连接
    /// 在 run() 方法中异步执行，不阻塞 UI 启动。
    pub fn new() -> Result<(Self, mpsc::Receiver<String>), String> {
        Self::from_url("ws://127.0.0.1:18789".to_string())
    }

    /// 使用指定的 Gateway URL 创建客户端
    pub fn from_url(gateway_url: String) -> Result<(Self, mpsc::Receiver<String>), String> {
        let identity = DeviceIdentity::load(None)?;
        let gateway_token = DeviceIdentity::load_gateway_token(&identity.device_id)?;
        let (msg_tx, msg_rx) = mpsc::channel::<String>(32);

        let client = Self {
            identity: Arc::new(identity),
            gateway_token: Arc::new(RwLock::new(Some(gateway_token))),
            gateway_url,
            connection_status_callback: RwLock::new(None),
            lifecycle_callback: RwLock::new(None),
            tool_callback: RwLock::new(None),
            response_callback: RwLock::new(None),
            accumulated_response: RwLock::new(String::new()),
            current_session: RwLock::new(None),
            msg_tx,
        };

        Ok((client, msg_rx))
    }

    /// 设置连接状态回调
    pub fn set_connection_status_callback(&self, callback: ConnectionStatusCallback) {
        if let Ok(mut guard) = self.connection_status_callback.try_write() {
            *guard = Some(callback);
        }
    }

    /// 设置 lifecycle 回调
    pub fn set_lifecycle_callback(&self, callback: LifecycleCallback) {
        if let Ok(mut guard) = self.lifecycle_callback.try_write() {
            *guard = Some(callback);
        }
    }

    /// 设置 tool 回调
    pub fn set_tool_callback(&self, callback: ToolCallback) {
        if let Ok(mut guard) = self.tool_callback.try_write() {
            *guard = Some(callback);
        }
    }

    /// 设置 response 回调
    pub fn set_response_callback(&self, callback: ResponseCallback) {
        if let Ok(mut guard) = self.response_callback.try_write() {
            *guard = Some(callback);
        }
    }

    /// 清空累积的回答文本（每次新会话开始时调用）
    pub fn clear_response(&self) {
        if let Ok(mut guard) = self.accumulated_response.try_write() {
            guard.clear();
        }
    }

    /// 发送消息到 OpenClaw session（支持附件）
    pub async fn sessions_send(&self, message: &str, attachments: Vec<Attachment>) -> Result<(), String> {
        let session = self.current_session.read().await;
        let session_info = session.as_ref().ok_or_else(|| "No active session".to_string())?;

        let mut params = serde_json::json!({
            "key": session_info.key,
            "message": message
        });

        // 如果有附件，添加到 params
        if !attachments.is_empty() {
            params["attachments"] = serde_json::to_value(&attachments)
                .map_err(|e| format!("Failed to serialize attachments: {}", e))?;
        }

        let req = WsMessage::Request {
            id: format!("msg-{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis()),
            method: "sessions.send".to_string(),
            params,
        };

        let json_str = serde_json::to_string(&req)
            .map_err(|e| format!("Failed to serialize: {}", e))?;

        self.msg_tx.send(json_str.clone()).await
            .map_err(|e| format!("Failed to send message: {}", e))?;

        log::debug!("[OpenClaw] sessions.send JSON: {:.500}", json_str);
        log::info!("[OpenClaw] sessions.send: {} (attachments: {})", message, attachments.len());
        Ok(())
    }

    /// 启动客户端（异步）
    pub async fn run(&self, mut msg_rx: mpsc::Receiver<String>) -> Result<(), String> {
        let mut retry_count = 0u32;
        let mut delay_ms = RECONNECT_INITIAL_DELAY_MS;

        loop {
            log::info!("[OpenClaw] Connecting (attempt {})", retry_count + 1);

            match self.connect_and_run(&mut msg_rx).await {
                Ok(()) => {
                    log::info!("[OpenClaw] Connection closed normally");
                    break;
                }
                Err(e) => {
                    retry_count += 1;

                    if retry_count > RECONNECT_MAX_RETRIES {
                        log::error!("[OpenClaw] Max retries ({}) exceeded, giving up: {}", RECONNECT_MAX_RETRIES, e);
                        // 通知断开
                        self.notify_connection_status(false).await;
                        return Err(format!("Max retries exceeded: {}", e));
                    }

                    log::warn!("[OpenClaw] Connection error: {}, retrying in {}ms (attempt {}/{})",
                        e, delay_ms, retry_count, RECONNECT_MAX_RETRIES);

                    // 通知断开
                    self.notify_connection_status(false).await;

                    tokio::time::sleep(Duration::from_millis(delay_ms)).await;

                    // 指数退避
                    delay_ms = (delay_ms * 2).min(RECONNECT_MAX_DELAY_MS);
                }
            }
        }

        Ok(())
    }

    /// 连接到 Gateway 并运行主事件循环
    async fn connect_and_run(&self, msg_rx: &mut mpsc::Receiver<String>) -> Result<(), String> {
        // 重连时重新探测端口（因为用户可能切换了端口）
        let url = discover()
            .map(|r| r.url)
            .ok_or_else(|| "OpenClaw Gateway not found on ports 18789 or 19001".to_string())?;

        log::info!("[OpenClaw] Connecting to {}...", url);

        let (ws_stream, _) = connect_async(&url)
            .await
            .map_err(|e| format!("Connect failed: {}", e))?;

        log::info!("[OpenClaw] Connected, awaiting challenge...");
        // 通知已连接
        self.notify_connection_status(true).await;

        let (mut write, mut read) = ws_stream.split();

        // 等待并处理 connect.challenge
        let (nonce, challenge_ts) = self.wait_challenge(&mut read).await?;

        log::info!("[OpenClaw] Received challenge, nonce: {}, ts: {}", &nonce[..nonce.len().min(8)], challenge_ts);

        // 发送认证请求
        let token = self.gateway_token.read().await.clone()
            .ok_or_else(|| "No gateway token".to_string())?;

        let connect_req = self.build_connect_request(&nonce, challenge_ts, &token);
        write.send(Message::Text(connect_req.into()))
            .await
            .map_err(|e| format!("Send failed: {}", e))?;

        // 等待认证结果
        self.wait_hello_ok(&mut read).await?;

        log::info!("[OpenClaw] Authenticated successfully!");

        // 设置默认 session（从 mainSessionKey）
        {
            let mut current = self.current_session.write().await;
            *current = Some(SessionInfo {
                key: "agent:main:main".to_string(),
                session_id: Some("agent:main:main".to_string()),
                agent_id: Some("main".to_string()),
                updated_at_ms: None,
                flags: vec![],
            });
            log::info!("[OpenClaw] Set default session: agent:main:main");
        }

        // 订阅 sessions
        let subscribe_req = WsMessage::sessions_subscribe("sub-1");
        write.send(Message::Text(serde_json::to_string(&subscribe_req).unwrap().into()))
            .await
            .map_err(|e| format!("Subscribe failed: {}", e))?;

        log::info!("[OpenClaw] Subscribed to sessions");

        // 订阅 session messages（包含工具调用信息）
        let session_key = {
            let session = self.current_session.read().await;
            session.as_ref().map(|s| s.key.clone()).unwrap_or_default()
        };
        let msg_subscribe_req = WsMessage::sessions_messages_subscribe("sub-2", &session_key);
        write.send(Message::Text(serde_json::to_string(&msg_subscribe_req).unwrap().into()))
            .await
            .map_err(|e| format!("Subscribe messages failed: {}", e))?;

        log::info!("[OpenClaw] Subscribed to session messages for {}", session_key);

        // 主事件循环
        loop {
            tokio::select! {
                // 处理接收到的消息
                msg = read.next() => {
                    match msg {
                        Some(Ok(Message::Text(text))) => {
                            // 打印所有收到的消息用于调试
                            log::info!("[OpenClaw] RAW WS MSG: {:.500}", text);
                            if let Err(e) = self.handle_message(&text).await {
                                log::error!("[OpenClaw] Handle message error: {}", e);
                            }
                        }
                        Some(Ok(Message::Close(_))) => {
                            log::warn!("[OpenClaw] Connection closed");
                            break;
                        }
                        Some(Err(e)) => {
                            log::error!("[OpenClaw] WebSocket error: {}", e);
                            break;
                        }
                        None => break,
                        _ => {}
                    }
                }
                // 处理待发送的消息
                Some(text) = msg_rx.recv() => {
                    write.send(Message::Text(text.into())).await
                        .map_err(|e| format!("Send failed: {}", e))?;
                }
            }
        }

        Ok(())
    }

    /// 通知连接状态变化
    async fn notify_connection_status(&self, connected: bool) {
        let callback = self.connection_status_callback.read().await;
        if let Some(cb) = callback.as_ref() {
            cb(connected);
        }
    }

    /// 等待 connect.challenge
    async fn wait_challenge(
        &self,
        read: &mut futures_util::stream::SplitStream<
            tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
        >,
    ) -> Result<(String, i64), String> {
        // 等待消息
        let opt = timeout(Duration::from_secs(10), read.next())
            .await
            .map_err(|_| "Timeout waiting for challenge".to_string())?;

        let result = opt.ok_or_else(|| "Connection closed".to_string())?;

        let msg = result.map_err(|e| format!("WS error: {}", e))?;

        let text = msg.into_text()
            .map_err(|e| format!("Not text: {}", e))?;

        let msg: WsMessage = serde_json::from_str(&text)
            .map_err(|e| format!("Invalid JSON: {}", e))?;

        log::info!("[OpenClaw] Challenge message raw: {}", text);

        match msg {
            WsMessage::Event { event, payload, .. } => {
                if event == "connect.challenge" {
                    let challenge: ConnectChallenge = serde_json::from_value(payload)
                        .map_err(|e| format!("Invalid challenge: {}", e))?;
                    log::info!("[OpenClaw] Challenge nonce received: {}", challenge.nonce);
                    // Return both nonce and timestamp
                    let ts = challenge.timestamp.unwrap_or_else(|| {
                        std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_millis() as i64
                    });
                    Ok((challenge.nonce, ts))
                } else {
                    Err(format!("Unexpected event: {}", event))
                }
            }
            _ => Err("Expected challenge event".to_string()),
        }
    }

    /// 构建 connect 请求
    fn build_connect_request(&self, nonce: &str, _challenge_ts: i64, token: &str) -> String {
        // Use current time like the JS implementation (Date.now())
        let signed_at_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        // Use v2 payload format (same as official CLI)
        let payload = self.identity.build_v2_payload(
            CLIENT_ID,
            CLIENT_MODE,
            ROLE,
            &SCOPES.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
            token,
            nonce,
            signed_at_ms,
        );

        let signature = self.identity.sign(payload.as_bytes());
        let signature_b64 = URL_SAFE_NO_PAD.encode(&signature);
        let public_key_b64 = self.identity.public_key_b64();
        log::info!("[Auth] Signature (base64url): {}", signature_b64);

        let params = serde_json::json!({
            "minProtocol": 3,
            "maxProtocol": 3,
            "client": {
                "id": CLIENT_ID,
                "version": CLIENT_VERSION,
                "platform": platform(),
                "mode": CLIENT_MODE
            },
            "role": ROLE,
            "scopes": SCOPES,
            "caps": [],
            "commands": [],
            "permissions": {},
            "auth": { "token": token },
            "locale": "en-US",
            "userAgent": format!("{}/{}", CLIENT_ID, CLIENT_VERSION),
            "device": {
                "id": self.identity.device_id,
                "publicKey": public_key_b64,
                "signature": signature_b64,
                "signedAt": signed_at_ms,
                "nonce": nonce
            }
        });

        let req = WsMessage::Request {
            id: "conn-1".to_string(),
            method: "connect".to_string(),
            params,
        };

        let json_str = serde_json::to_string(&req).unwrap();
        log::info!("[Auth] Full connect request JSON:\n{}", json_str);
        json_str
    }

    /// 等待 hello-ok
    async fn wait_hello_ok(
        &self,
        read: &mut futures_util::stream::SplitStream<
            tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
        >,
    ) -> Result<(), String> {
        // 等待消息
        let opt = timeout(Duration::from_secs(10), read.next())
            .await
            .map_err(|_| "Timeout waiting for hello-ok".to_string())?;

        let result = opt.ok_or_else(|| "Connection closed".to_string())?;

        let msg = result.map_err(|e| format!("WS error: {}", e))?;

        let text = msg.into_text()
            .map_err(|e| format!("Not text: {}", e))?;

        log::info!("[OpenClaw] Auth response: {}", &text);

        let msg: WsMessage = serde_json::from_str(&text)
            .map_err(|e| format!("Invalid JSON: {}", e))?;

        match msg {
            WsMessage::Response { ok, error, .. } => {
                if ok {
                    Ok(())
                } else {
                    Err(format!("Auth failed: {:?}", error))
                }
            }
            _ => Err("Expected response".to_string()),
        }
    }

    /// 处理收到的消息
    async fn handle_message(&self, text: &str) -> Result<(), String> {
        let msg: WsMessage = serde_json::from_str(text)
            .map_err(|e| format!("Invalid JSON: {}", e))?;

        match msg {
            WsMessage::Event { event, payload, .. } => {
                log::debug!("[OpenClaw] Event: {} payload={}", event, payload);
                if event == "session.message" {
                    if let Ok(session_msg) = serde_json::from_value::<SessionMessage>(payload) {
                        self.handle_session_message(session_msg).await?;
                    }
                } else if event == "session.tool" {
                    // 处理 session.tool 事件（工具调用）
                    log::info!("[OpenClaw] Session tool event: {:?}", payload);
                    if let Ok(tool_event) = serde_json::from_value::<ToolEvent>(payload.clone()) {
                        let tool_name = tool_event.data.name.clone().unwrap_or_default();
                        let phase = tool_event.data.phase.clone();
                        let args = tool_event.data.args.clone().unwrap_or(serde_json::Value::Null);

                        log::info!("[OpenClaw] Tool: {} (phase: {})", tool_name, phase);

                        // 调用 tool 回调
                        let callback = self.tool_callback.read().await;
                        if let Some(cb) = callback.as_ref() {
                            cb(tool_name, phase, args);
                        }
                    }
                } else if event == "agent" {
                    // 处理 agent 事件 (包含 lifecycle 和 assistant)
                    if let Ok(agent_event) = serde_json::from_value::<AgentEvent>(payload.clone()) {
                        if agent_event.stream == "lifecycle" {
                            if let Some(phase) = &agent_event.data.phase {
                                log::info!("[OpenClaw] Agent lifecycle: {}", phase);

                                // 会话开始时清空累积的回答
                                if phase == "start" {
                                    self.clear_response();
                                } else if phase == "end" {
                                    // 发送最终累积的响应（只在结束时发送，避免多次音效）
                                    let callback = self.response_callback.read().await;
                                    if let Some(cb) = callback.as_ref() {
                                        let acc = self.accumulated_response.read().await;
                                        if !acc.is_empty() {
                                            cb(acc.clone());
                                        }
                                    }
                                    self.clear_response();
                                }

                                // 调用 lifecycle 回调
                                let callback = self.lifecycle_callback.read().await;
                                if let Some(cb) = callback.as_ref() {
                                    let lifecycle = LifecycleEvent {
                                        run_id: Some(agent_event.run_id.clone()),
                                        session_key: Some(agent_event.session_key.clone()),
                                        session_id: None,
                                        phase: phase.clone(),
                                        error: None,
                                        summary: None,
                                    };
                                    cb(lifecycle);
                                }
                            }
                        } else if agent_event.stream == "assistant" {
                            // 处理 assistant 事件（回答文本）
                            if let Some(delta) = &agent_event.data.delta {
                                if !delta.is_empty() {
                                    log::info!("[OpenClaw] Assistant delta: {:.100}", delta);
                                    // 只累积 delta，不发送回调（避免多次音效）
                                    let mut acc = self.accumulated_response.write().await;
                                    acc.push_str(delta);
                                }
                            }
                        }
                    }
                } else if event == "sessions.changed" {
                    // 处理 session 列表变化
                    log::info!("[OpenClaw] Sessions changed event: {:?}", payload);
                    if let Ok(sessions) = serde_json::from_value::<Vec<SessionInfo>>(payload) {
                        log::info!("[OpenClaw] Sessions changed: {} sessions", sessions.len());
                        // 选择第一个可用的 session
                        if let Some(session) = sessions.first() {
                            let mut current = self.current_session.write().await;
                            *current = Some(session.clone());
                            log::info!("[OpenClaw] Selected session: key={}", session.key);
                        }
                    }
                } else if event == "chat" {
                    // 处理 chat 事件（如 /status、/stop 等命令的返回内容）
                    if let Ok(chat_event) = serde_json::from_value::<ChatEvent>(payload.clone()) {
                        log::info!("[OpenClaw] Chat event: state={}", chat_event.state);
                        // chat 事件的 final state 表示命令响应结束，触发 lifecycle end 回调
                        // 注意：不再调用 response_callback，因为正常对话通过 agent 事件处理
                        if chat_event.state == "final" {
                            let callback = self.lifecycle_callback.read().await;
                            if let Some(cb) = callback.as_ref() {
                                let lifecycle = LifecycleEvent {
                                    run_id: Some(chat_event.run_id.clone()),
                                    session_key: Some(chat_event.session_key.clone()),
                                    session_id: None,
                                    phase: "end".to_string(),
                                    error: None,
                                    summary: None,
                                };
                                cb(lifecycle);
                            }
                        }
                    }
                }
            }
            WsMessage::Response { id, ok, error, .. } => {
                log::debug!("[OpenClaw] Response {}: ok={}", id, ok);
                if !ok {
                    log::error!("[OpenClaw] Response error: {:?}", error);
                }
            }
            _ => {}
        }

        Ok(())
    }

    /// 处理 session 消息
    async fn handle_session_message(&self, msg: SessionMessage) -> Result<(), String> {
        // 检查是否是 lifecycle 事件
        if let Some(stream) = &msg.stream {
            if stream == "lifecycle" {
                if let Some(phase) = &msg.phase {
                    let lifecycle = LifecycleEvent {
                        run_id: msg.run_id,
                        session_key: Some(msg.session_key.clone()),
                        session_id: Some(msg.session_id.clone()),
                        phase: phase.clone(),
                        error: msg.error,
                        summary: None,
                    };

                    log::info!("[OpenClaw] Lifecycle: {} for session {}", phase, msg.session_key);

                    // 调用 lifecycle 回调
                    let callback = self.lifecycle_callback.read().await;
                    if let Some(cb) = callback.as_ref() {
                        cb(lifecycle);
                    }
                }
            } else if stream == "tool" {
                // 处理 tool 事件
                self.handle_tool_event(&msg).await?;
            } else if let Some(text) = &msg.text {
                // 处理文本响应（如 /status、/stop 等命令的返回内容）
                if !text.is_empty() {
                    log::info!("[OpenClaw] Text response: {:.200}", text);
                    let callback = self.response_callback.read().await;
                    if let Some(cb) = callback.as_ref() {
                        cb(text.clone());
                    }
                }
            }
        } else if let Some(text) = &msg.text {
            // 没有 stream 字段但有 text 内容的情况
            if !text.is_empty() {
                log::info!("[OpenClaw] Text response (no stream): {:.200}", text);
                let callback = self.response_callback.read().await;
                if let Some(cb) = callback.as_ref() {
                    cb(text.clone());
                }
            }
        }

        Ok(())
    }

    /// 处理 tool 事件
    async fn handle_tool_event(&self, msg: &SessionMessage) -> Result<(), String> {
        // 解析 tool event 数据
        if let Some(payload) = &msg.payload {
            if let Ok(tool_event) = serde_json::from_value::<ToolEvent>(payload.clone()) {
                let tool_name = tool_event.data.name.clone().unwrap_or_default();
                let phase = tool_event.data.phase.clone();
                let args = tool_event.data.args.clone().unwrap_or(serde_json::Value::Null);

                log::info!("[OpenClaw] Tool event: {} (phase: {}) args: {:?}", tool_name, phase, args);

                // 调用 tool 回调
                let callback = self.tool_callback.read().await;
                if let Some(cb) = callback.as_ref() {
                    cb(tool_name, phase, args);
                }
            }
        }

        Ok(())
    }
}