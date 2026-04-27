//! Stub for the headless VS Code server.

/// Headless web server stub (will use Hyper/axum when implemented).
pub struct HeadlessServer {
    _priv: (),
}

impl HeadlessServer {
    pub fn new() -> Self {
        Self { _priv: () }
    }
}

impl Default for HeadlessServer {
    fn default() -> Self {
        Self::new()
    }
}
