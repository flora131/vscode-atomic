(no external research applicable)

The `src/vscode-dts/` partition contains only pure TypeScript declaration files (`.d.ts`) that define the VS Code public extension API surface. These files have no third-party library imports, no runtime dependencies, and no external framework bindings — they are self-contained type contracts. Therefore, no external library documentation is central to researching what a Tauri/Rust port would require from this partition.

For the purpose of a Tauri/Rust port analysis, the significance of `src/vscode-dts/` is that it represents the stable API contract that all VS Code extensions depend on. Any port would need to either re-implement this entire API surface in the new host environment or provide a compatibility shim layer. The `.d.ts` files themselves are purely declarative TypeScript with no external dependencies to research via online documentation sources.
