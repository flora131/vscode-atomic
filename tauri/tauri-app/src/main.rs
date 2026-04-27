//! tauri-app — Tauri shell entry point.
//!
//! Replaces Electron main (src/main.ts, src/vs/code/electron-main/app.ts)
//! with a Tauri 2 builder.  All platform services will be injected via
//! vscode-platform's ServiceRegistry in subsequent tasks.

// Prevent a console window from appearing on Windows release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
