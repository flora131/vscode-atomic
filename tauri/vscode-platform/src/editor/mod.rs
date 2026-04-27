//! editor — server-side Monaco state authority.
//!
//! Architecture:
//!   - TextModel (text_model.rs)  — authoritative document state, apply_edits, version counter
//!   - RopeBuffer (buffer.rs)     — ropey-backed efficient storage (piece-tree port deferred)
//!   - ModelDecorationsTracker (decorations.rs) — decoration registry synced to renderer
//!   - Position / Range / edit / undo_stack — supporting types
//!
//! Renderer lives in the Tauri WebView (TypeScript Monaco). State changes flow via
//! Tauri events: renderer→Rust (edit operations) and Rust→renderer (decorations, diagnostics).
//! See tauri/docs/monaco-integration.md for full architecture.

pub mod buffer;
pub mod decorations;
pub mod edit;
pub mod model;
pub mod position;
pub mod range;
pub mod text_model;
pub mod undo_stack;

// Core types
pub use buffer::RopeBuffer;
pub use decorations::{
    Decoration, DecorationId, DecorationOptions, DecorationRange, ModelDecorationsTracker,
};
pub use edit::{EndOfLineSequence, SingleEditOperation};
pub use model::{ModelChangedEvent, TextEdit, TextModel as LegacyTextModel};
pub use position::Position;
pub use range::Range;
pub use text_model::{TextModel, TextModelChangedEvent, TextModelContentChange};
pub use undo_stack::{UndoEntry, UndoStack};
