(no external research applicable)

The scope (`extensions/yaml/`, 1 file, 18 LOC) contains only a declarative TextMate grammar or language configuration file — pure static JSON/YAML with no TypeScript logic, no Electron API calls, and no runtime behavior — so there are no external library docs relevant to porting it to Tauri/Rust; it would simply be copied or registered verbatim in whatever editor host replaces VS Code's extension host.
