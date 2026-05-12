use serde::{Deserialize, Serialize};
use serde_json::Value;

pub type RequestId = String;
pub type TraceId = String;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SubscriptionId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CancellationId(pub String);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelCallRequest {
    pub request_id: RequestId,
    pub channel: String,
    pub command: String,
    #[serde(default)]
    pub args: Vec<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_id: Option<CancellationId>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelListenRequest {
    pub request_id: RequestId,
    pub channel: String,
    pub event: String,
    #[serde(default)]
    pub args: Vec<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<SubscriptionId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelDisposeRequest {
    pub subscription_id: SubscriptionId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelRequest {
    pub cancellation_id: CancellationId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<RequestId>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelResponse {
    pub request_id: RequestId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ChannelError>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelEventMessage {
    pub subscription_id: SubscriptionId,
    pub channel: String,
    pub event: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelError {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Value>,
}

pub type ExtHostRpcActor = String;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ExtHostRpcDirection {
    MainToExtHost,
    ExtHostToMain,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtHostRpcEnvelope {
    pub protocol: String,
    pub request_id: RequestId,
    pub trace_id: TraceId,
    pub direction: ExtHostRpcDirection,
    #[serde(flatten)]
    pub message: ExtHostRpcMessage,
}

impl ExtHostRpcEnvelope {
    pub const PROTOCOL: &'static str = "extHost.protocol";

    pub fn request(
        request_id: RequestId,
        trace_id: TraceId,
        direction: ExtHostRpcDirection,
        request: ExtHostRpcRequest,
    ) -> Self {
        Self {
            protocol: Self::PROTOCOL.to_string(),
            request_id,
            trace_id,
            direction,
            message: ExtHostRpcMessage::Request { request },
        }
    }

    pub fn response(
        request_id: RequestId,
        trace_id: TraceId,
        direction: ExtHostRpcDirection,
        result: Option<Value>,
    ) -> Self {
        Self {
            protocol: Self::PROTOCOL.to_string(),
            request_id,
            trace_id,
            direction,
            message: ExtHostRpcMessage::Response { result },
        }
    }

    pub fn error(
        request_id: RequestId,
        trace_id: TraceId,
        direction: ExtHostRpcDirection,
        error: ChannelError,
    ) -> Self {
        Self {
            protocol: Self::PROTOCOL.to_string(),
            request_id,
            trace_id,
            direction,
            message: ExtHostRpcMessage::Error { error },
        }
    }

    pub fn cancel(
        request_id: RequestId,
        trace_id: TraceId,
        direction: ExtHostRpcDirection,
        cancellation_id: Option<CancellationId>,
    ) -> Self {
        Self {
            protocol: Self::PROTOCOL.to_string(),
            request_id,
            trace_id,
            direction,
            message: ExtHostRpcMessage::Cancel { cancellation_id },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ExtHostRpcMessage {
    Request {
        request: ExtHostRpcRequest,
    },
    Response {
        #[serde(skip_serializing_if = "Option::is_none")]
        result: Option<Value>,
    },
    Error {
        error: ChannelError,
    },
    Cancel {
        #[serde(skip_serializing_if = "Option::is_none")]
        cancellation_id: Option<CancellationId>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtHostRpcRequest {
    pub actor: ExtHostRpcActor,
    pub method: String,
    #[serde(default)]
    pub args: Vec<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_id: Option<CancellationId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UriDto {
    pub scheme: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authority: Option<String>,
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fragment: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FileTypeDto {
    Unknown,
    File,
    Directory,
    SymbolicLink,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileStatDto {
    pub resource: UriDto,
    pub r#type: FileTypeDto,
    pub ctime: u64,
    pub mtime: u64,
    pub size: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileReadRequest {
    pub resource: UriDto,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileReadResponse {
    pub data_base64: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileWriteRequest {
    pub resource: UriDto,
    pub data_base64: String,
    #[serde(default)]
    pub create: bool,
    #[serde(default)]
    pub overwrite: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileDeleteRequest {
    pub resource: UriDto,
    #[serde(default)]
    pub recursive: bool,
    #[serde(default)]
    pub use_trash: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileStatRequest {
    pub resource: UriDto,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileMkdirRequest {
    pub resource: UriDto,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileReaddirRequest {
    pub resource: UriDto,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileDirEntryDto {
    pub name: String,
    pub r#type: FileTypeDto,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn channel_call_request_roundtrips_camel_case_json() {
        let request = ChannelCallRequest {
            request_id: "req-1".to_string(),
            channel: "files".to_string(),
            command: "stat".to_string(),
            args: vec![json!({ "resource": { "scheme": "file", "path": "/tmp/a.txt" } })],
            cancellation_id: Some(CancellationId("cancel-1".to_string())),
        };

        let serialized = serde_json::to_value(&request).expect("serialize request");
        assert_eq!(serialized["requestId"], "req-1");
        assert_eq!(serialized["cancellationId"], "cancel-1");
        assert_eq!(serialized["args"][0]["resource"]["path"], "/tmp/a.txt");

        let deserialized: ChannelCallRequest =
            serde_json::from_value(serialized).expect("deserialize request");
        assert_eq!(deserialized, request);
    }

    #[test]
    fn file_stat_roundtrips_uri_and_file_type() {
        let stat = FileStatDto {
            resource: UriDto {
                scheme: "file".to_string(),
                authority: None,
                path: "/workspace/src/main.rs".to_string(),
                query: None,
                fragment: None,
            },
            r#type: FileTypeDto::File,
            ctime: 10,
            mtime: 20,
            size: 30,
            readonly: Some(false),
        };

        let serialized = serde_json::to_value(&stat).expect("serialize stat");
        assert_eq!(serialized["type"], "file");
        assert_eq!(serialized["resource"]["scheme"], "file");
        assert_eq!(serialized["readonly"], false);

        let deserialized: FileStatDto =
            serde_json::from_value(serialized).expect("deserialize stat");
        assert_eq!(deserialized, stat);
    }

    #[test]
    fn ext_host_rpc_request_envelope_roundtrips_protocol_fields() {
        let envelope = ExtHostRpcEnvelope::request(
            "req-1".to_string(),
            "trace-1".to_string(),
            ExtHostRpcDirection::ExtHostToMain,
            ExtHostRpcRequest {
                actor: "MainThreadCommands".to_string(),
                method: "$executeCommand".to_string(),
                args: vec![json!("workbench.action.files.save")],
                cancellation_id: Some(CancellationId("cancel-1".to_string())),
            },
        );

        let serialized = serde_json::to_value(&envelope).expect("serialize envelope");
        assert_eq!(serialized["protocol"], "extHost.protocol");
        assert_eq!(serialized["requestId"], "req-1");
        assert_eq!(serialized["traceId"], "trace-1");
        assert_eq!(serialized["direction"], "extHostToMain");
        assert_eq!(serialized["type"], "request");
        assert_eq!(serialized["request"]["actor"], "MainThreadCommands");
        assert_eq!(serialized["request"]["method"], "$executeCommand");
        assert_eq!(serialized["request"]["cancellationId"], "cancel-1");

        let deserialized: ExtHostRpcEnvelope =
            serde_json::from_value(serialized).expect("deserialize envelope");
        assert_eq!(deserialized, envelope);
    }

    #[test]
    fn ext_host_rpc_request_accepts_current_main_context_actor_names() {
        let envelope: ExtHostRpcEnvelope = serde_json::from_value(json!({
            "protocol": "extHost.protocol",
            "requestId": "req-language-models",
            "traceId": "trace-language-models",
            "direction": "extHostToMain",
            "type": "request",
            "request": {
                "actor": "MainThreadLanguageModels",
                "method": "$selectChatModels",
                "args": []
            }
        }))
        .expect("deserialize envelope with current MainContext actor");

        match envelope.message {
            ExtHostRpcMessage::Request { request } => {
                assert_eq!(request.actor, "MainThreadLanguageModels");
                assert_eq!(request.method, "$selectChatModels");
            }
            other => panic!("expected request envelope, got {other:?}"),
        }
    }

    #[test]
    fn ext_host_rpc_error_and_cancel_envelopes_preserve_trace_id() {
        let error = ExtHostRpcEnvelope::error(
            "req-2".to_string(),
            "trace-2".to_string(),
            ExtHostRpcDirection::MainToExtHost,
            ChannelError {
                code: "ENOENT".to_string(),
                message: "Missing".to_string(),
                details: Some(json!({ "path": "/missing" })),
            },
        );
        let cancel = ExtHostRpcEnvelope::cancel(
            "req-3".to_string(),
            "trace-3".to_string(),
            ExtHostRpcDirection::ExtHostToMain,
            Some(CancellationId("cancel-3".to_string())),
        );

        assert_eq!(
            serde_json::to_value(error).expect("serialize error")["traceId"],
            "trace-2"
        );
        assert_eq!(
            serde_json::to_value(cancel).expect("serialize cancel")["traceId"],
            "trace-3"
        );
    }
}
