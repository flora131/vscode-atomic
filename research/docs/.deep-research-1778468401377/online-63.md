# extensions/razor — TextMate Grammar Extension: External Research Assessment

(no external research applicable)

## Justification

The `extensions/razor/` directory contains a minimal TextMate grammar extension totalling approximately 44 lines of meaningful source code across three functional files:

- `build/update-grammar.mjs` — a build-time script (~43 LOC) that uses `vscode-grammar-updater` to fetch the upstream Razor grammar from `dotnet/roslyn` and patch one scope-name reference (`text.html.basic` → `text.html.derivative`).
- `syntaxes/cshtml.tmLanguage.json` — the vendored TextMate grammar JSON blob (auto-generated; not hand-authored).
- `language-configuration.json` — bracket/comment/pair metadata consumed by VS Code's language service (~22 LOC).

### Why no external library research is needed for porting

**The build script is not ported — it is discarded.** `vscode-grammar-updater` is a Node.js developer tooling helper used only at grammar-update time to pull a fresh copy of the upstream `.tmLanguage.json` from the roslyn repository. It has no runtime presence. In a Tauri/Rust port the equivalent would be a one-time manual download or a trivial shell script; there is nothing to port and no Rust equivalent is needed.

**The grammar file is static JSON — it moves as-is.** The `cshtml.tmLanguage.json` file is a serialised TextMate grammar. Its format (a JSON representation of the TextMate PList grammar schema) is consumed at runtime by whichever syntax-highlighting engine the new host provides. Whether that engine is VS Code's `vscode-textmate` (JavaScript), the Rust crate `syntect` (which natively reads `.tmLanguage` / `.tmLanguage.json`), or a tree-sitter grammar is a decision for the host editor layer, not for this extension. The file itself requires no transformation and no library research.

**The language-configuration.json is also static.** It declares bracket pairs and auto-closing rules in a generic JSON schema. Whatever Tauri-based editor host is built will need to consume or re-express this data in its own configuration format, but that is a host-side concern, not an extension-library concern.

**TextMate grammar format is not "broadly relevant" at the library-research level for this scope.** While the overall porting project may eventually need to evaluate whether to continue using `vscode-textmate` (via a WebView/WASM boundary) or adopt `syntect` in Rust for syntax highlighting, that decision sits in the core editor host layer, not in this extension. The extension contributes only a data file and a build helper; neither has library dependencies that require investigation for the port.

### Summary

`extensions/razor/` presents no porting challenge that warrants external library research. The vendored `.tmLanguage.json` grammar file is static data that travels unchanged to any target host. The `vscode-grammar-updater` build dependency is pure developer tooling that is thrown away rather than ported. No Rust crate, Tauri API, tree-sitter grammar, or Oniguruma binding needs to be evaluated for this extension specifically; any such evaluation belongs to the host-layer syntax-highlighting subsystem analysis, which is outside this extension's scope.
