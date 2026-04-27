//! editor — server-side Monaco state authority.
//!
//! Architecture:
//!   - TextModel (model.rs)  — authoritative document state, apply_edits, version counter
//!   - RopeBuffer (buffer.rs) — ropey-backed efficient storage (piece-tree port deferred)
//!   - ModelDecorationsTracker (decorations.rs) — decoration registry synced to renderer
//!
//! Renderer lives in the Tauri WebView (TypeScript Monaco). State changes flow via
//! Tauri events: renderer→Rust (edit operations) and Rust→renderer (decorations, diagnostics).
//! See tauri/docs/monaco-integration.md for full architecture.

pub mod buffer;
pub mod decorations;
pub mod model;

pub use buffer::RopeBuffer;
pub use decorations::{
    Decoration, DecorationId, DecorationOptions, DecorationRange, ModelDecorationsTracker,
};
pub use model::{ModelChangedEvent, TextEdit, TextModel};
