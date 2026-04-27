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
