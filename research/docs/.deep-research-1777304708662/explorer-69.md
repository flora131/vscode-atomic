# Partition 69 of 79 — Findings

## Scope
`extensions/types/` (2 files, 21 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Codebase Locator: extensions/types/

## Scope
This partition contains the TypeScript type definitions contribution package for VS Code.

### Implementation
- `extensions/types/lib.textEncoder.d.ts` — Global TextEncoder/TextDecoder type definitions for cross-runtime compatibility
- `extensions/types/lib.url.d.ts` — Global URL type definition for cross-runtime compatibility

## Summary
The `extensions/types/` directory contains 2 minimal TypeScript declaration files that provide polyfill type definitions for standard Web APIs (TextEncoder, TextDecoder, URL) across both browser and Node.js runtime environments. These are utility types used to ensure type consistency when VS Code code runs in different execution contexts. The declarations reference native Node.js `util` module types and DOM APIs, supporting the heterogeneous runtime environment that VS Code needs to support during an Electron-to-Tauri migration.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
### Files Analysed

- `/Users/norinlavaee/vscode-atomic/extensions/types/lib.textEncoder.d.ts` (12 lines)
- `/Users/norinlavaee/vscode-atomic/extensions/types/lib.url.d.ts` (11 lines)

---

### Per-File Notes

#### `/Users/norinlavaee/vscode-atomic/extensions/types/lib.textEncoder.d.ts`

- **Lines 1–4**: Standard Microsoft copyright header (MIT License).
- **Line 6**: Block comment explains the purpose: "Define TextEncoder + TextDecoder globals for both browser and node runtimes".
- **Line 8**: References upstream TypeScript issue https://github.com/microsoft/TypeScript/issues/31535 as the "Proper fix", indicating this file exists as a workaround for an open TypeScript gap.
- **Line 10**: `declare var TextDecoder: typeof import('util').TextDecoder;` — Declares a global ambient `TextDecoder` variable whose type is aliased from Node.js's `util` module import. This hoists the Node.js `TextDecoder` type into the global TypeScript scope so code that runs in both environments can use `TextDecoder` without a qualifying namespace.
- **Line 11**: `declare var TextEncoder: typeof import('util').TextEncoder;` — Same pattern for `TextEncoder`. Both declarations use `typeof import(...)` to borrow the type from the Node.js `util` built-in without re-declaring the interface manually, ensuring the declarations stay in sync with Node.js typings.

The file has no exports; it is a pure ambient declaration file (`.d.ts`) affecting the global TypeScript type namespace when included via `tsconfig.json`'s `include` or `files` array.

**Known consumer**: `extensions/git/tsconfig.json:36` includes `"../types/lib.textEncoder.d.ts"` directly in its `include` array, making these globals available to the entire `git` extension's TypeScript compilation unit.

---

#### `/Users/norinlavaee/vscode-atomic/extensions/types/lib.url.d.ts`

- **Lines 1–4**: Standard Microsoft copyright header (MIT License).
- **Line 6**: Block comment: "Define Url global for both browser and node runtimes".
- **Line 8**: Attribution comment: "Copied from https://github.com/DefinitelyTyped/DefinitelyTyped/issues/34960", indicating the pattern originates from a DefinitelyTyped cross-runtime compatibility discussion.
- **Line 10**: `declare const URL: typeof import('url').URL;` — Declares a global ambient `URL` constant whose type is drawn from Node.js's `url` module via a type-only dynamic import expression. Unlike `TextDecoder`/`TextEncoder`, this uses `const` (not `var`), making the declared global non-reassignable at the type level.

The file has no exports. It is also a pure ambient declaration file. No direct `tsconfig.json` `include` reference to this file was found in the searched files beyond the `extensions/git/tsconfig.json` which only references `lib.textEncoder.d.ts`; `lib.url.d.ts` may be consumed by other extension tsconfigs not surfaced in the search results.

---

### Cross-Cutting Synthesis

Both files in `extensions/types/` are thin ambient TypeScript declaration shims (`.d.ts`) that solve a single problem: the TypeScript compiler's `lib` setting for extensions (`extensions/tsconfig.base.json:5–7`) is `["ES2024"]` only, which omits DOM and Node.js globals. Standard Web APIs — `TextEncoder`, `TextDecoder`, and `URL` — exist in both browser and Node.js runtimes but are not included in the `ES2024` lib, so they would otherwise be invisible to the TypeScript compiler when targeting extensions. Rather than switching the entire `lib` to `DOM` (which would import thousands of browser-specific declarations inappropriate for Node.js extension hosts), each file uses `typeof import('util').X` or `typeof import('url').X` to re-export exactly the required type into the global scope. This technique lets an extension's source code reference `TextEncoder`, `TextDecoder`, and `URL` as bare globals without qualifying them under `util.TextEncoder` or similar, while keeping the type definition anchored to Node.js's own typings. The workaround pattern is acknowledged as a stopgap: `lib.textEncoder.d.ts:8` cites an open TypeScript issue (`microsoft/TypeScript#31535`) as the proper long-term fix.

For a Tauri/Rust port, these files represent a TypeScript-layer concern. In Rust, `TextEncoder`/`TextDecoder`/`URL` equivalents must be provided by the WebView2/WKWebView JavaScript bridge or by Rust crates, and any TypeScript code compiled against extension APIs would need equivalent type stubs for whatever JS runtime the Tauri webview exposes.

---

### Out-of-Partition References

- `/Users/norinlavaee/vscode-atomic/extensions/git/tsconfig.json:36` — Directly includes `../types/lib.textEncoder.d.ts` in its compilation unit.
- `/Users/norinlavaee/vscode-atomic/extensions/tsconfig.base.json:5–7` — Sets `lib: ["ES2024"]` for all extensions, establishing the gap that makes these shims necessary.
- TypeScript upstream issue `microsoft/TypeScript#31535` — Referenced at `lib.textEncoder.d.ts:8` as the proper fix for missing global TextEncoder/TextDecoder declarations.
- DefinitelyTyped issue `DefinitelyTyped/DefinitelyTyped#34960` — Referenced at `lib.url.d.ts:8` as the source of the `URL` declaration pattern.
- `/Users/norinlavaee/vscode-atomic/src/vs/base/test/node/uri.perf.data.txt:53912–53913` — Contains example file paths matching both declaration files (performance test fixture data, not a functional consumer).

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Pattern Research: Porting VS Code Core IDE Functionality from TypeScript/Electron to Tauri/Rust
## Partition 69 of 79: Extensions Type Declarations

### Scope
- `extensions/types/` (2 files, 21 LOC) — types contribution

---

## Patterns Found

#### Pattern 1: Cross-Runtime Global Type Declaration for TextEncoder/TextDecoder
**Where:** `extensions/types/lib.textEncoder.d.ts:1-12`
**What:** Global type declarations that unify TextEncoder and TextDecoder APIs across Node.js and browser runtimes using typeof imports from the 'util' module.

```typescript
/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

// Define TextEncoder + TextDecoder globals for both browser and node runtimes
//
// Proper fix: https://github.com/microsoft/TypeScript/issues/31535

declare var TextDecoder: typeof import('util').TextDecoder;
declare var TextEncoder: typeof import('util').TextEncoder;
```

**Key aspects:**
- Uses `declare var` for global scope availability
- References 'util' module types via typeof imports
- Enables code to use `new TextEncoder()` and `new TextDecoder()` without explicit imports
- Works across both Node.js and browser contexts
- References TypeScript issue #31535 as the upstream motivation

---

#### Pattern 2: Cross-Runtime Global Type Declaration for URL
**Where:** `extensions/types/lib.url.d.ts:1-11`
**What:** Global type declaration for the URL constructor with similar unification pattern, sourced from DefinitelyTyped discussion about cross-runtime compatibility.

```typescript
/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

// Define Url global for both browser and node runtimes
//
// Copied from https://github.com/DefinitelyTyped/DefinitelyTyped/issues/34960

declare const URL: typeof import('url').URL;
```

**Key aspects:**
- Uses `declare const` for read-only global
- Establishes URL as available globally without explicit import
- Enables `new URL()` constructor usage
- Sourced from DefinitelyTyped issue tracking
- Mirrors the TextEncoder/TextDecoder pattern structure

---

## Related Implementation Patterns

#### Runtime Abstraction with Lazy Initialization
**Found in:** `src/vs/base/common/buffer.ts:21-65`
**Used for:** Safe encoding/decoding with fallback from Node Buffer to TextEncoder/TextDecoder

The buffer implementation demonstrates lazy initialization of TextEncoder/TextDecoder:

```typescript
let textEncoder: { encode: (input: string) => Uint8Array } | null;
let textDecoder: { decode: (input: Uint8Array) => string } | null;

static fromString(source: string, options?: { dontUseNodeBuffer?: boolean }): VSBuffer {
	const dontUseNodeBuffer = options?.dontUseNodeBuffer || false;
	if (!dontUseNodeBuffer && hasBuffer) {
		return new VSBuffer(Buffer.from(source));
	} else {
		if (!textEncoder) {
			textEncoder = new TextEncoder();
		}
		return new VSBuffer(textEncoder.encode(source));
	}
}

toString(): string {
	if (hasBuffer) {
		return this.buffer.toString();
	} else {
		if (!textDecoder) {
			textDecoder = new TextDecoder(undefined, { ignoreBOM: true });
		}
		return textDecoder.decode(this.buffer);
	}
}
```

**Pattern aspects:**
- Detects availability of Node.js Buffer at runtime
- Uses Buffer for performance when available
- Falls back to TextEncoder/TextDecoder for browser contexts
- Lazy-initializes encoders on first use
- Accepts options to override Node Buffer usage

#### Specialized TextDecoder Usage for Encoding Variants
**Found in:** `src/vs/editor/common/core/stringBuilder.ts:10-24`
**Used for:** Handling UTF-16LE and UTF-16BE encoding scenarios

```typescript
let _utf16LE_TextDecoder: TextDecoder | null;
function getUTF16LE_TextDecoder(): TextDecoder {
	if (!_utf16LE_TextDecoder) {
		_utf16LE_TextDecoder = new TextDecoder('UTF-16LE');
	}
	return _utf16LE_TextDecoder;
}

let _utf16BE_TextDecoder: TextDecoder | null;
function getUTF16BE_TextDecoder(): TextDecoder {
	if (!_utf16BE_TextDecoder) {
		_utf16BE_TextDecoder = new TextDecoder('UTF-16BE');
	}
	return _utf16BE_TextDecoder;
}
```

**Pattern aspects:**
- Static per-encoding decoders with lazy initialization
- Caches decoder instances to avoid repeated instantiation
- Supports multiple encodings (UTF-16LE, UTF-16BE)
- Single global instance per encoding type

#### TextEncoder Usage for Cryptographic Operations
**Found in:** `src/vs/platform/sign/browser/signService.ts:77-79`
**Used for:** Converting license keys to byte arrays for AES encryption

```typescript
const keyBytes = new TextEncoder().encode(this.productService.serverLicense?.join('\n') || '');
for (let i = 0; i + STEP_SIZE < keyBytes.length; i += STEP_SIZE) {
	const key = await crypto.subtle.importKey('raw', keyBytes.slice(i + IV_SIZE, i + IV_SIZE + KEY_SIZE), { name: 'AES-CBC' }, false, ['decrypt']);
	wasm = await crypto.subtle.decrypt({ name: 'AES-CBC', iv: keyBytes.slice(i, i + IV_SIZE) }, key, wasm);
```

**Pattern aspects:**
- Inline instantiation for single-use operations
- Encodes strings to UTF-8 bytes for Web Crypto API
- Works with subsequent cryptographic operations
- No caching needed for one-time operations

#### URL Constructor Usage for Import Meta Resolution
**Found in:** `src/bootstrap-import.ts:25`
**Used for:** Converting file paths to file URLs with import.meta.url resolution

```typescript
const injectPackageJSONPath = fileURLToPath(new URL('../package.json', pathToFileURL(injectPath)));
```

**Pattern aspects:**
- Uses URL constructor for relative path resolution
- Works with file:// protocol URLs
- Enables portable cross-platform module path resolution
- Combines with fileURLToPath utility for filesystem paths

---

## Cross-Runtime Type System Summary

The extensions/types directory establishes a pattern for declaring APIs that differ between Node.js and browser environments:

1. **TextEncoder/TextDecoder** (`lib.textEncoder.d.ts`): Global declaration that unifies encoding/decoding across runtimes via the Node.js 'util' module types.

2. **URL** (`lib.url.d.ts`): Global declaration for the URL constructor available in both runtimes, with types sourced from the 'url' module.

3. **Runtime Detection Patterns** (evidenced in buffer.ts): Code checks for global availability (e.g., `typeof Buffer !== 'undefined'`) and falls back to TextEncoder/TextDecoder when Node APIs unavailable.

4. **Lazy Initialization**: Actual TextEncoder/TextDecoder instances created on first use to avoid unnecessary overhead in paths that may use native Node Buffer instead.

5. **Encoding Variants**: Multiple TextDecoder instances can be cached for different encodings (UTF-8, UTF-16LE, UTF-16BE).

### Porting Implications for Tauri/Rust

These patterns represent abstraction layers for platform-specific functionality:

- **TextEncoder/TextDecoder abstraction**: In Tauri/Rust, equivalent encoding would come from Rust's `std::str` and UTF-8 handling or the `encoding_rs` crate for variant encodings.
- **URL handling**: Rust's `url` crate provides parsing functionality equivalent to JavaScript's URL constructor.
- **Runtime detection pattern**: Rust compile-time features or runtime capability checks would replace JavaScript's dynamic `typeof Buffer` detection.
- **Lazy initialization**: Rust's `OnceCell` or `lazy_static` patterns would replace JavaScript's mutable static variables.

The type declarations in extensions/types/ are minimal contributions (21 LOC total) that establish compatibility layer assumptions used throughout the IDE codebase.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
