# Bootstrap Server Pattern Analysis

## Pattern 1: Electron Node.js Environment Isolation

**Where:** `src/bootstrap-server.ts:7`
**What:** Prevents interference from Electron's Node.js mode during server bootstrap. The bootstrap-server module must execute before any other imports to ensure clean process environment state.

```typescript
// Keep bootstrap-esm.js from redefining 'fs'.
delete process.env['ELECTRON_RUN_AS_NODE'];
```

**Variations / call-sites:**
- Referenced as the **first import** in both `src/server-main.ts:6` and `src/server-cli.ts:6`
- Comment explicitly states: "this MUST come before other imports as it changes global state"
- Critical for node module resolution in server contexts (see `src/server-main.ts:14-21` and `src/server-cli.ts:238-245` for related module lookup path setup)

**Implications for Tauri/Rust port:**
The bootstrap-server pattern demonstrates that VS Code's Node.js server startup requires early environment cleanup to prevent Electron-specific configurations from contaminating the server process. A Rust port would not face this exact issue (no Electron context), but the architectural principle—that global initialization must happen before module loading—remains critical. The Rust entrypoint must complete all environment setup (signal handlers, module paths, NLS configuration) before initializing the core server logic.

Both TypeScript servers (`server-main.ts` and `server-cli.ts`) follow an identical pattern after the bootstrap import: (1) environment cleanup, (2) NLS config resolution, (3) ESM bootstrap, (4) deferred server initialization. This sequential initialization chain should mirror in Rust via a clear main() → setup_environment() → initialize_nls() → run_server() flow.
