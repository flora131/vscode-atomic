(no external research applicable)

The scope for partition 76 is `src/bootstrap-cli.ts`, an 11-line CLI bootstrap shim whose sole behavior is deleting the `VSCODE_CWD` environment variable before delegating to the main entry point. It uses no external libraries and has no surface area relevant to porting VS Code's core IDE functionality to Tauri/Rust.
