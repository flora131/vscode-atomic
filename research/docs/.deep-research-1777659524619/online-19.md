# Partition 19: `.eslint-plugin-local/` — Custom ESLint Rules

## Decision

(no external research applicable)

The `.eslint-plugin-local/` directory contains 47 TypeScript source files implementing custom ESLint rules (plus `utils.ts`, `index.ts`, `tsconfig.json`, `package.json`, and two test files), all of which operate exclusively at development/CI time to enforce VS Code's coding conventions via static analysis. The ESLint rule authoring API (`eslint.Rule.RuleModule`, `@typescript-eslint/utils`) and the TypeScript AST node types these rules traverse are dev-toolchain concerns — they have no runtime presence, produce no artifacts that ship to users, and carry zero weight in a Tauri/Rust porting decision.

What *does* matter for porting is the set of architectural invariants these rules are designed to guard, not the linting machinery itself. The rules document several VS Code-specific design constraints that a port must preserve or deliberately redesign:

- **`code-layering.ts`** and **`code-import-patterns.ts`** enforce the `common → browser/node/electron-browser` layering hierarchy; any Rust/Tauri equivalent must respect an analogous layer boundary or consciously collapse it.
- **`code-no-potentially-unsafe-disposables.ts`** and **`code-must-use-super-dispose.ts`** guard the `IDisposable` / `DisposableStore` lifetime model — a pattern that would need a Rust-native equivalent (e.g., RAII `Drop` impls or an explicit arena).
- **`code-must-use-result.ts`** enforces that certain APIs return a `Result`-like type and callers handle it — directly analogous to Rust's `Result<T, E>`.
- **`code-no-global-document-listener.ts`**, **`code-no-runtime-import.ts`**, and **`code-no-static-node-module-import.ts`** capture environment-boundary assumptions (browser DOM, Node.js module system) that would simply not exist in a Tauri/Rust context.
- **`code-no-unexternalized-strings.ts`**, **`code-no-localization-template-literals.ts`**, and the `vscode-dts-*` family guard the public extension API surface (`vscode.d.ts`) conventions, which are orthogonal to the Tauri port question.

In summary: ignore the ESLint/typescript-eslint plugin API entirely. The rules are a machine-readable specification of VS Code's architectural constraints; consult the rule *logic* as a cross-check when mapping VS Code subsystems to their Tauri/Rust replacements, but fetch no external docs for the linting framework itself.
