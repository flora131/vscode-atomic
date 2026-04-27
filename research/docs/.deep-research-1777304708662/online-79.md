# Partition 79 of 79 — Build Orchestration: `gulpfile.mjs` + `build/gulpfile.ts`

(no external research applicable)

## Justification

The scope for this partition is the VS Code build-time orchestration layer: the 5-line root `gulpfile.mjs` (which is solely `import './build/gulpfile.ts'`) and the `build/gulpfile.ts` entrypoint together with the family of sibling gulpfiles it pulls in (`gulpfile.vscode.ts`, `gulpfile.compile.ts`, `gulpfile.extensions.ts`, `gulpfile.editor.ts`, `gulpfile.vscode.linux.ts`, `gulpfile.vscode.win32.ts`, `gulpfile.reh.ts`, `gulpfile.vscode.web.ts`, `gulpfile.scan.ts`, `gulpfile.hygiene.ts`, `gulpfile.cli.ts`).

The task method explicitly states the answer: "Build-time orchestration is replaced wholesale in any Tauri port (cargo + new bundler)." This is the correct conclusion, and it makes external documentation on Gulp, esbuild, or the VS Code TypeScript compilation pipeline non-central for the following reasons:

1. **One-for-one replacement, not migration.** Porting VS Code to Tauri/Rust does not involve adapting or extending the Gulp/esbuild pipeline. The entire pipeline — task sequencing (gulp, `build/lib/task.ts`), TypeScript compilation (tsc/SWC/esbuild transpile steps), asset copying (vinyl-fs), Electron packaging (`@vscode/gulp-electron`), ASAR creation, NLS bundling, extension compilation — is discarded in its entirety. Cargo becomes the top-level build driver; `tauri build` and `cargo build` replace `gulp compile` and the downstream `compile-build-*` tasks.

2. **No transferable API surface.** The build system in scope is pure Node.js tooling (Gulp 5, esbuild, SWC, vinyl streams, minimist). None of these APIs or their documentation are relevant to a Rust/Cargo/Tauri build graph. The Tauri build system uses `tauri.conf.json`, `Cargo.toml`, and optional `build.rs` scripts — a completely different vocabulary.

3. **The five-line root file is a stub.** `gulpfile.mjs` contains only a single `import` re-export. Its only design decision (ESM over CJS) is irrelevant to a Rust port.

4. **Conceptual mapping, not documentation lookup.** The only intellectually interesting question for a Tauri porter reading this scope is "what responsibilities does the Gulp pipeline own that I must now assign to Cargo / tauri-cli / a custom build script?" That question is answered by reading the source files themselves (already done above), not by fetching external Gulp or esbuild documentation. The responsibilities are: TypeScript transpilation (gone — replaced by Rust source; any remaining TS for the webview layer would use Vite or plain tsc directly), extension bundling (replaced by a tauri `beforeBuildCommand` or a dedicated bundler invocation), asset packaging (replaced by Tauri's `resources` and `bundle` configuration), and platform-specific packaging (replaced by `tauri build --target`).

5. **No ambiguity requiring external resolution.** The build files are self-contained. The gulp tasks, their dependencies, and the tools they invoke are all legible from the source without consulting upstream documentation. There is no version-specific behavior, deprecated API, or undocumented edge case that would benefit from fetching Gulp or esbuild docs.

## Summary

The `gulpfile.mjs` / `build/gulpfile.ts` build orchestration layer examined in this partition is the VS Code-specific Gulp pipeline that drives TypeScript compilation, extension bundling, asset packaging, and Electron distribution. For a Tauri/Rust port this entire subsystem is replaced wholesale: Cargo and `tauri-cli` assume the build-driver role, TypeScript source largely disappears in favor of Rust, any residual web-layer TypeScript migrates to a modern bundler like Vite invoked via tauri's `beforeBuildCommand`, and platform-specific packaging is handled by Tauri's built-in bundle targets. Because the replacement is a clean substitution rather than a migration or adaptation, no external documentation on Gulp, esbuild, SWC, vinyl-fs, or the existing VS Code build library (`build/lib/`) is central to understanding what the port requires. The mapping is conceptually straightforward — every gulp task category has a Tauri/Cargo analogue — and that mapping is derivable entirely from reading the source files themselves.
