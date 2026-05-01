# File Locations: extensions/vscode-colorize-perf-tests/

extensions/vscode-colorize-perf-tests/ contains only colorization perf fixtures (large grammar fixture files); not relevant to porting IDE runtime to Tauri/Rust.

## Summary

This extension is a performance testing harness for syntax colorization (tokenization) in VS Code. It contains:

- **Configuration & Build**: package.json, tsconfig.json, .npmrc — standard VS Code extension packaging
- **Test Harnesses**: colorizer.test.ts — Mocha test suite comparing TreeSitter vs TextMate tokenization performance
- **Grammar Fixtures**: test/colorize-fixtures/ — Three large fixture files (148k+ lines total) used for syntax highlighting benchmarks
  - test-checker.ts (146,620 lines) — TypeScript checker code copied from Microsoft/TypeScript repo
  - test-treeView.ts (2,067 lines) — TreeView implementation fixture
  - test.ts (111 lines) — Simple Game of Life fixture

- **Extension Stub**: colorizerTestMain.ts — Empty activation function; extension is test-only

This is purely a performance testing artifact for grammar/colorization benchmarking and has no relevance to IDE runtime functionality or a Tauri/Rust port.
