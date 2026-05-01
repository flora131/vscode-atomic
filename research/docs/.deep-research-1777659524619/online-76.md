(no external research applicable)

The `extensions/rust/` partition contains only a pure language support extension with no external library dependencies: its `package.json` declares no `dependencies` or `devDependencies` beyond the VS Code engine itself, and the extension contributes only a TextMate grammar (`syntaxes/rust.tmLanguage.json`) and a language configuration file (`language-configuration.json`). There is nothing in this partition that bears on porting core VS Code IDE functionality to Tauri/Rust, so no external documentation fetching is warranted.
