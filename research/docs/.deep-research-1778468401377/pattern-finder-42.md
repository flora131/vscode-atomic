# Type Definition Patterns in VS Code Atomic Port

## Overview
The `src/typings/` directory (9 files, 522 LOC) contains ambient type declarations that bridge VS Code's TypeScript codebase with browser APIs, Node.js globals, and external packages. These patterns reveal integration points critical for porting VS Code functionality to Tauri/Rust.

---

## Patterns Found

#### Pattern: Global Runtime Environment Declarations
**Where:** `src/typings/base-common.d.ts:8-38`
**What:** Polyfill declarations for cross-runtime utilities (timers, idle callbacks, error stack traces) available in all JS contexts.

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

	interface ErrorConstructor {
		captureStackTrace(targetObject: object, constructorOpt?: Function): void;
		stackTraceLimit: number;
	}
}
```

**Variations / call-sites:** Probes for utilities used in the `/common/` layer shared across Electron main process, Electron renderer, and Node.js contexts.

---

#### Pattern: Web Crypto API Declarations
**Where:** `src/typings/crypto.d.ts:10-82`
**What:** Partial Web Crypto API interfaces for `SubtleCrypto` and `Crypto` available in both browsers and Node.js, with most methods commented out (minimal surface exposed).

```typescript
declare global {
	interface SubtleCrypto {
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
```

**Variations / call-sites:** Comment indicates this is a partial copy from `lib.dom.d.ts` to allow crypto usage in `/common/` layer without DOM dependency. Selective API surface with most methods commented out.

---

#### Pattern: Module Loader Ambient Declarations
**Where:** `src/typings/css.d.ts:7-9`
**What:** Module declaration wildcards for CSS imports and custom loader syntax (`vs/css!*`).

```typescript
declare module "vs/css!*" { }
declare module "*.css" { }
```

**Variations / call-sites:** Minimal declarations recognizing CSS as valid module imports; used for bundler integration.

---

#### Pattern: Global Build-Time Injected Variables
**Where:** `src/typings/vscode-globals-product.d.ts:8-44`
**What:** Declarations for globals injected during build time (product metadata, CSS loader, import maps).

```typescript
declare global {
	var _VSCODE_FILE_ROOT: string;
	var _VSCODE_CSS_LOAD: (module: string) => void;

	/**
	 * @deprecated You MUST use `IProductService` whenever possible.
	 */
	var _VSCODE_PRODUCT_JSON: Record<string, any>;
	var _VSCODE_PACKAGE_JSON: Record<string, any>;

	var _VSCODE_DISABLE_CSS_IMPORT_MAP: boolean | undefined;
	var _VSCODE_USE_RELATIVE_IMPORTS: boolean | undefined;
}
```

**Variations / call-sites:** Comments flag deprecations (prefer `IProductService`), indicating migration away from global injection toward DI patterns.

---

#### Pattern: NLS (Natural Language Support) Message Injection
**Where:** `src/typings/vscode-globals-nls.d.ts:22-37`
**What:** Global message arrays and language metadata injected at build time for localization across all runtime contexts.

```typescript
declare global {
	/**
	 * All NLS messages produced by `localize` and `localize2` calls
	 * under `src/vs` translated to the language as indicated by
	 * `_VSCODE_NLS_LANGUAGE`.
	 */
	var _VSCODE_NLS_MESSAGES: string[];
	var _VSCODE_NLS_LANGUAGE: string | undefined;
}
```

**Variations / call-sites:** Comment notes these globals support `nls.localize` and `nls.localize2` in Electron main, Electron renderer, Utility Process, Node.js, Browser, and Web Worker contexts.

---

#### Pattern: Trusted Types API for Security Policy
**Where:** `src/typings/vscode-globals-ttp.d.ts:8-11`
**What:** Optional Trusted Types Policy attached to global scope for AMD-to-ESM migration context.

```typescript
declare global {
	var _VSCODE_WEB_PACKAGE_TTP: Pick<TrustedTypePolicy, 'name' | 'createScriptURL'> | undefined;
}
```

**Variations / call-sites:** Minimal surface (only `name` and `createScriptURL` picked); flagged as "AMD2ESM migration relevant."

---

#### Pattern: Ambient Third-Party Package Type Shims
**Where:** `src/typings/copilot-api.d.ts:13-153`
**What:** Full ambient module shim for `@vscode/copilot-api` due to incompatibility with `moduleResolution: "nodenext"` in the package's `.d.ts` files.

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

**Variations / call-sites:** Includes complex nested interfaces for models (`CCAModel`, `CCAModelLimits`, `CCAModelCapabilities`), request types (`RequestType` enum), and billing/policy structures.

---

#### Pattern: Web Standard Event Target Augmentation
**Where:** `src/typings/editContext.d.ts:8-43, 61-69`
**What:** EditContext API for rich text editing; combines interface definitions with concrete event classes, augmenting HTMLElement.

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

	addEventListener<K extends keyof EditContextEventHandlersEventMap>(type: K, listener: (this: GlobalEventHandlers, ev: EditContextEventHandlersEventMap[K]) => any, options?: boolean | AddEventListenerOptions): void;
}

declare class TextUpdateEvent extends Event {
	constructor(type: DOMString, options?: TextUpdateEventInit);
	readonly updateRangeStart: number;
	readonly updateRangeEnd: number;
	readonly text: DOMString;
	readonly selectionStart: number;
	readonly selectionEnd: number;
}

interface HTMLElement {
	editContext?: EditContext;
}
```

**Variations / call-sites:** Event system uses mapped type for type-safe listeners; supports multiple event types (textupdate, textformatupdate, characterboundsupdate, compositionstart, compositionend).

---

#### Pattern: Promise-Like Abstraction
**Where:** `src/typings/thenable.d.ts:12`
**What:** Minimal Thenable interface extending PromiseLike for cross-library promise compatibility (Q, jQuery.Deferred, WinJS.Promise, native Promises).

```typescript
interface Thenable<T> extends PromiseLike<T> { }
```

**Variations / call-sites:** Enables reuse of code without forcing a specific promise implementation; documented as common denominator across promise libraries.

---

## Summary

The typings directory reveals **8 distinct patterns** for integrating JavaScript runtime APIs with VS Code's TypeScript codebase:

1. **Global Runtime Polyfills** – Timer and error utilities spanning Electron/Node.js/Browser
2. **Selective Web API Exports** – Crypto API with minimal surface (digest, randomUUID only)
3. **Module Loaders** – CSS import wildcards for bundler integration
4. **Build-Time Injection** – Product metadata, file roots, and CSS loaders as globals
5. **Localization Infrastructure** – NLS message arrays and language metadata injected per-context
6. **Security Policies** – Trusted Types for AMD-to-ESM migration
7. **Third-Party Shims** – Full ambient modules for incompatible packages (Copilot API)
8. **Web Standard Extensions** – EditContext API with event classes and mapped types

**Key implications for Tauri/Rust port:**
- Runtime globals (`_VSCODE_*` variables) would need Rust equivalents or removal in favor of DI
- Web APIs (Crypto, EditContext) may require browser/WASM integration or Rust FFI
- Module system changes would affect CSS loading and import resolution
- NLS system requires localization injection mechanism in new platform
- Third-party package compatibility depends on API availability in target stack

All patterns employ `declare global` or `declare module` syntax with zero-implementation bodies—they are purely type contracts for external dependencies and build-time injections.

