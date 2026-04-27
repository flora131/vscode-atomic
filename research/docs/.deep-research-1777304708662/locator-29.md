# File Locator Report: vscode-colorize-tests Extension

**Partition:** 29 of 79  
**Scope:** `extensions/vscode-colorize-tests/` (69 files, ~1,946 LOC)  
**Focus:** Tokenization fixtures and semantic token tests

---

## Summary

The `vscode-colorize-tests` extension is a test harness for VS Code's syntax tokenization and semantic token systems. It validates colorization behavior across 50+ languages by exercising both TextMate-based tokenization (`_workbench.captureSyntaxTokens`) and Tree-sitter-based tokenization (`_workbench.captureTreeSitterSyntaxTokens`). The extension captures token output snapshots that serve as regression tests for the core tokenizer implementations.

**Key Insight for Tauri/Rust Port:** A Rust-based IDE port would need to either embed or reimplement:
- TextMate grammar engine (for classic tokenization)
- Tree-sitter grammar engine (for modern language support)
- Semantic token type/modifier system (custom token classifications)
- Theme color mapping to tokens

---

## Implementation

### Source Code (3 files, 201 LOC)

| File | Purpose |
|------|---------|
| `src/colorizerTestMain.ts` | Extension activation; registers semantic token provider for JSON fixture files; builds semantic token legend and applies token metadata |
| `src/colorizer.test.ts` | Integration test suite; orchestrates dual tokenization (TextMate + Tree-sitter) and compares against baseline snapshots; handles theme-only changes vs. tokenization changes |
| `src/index.ts` | Test runner configuration; wraps Mocha test harness with JUnit reporter for CI |

### Configuration

| File | Purpose |
|------|---------|
| `package.json` | Extension manifest; declares semantic token types/modifiers, semantic token scopes, product icon theme contribution |
| `tsconfig.json` | TypeScript compiler config; targets `./out/` directory |

### Product Icon Theme

| File | Purpose |
|------|---------|
| `producticons/test-product-icon-theme.json` | Icon theme test fixture; defines icon mappings for UI elements (chevrons, error, warning, settings, files, extensions, run-view) |
| `producticons/ElegantIcons.woff` | Web font asset for icon rendering |
| `producticons/index.html` | (HTML placeholder/documentation for icon theme) |
| `producticons/mit_license.txt` | License for ElegantIcons font |

---

## Tests

### Test Harness (1 file, 96 LOC)

| File | Purpose |
|------|---------|
| `src/colorizer.test.ts` | **Primary test suite** for tokenization. For each fixture file in `test/colorize-fixtures/`, tests: (1) captures TextMate tokens, (2) captures Tree-sitter tokens, (3) compares against baseline JSON in `test/colorize-results/` and `test/colorize-tree-sitter-results/`. Detects regressions: changes in tokenization OR theme color mappings |

### Test Fixtures (108 files, distributed across languages)

**TypeScript/JavaScript/JSX** (11 files)
- `test-241001.ts` — Issue-specific test case
- `test-function-inv.ts`, `test-issue11.ts`, `test-issue241715.ts`, `test-issue5431.ts`, `test-issue5465.ts`, `test-issue5566.ts`, `test-jsdoc-multiline-type.ts`, `test-keywords.ts`, `test-members.ts`, `test-object-literals.ts`, `test-strings.ts`, `test-this.ts`
- `test.ts`, `test.tsx`, `test.regexp.ts`, `test.jsx`
- `test6916.js`, `test.js`

**Markup/Styling** (12 files)
- HTML: `test.html`, `test-embedding.html`, `12750.html`, `13448.html`, `25920.html`
- CSS: `test.css`, `test-variables.css`
- SCSS/LESS: `test.scss`, `test.less`, `test-cssvariables.scss`, `test-cssvariables.less`, `14119.less`

**Compiled/Backend Languages** (13 files)
- C/C++: `test.c`, `test.cc`, `test.cpp`, `test.cu`, `test-23630.cpp`, `test-23850.cpp`, `test-78769.cpp`, `test-80644.cpp`
- C#: `test.cs`, `test.cshtml`
- Java: `basic.java`
- PHP: `test.php`, `issue-28354.php`, `issue-76997.php`

**Dynamic Languages** (8 files)
- Python: `test.py`, `test-freeze-56377.py`
- Ruby: `test.rb`
- Go: `test.go`, `test-13777.go`
- Rust: `test.rs`, `test-166781.rs`, `test-6611.rs`
- Clojure: `test.clj`

**Scripting/Configuration** (15 files)
- Shell: `test.sh`, `test-173216.sh`, `test-173224.sh`, `test-173336.sh`
- PowerShell: `test.ps1`, `test-freeze-56476.ps1`
- YAML: `test.yaml`, `issue-1550.yaml`, `issue-4008.yaml`, `issue-6303.yaml`, `issue-224862.yaml`
- Batch: `test.bat`
- JSON: `test.json`, `tsconfig_off.json`
- JSONC: `test.code-snippets`

**Markup/Data** (9 files)
- Markdown: `test.md`, `test-33886.md`, `md-math.md`
- XML: `test.xml`, `test-7115.xml`
- Log: `test.log`
- Diff: `test.diff`
- BibTeX: `test.bib`
- Ini: `test.ini`

**Specialized Languages** (20+ files)
- Handlebars: `test.handlebars`, `test.hbs`
- Pug: `test-4287.pug`, `test.pug`
- CoffeeScript: `test.coffee`, `test-regex.coffee`
- HLSL/Shaders: `test.hlsl`, `test.shader`
- Tex/LaTeX: `test.sty`, `test.tex`
- Dart: `test.dart`
- Groovy: `test.groovy`
- Julia: `test.jl`
- Lua: `test.lua`
- Lisp: `test.p6`
- Perl: `test.pl`, `test2.pl`
- R: `test.r`
- ReStructuredText: `test.rst`
- SQL: `test.sql`
- Swift: `test.swift`
- Objective-C: `test.m`, `test.mm`
- VB.NET: `test.vb`
- FSharp: `test.fs`

**System/Control Files** (5 files)
- `COMMIT_EDITMSG` — Git commit message template test
- `Dockerfile` — Docker configuration
- `git-rebase-todo` — Git rebase script
- `makefile` — Make build script

### Baseline Results (50+ JSON snapshots)

| Directory | Purpose |
|-----------|---------|
| `test/colorize-results/` | TextMate tokenization baseline snapshots (~50 JSON files mapping fixture names to token metadata). Each JSON contains array of token objects with: `.c` (character content), `.t` (TextMate scope), `.r` (theme color mappings across built-in themes: dark_plus, light_plus, dark_vs, light_vs, hc_black, dark_modern, hc_light, light_modern, 2026-dark, 2026-light) |
| `test/colorize-tree-sitter-results/` | Tree-sitter tokenization baseline snapshots (parallel structure to colorize-results) |

---

## Types / Interfaces

**TypeScript Type System**

| File | Symbols |
|------|---------|
| `src/colorizerTestMain.ts` | Implements: `vscode.DocumentSemanticTokensProvider`, uses `vscode.SemanticTokensLegend`, `vscode.SemanticTokensBuilder`, `jsonc-parser.JSONVisitor` |
| `src/colorizer.test.ts` | Async test harness; uses Mocha interfaces (`suite`, `suiteSetup`, `suiteTeardown`, `test`), VS Code command API, workspace configuration API |

---

## Configuration

| File | Purpose |
|------|---------|
| `.vscode/launch.json` | Debug launch config for extension host; runs tests via `--extensionTestsPath` |
| `.vscode/tasks.json` | Pre-launch compilation task (runs `npm run compile`) |
| `package.json` | Defines extension metadata, dependencies (`jsonc-parser`), and contributes semantic token types, modifiers, and scopes |
| `test/semantic-test/.vscode/settings.json` | Workspace-level settings for semantic token color customization testing; tests mapping of token types (class, interface, function, etc.) to colors and modifiers to font styles |

---

## Examples / Fixtures

### Tokenization Test Corpus

**Structure:** `test/colorize-fixtures/` contains **108 fixture files** spanning 50+ languages representing:
- **Common syntactic patterns:** keywords, comments, strings, operators, literals, identifiers
- **Language-specific edge cases:** issue-numbered files (e.g., `test-241001.ts`, `test-13777.go`) targeting historical regressions
- **Advanced features:** JSDoc multi-line types, CSS variables, embedding (HTML with JS), frozen tokenization (Python/PowerShell)
- **Scope coverage:** From minimal test snippets to realistic code samples (Game of Life in TypeScript)

**Sample Fixture Content (test.ts):**
```
/* Game of Life
 * Implemented in TypeScript
 */
module Conway {
  export class Cell {
    public row: number;
    ...
```

Corresponding baseline snapshot in `test/colorize-results/test_ts.json`:
```json
[
  {
    "c": "/*",
    "t": "source.ts comment.block.ts punctuation.definition.comment.ts",
    "r": {
      "dark_plus": "comment: #6A9955",
      "light_plus": "comment: #008000",
      ...
    }
  },
  ...
]
```

### Semantic Token Test Corpus

**File:** `test/semantic-test/semantic-test.json` — defines expected semantic token classifications for a test document:
```json
[
  "class",
  "function.member.declaration",
  "parameterType.declaration",
  "type",
  "parameterType.declaration",
  "type",
  "variable.declaration",
  "parameterNames",
  "function.member.declaration",
  "interface.declaration",
  "function.member.declaration",
  "function.notInLegend"
]
```

This validates that the extension correctly assigns token types and modifiers (e.g., `declaration`, `member`) from a custom legend.

---

## Documentation

### Extension Metadata

**Via `package.json`:**
- Extension name: `vscode-colorize-tests`
- Description: "Colorize tests for VS Code"
- Contributes:
  - Semantic token type: `testToken` ("A test token")
  - Semantic token modifier: `testModifier` ("A test modifier")
  - Semantic token scope mappings: `testToken` -> `entity.name.function.special`
  - Product icon theme: "Test Product Icons" (path: `./producticons/test-product-icon-theme.json`)

### Inline Comments

**Icon Theme:** `producticons/mit_license.txt` credits ElegantIcons font from https://www.elegantthemes.com/icons/elegant_font.zip

---

## Notable Clusters

### Dual Tokenization Architecture

The test suite simultaneously validates **TextMate** and **Tree-sitter** tokenization engines. This is critical because:

1. **TextMate grammars** (regex-based, `.plist` / `.json` format) have been VS Code's primary tokenizer for 10+ years.
2. **Tree-sitter** (C-based parser generators with WASM bindings) provide more accurate parsing for complex grammars (TypeScript, CSS, INI, Regex).

A Rust/Tauri port would need to decide:
- Embed both engines? (Maintenance burden)
- Port to pure-Rust Tree-sitter fork? (Possible via `tree-sitter-rs`)
- Implement a new grammar system? (Significant engineering effort)

**Test Files:**
- `src/colorizer.test.ts` (lines 14-15): Iterates dual tokenizers
- `src/colorizerTestMain.ts`: Demonstrates custom semantic token provider (could be basis for a plugin system)

### Multi-Language Fixture Suite

The **108 fixture files** represent breadth across programming language ecosystems:
- **Modern:** TypeScript, Rust, Go, Julia
- **Enterprise:** C#, Java, PowerShell
- **Web:** HTML, CSS, SCSS, LESS, Handlebars
- **Scripting:** Python, Ruby, Bash, CoffeeScript
- **Markup:** Markdown, YAML, JSON, XML
- **System:** Dockerfile, Makefiles, Git metadata

**Insight for Porting:** A Tauri IDE would need to ship or dynamically load grammar files for each of these ~50 languages. Tree-sitter has prebuilt grammars for most; TextMate grammars exist in VS Code's built-in collections.

### Snapshot-Based Regression Detection

**Baseline Directories:**
- `test/colorize-results/` — TextMate token snapshots
- `test/colorize-tree-sitter-results/` — Tree-sitter token snapshots

**Strategy** (from `colorizer.test.ts`):
1. Run fixture through tokenizer
2. Serialize tokens to JSON snapshot
3. Compare against baseline
4. If different, check if diff is only in theme colors (line 34: `hasThemeChange()`) or actual tokenization
5. If only theme color change, pass; if tokenization changed, fail

**For Rust Port:** This pattern suggests tokenizers must produce stable, comparable output. A custom Tauri tokenizer would need identical snapshot infrastructure to prevent silent regressions.

### Semantic Token Provider Pattern

**File:** `src/colorizerTestMain.ts`

Demonstrates VS Code's extension API for custom semantic token providers:
- Register legend: `new vscode.SemanticTokensLegend(tokenTypes, tokenModifiers)`
- Implement provider: `vscode.DocumentSemanticTokensProvider.provideDocumentSemanticTokens()`
- Parse document, build token ranges
- Apply modifiers as bitmasks (line 40: `tokenModifiers | 1 << index`)

**For Rust Port:** This is the interface a plugin/extension system would expose to contributors wanting custom language support. A Tauri IDE could replicate this via WebAssembly or native module bindings.

---

## Entry Points

- **Extension Activation:** `package.json` → `main: ./out/colorizerTestMain`
- **Test Runner:** `src/index.ts` (wraps `mocha` via `../../../test/integration/electron/testrunner`)
- **Test Suite:** `src/colorizer.test.ts` (Mocha test cases defined by `suite('colorization', ...)`)

---

## Partition Justification

This partition isolates **tokenization validation infrastructure** critical to understanding what a Rust/Tauri port must preserve:

1. **Dual tokenization engines** (TextMate + Tree-sitter) — requires porting or FFI
2. **Multi-language fixture coverage** — defines scope of language support
3. **Snapshot regression testing** — establishes baseline stability expectations
4. **Theme color mapping** — demonstrates coupling between tokenization and UI theming
5. **Semantic token system** — shows custom classification layer above raw tokens

A port that ignores this would risk tokenization regressions, unexpected color shifts, or language-specific parsing bugs in production.

