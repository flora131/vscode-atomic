(no external research applicable)

The `extensions/sql/` partition contains only a declarative TextMate grammar extension with no external npm dependencies, no Rust/Tauri components, and no executable logic — its `package.json` lists no `dependencies` or `devDependencies` beyond the VS Code engine peer requirement, so there are no library docs that bear on porting VS Code's core IDE functionality to Tauri/Rust.
