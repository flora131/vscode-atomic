# Colorize/Grammar Tests Locator (Partition 29)

**Scope:** `extensions/vscode-colorize-tests/` (341 files, ~13MB, 502 LOC of test harness code)

---

## Implementation

### Test Harness & Orchestration
- `/home/norinlavaee/projects/vscode-atomic/extensions/vscode-colorize-tests/src/index.ts`
  - Extension activation hook; registers semantic token provider
  - Semantic tokens legend (types: type, struct, class, interface, enum, parameterType, function, variable, testToken)
  - Semantic tokens modifiers (static, abstract, deprecated, declaration, documentation, member, async, testModifier)
  - JSON-based semantic token test provider for pattern matching
  - ~73 LOC

- `/home/norinlavaee/projects/vscode-atomic/extensions/vscode-colorize-tests/src/colorizerTestMain.ts`
  - Test runner configuration entry point
  - Mocha orchestration (ui: 'tdd', timeout: 60s)
  - Multi-reporter setup for CI (mocha-junit-reporter)
  - JUnit XML test result output to `BUILD_ARTIFACTSTAGINGDIRECTORY` or `GITHUB_WORKSPACE`
  - Platform/arch tagged result files
  - ~32 LOC

### Test Implementation
- `/home/norinlavaee/projects/vscode-atomic/extensions/vscode-colorize-tests/src/colorizer.test.ts`
  - Core test suite: `suite('colorization')`
  - Two tokenizer variants per fixture: TextMate (`_workbench.captureSyntaxTokens`) + Tree-sitter (`_workbench.captureTreeSitterSyntaxTokens`)
  - Regression test loop: iterates all fixtures in `test/colorize-fixtures/`
  - Token capture -> JSON serialization -> deep equality assertion
  - Diff handling: accepts same colorization even if theme or position changed (line 34-37)
  - Result persistence: writes JSON snapshots to `colorize-results/` and `colorize-tree-sitter-results/`
  - ~97 LOC

---

## Tests

### Colorization Fixture Corpus
- `/home/norinlavaee/projects/vscode-atomic/extensions/vscode-colorize-tests/test/colorize-fixtures/`
  - **108 test fixtures** across 40+ language modes
  - Cover edge cases and regression issues tied to GitHub issue numbers (issue-1550.yaml, issue-6303.yaml, issue-224862.yaml, test-23850.cpp, test-166781.rs, test-241001.ts, etc.)
  - Language families tested:
    - **C/C++/C#:** test.c, test.cpp, test.cc, test.cu, test-23850.cpp, test-23630.cpp, test-80644.cpp, test-78769.cpp
    - **TypeScript/JavaScript:** test.ts, test.tsx, test.js, test.jsx, test-241001.ts, test-issue241715.ts, test-issue5465.ts, test-object-literals.ts, test-strings.ts, test-issue5431.ts, test-issue11.ts, test-jsdoc-multiline-type.ts, test-function-inv.ts, test-issue5566.ts, test-keywords.ts, test-members.ts, test-this.ts, test6916.js
    - **Web:** test.html, test.xml, test.css, test.less, test.scss, test-embedding.html, test-variables.css, test-cssvariables.less, test-cssvariables.scss, test-4287.pug, test-variables.css
    - **Python:** test.py, test-freeze-56377.py
    - **Rust:** test.rs, test-166781.rs, test-6611.rs
    - **Go:** test.go, test-13777.go
    - **Shell/Scripting:** test.sh, test-173216.sh, test-173336.sh, test-173224.sh
    - **Config/Data:** test.json, test.yaml, issue-6303.yaml, issue-224862.yaml, issue-4008.yaml, issue-1550.yaml, tsconfig_off.json, test.ini, test.env
    - **Markup:** test.md, md-math.md, test-33886.md, test.rst, test.tex, test.sty, test.bib
    - **Other:** test.php, test.java, test.go, test.rb, test.r, test.pl, test2.pl, test.lua, test.clj, test.fs, test.dart, test.swift, test.m, test.mm, test.p6, test.handlebars, test.hbs, test.cshtml, test.bat, test.ps1, test-freeze-56476.ps1, test.shader, test.hlsl, test.sql, test.log, test.bib, test.coffee, test-regex.coffee, test.diff, Dockerfile, git-rebase-todo, COMMIT_EDITMSG, makefile, test.code-snippets, test-13777.go, test.jl, test.groovy, test.vb

### Result Snapshots
- `/home/norinlavaee/projects/vscode-atomic/extensions/vscode-colorize-tests/test/colorize-results/`
  - **108 JSON snapshot files** (one per fixture)
  - Each snapshot contains token array: `[{c: int, r: {fg?, bg?, fontStyle?}}, ...]`
  - c = token class/scopes, r = theme color info
  - Files named matching fixture inputs: test_cpp.json, test-7115_xml.json, test_mm.json, etc.

- `/home/norinlavaee/projects/vscode-atomic/extensions/vscode-colorize-tests/test/colorize-tree-sitter-results/`
  - **108 JSON snapshot files** (parallel to colorize-results)
  - Tree-sitter tokenization baseline (experimental feature under `editor.experimental.preferTreeSitter.*` config)
  - Allows parallel validation of both TextMate and Tree-sitter engines

### Semantic Token Test
- `/home/norinlavaee/projects/vscode-atomic/extensions/vscode-colorize-tests/test/semantic-test/`
  - `semantic-test.json` - JSON document with semantic token decoration markers
  - `.vscode/settings.json` - Local workspace config for semantic test isolation

---

## Configuration

### Build & Deployment
- `/home/norinlavaee/projects/vscode-atomic/extensions/vscode-colorize-tests/package.json`
  - Entry point: `./out/colorizerTestMain` (compiled TS)
  - Activation: `onLanguage:json` (semantic test provider trigger)
  - Build: `gulp compile-extension:vscode-colorize-tests` (tsconfig.json driven)
  - CI: Mocha + multi-reporter (spec + JUnit XML)
  - Contributes semantic token types/modifiers/scopes, product icon theme

- `/home/norinlavaee/projects/vscode-atomic/extensions/vscode-colorize-tests/tsconfig.json`
  - Extends `tsconfig.base.json` (root)
  - rootDir: `./src`, outDir: `./out`
  - Includes `../../src/vscode-dts/vscode.d.ts` (API types)

### IDE/Debugger
- `/home/norinlavaee/projects/vscode-atomic/extensions/vscode-colorize-tests/.vscode/launch.json`
  - Launch config: "Launch Tests" 
  - Type: `extensionHost` (runs tests in VS Code host)
  - Args: Opens workspace root + test folder with extension enabled
  - Sourcemaps + out/ folder breakpoint support

- `/home/norinlavaee/projects/vscode-atomic/extensions/vscode-colorize-tests/.vscode/tasks.json`
  - Pre-launch task: `npm` (compile step)

### Misc
- `/home/norinlavaee/projects/vscode-atomic/extensions/vscode-colorize-tests/.npmrc` - NPM config
- `/home/norinlavaee/projects/vscode-colorize-tests/package-lock.json` - Locked deps (jsonc-parser 3.2.0, @types/node 22.x)

---

## Examples / Fixtures

**Fixture Naming Patterns:**

1. **Regression Issue-driven:** issue-XXX.{yaml,php,xml}
   - issue-1550.yaml, issue-4008.yaml, issue-6303.yaml, issue-224862.yaml, issue-28354.php, issue-76997.php, test-7115.xml

2. **Feature Test-driven:** test-XXX.{ts,js,cpp,rs,py,sh}
   - test-241001.ts, test-23850.cpp, test-166781.rs, test-freeze-56377.py, test-173216.sh, test-freeze-56476.ps1

3. **Language General:** test.{ext}
   - Basic corpus (100+ files, one or two per language)

4. **Special Cases:** test-embedding.html (nested language), test-jsdoc-multiline-type.ts (JSDoc edge cases), md-math.md (Markdown with math), test-brackets.tsx (bracket pairing)

**Product Icon Theme:**
- `/home/norinlavaee/projects/vscode-atomic/extensions/vscode-colorize-tests/producticons/test-product-icon-theme.json`
  - Icon theme contribution test fixture
- `/home/norinlavaee/projects/vscode-atomic/extensions/vscode-colorize-tests/producticons/ElegantIcons.woff` - Test icon font

---

## Notable Clusters

### Dual-Engine Tokenization Pipeline
The test suite captures and compares outputs from **two syntactic analysis engines:**

1. **TextMate (default):** `_workbench.captureSyntaxTokens` command
   - Snapshot results → `test/colorize-results/`
   - Grammar-based (XML TextMate grammar files in VS Code core)
   - Used for all 108 fixtures

2. **Tree-sitter (experimental):** `_workbench.captureTreeSitterSyntaxTokens` command
   - Snapshot results → `test/colorize-tree-sitter-results/`
   - Parallel parser library (WASM/native bindings in VS Code core)
   - Controlled by `editor.experimental.preferTreeSitter.*` config flags (typescript, ini, regex, css)
   - Line 74-80 in colorizer.test.ts

**Tauri/Rust Port Implication:**
- A Rust-based VS Code port must replicate both pipelines or choose one:
  - TextMate path: Integrate an existing TextMate grammar parser + theme applier (Rust crate: `text-mate` or similar)
  - Tree-sitter path: Leverage existing Rust Tree-sitter bindings, port grammar queries
  - Semantic tokens: Implement `SemanticTokensProvider` API (language-agnostic, defined in vscode.d.ts)

### Test Data Regression Coverage
- 108 fixtures = broad language coverage (40+ modes)
- 216 snapshots (TextMate + Tree-sitter) = two-engine baseline
- Test comparison: token sequence + color theme info (lines 34-37: tolerates color theme changes but fails on token boundary/scope shifts)

### CI/CD Integration
- Mocha suite runs in VS Code host (no standalone runner)
- JUnit XML export to `test-results/{platform}-{arch}-{suite}.xml`
- Used for build verification in Azure Pipelines / GitHub Actions

---

## Summary

**Partition 29 is primarily a **grammar fixture corpus + test harness** with minimal IDE-runtime logic.**

The 502 lines of TypeScript code comprise:
- **~73 LOC:** Semantic token provider (index.ts) - VSCode API contract
- **~97 LOC:** Core regression test loop (colorizer.test.ts) - snapshot comparison
- **~32 LOC:** Mocha orchestration (colorizerTestMain.ts) - CI plumbing
- **~300 LOC:** Remaining stubs/config

**Key insight for Tauri/Rust port:** This partition does NOT contain IDE runtime code. However, it documents the **textual colorization pipeline** that a Rust host must reproduce:

1. **Grammar Engines:** TextMate (regex-based) + Tree-sitter (parsed AST)
2. **Token Output Format:** Scopes string (e.g., "source.ts keyword.control") → token type/modifiers → theme colors
3. **Semantic Tokens API:** Parallel mechanism for LSP-driven highlighting (language servers emit token ranges + types)
4. **Test Infrastructure:** Uses VS Code commands (`_workbench.captureSyntaxTokens`, `_workbench.captureTreeSitterSyntaxTokens`) to validate output

**For a Rust rewrite, prioritize:**
- Grammar parser integration (TextMate or Tree-sitter, ideally both)
- Semantic token provider registration mechanism
- Theme color application layer (maps token scope → theme variable)
- Regression test harness (corpus + snapshot comparison still valid)

The fixture corpus itself (108 files across 40+ languages) is reusable as golden data for validating colorization parity.

