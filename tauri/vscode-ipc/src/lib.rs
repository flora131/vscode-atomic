//! vscode-ipc — IPC/RPC abstraction layer.
//!
//! Mirrors cli/src/rpc.rs + cli/src/json_rpc.rs + cli/src/util/sync.rs.

pub mod channel;
pub mod dispatcher;
pub mod framing;
pub mod sync;
pub mod transport;

// Top-level re-exports for convenience
pub use channel::{ChannelMessage, IChannel, IServerChannel, IpcMessage, RequestId, WireError};
pub use dispatcher::{MaybeSync, RpcCaller, RpcDispatcher, RpcMethodBuilder, RpcResponseDispatcher};
pub use framing::{start_json_rpc, JsonRpcSerializer};
pub use sync::{new_barrier, Barrier, BarrierOpener, Receivable};
pub use transport::{DuplexTransport, Transport};

#[cfg(test)]
mod tests {
    use super::IpcMessage;

    #[test]
    fn ipc_message_serialises_and_deserialises() {
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
