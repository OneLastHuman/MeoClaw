//! OpenClaw 模块
//!
//! 负责与 OpenClaw Gateway 的 WebSocket 连接、认证和状态同步

mod auth;
mod client;
mod discovery;
mod protocol;

pub use client::{ConnectionStatusCallback, OpenClawClient, ToolCallback};
pub use discovery::DiscoveryResult;
pub use protocol::{AgentEvent, Attachment, LifecycleEvent, ToolEvent};