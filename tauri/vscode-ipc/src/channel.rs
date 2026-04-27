//! IpcChannel — thin async channel abstraction over Tauri invoke / Unix socket /
//! named pipe, mirroring cli/src/async_pipe.rs.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// An opaque IPC message (JSON-serialisable envelope).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpcMessage {
    pub method: String,
    pub payload: serde_json::Value,
}

/// Abstraction over a bidirectional IPC transport.
#[async_trait]
pub trait IpcChannel: Send + Sync {
    async fn send(&self, msg: IpcMessage) -> anyhow::Result<()>;
    async fn recv(&self) -> anyhow::Result<IpcMessage>;
}
