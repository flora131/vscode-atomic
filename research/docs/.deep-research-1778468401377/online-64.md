# External Research Assessment: `extensions/json/`

## Verdict

**(no external research applicable)**

## Justification

The `extensions/json/` partition is purely a **declarative grammar extension**. Its entire contents are static configuration and data files with no runtime TypeScript or Rust logic of their own:

- `package.json` — VS Code extension manifest declaring language IDs (json, jsonc, jsonl, snippets), file-extension mappings, and grammar contribution points. No runtime dependencies are listed.
- `language-configuration.json` — Declarative bracket/comment/indentation rules consumed by the VS Code language server host at load time. Plain JSON; no code.
- `syntaxes/JSON.tmLanguage.json`, `JSONC.tmLanguma.json`, `JSONL.tmLanguage.json`, `snippets.tmLanguage.json` — Static TextMate grammar files (converted from `microsoft/vscode-JSON.tmLanguage`). These are inert JSON blobs; they are *consumed by* a tokenizer engine but contain no engine code themselves.
- `build/update-grammars.js` — A **build-time-only** helper that uses `vscode-grammar-updater` to pull fresh copies of the upstream `.tmLanguage` files from GitHub. It runs during development/CI, not at IDE runtime. It is not shipped with the extension.
- `cgmanifest.json` — Component governance metadata (upstream repo URLs + commit hashes for the two grammar sources). Also build/audit-time only.

### Why the TextMate tokenizer is out of scope here

The TextMate grammar *engine* (`vscode-textmate` + `vscode-oniguruma` / the Oniguruma WASM binding) lives in VS Code core (`src/vs/workbench/services/textMate/`) and the `vscode-textmate` npm package, **not** inside `extensions/json/`. When porting to Tauri/Rust the question of *which engine tokenizes these grammars* is a core-infrastructure concern — a Rust implementation such as `syntect` (which has native Oniguruma/PCRE2 support and reads `.tmLanguage` files directly) would replace `vscode-textmate` at the platform layer. That research belongs to the core tokenizer partition, not here.

The JSON grammar files themselves are format-stable: any TextMate-compatible engine (whether `vscode-textmate` in Node, `syntect` in Rust, or any other compliant engine) consumes the same `.tmLanguage.json` files without modification. No source transformation of these files is required by the port.

### What the port actually requires for this partition

| File / artifact | Port action |
|---|---|
| `syntaxes/*.tmLanguage.json` | Copy as-is into the Tauri bundle; consumed unchanged by whatever TextMate engine is used. |
| `language-configuration.json` | Copy as-is; the Tauri IDE host must implement a language-configuration loader (identical concern to the generic extension host, not JSON-specific). |
| `package.json` contributions block | Translate declarative language/grammar registration into whatever extension-manifest format the Tauri IDE host uses. No external library needed. |
| `build/update-grammars.js` | Keep or replace with an equivalent build-time script; purely a developer tooling concern. |

No runtime library is introduced or exercised by this partition. The only external tool, `vscode-grammar-updater`, is a build-time Node utility for fetching upstream grammar files, and it has no bearing on the Tauri runtime at all.

## Summary

`extensions/json/` is a thin declarative package: four static TextMate grammar JSON files, a language-configuration JSON file, and a build-only grammar-update script. It contains no runtime logic, no runtime npm dependencies, and no Rust-facing concerns. The TextMate tokenizer engine that will eventually process these grammars is a core-layer concern handled elsewhere in the port. For this specific partition, no external library research is needed — the files are copied as-is, and the only porting work is registering them with the new IDE host's extension manifest system.
