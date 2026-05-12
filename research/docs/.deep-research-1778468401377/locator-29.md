# vscode-colorize-tests: File Locator Index

## Overview

The `extensions/vscode-colorize-tests/` directory (69 files, ~1,946 LOC) contains TextMate grammar and tokenization integration test infrastructure for VS Code's syntax highlighting layer. This extension validates that the editor's colorization/tokenization pipeline produces consistent output across different languages and grammar implementations.

## Implementation

- `extensions/vscode-colorize-tests/src/index.ts` - Test runner entry point; configures Mocha test framework and outputs JUnit XML for CI/CD environments
- `extensions/vscode-colorize-tests/src/colorizerTestMain.ts` - Main test suite entry; orchestrates test execution
- `extensions/vscode-colorize-tests/src/colorizer.test.ts` - Core test logic; compares current tokenization output against baseline results using `_workbench.captureSyntaxTokens` and `_workbench.captureTreeSitterSyntaxTokens` commands

## Configuration

- `extensions/vscode-colorize-tests/package.json` - Extension manifest defining semantic token types, modifiers, and product icon theme contribution
- `extensions/vscode-colorize-tests/tsconfig.json` - TypeScript configuration
- `extensions/vscode-colorize-tests/.npmrc` - NPM configuration
- `extensions/vscode-colorize-tests/package-lock.json` - Dependency lock file
- `extensions/vscode-colorize-tests/.vscode/launch.json` - Debug launch configuration
- `extensions/vscode-colorize-tests/.vscode/tasks.json` - VSCode build tasks

## Examples / Fixtures

### Source Fixtures (108 files in `test/colorize-fixtures/`)
Language-specific source files used as input for tokenization tests, covering 40+ languages:

- C/C++/C#: `test.c`, `test.cc`, `test.cpp`, `test-*.cpp` (multiple issue-specific fixtures)
- TypeScript/JavaScript: `test.ts`, `test.js`, `test.tsx`, `test-*.ts` (issue-specific variants), `test-issue*.ts`, `test.jsx`
- Python: `test.py`, `test-freeze-56377.py`
- Rust: `test.rs`, `test-6611.rs`, `test-166781.rs`
- Go: `test.go`, `test-13777.go`
- Java: `basic.java`
- C#: `test.cs`
- VB.NET: `test.vb`
- F#: `test.fs`
- PHP: `test.php`, `issue-28354.php`, `issue-76997.php`
- Web: `test.html`, `test.embedding.html`, `test-embedding.html`
- Styling: `test.css`, `test.scss`, `test.less`, `test-cssvariables.scss`, `test-cssvariables.less`, `test-variables.css`
- Markup: `test.xml`, `test-7115.xml`, `test.json`, `test-embedding.html`
- Templates: `test.handlebars`, `test.pug`, `test-4287.pug`, `test.cshtml`
- Scripting: `test.py`, `test.rb`, `test.lua`, `test.perl`, `test.sh`, `test-*.sh` (multiple shell variants), `test.bat`
- YAML: `test.yaml`, `issue-*.yaml` (multiple variants)
- Other: `test.dart`, `test.coffee`, `test-regex.coffee`, `test.clj`, `test.rs`, `test.m`, `test.mm`, `test.pl`, `test2.pl`, `test.p6`, `test.r`, `test.jl`, `test.rst`, `test.bib`, `test.log`, `test.ini`, `test.diff`, `Dockerfile`, `makefile`, `git-rebase-todo`, `COMMIT_EDITMSG`, `test.env`, `test.code-snippets`, `md-math.md`, `test-33886.md`
- Special: Issue-specific test cases like `test-241001.ts`, `test-function-inv.ts`, `test-jsdoc-multiline-type.ts`, `test-keywords.ts`, `test-members.ts`, `test-object-literals.ts`, `test-strings.ts`, `test-this.ts`, `test-brackets.tsx`, `test-issue11.ts`, `test-issue5431.ts`, `test-issue5465.ts`, `test-issue5566.ts`

### Expected Output Baselines

- `test/colorize-results/` (180+ JSON files) - TextMate tokenizer baseline outputs; one JSON file per fixture file
- `test/colorize-tree-sitter-results/` (180+ JSON files) - Tree-sitter tokenizer baseline outputs; parallel structure to TextMate results
- `test/semantic-test/semantic-test.json` - Semantic token highlighting test data

Format: Each JSON file contains an array of token objects with colorization and scope information, used for regression detection.

## Documentation

- `extensions/vscode-colorize-tests/producticons/mit_license.txt` - License for product icons

## Notable Clusters

**Dual Tokenization Tracking**
- The extension maintains separate baseline directories for two tokenization backends: TextMate (traditional) and Tree-sitter (modern). This dual-path architecture documents VS Code's shift toward Tree-sitter support for certain languages (TypeScript, CSS, regex, INI).

**Language Coverage**
- Comprehensive fixture library spanning 40+ programming and markup languages. Each language has at least one generic fixture (`test.*`) and multiple issue-specific variants capturing edge cases and past bug reports.

**Product Icon Theme Testing**
- `producticons/` directory with custom product icon theme for semantic token visual testing (`test-product-icon-theme.json`)

## Summary

This extension serves as a regression test harness for VS Code's tokenization layer. It captures syntax highlighting output for hundreds of language samples using two different tokenization engines (TextMate and Tree-sitter) and validates consistency against stored baselines. For a Tauri/Rust port, this directory documents the exact tokenization contract the editor must honor: what token types and scopes every supported language must produce. The dual-tokenization architecture signals that a production implementation would need either TextMate grammar support (complex, language-specific) or Tree-sitter integration (modern, unified parsing), or both. The fixture files themselves are valuable specifications of language syntax that must continue to tokenize correctly.
