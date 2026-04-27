# CSS Language Features Extension - File Locator

## Partition 28 of 79: `extensions/css-language-features/`
CSS/LESS/SCSS language support via LSP client-server architecture.

---

## Implementation

### Client Extension (Extension Host Process)
- `extensions/css-language-features/client/src/cssClient.ts` (193 LOC) - Main client initialization, LanguageClient setup, formatter registration, custom data handling
- `extensions/css-language-features/client/src/customData.ts` (88 LOC) - Custom CSS data source management, file watching
- `extensions/css-language-features/client/src/requests.ts` (90 LOC) - Custom request handlers for client-side operations

#### Drop/Paste Feature
- `extensions/css-language-features/client/src/dropOrPaste/dropOrPasteResource.ts` - Resource drop/paste handler
- `extensions/css-language-features/client/src/dropOrPaste/shared.ts` - Shared utilities
- `extensions/css-language-features/client/src/dropOrPaste/uriList.ts` - URI list parsing

#### Environment-Specific Entry Points
- `extensions/css-language-features/client/src/node/cssClientMain.ts` - Node.js client entry point, instantiates LanguageClient with Node server process
- `extensions/css-language-features/client/src/browser/cssClientMain.ts` - Browser client entry point, instantiates LanguageClient with Web Worker
- `extensions/css-language-features/client/src/node/nodeFs.ts` - Node.js filesystem abstraction

### Language Server (Separate Node.js Process or Web Worker)
- `extensions/css-language-features/server/src/cssServer.ts` (392 LOC) - Core server implementation, LSP request handlers (completion, hover, diagnostics, formatting, etc.)
- `extensions/css-language-features/server/src/customData.ts` (38 LOC) - Custom CSS property and pseudo-selector data loading
- `extensions/css-language-features/server/src/languageModelCache.ts` (82 LOC) - Parsed CSS document caching for performance
- `extensions/css-language-features/server/src/requests.ts` (103 LOC) - Custom request handlers for server-side operations

#### Server Utilities
- `extensions/css-language-features/server/src/utils/validation.ts` - CSS validation utilities
- `extensions/css-language-features/server/src/utils/strings.ts` - String manipulation helpers
- `extensions/css-language-features/server/src/utils/runner.ts` - Test runner utility
- `extensions/css-language-features/server/src/utils/documentContext.ts` - Document context for path resolution

#### Environment-Specific Server Entry Points
- `extensions/css-language-features/server/src/node/cssServerMain.ts` - Node.js server entry point
- `extensions/css-language-features/server/src/node/cssServerNodeMain.ts` - Alternative Node.js server initialization
- `extensions/css-language-features/server/src/node/nodeFs.ts` - Node.js filesystem adapter for server
- `extensions/css-language-features/server/src/browser/cssServerMain.ts` - Browser server entry point
- `extensions/css-language-features/server/src/browser/cssServerWorkerMain.ts` - Web Worker initialization for browser server

---

## Tests

### Unit Tests (Server)
- `extensions/css-language-features/server/src/test/completion.test.ts` - Completion provider tests
- `extensions/css-language-features/server/src/test/links.test.ts` - Link detection and navigation tests

### Test Fixtures
- `extensions/css-language-features/server/test/pathCompletionFixtures/` - 10 files covering CSS import path completion scenarios
  - `index.html`, `about/about.html`, `about/about.css`
  - `scss/_foo.scss`, `scss/main.scss`
  - `src/feature.js`, `src/test.js`, `src/data/foo.asar`
  - `.foo.js`

- `extensions/css-language-features/server/test/linksTestFixtures/` - Fixtures for link resolution testing
  - `node_modules/foo/package.json`
  - `.gitignore`

### Test Execution
- `extensions/css-language-features/server/test/index.js` - Test harness entry point

---

## Types / Interfaces

### Shared Type Definition (Main Client)
- `extensions/css-language-features/client/src/cssClient.ts` defines:
  - `LanguageClientConstructor` - Type for LanguageClient factory function
  - `Runtime` - Runtime environment abstraction (TextDecoder, fs service)
  - `FormatterRegistration` - Formatter tracking interface
  - `CSSFormatSettings` - Configuration shape for CSS formatting options

---

## Configuration

### Extension Manifest
- `extensions/css-language-features/package.json` - Activates on CSS/LESS/SCSS files, registers LanguageClient, defines 3 language-specific configuration blocks with 40+ lint/format options
- `extensions/css-language-features/package.nls.json` - Localization strings

### TypeScript Configuration
- `extensions/css-language-features/client/tsconfig.json` - Client compilation settings
- `extensions/css-language-features/client/tsconfig.browser.json` - Browser-specific TypeScript options
- `extensions/css-language-features/server/tsconfig.json` - Server compilation settings
- `extensions/css-language-features/server/tsconfig.browser.json` - Server browser build options

### Build Configuration
- `extensions/css-language-features/esbuild.mts` - Node.js bundle configuration
- `extensions/css-language-features/esbuild.browser.mts` - Browser bundle configuration

### Server Package
- `extensions/css-language-features/server/package.json` - vscode-css-languageservice dependency, vscode-languageserver dependency, dual ESM/browser output

### Development Configuration
- `extensions/css-language-features/.vscode/launch.json` - Debug configurations
- `extensions/css-language-features/.vscode/tasks.json` - Build tasks
- `extensions/css-language-features/.vscode/settings.json` - Workspace settings
- `extensions/css-language-features/server/.vscode/launch.json` - Server debug config
- `extensions/css-language-features/server/.vscode/tasks.json` - Server build tasks

### Metadata
- `extensions/css-language-features/.vscodeignore` - Packaging exclusions
- `extensions/css-language-features/.npmrc` - npm configuration
- `extensions/css-language-features/server/.npmrc` - Server npm configuration

### JSON Schema
- `extensions/css-language-features/schemas/package.schema.json` - Validation schema for extension package.json

---

## Examples / Fixtures

### Test Data Sets
- Path completion: HTML/CSS files with import statements across directories
- Links: CSS @import and url() resolution with node_modules handling
- Completion scenarios: Property names, values, vendor prefixes, at-rules

---

## Documentation

- `extensions/css-language-features/README.md` - Extension overview, capabilities, configuration
- `extensions/css-language-features/CONTRIBUTING.md` - Contribution guidelines

---

## Notable Clusters

### LanguageClient Dual-Runtime Strategy
Files in `client/src/{node,browser}` and `server/src/{node,browser}` demonstrate VS Code's pattern for supporting both Electron (Node.js) and web contexts:
- **Node path**: spawns standalone Node process via `LanguageServerOptions`
- **Browser path**: uses Web Worker via `WorkerLanguageClient`
Both converge to the same `vscode-languageclient` API.

### Separate Language Server Package
`server/` is a distinct npm package (`vscode-css-languageserver`) with independent dependencies:
- `vscode-css-languageservice` (parsing/analysis library)
- `vscode-languageserver` (LSP protocol implementation)
Can be deployed standalone or embedded in the extension.

### Configuration Scope (3 Languages × ~20 options each = 60+ settings)
Single extension manages CSS, SCSS, and LESS with separate configuration sections, each with lint rules, formatting, hover, and completion behaviors. See `contributes.configuration` in root `package.json` (lines 35-971).

### Custom Request Pattern
Both client and server expose custom requests (not part of standard LSP) via `requests.ts` files. Client initiates, server responds. Used for filesystem access in sandboxed/browser contexts.

---

## Structural Summary

The CSS extension exemplifies VS Code's LSP client plugin architecture:

- **30 files, ~3,647 LOC** across implementation, tests, fixtures
- **Clear separation**: Client (9 impl files) / Server (13 impl files) / Tests (2) / Fixtures (12)
- **Platform abstraction**: Node.js and browser runtimes via environment-specific entry points
- **Dual-module packaging**: Client in main extension, server as separate npm package
- **Production dependencies**: Only `vscode-languageclient` for client; `vscode-css-languageservice` + `vscode-languageserver` for server
- **Porting implications**: Would require replacing `LanguageClient` instantiation (lines 22 & 32 in client mains) and translating server LSP handlers to native Rust crates (tower-lsp, ropey, tree-sitter for CSS parsing)

Key instantiation points for porting study:
- `extensions/css-language-features/client/src/node/cssClientMain.ts:32` - Node LanguageClient instantiation
- `extensions/css-language-features/client/src/browser/cssClientMain.ts:22` - Browser/Worker LanguageClient instantiation
- `extensions/css-language-features/server/src/cssServer.ts` - LSP server request handler implementations
