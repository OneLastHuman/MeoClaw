//! OpenClaw Gateway 协议消息定义

use serde::{Deserialize, Serialize};

// ============================================================================
// 基础消息框架
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WsMessage {
    #[serde(rename = "req")]
    Request {
        id: String,
        method: String,
        params: serde_json::Value,
    },
    #[serde(rename = "res")]
    Response {
        id: String,
        ok: bool,
        #[serde(default)]
        payload: serde_json::Value,
        #[serde(default)]
        error: Option<serde_json::Value>,
    },
    #[serde(rename = "event")]
    Event {
        event: String,
        #[serde(default)]
        payload: serde_json::Value,
        #[serde(default)]
        seq: Option<u64>,
        #[serde(default)]
        state_version: Option<u64>,
    },
}

// ============================================================================
// Connect 相关
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectChallenge {
    pub nonce: String,
    #[serde(rename = "ts")]
    pub timestamp: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HelloOk {
    #[serde(rename = "type")]
    pub hello_type: String,
    pub protocol: u32,
    pub policy: Policy,
    #[serde(default)]
    pub auth: Option<HelloAuth>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    #[serde(rename = "tickIntervalMs")]
    pub tick_interval_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HelloAuth {
    #[serde(rename = "deviceToken")]
    pub device_token: Option<String>,
    pub role: Option<String>,
    pub scopes: Option<Vec<String>>,
}

// ============================================================================
// Session 订阅
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    pub key: String,
    #[serde(rename = "sessionId")]
    pub session_id: Option<String>,
    #[serde(rename = "agentId")]
    pub agent_id: Option<String>,
    #[serde(rename = "updatedAtMs")]
    pub updated_at_ms: Option<i64>,
    #[serde(default)]
    pub flags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionMessage {
    #[serde(rename = "sessionKey")]
    pub session_key: String,
    #[serde(rename = "sessionId")]
    pub session_id: String,
    #[serde(rename = "runId")]
    pub run_id: Option<String>,
    #[serde(rename = "type")]
    pub msg_type: String,
    #[serde(default)]
    pub delta: Option<String>,
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub stream: Option<String>,
    #[serde(default)]
    pub phase: Option<String>,
    #[serde(default)]
    pub error: Option<String>,
    #[serde(default)]
    pub payload: Option<serde_json::Value>,
}

// ============================================================================
// Lifecycle 事件
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleEvent {
    #[serde(rename = "runId")]
    pub run_id: Option<String>,
    #[serde(rename = "sessionKey")]
    pub session_key: Option<String>,
    #[serde(rename = "sessionId")]
    pub session_id: Option<String>,
    pub phase: String, // "start" | "end" | "error"
    #[serde(default)]
    pub error: Option<String>,
    #[serde(default)]
    pub summary: Option<String>,
}

impl LifecycleEvent {
    pub fn is_start(&self) -> bool {
        self.phase == "start"
    }

    pub fn is_end(&self) -> bool {
        self.phase == "end"
    }

    pub fn is_error(&self) -> bool {
        self.phase == "error"
    }
}

// ============================================================================
// Tool 事件 (session.tool)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolEventData {
    pub phase: String, // "start" | "update" | "end"
    pub name: Option<String>,
    #[serde(rename = "toolCallId")]
    pub tool_call_id: Option<String>,
    #[serde(default)]
    pub args: Option<serde_json::Value>,
    #[serde(default)]
    pub partial_result: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolEvent {
    #[serde(rename = "runId")]
    pub run_id: Option<String>,
    pub stream: String,
    pub data: ToolEventData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentEvent {
    #[serde(rename = "runId")]
    pub run_id: String,
    pub stream: String,  // "lifecycle" | "assistant" | ...
    pub data: AgentEventData,
    #[serde(rename = "sessionKey")]
    pub session_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessageContent {
    #[serde(rename = "type")]
    pub content_type: String,  // "text"
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: Vec<ChatMessageContent>,
    pub timestamp: Option<i64>,
    #[serde(rename = "stopReason")]
    pub stop_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatEvent {
    #[serde(rename = "runId")]
    pub run_id: String,
    #[serde(rename = "sessionKey")]
    pub session_key: String,
    pub state: String,
    pub message: ChatMessage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentEventData {
    pub phase: Option<String>,
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub delta: Option<String>,
}

// ============================================================================
// Attachment (附件)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    #[serde(rename = "type")]
    pub attachment_type: String,  // "image" or "file"
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    #[serde(rename = "fileName")]
    pub file_name: String,
    pub content: String,  // 文件路径
}

// ============================================================================
// 请求构建辅助
// ============================================================================

impl WsMessage {
    /// 构建 connect 请求
    pub fn connect_request(id: &str, params: serde_json::Value) -> Self {
        WsMessage::Request {
            id: id.to_string(),
            method: "connect".to_string(),
            params,
        }
    }

    /// 构建 sessions.subscribe 请求
    pub fn sessions_subscribe(id: &str) -> Self {
        WsMessage::Request {
            id: id.to_string(),
            method: "sessions.subscribe".to_string(),
            params: serde_json::json!({}),
        }
    }

    /// 构建 sessions.messages.subscribe 请求
    pub fn sessions_messages_subscribe(id: &str, session_key: &str) -> Self {
        WsMessage::Request {
            id: id.to_string(),
            method: "sessions.messages.subscribe".to_string(),
            params: serde_json::json!({ "key": session_key }),
        }
    }
}