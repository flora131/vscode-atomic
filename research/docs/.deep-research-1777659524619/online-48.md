(no external research applicable)

The `extensions/grunt/` scope (2 files, 382 LOC) has zero npm production dependencies (`"dependencies": {}` in `package.json`). Its sole TypeScript source file (`src/main.ts`) imports only Node.js built-in modules (`path`, `fs`, `child_process`) and the ambient `vscode` extension API; no third-party libraries are involved, so there is nothing externally researchable that is central to porting this extension from a TS/Electron host to a Tauri/Rust host.
