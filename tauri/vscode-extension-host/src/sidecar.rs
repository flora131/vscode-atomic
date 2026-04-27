//! Stub for the extension-host sidecar manager.

/// Manages lifecycle of the extension host child process / sidecar.
pub struct ExtensionHostSidecar {
    _priv: (),
}

impl ExtensionHostSidecar {
    pub fn new() -> Self {
        Self { _priv: () }
    }
}

impl Default for ExtensionHostSidecar {
    fn default() -> Self {
        Self::new()
    }
}
