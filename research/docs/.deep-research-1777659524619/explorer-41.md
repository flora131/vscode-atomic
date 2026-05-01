# Partition 41 of 79 — Findings

## Scope
`src/typings/` (10 files, 583 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
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

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
### Files Analysed

1. `src/typings/electron-cross-app-ipc.d.ts` (62 lines)
2. `src/typings/editContext.d.ts` (124 lines)
3. `src/typings/base-common.d.ts` (41 lines)
4. `src/typings/vscode-globals-nls.d.ts` (40 lines)
5. `src/typings/vscode-globals-product.d.ts` (47 lines)
6. `src/typings/vscode-globals-ttp.d.ts` (15 lines)
7. `src/typings/thenable.d.ts` (12 lines)
8. `src/typings/css.d.ts` (9 lines)
9. `src/typings/crypto.d.ts` (83 lines)
10. `src/typings/copilot-api.d.ts` (154 lines)

---

### Per-File Notes

#### 1. `src/typings/electron-cross-app-ipc.d.ts`

**Role:** Ambient type declarations for a custom Electron IPC module that facilitates secure message passing between a host Electron app and an embedded "MiniApp" sub-process running in a nested bundle.

**Key Symbols:**

- `Electron.CrossAppIPCMessageEvent` (line 17-22): Event payload carrying `data: any` and `ports: Electron.MessagePortMain[]`. The use of `MessagePortMain` shows this is a main-process construct.
- `Electron.CrossAppIPCDisconnectReason` (line 24-29): Union string literal type enumerating four failure modes — `'peer-disconnected'`, `'handshake-failed'`, `'connection-failed'`, `'connection-timeout'` — that represent the full lifecycle of an IPC connection failure.
- `Electron.CrossAppIPC` (line 30-48): The core interface, extending `NodeJS.EventEmitter`. Exposes:
  - `connect(): void` (line 43) — initiates the connection
  - `close(): void` (line 44) — tears it down
  - `postMessage(message: any, transferables?: Electron.MessagePortMain[]): void` (line 45) — sends a message, optionally transferring `MessagePort` objects
  - `connected: boolean` (line 46) — readable connection state
  - `isServer: boolean` (line 47) — indicates which side of the pipe this instance represents
- `Electron.CrossAppIPCModule` (line 50-52): Factory interface with a single method `createCrossAppIPC(): CrossAppIPC`.
- `Electron.Main.crossAppIPC` (line 55) and `Electron.CrossProcessExports.crossAppIPC` (line 59): Both typed as `CrossAppIPCModule | undefined`, indicating the module is optionally present and must be checked before use.

**Architecture Notes:** The comment block (lines 7-13) states that authentication is done via OS-level mechanisms: Mach ports on macOS, named pipes on Windows. This is deeply tied to Electron's main-process model and `MessagePortMain` types that have no equivalent in Tauri's webview-based IPC.

---

#### 2. `src/typings/editContext.d.ts`

**Role:** Ambient declarations for the W3C EditContext API, a browser standard for advanced text input handling. This API allows the editor to intercept IME (Input Method Editor) composition events at a lower level than standard `<input>` or `contenteditable` elements.

**Key Symbols:**

- `EditContext` interface (line 8-43): The central interface extending `EventTarget`. Key mutating methods:
  - `updateText(rangeStart, rangeEnd, text)` (line 10): Replaces a character range in the EditContext's virtual text buffer.
  - `updateSelection(start, end)` (line 11): Notifies the OS/IME of the current selection.
  - `updateControlBounds(controlBounds: DOMRect)` (line 12): Tells the IME the pixel coordinates of the editing control.
  - `updateSelectionBounds(selectionBounds: DOMRect)` (line 13): Reports selection geometry for IME popup positioning.
  - `updateCharacterBounds(rangeStart, characterBounds: DOMRect[])` (line 14): Provides per-character bounding boxes for fine-grained IME cursor placement.
  - `attachedElements(): HTMLElement[]` (line 16): Returns which DOM elements are currently linked to this context.
- `EditContextEventHandlersEventMap` (line 51-57): Maps event names to typed event classes:
  - `textupdate` → `TextUpdateEvent`
  - `textformatupdate` → `TextFormatUpdateEvent`
  - `characterboundsupdate` → `CharacterBoundsUpdateEvent`
  - `compositionstart` / `compositionend` → `Event`
- `TextUpdateEvent` (line 61-69): Fired when the OS/IME wants to change text. Carries `updateRangeStart`, `updateRangeEnd`, `text`, `selectionStart`, `selectionEnd`.
- `TextFormatUpdateEvent` (line 100-103): Carries an array of `TextFormat` objects (line 82-88), each describing underline decorations (`underlineStyle`, `underlineThickness`) used for IME composition suggestions.
- `CharacterBoundsUpdateEvent` (line 109-114): Fired when the IME needs to know the pixel bounds of a specific character range.
- `HTMLElement.editContext?: EditContext` (line 121-123): Augments the global `HTMLElement` to accept an `EditContext` attachment.

**Architecture Notes:** This API bridges the browser's text input model and the editor's internal document model. In a Tauri port, if a WebView (Chromium/WebKit) is retained for the editor surface, this API would be preserved — however its availability depends on the specific WebView version bundled.

---

#### 3. `src/typings/base-common.d.ts`

**Role:** Global ambient declarations for APIs that exist across all VS Code execution contexts (browser, Node.js/Electron main, Utility Process, Web Worker) but whose TypeScript signatures differ between environments. This file resolves those signature conflicts.

**Key Symbols:**

- `IdleDeadline` / `requestIdleCallback` / `cancelIdleCallback` (lines 12-18): Declares the idle callback API. The `IdleDeadline.timeRemaining()` method returns how many milliseconds remain in the current idle period. Used for deferred, low-priority work scheduling.
- `TimeoutHandle` interface (line 23): A branded opaque type (`readonly _: never`) used instead of raw `number` or Node.js's `NodeJS.Timeout` to prevent cross-context assignment errors. `Timeout` is a type alias for it (line 24).
- `setTimeout` (line 25): Redeclared globally with the `Timeout` return type, overriding both browser (`number`) and Node.js (`NodeJS.Timeout`) signatures.
- `clearTimeout(timeout: Timeout | undefined)` (line 26): Accepts `undefined` to avoid null-check noise at call sites.
- `setInterval` / `clearInterval` (lines 28-29): Same `Timeout` type treatment as setTimeout/clearTimeout.
- `ErrorConstructor` augmentation (lines 32-37): Adds `captureStackTrace(targetObject, constructorOpt?)` (a V8-specific extension) and `stackTraceLimit: number` to the standard `ErrorConstructor`, making them typeable in non-Node contexts.

**Architecture Notes:** The `TimeoutHandle` branded-type trick (line 23) is critical — it ensures that timeout handles returned in Node.js contexts (which are objects, not numbers) are not accidentally used as numbers in browser contexts, and vice versa. All VS Code async scheduling that flows through setTimeout/setInterval uses this type boundary. In a Tauri port, these would map to either browser WebView timers (if the UI layer is retained) or Rust `tokio::time` constructs on the backend.

---

#### 4. `src/typings/vscode-globals-nls.d.ts`

**Role:** Declares two global variables that form the runtime contract for VS Code's build-time internationalization system. Annotated `// AMD2ESM migration relevant` (line 6), indicating it is being updated as VS Code moves from AMD module loading to ES modules.

**Key Symbols:**

- `_VSCODE_NLS_MESSAGES: string[]` (line 30): A global array of all translated strings for the current language. At build time, `nls.localize()` and `nls.localize2()` calls have their string arguments stripped and replaced with numeric indices into this array. At runtime, the index is looked up here.
- `_VSCODE_NLS_LANGUAGE: string | undefined` (line 36): A global string holding the active language code (e.g., `'en'`, `'de'`, `'pt-br'`). `undefined` indicates the default English fallback.

**Architecture Notes:** The comment block (lines 17-21) explicitly lists all contexts where these globals must be defined: Electron main process, Electron renderer (window), Utility Process, Node.js, Browser, and Web Worker. This means the NLS bootstrap code must run in every execution context before any `localize()` call. In a Tauri port, this two-global bootstrap contract would need to be re-established — potentially via Tauri's `invoke` system to deliver the message array from Rust to the WebView at startup.

---

#### 5. `src/typings/vscode-globals-product.d.ts`

**Role:** Declares global variables for build-time asset loading and product/package metadata. Also marked `// AMD2ESM migration relevant` (line 6).

**Key Symbols:**

- `_VSCODE_FILE_ROOT: string` (line 13): The root URL/path for static resource resolution at runtime.
- `_VSCODE_CSS_LOAD: (module: string) => void` (line 18-19): A development-time CSS loader function. The comment instructs that code should `import 'some.css'` rather than calling this directly; it is a low-level escape hatch.
- `_VSCODE_PRODUCT_JSON: Record<string, any>` (line 23-24): Holds parsed `product.json` contents. Marked `@deprecated` — callers should use `IProductService` instead.
- `_VSCODE_PACKAGE_JSON: Record<string, any>` (line 27-28): Holds parsed `package.json` contents. Also `@deprecated` in favor of `IProductService`.
- `_VSCODE_DISABLE_CSS_IMPORT_MAP: boolean | undefined` (line 33-35): Disables CSS import-map loading when a bundler handles CSS directly.
- `_VSCODE_USE_RELATIVE_IMPORTS: boolean | undefined` (line 40-43): When set, directs the module resolver to use relative source paths instead of compiled output paths.

**Architecture Notes:** These globals are injected at application bootstrap, likely through `src/vs/workbench/browser/client.ts` or the Electron `preload.js`. The `_VSCODE_FILE_ROOT` variable is essential for loading extension icons, themes, and other static assets — in Tauri, this would need to be served via Tauri's asset protocol (`tauri://localhost/`).

---

#### 6. `src/typings/vscode-globals-ttp.d.ts`

**Role:** Declares the global for the web package's Trusted Type Policy, a browser security feature that restricts how scripts and URLs can be created.

**Key Symbols:**

- `_VSCODE_WEB_PACKAGE_TTP: Pick<TrustedTypePolicy, 'name' | 'createScriptURL'> | undefined` (line 10): A partial `TrustedTypePolicy` exposing only `name` and `createScriptURL`. `undefined` when the browser does not support Trusted Types or the policy is not initialized.

**Architecture Notes:** Trusted Types is a browser Content Security Policy mechanism. Its presence signals that VS Code's web rendering layer enforces `createScriptURL` validation to prevent DOM-based XSS. In a Tauri port with a retained WebView, this policy would be relevant for any dynamic script URL construction in the renderer. The `createScriptURL` function would need to be callable in the WebView's JavaScript context.

---

#### 7. `src/typings/thenable.d.ts`

**Role:** Provides a minimal ambient interface definition for the `Thenable<T>` type used throughout VS Code's public and internal APIs.

**Key Symbols:**

- `Thenable<T>` (line 12): Declared as `interface Thenable<T> extends PromiseLike<T> { }`. It is an empty extension of the built-in `PromiseLike<T>`, meaning anything with a `.then()` method satisfies this contract.

**Architecture Notes:** The comment (lines 7-11) explicitly documents the design intent: `Thenable` is a lowest-common-denominator async primitive that abstracts over ES6 Promises, Q, jQuery Deferred, WinJS.Promise, etc. Because VS Code's extension API uses `Thenable` rather than `Promise`, extension code is not locked into a specific promise implementation. In a Tauri port, the TypeScript/JavaScript async layer would remain as-is in the WebView; Rust's `async/await` with `tokio` futures would only appear in the Rust backend, not in this abstraction layer.

---

#### 8. `src/typings/css.d.ts`

**Role:** Module declaration stubs that tell TypeScript's module resolver to treat CSS files as valid importable modules.

**Key Symbols:**

- `declare module "vs/css!*"` (line 8): Wildcards any module path prefixed with the AMD-era `vs/css!` loader plugin syntax.
- `declare module "*.css"` (line 9): Wildcards any `.css` file import in ESM style.

**Architecture Notes:** These declarations enable `import 'path/to/styles.css'` throughout the VS Code TypeScript source without type errors. At build time, these imports are handled by a bundler (esbuild or webpack) that injects the styles into the document. In a Tauri port retaining the WebView UI, CSS import behavior would remain identical; the bundler would still handle style injection into the WebView's DOM.

---

#### 9. `src/typings/crypto.d.ts`

**Role:** A partial copy of `lib.dom.d.ts` for the `Crypto` and `SubtleCrypto` interfaces, made available to code in the `/common/` layer that has no dependency on the DOM.

**Key Symbols:**

- `SubtleCrypto.digest(algorithm, data): Promise<ArrayBuffer>` (line 26): The only non-commented method in the interface. All other `SubtleCrypto` methods (decrypt, deriveBits, encrypt, exportKey, generateKey, importKey, sign, verify, wrapKey) are commented out (lines 19-48), meaning VS Code's common layer only actively uses `digest`.
- `Crypto.subtle: SubtleCrypto` (line 62-64): Readonly access to the subtle interface.
- `Crypto.getRandomValues<T extends ArrayBufferView | null>(array: T): T` (line 67): Fills a typed array with cryptographically random bytes.
- `Crypto.randomUUID(): \`${string}-${string}-${string}-${string}-${string}\`` (line 72): Returns a UUID v4 string using a template literal type for structural validation.
- `var crypto: Crypto` (line 80): The global `crypto` object, typed to the declared `Crypto` interface.

**Architecture Notes:** The comment (lines 7-9) explains that these types are replicated here because `common/` layer code cannot import DOM types, yet `crypto` is available as a global in both browsers and Node.js. Only `digest` (hashing), `getRandomValues`, and `randomUUID` are actively typed for use. In a Tauri port, the WebView's JS runtime provides all three natively; the Rust backend would use `ring` or `sha2` crates for equivalent hashing operations.

---

#### 10. `src/typings/copilot-api.d.ts`

**Role:** Ambient module declaration for `@vscode/copilot-api`, a package whose own `.d.ts` files use extensionless relative imports incompatible with `moduleResolution: "nodenext"`. The comment (lines 9-12) states this is a partial workaround until the upstream package is fixed.

**Key Symbols:**

- `IAbortSignal` (line 15-19): A minimal abort signal interface with `aborted: boolean` and `addEventListener`/`removeEventListener` for `'abort'` events.
- `FetchOptions` (line 21-31): HTTP request configuration including `callSite` (required string for telemetry attribution), optional `headers`, `body`, `timeout`, `json`, `method` (`'GET' | 'POST' | 'PUT'`), `signal`, and `suppressIntegrationId`.
- `MakeRequestOptions` (line 33-35): Derived from `FetchOptions` by omitting `callSite` then making it optional — used for the public API surface of `CAPIClient`.
- `IFetcherService` (line 37-39): Single-method interface `fetch(url, options): Promise<unknown>`. The abstraction point for all HTTP calls, injectable for testing.
- `IExtensionInformation` (line 41-48): Identity information sent with every Copilot request: `name`, `sessionId`, `machineId`, `deviceId`, `vscodeVersion`, `version`, `buildType`.
- `CopilotToken` (line 50-58): Token payload with an `endpoints` record (optional keys: `api`, `telemetry`, `proxy`, `origin-tracker`) and a `sku` string.
- `RequestType` enum (line 60-66): Discriminant for request kinds — `CopilotToken`, `ChatCompletions`, `ChatResponses`, `ChatMessages`, `Models`.
- `CAPIClient` class (line 79-89):
  - Constructor takes `extensionInfo`, `license`, optional `fetcherService`, optional `hmacSecret`, optional `integrationId`.
  - `updateDomains(copilotToken, enterpriseUrlConfig): IDomainChangeResponse` (line 87): Updates the set of API endpoints based on the current token; returns a change-flag record.
  - `makeRequest<T>(requestOptions, requestMetadata): Promise<T>` (line 88): The central HTTP dispatch method, generic over response type.
- `CCAModel` (line 133-152): Full model descriptor with `billing` (premium status, multiplier, restrictions), `capabilities` (context window, output limits, vision limits, streaming/tool-call support), `policy`, `supported_endpoints` (e.g., `'/v1/messages'` for Anthropic-format models, `'/chat/completions'` for OpenAI-format), `vendor`, and `version`.

**Architecture Notes:** The `IFetcherService` injection (line 37-39) is the key seam for a Tauri port — all Copilot HTTP traffic flows through a single injectable `fetch` call, meaning a Tauri port could replace `IFetcherService` with a Rust-backed implementation that routes requests through `tauri::http` or `reqwest`. The `hmacSecret` parameter in `CAPIClient`'s constructor (line 86) indicates request signing is used; this would map to Rust's `ring` HMAC implementation.

---

### Cross-Cutting Synthesis

The `src/typings/` directory serves as a type contract layer documenting exactly where VS Code's TypeScript source touches platform-specific or browser-specific capabilities. Five distinct dependency clusters emerge from these files:

1. **IPC / Process Communication** (`electron-cross-app-ipc.d.ts`): The `CrossAppIPC` interface encodes Electron's multi-process model including OS-authenticated channels (Mach ports / named pipes). This entire module has no equivalent in Tauri; Tauri's `invoke`/`emit` command system would replace it, but the authenticated multi-app embedding pattern would require a full redesign.

2. **Text Input / IME** (`editContext.d.ts`): The EditContext API is a WebView-level API. In Tauri, this is available if the bundled Chromium/WebKit WebView supports it. The `updateCharacterBounds` method pattern specifically ties editor-internal character metrics to OS IME popup positioning — a coordination that in Tauri would still pass through the WebView's JavaScript runtime but depends on WebView version support.

3. **Runtime Environment Globals** (`base-common.d.ts`, `vscode-globals-nls.d.ts`, `vscode-globals-product.d.ts`, `vscode-globals-ttp.d.ts`): All four files declare globals that must be injected at bootstrap time into every execution context. In Tauri, bootstrapping these globals would be the responsibility of a Rust-side initialization sequence that serializes the NLS messages array and product metadata and passes them to the WebView via `tauri::webview::Window::eval` or initialization scripts.

4. **Async Patterns** (`thenable.d.ts`, `base-common.d.ts`): The `Thenable<T>` abstraction and the `Timeout` branded type represent the TypeScript async boundary. These remain entirely in the WebView/JavaScript layer and do not map directly to Rust futures; the Rust backend would use its own async runtime independently.

5. **External Service Integration** (`copilot-api.d.ts`, `crypto.d.ts`): The `CAPIClient` with its injectable `IFetcherService` represents all Copilot HTTP traffic. The `crypto` types show that only `digest`, `getRandomValues`, and `randomUUID` are used from the common layer. Both have clean abstraction seams that would allow routing through Tauri's Rust HTTP client (`reqwest`) without changes to the TypeScript call sites.

The CSS module declarations (`css.d.ts`) represent the only entirely portable construct — bundler behavior for CSS imports is unaffected by the Electron-to-Tauri transition.

---

### Out-of-Partition References

- `Electron.MessagePortMain` — Electron main-process type referenced at `electron-cross-app-ipc.d.ts:21,45`. Defined in Electron's own type packages, not in `src/typings/`.
- `NodeJS.EventEmitter` — referenced at `electron-cross-app-ipc.d.ts:30`. Defined in `@types/node`.
- `TrustedTypePolicy` — referenced at `vscode-globals-ttp.d.ts:10`. Defined in `lib.dom.d.ts` from TypeScript's standard library.
- `PromiseLike<T>` — referenced at `thenable.d.ts:12`. TypeScript built-in type from `lib.es2015.promise.d.ts`.
- `nls.localize` / `nls.localize2` — referenced by description in `vscode-globals-nls.d.ts:23`. Implemented at `src/vs/nls.ts` (outside this partition).
- `IProductService` — referenced by `@deprecated` JSDoc comments at `vscode-globals-product.d.ts:23,27`. Implemented in `src/vs/platform/product/common/productService.ts` (outside this partition).
- `@vscode/copilot-api` npm package — referenced at `copilot-api.d.ts:13`. The actual package is external; the file provides a local override of its type declarations due to `moduleResolution: "nodenext"` incompatibility.
- `IFetcherService` implementations — `copilot-api.d.ts:37-39` defines the interface; concrete implementations reside in extension host or workbench service code outside this partition.

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# VS Code Ambient Type Declarations & Host Injection Patterns

## Research Question
What runtime contracts (via TypeScript ambient declarations) must a Rust/Tauri port of VS Code's core IDE functionality replicate from the Electron/Node host?

## Summary
VS Code's codebase declares 10 ambient `.d.ts` files in `src/typings/` that define critical host-injected globals and browser APIs used throughout the IDE. These declarations fall into 5 categories: (1) async scheduling APIs, (2) NLS/localization globals, (3) product/build metadata, (4) Electron IPC modules, and (5) DOM/crypto standards. A Tauri port must provide equivalent implementations of these contracts.

---

## Patterns Found

#### Pattern 1: Async Scheduling Fallbacks
**Where:** `src/typings/base-common.d.ts:10-29`
**What:** Polyfillable async APIs that the Electron host must provide for cross-context execution.
```typescript
declare global {
	interface IdleDeadline {
		readonly didTimeout: boolean;
		timeRemaining(): number;
	}

	function requestIdleCallback(callback: (args: IdleDeadline) => void, options?: { timeout: number }): number;
	function cancelIdleCallback(handle: number): void;

	interface TimeoutHandle { readonly _: never; }
	type Timeout = TimeoutHandle;
	function setTimeout(handler: string | Function, timeout?: number, ...arguments: any[]): Timeout;
	function clearTimeout(timeout: Timeout | undefined): void;

	function setInterval(callback: (...args: unknown[]) => void, delay?: number, ...args: unknown[]): Timeout;
	function clearInterval(timeout: Timeout | undefined): void;
}
```

**Variations / call-sites:**
- `test/unit/electron/renderer.js:408-444` implements `setTimeout0` optimization to bypass 4ms throttling in nested message event handlers
- `test/unit/browser/renderer.html:46-73` mirrors the same pattern for browser-based tests
- These are used in Mocha test harnesses to schedule micro-tasks without browser delays

---

#### Pattern 2: NLS Message Injection for Multi-Context Localization
**Where:** `src/typings/vscode-globals-nls.d.ts:22-36`
**What:** Build-time string stripping: English strings are removed during build and replaced with numeric indices, then looked up at runtime from a host-injected message array.
```typescript
declare global {
	/**
	 * All NLS messages produced by `localize` and `localize2` calls
	 * under `src/vs` translated to the language as indicated by
	 * `_VSCODE_NLS_LANGUAGE`.
	 *
	 * Instead of accessing this global variable directly, use function getNLSMessages.
	 */
	var _VSCODE_NLS_MESSAGES: string[];
	/**
	 * The actual language of the NLS messages (e.g. 'en', de' or 'pt-br').
	 *
	 * Instead of accessing this global variable directly, use function getNLSLanguage.
	 */
	var _VSCODE_NLS_LANGUAGE: string | undefined;
}
```

**Variations / call-sites:**
- `test/unit/electron/renderer.js:106-113` initializes from `nls.messages.json` build artifact when opts.build=true
- Must be available in: Electron main, renderer, utility process, Node.js, browser, and web worker contexts
- Localization framework depends on this being set before any module imports

---

#### Pattern 3: Product & Build Metadata Globals
**Where:** `src/typings/vscode-globals-product.d.ts:8-43`
**What:** Host-injected build configuration including file root, CSS loader, and deprecated product.json/package.json references.
```typescript
declare global {
	/**
	 * Holds the file root for resources.
	 */
	var _VSCODE_FILE_ROOT: string;

	/**
	 * CSS loader that's available during development time.
	 * DO NOT call directly, instead just import css modules, like `import 'some.css'`
	 */
	var _VSCODE_CSS_LOAD: (module: string) => void;

	/**
	 * @deprecated You MUST use `IProductService` whenever possible.
	 */
	var _VSCODE_PRODUCT_JSON: Record<string, any>;
	/**
	 * @deprecated You MUST use `IProductService` whenever possible.
	 */
	var _VSCODE_PACKAGE_JSON: Record<string, any>;
}
```

**Variations / call-sites:**
- `test/unit/electron/renderer.js:79-80` initializes `_VSCODE_PRODUCT_JSON` and `_VSCODE_PACKAGE_JSON` from require() calls
- `test/unit/electron/renderer.js:120` sets `_VSCODE_FILE_ROOT` to file:// URL for ES module resolution
- `test/unit/browser/renderer.html:114-126` injects `_VSCODE_CSS_LOAD` as a function that inserts @import rules
- `test/unit/browser/renderer.html:151` sets `_VSCODE_FILE_ROOT` to source directory URL

---

#### Pattern 4: Electron Cross-App IPC for Nested Bundles
**Where:** `src/typings/electron-cross-app-ipc.d.ts:15-60`
**What:** Custom Electron module for secure IPC between host app and embedded MiniApp with code-signature authentication.
```typescript
declare namespace Electron {
	interface CrossAppIPCMessageEvent {
		/** The deserialized message data sent by the peer app. */
		data: any;
		/** Array of transferred MessagePortMain objects (if any). */
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

	interface CrossAppIPCModule {
		createCrossAppIPC(): CrossAppIPC;
	}

	namespace Main {
		const crossAppIPC: CrossAppIPCModule | undefined;
	}

	namespace CrossProcessExports {
		const crossAppIPC: CrossAppIPCModule | undefined;
	}
}
```

**Variations / call-sites:**
- Platform-specific: authentication via Mach ports (macOS) or named pipes (Windows)
- Used in `src/vs/platform/update/electron-main/crossAppUpdateIpc.ts` for inter-process update coordination
- Marked optional (`undefined`) because it's only available in custom Electron builds with MiniApp support

---

#### Pattern 5: EditContext API for Advanced Text Input
**Where:** `src/typings/editContext.d.ts:8-123`
**What:** Proposed DOM standard for IME, spell-check, and advanced text editing that must be polyfilled or natively provided.
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

	get ontextformatupdate(): EventHandler | null;
	set ontextformatupdate(value: EventHandler | null);

	get oncharacterboundsupdate(): EventHandler | null;
	set oncharacterboundsupdate(value: EventHandler | null);

	get oncompositionstart(): EventHandler | null;
	set oncompositionstart(value: EventHandler | null);

	get oncompositionend(): EventHandler | null;
	set oncompositionend(value: EventHandler | null);

	addEventListener<K extends keyof EditContextEventHandlersEventMap>(type: K, listener: (this: GlobalEventHandlers, ev: EditContextEventHandlersEventMap[K]) => any, options?: boolean | AddEventListenerOptions): void;
	addEventListener(type: string, listener: EventListenerOrEventListenerObject, options?: boolean | AddEventListenerOptions): void;
}

interface HTMLElement {
	editContext?: EditContext;
}
```

**Variations / call-sites:**
- Still experimental (not universally available)
- Provides rich text editing with bounds tracking for IME overlay positioning
- Alternative paths exist in the codebase using contenteditable and composition events

---

#### Pattern 6: Crypto Standard API
**Where:** `src/typings/crypto.d.ts:10-82`
**What:** Web Crypto API with SubtleCrypto digest support for cross-context hashing (Node.js + browser).
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

	var crypto: Crypto;
}
```

**Variations / call-sites:**
- Most methods are commented out (unused) — only digest, getRandomValues, randomUUID are active
- Available in both browser and Node.js contexts
- Shared between `src/vs/common/` layer (no DOM dependency) across all execution contexts

---

#### Pattern 7: Thenable Abstraction (Promise Portability)
**Where:** `src/typings/thenable.d.ts:1-12`
**What:** Cross-promise-library compatibility layer for code that predates ES6 Promise standardization.
```typescript
/**
 * Thenable is a common denominator between ES6 promises, Q, jquery.Deferred, WinJS.Promise,
 * and others. This API makes no assumption about what promise library is being used which
 * enables reusing existing code without migrating to a specific promise implementation. Still,
 * we recommend the use of native promises which are available in VS Code.
 */
interface Thenable<T> extends PromiseLike<T> { }
```

**Variations / call-sites:**
- Minimal modern usage (mostly legacy compatibility)
- All new code should use native Promise

---

#### Pattern 8: TrustedTypes Policy for Web Security
**Where:** `src/typings/vscode-globals-ttp.d.ts:8-14`
**What:** Host-injected TrustedType policy for safe dynamic script/HTML manipulation during development.
```typescript
declare global {
	var _VSCODE_WEB_PACKAGE_TTP: Pick<TrustedTypePolicy, 'name' | 'createScriptURL'> | undefined;
}

// fake export to make global work
export { };
```

**Variations / call-sites:**
- Only for web distribution (Trusted Types standard)
- Optional (`undefined`) when not running in browsers that support TrustedTypes
- Used by AMD-to-ESM migration tooling

---

#### Pattern 9: CSS Module Declarations
**Where:** `src/typings/css.d.ts:1-9`
**What:** Module resolution for CSS imports via bundler or runtime loader.
```typescript
// Recognize all CSS files as valid module imports
declare module "vs/css!*" { }
declare module "*.css" { }
```

**Variations / call-sites:**
- First pattern handles legacy CSS loader syntax (AMD-era)
- Second pattern handles native CSS module imports (ESM)
- Either bundler or runtime CSS loader must be present

---

#### Pattern 10: Copilot API Type Stubs
**Where:** `src/typings/copilot-api.d.ts:13-153`
**What:** Ambient module re-export for @vscode/copilot-api package (workaround for incompatible extensionless imports).
```typescript
declare module '@vscode/copilot-api' {
	export interface IAbortSignal {
		readonly aborted: boolean;
		addEventListener(type: 'abort', listener: (this: AbortSignal) => void): void;
		removeEventListener(type: 'abort', listener: (this: AbortSignal) => void): void;
	}

	export interface FetchOptions {
		callSite: string;
		headers?: { [name: string]: string };
		body?: BodyInit;
		timeout?: number;
		json?: unknown;
		method?: 'GET' | 'POST' | 'PUT';
		signal?: IAbortSignal;
		suppressIntegrationId?: boolean;
	}

	export interface IFetcherService {
		fetch(url: string, options: FetchOptions): Promise<unknown>;
	}

	export class CAPIClient {
		constructor(
			extensionInfo: IExtensionInformation,
			license: string | undefined,
			fetcherService?: IFetcherService,
			hmacSecret?: string,
			integrationId?: string,
		);
		updateDomains(copilotToken: CopilotToken | undefined, enterpriseUrlConfig: string | undefined): IDomainChangeResponse;
		makeRequest<T>(requestOptions: MakeRequestOptions, requestMetadata: RequestMetadata): Promise<T>;
	}
}
```

**Variations / call-sites:**
- Includes model capability descriptors (CCAModel) with token limits and supported API endpoints
- Temporary workaround until @vscode/copilot-api fixes module resolution

---

## Porting Implications for Tauri/Rust

### Global Injection Points
A Tauri port must inject into the JavaScript/WebView context:
1. **NLS globals** before module initialization
2. **File root URL** for ES module resolution
3. **CSS loader function** for stylesheet management
4. **Async scheduling** utilities (mostly std, but optimize requestIdleCallback)
5. **Crypto API** (Node.js-like) as-is for hashing
6. **EditContext polyfill** if targeting older browsers

### IPC Translation
- **Electron.ipcRenderer** → Tauri's `invoke()` command system
- **CrossAppIPC** → Tauri plugin or dropped if not needed
- Event-based messaging → Tauri's event system

### Missing Host Contracts
- TrustedTypes (web-only) — can be stub
- Thenable (legacy) — use native Promise
- Copilot API — may be feature-flagged or require Copilot plugin

### Initialization Order
Critical: NLS and product metadata must be injected before the first module import to ensure localization works across all contexts (main, renderer, worker).

---

## Files Referenced
- `src/typings/base-common.d.ts` — Async scheduling
- `src/typings/vscode-globals-nls.d.ts` — NLS message injection
- `src/typings/vscode-globals-product.d.ts` — Build metadata
- `src/typings/electron-cross-app-ipc.d.ts` — Inter-process communication
- `src/typings/editContext.d.ts` — Advanced text input API
- `src/typings/crypto.d.ts` — Web Crypto standard
- `src/typings/thenable.d.ts` — Promise compatibility
- `src/typings/vscode-globals-ttp.d.ts` — TrustedTypes
- `src/typings/css.d.ts` — CSS module imports
- `src/typings/copilot-api.d.ts` — Copilot API stubs
- `test/unit/electron/renderer.js` — Electron initialization pattern (lines 79–120)
- `test/unit/browser/renderer.html` — Browser initialization pattern (lines 114–151)

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
