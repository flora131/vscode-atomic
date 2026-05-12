### Files Analysed

- `/home/norinlavaee/projects/vscode-atomic/eslint.config.js` (2,832 lines)
- `/home/norinlavaee/projects/vscode-atomic/.eslint-plugin-local/index.ts`
- `/home/norinlavaee/projects/vscode-atomic/.eslint-plugin-local/code-layering.ts`
- `/home/norinlavaee/projects/vscode-atomic/.eslint-plugin-local/code-import-patterns.ts`
- `/home/norinlavaee/projects/vscode-atomic/.eslint-plugin-local/code-no-static-node-module-import.ts`

---

### Per-File Notes

#### `eslint.config.js`

- **Role:** The single, authoritative enforcement mechanism for VS Code's architectural invariants. It encodes environment partitioning, inter-layer import rules, banned APIs, third-party dependency allow-lists, and custom code-quality invariants through a flat ESLint config backed by a suite of local plugins.

---

- **Key sections (with line refs):**

  - Lines 1–31: Imports and global ignores. Loads `typescript-eslint`, `@stylistic/eslint-plugin-ts`, the project-local plugin at `./.eslint-plugin-local/index.ts`, the copilot-local plugin at `./extensions/copilot/.eslintplugin/index.ts`, `eslint-plugin-import`, `eslint-plugin-jsdoc`, and `eslint-plugin-header`. The global `.eslint-ignore` file is read dynamically at line 19.
  - Lines 32–137: Global rules for all JS/TS files. Core quality rules plus the `local/code-layering` rule (lines 101–125) that defines the six recognized runtime environments and their allowed import relationships.
  - Lines 138–175: TypeScript-specific rules block. Adds `@stylistic/ts` and `@typescript-eslint` plugins; enforces PascalCase class names.
  - Lines 176–187: Telemetry property ban for `src/**/*.ts` via `local/code-no-telemetry-common-property`.
  - Lines 188–338: `local/code-no-in-operator` rule applied to `**/*.ts` with an extensive set of known exceptions (roughly 150 files exempt from this restriction).
  - Lines 340–783: Strict `no-explicit-any` block applied to `git`, `git-base`, `github` extensions and all `src/**/*.ts` with ~200 known legacy exceptions.
  - Lines 784–836: Test-specific rule overrides — disabling `code-no-dangerous-type-assertions`, enabling `code-no-test-only` as error, `code-no-test-async-suite`, `code-must-use-result`, and `code-ensure-no-disposables-leak-in-test`.
  - Lines 837–866: `git` extension bans non-type imports from `*/api/git`; runtime const enum values must come from `git.constants`.
  - Lines 867–958: `vscode.d.ts` / `vscode.proposed.*.d.ts` API shape rules: `vscode-dts-create-func`, `vscode-dts-literal-or-types`, `vscode-dts-string-type-literals`, `vscode-dts-interface-naming`, `vscode-dts-cancellation`, `vscode-dts-use-export`, `vscode-dts-use-thenable`, `vscode-dts-vscode-in-comments`, `vscode-dts-provider-naming`, `vscode-dts-event-naming`.
  - Lines 960–1003: `vscode.d.ts`-only jsdoc requirements.
  - Lines 1005–1019: `local/code-amd-node-module` warning for `src/**/{common,browser}/**/*.ts`.
  - Lines 1020–1050: `node/electron-main/electron-utility` layer adds `__dirname`, `__filename`, `require` to the globally-banned globals list (ESM incompatibility signal).
  - Lines 1051–1077: `local/code-no-static-node-module-import` error for `src/vs/code/electron-main/**`, `src/vs/code/node/**`, `src/vs/platform/*/electron-main/**`, `src/vs/platform/*/node/**`. Exceptions granted for three sub-paths that run in separate processes or use small/safe modules.
  - Lines 1078–1427: Browser/electron-browser layer document and window API restrictions via `no-restricted-syntax` and `no-restricted-globals`. Every bare `document.*` property access is banned in favour of `<targetWindow>.document.*`. Every bare `window`, `setInterval`, `clearInterval`, `requestAnimationFrame`, `cancelAnimationFrame`, `requestIdleCallback`, `cancelIdleCallback`, `addEventListener`, `removeEventListener`, `getComputedStyle`, `focus`, `blur`, `close`, `dispatchEvent`, `getSelection`, `matchMedia`, `open`, `parent`, `postMessage`, `devicePixelRatio`, `frames`, `frameElement`, `innerHeight`, `innerWidth`, `outerHeight`, `outerWidth`, `opener`, `origin`, `screen`, `screenLeft`, `screenTop`, `screenX`, `screenY`, `scrollX`, `scrollY`, `top`, `visualViewport` global is banned.
  - Lines 1428–1453: `electron-utility` layer: only `net` and `system-preferences` may be imported from `electron`.
  - Lines 1454–2176: `local/code-import-patterns` rule for `src/**/*.ts`. Defines conditional allowlists (`hasBrowser`, `hasNode`, `hasElectron`, `test`) and per-path target/restriction pairs for every major sub-tree of the source tree.
  - Lines 2178–2261: Import patterns for `test/**/*.ts` (smoke, sanity, automation, integration, monaco, mcp, componentFixtures).
  - Lines 2263–2292: `local/code-no-runtime-import` error and `local/code-limited-top-functions` error for notebook webview preloads renderer files.
  - Lines 2293–2320: Terminal contrib naming convention overrides.
  - Lines 2321–2343: Extension tree bans dynamic `require()` and dynamic `import()` calls to enable tree-shaking.
  - Lines 2412–2553: Copilot extension main rules block: bans all Node.js built-in modules (via spread of `builtinModules`), many npm packages, and applies `import/no-restricted-paths` zone rules that mirror the six-layer partition within the extension.
  - Lines 2812–2832: Test files are allowed to use `querySelector`/`querySelectorAll`; sinon stub cast pattern is warned.

---

- **Environment partitions:**

  The `local/code-layering` rule at lines 101–125 formally defines six runtime environments and their import lattice:

  | Layer | May import from |
  |---|---|
  | `common` | (nothing; base layer) |
  | `node` | `common` |
  | `browser` | `common` |
  | `electron-browser` | `common`, `browser` |
  | `electron-utility` | `common`, `node` |
  | `electron-main` | `common`, `node`, `electron-utility` |

  In `code-import-patterns` (lines 1476–1562), those six layers are referenced as conditions `hasBrowser` (browser, electron-browser), `hasNode` (node, electron-utility, electron-main), `hasElectron` (electron-utility, electron-main), and `test`.

  The `RawImportPatternsConfig` interface in `.eslint-plugin-local/code-import-patterns.ts` (line 20) enumerates the valid `layer` values: `common | worker | browser | electron-browser | node | electron-utility | electron-main`.

  A seventh, implicit `worker` environment appears in the `code-import-patterns` targets for `src/vs/editor/editor.worker.start.ts` at line 1696 and in `base/~` expansion comments (line 1587).

---

- **Architectural rules:**

  1. **Layer lattice (lines 101–125):** No file in a lower layer may import from a higher layer. This is enforced by the `local/code-layering` rule which walks the directory path components to detect layer names.
  2. **Per-path import restrictions (lines 1600–2175):** The `local/code-import-patterns` rule defines a closed allowlist of importable paths for every major source subtree. Each named subtree (`vs/base`, `vs/platform`, `vs/editor`, `vs/workbench`, `vs/code`, `vs/server`, `vs/sessions`) has a matching restriction entry. Any `src/**/*.ts` file not matched by any pattern produces `badFilename` error, ensuring coverage completeness.
  3. **Workbench cannot import from `workbench/contrib` (line 1719–1733):** `vs/workbench/~` may import `vs/workbench/services/*/~` and `vs/workbench/~` but not `vs/workbench/contrib/*/~` (except in test context).
  4. **No static heavy node_module imports on electron-main startup path (lines 1051–1077):** `local/code-no-static-node-module-import` is an `error` for `electron-main` and top-level `node` code, requiring dynamic `await import(...)` instead.
  5. **Multi-window document/window API discipline (lines 1078–1426):** Every direct `document.*` and every bare `window`-scoped global in browser/electron-browser code must be replaced with a `<targetWindow>`-qualified call through helpers in `DOM.ts`.
  6. **No bare `path` built-in module in `hasNode` layers (line 1523 comment):** The `hasNode` allow-list at lines 1491–1551 explicitly excludes `path`; a comment says "use `src/vs/base/common/path.ts` instead."
  7. **No HTTP-sourced imports (line 99):** `local/code-no-http-import` is warned for `src/vs/**`.
  8. **No deep imports of `*Internal` APIs (line 100):** `local/code-no-deep-import-of-internal` is an `error` globally.
  9. **electron-utility restricted to two electron APIs (lines 1437–1452):** Only `net` and `system-preferences` from `electron` are allowed in utility process code.
  10. **dompurify banned (line 1469–1474):** All `src/**/*.ts` must use `domSanitize` wrapper instead of `dompurify` directly.
  11. **`electron` import allowed only in `hasElectron` layers (lines 1554–1562):** The `electron` package is in the allowlist only when `when: 'hasElectron'`.

---

- **Banned patterns:**

  - `local/code-no-nls-in-standalone-editor` (line 84): NLS strings banned in standalone editor context.
  - `local/code-no-standalone-editor` (line 88): standalone editor APIs banned broadly.
  - `local/code-no-unexternalized-strings` (line 89): user-visible string literals must be externalized through NLS.
  - `local/code-no-localization-template-literals` (line 97): error-level ban on template literals inside NLS calls.
  - `local/code-no-http-import` (line 99): HTTP imports banned in `src/vs/**`.
  - `local/code-no-in-operator` (lines 336–338): `in` operator banned across most TypeScript sources (with exceptions).
  - `no-restricted-globals: __dirname, __filename, require` (lines 1044–1047): CommonJS globals banned in `node/electron` layers under ESM.
  - `no-restricted-syntax` with `querySelector`, `querySelectorAll`, `getElementById`, `getElementsByClassName`, `getElementsByTagName`, `getElementsByName`, `getElementsByTagNameNS` (lines 1238–1264): banned in `browser/electron-browser` layer — use `dom.ts h()` builder instead.
  - `no-restricted-syntax` with `new Intl.*` (lines 1093–1096): use `safeIntl` helper instead.
  - `no-restricted-syntax` with `instanceof MouseEvent/KeyboardEvent/PointerEvent/DragEvent/HTML*/SVG*` (lines 1097–1120): use `DOM.is*()` multi-window helpers instead.
  - `no-restricted-imports: dompurify*` (line 1469–1474): use `domSanitize`.
  - Extensions: dynamic `require()` and `import()` calls banned to enable tree-shaking (lines 2321–2343).
  - Copilot extension: all `builtinModules` and a list of npm packages banned in `src/**/common/**` and `src/**/vscode/**` layers (lines 2433–2447); `import/no-restricted-paths` zone rules mirror the six-layer partition (lines 2448–2528).
  - `local/code-no-telemetry-common-property` (lines 182–186): disallow common telemetry property names in event data objects.

---

- **Custom plugin rules referenced:**

  All from `./.eslint-plugin-local/`:

  - `local/code-translation-remind` — reminds to translate strings
  - `local/code-no-declare-const-enum` — bans `const enum` declarations
  - `local/code-parameter-properties-must-have-explicit-accessibility` — accessibility modifiers required
  - `local/code-no-nls-in-standalone-editor` — NLS ban in standalone context
  - `local/code-no-potentially-unsafe-disposables` — disposable safety
  - `local/code-no-dangerous-type-assertions` — type assertion safety
  - `local/code-no-any-casts` — bans `as any`
  - `local/code-no-standalone-editor` — standalone editor API ban
  - `local/code-no-unexternalized-strings` — NLS externalization requirement
  - `local/code-must-use-super-dispose` — `super.dispose()` call enforcement
  - `local/code-declare-service-brand` — service brand declaration requirement
  - `local/code-no-reader-after-await` — reader-after-await safety
  - `local/code-no-accessor-after-await` — accessor-after-await safety
  - `local/code-no-observable-get-in-reactive-context` — observable reactivity safety
  - `local/code-no-localized-model-description` — localized model description ban
  - `local/code-policy-localization-key-match` — policy localization key correctness
  - `local/code-no-localization-template-literals` — error-level template literal ban in NLS
  - `local/code-no-icons-in-localized-strings` — icon characters in NLS strings ban
  - `local/code-no-http-import` — HTTP import ban
  - `local/code-no-deep-import-of-internal` — internal API deep import ban
  - `local/code-layering` — layer access control rule
  - `local/code-no-telemetry-common-property` — telemetry property ban
  - `local/code-no-in-operator` — `in` operator ban
  - `local/code-no-unused-expressions` — unused expression ban
  - `local/code-no-static-self-ref` — static self-reference ban
  - `local/code-amd-node-module` — AMD/node module interop signal for common/browser
  - `local/code-no-static-node-module-import` — startup-path static import ban
  - `local/code-no-global-document-listener` — global document listener ban
  - `local/code-import-patterns` — per-path import allowlist enforcement
  - `local/code-no-runtime-import` — runtime import ban for preload scripts
  - `local/code-limited-top-functions` — top-level function count limit for preloads
  - `local/code-no-test-only` — `.only` test ban (error in main code)
  - `local/code-no-test-async-suite` — async suite ban
  - `local/code-must-use-result` — force-await specific async functions
  - `local/code-ensure-no-disposables-leak-in-test` — disposable leak detector
  - `local/code-no-static-self-ref` — static self-reference ban (TS)
  - `local/vscode-dts-create-func` — API shape
  - `local/vscode-dts-literal-or-types` — API shape
  - `local/vscode-dts-string-type-literals` — API shape
  - `local/vscode-dts-interface-naming` — API shape
  - `local/vscode-dts-cancellation` — API shape
  - `local/vscode-dts-use-export` — API shape
  - `local/vscode-dts-use-thenable` — API shape
  - `local/vscode-dts-vscode-in-comments` — API shape
  - `local/vscode-dts-provider-naming` — API shape
  - `local/vscode-dts-event-naming` — API shape

  From `./extensions/copilot/.eslintplugin/`:
  - `copilot-local/no-instanceof-uri`
  - `copilot-local/no-test-imports`
  - `copilot-local/no-runtime-import`
  - `copilot-local/no-funny-filename`
  - `copilot-local/no-bad-gdpr-comment`
  - `copilot-local/no-gdpr-event-name-mismatch`
  - `copilot-local/no-unlayered-files`
  - `copilot-local/no-restricted-copilot-pr-string`
  - `copilot-local/no-nls-localize`
  - `copilot-local/no-missing-linebreak`
  - `copilot-local/no-test-only`

---

### Cross-Cutting Synthesis

The eslint.config.js is the definitive machine-readable specification of VS Code's architectural boundaries. It encodes a six-layer execution-environment lattice (`common → browser/node → electron-browser/electron-utility → electron-main`) where each layer has a strictly defined set of permitted upstream imports. The `local/code-import-patterns` rule enforces a closed-world assumption: every `src/**/*.ts` file must match a declared target pattern or it is flagged. The `hasElectron` conditional is the only path by which the `electron` package itself enters the allow-list; all other layers are barred from it. For a Tauri port, this means every file currently carrying `electron` imports or residing in `electron-main`, `electron-utility`, or `electron-browser` directories represents a porting boundary requiring substitution with Tauri/Rust IPC equivalents. The browser-layer DOM rules (lines 1078–1426) reveal that VS Code has already abstracted direct `document`/`window` global access through window-context helpers — a partial structural prerequisite for running in a different renderer host. The `code-no-static-node-module-import` error rule on the electron-main startup path (lines 1051–1077) shows awareness of startup-time costs; Tauri's equivalent would be Rust initialization cost. The `path` module exclusion from the Node allow-list (line 1523) and the requirement to use `src/vs/base/common/path.ts` instead is an example of the pattern where platform-specific primitives are already abstracted, easing cross-platform porting.

---

### Out-of-Partition References

- `/home/norinlavaee/projects/vscode-atomic/.eslint-plugin-local/index.ts` — plugin entry point, auto-loads all `*.ts` rule files in the directory
- `/home/norinlavaee/projects/vscode-atomic/.eslint-plugin-local/code-layering.ts` — implements the six-layer lattice check
- `/home/norinlavaee/projects/vscode-atomic/.eslint-plugin-local/code-import-patterns.ts` — implements per-path import allowlist with `hasBrowser`/`hasNode`/`hasElectron`/`test` conditions and `~` layer template expansion
- `/home/norinlavaee/projects/vscode-atomic/.eslint-plugin-local/code-no-static-node-module-import.ts` — implements startup-path static import ban
- `/home/norinlavaee/projects/vscode-atomic/.eslint-plugin-local/utils.ts` — shared `createImportRuleListener` utility used by layering and import-patterns rules
- `/home/norinlavaee/projects/vscode-atomic/extensions/copilot/.eslintplugin/index.ts` — copilot-local plugin, referenced at line 12 of eslint.config.js
- `/home/norinlavaee/projects/vscode-atomic/.eslint-ignore` — ignore patterns read dynamically at line 19
