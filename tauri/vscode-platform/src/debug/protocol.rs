//! DAP protocol types — mirrors debugProtocol.d.ts
//!
//! Ref: https://microsoft.github.io/debug-adapter-protocol/specification

use serde::{Deserialize, Serialize};
use serde_json::Value;

// ─────────────────────────────────────────────────────────────────────────────
// Top-level message
// ─────────────────────────────────────────────────────────────────────────────

/// Discriminant for ProtocolMessage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageKind {
    Request,
    Response,
    Event,
}

/// DAP `ProtocolMessage` — base for all messages exchanged with adapters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolMessage {
    pub seq: u64,
    #[serde(rename = "type")]
    pub kind: MessageKind,
    /// Present when kind == Request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    /// Present when kind == Request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Value>,
    /// Present when kind == Response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_seq: Option<u64>,
    /// Present when kind == Response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// Present when kind == Response — mirrors `command` of the originating request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Present when kind == Response (body) or kind == Event (body)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<Value>,
    /// Present when kind == Event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
}

impl ProtocolMessage {
    /// Construct a request message.
    pub fn request(seq: u64, command: impl Into<String>, arguments: Option<Value>) -> Self {
        Self {
            seq,
            kind: MessageKind::Request,
            command: Some(command.into()),
            arguments,
            request_seq: None,
            success: None,
            message: None,
            body: None,
            event: None,
        }
    }

    /// Construct a success response.
    pub fn response_ok(
        seq: u64,
        request_seq: u64,
        command: impl Into<String>,
        body: Option<Value>,
    ) -> Self {
        Self {
            seq,
            kind: MessageKind::Response,
            command: Some(command.into()),
            request_seq: Some(request_seq),
            success: Some(true),
            message: None,
            body,
            arguments: None,
            event: None,
        }
    }

    /// Construct a failure response.
    pub fn response_err(
        seq: u64,
        request_seq: u64,
        command: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            seq,
            kind: MessageKind::Response,
            command: Some(command.into()),
            request_seq: Some(request_seq),
            success: Some(false),
            message: Some(message.into()),
            body: None,
            arguments: None,
            event: None,
        }
    }

    /// Construct an event message.
    pub fn event(seq: u64, event: impl Into<String>, body: Option<Value>) -> Self {
        Self {
            seq,
            kind: MessageKind::Event,
            event: Some(event.into()),
            body,
            command: None,
            arguments: None,
            request_seq: None,
            success: None,
            message: None,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Common body types
// ─────────────────────────────────────────────────────────────────────────────

/// Arguments for the `initialize` request (subset of DAP spec).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InitializeArguments {
    pub adapter_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines_start_at1: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns_start_at1: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_variable_type: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_run_in_terminal_request: Option<bool>,
}

/// Capabilities returned by adapter in `initialize` response body.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Capabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_configuration_done_request: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_function_breakpoints: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_conditional_breakpoints: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_hit_conditional_breakpoints: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_set_variable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_restart_frame: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_goto_targets_request: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_step_in_targets_request: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_completions_request: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_modules_request: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_restart_request: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_exception_options: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_value_formatting_options: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_exception_info_request: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_terminate_debuggee: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_loaded_sources_request: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_log_points: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_terminate_threads_request: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_set_expression: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_terminate_request: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_data_breakpoints: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_read_memory_request: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_cancel_request: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_breakpoint_locations_request: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_clipboard_context: Option<bool>,
}

/// Body for the `stopped` event.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoppedEventBody {
    /// Reason — e.g. "step", "breakpoint", "exception", "pause", "entry", "goto", "function breakpoint", "data breakpoint".
    pub reason: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_focus_hint: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_threads_stopped: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hit_breakpoint_ids: Option<Vec<u64>>,
}

/// Body for the `output` event.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputEventBody {
    /// Category — e.g. "console", "stdout", "stderr", "telemetry".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    pub output: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables_reference: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

/// Body for the `breakpoint` event.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BreakpointEventBody {
    /// Reason — e.g. "changed", "new", "removed".
    pub reason: String,
    pub breakpoint: Value,
}

/// Arguments for the `runInTerminal` reverse request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunInTerminalRequestArguments {
    /// "integrated" | "external"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub cwd: String,
    pub args: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<std::collections::HashMap<String, Option<String>>>,
}

/// Response body for `runInTerminal` reverse request.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RunInTerminalResponseBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shell_process_id: Option<u64>,
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn request_serializes_type_field() {
        let msg = ProtocolMessage::request(1, "initialize", None);
        let v: Value = serde_json::to_value(&msg).unwrap();
        assert_eq!(v["type"], "request");
        assert_eq!(v["seq"], 1);
        assert_eq!(v["command"], "initialize");
        assert!(v.get("arguments").is_none() || v["arguments"].is_null());
    }

    #[test]
    fn response_ok_serializes_correctly() {
        let caps = serde_json::to_value(Capabilities {
            supports_configuration_done_request: Some(true),
            ..Default::default()
        })
        .unwrap();
        let msg = ProtocolMessage::response_ok(2, 1, "initialize", Some(caps));
        let v: Value = serde_json::to_value(&msg).unwrap();
        assert_eq!(v["type"], "response");
        assert_eq!(v["success"], true);
        assert_eq!(v["request_seq"], 1);
        assert_eq!(v["body"]["supportsConfigurationDoneRequest"], true);
    }

    #[test]
    fn event_serializes_correctly() {
        let body = serde_json::to_value(StoppedEventBody {
            reason: "breakpoint".into(),
            thread_id: Some(1),
            description: None,
            preserve_focus_hint: None,
            text: None,
            all_threads_stopped: None,
            hit_breakpoint_ids: None,
        })
        .unwrap();
        let msg = ProtocolMessage::event(3, "stopped", Some(body));
        let v: Value = serde_json::to_value(&msg).unwrap();
        assert_eq!(v["type"], "event");
        assert_eq!(v["event"], "stopped");
        assert_eq!(v["body"]["reason"], "breakpoint");
    }

    #[test]
    fn response_err_has_success_false() {
        let msg = ProtocolMessage::response_err(4, 1, "initialize", "not supported");
        let v: Value = serde_json::to_value(&msg).unwrap();
        assert_eq!(v["success"], false);
        assert_eq!(v["message"], "not supported");
    }

    #[test]
    fn capabilities_round_trips() {
        let caps = Capabilities {
            supports_configuration_done_request: Some(true),
            supports_function_breakpoints: Some(false),
            ..Default::default()
        };
        let json_str = serde_json::to_string(&caps).unwrap();
        let caps2: Capabilities = serde_json::from_str(&json_str).unwrap();
        assert_eq!(caps2.supports_configuration_done_request, Some(true));
        assert_eq!(caps2.supports_function_breakpoints, Some(false));
    }

    #[test]
    fn initialize_arguments_round_trips() {
        let args = InitializeArguments {
            adapter_id: "test-adapter".into(),
            client_id: Some("vscode".into()),
            lines_start_at1: Some(true),
            ..Default::default()
        };
        let json_str = serde_json::to_string(&args).unwrap();
        let args2: InitializeArguments = serde_json::from_str(&json_str).unwrap();
        assert_eq!(args2.adapter_id, "test-adapter");
        assert_eq!(args2.lines_start_at1, Some(true));
    }
}
