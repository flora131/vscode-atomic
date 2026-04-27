# Online Research: scripts/ — Porting to Tauri/Rust

(no external research applicable)

The `scripts/` partition consists of shell wrappers and Node.js helper scripts whose only external dependencies are Electron CLI conventions (`ELECTRON_RUN_AS_NODE`), test-harness packages (`@vscode/test-web`, `@vscode/vscode-perf`), and terminal-emulator modules (`xterm`). None of these are load-bearing for the Tauri/Rust porting effort: the runtime and IPC architecture that must be redesigned lives entirely in the `src/` partitions, and the scripts themselves will simply be replaced by equivalent Tauri/Cargo build and test invocations. Consulting external documentation for Electron flags or `xterm` bindings would not inform any architectural decision relevant to the port.
