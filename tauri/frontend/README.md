# vscode-tauri-frontend

Vite + TypeScript shell that loads Monaco editor and provides the skeleton
workbench DOM for the Tauri desktop target.

## Prerequisites

- Node.js ≥ 18
- Rust toolchain + `tauri-cli` (for full `tauri dev` workflow)

## Dev

```bash
cd tauri/frontend
npm install
npm run dev          # starts Vite on port 1420
```

Open `http://localhost:1420` in a browser for hot-reload iteration,
**or** run `tauri dev` from the repo root so Tauri spawns the Vite server
automatically and injects the IPC bridge.

## Build

```bash
npm run build        # type-checks then bundles to dist/
```

## Type checking only

```bash
npm run typecheck    # tsc --noEmit
```

## Architecture

| File | Purpose |
|---|---|
| `src/main.ts` | Entry point — mounts Monaco, registers key bindings, calls `workbench_ready` |
| `src/bridge.ts` | Typed wrappers around `@tauri-apps/api` invoke + event.listen |
| `src/types.ts` | TS interfaces mirroring Rust structs in `tauri/vscode-base` |
| `vite.config.ts` | Vite config tuned for Tauri (port 1420, TAURI_ env prefix) |

## IPC Commands (stubs)

The following Tauri commands are called from the front-end and must be
registered on the Rust side before they become functional:

| Command | Request | Response |
|---|---|---|
| `workbench_ready` | `{ version: string }` | `void` |
| `open_folder` | `{ uri: Uri }` | `void` |
| `read_file` | `{ uri: Uri }` | `{ content: string }` |
| `write_file` | `{ uri: Uri, content: string }` | `void` |
