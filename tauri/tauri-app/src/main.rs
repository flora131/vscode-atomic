//! tauri-app — Tauri shell entry point.
//!
//! Replaces Electron main (src/main.ts, src/vs/code/electron-main/app.ts)
//! with a Tauri 2 builder.  All platform services will be injected via
//! vscode-platform's ServiceRegistry in subsequent tasks.

// Prevent a console window from appearing on Windows release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};

// ─────────────────────────────────────────────────────────────────────────────
// Shared LSP types
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct LspPosition {
    pub line: u32,
    pub character: u32,
}

#[derive(Deserialize)]
pub struct LspRange {
    pub start: LspPosition,
    pub end: LspPosition,
}

// ─────────────────────────────────────────────────────────────────────────────
// Workbench view containers
// ─────────────────────────────────────────────────────────────────────────────

/// A view container entry returned to the frontend workbench.
/// Mirrors the `ViewContainer` TypeScript interface in
/// tauri/frontend/src/workbench/types.ts.
#[derive(Serialize, Clone)]
pub struct ViewContainer {
    pub id: &'static str,
    pub name: &'static str,
    pub icon: &'static str,
}

/// Stub handler for `workbench_view_containers` IPC command.
///
/// Returns a static set of built-in view containers (Explorer, Search, SCM,
/// Run & Debug, Extensions).  In a later task this will be driven by the
/// real vscode-platform ServiceRegistry.
#[tauri::command]
fn workbench_view_containers() -> Vec<ViewContainer> {
    vec![
        ViewContainer { id: "explorer",  name: "Explorer",           icon: "$(files)"          },
        ViewContainer { id: "search",    name: "Search",             icon: "$(search)"         },
        ViewContainer { id: "scm",       name: "Source Control",     icon: "$(source-control)" },
        ViewContainer { id: "debug",     name: "Run and Debug",      icon: "$(debug-alt)"      },
        ViewContainer { id: "extensions",name: "Extensions",         icon: "$(extensions)"     },
    ]
}

// ─────────────────────────────────────────────────────────────────────────────
// Language feature stubs — delegate to LanguageFeatureRegistry (future task)
// Each returns an empty/null result until the real registry is wired up.
// ─────────────────────────────────────────────────────────────────────────────

/// Stub: completion items for a document position.
#[tauri::command]
fn language_feature_completion(uri: String, position: LspPosition) -> serde_json::Value {
    let _ = (uri, position);
    serde_json::json!({ "suggestions": [] })
}

/// Stub: hover information for a document position.
#[tauri::command]
fn language_feature_hover(uri: String, position: LspPosition) -> Option<serde_json::Value> {
    let _ = (uri, position);
    None
}

/// Stub: go-to-definition locations.
#[tauri::command]
fn language_feature_definition(uri: String, position: LspPosition) -> Vec<serde_json::Value> {
    let _ = (uri, position);
    vec![]
}

/// Stub: find-references locations.
#[tauri::command]
fn language_feature_references(uri: String, position: LspPosition) -> Vec<serde_json::Value> {
    let _ = (uri, position);
    vec![]
}

/// Stub: code actions for a document range.
#[tauri::command]
fn language_feature_code_actions(uri: String, range: LspRange) -> serde_json::Value {
    let _ = (uri, range);
    serde_json::json!({ "actions": [] })
}

/// Stub: rename edits for a symbol.
#[tauri::command]
fn language_feature_rename(uri: String, position: LspPosition, new_name: String) -> serde_json::Value {
    let _ = (uri, position, new_name);
    serde_json::json!({ "edits": [] })
}

/// Stub: signature help for a function call.
#[tauri::command]
fn language_feature_signature_help(uri: String, position: LspPosition) -> Option<serde_json::Value> {
    let _ = (uri, position);
    None
}

/// Stub: document formatting edits.
#[tauri::command]
fn language_feature_formatting(uri: String) -> Vec<serde_json::Value> {
    let _ = uri;
    vec![]
}

/// Stub: document symbol outline.
#[tauri::command]
fn language_feature_document_symbols(uri: String) -> Vec<serde_json::Value> {
    let _ = uri;
    vec![]
}

/// Stub: semantic tokens for a document.
#[tauri::command]
fn language_feature_semantic_tokens(uri: String) -> Option<serde_json::Value> {
    let _ = uri;
    None
}

// ─────────────────────────────────────────────────────────────────────────────
// Terminal IPC stubs — wire to PTY backend (vscode-platform) in future task
// ─────────────────────────────────────────────────────────────────────────────

/// Stub: write data to a PTY session's stdin.
/// TODO: route to vscode-platform PTY registry by `id`.
#[tauri::command]
fn terminal_write(id: String, data: String) {
    let _ = (id, data);
}

// ─────────────────────────────────────────────────────────────────────────────
// Entry point
// ─────────────────────────────────────────────────────────────────────────────

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            workbench_view_containers,
            language_feature_completion,
            language_feature_hover,
            language_feature_definition,
            language_feature_references,
            language_feature_code_actions,
            language_feature_rename,
            language_feature_signature_help,
            language_feature_formatting,
            language_feature_document_symbols,
            language_feature_semantic_tokens,
            terminal_write,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
