use std::{collections::BTreeSet, fs, path::Path};

#[test]
fn tauri_bridge_commands_match_rust_invoke_handlers() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let bridge_source =
        fs::read_to_string(manifest_dir.join("../src/vs/platform/tauri/common/tauriBridge.ts"))
            .expect("read tauriBridge.ts");
    let runtime_source =
        fs::read_to_string(manifest_dir.join("src/runtime.rs")).expect("read runtime.rs");
    let commands_source =
        fs::read_to_string(manifest_dir.join("src/commands.rs")).expect("read commands.rs");

    let bridge_commands = extract_tauri_bridge_commands(&bridge_source);
    let rust_handlers = extract_generate_handler_commands(&runtime_source);
    let command_functions = extract_tauri_command_functions(&commands_source);

    assert_eq!(bridge_commands, rust_handlers);
    assert_eq!(bridge_commands, command_functions);
}

fn extract_tauri_bridge_commands(source: &str) -> BTreeSet<String> {
    let body = source
        .split_once("export interface TauriBridgeCommands {")
        .and_then(|(_, rest)| rest.split_once('}'))
        .map(|(body, _)| body)
        .expect("TauriBridgeCommands interface exists");

    body.lines()
        .filter_map(|line| {
            line.trim()
                .strip_prefix("readonly ")?
                .split_once(':')
                .map(|(command, _)| command.trim().to_string())
        })
        .collect()
}

fn extract_generate_handler_commands(source: &str) -> BTreeSet<String> {
    let body = source
        .split_once("tauri::generate_handler![")
        .and_then(|(_, rest)| rest.split_once(']'))
        .map(|(body, _)| body)
        .expect("tauri::generate_handler! block exists");

    body.lines()
        .filter_map(|line| {
            line.trim()
                .trim_end_matches(',')
                .rsplit("::")
                .next()
                .map(str::trim)
                .filter(|command| !command.is_empty())
                .map(str::to_string)
        })
        .collect()
}

fn extract_tauri_command_functions(source: &str) -> BTreeSet<String> {
    let mut commands = BTreeSet::new();
    let mut previous_line_was_tauri_command = false;

    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed == "#[tauri::command]" {
            previous_line_was_tauri_command = true;
            continue;
        }

        if previous_line_was_tauri_command {
            if let Some(name) = trimmed
                .strip_prefix("pub async fn ")
                .and_then(|rest| rest.split_once('(').map(|(name, _)| name.to_string()))
            {
                commands.insert(name);
            }
            previous_line_was_tauri_command = false;
        }
    }

    commands
}
