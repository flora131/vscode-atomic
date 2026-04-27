# Partition 70 — extensions/less/

(no external research applicable)

The `extensions/less/` directory is a purely declarative VS Code extension that contributes a TextMate grammar (`less.tmLanguage.json`), a `language-configuration.json`, and a `lessc` problem-matcher pattern — all expressed as static JSON. It contains no TypeScript runtime code, no Node.js or Electron API calls, and no npm library dependencies. Because the extension is entirely data-driven (grammar scopes, bracket/comment rules, regex patterns), porting it in the context of a Tauri/Rust IDE amounts to nothing more than shipping the same JSON files alongside whatever grammar-loading mechanism the new host provides (e.g., tree-sitter or a TextMate-compatible engine). No external library documentation is central to understanding or executing that work.
