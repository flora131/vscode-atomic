# Partition 41: src/typings/ - Porting Research Index

## Types / Interfaces

- `src/typings/electron-cross-app-ipc.d.ts` — Custom Electron IPC module for cross-app communication; directly relevant to architecture migration as Tauri would replace this with its own IPC/command system
- `src/typings/editContext.d.ts` — DOM EditContext API types for text editing; relevant to how text input is handled in the editor UI layer
- `src/typings/base-common.d.ts` — Global ambient declarations for timeout/interval/error handling; foundational APIs that would need Rust/Tauri equivalents
- `src/typings/vscode-globals-nls.d.ts` — NLS (i18n) message globals; documents internationalization infrastructure that would require reimplementation
- `src/typings/vscode-globals-product.d.ts` — Product metadata and CSS loader globals; tracks AMD2ESM migration and build-time asset loading patterns
- `src/typings/vscode-globals-ttp.d.ts` — Trusted Type Policy globals for web security; relevant if web components are preserved in Tauri port
- `src/typings/thenable.d.ts` — Promise-like abstraction layer; relevant to async patterns that would map to Rust futures
- `src/typings/css.d.ts` — CSS module declaration; tracks how stylesheets are imported and bundled
- `src/typings/crypto.d.ts` — Web Crypto API; documents cryptographic primitives available in both browser and Node.js contexts
- `src/typings/copilot-api.d.ts` — Copilot API client types; relevant to external service integration patterns that would need preservation in Rust backend

## Notable Clusters

- `src/typings/` — 10 files, 583 LOC total; contains ambient TypeScript declarations documenting critical infrastructure abstractions (IPC, async patterns, i18n, product metadata, security, cryptography, AI service integration) that form the boundary between VS Code's core logic and platform-specific implementations; essential reference for understanding which layers must be reimplemented for Tauri/Rust
