(no external research applicable)

The `extensions/html/` partition in VS Code contains only TextMate grammar definitions and snippet files for HTML syntax highlighting and code snippets; it carries no third-party library dependencies that would require research or porting consideration when moving VS Code's extension host from Electron to a Tauri/Rust backend.

This partition is grammar and snippets only. All of its content consists of static JSON/YAML grammar files (`syntaxes/`) and snippet definition files (`snippets/`), which are declarative data consumed by VS Code's extension infrastructure at runtime. There are no npm dependencies, no native bindings, no IPC surfaces, and no platform-specific code paths. Because the Tauri/Rust port affects the host process and native integration layer — not the passive data files loaded by the extension host — no external research into third-party libraries, APIs, or porting strategies is needed for this partition.
