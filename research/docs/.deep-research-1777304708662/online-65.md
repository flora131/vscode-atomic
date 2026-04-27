# Online Research: extensions/json/ — Port to Tauri/Rust

(no external research applicable)

## Justification

The `extensions/json/` directory (39 LOC across its meaningful files) is entirely declarative and contains no runtime TypeScript or Electron-specific logic. Its contents are:

- `package.json` — VS Code extension manifest. Declares language IDs (`json`, `jsonc`, `jsonl`, `snippets`), their file-extension associations, MIME types, and pointers to grammar files. No runtime code.
- `language-configuration.json` — Declarative bracket/comment/indentation rules for the JSON language. No runtime code.
- `syntaxes/JSON.tmLanguage.json`, `JSONC.tmLanguage.json`, `JSONL.tmLanguage.json`, `snippets.tmLanguage.json` — TextMate grammar files in JSON format. Pure declarative regex-based tokenization rules. No runtime code.
- `build/update-grammars.js` — A one-off build-time Node script that pulls upstream grammar files from `microsoft/vscode-JSON.tmLanguage` and `jeff-hykin/better-snippet-syntax` via the `vscode-grammar-updater` npm package, then writes them locally. This script is never shipped or executed at runtime; it is only run by a developer to refresh vendored grammar files.
- `.vscodeignore`, `cgmanifest.json`, `package.nls.json` — Packaging and localization metadata. No runtime code.

None of these artifacts have any dependency on Electron, Node.js APIs, or VS Code's TypeScript runtime host. They are static data files consumed by whatever syntax-highlighting engine the target platform provides. In a Tauri/Rust port, TextMate grammars are supported directly by editors such as Zed (via `tree-sitter` or `syntect`) or by any Tauri webview front-end that embeds a Monaco/CodeMirror instance — in all cases the `.tmLanguage.json` files are loaded as-is without modification. The build script (`update-grammars.js`) likewise requires no porting: it is a developer convenience tool that merely downloads files, and its `vscode-grammar-updater` dependency is a generic npm utility with no VS Code host coupling.

No external library documentation needs to be consulted because the porting work for this scope is limited to:

1. Copying the four `.tmLanguage.json` grammar files verbatim into the Tauri project's asset bundle.
2. Copying `language-configuration.json` verbatim.
3. Registering the language IDs and file-extension associations in whatever plugin/extension manifest format the Tauri front-end framework uses (replacing the VS Code `package.json` contributes schema with the target framework's equivalent).
4. Optionally retaining or discarding `build/update-grammars.js` as a developer tool — no porting required either way.

All of these steps are mechanical file operations or trivial manifest translations. No API research, no library migration, and no Rust implementation work is needed for this scope.
