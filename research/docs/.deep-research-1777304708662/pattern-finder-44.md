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

