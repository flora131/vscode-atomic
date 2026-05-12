use std::{
    borrow::Cow,
    sync::atomic::{AtomicU64, Ordering},
};

use serde::{Deserialize, Serialize};
use tracing_subscriber::{fmt, EnvFilter};

static NEXT_TRACE_ID: AtomicU64 = AtomicU64::new(1);

pub const SIDECAR_TOKEN_ENV_NAME: &str = "VSCODE_ATOMIC_EXTENSION_HOST_TOKEN";
pub const REDACTED_SECRET: &str = "<redacted>";

pub const CHANNEL_CALL_INVOCATIONS_METRIC: &str = "channelCall.invocations";
pub const CHANNEL_LISTEN_ACTIVE_METRIC: &str = "channelListen.active";
pub const CHANNEL_EVENT_DELIVERED_METRIC: &str = "channelEvent.delivered";
pub const CHANNEL_EVENT_DROPPED_METRIC: &str = "channelEvent.dropped";
pub const FILE_WRITE_FAILURES_METRIC: &str = "fileWrite.failures";
pub const BRIDGE_ROUND_TRIP_MS_METRIC: &str = "bridge.roundTripMs";
pub const WORKBENCH_READY_MS_METRIC: &str = "workbench.readyMs";
pub const EXTENSION_HOST_READY_MS_METRIC: &str = "extensionHost.readyMs";
pub const FILE_WATCH_ACTIVE_METRIC: &str = "fileWatch.active";
pub const TERMINAL_SPAWN_MS_METRIC: &str = "terminal.spawnMs";
pub const BRIDGE_SERVICE_BOOT_MS_METRIC: &str = "bridge.serviceBootMs";
pub const PACKAGE_MARKER_METRIC: &str = "package.marker";

pub const REQUIRED_BRIDGE_METRICS: &[&str] = &[
    CHANNEL_CALL_INVOCATIONS_METRIC,
    CHANNEL_LISTEN_ACTIVE_METRIC,
    CHANNEL_EVENT_DELIVERED_METRIC,
    CHANNEL_EVENT_DROPPED_METRIC,
    FILE_WRITE_FAILURES_METRIC,
    FILE_WATCH_ACTIVE_METRIC,
    BRIDGE_ROUND_TRIP_MS_METRIC,
    EXTENSION_HOST_READY_MS_METRIC,
    WORKBENCH_READY_MS_METRIC,
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TraceId(pub String);

impl TraceId {
    pub fn new(scope: &str) -> Self {
        let sequence = NEXT_TRACE_ID.fetch_add(1, Ordering::Relaxed);
        Self(format!("tauri-{scope}-{sequence:016x}"))
    }

    pub fn for_command(command: &str, request_id: &str) -> Self {
        Self(format!(
            "tauri-command-{}-{}",
            stable_token(command),
            stable_token(request_id)
        ))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetricEvent {
    pub trace_id: TraceId,
    pub name: String,
    pub value: f64,
    pub unit: MetricUnit,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MetricUnit {
    Count,
    Milliseconds,
    Bytes,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BridgeRoundTripSummary {
    pub count: usize,
    pub p50: f64,
    pub p95: f64,
    pub p99: f64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MetricValidationError {
    MissingRequiredMetric(String),
    UnknownMetric(String),
    NonFiniteValue(String),
    InvalidUnit {
        name: String,
        expected: MetricUnit,
        actual: MetricUnit,
    },
}

pub const RUNTIME_DEPENDENCY_PREFLIGHT_EVENT_NAME: &str = "runtimeDependency.preflight";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RuntimeDependencyPreflightStatus {
    Pass,
    Fail,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RuntimeDependencyKind {
    #[serde(rename = "pkg-config")]
    PkgConfig,
    #[serde(rename = "gtk-glib-webkitgtk")]
    GtkGlibWebKitGtk,
    Tsgo,
    #[serde(rename = "eslint-plugin-import")]
    EslintPluginImport,
}

impl RuntimeDependencyKind {
    pub fn display_name(self) -> &'static str {
        match self {
            Self::PkgConfig => "pkg-config",
            Self::GtkGlibWebKitGtk => "GTK/GLib/WebKitGTK",
            Self::Tsgo => "tsgo",
            Self::EslintPluginImport => "ESLint plugin import",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeDependencyPreflightCheck {
    pub name: String,
    pub status: RuntimeDependencyPreflightStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

impl RuntimeDependencyPreflightCheck {
    pub fn pass(name: impl Into<String>, version: Option<impl Into<String>>) -> Self {
        Self {
            name: name.into(),
            status: RuntimeDependencyPreflightStatus::Pass,
            version: version.map(Into::into),
            detail: None,
        }
    }

    pub fn fail(name: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            status: RuntimeDependencyPreflightStatus::Fail,
            version: None,
            detail: Some(detail.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeDependencyPreflightSummary {
    pub status: RuntimeDependencyPreflightStatus,
    pub passed: usize,
    pub failed: usize,
    pub total: usize,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeDependencyPreflightEvent {
    pub trace_id: TraceId,
    pub name: String,
    pub dependency: RuntimeDependencyKind,
    pub status: RuntimeDependencyPreflightStatus,
    pub summary: RuntimeDependencyPreflightSummary,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub checks: Vec<RuntimeDependencyPreflightCheck>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrashReporterPlaceholder {
    pub owner_question: &'static str,
}

impl Default for CrashReporterPlaceholder {
    fn default() -> Self {
        Self {
            // OPEN QUESTION(rfc-observability): decide crash reporter owner/replacement for
            // Tauri runtime before enabling native crash upload. VS Code Electron crash
            // ownership does not map directly to this process boundary.
            owner_question: "Who owns the Tauri crash reporter replacement and upload pipeline?",
        }
    }
}

pub fn init_tracing() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("vscode_atomic_tauri=info"));

    let _ = fmt()
        .with_env_filter(filter)
        .with_target(true)
        .with_thread_ids(true)
        .compact()
        .try_init();
}

pub fn command_trace_id(command: &str, request_id: &str) -> TraceId {
    TraceId::for_command(command, request_id)
}

pub fn ext_host_rpc_trace_id(request_id: &str) -> TraceId {
    TraceId::for_command("extHostRpc", request_id)
}

pub fn record_metric(event: &MetricEvent) -> MetricEvent {
    let redacted = MetricEvent {
        trace_id: event.trace_id.clone(),
        name: event.name.clone(),
        value: event.value,
        unit: event.unit,
        context: event.context.as_ref().map(redact_json_value),
    };
    let context = event.context.as_ref().map(redact_json_value);
    tracing::info!(
        target: "vscode_atomic_tauri::metrics",
        command_trace_id = redacted.trace_id.as_str(),
        ext_host_rpc_trace_id = %ext_host_rpc_trace_id(redacted.trace_id.as_str()).as_str(),
        metric_name = redacted.name.as_str(),
        value = redacted.value,
        unit = ?redacted.unit,
        context = ?context,
        "metric event placeholder"
    );
    redacted
}

pub fn metric_event(
    trace_id: TraceId,
    name: impl Into<String>,
    value: f64,
    unit: MetricUnit,
    context: Option<serde_json::Value>,
) -> MetricEvent {
    MetricEvent {
        trace_id,
        name: name.into(),
        value,
        unit,
        context: context.map(|value| redact_json_value(&value)),
    }
}

pub fn redact_json_value(value: &serde_json::Value) -> serde_json::Value {
    match value {
        serde_json::Value::Object(object) => serde_json::Value::Object(
            object
                .iter()
                .map(|(key, value)| {
                    if is_sensitive_key(key) {
                        (
                            key.clone(),
                            serde_json::Value::String(REDACTED_SECRET.to_string()),
                        )
                    } else if is_sensitive_path_key(key) {
                        (key.clone(), redact_path_json_value(value))
                    } else {
                        (key.clone(), redact_json_value(value))
                    }
                })
                .collect(),
        ),
        serde_json::Value::Array(values) => {
            serde_json::Value::Array(values.iter().map(redact_json_value).collect())
        }
        serde_json::Value::String(value) => serde_json::Value::String(redact_token_values(value)),
        _ => value.clone(),
    }
}

pub fn validate_required_metrics(events: &[MetricEvent]) -> Result<(), MetricValidationError> {
    for required_name in REQUIRED_BRIDGE_METRICS {
        if !events.iter().any(|event| event.name == *required_name) {
            return Err(MetricValidationError::MissingRequiredMetric(
                (*required_name).to_string(),
            ));
        }
    }

    for event in events {
        validate_metric_event(event)?;
    }

    Ok(())
}

pub fn validate_metric_event(event: &MetricEvent) -> Result<(), MetricValidationError> {
    if !event.value.is_finite() {
        return Err(MetricValidationError::NonFiniteValue(event.name.clone()));
    }

    let expected = expected_metric_unit(event.name.as_str())
        .ok_or_else(|| MetricValidationError::UnknownMetric(event.name.clone()))?;
    if event.unit != expected {
        return Err(MetricValidationError::InvalidUnit {
            name: event.name.clone(),
            expected,
            actual: event.unit,
        });
    }

    Ok(())
}

pub fn summarize_bridge_round_trip_ms(samples: &[f64]) -> Option<BridgeRoundTripSummary> {
    if samples.is_empty() || samples.iter().any(|sample| !sample.is_finite()) {
        return None;
    }

    let mut sorted = samples.to_vec();
    sorted.sort_by(|a, b| a.total_cmp(b));
    Some(BridgeRoundTripSummary {
        count: sorted.len(),
        p50: percentile(&sorted, 50.0),
        p95: percentile(&sorted, 95.0),
        p99: percentile(&sorted, 99.0),
    })
}

pub fn redact_env_value<'a>(name: &str, value: &'a str) -> Cow<'a, str> {
    if name == SIDECAR_TOKEN_ENV_NAME {
        Cow::Borrowed(REDACTED_SECRET)
    } else {
        Cow::Borrowed(value)
    }
}

pub fn redacted_sidecar_token() -> &'static str {
    REDACTED_SECRET
}

pub fn runtime_dependency_preflight_trace_id(dependency: RuntimeDependencyKind) -> TraceId {
    TraceId::new(&format!(
        "runtimeDependency-preflight-{}",
        stable_token(dependency.display_name())
    ))
}

pub fn runtime_dependency_preflight_event(
    trace_id: TraceId,
    dependency: RuntimeDependencyKind,
    checks: Vec<RuntimeDependencyPreflightCheck>,
) -> RuntimeDependencyPreflightEvent {
    let summary = summarize_runtime_dependency_preflight(dependency, &checks);

    RuntimeDependencyPreflightEvent {
        trace_id,
        name: RUNTIME_DEPENDENCY_PREFLIGHT_EVENT_NAME.to_string(),
        dependency,
        status: summary.status,
        summary,
        checks,
    }
}

pub fn summarize_runtime_dependency_preflight(
    dependency: RuntimeDependencyKind,
    checks: &[RuntimeDependencyPreflightCheck],
) -> RuntimeDependencyPreflightSummary {
    let failed = checks
        .iter()
        .filter(|check| check.status == RuntimeDependencyPreflightStatus::Fail)
        .count();
    let total = checks.len();
    let passed = total.saturating_sub(failed);
    let status = if failed == 0 {
        RuntimeDependencyPreflightStatus::Pass
    } else {
        RuntimeDependencyPreflightStatus::Fail
    };

    RuntimeDependencyPreflightSummary {
        status,
        passed,
        failed,
        total,
        message: format_runtime_dependency_preflight_summary(
            dependency, status, passed, failed, total,
        ),
    }
}

pub fn record_runtime_dependency_preflight(event: &RuntimeDependencyPreflightEvent) {
    tracing::info!(
        target: "vscode_atomic_tauri::runtime_dependency",
        trace_id = event.trace_id.as_str(),
        event_name = event.name.as_str(),
        dependency = ?event.dependency,
        status = ?event.status,
        summary = event.summary.message.as_str(),
        passed = event.summary.passed,
        failed = event.summary.failed,
        total = event.summary.total,
        checks = ?event.checks,
        "runtime dependency preflight"
    );
}

fn format_runtime_dependency_preflight_summary(
    dependency: RuntimeDependencyKind,
    status: RuntimeDependencyPreflightStatus,
    passed: usize,
    failed: usize,
    total: usize,
) -> String {
    let status_label = match status {
        RuntimeDependencyPreflightStatus::Pass => "pass",
        RuntimeDependencyPreflightStatus::Fail => "fail",
    };

    format!(
        "{} preflight {status_label}: {passed}/{total} passed, {failed} failed",
        dependency.display_name()
    )
}

fn stable_token(value: &str) -> String {
    value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || character == '-' || character == '_' {
                character
            } else {
                '-'
            }
        })
        .collect()
}

fn expected_metric_unit(name: &str) -> Option<MetricUnit> {
    match name {
        CHANNEL_CALL_INVOCATIONS_METRIC
        | CHANNEL_LISTEN_ACTIVE_METRIC
        | CHANNEL_EVENT_DELIVERED_METRIC
        | CHANNEL_EVENT_DROPPED_METRIC
        | FILE_WRITE_FAILURES_METRIC
        | FILE_WATCH_ACTIVE_METRIC
        | PACKAGE_MARKER_METRIC => Some(MetricUnit::Count),
        BRIDGE_ROUND_TRIP_MS_METRIC
        | EXTENSION_HOST_READY_MS_METRIC
        | WORKBENCH_READY_MS_METRIC
        | TERMINAL_SPAWN_MS_METRIC
        | BRIDGE_SERVICE_BOOT_MS_METRIC => Some(MetricUnit::Milliseconds),
        _ => None,
    }
}

fn redact_path_json_value(value: &serde_json::Value) -> serde_json::Value {
    match value {
        serde_json::Value::String(value) => {
            serde_json::Value::String(redact_path_value(value).into_owned())
        }
        serde_json::Value::Object(object) => serde_json::Value::Object(
            object
                .iter()
                .map(|(key, value)| {
                    if is_sensitive_path_key(key) {
                        (key.clone(), redact_path_json_value(value))
                    } else {
                        (key.clone(), redact_json_value(value))
                    }
                })
                .collect(),
        ),
        serde_json::Value::Array(values) => {
            serde_json::Value::Array(values.iter().map(redact_path_json_value).collect())
        }
        _ => value.clone(),
    }
}

fn redact_path_value(value: &str) -> Cow<'_, str> {
    if value.starts_with('/')
        || value.starts_with("~/")
        || value.starts_with("file://")
        || value.contains("/Users/")
        || value.contains("/home/")
        || value.contains("\\Users\\")
    {
        Cow::Borrowed(REDACTED_SECRET)
    } else {
        Cow::Borrowed(value)
    }
}

fn percentile(sorted_samples: &[f64], percentile: f64) -> f64 {
    let index =
        ((percentile / 100.0) * (sorted_samples.len().saturating_sub(1) as f64)).ceil() as usize;
    sorted_samples[index]
}

fn redact_token_values(value: &str) -> String {
    value
        .split_whitespace()
        .map(|segment| {
            let trimmed = segment.trim_matches(|character: char| {
                matches!(
                    character,
                    ',' | ';' | ':' | ')' | '(' | '[' | ']' | '{' | '}'
                )
            });
            if is_sidecar_token_value(trimmed) {
                segment.replace(trimmed, REDACTED_SECRET)
            } else {
                segment.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn is_sidecar_token_value(value: &str) -> bool {
    const SIDECAR_TOKEN_PREFIX: &str = "vscode-atomic-";
    value
        .strip_prefix(SIDECAR_TOKEN_PREFIX)
        .is_some_and(|suffix| {
            suffix.len() >= 16
                && suffix
                    .chars()
                    .all(|character| character.is_ascii_hexdigit())
        })
}

fn is_sensitive_key(key: &str) -> bool {
    let normalized = key
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect::<String>();

    normalized.contains("token")
        || normalized.contains("credential")
        || normalized.contains("password")
        || normalized.contains("secret")
        || normalized.contains("signing")
        || normalized.contains("signature")
        || normalized.contains("privatekey")
        || normalized == "database64"
        || normalized == "filecontents"
        || normalized == "content"
        || normalized == "contents"
        || normalized == "payload"
        || normalized == "extensionpayload"
}

fn is_sensitive_path_key(key: &str) -> bool {
    let normalized = key
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect::<String>();

    normalized == "path"
        || normalized == "filepath"
        || normalized == "filename"
        || normalized == "resource"
        || normalized == "uri"
        || normalized.ends_with("path")
        || normalized.ends_with("uri")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn command_trace_id_is_stable_and_serializes_as_string() {
        let trace_id = command_trace_id("channel.call", "req/1");

        assert_eq!(trace_id.as_str(), "tauri-command-channel-call-req-1");
        assert_eq!(
            serde_json::to_value(&trace_id).expect("serialize trace id"),
            json!("tauri-command-channel-call-req-1")
        );
    }

    #[test]
    fn ext_host_rpc_trace_id_uses_ext_host_scope() {
        let trace_id = ext_host_rpc_trace_id("req-2");

        assert_eq!(trace_id.as_str(), "tauri-command-extHostRpc-req-2");
    }

    #[test]
    fn metric_event_serializes_camel_case_payload() {
        let metric = MetricEvent {
            trace_id: TraceId("trace-1".to_string()),
            name: "channelCall.duration".to_string(),
            value: 12.5,
            unit: MetricUnit::Milliseconds,
            context: Some(json!({ "channel": "extensionHostService" })),
        };

        let serialized = serde_json::to_value(&metric).expect("serialize metric");

        assert_eq!(serialized["traceId"], "trace-1");
        assert_eq!(serialized["name"], "channelCall.duration");
        assert_eq!(serialized["unit"], "milliseconds");
        assert_eq!(serialized["context"]["channel"], "extensionHostService");
    }

    #[test]
    fn metric_event_redacts_sensitive_context() {
        let metric = metric_event(
            TraceId("trace-2".to_string()),
            CHANNEL_CALL_INVOCATIONS_METRIC,
            1.0,
            MetricUnit::Count,
            Some(json!({
                "channel": "extensionHostService",
                "dataBase64": "file contents",
                "payload": { "token": "abc", "safe": "not logged because payload redacted" },
                "nested": [{ "credential": "secret" }],
                "signingMaterial": "key"
            })),
        );

        let context = metric.context.expect("redacted context");

        assert_eq!(context["channel"], "extensionHostService");
        assert_eq!(context["dataBase64"], REDACTED_SECRET);
        assert_eq!(context["payload"], REDACTED_SECRET);
        assert_eq!(context["nested"][0]["credential"], REDACTED_SECRET);
        assert_eq!(context["signingMaterial"], REDACTED_SECRET);
    }

    #[test]
    fn required_metric_names_remain_stable() {
        assert_eq!(CHANNEL_CALL_INVOCATIONS_METRIC, "channelCall.invocations");
        assert_eq!(CHANNEL_LISTEN_ACTIVE_METRIC, "channelListen.active");
        assert_eq!(CHANNEL_EVENT_DELIVERED_METRIC, "channelEvent.delivered");
        assert_eq!(CHANNEL_EVENT_DROPPED_METRIC, "channelEvent.dropped");
        assert_eq!(FILE_WRITE_FAILURES_METRIC, "fileWrite.failures");
        assert_eq!(BRIDGE_ROUND_TRIP_MS_METRIC, "bridge.roundTripMs");
        assert_eq!(WORKBENCH_READY_MS_METRIC, "workbench.readyMs");
        assert_eq!(EXTENSION_HOST_READY_MS_METRIC, "extensionHost.readyMs");
        assert_eq!(FILE_WATCH_ACTIVE_METRIC, "fileWatch.active");
        assert_eq!(TERMINAL_SPAWN_MS_METRIC, "terminal.spawnMs");
        assert_eq!(BRIDGE_SERVICE_BOOT_MS_METRIC, "bridge.serviceBootMs");
        assert_eq!(PACKAGE_MARKER_METRIC, "package.marker");
    }

    #[test]
    fn package_and_boot_marker_units_are_known() {
        assert_eq!(
            expected_metric_unit(PACKAGE_MARKER_METRIC),
            Some(MetricUnit::Count)
        );
        assert_eq!(
            expected_metric_unit(BRIDGE_SERVICE_BOOT_MS_METRIC),
            Some(MetricUnit::Milliseconds)
        );
    }

    #[test]
    fn required_metric_set_matches_rfc_names() {
        assert_eq!(
            REQUIRED_BRIDGE_METRICS,
            &[
                "channelCall.invocations",
                "channelListen.active",
                "channelEvent.delivered",
                "channelEvent.dropped",
                "fileWrite.failures",
                "fileWatch.active",
                "bridge.roundTripMs",
                "extensionHost.readyMs",
                "workbench.readyMs",
            ]
        );
    }

    #[test]
    fn record_metric_returns_redacted_event() {
        let recorded = record_metric(&MetricEvent {
            trace_id: TraceId("trace-record".to_string()),
            name: FILE_WRITE_FAILURES_METRIC.to_string(),
            value: 1.0,
            unit: MetricUnit::Count,
            context: Some(json!({
                "path": "/home/alice/project/.env",
                "token": "abc",
                "message": "failed without secret",
            })),
        });

        let context = recorded.context.expect("redacted context");
        assert_eq!(recorded.name, FILE_WRITE_FAILURES_METRIC);
        assert_eq!(context["path"], REDACTED_SECRET);
        assert_eq!(context["token"], REDACTED_SECRET);
        assert_eq!(context["message"], "failed without secret");
    }

    #[test]
    fn metric_event_redacts_sensitive_paths() {
        let metric = metric_event(
            TraceId("trace-paths".to_string()),
            CHANNEL_EVENT_DROPPED_METRIC,
            1.0,
            MetricUnit::Count,
            Some(json!({
                "path": "/Users/alice/workspace/settings.json",
                "resource": { "scheme": "file", "path": "/home/alice/.ssh/id_rsa" },
                "uri": "file:///home/alice/project/.env",
                "safe": "relative-name.txt"
            })),
        );

        let context = metric.context.expect("redacted context");
        assert_eq!(context["path"], REDACTED_SECRET);
        assert_eq!(context["resource"]["path"], REDACTED_SECRET);
        assert_eq!(context["uri"], REDACTED_SECRET);
        assert_eq!(context["safe"], "relative-name.txt");
    }

    #[test]
    fn validates_required_metrics_and_units() {
        let events = REQUIRED_BRIDGE_METRICS
            .iter()
            .map(|name| MetricEvent {
                trace_id: TraceId(format!("trace-{name}")),
                name: (*name).to_string(),
                value: 1.0,
                unit: expected_metric_unit(name).expect("known metric"),
                context: None,
            })
            .collect::<Vec<_>>();

        assert_eq!(validate_required_metrics(&events), Ok(()));

        let mut malformed = events.clone();
        malformed[0].unit = MetricUnit::Milliseconds;
        assert_eq!(
            validate_required_metrics(&malformed),
            Err(MetricValidationError::InvalidUnit {
                name: CHANNEL_CALL_INVOCATIONS_METRIC.to_string(),
                expected: MetricUnit::Count,
                actual: MetricUnit::Milliseconds,
            })
        );

        assert_eq!(
            validate_required_metrics(&events[1..]),
            Err(MetricValidationError::MissingRequiredMetric(
                CHANNEL_CALL_INVOCATIONS_METRIC.to_string()
            ))
        );
    }

    #[test]
    fn summarizes_bridge_round_trip_percentiles() {
        let summary =
            summarize_bridge_round_trip_ms(&[1.0, 10.0, 2.0, 100.0, 20.0]).expect("summary");

        assert_eq!(summary.count, 5);
        assert_eq!(summary.p50, 10.0);
        assert_eq!(summary.p95, 100.0);
        assert_eq!(summary.p99, 100.0);
        assert_eq!(summarize_bridge_round_trip_ms(&[]), None);
        assert_eq!(summarize_bridge_round_trip_ms(&[f64::NAN]), None);
    }

    #[test]
    fn metric_event_redacts_sidecar_token_values_inside_safe_strings() {
        let metric = metric_event(
            TraceId("trace-3".to_string()),
            CHANNEL_EVENT_DROPPED_METRIC,
            1.0,
            MetricUnit::Count,
            Some(json!({
                "message": "callback rejected token vscode-atomic-0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
                "safe": "vscode-atomic-not-a-token"
            })),
        );

        let context = metric.context.expect("redacted context");

        assert_eq!(context["message"], "callback rejected token <redacted>");
        assert_eq!(context["safe"], "vscode-atomic-not-a-token");
    }

    #[test]
    fn redact_env_value_redacts_sidecar_token_only() {
        let token = "vscode-atomic-secret-token";

        assert_eq!(
            redact_env_value(SIDECAR_TOKEN_ENV_NAME, token),
            REDACTED_SECRET
        );
        assert_eq!(redact_env_value("PATH", token), token);
    }

    #[test]
    fn runtime_dependency_preflight_event_serializes_schema() {
        let event = runtime_dependency_preflight_event(
            TraceId("trace-preflight-1".to_string()),
            RuntimeDependencyKind::GtkGlibWebKitGtk,
            vec![
                RuntimeDependencyPreflightCheck::pass("gtk4", Some("4.14.0")),
                RuntimeDependencyPreflightCheck::pass("glib-2.0", Some("2.80.0")),
                RuntimeDependencyPreflightCheck::fail(
                    "webkitgtk-6.0",
                    "pkg-config could not find webkitgtk-6.0",
                ),
            ],
        );

        let serialized = serde_json::to_value(&event).expect("serialize preflight event");

        assert_eq!(serialized["traceId"], "trace-preflight-1");
        assert_eq!(serialized["name"], "runtimeDependency.preflight");
        assert_eq!(serialized["dependency"], "gtk-glib-webkitgtk");
        assert_eq!(serialized["status"], "fail");
        assert_eq!(serialized["summary"]["status"], "fail");
        assert_eq!(serialized["summary"]["passed"], 2);
        assert_eq!(serialized["summary"]["failed"], 1);
        assert_eq!(serialized["summary"]["total"], 3);
        assert_eq!(
            serialized["summary"]["message"],
            "GTK/GLib/WebKitGTK preflight fail: 2/3 passed, 1 failed"
        );
        assert_eq!(serialized["checks"][0]["status"], "pass");
        assert_eq!(serialized["checks"][0]["version"], "4.14.0");
        assert_eq!(
            serialized["checks"][2]["detail"],
            "pkg-config could not find webkitgtk-6.0"
        );
    }

    #[test]
    fn runtime_dependency_preflight_summary_passes_when_no_failures() {
        let summary = summarize_runtime_dependency_preflight(
            RuntimeDependencyKind::EslintPluginImport,
            &[
                RuntimeDependencyPreflightCheck::pass("eslint", Some("9.0.0")),
                RuntimeDependencyPreflightCheck::pass("eslint-plugin-import", Some("2.29.1")),
            ],
        );

        assert_eq!(summary.status, RuntimeDependencyPreflightStatus::Pass);
        assert_eq!(summary.passed, 2);
        assert_eq!(summary.failed, 0);
        assert_eq!(summary.total, 2);
        assert_eq!(
            summary.message,
            "ESLint plugin import preflight pass: 2/2 passed, 0 failed"
        );
    }
}
