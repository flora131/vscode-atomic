(no external research applicable)

The `extensions/gulp/` partition contains a thin VS Code extension that auto-detects Gulp tasks by shelling out to the `gulp` CLI. Its only dependencies are the VS Code extension host API (`vscode`) and Node.js built-ins (`path`, `fs`, `child_process`); there are no third-party npm packages and no Rust/Tauri surface. None of these dependencies are central to the question of porting VS Code's core IDE functionality to Tauri/Rust.
