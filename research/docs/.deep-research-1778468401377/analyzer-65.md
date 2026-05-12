### Files Analysed

- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-meta.ts` (31 LOC — primary file)
- `/home/norinlavaee/projects/vscode-atomic/src/vs/base/common/product.ts` (lines 67+, for `IProductConfiguration` interface shape)
- `/home/norinlavaee/projects/vscode-atomic/product.json` (runtime source of `productObj`)
- `/home/norinlavaee/projects/vscode-atomic/build/lib/inlineMeta.ts` (build-time patching of `BUILD_INSERT_*` sentinels)

---

### Per-File Notes

#### `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-meta.ts`

**Module setup (lines 6–9)**

The file is an ES module. It imports `createRequire` from the Node built-in `node:module` at line 6, then calls `createRequire(import.meta.url)` at line 9 to produce a CommonJS-style `require` bound to the current file's URL. This is necessary because `product.json` and `package.json` are plain JSON files that cannot be statically imported in all build contexts.

`IProductConfiguration` is imported as a `type`-only import at line 7 from `./vs/base/common/product.js`, meaning it is erased entirely at runtime and carries no executable cost.

**Product metadata initialization (lines 11–14)**

`productObj` is declared as `let` with type `Partial<IProductConfiguration> & { BUILD_INSERT_PRODUCT_CONFIGURATION?: string }`. Its initial value at line 11 is an object literal `{ BUILD_INSERT_PRODUCT_CONFIGURATION: 'BUILD_INSERT_PRODUCT_CONFIGURATION' }`.

This object literal is a sentinel: the build pipeline (`build/lib/inlineMeta.ts:32–35`) searches the compiled output for the string pattern `BUILD_INSERT_PACKAGE_CONFIGURATION:"BUILD_INSERT_PACKAGE_CONFIGURATION"` and replaces it in-place with the actual JSON contents (without the outer braces). For `productObj`, the analogous `productJsonMarkerId` replacement is currently **commented out** in `inlineMeta.ts` at lines 38–42, with a TODO comment explaining that late build-time mutations (e.g., `darwinUniversalAssetId`, `target`) make full inlining unsafe for product.json at this time.

The guard `if (productObj['BUILD_INSERT_PRODUCT_CONFIGURATION'])` at line 12 detects whether the build-time patch was applied: if the sentinel key is still present (truthy string), it means the file is running from sources, so `require('../product.json')` is called at line 13 to load the file via the filesystem. When the sentinel has been replaced with inlined JSON, the key is absent and the `require` is skipped.

**Package metadata initialization (lines 16–18)**

`pkgObj` follows the same two-stage pattern. The sentinel `{ BUILD_INSERT_PACKAGE_CONFIGURATION: 'BUILD_INSERT_PACKAGE_CONFIGURATION' }` is declared at line 16. The guard at line 17 checks presence of the key; if still present, line 18 calls `require('../package.json')`. For `pkgObj`, the `inlineMeta.ts` build step **is** active: `packageJsonMarkerId` at `build/lib/inlineMeta.ts:16` is uncommented, and the replacement logic at lines 32–35 splices the serialized `package.json` content into the compiled bundle string.

**Development override (lines 21–27)**

`productOverridesObj` is initialized to `{}` at line 21. When the environment variable `VSCODE_DEV` is set (line 22), the file attempts `require('../product.overrides.json')` at line 24 inside a try/catch that silently swallows `ENOENT` or any parse error. On success, `Object.assign(productObj, productOverridesObj)` at line 25 merges the overrides into `productObj` shallowly — keys present in `product.overrides.json` overwrite corresponding keys in the base product object, enabling local customization without modifying `product.json`. `product.overrides.json` is listed in `.gitignore` at line 24 of that file and excluded from build filters at `build/filters.ts:97`.

**Exports (lines 29–30)**

`product` and `pkg` are exported as named constants. At the point of export, `productObj` is either the inlined singleton, the `require`d `product.json` contents optionally merged with overrides, or the inlined `package.json` for `pkgObj`.

**Consumers**

The exported names are imported by five entry-point files in `src/`:
- `bootstrap-esm.ts:8` — imports both `product` and `pkg`
- `main.ts:14` — imports `product`
- `cli.ts:10` — imports `product`
- `server-main.ts:17` — imports `product`
- `server-cli.ts:11` — imports `product`

**`IProductConfiguration` shape (product.ts:67–)**

The interface at `src/vs/base/common/product.ts:67` defines all fields that `productObj` can carry: `version`, `nameShort`, `nameLong`, `applicationName`, `urlProtocol`, `dataFolderName`, `sharedDataFolderName`, `builtInExtensions`, `extensionsGallery`, `mcpGallery`, and many more platform-specific and telemetry fields. `Partial<IProductConfiguration>` at line 11 of `bootstrap-meta.ts` means all fields are optional on the declared type, which accommodates the sentinel-only initial state.

**Build integration (`build/lib/inlineMeta.ts`)**

`inlineMeta` at line 26 is a Gulp transform stream. It scans compiled output files whose basenames match entries in `ctx.targetPaths`. For matched files, it finds the `packageJsonMarkerId` sentinel string (post-esbuild double-quote form) at line 32 and splices in the full `package.json` object body at line 34 using `JSON.stringify(...).slice(1, -1)` to strip surrounding braces. The product sentinel replacement is commented out (lines 38–42). The file is re-emitted as a modified Vinyl file at line 49.

---

### Cross-Cutting Synthesis

`bootstrap-meta.ts` is a 31-line module that provides the two most fundamental metadata objects — `product` and `pkg` — needed by all VS Code entry points before any workbench or service infrastructure is initialized. Its design encodes a dual-path loading strategy governed by sentinel string detection: in a fully bundled build the sentinel is replaced inline by the `inlineMeta` Gulp transform (`build/lib/inlineMeta.ts`) so no filesystem I/O is needed for `pkg`; when running from sources the sentinel remains and `require()` loads the JSON files directly. A third path layer activates when `VSCODE_DEV` is set, merging an optional `product.overrides.json` into the product object to allow per-developer feature flag injection without altering committed files. The type contract is enforced only at compile time via the imported `IProductConfiguration` interface, which disappears entirely at runtime. All five process entry points (`main.ts`, `cli.ts`, `server-main.ts`, `server-cli.ts`, `bootstrap-esm.ts`) are wired to import from this single source of truth before doing any further initialization work.

---

### Out-of-Partition References

- `/home/norinlavaee/projects/vscode-atomic/src/vs/base/common/product.ts` — defines `IProductConfiguration` (line 67); governs the full type contract for `product` export
- `/home/norinlavaee/projects/vscode-atomic/build/lib/inlineMeta.ts` — `inlineMeta()` function (line 26); implements build-time sentinel replacement for `BUILD_INSERT_PACKAGE_CONFIGURATION`; product sentinel is currently commented out (lines 38–42)
- `/home/norinlavaee/projects/vscode-atomic/product.json` — runtime source loaded at `bootstrap-meta.ts:13`; contains `nameShort`, `applicationName`, `urlProtocol`, `builtInExtensions`, etc.
- `/home/norinlavaee/projects/vscode-atomic/package.json` — runtime source loaded at `bootstrap-meta.ts:18`; consumed as `pkg` export
- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-esm.ts:8` — imports both `product` and `pkg`
- `/home/norinlavaee/projects/vscode-atomic/src/main.ts:14` — imports `product`
- `/home/norinlavaee/projects/vscode-atomic/src/cli.ts:10` — imports `product`
- `/home/norinlavaee/projects/vscode-atomic/src/server-main.ts:17` — imports `product`
- `/home/norinlavaee/projects/vscode-atomic/src/server-cli.ts:11` — imports `product`
- `/home/norinlavaee/projects/vscode-atomic/.gitignore:24` — excludes `product.overrides.json` from version control
- `/home/norinlavaee/projects/vscode-atomic/build/filters.ts:97` — excludes `product.overrides.json` from build artifact filters
