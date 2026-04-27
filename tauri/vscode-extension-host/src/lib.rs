//! vscode-extension-host — sidecar bridge to the Node.js extension host.
//!
//! Placeholder. Full implementation delivered in task #8 (Implementing extension
//! host sidecar).  Mirrors the bootstrap-fork.ts → Tauri sidecar spawn pattern
//! and cli/src/json_rpc.rs framing.

pub mod sidecar;

pub use sidecar::ExtensionHostSidecar;
