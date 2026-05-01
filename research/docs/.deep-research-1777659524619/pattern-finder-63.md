# Research: Porting VS Code Bootstrap Metadata to Tauri/Rust

## Overview
This research analyzes `src/bootstrap-meta.ts` (~56 LOC), which injects critical product metadata (name, version, build configuration) early in VS Code's startup sequence. This file demonstrates the patterns required to bridge Electron/Node.js initialization with TypeScript/JavaScript application code. For a Tauri/Rust port, these patterns reveal what a Rust-based host must provide to any JavaScript application layer.

---

## Pattern 1: Build-Time Metadata Injection via Placeholder Substitution

**Where:** `src/bootstrap-meta.ts:12-20`

**What:** Metadata placeholders are marked during source development and replaced with actual values during build compilation.

```typescript
let productObj: Partial<IProductConfiguration> & { BUILD_INSERT_PRODUCT_CONFIGURATION?: string } = { BUILD_INSERT_PRODUCT_CONFIGURATION: 'BUILD_INSERT_PRODUCT_CONFIGURATION' }; // DO NOT MODIFY, PATCHED DURING BUILD
if (productObj['BUILD_INSERT_PRODUCT_CONFIGURATION']) {
	productObj = require('../product.json'); // Running out of sources
}

let pkgObj = { BUILD_INSERT_PACKAGE_CONFIGURATION: 'BUILD_INSERT_PACKAGE_CONFIGURATION' }; // DO NOT MODIFY, PATCHED DURING BUILD
if (pkgObj['BUILD_INSERT_PACKAGE_CONFIGURATION']) {
	pkgObj = require('../package.json'); // Running out of sources
}
```

**Key aspects:**
- Development mode falls back to `require()` loading of JSON config files
- Production builds replace sentinel strings with actual JSON content via build tooling
- Two separate metadata objects: product configuration (VS Code-specific) and package metadata (Node.js version info)
- Conditional logic checks for sentinel presence to determine whether data was already inlined

**Variations / call-sites:**
- `build/lib/inlineMeta.ts:16,32-35` handles the build-time string replacement for package.json metadata
- Product.json inlining is commented out (lines 24, 38-42) due to late-stage build modifications
- Used in: `src/main.ts:14`, `src/server-cli.ts:11`, `src/cli.ts:10`

**Tauri/Rust porting implications:**
- Rust host must either serve metadata as environment variables, HTTP endpoints, or embed at link-time
- JavaScript loader needs equivalent detection mechanism to know if metadata is embedded or must be loaded
- Build system must be augmented to inject metadata at compilation rather than relying on Node.js require()

---

## Pattern 2: Embedded Application Configuration Override

**Where:** `src/bootstrap-meta.ts:23-44`

**What:** Support for sandboxed embedded applications that override parent application's configuration while preserving compatibility.

```typescript
if ((process as INodeProcess).isEmbeddedApp) {
	// Preserve the parent VS Code's policy identity before the
	// embedded app overrides win32RegValueName / darwinBundleIdentifier.
	productObj.parentPolicyConfig = {
		win32RegValueName: productObj.win32RegValueName,
		darwinBundleIdentifier: productObj.darwinBundleIdentifier,
		urlProtocol: productObj.urlProtocol,
	};

	try {
		const productSubObj = require('../product.sub.json');
		if (productObj.embedded && productSubObj.embedded) {
			Object.assign(productObj.embedded, productSubObj.embedded);
			delete productSubObj.embedded;
		}
		Object.assign(productObj, productSubObj);
	} catch (error) { /* ignore */ }
	try {
		const pkgSubObj = require('../package.sub.json');
		pkgObj = Object.assign(pkgObj, pkgSubObj);
	} catch (error) { /* ignore */ }
}
```

**Key aspects:**
- `INodeProcess.isEmbeddedApp` flag on `process` object enables conditional behavior
- Parent configuration preserved before override to support policy chains (Windows registry, macOS bundle identifiers)
- `.sub.json` files provide embedded variant configurations (Codium, custom distributions)
- Graceful error handling—missing sub-files don't halt startup
- Nested merge logic for `embedded` property maintains configuration hierarchy

**Variations / call-sites:**
- Referenced in: `src/vs/platform/update/electron-main/updateService.win32.ts:388`
- Used by: `src/vs/platform/secrets/electron-main/macOSCrossAppSecretSharing.ts:63-110`
- Checked in: `src/vs/platform/native/node/siblingApp.ts:66,76`

**Tauri/Rust porting implications:**
- Tauri host must expose an `isEmbeddedApp` flag through IPC or globalThis
- Configuration layering system must support product.sub.json patterns
- Parent/child process metadata inheritance requires explicit tracking in Rust initialization layer

---

## Pattern 3: Development vs. Production Configuration Branching

**Where:** `src/bootstrap-meta.ts:46-52`

**What:** Environment-specific configuration overrides applied only during development.

```typescript
let productOverridesObj = {};
if (process.env['VSCODE_DEV']) {
	try {
		productOverridesObj = require('../product.overrides.json');
		productObj = Object.assign(productObj, productOverridesObj);
	} catch (error) { /* ignore */ }
}
```

**Key aspects:**
- `VSCODE_DEV` environment variable gates optional development configuration
- Overrides applied via `Object.assign()`, so they take precedence over base product configuration
- File-not-found errors are silently ignored (try/catch with empty catch)
- Single override file merges into base configuration

**Variations / call-sites:**
- `src/main.ts:441-442` uses `VSCODE_DEV` to add "-dev" suffix to data folder names
- `src/vs/platform/product/common/product.ts:33-39` applies development modifications to product names
- Checked in: `src/cli.ts:17-21`, `src/server-cli.ts`

**Tauri/Rust porting implications:**
- Rust startup must check for dev environment flag before passing metadata to JS
- Dev mode should support loading separate configuration files or environment variable overrides
- Configuration precedence must be explicit: default < embedded/sub < dev overrides

---

## Pattern 4: Global State Injection into Runtime

**Where:** `src/bootstrap-esm.ts:33-35`

**What:** Product and package metadata propagated to JavaScript global scope for immediate consumption by AMD/ESM module system.

```typescript
globalThis._VSCODE_PRODUCT_JSON = { ...product };
globalThis._VSCODE_PACKAGE_JSON = { ...pkg };
globalThis._VSCODE_FILE_ROOT = import.meta.dirname;
```

**Key aspects:**
- Three distinct globals established before any other module loading
- `_VSCODE_PRODUCT_JSON`: full IProductConfiguration object
- `_VSCODE_PACKAGE_JSON`: package.json metadata with version info
- `_VSCODE_FILE_ROOT`: absolute file system path to VS Code resources
- Shallow copies via spread operator prevent external mutation

**Variations / call-sites:**
- Consumed by: `src/vs/amdX.ts:207,232` (AMD module system reads product)
- Used in: `src/vs/platform/product/common/product.ts:28-30` (web worker initialization)
- Referenced in: `src/vs/base/common/network.ts:366-367` (network path resolution)
- Set dynamically in browser contexts: `src/vs/code/electron-browser/workbench/workbench.ts:294`
- Propagated to workers: `src/vs/platform/webWorker/browser/webWorkerServiceImpl.ts:105`

**Tauri/Rust porting implications:**
- Rust/Tauri bridge must expose equivalent globals through JavaScript API (window object)
- `_VSCODE_FILE_ROOT` must resolve correctly in Tauri bundle structure (may differ from Electron's file paths)
- Worker threads need separate initialization to receive these globals
- Immutability contracts should be documented (these shouldn't be modified after injection)

---

## Pattern 5: Fallback Chain for Platform-Specific Identifiers

**Where:** `src/bootstrap-meta.ts:26-30` (excerpt from Pattern 2)

**What:** Preservation of platform-specific application identifiers before configuration override.

```typescript
productObj.parentPolicyConfig = {
	win32RegValueName: productObj.win32RegValueName,
	darwinBundleIdentifier: productObj.darwinBundleIdentifier,
	urlProtocol: productObj.urlProtocol,
};
```

**Key aspects:**
- Three platform-specific properties explicitly preserved: Windows registry value, macOS bundle ID, URL protocol scheme
- Forms a `parentPolicyConfig` object for policy chain lookups
- Enables embedded applications to override identifiers while preserving parent chain

**Related interface definitions:**
- `IProductConfiguration` in `src/vs/base/common/product.ts` defines ~200 configuration properties
- Platform-specific fields: `win32RegValueName`, `win32MutexName`, `darwinBundleIdentifier`, `urlProtocol`
- Full property list includes: names (short/long), Electron identifiers, URLs, extension galleries, crash reporting, NLS, and policies

**Tauri/Rust porting implications:**
- Rust host must extract platform-specific metadata from `.product` and `.package` before JavaScript initialization
- Parent policy chain tracking requires process hierarchy awareness in Rust layer
- Identifier validation should occur at bootstrap time to prevent late-stage failures

---

## Structural Summary

The `bootstrap-meta.ts` file operates at VS Code's application level zero—before any UI frameworks, services, or workbench features load. It establishes three critical initialization patterns:

1. **Metadata Loading**: Chooses between inlined (production), required from disk (development), or embedded configuration
2. **Configuration Layering**: Base → embedded override → dev override, with selective property preservation
3. **Global Exposure**: Injects metadata into JavaScript global scope for AMD/ESM modules and all workers

### Key Architectural Dependencies

- Depends on: `IProductConfiguration` and `INodeProcess` type definitions
- Feeds into: Bootstrap ESM (`bootstrap-esm.ts`), main process (`main.ts`), CLI processes (`cli.ts`, `server-cli.ts`)
- Consumed by: AMD module loader, product service, worker initialization, network path resolution

### For a Tauri/Rust Port

A Rust-based Tauri host replacing Electron must:

1. **Embed or Serve Metadata** – Replace Node.js `require()` with Rust serialization or HTTP endpoints
2. **Expose Platform Context** – Create equivalent of `INodeProcess.isEmbeddedApp` through Tauri IPC
3. **Initialize Global State** – Inject `_VSCODE_*_JSON` globals before JavaScript execution
4. **Support Configuration Layering** – Implement `.sub.json` and `.overrides.json` equivalent in Rust (TOML, JSON, or Rust structs)
5. **Preserve Policy Chains** – Track parent/child process metadata relationships in Rust process model

The 56 lines of TypeScript shown here require substantial Rust scaffolding to replicate because they rely on Node.js filesystem APIs, environment variable resolution, and dynamic `require()` semantics—all of which must be reimplemented or replaced in a Tauri context.

