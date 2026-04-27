# File Locations: HTML Language Features Extension (Partition 12)

## Implementation

### Client-Side (LanguageClient)
- `extensions/html-language-features/client/src/htmlClient.ts` — Main LanguageClient initialization and feature registration
- `extensions/html-language-features/client/src/node/htmlClientMain.ts` — Node.js entry point, spawns language server process
- `extensions/html-language-features/client/src/browser/htmlClientMain.ts` — Browser/web entry point for webworker-based server
- `extensions/html-language-features/client/src/requests.ts` — Custom request/response protocol definitions for IDE features
- `extensions/html-language-features/client/src/customData.ts` — Custom HTML/CSS data loading mechanism
- `extensions/html-language-features/client/src/languageParticipants.ts` — Feature registration for languages (HTML, Handlebars)
- `extensions/html-language-features/client/src/autoInsertion.ts` — Auto-insertion feature handler
- `extensions/html-language-features/client/src/node/nodeFs.ts` — Node.js filesystem abstraction

### Server-Side (LanguageServer)
- `extensions/html-language-features/server/src/htmlServer.ts` — Core server logic, ServerCapabilities registration, protocol handlers
- `extensions/html-language-features/server/src/node/htmlServerMain.ts` — Node.js server entry point with stdio/socket transport setup
- `extensions/html-language-features/server/src/browser/htmlServerMain.ts` — Browser/webworker server entry point
- `extensions/html-language-features/server/src/browser/htmlServerWorkerMain.ts` — Worker thread bootstrapping for browser environment
- `extensions/html-language-features/server/src/requests.ts` — Custom request type definitions matching client protocol

### Language Mode Support
- `extensions/html-language-features/server/src/modes/languageModes.ts` — Multi-language mode orchestrator (HTML, CSS, JavaScript)
- `extensions/html-language-features/server/src/modes/htmlMode.ts` — HTML-specific completion, hover, diagnostic providers
- `extensions/html-language-features/server/src/modes/cssMode.ts` — CSS mode wrapper with language service integration
- `extensions/html-language-features/server/src/modes/javascriptMode.ts` — JavaScript mode with semantic tokens and formatting
- `extensions/html-language-features/server/src/modes/embeddedSupport.ts` — Multi-language embedded content handling (CSS/JS in HTML)
- `extensions/html-language-features/server/src/modes/formatting.ts` — Document formatting orchestration across modes
- `extensions/html-language-features/server/src/modes/htmlFolding.ts` — Range folding provider
- `extensions/html-language-features/server/src/modes/selectionRanges.ts` — Smart selection expansion provider
- `extensions/html-language-features/server/src/modes/semanticTokens.ts` — Semantic highlighting token provider
- `extensions/html-language-features/server/src/modes/javascriptSemanticTokens.ts` — JavaScript semantic token customization
- `extensions/html-language-features/server/src/modes/javascriptLibs.ts` — JavaScript library definitions loader

### Server Utilities
- `extensions/html-language-features/server/src/customData.ts` — Custom HTML/CSS data parsing and caching
- `extensions/html-language-features/server/src/languageModelCache.ts` — Document AST caching mechanism for performance
- `extensions/html-language-features/server/src/utils/documentContext.ts` — Document position/range utilities
- `extensions/html-language-features/server/src/utils/validation.ts` — URI/URI component validation and helpers
- `extensions/html-language-features/server/src/utils/strings.ts` — String manipulation utilities
- `extensions/html-language-features/server/src/utils/arrays.ts` — Array utility functions
- `extensions/html-language-features/server/src/utils/positions.ts` — Position/offset conversion helpers
- `extensions/html-language-features/server/src/utils/runner.ts` — Protocol request handling runner (wraps onRequest/onNotification)
- `extensions/html-language-features/server/src/node/nodeFs.ts` — Node.js filesystem abstraction

## Tests

- `extensions/html-language-features/server/src/test/completions.test.ts` — Completion provider tests
- `extensions/html-language-features/server/src/test/documentContext.test.ts` — Document context utility tests
- `extensions/html-language-features/server/src/test/embedded.test.ts` — Embedded language mode tests
- `extensions/html-language-features/server/src/test/folding.test.ts` — Range folding tests
- `extensions/html-language-features/server/src/test/formatting.test.ts` — Formatting tests
- `extensions/html-language-features/server/src/test/rename.test.ts` — Rename refactoring tests
- `extensions/html-language-features/server/src/test/selectionRanges.test.ts` — Selection range tests
- `extensions/html-language-features/server/src/test/semanticTokens.test.ts` — Semantic token tests
- `extensions/html-language-features/server/src/test/words.test.ts` — Word boundary detection tests

## Types / Interfaces

- `extensions/html-language-features/server/lib/jquery.d.ts` — jQuery type definitions for JavaScript analysis

## Configuration

- `extensions/html-language-features/package.json` — Extension manifest; declares vscode-languageclient ^10.0.0, contributes HTML settings
- `extensions/html-language-features/server/package.json` — Server package manifest; depends on vscode-languageserver ^10.0.0-next.16, vscode-html-languageservice, vscode-css-languageservice
- `extensions/html-language-features/package-lock.json` — Client dependency lock
- `extensions/html-language-features/server/package-lock.json` — Server dependency lock
- `extensions/html-language-features/client/tsconfig.json` — TypeScript config for client (both Node and Browser)
- `extensions/html-language-features/server/tsconfig.json` — TypeScript config for server
- `extensions/html-language-features/client/tsconfig.browser.json` — Browser-specific TypeScript overrides
- `extensions/html-language-features/server/tsconfig.browser.json` — Server browser TypeScript overrides
- `extensions/html-language-features/.vscode/settings.json` — VS Code workspace settings
- `extensions/html-language-features/.vscode/launch.json` — Debug launch configurations
- `extensions/html-language-features/server/.vscode/launch.json` — Server debug configurations
- `extensions/html-language-features/.vscode/tasks.json` — VS Code tasks
- `extensions/html-language-features/server/.vscode/tasks.json` — Server tasks
- `extensions/html-language-features/cgmanifest.json` — Third-party component manifest
- `extensions/html-language-features/server/lib/cgmanifest.json` — Server dependencies manifest
- `extensions/html-language-features/schemas/package.schema.json` — JSON schema for package.json validation
- `extensions/html-language-features/package.nls.json` — Localization strings for UI labels

## Documentation

- `extensions/html-language-features/README.md` — Extension overview and feature documentation
- `extensions/html-language-features/CONTRIBUTING.md` — Contribution guidelines

## Test Fixtures

- `extensions/html-language-features/server/src/test/pathCompletionFixtures/` — Test fixture directory with sample files for path completion
  - `src/feature.js` — JavaScript test file
  - `src/test.js` — JavaScript test file
  - `.foo.js` — Hidden file fixture

## Notable Clusters

- `extensions/html-language-features/client/` — 8 TypeScript files implementing LanguageClient integration; coordinates editor features (completion, hover, formatting) with language server; uses vscode-languageclient v10 API
- `extensions/html-language-features/server/src/modes/` — 11 files orchestrating multi-language support; HTML, CSS, JavaScript each have dedicated mode handlers; embeddedSupport coordinates cross-language interactions
- `extensions/html-language-features/server/src/` — 31 core server files; implements full LSP ServerCapabilities for HTML/CSS/JS; includes semantic tokens, folding, formatting, document symbols
- `extensions/html-language-features/server/src/test/` — 9 test files covering completion, embedded modes, formatting, semantic tokens, document navigation
- `extensions/html-language-features/server/src/utils/` — 7 utility modules providing document position handling, string manipulation, validation, and request routing

## Summary

This HTML language features extension demonstrates a complete bidirectional Language Server Protocol (LSP) implementation. The client/server architecture uses vscode-languageclient on the client side and vscode-languageserver on the server side, with custom request/response types defined in both `client/src/requests.ts` and `server/src/requests.ts`. The server supports multiple embedded language modes (HTML, CSS, JavaScript) with specialized handlers for completion, hover, semantic tokens, formatting, and document folding. For Tauri/Rust porting, this codebase reveals the LSP surface that must be maintained: ServerCapabilities declaration, TextDocumentSync modes, request/notification handlers for all IDE features, and the transport layer (stdio/socket). The modes structure shows that language intelligence is modular and stackable, crucial for a web-centric editor reimplementation.
