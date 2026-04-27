//! vscode-ipc — IPC abstraction layer.
//!
//! Placeholder. Full implementation delivered in task #4 (Building Rust IPC abstraction).
//! Mirrors cli/src/rpc.rs generic RPC builder and cli/src/async_pipe.rs cross-platform
//! channel selection.

pub mod channel;

pub use channel::{IpcChannel, IpcMessage};

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
