# Bootstrap Server Location and Metadata

## Implementation

- `src/bootstrap-server.ts` — Remote-server bootstrap shim (444 bytes, 7 lines of code)
  - Purpose: A minimal initialization module that clears the `ELECTRON_RUN_AS_NODE` environment variable before other modules are loaded
  - Used as a first import in server startup paths to modify global state
  - Related bootstrap files in src/:
    - `src/bootstrap-node.ts` — Node.js environment setup
    - `src/bootstrap-fork.ts` — Fork process initialization
    - `src/bootstrap-cli.ts` — CLI bootstrap
    - `src/bootstrap-import.ts` — Import system setup
    - `src/bootstrap-esm.ts` — ESM module bootstrapping
    - `src/bootstrap-meta.ts` — Metadata initialization

## Entry Points / Usage

- `src/server-cli.ts` (line 6) — Imports bootstrap-server.js as first import with comment "this MUST come before other imports as it changes global state"
- `src/server-main.ts` (line 6) — Imports bootstrap-server.js as first import with same requirement comment

## Overview

The `bootstrap-server.ts` file is a minimal 7-line TypeScript shim located at the repository root of `src/`. It serves as a required initialization module for remote-server startup, ensuring environment variables are properly cleaned before subsequent module loading. The file is part of a broader bootstrap system in VS Code that includes six related bootstrap modules, all with similar initialization purposes. No dedicated tests or configuration files reference this module specifically; instead, it is implicitly tested through server-cli and server-main integration paths.
