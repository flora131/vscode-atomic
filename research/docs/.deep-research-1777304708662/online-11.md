(no external research applicable)

The `extensions/vscode-api-tests/` partition contains only behavioral contract tests written against the `vscode` extension host API (using mocha and Node's assert module). No third-party libraries central to porting VS Code to Tauri/Rust are declared or exercised here; the tests are pure oracles verifying API surface compatibility, not implementations of editing, language intelligence, debugging, source control, terminal, or navigation subsystems.
