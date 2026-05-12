# Partition 27: extensions/npm/ — Library Analysis for Rust/Tauri Port

## Finding

(no external research applicable)

All dependencies in `extensions/npm/` are thin Node.js utility libraries that perform functions trivially replicated in a Rust/Tauri world using the standard library, crates, or simple inline logic. None of these libraries represent a porting blocker or require deep research into external documentation. The analysis below is based entirely on reading `extensions/npm/package.json` and the extension source files in-tree.

---

## Libraries in `extensions/npm/package.json`

### 1. jsonc-parser `^3.2.0`

- **NPM**: https://www.npmjs.com/package/jsonc-parser
- **GitHub**: https://github.com/microsoft/node-jsonc-parser
- **Used in**:
  - `extensions/npm/src/readScripts.ts:6` — `import { JSONVisitor, visit } from 'jsonc-parser';`
  - `extensions/npm/src/features/jsonContributions.ts:6` — `import { Location, getLocation, createScanner, SyntaxKind, ScanError, JSONScanner } from 'jsonc-parser';`
  - `extensions/npm/src/features/packageJSONContribution.ts:9` — `import { Location } from 'jsonc-parser';`
- **What it does**: Parses JSON with Comments (JSONC). The `visit()` streaming visitor is used in `readScripts.ts` to walk `package.json` and extract the `scripts` block without fully deserializing the document. `createScanner` / `getLocation` provide cursor-aware scanning to support completion/hover in `.json` files.
- **Porting note**: In a Rust/Tauri environment this extension either does not exist (the npm integration is a UI-layer extension, not core IDE infrastructure) or its logic is replaced by a Rust JSONC crate such as `jsonc-parser` (Rust). The streaming-visitor pattern is straightforward to replicate. Not a porting concern.

### 2. which-pm `^2.1.1`

- **NPM**: https://www.npmjs.com/package/which-pm
- **GitHub**: https://github.com/pnpm/which-pm
- **Used in**:
  - `extensions/npm/src/preferred-pm.ts:9` — `import whichPM from 'which-pm';`
  - `extensions/npm/src/preferred-pm.ts:99` — `const pmUsedForInstallation: { name: string } | null = await whichPM(pkgPath);`
- **What it does**: Reads the `engines.npm` / `packageManager` field in `package.json` (and inspects lock-file presence) to infer which package manager (npm, yarn, pnpm, bun) was used to install dependencies. Returns `{ name: string }`.
- **Porting note**: The same detection could be done with 10–15 lines of Rust (parse JSON, check field). Not a porting concern.

### 3. find-up `^5.0.0`

- **NPM**: https://www.npmjs.com/package/find-up
- **GitHub**: https://github.com/sindresorhus/find-up
- **Used in**:
  - `extensions/npm/src/preferred-pm.ts:7` — `import findUp from 'find-up';`
  - `extensions/npm/src/preferred-pm.ts:45` — `if (await findUp('pnpm-lock.yaml', { cwd: pkgPath }))` — walks up the directory tree looking for `pnpm-lock.yaml`.
- **What it does**: Walks ancestor directories until a file matching the given name is found. Purely filesystem traversal.
- **Porting note**: Trivially implemented in Rust with a loop over `Path::parent()`. Not a porting concern.

### 4. find-yarn-workspace-root `^2.0.0`

- **NPM**: https://www.npmjs.com/package/find-yarn-workspace-root
- **GitHub**: https://github.com/square/find-yarn-workspace-root
- **Used in**:
  - `extensions/npm/src/preferred-pm.ts:6` — `import findWorkspaceRoot = require('../node_modules/find-yarn-workspace-root');`
  - `extensions/npm/src/preferred-pm.ts:58` — called to detect whether the project is inside a Yarn workspace.
- **What it does**: Walks up directories looking for a `package.json` containing a `workspaces` field, indicating a Yarn workspace root.
- **Porting note**: Equivalent to a few lines of Rust filesystem + JSON parsing. Not a porting concern.

### 5. minimatch `^5.1.8`

- **NPM**: https://www.npmjs.com/package/minimatch
- **GitHub**: https://github.com/isaacs/minimatch
- **Used in**:
  - `extensions/npm/src/tasks.ts:13` — `import minimatch from 'minimatch';`
  - `extensions/npm/src/tasks.ts:247` — `return minimatch(path, pattern, { dot: true });` — used to test workspace folder paths against user-configured `npm.exclude` glob patterns.
- **What it does**: Glob pattern matching (`*.json`, `**/node_modules/**`, etc.).
- **Porting note**: The `glob` crate in Rust covers this entirely. Not a porting concern.

### 6. request-light `^0.7.0`

- **NPM**: https://www.npmjs.com/package/request-light
- **GitHub**: https://github.com/microsoft/node-request-light
- **Used in**:
  - `extensions/npm/src/npmMain.ts:6` and `:35`, `:169` — configured with VS Code proxy settings and passed as an XHR function to JSON contribution providers.
  - `extensions/npm/src/npmBrowserMain.ts:6`, `:11` — same, browser variant.
  - `extensions/npm/src/features/jsonContributions.ts:8` — type `XHRRequest` accepted as a parameter.
  - `extensions/npm/src/features/packageJSONContribution.ts:8` — type `XHRRequest`, used to fetch package metadata from the npm registry for auto-complete.
- **What it does**: A lightweight XHR/HTTP client with proxy support, authored by Microsoft for use in VS Code extensions. Used here to fetch online package information from the npm registry (completions, version hover). Controlled by the `npm.fetchOnlinePackageInfo` setting.
- **Porting note**: In a Rust/Tauri port the HTTP call would be made with `reqwest` or `ureq`. The functionality is a minor convenience feature (online package version completions). Not a porting concern.

### 7. which `^4.0.0`

- **NPM**: https://www.npmjs.com/package/which
- **GitHub**: https://github.com/isaacs/node-which
- **Used in**:
  - `extensions/npm/src/npmMain.ts:14` — `import which from 'which';`
  - Called via `getNPMCommandPath()` to locate the `npm` binary on `PATH`.
- **What it does**: Cross-platform equivalent of the Unix `which` command — resolves a binary name to its full path by searching `PATH`.
- **Porting note**: The `which` crate in Rust (`which = "4"`) is a direct equivalent. Or: `std::process::Command` with `which npm`. Not a porting concern.

### 8. vscode-uri `^3.0.8`

- **NPM**: https://www.npmjs.com/package/vscode-uri
- **GitHub**: https://github.com/microsoft/vscode-uri
- **Used in**:
  - `extensions/npm/src/tasks.ts:14` — `import { Utils } from 'vscode-uri';`
  - `extensions/npm/src/tasks.ts:174,194,212` — `Utils.basename(folder.uri)` — extracts the last path segment from a VS Code `Uri`.
- **What it does**: Pure utility helpers for VS Code `Uri` objects (basename, dirname, joinPath, etc.). Ships the same URI manipulation code used inside VS Code itself.
- **Porting note**: `Path::file_name()` in Rust's standard library covers `basename`. Not a porting concern.

---

## Summary

The `extensions/npm/` partition is a self-contained VS Code extension that provides npm/yarn/pnpm/bun task running, script exploration, and package.json IntelliSense (completions, hover for version info). Its eight runtime dependencies are all shallow utility libraries:

- Two are Microsoft-authored VS Code ecosystem helpers (`jsonc-parser`, `request-light`, `vscode-uri`) with no complex behavior.
- The rest are single-purpose Unix utilities (`which`, `find-up`, `find-yarn-workspace-root`, `which-pm`) or a glob matcher (`minimatch`).

None of these libraries are architectural dependencies of VS Code's core IDE — they exclusively serve this one optional extension. In a Rust/Tauri port, this extension either would not exist (task running is orchestrated differently), or each library would be replaced by a handful of lines of Rust using standard-library primitives and well-known crates (`glob`, `which`, `serde_json`/`jsonc-parser`). No library here requires external research, API deep-dives, or presents any meaningful porting complexity.
