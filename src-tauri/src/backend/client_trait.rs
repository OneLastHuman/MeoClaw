use std::collections::HashMap;
use std::error::Error;
use serde::{Deserialize, Serialize};

/// 工具事件的执行阶段
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ToolPhase {
    Started,
    Progress,
    Result,
    Error,
}

/// 工具事件结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolEvent {
    pub name: String,
    pub phase: ToolPhase,
    pub args: Option<serde_json::Value>,
    pub preview: Option<String>,
}

/// 流式响应结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamResponse {
    pub text_delta: String,
    pub tool_event: Option<ToolEvent>,
    pub is_final: bool,
}

/// Stream trait 用于返回流式数据
pub trait Stream: Send {
    fn poll_next(&mut self) -> Option<Result<StreamResponse, Box<dyn Error + Send + Sync>>>;
}

/// BackendClient trait - 所有后端客户端必须实现此接口
pub trait BackendClient: Send + Sync {
    /// 返回后端名称
    fn name(&self) -> &str;

    /// 健康检测
    fn check_health(&self) -> bool;

    /// 发送消息，返回流式响应
    fn send_message(&self, text: &str) -> Result<Box<dyn Stream>, Box<dyn Error + Send + Sync>>;

    /// 获取端点地址
    fn endpoint(&self) -> &str;

    /// 获取认证头
    fn auth_headers(&self) -> HashMap<String, String>;
}