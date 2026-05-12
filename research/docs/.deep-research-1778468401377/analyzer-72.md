# Analyzer 72: extensions/yaml/

## Scope
`extensions/yaml/` — YAML language extension (grammar + config, no runtime TS)

---

## File Inventory and Roles

| File | Role |
|---|---|
| `package.json` | VS Code extension manifest; declares language IDs, grammar contributions, file associations, editor defaults |
| `language-configuration.json` | Language configuration; defines brackets, autoclose pairs, folding rules, indentation patterns |
| `package.nls.json` | Localization strings for displayName and description |
| `cgmanifest.json` | Component governance manifest; records the upstream OSS dependency (RedCMD/YAML-Syntax-Highlighter @ commit `53d38bbc`) |
| `.vscodeignore` | Packaging exclusions: `test/**`, `cgmanifest.json`, `build/**` |
| `build/update-grammar.js` | Build-time Node script; fetches all six grammar files from the upstream GitHub repo via `vscode-grammar-updater` |
| `syntaxes/yaml.tmLanguage.json` | Primary grammar; `scopeName: source.yaml`; defaults to YAML 1.2 via `include: source.yaml.1.2`; also supports legacy FrontMatter and a basic embedded fallback |
| `syntaxes/yaml-1.2.tmLanguage.json` | Full YAML 1.2 grammar; `scopeName: source.yaml.1.2`; root rule `#stream`; handles standalone and Markdown-embedded contexts |
| `syntaxes/yaml-1.3.tmLanguage.json` | YAML 1.3 overlay; `scopeName: source.yaml.1.3`; detects `%YAML 1.3` directive and delegates block parsing to `source.yaml.1.2#document` |
| `syntaxes/yaml-1.1.tmLanguage.json` | YAML 1.1 grammar; `scopeName: source.yaml.1.1`; parallel structure to 1.2; same `#stream` root |
| `syntaxes/yaml-1.0.tmLanguage.json` | YAML 1.0 grammar; `scopeName: source.yaml.1.0` |
| `syntaxes/yaml-embedded.tmLanguage.json` | Embedded grammar; `scopeName: source.yaml.embedded`; delegates most rules to `source.yaml.1.2#*`; used when YAML appears inside other language contexts (e.g., Markdown fences) |

---

## Key Symbols and Declarations

### package.json — Language and Grammar Contributions

**Languages declared** (`package.json:16-52`):
- `dockercompose`: triggered by filename patterns (`compose.yml`, `*docker*compose*.yaml`, etc.)
- `yaml`: triggered by extensions `.yaml`, `.yml`, `.eyaml`, `.eyml`, `.cff`, `.yaml-tmlanguage`, `.yaml-tmpreferences`, `.yaml-tmtheme`, `.winget`; also by firstLine `^#cloud-config`
- Both languages reference `./language-configuration.json`

**Grammars declared** (`package.json:54-91`):
- `dockercompose` → `source.yaml` via `syntaxes/yaml.tmLanguage.json`
- `yaml` → `source.yaml` via `syntaxes/yaml.tmLanguage.json` with `unbalancedBracketScopes` set to `["invalid.illegal", "meta.scalar.yaml", "storage.type.tag.shorthand.yaml", "keyword.control.flow"]`
- Version-scoped grammars registered without a `language` field: `source.yaml.1.3`, `source.yaml.1.2`, `source.yaml.1.1`, `source.yaml.1.0`, `source.yaml.embedded`

**Editor defaults** (`package.json:92-108`):
- `[yaml]`: `insertSpaces: true`, `tabSize: 2`, `autoIndent: "advanced"`, `diffEditor.ignoreTrimWhitespace: false`, `defaultColorDecorators: "never"`, `quickSuggestions.strings: "on"`
- `[dockercompose]`: `insertSpaces: true`, `tabSize: 2`, `autoIndent: "advanced"`

### language-configuration.json

- Line comment: `#` (`line 3`)
- Bracket pairs: `{}`, `[]`, `()` (`lines 5-9`)
- Auto-closing pairs: same three plus `""` and `''` (`lines 10-14`)
- Surrounding pairs: same as auto-closing (`lines 17-23`)
- Folding: `offSide: true` (indentation-based); region markers via `#region` / `#endregion` (`lines 24-30`)
- Indentation increase pattern: `^\\s*.*(:|-) ?(&amp;\\w+)?(\\{[^}\"']*|\\([^)\"']*)?$` (`line 32`)
- Indentation decrease pattern: `^\\s+\\}$` (`line 33`)

### Grammar Architecture (tmLanguage files)

**yaml.tmLanguage.json** (`syntaxes/yaml.tmLanguage.json`):
- Top-level `patterns` array has three entries (`lines 10-36`):
  1. Default pattern: `begin: "\\A"`, `while: "^"` — anchors to stream start; includes `source.yaml.1.2`
  2. FrontMatter pattern: `begin: "(?<=^-{3,}\\s*+)\\G$"` — handles front matter blocks embedded in other languages; includes `source.yaml.1.2`
  3. Embedded fallback: includes `source.yaml.embedded`
- A `repository` key exists with version-compatibility notes and shared rule definitions (e.g., `block-map-key-single`, `key-single`) used by the version-specific grammars.

**yaml-1.2.tmLanguage.json** (`syntaxes/yaml-1.2.tmLanguage.json`):
- Root pattern: `#stream` (`line 12`)
- `#stream` has two variants (`lines 17-60`):
  1. Standalone mode: `begin: "^(?!\\G)"`, `while: "^"` — processes lines one at a time via `#byte-order-mark`, `#directives`, `#document`, `#presentation-detail`
  2. Embedded mode (Markdown code-blocks): `begin: "\\G(?!$)"`, `while: "\\G"` — same sub-patterns but anchored to match position

**yaml-1.3.tmLanguage.json** (`syntaxes/yaml-1.3.tmLanguage.json`):
- Detects `%YAML 1.3` directive via `begin: "(?=%YAML[\\t ]+1\\.3(?=[\\r\\n\\t ]))"` (`line 19`)
- Captures version-specific directive using named capture groups (`lines 25-40`): `punctuation.definition.directive.begin.yaml`, `keyword.other.directive.yaml.yaml`, `constant.numeric.yaml-version.yaml`
- Delegates all non-directive content to `source.yaml.1.2#document` and shared rules like `source.yaml.1.2#directive-invalid`, `source.yaml.1.2#directives`
- Root `patterns` entry simply includes `source.yaml` (`line 13`), meaning 1.3 is an overlay injected on top of the default grammar

**yaml-embedded.tmLanguage.json** (`syntaxes/yaml-embedded.tmLanguage.json`):
- Provides a stripped-down grammar for embedded contexts
- Top-level includes (`lines 10-62`): `source.yaml.1.2#byte-order-mark`, `#directives`, `#document`, `#block-sequence`, `#block-mapping`, `#block-map-key-explicit`, `#block-map-value`, `#block-scalar`, `source.yaml.1.2#anchor-property`, `source.yaml.1.2#tag-property`, `#alias`, `source.yaml.1.2#double`, `source.yaml.1.2#single`, `source.yaml.1.2#flow-mapping`, `source.yaml.1.2#flow-sequence`, `#block-plain-out`, `#presentation-detail`
- Mixed sourcing: some rules are local (`#`), others delegated directly to `source.yaml.1.2#*`

### build/update-grammar.js

- Requires `vscode-grammar-updater` (external npm package, not present in repo) (`line 7`)
- Calls `updateGrammar.update(repo, sourcePath, destPath, undefined, 'main')` six times sequentially (`lines 10-15`), once per grammar file, all from `RedCMD/YAML-Syntax-Highlighter` on branch `main`
- Target files written: `yaml-1.0.tmLanguage.json`, `yaml-1.1.tmLanguage.json`, `yaml-1.2.tmLanguage.json`, `yaml-1.3.tmLanguage.json`, `yaml-embedded.tmLanguage.json`, `yaml.tmLanguage.json`
- This script is invoked only during development via `npm run update-grammar` (defined in `package.json:12`); it has no runtime presence in the packaged extension

### cgmanifest.json

- Records one component registration: `RedCMD/YAML-Syntax-Highlighter` (`line 4`)
- Upstream repo: `https://github.com/RedCMD/YAML-Syntax-Highlighter` (`line 7`)
- Pinned commit: `53d38bbc66b704803de54ffce5b251bf97211c60` (`line 8`)
- License: MIT, Copyright 2024 RedCMD (`lines 12-21`)
- Component version: `1.3.2` (`line 23`)

---

## Data Flow

1. **At install/load time**: VS Code reads `package.json`; registers `yaml` and `dockercompose` language IDs with their file patterns and loads `language-configuration.json` for bracket/indent behavior.
2. **On file open**: VS Code matches the file extension or first-line against declarations in `package.json:39-51`. For `.yaml`/`.yml` files, it activates `source.yaml` grammar from `syntaxes/yaml.tmLanguage.json`.
3. **Grammar resolution**: `yaml.tmLanguage.json` anchors to `\A` (stream start) and includes `source.yaml.1.2` by default. If a `%YAML 1.3` directive is encountered, the 1.3 overlay grammar activates and delegates document content back to 1.2 rules.
4. **Tokenization**: The TextMate grammar engine (vscode's built-in `vscode-textmate`) executes the regex patterns sequentially, producing scope name annotations for each character range. These scope names drive syntax highlighting theme tokens.
5. **Embedded contexts**: When YAML appears in Markdown fences, `source.yaml.embedded` is included, which delegates leaf rules back to `source.yaml.1.2#*` cross-grammar includes.
6. **Grammar update (dev only)**: `build/update-grammar.js` fetches updated grammar files from upstream GitHub and writes them to `syntaxes/`; `cgmanifest.json` records the pinned commit for compliance tracking.

---

## Dependencies

### Internal (within VS Code)
- `vscode-textmate`: the built-in TextMate grammar evaluation engine that processes `.tmLanguage.json` files at runtime. Not referenced explicitly in this partition — it is a VS Code platform dependency.
- VS Code extension host contribution point system (reads `contributes.languages`, `contributes.grammars`, `contributes.configurationDefaults` from `package.json`)

### External (build-time only)
- `vscode-grammar-updater` npm package (`build/update-grammar.js:7`): fetches grammar files from GitHub during development; excluded from packaged extension via `.vscodeignore:3`
- `RedCMD/YAML-Syntax-Highlighter` GitHub repo: upstream source for all six `.tmLanguage.json` grammar files

### No runtime Node/Electron dependencies
- The extension contains no TypeScript source files, no activation events, no language server, and no runtime JavaScript. The only JS file (`build/update-grammar.js`) is excluded from the packaged extension by `.vscodeignore`.

---

## Synthesis (≤200 words)

The `extensions/yaml/` partition is a purely declarative VS Code extension. It contributes two language identifiers (`yaml`, `dockercompose`) and six TextMate grammars covering YAML spec versions 1.0 through 1.3 plus an embedded context, all sourced from the upstream `RedCMD/YAML-Syntax-Highlighter` project. At runtime, VS Code's built-in `vscode-textmate` engine loads the JSON grammar files directly; there is no TypeScript, no activation entry point, and no extension host process. Language configuration (`language-configuration.json`) provides bracket matching, auto-close, folding, and indentation rules. The grammar architecture uses cross-grammar `include` references (e.g., `source.yaml.1.3` includes `source.yaml.1.2#document`), enabling version overlays without duplicating rule definitions. The YAML 1.2 grammar is the canonical implementation; other version grammars and the embedded grammar delegate to it for most rules. The sole JavaScript file (`build/update-grammar.js`) is a developer utility excluded from the packaged extension. The extension has zero runtime coupling to Electron, Node.js APIs, or any VS Code extension API surface.

---

## Out-of-Partition References

- `vscode-textmate` (VS Code platform): processes `.tmLanguage.json` at runtime; source lives outside `extensions/yaml/`
- `vscode-markdown-tm-grammar` (referenced in comment at `syntaxes/yaml.tmLanguage.json:23`): the FrontMatter integration pattern cites `https://github.com/microsoft/vscode-markdown-tm-grammar/pull/162`
- `vscode-grammar-updater` npm package: used only in `build/update-grammar.js`; not vendored in this partition
- `RedCMD/YAML-Syntax-Highlighter` GitHub repository: external upstream for all grammar JSON files; pinned commit tracked in `cgmanifest.json`
