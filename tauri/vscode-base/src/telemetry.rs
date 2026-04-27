//! telemetry.rs — port of src/vs/platform/telemetry/common/telemetry.ts.
//!
//! TelemetryService trait + NullTelemetryService (no-op default).

use std::collections::HashMap;

use serde_json::Value;

// ─────────────────────────────────────────────────────────────────────────────
// TelemetryEvent
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct TelemetryEvent {
    pub event_name: String,
    pub properties: HashMap<String, Value>,
    pub measurements: HashMap<String, f64>,
}

impl TelemetryEvent {
    pub fn new(event_name: impl Into<String>) -> Self {
        Self {
            event_name: event_name.into(),
            properties: HashMap::new(),
            measurements: HashMap::new(),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// TelemetryService trait
// ─────────────────────────────────────────────────────────────────────────────

pub trait TelemetryService: Send + Sync {
    fn publish(&self, event: TelemetryEvent) -> Result<(), String>;
    fn set_enabled(&self, enabled: bool);
    fn is_enabled(&self) -> bool;
}

// ─────────────────────────────────────────────────────────────────────────────
// NullTelemetryService
// ─────────────────────────────────────────────────────────────────────────────

/// No-op default implementation.
pub struct NullTelemetryService;

impl TelemetryService for NullTelemetryService {
    fn publish(&self, _event: TelemetryEvent) -> Result<(), String> {
        Ok(())
    }
    fn set_enabled(&self, _enabled: bool) {}
    fn is_enabled(&self) -> bool {
        false
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── Test 1: NullTelemetryService::publish returns Ok ─────────────────────
    #[test]
    fn null_telemetry_publish_returns_ok() {
        let svc = NullTelemetryService;
        let evt = TelemetryEvent::new("test/event");
        assert!(svc.publish(evt).is_ok());
    }

    // ── Test 2: NullTelemetryService is always disabled ──────────────────────
    #[test]
    fn null_telemetry_is_disabled() {
        let svc = NullTelemetryService;
        assert!(!svc.is_enabled());
    }

    // ── Test 3: set_enabled is no-op on null service ──────────────────────────
    #[test]
    fn null_telemetry_set_enabled_noop() {
        let svc = NullTelemetryService;
        svc.set_enabled(true);
        assert!(!svc.is_enabled());
    }

    // ── Test 4: TelemetryEvent carries event_name ────────────────────────────
    #[test]
    fn telemetry_event_new_sets_name() {
        let evt = TelemetryEvent::new("my/event");
        assert_eq!(evt.event_name, "my/event");
        assert!(evt.properties.is_empty());
        assert!(evt.measurements.is_empty());
    }

    // ── Test 5: TelemetryEvent with properties and measurements ──────────────
    #[test]
    fn telemetry_event_fields() {
        let mut evt = TelemetryEvent::new("perf/metric");
        evt.properties.insert("source".to_string(), serde_json::json!("editor"));
        evt.measurements.insert("duration_ms".to_string(), 42.5);
        assert_eq!(evt.properties["source"], serde_json::json!("editor"));
        assert_eq!(evt.measurements["duration_ms"], 42.5);
    }
}
