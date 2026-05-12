use std::{thread, time::Duration};

use vscode_atomic_tauri::terminal_service::{
    ResizeTerminalRequest, RustTerminalService, SpawnTerminalRequest, TerminalLifecycleState,
    TerminalStreamKind,
};

#[cfg(windows)]
fn output_command() -> (String, Vec<String>) {
    (
        "cmd".to_string(),
        vec!["/C".to_string(), "echo terminal-parity".to_string()],
    )
}

#[cfg(not(windows))]
fn output_command() -> (String, Vec<String>) {
    (
        "sh".to_string(),
        vec!["-c".to_string(), "printf terminal-parity".to_string()],
    )
}

#[test]
fn default_terminal_backend_spawns_and_relays_output() {
    let service = RustTerminalService::default();
    let (command, args) = output_command();

    let status = service
        .spawn(SpawnTerminalRequest {
            id: Some("parity-term".to_string()),
            command,
            args,
            cwd: None,
            env: Default::default(),
            cols: Some(80),
            rows: Some(24),
        })
        .expect("spawn succeeds");

    assert_eq!(status.id, "parity-term");
    assert_eq!(status.state, TerminalLifecycleState::Running);

    let resized = service
        .resize(ResizeTerminalRequest {
            id: "parity-term".to_string(),
            cols: 100,
            rows: 30,
        })
        .expect("resize succeeds");
    assert_eq!(resized.size.expect("size present").cols, 100);

    let mut output = String::new();
    for _ in 0..40 {
        for event in service
            .drain_events("parity-term")
            .expect("drain events succeeds")
        {
            if event.stream == TerminalStreamKind::Stdout {
                output.push_str(&event.data);
            }
        }
        if output.contains("terminal-parity") {
            break;
        }
        thread::sleep(Duration::from_millis(25));
    }

    assert!(output.contains("terminal-parity"));
}
