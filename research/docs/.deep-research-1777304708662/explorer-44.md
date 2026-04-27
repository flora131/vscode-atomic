# Partition 44 of 79 — Findings

## Scope
`src/typings/` (9 files, 430 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
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

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
### Files Analysed

| File | LOC | Role |
|---|---|---|
| `src/typings/electron-cross-app-ipc.d.ts` | 62 | Declares `Electron.CrossAppIPC` and `Electron.CrossAppIPCModule` for code-signed inter-app IPC |
| `src/typings/vscode-globals-product.d.ts` | 48 | Declares build-time globals: `_VSCODE_FILE_ROOT`, `_VSCODE_CSS_LOAD`, `_VSCODE_PRODUCT_JSON`, `_VSCODE_PACKAGE_JSON`, `_VSCODE_DISABLE_CSS_IMPORT_MAP`, `_VSCODE_USE_RELATIVE_IMPORTS` |
| `src/typings/vscode-globals-nls.d.ts` | 41 | Declares `_VSCODE_NLS_MESSAGES: string[]` and `_VSCODE_NLS_LANGUAGE: string \| undefined` for build-stripped i18n |
| `src/typings/vscode-globals-ttp.d.ts` | 15 | Declares `_VSCODE_WEB_PACKAGE_TTP` as a partial `TrustedTypePolicy` |
| `src/typings/base-common.d.ts` | 41 | Declares `requestIdleCallback`/`cancelIdleCallback`, a nominal `Timeout`/`TimeoutHandle` type, and `Error.captureStackTrace` |
| `src/typings/crypto.d.ts` | 84 | Declares partial Web Crypto API: `SubtleCrypto.digest`, `Crypto.getRandomValues`, `Crypto.randomUUID`, and a global `crypto` variable |
| `src/typings/editContext.d.ts` | 124 | Declares the experimental W3C EditContext API: `EditContext`, `TextUpdateEvent`, `TextFormatUpdateEvent`, `CharacterBoundsUpdateEvent`, and `HTMLElement.editContext` extension |
| `src/typings/css.d.ts` | 10 | Module shims: `declare module "vs/css!*"` and `declare module "*.css"` to satisfy TypeScript module resolution for CSS imports |
| `src/typings/thenable.d.ts` | 13 | Declares `interface Thenable<T> extends PromiseLike<T>` as a cross-library promise compatibility type |

---

### Per-File Notes

#### `electron-cross-app-ipc.d.ts`

- Augments the `Electron` namespace at lines 15–61, adding `CrossAppIPCMessageEvent` (line 17), `CrossAppIPCDisconnectReason` (line 24), `CrossAppIPC` (line 30), and `CrossAppIPCModule` (line 50).
- `CrossAppIPC` (line 30) extends `NodeJS.EventEmitter` with typed event methods for three events: `'connected'`, `'message'` (carrying `CrossAppIPCMessageEvent` with a `ports: Electron.MessagePortMain[]` field at line 21), and `'disconnected'` (with a `CrossAppIPCDisconnectReason` at line 24).
- Connection lifecycle methods on `CrossAppIPC` (lines 43–46): `connect()`, `close()`, `postMessage(message, transferables?)`, plus read-only `connected` and `isServer`.
- `CrossAppIPCModule` exposes a single factory at line 51: `createCrossAppIPC(): CrossAppIPC`.
- The module is exposed at two Electron attachment points: `Electron.Main.crossAppIPC` (line 55) and `Electron.CrossProcessExports.crossAppIPC` (line 59), both typed `CrossAppIPCModule | undefined`, indicating an optional custom Electron build feature.
- The comment at lines 7–13 specifies the authentication mechanism: macOS Mach ports for code-signature verification, Windows named pipes — both OS-native mechanisms entirely outside the web platform.

**Host assumptions:** The renderer/main process expects a custom-patched Electron binary that exposes `crossAppIPC` on `electron`'s module exports. The `MessagePortMain` transfer type is an Electron-main-process-only construct.

---

#### `vscode-globals-product.d.ts`

- Six `var` declarations inside `declare global` (lines 13–43).
- `_VSCODE_FILE_ROOT: string` (line 13) — a runtime-injected base URL used by the resource loader.
- `_VSCODE_CSS_LOAD: (module: string) => void` (line 19) — a loader callback for development-time CSS injection; the comment at line 17 says not to call directly but to `import 'some.css'`.
- `_VSCODE_PRODUCT_JSON: Record<string, any>` (line 24) and `_VSCODE_PACKAGE_JSON: Record<string, any>` (line 28) — deprecated legacy access paths for product/package data; both marked `@deprecated` in favour of `IProductService`.
- `_VSCODE_DISABLE_CSS_IMPORT_MAP: boolean | undefined` (line 35) and `_VSCODE_USE_RELATIVE_IMPORTS: boolean | undefined` (line 43) — deprecated development-time bundler flags.

**Host assumptions:** All six globals must be present on `globalThis` before any module that consumes them runs. They are injected by the bootstrap layer (not by the browser or Node.js itself).

---

#### `vscode-globals-nls.d.ts`

- Two `var` declarations inside `declare global` (lines 30–36).
- `_VSCODE_NLS_MESSAGES: string[]` (line 30) — the full translated message array; English strings are stripped at build time and replaced with numeric indices.
- `_VSCODE_NLS_LANGUAGE: string | undefined` (line 36) — the BCP-47 language tag (e.g., `'de'`, `'pt-br'`).
- The comment at lines 9–21 specifies that these globals must be present in every execution context: Electron main, Electron renderer, Utility Process, Node.js, browser, and web workers.

**Host assumptions:** The bootstrap for each process type must populate these globals before any `nls.localize`/`nls.localize2` call executes. The strings array is flat and indexed by position — the index mapping is established at build time.

---

#### `vscode-globals-ttp.d.ts`

- Single `var` declaration at line 10: `_VSCODE_WEB_PACKAGE_TTP: Pick<TrustedTypePolicy, 'name' | 'createScriptURL'> | undefined`.
- Typed as `Pick<TrustedTypePolicy, ...>` — only the `name` and `createScriptURL` facets of the browser Trusted Types API are required.
- `undefined` is explicitly allowed, indicating the feature is treated as optional/conditional.

**Host assumptions:** Browsers that enforce Trusted Types CSP headers require a policy object here before dynamic script URL construction. If absent, the code falls through to `window.trustedTypes?.createPolicy(...)`.

---

#### `base-common.d.ts`

- `IdleDeadline` interface (lines 12–15): `didTimeout: boolean` and `timeRemaining(): number`.
- `requestIdleCallback(callback, options?)` (line 17) and `cancelIdleCallback(handle)` (line 18): standard browser idle scheduling API, typed for use in the `/common/` layer.
- `TimeoutHandle` interface (lines 23–24): a nominal type using the `readonly _: never` trick to prevent raw `number` assignment. `Timeout` is aliased to `TimeoutHandle` at line 25.
- `setTimeout` (line 25), `clearTimeout` (line 26), `setInterval` (line 28), `clearInterval` (line 29): redeclared with the `Timeout` return type instead of the Node.js `NodeJS.Timeout` type, so they unify across browser and Node contexts without importing `NodeJS` types in `/common/` code.
- `ErrorConstructor.captureStackTrace` (line 35) and `stackTraceLimit` (line 36): V8-specific stack introspection methods, typed globally for use in both Node.js (Electron) and V8-powered browser contexts.

**Host assumptions:** All execution environments (main, renderer, workers) share a global `setTimeout`/`setInterval` with numeric-handle semantics. `captureStackTrace` is assumed to be present (V8-specific).

---

#### `crypto.d.ts`

- Partial `SubtleCrypto` interface (lines 18–49) — only `digest(algorithm, data): Promise<ArrayBuffer>` (line 26) is uncommented; all other methods (`decrypt`, `encrypt`, `sign`, `verify`, etc.) are commented out at lines 20–48.
- `Crypto` interface (lines 56–73): declares `subtle: SubtleCrypto` (line 62), `getRandomValues<T>(array: T): T` (line 66), and `randomUUID(): \`${string}-${string}-${string}-${string}-${string}\`` (line 72).
- `var Crypto` constructor declaration (lines 75–78) and `var crypto: Crypto` (line 80).
- The comment at lines 6–8 explains the rationale: Web Crypto is available globally in browsers and Node.js, so it is safe to declare in the `/common/` layer without a DOM dependency.

**Host assumptions:** A global `crypto` object with `subtle.digest`, `getRandomValues`, and `randomUUID` must be present. This is satisfied natively in browsers, Node 15+, and Electron. The commented-out operations (encrypt, sign, etc.) are not currently consumed by the `/common/` layer.

---

#### `editContext.d.ts`

- `EditContext` interface (lines 8–43): the experimental W3C EditContext API for advanced IME and composition input, extending `EventTarget`. Methods: `updateText`, `updateSelection`, `updateControlBounds`, `updateSelectionBounds`, `updateCharacterBounds`, `attachedElements`. Properties: `text`, `selectionStart`, `selectionEnd`, `characterBoundsRangeStart`, `characterBounds()`. Event handler setters for `ontextupdate`, `ontextformatupdate`, `oncharacterboundsupdate`, `oncompositionstart`, `oncompositionend` (lines 25–37).
- `EditContextInit` (lines 45–49): constructor options object.
- `EditContextEventHandlersEventMap` (lines 51–57): event type map linking `'textupdate'` → `TextUpdateEvent`, `'textformatupdate'` → `TextFormatUpdateEvent`, `'characterboundsupdate'` → `CharacterBoundsUpdateEvent`.
- `TextUpdateEvent` class (lines 61–68): carries `updateRangeStart`, `updateRangeEnd`, `text`, `selectionStart`, `selectionEnd`.
- `TextFormatUpdateEvent` (lines 100–103): provides `getTextFormats(): TextFormat[]`.
- `CharacterBoundsUpdateEvent` (lines 109–113): carries `rangeStart`, `rangeEnd` for pixel-accurate IME candidate window positioning.
- `UnderlineStyle` (line 97) and `UnderlineThickness` (line 98): IME underline decoration types.
- `HTMLElement` interface augmented at line 121–123 with optional `editContext?: EditContext`.

**Host assumptions:** The host rendering engine must implement the (as-of-2024 experimental) W3C EditContext API. Currently Chrome/Edge 121+. Not available in Firefox, Safari, or any non-Chromium WebView. Electron's Chromium base satisfies this.

---

#### `css.d.ts`

- Two module declarations (lines 8–9): `declare module "vs/css!*" { }` and `declare module "*.css" { }`.
- Both produce empty module shapes, meaning any `import` of these paths satisfies TypeScript's type checker but contributes no exported types.
- `"vs/css!*"` is the legacy AMD/RequireJS CSS loader plugin prefix used before the AMD-to-ESM migration.

**Host assumptions:** The build toolchain (esbuild, webpack, or the custom AMD loader) intercepts `vs/css!*` and `*.css` imports and performs actual CSS injection. TypeScript only needs to know the imports are valid module specifiers.

---

#### `thenable.d.ts`

- Single interface declaration at line 12: `interface Thenable<T> extends PromiseLike<T> { }`.
- Extends the standard `PromiseLike<T>` (which requires only a `.then()` method), adding no additional members — it is a pure nominal alias.
- Used across the VS Code extension API (`vscode.d.ts`) and internal common utilities to accept any then-able without committing to a specific promise implementation.

**Host assumptions:** No host environment assumption; this is a pure TypeScript type that compiles away entirely. It is defined globally (not in a module) so it is available in all `.ts` files without an import.

---

### Cross-Cutting Synthesis

The nine ambient files carve out five distinct capability categories that a Tauri/Rust port must remap.

**IPC transport** (`electron-cross-app-ipc.d.ts`): The `CrossAppIPC` contract is entirely Electron-specific — it depends on Mach ports (macOS) and named pipes (Windows) threaded through a custom Electron binary, with `MessagePortMain` as the transfer primitive. Tauri's IPC uses a different mechanism (Rust `tauri::command` invocations over a JSON bridge or `ipc://` protocol), and has no equivalent of MessagePort transfers between OS processes. This is the deepest structural dependency; replacing it requires designing a new authenticated cross-process channel in Rust.

**Bootstrap globals** (`vscode-globals-product.d.ts`, `vscode-globals-nls.d.ts`, `vscode-globals-ttp.d.ts`): Six product globals and two NLS globals are injected onto `globalThis` before module evaluation. In Electron these are written by the bootstrap `.js` files that run before the renderer bundle. In Tauri the equivalent injection point is the JS preload or a Rust-side `evaluate_script` call before the WebView's page load. The NLS globals are especially pervasive — `src/vs/nls.ts:7` reads `globalThis._VSCODE_NLS_MESSAGES` on every `localize` call, and the comment in `vscode-globals-nls.d.ts:9-21` enumerates every process type that needs them.

**Scheduler and timer types** (`base-common.d.ts`): The nominal `Timeout` type and the `requestIdleCallback` polyfill are designed to unify Node.js and browser execution. Tauri runs a WebView, so the browser-side `requestIdleCallback` is available in the renderer. `captureStackTrace` (line 35) is V8-only; if Tauri's future WASM/Rust code paths ever need it, a stub would be required.

**Cryptography** (`crypto.d.ts`): Only `subtle.digest`, `getRandomValues`, and `randomUUID` are actively consumed. All three are part of the W3C Web Crypto API available in Tauri's WebView renderer and via Rust's `ring`/`rand` crates on the native side. This is the category with the least friction.

**DOM and text editing** (`editContext.d.ts`, `css.d.ts`, `thenable.d.ts`): `EditContext` is a Chromium-only experimental API that VS Code's editor uses in `nativeEditContext.ts` for high-fidelity IME. Tauri uses OS WebViews (WKWebView on macOS, WebView2 on Windows, WebKitGTK on Linux). WebView2 (Windows) is Chromium-based and supports EditContext; WKWebView (macOS) does not. This creates a platform asymmetry that would require per-platform IME input paths. The CSS module shims are pure build-system artifacts with no runtime implications for Tauri.

In aggregate, the IPC transport and bootstrap injection layers represent the highest-effort remapping work. The Electron-binary dependency in `crossAppIPC` has no direct Tauri equivalent; the multi-process architecture (main process + renderer + utility processes) that these globals serve would need to be re-expressed in terms of Tauri's Rust core and its WebView-facing command/event API.

---

### Out-of-Partition References

The following consumer files (outside `src/typings/`) directly reference symbols declared in the ambient files above:

- `/Users/norinlavaee/vscode-atomic/src/vs/nls.ts:7` — reads `globalThis._VSCODE_NLS_MESSAGES` in `getNLSMessages()`; line 11 reads `globalThis._VSCODE_NLS_LANGUAGE` in `getNLSLanguage()`
- `/Users/norinlavaee/vscode-atomic/src/vs/base/common/network.ts:366-367` — reads `globalThis._VSCODE_FILE_ROOT` to build the file resource root URI
- `/Users/norinlavaee/vscode-atomic/src/vs/amdX.ts:76,88` — reads `globalThis._VSCODE_WEB_PACKAGE_TTP` as the Trusted Types policy for AMD script URL construction
- `/Users/norinlavaee/vscode-atomic/src/vs/base/common/async.ts:1422,1452,1479` — uses `requestIdleCallback`/`cancelIdleCallback` via the `IdleApi` pick type with a fallback to `setTimeout`
- `/Users/norinlavaee/vscode-atomic/src/vs/platform/crossAppIpc/electron-main/crossAppIpcService.ts:73,91,98` — reads `electron.crossAppIPC` (typed as `Electron.CrossAppIPCModule | undefined`) and calls `createCrossAppIPC()` to establish the connection
- `/Users/norinlavaee/vscode-atomic/src/vs/editor/browser/controller/editContext/native/nativeEditContext.ts` — the only site that instantiates the `EditContext` interface declared in `editContext.d.ts`, attaching it to `HTMLElement.editContext` (line 121 in the declaration)
- `/Users/norinlavaee/vscode-atomic/src/vs/platform/product/common/product.ts` — reads `_VSCODE_PRODUCT_JSON` and `_VSCODE_PACKAGE_JSON` as the fallback product data source before `IProductService` is available
- `/Users/norinlavaee/vscode-atomic/src/bootstrap-esm.ts` — populates `_VSCODE_NLS_MESSAGES`, `_VSCODE_NLS_LANGUAGE`, `_VSCODE_FILE_ROOT`, and related globals on `globalThis` before module graph evaluation begins
- `/Users/norinlavaee/vscode-atomic/src/vs/workbench/api/common/extHost.api.impl.ts` — references `Thenable<T>` (27 occurrences) as the return-type contract for extension API functions

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Partition 44: Ambient TypeScript Declarations (`src/typings/`)

## Overview
The `src/typings/` directory contains 9 files (430 LOC) of ambient TypeScript declarations that describe runtime capabilities for Node, Electron, and browser contexts. These declarations establish type contracts for global APIs, namespace extensions, and module type mappings that a Rust/Tauri port must replace or provide equivalent implementations for.

---

## Patterns Found

#### Pattern 1: Global API Declarations
**Where:** `src/typings/base-common.d.ts:8-38`
**What:** Declares global functions and interfaces available across Node.js and browser contexts (idle callbacks, timers, error handling). Uses `declare global` block with interface and function declarations.

```typescript
declare global {
	// --- idle callbacks
	interface IdleDeadline {
		readonly didTimeout: boolean;
		timeRemaining(): number;
	}

	function requestIdleCallback(callback: (args: IdleDeadline) => void, options?: { timeout: number }): number;
	function cancelIdleCallback(handle: number): void;

	// --- timeout / interval (available in all contexts, but different signatures in node.js vs web)
	interface TimeoutHandle { readonly _: never; }
	type Timeout = TimeoutHandle;
	function setTimeout(handler: string | Function, timeout?: number, ...arguments: any[]): Timeout;
	function clearTimeout(timeout: Timeout | undefined): void;

	function setInterval(callback: (...args: unknown[]) => void, delay?: number, ...args: unknown[]): Timeout;
	function clearInterval(timeout: Timeout | undefined): void;

	// --- error
	interface ErrorConstructor {
		captureStackTrace(targetObject: object, constructorOpt?: Function): void;
		stackTraceLimit: number;
	}
}

export { }
```

**Variations:** Similar patterns in `vscode-globals-nls.d.ts:22-40` (declares `_VSCODE_NLS_MESSAGES` and `_VSCODE_NLS_LANGUAGE`), `vscode-globals-product.d.ts:8-47` (declares CSS/product configuration globals), and `vscode-globals-ttp.d.ts:8-11` (declares Trusted Type Policy).

---

#### Pattern 2: Web Crypto API Partial Polyfill
**Where:** `src/typings/crypto.d.ts:10-82`
**What:** Declares Web Crypto API interfaces (SubtleCrypto, Crypto) that are available across browser and Node.js, with most methods commented-out to expose only the used subset (digest, getRandomValues, randomUUID). Includes both interface definitions and global variable declarations.

```typescript
declare global {
	interface SubtleCrypto {
		/** [MDN Reference](https://developer.mozilla.org/docs/Web/API/SubtleCrypto/digest) */
		digest(algorithm: { name: string } | string, data: ArrayBufferView | ArrayBuffer): Promise<ArrayBuffer>;
	}

	interface Crypto {
		readonly subtle: SubtleCrypto;
		getRandomValues<T extends ArrayBufferView | null>(array: T): T;
		randomUUID(): `${string}-${string}-${string}-${string}-${string}`;
	}

	var Crypto: {
		prototype: Crypto;
		new(): Crypto;
	};

	var crypto: Crypto;
}
export { }
```

---

#### Pattern 3: Module Path Wildcards
**Where:** `src/typings/css.d.ts:7-9`
**What:** Declares module types for asset imports using wildcard patterns, allowing CSS to be imported as modules. Uses simple empty module blocks to enable TypeScript to recognize import statements.

```typescript
declare module "vs/css!*" { }
declare module "*.css" { }
```

---

#### Pattern 4: Event Target Interface Extensions
**Where:** `src/typings/editContext.d.ts:8-43`
**What:** Declares browser experimental API (EditContext) with interface extending EventTarget, including update methods, event handlers, and event listener overloads typed by event map. Shows pattern of extending standard DOM interfaces with event handler properties and typed addEventListener overloads.

```typescript
interface EditContext extends EventTarget {
	updateText(rangeStart: number, rangeEnd: number, text: DOMString): void;
	updateSelection(start: number, end: number): void;
	updateControlBounds(controlBounds: DOMRect): void;
	updateSelectionBounds(selectionBounds: DOMRect): void;
	updateCharacterBounds(rangeStart: number, characterBounds: DOMRect[]): void;

	attachedElements(): HTMLElement[];

	get text(): DOMString;
	get selectionStart(): number;
	get selectionEnd(): number;
	get characterBoundsRangeStart(): number;
	characterBounds(): DOMRect[];

	get ontextupdate(): EventHandler<TextUpdateEvent> | null;
	set ontextupdate(value: EventHandler | null);
	// ... more event handler properties

	addEventListener<K extends keyof EditContextEventHandlersEventMap>(
		type: K, 
		listener: (this: GlobalEventHandlers, ev: EditContextEventHandlersEventMap[K]) => any, 
		options?: boolean | AddEventListenerOptions
	): void;
	addEventListener(type: string, listener: EventListenerOrEventListenerObject, options?: boolean | AddEventListenerOptions): void;
}
```

---

#### Pattern 5: Custom Event Class Declarations
**Where:** `src/typings/editContext.d.ts:61-79`
**What:** Declares custom event classes (TextUpdateEvent) extending standard Event with readonly properties initialized via constructor options. Pairs with EventInit interface patterns.

```typescript
declare class TextUpdateEvent extends Event {
	constructor(type: DOMString, options?: TextUpdateEventInit);

	readonly updateRangeStart: number;
	readonly updateRangeEnd: number;
	readonly text: DOMString;
	readonly selectionStart: number;
	readonly selectionEnd: number;
}

interface TextUpdateEventInit extends EventInit {
	updateRangeStart: number;
	updateRangeEnd: number;
	text: DOMString;
	selectionStart: number;
	selectionEnd: number;
	compositionStart: number;
	compositionEnd: number;
}
```

---

#### Pattern 6: Electron Namespace Extensions
**Where:** `src/typings/electron-cross-app-ipc.d.ts:15-61`
**What:** Extends Electron namespace with custom module (CrossAppIPC) featuring event emitter pattern with typed event listeners (on/once/removeListener overloads), state properties, and method declarations. Shows nested namespace structure (Electron.Main, Electron.CrossProcessExports) for optional module availability.

```typescript
declare namespace Electron {
	interface CrossAppIPCMessageEvent {
		data: any;
		ports: Electron.MessagePortMain[];
	}

	type CrossAppIPCDisconnectReason =
		| 'peer-disconnected'
		| 'handshake-failed'
		| 'connection-failed'
		| 'connection-timeout';

	interface CrossAppIPC extends NodeJS.EventEmitter {
		on(event: 'connected', listener: () => void): this;
		once(event: 'connected', listener: () => void): this;
		removeListener(event: 'connected', listener: () => void): this;

		on(event: 'message', listener: (messageEvent: CrossAppIPCMessageEvent) => void): this;
		once(event: 'message', listener: (messageEvent: CrossAppIPCMessageEvent) => void): this;
		removeListener(event: 'message', listener: (messageEvent: CrossAppIPCMessageEvent) => void): this;

		on(event: 'disconnected', listener: (reason: CrossAppIPCDisconnectReason) => void): this;
		once(event: 'disconnected', listener: (reason: CrossAppIPCDisconnectReason) => void): this;
		removeListener(event: 'disconnected', listener: (reason: CrossAppIPCDisconnectReason) => void): this;

		connect(): void;
		close(): void;
		postMessage(message: any, transferables?: Electron.MessagePortMain[]): void;
		readonly connected: boolean;
		readonly isServer: boolean;
	}

	namespace Main {
		const crossAppIPC: CrossAppIPCModule | undefined;
	}

	namespace CrossProcessExports {
		const crossAppIPC: CrossAppIPCModule | undefined;
	}
}
```

---

#### Pattern 7: Promise-Like Type Aliases
**Where:** `src/typings/thenable.d.ts:12`
**What:** Declares minimal Thenable type as alias extending PromiseLike, providing cross-library promise compatibility without implementation. Represents lowest-common-denominator promise interface.

```typescript
interface Thenable<T> extends PromiseLike<T> { }
```

---

## Summary

The ambient declarations in `src/typings/` establish type contracts across seven distinct categories: **global runtime APIs** (timers, idle callbacks, error handling), **crypto primitives** (digest, random generation), **asset module resolution** (CSS imports), **DOM experimental APIs** (EditContext for text input), **custom event hierarchies** (TextUpdateEvent, format/bounds updates), **Electron native extensions** (cross-app IPC with event emitter patterns), and **promise abstractions** (Thenable). 

A Rust/Tauri port must replace these declarations with native FFI bindings or provide JavaScript shims that expose equivalent functionality. Key migration targets include: substituting Electron's CrossAppIPC with Tauri's ipc module, replacing Web Crypto with Rust crypto libraries (ring, rustls), mapping timer APIs to platform equivalents, and implementing event handling through Tauri's event system. The EditContext and custom event patterns require careful transformation since they depend on DOM and W3C standard event propagation, which may need synthetic implementation in Tauri's web view or custom Rust-side event marshaling.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
