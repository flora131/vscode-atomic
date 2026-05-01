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
