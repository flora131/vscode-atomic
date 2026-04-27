# File Locations: Tokenization Performance Benchmarks (vscode-colorize-perf-tests)

## Overview
This partition contains a VS Code extension designed to measure and compare tokenization performance between TreeSitter and TextMate syntax engines. The benchmark suite provides baseline metrics essential for evaluating a hypothetical Rust/Tauri replacement of VS Code's core tokenization and colorization subsystem.

## Implementation
- `extensions/vscode-colorize-perf-tests/src/colorizerTestMain.ts` — Test harness initialization and Mocha configuration; sets up test reporting for CI/CD environments (Azure Pipelines, GitHub Actions)
- `extensions/vscode-colorize-perf-tests/src/colorizer.test.ts` — Core performance test suite; implements comparative benchmarking between TreeSitter (parse/capture/metadata phases) and TextMate tokenization engines with best/worst/first-run metrics

## Tests
- `extensions/vscode-colorize-perf-tests/test/colorize-fixtures/test-checker.ts` — TypeScript fixture (~8MB); stress test with complex language features
- `extensions/vscode-colorize-perf-tests/test/colorize-fixtures/test-treeView.ts` — TypeScript fixture (~900KB); comprehensive syntax tree operations
- `extensions/vscode-colorize-perf-tests/test/colorize-fixtures/test.ts` — TypeScript fixture (Game of Life implementation); moderate complexity baseline

## Configuration
- `extensions/vscode-colorize-perf-tests/package.json` — Extension metadata; declares `onLanguage:json` activation event and dependencies (jsonc-parser)
- `extensions/vscode-colorize-perf-tests/tsconfig.json` — Extends base tsconfig; compiles src to out directory with Node.js typings
- `extensions/vscode-colorize-perf-tests/.npmrc` — NPM configuration
- `extensions/vscode-colorize-perf-tests/.vscode/launch.json` — Debug launch configuration for extensionHost tests
- `extensions/vscode-colorize-perf-tests/.vscode/tasks.json` — Build task (npm compile) with watch support

## Examples / Fixtures
- `extensions/vscode-colorize-perf-tests/media/icon.png` — Extension icon asset

## Notable Clusters

**Performance Measurement Framework**: The test suite measures three distinct tokenization phases:
- **TreeSitter components**: parse time, capture time, metadata extraction time
- **TextMate engine**: unified tokenizeTime metric
- **Metrics tracked**: first run, best case, worst case across 6 test iterations

**Key Baseline Targets**: Fixtures range from ~8MB (stress test) to modest sizes, establishing throughput requirements any Rust tokenizer implementation would need to meet or exceed. The benchmark infrastructure directly evaluates whether TreeSitter or TextMate approaches are feasible for core IDE performance when reimplemented.

**CI Integration**: Test results are exported to JUnit XML format for both Azure DevOps and GitHub Actions, enabling trend analysis and performance regression detection across platforms and architectures.
