# Partition 44: TypeScript Ambient Declarations (`src/typings/`)

## Overview

The `src/typings/` directory contains 9 ambient TypeScript declaration files (430 LOC) that define global types and interfaces the renderer assumes are available from:
- Electron native APIs and custom modules
- Node.js runtime capabilities
- Browser/DOM APIs (polyfills and probes)
- VS Code build-time globals for localization and resource management

These declarations establish contracts between the frontend JavaScript/TypeScript code and its execution environment (Electron, Node.js, Browser). A Tauri/Rust port must replace or reimplement the capabilities advertised by each ambient module.

## Types/Interfaces

### Electron Cross-App IPC Module
**File:** `src/typings/electron-cross-app-ipc.d.ts` (62 LOC)

Declares a custom Electron namespace extension for secure inter-process communication between a host app and embedded Electron app (MiniApp):

- `Electron.CrossAppIPCMessageEvent` — Message payload with transferred `MessagePortMain` objects
- `Electron.CrossAppIPCDisconnectReason` — Union type for disconnection events ('peer-disconnected', 'handshake-failed', 'connection-failed', 'connection-timeout')
- `Electron.CrossAppIPC` — EventEmitter interface with methods: `connect()`, `close()`, `postMessage()`, property: `connected`, `isServer`
- `Electron.CrossAppIPCModule` — Factory interface with `createCrossAppIPC()` method
- `Electron.Main.crossAppIPC` — Module namespace binding (optional)
- `Electron.CrossProcessExports.crossAppIPC` — Process export binding (optional)

### VS Code Product & Build-Time Globals
**File:** `src/typings/vscode-globals-product.d.ts` (48 LOC)

Declares global variables injected at build/runtime:

- `_VSCODE_FILE_ROOT: string` — Root path for resource resolution
- `_VSCODE_CSS_LOAD: (module: string) => void` — CSS loader callback (dev-time only)
- `_VSCODE_PRODUCT_JSON: Record<string, any>` — Product metadata (deprecated, prefer `IProductService`)
- `_VSCODE_PACKAGE_JSON: Record<string, any>` — Package metadata (deprecated, prefer `IProductService`)
- `_VSCODE_DISABLE_CSS_IMPORT_MAP: boolean | undefined` — Flag to disable CSS import map loading
- `_VSCODE_USE_RELATIVE_IMPORTS: boolean | undefined` — Flag for relative import resolution

### Localization (NLS) Globals
**File:** `src/typings/vscode-globals-nls.d.ts` (41 LOC)

Declares global variables for the National Language Support (NLS) system available across all execution contexts (Electron main/renderer, utility process, Node.js, browser, web worker):

- `_VSCODE_NLS_MESSAGES: string[]` — Array of localized message strings (indexed by build-time message IDs)
- `_VSCODE_NLS_LANGUAGE: string | undefined` — Current language code (e.g., 'en', 'de', 'pt-br')

The build system strips English strings and replaces them with array indices; runtime lookup occurs via these globals.

### Trusted Types Policy (TTP) Global
**File:** `src/typings/vscode-globals-ttp.d.ts` (15 LOC)

Declares browser security policy for Trusted Types:

- `_VSCODE_WEB_PACKAGE_TTP: Pick<TrustedTypePolicy, 'name' | 'createScriptURL'> | undefined` — Optional Trusted Type policy restricting script URL creation in web builds

### Base Common Runtime Capabilities
**File:** `src/typings/base-common.d.ts` (41 LOC)

Declares global functions and interfaces probed/polyfilled across all JavaScript contexts:

- `IdleDeadline` interface — Deadline object for requestIdleCallback
- `requestIdleCallback(callback, options?): number` — Schedule callback during idle time
- `cancelIdleCallback(handle): void` — Cancel scheduled idle callback
- `TimeoutHandle` — Opaque timeout handle type
- `Timeout` — Alias for timeout handles
- `setTimeout(handler, timeout?, ...args): Timeout`
- `clearTimeout(timeout: Timeout | undefined): void`
- `setInterval(callback, delay?, ...args): Timeout`
- `clearInterval(timeout: Timeout | undefined): void`
- `ErrorConstructor.captureStackTrace(targetObject, constructorOpt?): void` — V8 stack trace API
- `ErrorConstructor.stackTraceLimit: number` — Stack trace depth limit

### Web Crypto API
**File:** `src/typings/crypto.d.ts` (84 LOC)

Partial declaration of the Web Crypto API available globally in all secure contexts (browser, Node.js 15+, Electron):

- `SubtleCrypto` interface — Cryptographic operations (only `digest()` is uncommented; others available but commented out)
- `Crypto` interface — Global crypto object with:
  - `readonly subtle: SubtleCrypto` — Subtle cryptography API
  - `getRandomValues<T extends ArrayBufferView | null>(array: T): T` — Fill buffer with random values
  - `randomUUID(): string` — Generate UUID v4
- `var Crypto` — Constructor
- `var crypto: Crypto` — Global instance

### EditContext API (Experimental)
**File:** `src/typings/editContext.d.ts` (124 LOC)

Declares experimental browser API for advanced text editing controls and composition events:

- `EditContext` interface extends `EventTarget` with:
  - Methods: `updateText()`, `updateSelection()`, `updateControlBounds()`, `updateSelectionBounds()`, `updateCharacterBounds()`, `attachedElements()`
  - Getters: `text`, `selectionStart`, `selectionEnd`, `characterBoundsRangeStart`, `characterBounds()`
  - Event handlers: `ontextupdate`, `ontextformatupdate`, `oncharacterboundsupdate`, `oncompositionstart`, `oncompositionend`
- `EditContextInit` interface — Constructor options
- `EditContextEventHandlersEventMap` — Event type mapping
- `TextUpdateEvent` class — Fired on text mutations, carries `updateRangeStart`, `updateRangeEnd`, `text`, `selectionStart`, `selectionEnd`
- `TextFormatUpdateEvent` class — Formatting events with `getTextFormats()`
- `CharacterBoundsUpdateEvent` class — Character position updates
- `TextFormat`, `UnderlineStyle`, `UnderlineThickness` — Text formatting metadata
- `HTMLElement.editContext?: EditContext` — Element attachment point

### CSS Module Loading
**File:** `src/typings/css.d.ts` (10 LOC)

Declares module resolution for CSS imports:

- `declare module "vs/css!*"` — Matches VS Code's AMD CSS loader syntax (e.g., `require("vs/css!path/to/file")`)
- `declare module "*.css"` — Standard CSS module imports

### Thenable (Promise-like abstraction)
**File:** `src/typings/thenable.d.ts` (13 LOC)

Declares a duck-typed promise interface:

- `Thenable<T> extends PromiseLike<T>` — Common denominator for multiple promise libraries (Q, WinJS, jQuery, native Promises)

## Notable Clusters

### Electron/Native Runtime Capabilities
- `src/typings/electron-cross-app-ipc.d.ts` — Custom Electron module (macOS/Windows code-signed IPC)

**Renderer assumes:** Electron's custom cross-process communication for MiniApp scenarios. A Tauri port must implement equivalent macOS Mach port + Windows named pipe IPC.

### Build & Runtime Injection Globals
- `src/typings/vscode-globals-product.d.ts` — Product/package metadata + CSS loader
- `src/typings/vscode-globals-nls.d.ts` — NLS message arrays and language code

**Renderer assumes:** Build system injects these globals; they're read-only at runtime. A Tauri port must inject equivalents via build-time codegen or runtime initialization.

### Browser/Platform APIs (Polyfills)
- `src/typings/base-common.d.ts` — requestIdleCallback, timeout/interval, Error.captureStackTrace
- `src/typings/crypto.d.ts` — Web Crypto API
- `src/typings/editContext.d.ts` — EditContext API (experimental)
- `src/typings/vscode-globals-ttp.d.ts` — Trusted Types policy

**Renderer assumes:** These exist in the JavaScript runtime. A Tauri port must either:
  1. Run in a browser/WebView environment that provides them natively, or
  2. Implement polyfills in JavaScript or via Tauri bindings

### CSS & Module Resolution
- `src/typings/css.d.ts` — AMD CSS loader + standard CSS modules
- `src/typings/thenable.d.ts` — Promise-like interface

**Renderer assumes:** Build tooling recognizes these module patterns. A Tauri build must configure similar resolution.

## Usage References

Key files consuming these ambient declarations:

- `src/vs/platform/secrets/electron-main/macOSCrossAppSecretSharing.ts` — Uses `Electron.crossAppIPC`
- `src/vs/platform/update/electron-main/crossAppUpdateIpc.ts` — Uses cross-app IPC
- `src/vs/platform/crossAppIpc/electron-main/crossAppIpcService.ts` — Central cross-app IPC service
- `src/vs/nls.ts` — Uses `_VSCODE_NLS_MESSAGES`, `_VSCODE_NLS_LANGUAGE`
- `src/vs/platform/product/common/product.ts` — Uses `_VSCODE_PRODUCT_JSON`, `_VSCODE_PACKAGE_JSON`
- `src/bootstrap-esm.ts` — Uses `_VSCODE_CSS_LOAD`, `_VSCODE_FILE_ROOT`
- `src/vs/base/common/async.ts` — Uses `requestIdleCallback`, timeout/interval, `Error.captureStackTrace`
- `src/vs/amdX.ts` — AMD module system setup, uses CSS loading
- `src/vscode-dts/vscode.d.ts` — Public extension API surface (references ambient types)

## Summary

The `src/typings/` partition defines the complete contract between VS Code's renderer code and its host environment. It encompasses:

1. **Custom Electron extensions** for secure inter-process communication (macOS/Windows code-signed channels)
2. **Build-time injected globals** for product metadata, file roots, and CSS loading behavior
3. **Localization infrastructure** (NLS messages and language selection)
4. **Standard Web APIs** (Crypto, EditContext, requestIdleCallback) that the renderer assumes are available
5. **Security policies** (Trusted Types) for web-based deployments
6. **Module resolution shims** for CSS and Thenable abstractions

A Tauri/Rust port must strategically replace each category:
- **Electron IPC** → Tauri's IPC (Command/Listen) or WebView bridge
- **Build-time globals** → Codegen or initial JavaScript bootstrap
- **Web APIs** → Implement in JavaScript layer or via Tauri bindings
- **Module resolution** → Configure in Vite/build toolchain

The ambient declarations themselves don't contain implementation; they are type-only contracts. Reading actual implementations of these capabilities requires examining files in `src/vs/platform/`, `src/vs/base/`, and `src/vs/code/electron-main/`.
