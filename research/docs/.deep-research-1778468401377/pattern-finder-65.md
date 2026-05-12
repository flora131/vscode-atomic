# Pattern Finder: Bootstrap Metadata Patterns

## Scope Analysis
File: `src/bootstrap-meta.ts` (31 LOC)

This file implements a configuration metadata bootstrapping pattern for VS Code, using build-time patching and runtime environment detection. It serves as the entry point for loading product and package metadata across different execution contexts (desktop, web, server, CLI).

---

#### Pattern 1: Build-Time Patched Constants with Runtime Fallback
**Where:** `src/bootstrap-meta.ts:11-14`
**What:** Placeholder constants that get replaced during build pipeline, with runtime fallback to load from JSON files for development mode.

```typescript
let productObj: Partial<IProductConfiguration> & { BUILD_INSERT_PRODUCT_CONFIGURATION?: string } = { BUILD_INSERT_PRODUCT_CONFIGURATION: 'BUILD_INSERT_PRODUCT_CONFIGURATION' }; // DO NOT MODIFY, PATCHED DURING BUILD
if (productObj['BUILD_INSERT_PRODUCT_CONFIGURATION']) {
	productObj = require('../product.json'); // Running out of sources
}
```

**Key aspects:**
- Sentinel string (`'BUILD_INSERT_PRODUCT_CONFIGURATION'`) serves as both placeholder and detection flag
- Build system replaces placeholder with actual product config object
- Development mode falls back to `require()` for JSON loading via CommonJS require
- Uses `createRequire()` from `node:module` (ESM compatibility)
- Type system maintains both production object shape and build marker

---

#### Pattern 2: Environment-Conditional Development Overrides
**Where:** `src/bootstrap-meta.ts:21-27`
**What:** Runtime loading of optional override configuration only when `VSCODE_DEV` environment variable is set.

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
- Guards development-only configuration loading with `process.env['VSCODE_DEV']` check
- Silent error handling (ignore missing override file)
- Shallow merge via `Object.assign()` applies overrides to base config
- Allows developers to customize product metadata without modifying checked-in files

---

#### Pattern 3: ESM-Compatible CommonJS Module Loading
**Where:** `src/bootstrap-meta.ts:6-9`
**What:** Bootstrap pattern for loading JSON configuration in ESM context using Node's `createRequire()` helper.

```typescript
import { createRequire } from 'node:module';

const require = createRequire(import.meta.url);
```

**Key aspects:**
- Creates CommonJS `require()` function within ESM module context
- Enables loading `.json` files in ESM (no native JSON module imports in standard spec)
- Uses `import.meta.url` to anchor require resolution to current file location
- Pattern appears across 10+ bootstrap and platform files (consistent convention)

---

#### Pattern 4: Typed Configuration Object Exports
**Where:** `src/bootstrap-meta.ts:29-30`
**What:** Named exports of configuration objects with type safety from `IProductConfiguration` interface.

```typescript
export const product = productObj;
export const pkg = pkgObj;
```

**Key aspects:**
- Simple re-export of mutable locals after build/override processing
- Consumed downstream by `bootstrap-esm.ts` (line 8: `import { product, pkg }`)
- Assigned to globalThis in ESM bootstrap: `globalThis._VSCODE_PRODUCT_JSON = { ...product }`
- Part of module dependency chain: bootstrap-meta → bootstrap-esm → bootstrap-node

---

#### Pattern 5: Type-Safe Partial Configuration with Build Marker
**Where:** `src/bootstrap-meta.ts:11-12` (type declaration)
**What:** Intersection type combining partial interface with build marker field for type safety during development.

```typescript
let productObj: Partial<IProductConfiguration> & { BUILD_INSERT_PRODUCT_CONFIGURATION?: string } = { BUILD_INSERT_PRODUCT_CONFIGURATION: 'BUILD_INSERT_PRODUCT_CONFIGURATION' };
```

**Key aspects:**
- `Partial<IProductConfiguration>` allows incomplete config in dev mode
- Additional intersection type `{ BUILD_INSERT_PRODUCT_CONFIGURATION?: string }` documents build marker
- TypeScript validates config shape while permitting sentinel value
- Same pattern applied to package config (line 16)

---

#### Pattern 6: Dual-Path Configuration Loading (Build vs Source)
**Where:** `src/bootstrap-meta.ts:11-19`
**What:** Conditional assignment pattern enabling two configuration sources: patched build artifacts or source files.

```typescript
let productObj = { BUILD_INSERT_PRODUCT_CONFIGURATION: '...' };
if (productObj['BUILD_INSERT_PRODUCT_CONFIGURATION']) {
	productObj = require('../product.json');
}

let pkgObj = { BUILD_INSERT_PACKAGE_CONFIGURATION: '...' };
if (pkgObj['BUILD_INSERT_PACKAGE_CONFIGURATION']) {
	pkgObj = require('../package.json');
}
```

**Key aspects:**
- Build system patches placeholder objects in `productObj` and `pkgObj` directly
- If patched (no sentinel present), bootstrap skips require() calls
- If unpatched (sentinel present), treats as source run and loads JSON
- Enables single file to work in both bundled and development contexts
- Parallel implementation for product and package configs

---

## Related Patterns in Codebase

**Usage context:** `src/bootstrap-esm.ts:8`
```typescript
import { product, pkg } from './bootstrap-meta.js';
```

**Global assignment:** `src/bootstrap-esm.ts:33-34`
```typescript
globalThis._VSCODE_PRODUCT_JSON = { ...product };
globalThis._VSCODE_PACKAGE_JSON = { ...pkg };
```

**Environment variable detection:** Pattern appears across:
- `src/main.ts` (lines 442, 530, 533, 631)
- `src/server-cli.ts` (line 17)
- `src/server-main.ts` (line 238)
- `src/bootstrap-esm.ts` (line 71)

The configuration patterns enable VS Code to support three execution modes: (1) production builds with patched configs, (2) development source mode with dynamic JSON loading, and (3) development overrides for customization. This architecture supports Tauri/Rust porting by establishing a clear metadata interface and allowing build-time configuration injection independent of runtime environment.

