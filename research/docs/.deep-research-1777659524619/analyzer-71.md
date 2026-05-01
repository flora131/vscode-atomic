# Partition 71 Analysis: `extensions/yaml/`

## Files Analysed

| File | LOC | Role |
|------|-----|------|
| `extensions/yaml/build/update-grammar.js` | 18 | Build-time grammar fetch script |
| `extensions/yaml/package.json` | 114 | Extension manifest (declarative) |
| `extensions/yaml/language-configuration.json` | (config) | Bracket/comment/indent rules |
| `extensions/yaml/syntaxes/*.tmLanguage.json` | (data) | TextMate grammar files |

---

## Per-File Notes

### `extensions/yaml/build/update-grammar.js` (lines 1–18)

**Role:** A standalone Node.js build script invoked manually (via `npm run update-grammar`) to pull fresh TextMate grammar files from the upstream GitHub repository `RedCMD/YAML-Syntax-Highlighter`. It is not part of the VS Code runtime.

**Dependencies:**
- Line 7: `require('vscode-grammar-updater')` — an npm dev-tool package that wraps GitHub raw file download.

**Control flow:**
1. `updateGrammars()` is defined as an `async` function (line 9) and immediately invoked (line 18).
2. Inside `updateGrammars()`, six sequential `await updateGrammar.update(...)` calls are made (lines 10–15), one for each versioned YAML grammar file:
   - `yaml-1.0.tmLanguage.json` (line 10)
   - `yaml-1.1.tmLanguage.json` (line 11)
   - `yaml-1.2.tmLanguage.json` (line 12)
   - `yaml-1.3.tmLanguage.json` (line 13)
   - `yaml-embedded.tmLanguage.json` (line 14)
   - `yaml.tmLanguage.json` (line 15)
3. Each call signature is `update(repo, srcPath, destPath, undefined, 'main')`, where `undefined` is the version/tag argument (defaults to latest on `main`).
4. The script writes the downloaded JSON files into `./syntaxes/` in the extension directory and then exits. No output is registered with the VS Code extension host.

**Data flow:** Network (GitHub raw) → `vscode-grammar-updater` → local `syntaxes/*.tmLanguage.json` files on disk.

---

### `extensions/yaml/package.json` (lines 1–114)

**Role:** Declarative extension manifest. Registers two language IDs with VS Code's extension host and wires in the TextMate grammars and language configuration.

**Key declarations:**
- Lines 16–52: Two `languages` entries — `dockercompose` (line 17) and `yaml` (line 33). The `yaml` language associates file extensions `.yaml`, `.yml`, `.eyaml`, `.eyml`, `.cff`, `.yaml-tmlanguage`, `.yaml-tmpreferences`, `.yaml-tmtheme`, `.winget` (lines 39–48) and a `firstLine` pattern `^#cloud-config` (line 50).
- Lines 54–91: Seven `grammars` entries mapping TextMate scope names (`source.yaml`, `source.yaml.1.0`–`1.3`, `source.yaml.embedded`) to the JSON grammar files under `./syntaxes/`.
- Lines 92–108: `configurationDefaults` for both `[yaml]` and `[dockercompose]` set editor indentation (spaces, tabSize 2), autoIndent mode `advanced`, and enable `quickSuggestions` for strings.
- Line 9: `"engines": { "vscode": "*" }` — no minimum version constraint.
- Lines 11–13: Single script `"update-grammar"` points to `./build/update-grammar.js`.

There is no `main` activation entry point in this manifest; the extension is purely declarative (grammar + config contribution only).

---

### `extensions/yaml/language-configuration.json`

Not read in full (out of the 18-LOC scope reported by the locator), but its presence and reference in `package.json` at lines 32 and 51 confirms it carries standard bracket-pair, comment, and indentation rules consumed directly by the VS Code editor engine — no runtime code.

---

### `extensions/yaml/syntaxes/*.tmLanguage.json`

Six JSON files containing TextMate grammar rules (regex patterns, scopes, repository entries). These are static data blobs consumed by VS Code's TextMate tokenization engine (`vscode-textmate`) at runtime. They contain no executable logic.

---

## Cross-Cutting Synthesis

The `extensions/yaml/` partition is entirely a **declarative, grammar-only extension**. Its sole implementation file (`build/update-grammar.js`) is a developer utility that fetches upstream grammar artifacts from GitHub at development time; it is not linked into the VS Code runtime, extension host, or any build artifact that ships to end users.

**Relevance to a Tauri/Rust port:**

- **Nothing in this partition needs to be ported.** The TextMate grammars (`syntaxes/*.tmLanguage.json`) and language configuration (`language-configuration.json`) are plain JSON files that any editor framework consumes. A Tauri-based editor would need a TextMate tokenization library — the Rust ecosystem has `syntect` (which uses the same `.tmLanguage` format natively) — and could ingest these JSON files directly without any transformation.
- The `build/update-grammar.js` script is a development-time tool tied to the npm toolchain. An equivalent `build.rs` script or a simple Rust CLI could replicate its network-fetch behaviour if needed, but it would be run by developers, not end users.
- No TypeScript classes, VS Code API calls, IPC channels, Electron APIs, or runtime extension-host registrations exist in this partition. There are zero engine-level concerns to port.

---

## Out-of-Partition References

- `vscode-grammar-updater` (npm package, external) — used only at build time in `build/update-grammar.js:7`.
- `RedCMD/YAML-Syntax-Highlighter` (GitHub repository, external) — the upstream source of the six TextMate grammar files fetched by `update-grammar.js:10–15`.
- VS Code's TextMate tokenization engine (`vscode-textmate`, located in the VS Code core, not in this extension) — the runtime consumer of `syntaxes/*.tmLanguage.json`. That engine is the component relevant to porting; it lives in the core workbench, not here.
