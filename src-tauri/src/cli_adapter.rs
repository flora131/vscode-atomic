/// Minimal bridge proving the Tauri runtime can consume the existing Rust CLI crate in-process.
pub fn protocol_version() -> u32 {
    cli::constants::PROTOCOL_VERSION
}
