//! IChannel / IServerChannel async traits and wire-message types.
//!
//! Mirrors src/vs/base/parts/ipc/common/ipc.ts IChannel / IServerChannel.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

// ─────────────────────────────────────────────
// Wire envelope
// ─────────────────────────────────────────────

/// Unique identifier for an in-flight request.
pub type RequestId = u32;

/// Every message on the wire is wrapped in one of these variants.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ChannelMessage {
    /// Client → Server: call a command.
    Request {
        id: RequestId,
        command: String,
        #[serde(default)]
        arg: serde_json::Value,
    },
    /// Server → Client: successful reply.
    Response {
        id: RequestId,
        result: serde_json::Value,
    },
    /// Server → Client: error reply.
    ResponseError {
        id: RequestId,
        error: WireError,
    },
    /// Client → Server: cancel an in-flight request.
    Cancel { id: RequestId },
    /// Server → Client (or Client → Server): push-event notification.
    Event {
        #[serde(default)]
        event: String,
        data: serde_json::Value,
    },
    /// Client → Server: subscribe to a server-side event stream.
    Subscribe {
        id: RequestId,
        event: String,
    },
    /// Client → Server: unsubscribe from a server-side event stream.
    Unsubscribe { id: RequestId },
}

/// Portable error object sent over the wire.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WireError {
    pub code: i32,
    pub message: String,
}

// ─────────────────────────────────────────────
// IpcMessage — legacy thin envelope (kept for compat)
// ─────────────────────────────────────────────

/// Thin opaque IPC envelope (kept from original scaffold).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpcMessage {
    pub method: String,
    pub payload: serde_json::Value,
}

// ─────────────────────────────────────────────
// IChannel / IServerChannel traits
// ─────────────────────────────────────────────

/// Client-side channel: call commands and listen to events.
#[async_trait]
pub trait IChannel: Send + Sync {
    /// Invoke a command, awaiting the response.
    async fn call(
        &self,
        command: &str,
        arg: serde_json::Value,
    ) -> anyhow::Result<serde_json::Value>;

    // `listen` (event stream) is out of scope for this iteration.
}

/// Server-side channel: handle commands and emit events.
#[async_trait]
pub trait IServerChannel: Send + Sync {
    /// Handle an incoming command, returning a response value.
    async fn call(
        &self,
        command: &str,
        arg: serde_json::Value,
    ) -> anyhow::Result<serde_json::Value>;
}

/// Abstraction over a bidirectional IPC transport (legacy trait, kept for compat).
#[async_trait]
pub trait IpcChannel: Send + Sync {
    async fn send(&self, msg: IpcMessage) -> anyhow::Result<()>;
    async fn recv(&self) -> anyhow::Result<IpcMessage>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn channel_message_request_round_trips() {
        let msg = ChannelMessage::Request {
            id: 1,
            command: "window/getTitle".into(),
            arg: serde_json::json!({"windowId": 1}),
        };
        let json = serde_json::to_string(&msg).unwrap();
        let decoded: ChannelMessage = serde_json::from_str(&json).unwrap();
        match decoded {
            ChannelMessage::Request { id, command, arg } => {
                assert_eq!(id, 1);
                assert_eq!(command, "window/getTitle");
                assert_eq!(arg["windowId"], 1);
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn channel_message_cancel_round_trips() {
        let msg = ChannelMessage::Cancel { id: 42 };
        let json = serde_json::to_string(&msg).unwrap();
        let decoded: ChannelMessage = serde_json::from_str(&json).unwrap();
        match decoded {
            ChannelMessage::Cancel { id } => assert_eq!(id, 42),
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn ipc_message_round_trips() {
        let msg = IpcMessage {
            method: "window/getTitle".to_string(),
            payload: serde_json::json!({"windowId": 1}),
        };
        let json = serde_json::to_string(&msg).unwrap();
        let decoded: IpcMessage = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.method, "window/getTitle");
        assert_eq!(decoded.payload["windowId"], 1);
    }
}
