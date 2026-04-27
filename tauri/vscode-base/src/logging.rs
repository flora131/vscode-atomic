//! logging.rs — port of src/vs/platform/log/ + vscode.OutputChannel surface.
//!
//! TDD: tests written first; stubs below make them compile but fail.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::event::Emitter;

// ─────────────────────────────────────────────────────────────────────────────
// LogLevel
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Off = 0,
    Trace = 1,
    Debug = 2,
    Info = 3,
    Warning = 4,
    Error = 5,
    Critical = 6,
}

// ─────────────────────────────────────────────────────────────────────────────
// Logger trait
// ─────────────────────────────────────────────────────────────────────────────

pub trait Logger: Send + Sync {
    fn trace(&self, msg: &str, fields: Option<&HashMap<String, String>>);
    fn debug(&self, msg: &str, fields: Option<&HashMap<String, String>>);
    fn info(&self, msg: &str, fields: Option<&HashMap<String, String>>);
    fn warn(&self, msg: &str, fields: Option<&HashMap<String, String>>);
    fn error(&self, msg: &str, fields: Option<&HashMap<String, String>>);
    fn critical(&self, msg: &str, fields: Option<&HashMap<String, String>>);
    fn set_level(&self, level: LogLevel);
    fn get_level(&self) -> LogLevel;
    fn flush(&self);
}

// ─────────────────────────────────────────────────────────────────────────────
// ConsoleLogger
// ─────────────────────────────────────────────────────────────────────────────

pub struct ConsoleLogger {
    name: String,
    level: Mutex<LogLevel>,
}

impl ConsoleLogger {
    pub fn new(name: impl Into<String>, level: LogLevel) -> Self {
        Self { name: name.into(), level: Mutex::new(level) }
    }

    fn should_emit(&self, msg_level: LogLevel) -> bool {
        let current = *self.level.lock().unwrap();
        current != LogLevel::Off && msg_level >= current
    }
}

impl Logger for ConsoleLogger {
    fn trace(&self, msg: &str, _fields: Option<&HashMap<String, String>>) {
        if self.should_emit(LogLevel::Trace) {
            tracing::trace!(logger = %self.name, "{}", msg);
        }
    }
    fn debug(&self, msg: &str, _fields: Option<&HashMap<String, String>>) {
        if self.should_emit(LogLevel::Debug) {
            tracing::debug!(logger = %self.name, "{}", msg);
        }
    }
    fn info(&self, msg: &str, _fields: Option<&HashMap<String, String>>) {
        if self.should_emit(LogLevel::Info) {
            tracing::info!(logger = %self.name, "{}", msg);
        }
    }
    fn warn(&self, msg: &str, _fields: Option<&HashMap<String, String>>) {
        if self.should_emit(LogLevel::Warning) {
            tracing::warn!(logger = %self.name, "{}", msg);
        }
    }
    fn error(&self, msg: &str, _fields: Option<&HashMap<String, String>>) {
        if self.should_emit(LogLevel::Error) {
            tracing::error!(logger = %self.name, "{}", msg);
        }
    }
    fn critical(&self, msg: &str, _fields: Option<&HashMap<String, String>>) {
        if self.should_emit(LogLevel::Critical) {
            tracing::error!(logger = %self.name, "[CRITICAL] {}", msg);
        }
    }
    fn set_level(&self, level: LogLevel) {
        *self.level.lock().unwrap() = level;
    }
    fn get_level(&self) -> LogLevel {
        *self.level.lock().unwrap()
    }
    fn flush(&self) {}
}

// ─────────────────────────────────────────────────────────────────────────────
// OutputChannel
// ─────────────────────────────────────────────────────────────────────────────

/// Mirror of vscode.OutputChannel surface.
pub struct OutputChannel {
    pub name: String,
    lines: Mutex<Vec<String>>,
    on_did_change: Emitter<()>,
}

impl OutputChannel {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            lines: Mutex::new(Vec::new()),
            on_did_change: Emitter::new(),
        }
    }

    /// Append raw text (no newline).
    pub fn append(&self, value: &str) {
        self.lines.lock().unwrap().push(value.to_string());
        self.on_did_change.fire(&());
    }

    /// Append text followed by newline.
    pub fn append_line(&self, value: &str) {
        self.lines.lock().unwrap().push(format!("{}\n", value));
        self.on_did_change.fire(&());
    }

    /// Replace all content.
    pub fn replace(&self, value: &str) {
        let mut guard = self.lines.lock().unwrap();
        guard.clear();
        guard.push(value.to_string());
        drop(guard);
        self.on_did_change.fire(&());
    }

    /// Clear all content.
    pub fn clear(&self) {
        self.lines.lock().unwrap().clear();
        self.on_did_change.fire(&());
    }

    /// Snapshot of all stored lines.
    pub fn snapshot(&self) -> Vec<String> {
        self.lines.lock().unwrap().clone()
    }

    /// Subscribe to change events. Returns a handle; drop to unsubscribe.
    pub fn on_did_change(&self, f: impl Fn(&()) + Send + Sync + 'static) -> crate::event::ListenerHandle<()> {
        self.on_did_change.add_listener(f)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// LoggerService
// ─────────────────────────────────────────────────────────────────────────────

pub struct LoggerService {
    loggers: Mutex<HashMap<String, Arc<dyn Logger>>>,
    default_level: LogLevel,
}

impl LoggerService {
    pub fn new(default_level: LogLevel) -> Self {
        Self { loggers: Mutex::new(HashMap::new()), default_level }
    }

    pub fn get_logger(&self, name: &str) -> Arc<dyn Logger> {
        let mut map = self.loggers.lock().unwrap();
        map.entry(name.to_string())
            .or_insert_with(|| Arc::new(ConsoleLogger::new(name, self.default_level)))
            .clone()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    // ── Test 1: ConsoleLogger::get_level reflects set_level ──────────────────
    #[test]
    fn console_logger_set_level_reflected() {
        let logger = ConsoleLogger::new("test", LogLevel::Trace);
        assert_eq!(logger.get_level(), LogLevel::Trace);
        logger.set_level(LogLevel::Info);
        assert_eq!(logger.get_level(), LogLevel::Info);
    }

    // ── Test 2: set_level(Info) — should_emit respects threshold ─────────────
    // Trace should NOT emit when level=Info; Info SHOULD emit.
    #[test]
    fn console_logger_respects_level_threshold() {
        let logger = ConsoleLogger::new("lvl_test", LogLevel::Info);
        // should_emit is private; verify via public API surface
        assert!(!logger.should_emit(LogLevel::Trace));
        assert!(!logger.should_emit(LogLevel::Debug));
        assert!(logger.should_emit(LogLevel::Info));
        assert!(logger.should_emit(LogLevel::Warning));
        assert!(logger.should_emit(LogLevel::Error));
        assert!(logger.should_emit(LogLevel::Critical));
    }

    // ── Test 3: Off level suppresses everything ───────────────────────────────
    #[test]
    fn console_logger_off_suppresses_all() {
        let logger = ConsoleLogger::new("off_test", LogLevel::Off);
        for level in [
            LogLevel::Trace,
            LogLevel::Debug,
            LogLevel::Info,
            LogLevel::Warning,
            LogLevel::Error,
            LogLevel::Critical,
        ] {
            assert!(!logger.should_emit(level));
        }
    }

    // ── Test 4: OutputChannel append_line stores content ─────────────────────
    #[test]
    fn output_channel_append_line_stores() {
        let ch = OutputChannel::new("test-channel");
        ch.append_line("hello");
        ch.append_line("world");
        let snap = ch.snapshot();
        assert_eq!(snap, vec!["hello\n", "world\n"]);
    }

    // ── Test 5: OutputChannel append_line fires change event ─────────────────
    #[test]
    fn output_channel_append_line_fires_change() {
        let ch = OutputChannel::new("event-test");
        let fired = Arc::new(Mutex::new(0u32));
        let f = fired.clone();
        let _handle = ch.on_did_change(move |_| *f.lock().unwrap() += 1);
        ch.append_line("line1");
        ch.append_line("line2");
        assert_eq!(*fired.lock().unwrap(), 2);
    }

    // ── Test 6: OutputChannel clear empties lines and fires event ────────────
    #[test]
    fn output_channel_clear_empties_and_fires() {
        let ch = OutputChannel::new("clear-test");
        ch.append_line("x");
        let fired = Arc::new(Mutex::new(0u32));
        let f = fired.clone();
        let _h = ch.on_did_change(move |_| *f.lock().unwrap() += 1);
        ch.clear();
        assert!(ch.snapshot().is_empty());
        assert_eq!(*fired.lock().unwrap(), 1);
    }

    // ── Test 7: OutputChannel replace replaces content ───────────────────────
    #[test]
    fn output_channel_replace_replaces_content() {
        let ch = OutputChannel::new("replace-test");
        ch.append_line("old");
        ch.replace("new");
        assert_eq!(ch.snapshot(), vec!["new"]);
    }

    // ── Test 8: LoggerService returns same Arc for same name ─────────────────
    #[test]
    fn logger_service_same_name_same_arc() {
        let svc = LoggerService::new(LogLevel::Info);
        let a = svc.get_logger("foo");
        let b = svc.get_logger("foo");
        assert!(Arc::ptr_eq(&a, &b));
    }

    // ── Test 9: LoggerService different names different loggers ──────────────
    #[test]
    fn logger_service_different_names() {
        let svc = LoggerService::new(LogLevel::Info);
        let a = svc.get_logger("a");
        let b = svc.get_logger("b");
        assert!(!Arc::ptr_eq(&a, &b));
    }
}
