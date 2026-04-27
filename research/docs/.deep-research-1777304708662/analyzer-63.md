### Files Analysed

- `/Users/norinlavaee/vscode-atomic/src/bootstrap-meta.ts` (55 LOC)

---

### Per-File Notes

#### `/Users/norinlavaee/vscode-atomic/src/bootstrap-meta.ts`

**Role**

`bootstrap-meta.ts` is the canonical runtime loader for VS Code's two root JSON configuration objects: `product.json` (product branding, platform identifiers, feature flags, policy anchors) and `package.json` (version, dependencies). It runs as a native ESM module very early in the process lifecycle—before any VS Code service container is initialised—and exports the resolved objects as module-level constants consumed by every process entry point (`main.ts`, `cli.ts`, `server-main.ts`, `server-cli.ts`, and `bootstrap-esm.ts`). The file also owns three layered-override mechanisms that modify the base objects: a build-time inlining patch, an embedded-app sub-configuration merge, and a development-time overrides file.

**Key Symbols**

| Symbol | Line | Description |
|---|---|---|
| `require` | 10 | A CommonJS-compatible `require` function synthesised from `import.meta.url` via Node's `createRequire`. Enables `require('../product.json')` etc. inside an ESM module without needing dynamic `import()`. |
| `productObj` | 12 | `let` variable typed as `Partial<IProductConfiguration> & { BUILD_INSERT_PRODUCT_CONFIGURATION?: string }`. Initialised with the sentinel `{ BUILD_INSERT_PRODUCT_CONFIGURATION: 'BUILD_INSERT_PRODUCT_CONFIGURATION' }` and potentially replaced or mutated three times before export. |
| `pkgObj` | 17 | `let` variable with an analogous `BUILD_INSERT_PACKAGE_CONFIGURATION` sentinel. Replaced by `require('../package.json')` when the sentinel survives the build step, and further merged with `package.sub.json` in embedded-app mode. |
| `product` (export) | 54 | The final, fully-resolved product configuration object. |
| `pkg` (export) | 55 | The final, fully-resolved package configuration object. |

**Control Flow**

1. **`require` bootstrap** (line 10): `createRequire(import.meta.url)` constructs a CJS loader anchored to the file's own URL, allowing synchronous `require` calls from within ESM code.

2. **Product JSON resolution** (lines 12–15):
   - `productObj` is initialised with the build-time sentinel object (line 12). The inline comment `// DO NOT MODIFY, PATCHED DURING BUILD` signals that `build/lib/inlineMeta.ts` will replace the sentinel with the actual JSON content when building a distributed bundle.
   - At runtime when the sentinel string is still present (i.e., running from source), the `if` guard on line 13 is truthy and `productObj` is overwritten with `require('../product.json')` (line 14).
   - When building a production bundle, `inlineMeta.ts` replaces the `BUILD_INSERT_PACKAGE_CONFIGURATION:"BUILD_INSERT_PACKAGE_CONFIGURATION"` marker inline in the compiled JS, making the `if` guard falsy and skipping the `require` call entirely. Note: the product JSON inline path is currently commented out in `build/lib/inlineMeta.ts` (lines 24 and 38–42 of that file), so `productObj` always falls through to `require('../product.json')` at present.

3. **Package JSON resolution** (lines 17–20): Identical pattern to product JSON. The `BUILD_INSERT_PACKAGE_CONFIGURATION` sentinel is actively replaced by `inlineMeta.ts` during builds (line 34 of `build/lib/inlineMeta.ts`), meaning in production bundles `pkgObj` is inlined and `require('../package.json')` is not called.

4. **Embedded-app sub-configuration** (lines 23–44):
   - Checked via `(process as INodeProcess).isEmbeddedApp` (line 23). This property is set on the process object by embedded VS Code instances (e.g., a child IDE embedded within a parent product).
   - Lines 26–30: Before loading the sub-config, the parent product's policy identity fields (`win32RegValueName`, `darwinBundleIdentifier`, `urlProtocol`) are captured into `productObj.parentPolicyConfig`. This preserves the enterprise policy anchor so that policies deployed to the host product still apply to the embedded child.
   - Lines 32–39: `require('../product.sub.json')` is loaded inside a `try/catch` (line 32). If `productObj.embedded` and `productSubObj.embedded` both exist, the embedded sub-object is merged first with `Object.assign` and then deleted from the sub-config before the remainder is merged onto `productObj` (lines 34–38). This ensures the embedded-specific keys are merged deeply rather than overwritten wholesale.
   - Lines 40–43: `require('../package.sub.json')` is loaded and shallow-merged onto `pkgObj` via `Object.assign`. Both `catch` blocks silently swallow errors, so missing sub-config files are treated as no-ops.

5. **Development overrides** (lines 46–52):
   - Checked via `process.env['VSCODE_DEV']` (line 47). When set, `require('../product.overrides.json')` is loaded and shallow-merged onto `productObj` (line 50). The `catch` block silently swallows a missing file.

6. **Export** (lines 54–55): `product` and `pkg` are exported as named ES module bindings pointing to the final mutated `productObj` and `pkgObj` values.

**Data Flow**

- Input sources (in priority order, later overrides earlier):
  1. Build-time inlined sentinel replacement (highest priority for `pkg` in production; currently inactive for `product`)
  2. `../product.json` (source-run baseline for `product`)
  3. `../package.json` (source-run baseline for `pkg`)
  4. `../product.sub.json` + `../package.sub.json` (embedded-app layer, conditionally applied)
  5. `../product.overrides.json` (development-only override layer)

- Output: two module-level `let` variables (`productObj`, `pkgObj`) mutated in place and then exported as `product` and `pkg`.

- Downstream consumers read `product` and `pkg` synchronously at import time. In `bootstrap-esm.ts` (line 33–34) both are spread into `globalThis._VSCODE_PRODUCT_JSON` and `globalThis._VSCODE_PACKAGE_JSON`, making them accessible to renderer and worker processes via the global scope. In `main.ts` (line 34), `product` is passed directly to `configurePortable()` for portable-mode path configuration.

**Dependencies**

| Dependency | Kind | Usage |
|---|---|---|
| `node:module` → `createRequire` | Node.js built-in | Synthesises a CJS `require` scoped to `import.meta.url` (line 6, line 10) |
| `./vs/base/common/product.js` → `IProductConfiguration` | Type-only import | Constrains the type of `productObj` (line 7) |
| `./vs/base/common/platform.js` → `INodeProcess` | Type-only import | Provides the `isEmbeddedApp` property type for the `process` cast (line 8) |
| `../product.json` | JSON file | Loaded via `require` when running from source (line 14) |
| `../package.json` | JSON file | Loaded via `require` when running from source (line 19) |
| `../product.sub.json` | JSON file (optional) | Loaded in embedded-app mode (line 33); missing file silently ignored |
| `../package.sub.json` | JSON file (optional) | Loaded in embedded-app mode (line 41); missing file silently ignored |
| `../product.overrides.json` | JSON file (optional) | Loaded when `VSCODE_DEV` is set (line 49); missing file silently ignored |
| `process` (global) | Runtime global | Read for `isEmbeddedApp` (line 23) and `env.VSCODE_DEV` (line 47) |
| `import.meta.url` | ESM built-in | Passed to `createRequire` to anchor the CJS resolver to this file's directory (line 10) |

**Build-Time Interaction with `build/lib/inlineMeta.ts`**

`build/lib/inlineMeta.ts` exports an `inlineMeta` function (line 26) that operates as a Gulp stream transform. It scans the compiled JS of `bootstrap-meta` for the string `BUILD_INSERT_PACKAGE_CONFIGURATION:"BUILD_INSERT_PACKAGE_CONFIGURATION"` (post-esbuild double-quote form) and replaces it with the actual `package.json` content inlined directly into the JS bundle (line 34). The product-JSON inlining path (`productJsonMarkerId`) exists in that file but is commented out (lines 24, 38–42) because late-stage build modifications to `product.json` (e.g., adding `darwinUniversalAssetId` in `create-universal-app.ts`) occur after the inline step and would not be reflected. As a result, `productObj` is never inlined and always loaded from disk via `require('../product.json')` even in production builds.

---

### Cross-Cutting Synthesis

`bootstrap-meta.ts` is the single point of truth for configuration identity at process startup in VS Code. Its design reflects three layered concerns that intersect in this one file: (1) build-time optimisation—eliminating filesystem reads for `package.json` in production bundles by inlining JSON at compile time; (2) embedded-app extensibility—allowing a child VS Code product to layer its own branding (`product.sub.json`) over the parent's while preserving the parent's policy anchor via `parentPolicyConfig`; (3) developer ergonomics—permitting local `product.overrides.json` to tweak product metadata without modifying tracked files.

In the context of porting VS Code core to Tauri/Rust, this file represents the boundary between "process bootstrap" and "application logic". Its entire mechanism depends on three Node.js/ESM primitives that have no direct Tauri equivalent: `createRequire` from `node:module`, `import.meta.url` for file-relative resolution, and synchronous `require()` for JSON loading. In a Tauri context, the analogous work—reading and merging product/package JSON at startup—would need to move to Rust (using `serde_json` and `include_str!` or filesystem reads in `main.rs`) or be handled by Tauri's asset embedding system. The `isEmbeddedApp` flag and the `parentPolicyConfig` capture pattern would also need a Rust-side equivalent IPC or environment variable mechanism, since Tauri's `process` abstraction does not expose arbitrary custom properties on a Node process object.

The five consuming files (`main.ts`, `bootstrap-esm.ts`, `cli.ts`, `server-main.ts`, `server-cli.ts`) all import `product` and `pkg` synchronously at module load time and use them before any async initialisation begins. This makes `bootstrap-meta.ts` a hard dependency of every VS Code process type, and any port would need an equivalent early-init mechanism that can deliver the same two configuration objects before the rest of the service container initialises.

---

### Out-of-Partition References

- `/Users/norinlavaee/vscode-atomic/src/vs/base/common/product.ts` — Defines `IProductConfiguration` (the type of `productObj`), including the `parentPolicyConfig` field at line 251–255 that `bootstrap-meta.ts` populates at line 26–30.
- `/Users/norinlavaee/vscode-atomic/src/vs/base/common/platform.ts` — Defines `INodeProcess` (line 37–49), including the `isEmbeddedApp?: boolean` property (line 47) used in the cast at `bootstrap-meta.ts:23`.
- `/Users/norinlavaee/vscode-atomic/build/lib/inlineMeta.ts` — Build-time Gulp transform that performs the `BUILD_INSERT_PACKAGE_CONFIGURATION` substitution, inlining `package.json` content into the compiled `bootstrap-meta` bundle. Product-JSON inlining is present but commented out.
- `/Users/norinlavaee/vscode-atomic/src/bootstrap-esm.ts` — Imports both `product` and `pkg`; spreads them into `globalThis._VSCODE_PRODUCT_JSON` and `globalThis._VSCODE_PACKAGE_JSON` (lines 33–34).
- `/Users/norinlavaee/vscode-atomic/src/main.ts` — Imports `product`; passes it to `configurePortable(product)` (line 34) for portable-mode data path configuration.
- `/Users/norinlavaee/vscode-atomic/src/cli.ts` — Imports `product` for CLI branding and version display.
- `/Users/norinlavaee/vscode-atomic/src/server-main.ts` — Imports `product` for server process identification.
- `/Users/norinlavaee/vscode-atomic/src/server-cli.ts` — Imports `product` for server CLI branding.
- `/Users/norinlavaee/vscode-atomic/product.json` — The root product configuration file loaded at line 14 when running from source. Contains all platform-specific identifiers (`win32RegValueName`, `darwinBundleIdentifier`, `applicationName`, etc.).
- `/Users/norinlavaee/vscode-atomic/src/vs/code/electron-main/main.ts` — Consumes `productService.parentPolicyConfig` (lines 213–214) to resolve the correct Windows registry value name or macOS bundle identifier for enterprise policy lookup.
- `/Users/norinlavaee/vscode-atomic/src/vs/sessions/contrib/policyBlocked/browser/sessionsPolicyBlocked.ts` — Reads `productService.parentPolicyConfig?.urlProtocol` (line 162) to construct a URL scheme for policy-blocked session flows.
