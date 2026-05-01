# File Locations: JSON Language Features Extension

## Partition Summary
The `extensions/json-language-features/` directory (19 TypeScript files, 3,042 LOC) contains VS Code's built-in JSON language support via a client-server LSP architecture. This is a canonical template for how VS Code's core IDE functionality is split between:
- **Client**: VS Code extension API layer (runs in Electron main process on desktop, Web Worker on browser)
- **Server**: Language service implementation (Node process on desktop, Web Worker on browser)

## Implementation Files

### Client Layer (Extension Host)
- `/extensions/json-language-features/client/src/jsonClient.ts` — Core LSP client setup, request/notification handlers, schema management, diagnostics coordination (940 LOC)
- `/extensions/json-language-features/client/src/node/jsonClientMain.ts` — Node.js entry point; spawns LSP server process, telemetry setup, file schema caching (176 LOC)
- `/extensions/json-language-features/client/src/browser/jsonClientMain.ts` — Browser entry point; instantiates Web Worker as LSP server, schema fetching via CORS (56 LOC)
- `/extensions/json-language-features/client/src/languageParticipants.ts` — Discovers language extensions that participate in JSON LSP; monitors extension changes (91 LOC)
- `/extensions/json-language-features/client/src/languageStatus.ts` — Status bar items for schema resolution errors, document symbol limits, language diagnostics
- `/extensions/json-language-features/client/src/node/schemaCache.ts` — ETag-based schema cache for offline access on Node.js

### Utilities (Client)
- `/extensions/json-language-features/client/src/utils/hash.ts` — Custom hash function for schema object identity (59 LOC)
- `/extensions/json-language-features/client/src/utils/urlMatch.ts` — Trusted domain pattern matching (localhost, wildcards, glob patterns) (108 LOC)

### Server Layer (Language Service)
- `/extensions/json-language-features/server/src/jsonServer.ts` — LSP server implementation; defines request/notification types, diagnostics handling, schema associations, validation logic (583 LOC)
- `/extensions/json-language-features/server/src/node/jsonServerMain.ts` — Node.js server entry point; connection setup, HTTP/file request services (76 LOC)
- `/extensions/json-language-features/server/src/node/jsonServerNodeMain.ts` — Node module wrapper (referenced by esbuild config)
- `/extensions/json-language-features/server/src/browser/jsonServerMain.ts` — Browser server entry point; Web Worker message reader/writer (32 LOC)
- `/extensions/json-language-features/server/src/browser/jsonServerWorkerMain.ts` — Worker initialization; defers to jsonServerMain after l10n setup (36 LOC)

### Utilities (Server)
- `/extensions/json-language-features/server/src/utils/validation.ts` — Diagnostic push/pull support registration (100+ LOC)
- `/extensions/json-language-features/server/src/utils/runner.ts` — Safe async/sync execution wrappers for LSP handlers
- `/extensions/json-language-features/server/src/utils/strings.ts` — String utility functions
- `/extensions/json-language-features/server/src/languageModelCache.ts` — Document parse cache with time-based eviction

## Configuration

### Extension Manifest
- `/extensions/json-language-features/package.json` — VS Code extension metadata:
  - **Main entry**: `./client/out/node/jsonClientMain` (Node.js)
  - **Browser entry**: `./client/dist/browser/jsonClientMain` (Web)
  - **Activation events**: `onLanguage:json`, `onLanguage:jsonc`, `onLanguage:snippets`, `onCommand:json.validate`
  - **Contributions**: JSON schema configuration schema, custom JSON sort/validate commands
  - **Dependencies**: `vscode-languageclient@^10.0.0-next.20`, `request-light`, `@vscode/extension-telemetry`

### TypeScript Configuration
- `/extensions/json-language-features/client/tsconfig.json` — Client compilation (ES2024, webworker, Node16 modules)
- `/extensions/json-language-features/client/tsconfig.browser.json` — Browser-specific overrides
- `/extensions/json-language-features/server/tsconfig.json` — Server compilation (ES2024, ESM modules)
- `/extensions/json-language-features/server/tsconfig.browser.json` — Browser server compilation

### Build Configuration
- `/extensions/json-language-features/esbuild.mts` — Parallel esbuild for client (Node.js) and server (Node.js ESM with `require` polyfill)
- `/extensions/json-language-features/esbuild.browser.mts` — Browser builds (client Web Worker + server Web Worker)

### Language Server Manifest
- `/extensions/json-language-features/server/package.json` — Language server package metadata:
  - **Version**: 1.3.4
  - **Main**: `./out/node/jsonServerMain` (CommonJS)
  - **Dependencies**: `vscode-json-languageservice@^6.0.0-next.1`, `vscode-languageserver@^10.0.0-next.16`, `jsonc-parser`, `request-light`

### Runtime Configuration
- `/extensions/json-language-features/.npmrc` — NPM configuration
- `/extensions/json-language-features/server/.npmrc` — Server NPM configuration
- `/extensions/json-language-features/.vscodeignore` — Files excluded from VSIX packaging

### Localization
- `/extensions/json-language-features/package.nls.json` — Localization strings for extension metadata

### Development
- `/extensions/json-language-features/.vscode/launch.json` — Debug launch configurations
- `/extensions/json-language-features/.vscode/tasks.json` — VS Code build tasks
- `/extensions/json-language-features/server/.vscode/launch.json` — Server debug config
- `/extensions/json-language-features/server/.vscode/tasks.json` — Server build tasks

## Documentation

- `/extensions/json-language-features/README.md` — Extension overview (links to VS Code JSON docs)
- `/extensions/json-language-features/CONTRIBUTING.md` — Contribution guidelines
- `/extensions/json-language-features/server/README.md` — Language server details

## Asset
- `/extensions/json-language-features/icons/json.png` — Extension icon

## Key Architecture Patterns

### Client-Server Boundary
The extension demonstrates the canonical LSP split:
1. **Client** (`jsonClient.ts`, `jsonClientMain.ts`) — Implements VS Code extension API:
   - UI commands (validate, sort, clear cache)
   - Status bar items and diagnostics visualization
   - Settings/configuration change handling
   - File system watching for schema updates
   - Trust/security checks (schema download settings, trusted domains)
   
2. **Server** (`jsonServer.ts`, `jsonServerMain.ts`) — Pure language service:
   - Completion, hover, symbol, folding, color, formatting
   - Schema resolution and validation
   - Request types: `vscode/content`, `json/schemaAssociations`, `json/schemaContent`, `json/validate`, `json/sort`
   - No direct UI or file system access (defers to client via LSP requests)

### Platform Abstraction
- **Node.js path**: Uses `ipc` transport, spawned Node process
- **Browser path**: Uses Web Workers, CORS-based schema fetching

### Schema Management
- ETag-based caching (Node.js only)
- Trust domain validation (glob patterns, localhost special-casing)
- Dynamic schema associations from extensions
- Per-folder schema configuration

### Diagnostic Models
- Supports both LSP push (`diagnostics/pull` if unavailable) and pull models
- Schema resolution errors trigger code actions for trust/settings
- Document symbol and folding range limits with status bar feedback

## Notable Implementation Details

### LanguageClient Instantiation
The `new LanguageClient()` pattern is instantiated twice (Node.js and browser):
- **Node.js**: `new LanguageClient(id, name, serverOptions, clientOptions)` (line 42, jsonClientMain.ts)
  - `serverOptions`: IPC transport to spawned Node process
  - `clientOptions`: Editor integration (document selector, middleware, diagnostics handling)
- **Browser**: `new LanguageClient(id, name, worker, clientOptions)` (line 21, jsonClientMain.ts)
  - `worker`: Web Worker instance
  - Same `clientOptions` API

### Request/Notification Protocol
Custom protocol messages defined as TypeScript types:
```typescript
namespace VSCodeContentRequest {
  export const type: RequestType<string, string, any> = new RequestType('vscode/content');
}
namespace SchemaAssociationNotification {
  export const type: NotificationType<ISchemaAssociations | SchemaConfiguration[]> = new NotificationType('json/schemaAssociations');
}
```

### Settings Propagation
Settings flow bidirectionally:
- Client reads `json.*`, `http.*` VS Code settings via `workspace.getConfiguration()`
- Client sends via `DidChangeConfigurationNotification` to server
- Server applies settings to language service state (validation, formatting limits)

## File Count and Organization
- **Total**: 19 files (excluding lock files and node_modules)
- **TypeScript source**: 14 files
- **Configuration**: 5 files (tsconfig, package.json, esbuild)
- **Client split**: Node.js (~8 files) vs. Browser (~4 files)
- **Server split**: Node.js (~5 files) vs. Browser (~3 files)
- **Shared code**: jsonClient.ts, jsonServer.ts, utilities

## Porting Implications for Tauri/Rust

### Dependencies to Replace
- `vscode-languageclient` (Node/browser) → Tauri command/event system + custom LSP transport
- `vscode-json-languageservice` (Node) → Rust JSON language service crate (e.g., `jsonrpc`, custom parser)
- `request-light` (HTTP requests) → Tauri `http` plugin or `reqwest`
- Electron IPC → Tauri `invoke()` / `listen()` for process communication

### Architectural Changes
1. **Transport Layer**: Replace LSP client setup (currently IPC or Web Worker) with Tauri backend process
2. **Schema Caching**: Implement ETag cache in Rust (currently Node.js file-based)
3. **Trust Domain Matching**: Port URL pattern matching logic to Rust
4. **Diagnostics Model**: Preserve LSP protocol; adapt to Tauri command routing
5. **Settings Propagation**: Replace VS Code `workspace.getConfiguration()` with Tauri state/event system
6. **File System Access**: Replace VS Code `workspace.fs` API with Tauri `fs` plugin

### Core Complexity
- **LSP Protocol Compliance**: The server is a full LSP implementation; porting requires preserving request/response/notification semantics
- **Multi-Language Support**: Current client discovers language participants dynamically; Rust server must either hard-code JSON or provide equivalent plugin/configuration mechanism
- **Schema Resolution**: Critical feature with network I/O, ETag caching, trust validation; each must be ported faithfully
- **Incremental Sync**: Clients request document range operations; server state management must handle concurrent edits

The extension's client-server split is intentional and optimized for VS Code's multi-process architecture. A Tauri port would require similar separation (main process host ↔ Rust backend), but with different IPC primitives and runtime constraints.
