# Codebase Locator: extensions/types/

## Scope
This partition contains the TypeScript type definitions contribution package for VS Code.

### Implementation
- `extensions/types/lib.textEncoder.d.ts` — Global TextEncoder/TextDecoder type definitions for cross-runtime compatibility
- `extensions/types/lib.url.d.ts` — Global URL type definition for cross-runtime compatibility

## Summary
The `extensions/types/` directory contains 2 minimal TypeScript declaration files that provide polyfill type definitions for standard Web APIs (TextEncoder, TextDecoder, URL) across both browser and Node.js runtime environments. These are utility types used to ensure type consistency when VS Code code runs in different execution contexts. The declarations reference native Node.js `util` module types and DOM APIs, supporting the heterogeneous runtime environment that VS Code needs to support during an Electron-to-Tauri migration.
