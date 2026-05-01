# Partition 64 of 79 — Findings

## Scope
`extensions/razor/` (1 files, 44 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Razor Extension - File Location Report

## Scope
`extensions/razor/` — 7 files (1 directory with 44 LOC total)

## Findings

### Configuration & Manifest
- `extensions/razor/package.json` — Extension manifest declaring Razor language support for `.cshtml` and `.razor` files; registers syntax highlighting and language configuration contributions
- `extensions/razor/package.nls.json` — Localization file providing display name and description translations for the extension
- `extensions/razor/language-configuration.json` — Language configuration defining comments, bracket pairs, auto-closing, and surrounding pairs for Razor files
- `extensions/razor/cgmanifest.json` — Component governance manifest referencing the dotnet/razor repository (commit hash 743f32a68c61809b22fd84e8748c3686ef1bb8b8) as a licensed dependency

### Grammar & Syntax
- `extensions/razor/syntaxes/cshtml.tmLanguage.json` — TextMate grammar for CSHTML/Razor syntax highlighting; supports embedded C#, CSS, and JavaScript languages

### Build Tools
- `extensions/razor/build/update-grammar.mjs` — Build script (44 LOC) that updates the grammar from the dotnet/razor repository; patches grammar rules to replace `text.html.basic` with `text.html.derivative` includes

### Metadata
- `extensions/razor/.vscodeignore` — Excludes test, build, and cgmanifest files from the published extension package

## Summary

The `extensions/razor/` directory is a pure language grammar extension with no runtime code or backend service. It provides syntax highlighting and language support for Razor (`.cshtml`, `.razor`) files by:

1. **Grammar source**: Maintains a TextMate grammar pulled from the dotnet/razor repository via an automated build script
2. **Language registration**: Declares Razor as a VS Code language with proper aliases and MIME types
3. **Editor configuration**: Defines bracket matching, auto-close behavior, and comment patterns for the Razor syntax
4. **No IDE functionality**: This extension contains only grammar definitions and configuration—it provides syntax highlighting only, with no language services, IntelliSense, diagnostics, or debugging capabilities

**Relevance to Tauri/Rust port**: This extension would require minimal porting. A Rust-based VSCodium alternative would need:
- To preserve or regenerate the TextMate grammar (language-agnostic, can stay as-is)
- To reimplement the extension API hooks (less critical than full IDE functionality)
- A Rust-based language service provider if enhanced Razor support (IntelliSense, diagnostics) is desired in the future

Current scope is declarative grammar and configuration, not executable code—making it a low-priority item for a Tauri/Rust IDE rewrite.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
# Partition 64: `extensions/razor/` — Grammar-Only Extension

**Sentinel notice:** This partition contains no IDE runtime code. All files are declarative data (JSON) or a one-shot build utility script. Nothing here needs to be ported in the traditional sense; the grammar file is consumed by the host's TextMate tokenisation engine at runtime.

---

### Files Analysed

| File | Lines | Kind |
|------|------:|------|
| `extensions/razor/package.json` | 50 | Extension manifest (declarative) |
| `extensions/razor/package.nls.json` | 4 | Localisation strings |
| `extensions/razor/language-configuration.json` | 22 | Editor language config (declarative) |
| `extensions/razor/syntaxes/cshtml.tmLanguage.json` | 2031 | TextMate grammar (declarative JSON) |
| `extensions/razor/build/update-grammar.mjs` | 44 | Dev-time grammar sync script (Node.js) |
| `extensions/razor/cgmanifest.json` | 40 | Component-governance upstream declaration |
| `extensions/razor/.vscodeignore` | 3 | Packaging ignore list |

Total partition LOC stated in mission (44) refers only to the build script; the grammar is 2 031 lines of declarative JSON and is not counted as "runtime code."

---

### Per-File Notes

#### `extensions/razor/package.json` (lines 1–50)

The manifest registers a single language contribution (`"id": "razor"`) covering extensions `.cshtml` and `.razor` with MIME type `text/x-cshtml` (line 28). It also registers one grammar contribution at line 33, pointing to `./syntaxes/cshtml.tmLanguage.json` with scope name `text.html.cshtml` and three embedded-language mappings at lines 39–42:

- `section.embedded.source.cshtml` → `csharp`
- `source.css` → `css`
- `source.js` → `javascript`

These embedded-language mappings are the mechanism by which the VS Code tokeniser switches to the C#, CSS, and JavaScript grammars inside a Razor file's embedded regions. There is no `main` entry point; the extension is purely declarative and activates no JavaScript extension host code.

#### `extensions/razor/language-configuration.json` (lines 1–22)

Declares comment delimiters (HTML block comment `<!-- -->`), bracket pairs, auto-closing pairs, and surrounding pairs. Three bracket types are recognised: HTML comment, curly braces, and parentheses. This file is read by the VS Code editor core to configure bracket-matching, auto-close, and surrounding behaviour for the `razor` language id. No runtime logic is involved.

#### `extensions/razor/syntaxes/cshtml.tmLanguage.json` (lines 1–2031)

A verbatim (post-patch) copy of the upstream grammar from `dotnet/razor` at commit `743f32a68c61809b22fd84e8748c3686ef1bb8b8` (recorded in both line 7 of the grammar header and `cgmanifest.json`). Notable structural properties:

- **Scope name** `text.html.cshtml` (line 9), patched from the upstream `text.html.basic` delegation to `text.html.derivative` (lines 44–46 of the grammar's `patterns` array), enabling VS Code's derivative HTML grammar to be used as the base HTML layer.
- **Injections** (lines 10–38): Razor expression patterns (`#explicit-razor-expression`, `#implicit-expression`) are injected into `string.quoted.double.html` and `string.quoted.single.html` scopes, and an `#inline-template` pattern is injected into `source.cs`. These injections allow Razor syntax to appear inside HTML attribute values and inside C# code blocks respectively.
- **Top-level `patterns`** delegate first to `#razor-control-structures` and then to `text.html.derivative` (lines 39–46).
- **`repository`** defines rules including `razor-codeblock` (begins with `@{`, contentName `source.cs`, line 106–130), `razor-comment`, `explicit-razor-expression`, `escaped-transition` (matches `@@`, line 99), `transition` (matches `@`, line 102), `directives`, and various C# control-structure embeddings. All are regex-based TextMate rules with no imperative logic.

#### `extensions/razor/build/update-grammar.mjs` (lines 1–44)

A Node.js ES-module script run manually via `npm run update-grammar`. It imports `vscode-grammar-updater` (line 7) and calls `vscodeGrammarUpdater.update(...)` (line 42) pointing at the upstream repository `dotnet/razor`, the upstream file path, the local output path, and a patch callback.

The `patchGrammar` function (lines 9–38) performs a single structural transformation: it traverses the entire grammar object tree recursively and replaces every occurrence of an `include` value starting with `text.html.basic` with `text.html.derivative`. It expects exactly 4 such replacements and emits a `console.warn` if the count differs (line 33). The scope name is also explicitly set to `text.html.cshtml` at line 10. This script is a dev-time maintenance tool; it is not shipped in the extension package (excluded by `.vscodeignore` line 2: `build/**`).

#### `extensions/razor/cgmanifest.json` (lines 1–40)

Records the upstream `dotnet/razor` git repository and commit hash `743f32a68c61809b22fd84e8748c3686ef1bb8b8` for Microsoft's component-governance and supply-chain tracking. No runtime relevance.

#### `extensions/razor/.vscodeignore` (lines 1–3)

Excludes `test/**`, `cgmanifest.json`, and `build/**` from the packaged extension. Confirms the runtime extension surface is limited to `package.json`, `package.nls.json`, `language-configuration.json`, and `syntaxes/cshtml.tmLanguage.json`.

---

### Cross-Cutting Synthesis

Porting this partition to a Tauri/Rust host requires no translation of TypeScript or Rust logic from within this partition itself. The entire runtime payload is two declarative JSON files consumed by the host's grammar tokenisation engine. The porting work therefore lives entirely in the Rust host layer:

1. **TextMate grammar engine in Rust.** The host must load and evaluate `cshtml.tmLanguage.json` using a TextMate-compatible tokeniser (e.g. `syntect`, which reads `.tmLanguage` grammars). `syntect` supports JSON-format grammars, so the file can be used without modification, but the scope injection mechanism (`"injections"` key in the grammar) must be supported by whichever engine is chosen — `syntect` has partial injection support.

2. **Embedded-language switching.** The three `embeddedLanguages` mappings declared in `package.json` (C#, CSS, JavaScript) require the host to maintain a registry of scope-to-language-id mappings and switch tokeniser context when entering embedded regions. This is a core VS Code editor capability (`TextModel` + `TokenizationRegistry`) that must be replicated in the Rust host.

3. **Language configuration consumption.** The `language-configuration.json` bracket and auto-close data must be parsed and wired into the Rust editor's bracket-matching and auto-closing subsystems.

4. **Grammar update workflow.** The `build/update-grammar.mjs` script and `vscode-grammar-updater` npm package remain Node.js dev tooling; they are host-agnostic and can be reused as-is to refresh the grammar from upstream.

No C# language server (OmniSharp, Roslyn) interaction exists in this partition; full Razor IntelliSense is handled by the separate `ms-dotnettools.csharp` extension, which is out of scope here.

---

### Out-of-Partition References

- `text.html.derivative` scope: defined in `extensions/html/syntaxes/html-derivative.tmLanguage.json` — the HTML base grammar that Razor delegates to after the `patchGrammar` rewrite.
- `vscode-grammar-updater` npm package: used by at least four other extension build scripts (`html`, `php`, `rust`, `sql`) under `extensions/*/build/update-grammar.mjs`.
- Embedded language tokenisation registry: `src/vs/editor/common/languages/languageConfigurationRegistry.ts` and `src/vs/editor/common/tokenizationRegistry.ts` — these are the core host components that consume both `language-configuration.json` and the `embeddedLanguages` mappings from `package.json` at runtime.

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Pattern Analysis: Razor Extension (extensions/razor/)

## Summary

The Razor extension (`extensions/razor/`) is a **grammar-only language contribution package** with no runtime patterns.

## Scope Findings

The extension contains exclusively:
- **package.json**: Language registration and grammar contribution metadata
- **language-configuration.json**: Bracket matching and auto-closing pair rules (static configuration)
- **syntaxes/cshtml.tmLanguage.json**: TextMate grammar definition for Razor syntax highlighting
- **build/update-grammar.mjs**: Utility script to fetch and patch grammar from upstream (dotnet/razor)
- **cgmanifest.json** and **.vscodeignore**: Build/packaging metadata

## Analysis

This package contributes only declarative VS Code extension features:
1. Language ID registration for `.cshtml` and `.razor` files
2. TextMate grammar rules for syntax highlighting
3. Language configuration (bracket pairs, comment syntax)
4. Embedded language support (C#, CSS, JavaScript)

There is **no TypeScript/JavaScript runtime code** that performs IDE functionality, no activation events, no extension API calls, and no command implementations.

The build script (`update-grammar.mjs`) is a maintenance utility that pulls grammar updates from the upstream dotnet/razor repository—it does not represent VS Code IDE functionality.

## Relevance to Tauri/Rust Port

This extension demonstrates **zero IDE runtime patterns** and thus contributes no insights to a Tauri/Rust port of VS Code's core functionality. Grammar and static language configuration would be handled differently in a Rust-based IDE system (e.g., via tree-sitter grammars or similar), but that is architectural redesign, not pattern extraction.

**Verdict**: Skip—no runtime code or patterns to document.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
