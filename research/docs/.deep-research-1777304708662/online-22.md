# Online Research: notebook-renderers External Dependencies

(no external research applicable)

## Justification

The `extensions/notebook-renderers/` extension has **no `dependencies` field** in its `package.json` — the `dependencies` array is empty. All four entries under `devDependencies` are either test-only or type-only packages:

| Package | Kind | Used where |
|---|---|---|
| `jsdom` ^28.1.0 | Test-only runtime shim | `src/test/linkify.test.ts:7`, `src/test/notebookRenderer.test.ts:10` |
| `@types/jsdom` ^21.1.0 | TypeScript type definitions only | Compile-time, never bundled |
| `@types/node` ^22.18.10 | TypeScript type definitions only | Compile-time, never bundled |
| `@types/vscode-notebook-renderer` ^1.60.0 | TypeScript type definitions only | Compile-time, never bundled |

All runtime imports in the non-test source files (`src/ansi.ts`, `src/color.ts`, `src/colorMap.ts`, `src/htmlHelper.ts`, `src/index.ts`, `src/linkify.ts`, `src/rendererTypes.ts`, `src/stackTraceHelper.ts`, `src/textHelper.ts`) are exclusively internal relative-path imports or VS Code ambient type references (`vscode-notebook-renderer`, `vscode`). These are pure type-declaration packages resolved at compile time and not shipped as runtime dependencies.

Because there are no external runtime third-party libraries in this extension, no online documentation research is warranted. The extension's Tauri/Rust porting concerns (webview message passing, CSP, TrustedTypes policy, custom protocol handlers) derive from browser platform APIs and VS Code's own renderer API contract — both of which are in-scope for the broader porting effort but are not tied to any third-party npm library requiring external documentation lookup here.
