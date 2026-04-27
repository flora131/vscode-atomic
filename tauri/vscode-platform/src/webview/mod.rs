//! webview — Webview/WebviewPanel infrastructure for vscode-platform.
//!
//! Mirrors vscode.d.ts WebviewPanel/Webview interfaces (lines 10083-10164).
//! CSP form mirrors extensions/media-preview/src/videoPreview.ts:56-92.

pub mod csp;
pub mod registry;
pub mod uri_scheme;
pub mod webview;

pub use csp::{build_csp, generate_nonce, CspExtras};
pub use registry::WebviewRegistry;
pub use uri_scheme::{resolve_webview_uri, WebviewError};
pub use webview::{ViewColumn, Webview, WebviewOptions, WebviewPanel, WebviewPanelOptions};
