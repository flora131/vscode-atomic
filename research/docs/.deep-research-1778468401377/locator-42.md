# Ambient Type Definitions - Port Analysis (src/typings/)

## Summary

The `src/typings/` directory contains 9 ambient TypeScript declaration files (522 LOC total) that define runtime capabilities and third-party API contracts. These files define:

1. **Cross-platform runtime primitives** - Timer APIs, idle callbacks, error handling (base-common.d.ts)
2. **Browser/DOM cryptographic APIs** - Web Crypto API with SubtleCrypto, random UUID generation (crypto.d.ts)
3. **HTML5 text editing API** - EditContext interface for advanced text input handling (editContext.d.ts)
4. **Globalization infrastructure** - NLS message array and language configuration (vscode-globals-nls.d.ts)
5. **Build/dev tooling globals** - CSS loading, file root, product metadata (vscode-globals-product.d.ts)
6. **Security policy APIs** - Trusted Type Policy for CSP (vscode-globals-ttp.d.ts)
7. **Promise abstraction** - Thenable interface for promise library agnosticism (thenable.d.ts)
8. **Module loading** - CSS module declarations (css.d.ts)
9. **AI/Copilot integration** - CAPIClient types for Copilot API calls (copilot-api.d.ts)

## Implementation

### Core Runtime Primitives
- `/home/norinlavaee/projects/vscode-atomic/src/typings/base-common.d.ts` (40 LOC) - Declares `requestIdleCallback`, `setTimeout`, `clearTimeout`, `setInterval`, `clearInterval`, `IdleDeadline`, `Timeout`, and `ErrorConstructor.captureStackTrace`. Cross-context compatible with Node.js and browsers.

### Cryptographic & Security APIs
- `/home/norinlavaee/projects/vscode-atomic/src/typings/crypto.d.ts` (83 LOC) - Global `Crypto` and `SubtleCrypto` interfaces. Only `digest()` method is enabled; others are commented out. Provides `getRandomValues()` and `randomUUID()` for secure contexts.
- `/home/norinlavaee/projects/vscode-atomic/src/typings/vscode-globals-ttp.d.ts` (15 LOC) - `_VSCODE_WEB_PACKAGE_TTP` global for Trusted Type Policy during AMD2ESM migration.

### Text Input & Editing
- `/home/norinlavaee/projects/vscode-atomic/src/typings/editContext.d.ts` (123 LOC) - W3C EditContext interface (emerging web standard). Supports text updates, selection bounds, character bounds, and composition events. Enables advanced IME and text formatting integration.

### Globalization & Build Infrastructure
- `/home/norinlavaee/projects/vscode-atomic/src/typings/vscode-globals-nls.d.ts` (40 LOC) - `_VSCODE_NLS_MESSAGES` array and `_VSCODE_NLS_LANGUAGE` global. NLS messages are pre-compiled at build time across Electron main/renderer, utility process, Node.js, browser, and web worker contexts.
- `/home/norinlavaee/projects/vscode-atomic/src/typings/vscode-globals-product.d.ts` (47 LOC) - `_VSCODE_FILE_ROOT`, `_VSCODE_CSS_LOAD()`, `_VSCODE_PRODUCT_JSON`, `_VSCODE_PACKAGE_JSON` globals. CSS loader available during dev. Product/package metadata deprecated in favor of `IProductService`.

### Promise & Module Abstractions
- `/home/norinlavaee/projects/vscode-atomic/src/typings/thenable.d.ts` (12 LOC) - `Thenable<T>` interface extending `PromiseLike<T>`. Provides promise library agnosticism.
- `/home/norinlavaee/projects/vscode-atomic/src/typings/css.d.ts` (9 LOC) - Declare module statements for `"vs/css!*"` and `"*.css"` to recognize CSS as valid module imports.

### AI Integration
- `/home/norinlavaee/projects/vscode-atomic/src/typings/copilot-api.d.ts` (153 LOC) - Ambient types for `@vscode/copilot-api` package. Includes `CAPIClient` (Copilot API client), `CCAModel` (model capabilities, billing, limits), fetch/request options, extension information, token endpoints. Workaround for package's incompatible extensionless relative imports under `moduleResolution: "nodenext"`.

## Key Porting Considerations

### Browser/Web APIs vs Tauri/Rust
- **EditContext (editContext.d.ts)** - Requires Tauri equivalent for text input handling. Web API for advanced IME/composition events; Tauri would need custom bindings.
- **Crypto API (crypto.d.ts)** - Available in Tauri via Web APIs but needs secure context. Rust backend can handle crypto directly; no Tauri-specific adaptation needed.
- **Timer APIs (base-common.d.ts)** - Available in Tauri/web context. Can be replaced with Rust async/tokio for main logic.

### Electron-Specific Contexts
- **vscode-globals-nls.d.ts** explicitly mentions Electron main process, renderer process, and utility process. Porting requires:
  - NLS message compilation to handle Tauri's single-process model (no Electron main/renderer split)
  - Message lookup strategy may differ with unified Rust backend
  - Multi-context NLS still applies: browser, web worker, main Tauri process

### Build-Time Dependencies
- **vscode-globals-product.d.ts** - `_VSCODE_CSS_LOAD()` dev-time CSS loader requires Tauri equivalent. Product/package JSON injection depends on build system.
- **vscode-globals-ttp.d.ts** - Trusted Type Policy for CSP. Tauri inherits web context; may need CSP configuration adjustment.

### Module System
- **css.d.ts** - AMD2ESM migration. Tauri/web uses ES modules natively; no loader needed.

### Runtime Primitives
- **base-common.d.ts** - Cross-context compatibility (Node.js vs browser) must be maintained. Tauri's duality (Rust backend + web frontend) mirrors this split.
- **thenable.d.ts** - Promise agnosticism remains valid; can be preserved as-is.

### AI Integration
- **copilot-api.d.ts** - Copilot API client for token/chat endpoints. Network layer in Tauri would route through Rust backend or direct fetch from web frontend. No Electron-specific blocking; requires network permission configuration in Tauri.

## Notable Patterns

All files use ambient module declarations (`declare global`, `declare module`) rather than explicit exports. This indicates zero runtime dependencies between these typings—they're pure type annotations for globals and third-party APIs injected at load time or provided by the environment.

No Electron bindings (native modules, IPC, preload scripts) are present in this directory. This aligns with the fact that `src/typings/` defines *ambient* globals, not native integration points.
