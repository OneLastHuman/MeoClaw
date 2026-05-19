pub mod client_trait;
pub mod manager;
pub mod openclaw_client;
pub mod hermes_client;
pub mod hermes_discovery;

pub use client_trait::{BackendClient, StreamResponse, Stream, ToolEvent, ToolPhase};
pub use manager::{ActiveBackend, BackendConfig, BackendManager, BackendHealth, OpenClawConfig, LegacyHermesConfig};
pub use hermes_client::HermesClient;
pub use hermes_discovery::{HermesDiscovery, HermesConfig, HermesInstanceId, DiscoverySource};
pub use openclaw_client::OpenClawClientAdapter;