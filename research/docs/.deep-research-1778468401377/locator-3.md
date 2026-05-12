# Partition 3: `extensions/vscode-colorize-perf-tests/` — Sentinel Output

**Status:** Not relevant to VS Code core IDE porting effort.

## Summary

The `vscode-colorize-perf-tests` directory contains performance test fixtures for VS Code's tokenization/syntax highlighting system. It is a test-only extension that benchmarks two tokenization engines (TreeSitter vs. TextMate) against fixture files.

## Test Fixtures
- `extensions/vscode-colorize-perf-tests/test/colorize-fixtures/test.ts` — Small TypeScript fixture
- `extensions/vscode-colorize-perf-tests/test/colorize-fixtures/test-treeView.ts` — Medium TypeScript fixture (~80 KB)
- `extensions/vscode-colorize-perf-tests/test/colorize-fixtures/test-checker.ts` — Large TypeScript fixture (~8.3 MB)

## Configuration
- `extensions/vscode-colorize-perf-tests/tsconfig.json` — TypeScript compiler settings
- `extensions/vscode-colorize-perf-tests/package.json` — Extension metadata and build scripts
- `extensions/vscode-colorize-perf-tests/.npmrc` — NPM configuration

## Implementation
- `extensions/vscode-colorize-perf-tests/src/colorizer.test.ts` — Mocha test suite comparing TreeSitter parse/capture/metadata times vs. TextMate tokenize times
- `extensions/vscode-colorize-perf-tests/src/index.ts` — Test runner configuration targeting the Mocha test framework
- `extensions/vscode-colorize-perf-tests/src/colorizerTestMain.ts` — Extension activation entry point (empty stub)

## Relevance to Porting

**Not relevant.** This extension:
- Tests only the VS Code syntax highlighting/tokenization layer
- Does not cover core IDE functionality (editing, language intelligence, debugging, source control, terminal, navigation)
- Uses Mocha + VS Code API for benchmarking only
- Contains no architectural patterns applicable to a Tauri/Rust port

This is a specialized testing extension and can be safely disregarded for the porting effort.
