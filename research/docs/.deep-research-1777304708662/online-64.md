# Online Research: extensions/razor/ — Tauri/Rust Port Assessment

(no external research applicable)

## Justification

The `extensions/razor/` directory (7 files total, with the scope covering 44 LOC of declarative content) consists entirely of static, declarative configuration assets:

- `syntaxes/cshtml.tmLanguage.json` — A TextMate grammar file defining syntax highlighting rules for Razor/CSHTML files (`.cshtml`, `.razor`). This is a pure JSON data structure with no executable code.
- `language-configuration.json` — A VS Code language configuration file declaring bracket pairs, auto-closing pairs, comment markers, and surrounding pairs. Again, pure JSON with no logic.
- `package.json` — Extension manifest declaring language IDs, file extensions, MIME types, and grammar contribution points. Declarative only.

None of these files contain TypeScript, JavaScript runtime logic, Node.js API calls, Electron APIs, or any imperative code that would need to be ported. There is no dependency on VS Code's extension host runtime beyond the standard language/grammar contribution point mechanism, which is a universal feature of VS Code's extension model.

### Port Implications

When porting VS Code to Tauri/Rust, the handling of TextMate grammars and language configurations is managed by the editor core (the `vscode-textmate` library and the language service infrastructure), not by individual language extension packages. The JSON grammar and configuration files themselves are format-stable and editor-agnostic — they would be consumed unchanged by whatever TextMate grammar engine is integrated into the Tauri-based editor.

The `build/update-grammar.mjs` script is a one-time dev tooling script for updating the grammar from an upstream source; it is not part of the runtime and has no bearing on the port.

No external library documentation is relevant to porting this scope. The files require no code translation, no API mapping, and no Rust equivalents — they are data files that any compatible editor engine consumes directly.
