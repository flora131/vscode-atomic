### Files Analysed

- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-meta.ts` (55 lines)

---

### Per-File Notes

#### `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-meta.ts`

- **Role:**  
  This module is the single authoritative source for product and package metadata available at process startup. It runs in the Node.js ESM context (not the browser renderer), synchronously assembles two plain-object snapshots — `product` (typed as `Partial<IProductConfiguration>`) and `pkg` (untyped) — and re-exports them as named ES module exports. All subsequent runtime code that needs product identity or package version imports from this module.

- **Key symbols:**

  | Symbol | Line | Description |
  |---|---|---|
  | `productObj` | 12 | Mutable local accumulator for product configuration. Initially assigned the build-injection sentinel. |
  | `pkgObj` | 17 | Mutable local accumulator for package configuration. Same sentinel pattern. |
  | `product` (export) | 54 | Named export of the fully-resolved `productObj`. |
  | `pkg` (export) | 55 | Named export of the fully-resolved `pkgObj`. |
  | `BUILD_INSERT_PRODUCT_CONFIGURATION` | 12 | String sentinel literal that the build system patches in-place to embed the actual product JSON object. |
  | `BUILD_INSERT_PACKAGE_CONFIGURATION` | 17 | String sentinel literal patched by the build system to embed the actual package JSON object. |

- **Control flow:**

  1. **ESM `require` shim** (line 10): `createRequire(import.meta.url)` creates a CommonJS-compatible `require` bound to the file's own URL. This is used throughout the file because the adjacent JSON files (`product.json`, `package.json`, `product.sub.json`, `package.sub.json`, `product.overrides.json`) are loaded with synchronous `require`, not `import`.

  2. **Product sentinel check** (lines 12–15):  
     `productObj` is initialised with `{ BUILD_INSERT_PRODUCT_CONFIGURATION: 'BUILD_INSERT_PRODUCT_CONFIGURATION' }`. The comment "DO NOT MODIFY, PATCHED DURING BUILD" indicates the build pipeline replaces the entire right-hand-side expression with the inlined product JSON. At runtime (running from sources), the property is still the sentinel string, so the `if` at line 13 is truthy and `productObj` is replaced with `require('../product.json')`.

  3. **Package sentinel check** (lines 17–20):  
     Identical pattern for `pkgObj` using `BUILD_INSERT_PACKAGE_CONFIGURATION`. Falls back to `require('../package.json')` when running from sources (line 19).

  4. **Embedded-app branch** (lines 23–44):  
     Gated by `(process as INodeProcess).isEmbeddedApp` (line 23). When true:  
     a. Saves a `parentPolicyConfig` snapshot of the three platform-identity fields (`win32RegValueName`, `darwinBundleIdentifier`, `urlProtocol`) from the current `productObj` into `productObj.parentPolicyConfig` (lines 26–30). This preserves the host VS Code's policy identity before the embedded-app overrides clobber it.  
     b. Tries to `require('../product.sub.json')` (line 33). If both `productObj.embedded` and `productSubObj.embedded` exist, their `embedded` sub-objects are merged first with `Object.assign` (line 35), and the `embedded` key is deleted from `productSubObj` (line 36) before the remaining fields are merged into `productObj` (line 38). Errors are silently swallowed (line 39).  
     c. Tries to `require('../package.sub.json')` (line 41) and merges it into `pkgObj` (line 42). Errors are silently swallowed (line 43).

  5. **Dev-mode overrides** (lines 46–52):  
     When `process.env['VSCODE_DEV']` is set (line 47), tries to `require('../product.overrides.json')` (line 49) and merges it last into `productObj` (line 50), so overrides win over everything. Errors are silently swallowed (line 51).

  6. **Export** (lines 54–55): The final values of `productObj` and `pkgObj` are exported as the named constants `product` and `pkg`.

- **Data flow:**

  ```
  Build pipeline (patch sentinel) OR ../product.json (source run)
      → productObj  (Partial<IProductConfiguration>)
          [embedded-app] + ../product.sub.json  (Object.assign, embedded sub-merge first)
          [VSCODE_DEV]   + ../product.overrides.json (Object.assign, last-wins)
      → export product

  Build pipeline (patch sentinel) OR ../package.json (source run)
      → pkgObj  (untyped plain object)
          [embedded-app] + ../package.sub.json  (Object.assign)
      → export pkg
  ```

  `parentPolicyConfig` is a side-effect written into `productObj` before the sub-file merge so that the three identity strings are captured at their pre-override values.

- **Dependencies:**

  | Dependency | Import style | Line |
  |---|---|---|
  | `node:module` (`createRequire`) | ESM named import | 6 |
  | `./vs/base/common/product.js` (`IProductConfiguration`) | `import type` | 7 |
  | `./vs/base/common/platform.js` (`INodeProcess`) | `import type` | 8 |
  | `../product.json` | synchronous `require` | 14 |
  | `../package.json` | synchronous `require` | 19 |
  | `../product.sub.json` | synchronous `require` | 33 |
  | `../package.sub.json` | synchronous `require` | 41 |
  | `../product.overrides.json` | synchronous `require` | 49 |
  | `process` (global) | implicit Node.js global | 23, 47 |

  The `import type` imports at lines 7–8 are erased at compile time and have no runtime presence; they solely provide TypeScript type-checking for `productObj` and `(process as INodeProcess).isEmbeddedApp`.

---

### Cross-Cutting Synthesis

In the Electron/Node.js model, `bootstrap-meta.ts` runs synchronously in the main process before any renderer or extension host is spawned, making synchronous `require` calls to load JSON from the filesystem affordable. In a Tauri/Rust shell the equivalent layer must be designed for a different runtime topology: there is no Node.js main process, so the synchronous JSON `require` calls have no direct analogue.

The three-tier configuration merging (base JSON → sub-file overlay → dev overrides) and the `BUILD_INSERT_*` sentinel-replacement strategy both need a counterpart in Rust. The sentinel pattern could be replicated with compile-time `include_str!` macros or by embedding JSON via `build.rs` constants, allowing the build pipeline to substitute configuration at compile time rather than patching bytecode. The embedded-app `parentPolicyConfig` snapshot is a platform-specific concern (Windows registry key, macOS bundle identifier) that maps to Tauri's `tauri.conf.json` product identity fields; preserving a parent identity before child overrides would require an equivalent snapshot step during Tauri's initialization sequence. The `VSCODE_DEV` override mechanism maps naturally to a Tauri build profile or a runtime environment variable read via `std::env::var`. The two exported symbols `product` and `pkg` are effectively the serialization boundary between build-time identity data and runtime code; in Tauri/Rust they would be replaced by a static `ProductConfig` struct populated once at startup and made available through Tauri's `State` manager or a global `OnceLock`.

---

### Out-of-Partition References

- `/home/norinlavaee/projects/vscode-atomic/src/vs/base/common/product.ts` — defines `IProductConfiguration` (the type constraint on `productObj`), including `parentPolicyConfig`, `embedded`, `win32RegValueName`, `darwinBundleIdentifier`, and `urlProtocol` fields referenced at lines 7, 26–29.
- `/home/norinlavaee/projects/vscode-atomic/src/vs/base/common/platform.ts` — defines `INodeProcess` with the `isEmbeddedApp?: boolean` property used in the type cast at line 23.
- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-esm.ts` — likely imports `product`/`pkg` from this module to make metadata available to the ESM bootstrap sequence.
- `/home/norinlavaee/projects/vscode-atomic/src/main.ts` — the main-process entry point that depends on resolved product identity at startup.
- `/home/norinlavaee/projects/vscode-atomic/src/vs/platform/product/common/product.ts` — the platform-level product service that exposes `IProductConfiguration` to dependency-injection consumers throughout the application and likely sources its data from the `product` export of this module.
- `../product.json` (repo root) — primary product metadata loaded at runtime when running from sources (line 14).
- `../package.json` (repo root) — primary package metadata loaded at runtime when running from sources (line 19).
- `../product.sub.json` (repo root) — optional embedded-app product overlay (line 33).
- `../package.sub.json` (repo root) — optional embedded-app package overlay (line 41).
- `../product.overrides.json` (repo root) — optional developer-mode product overrides (line 49).
