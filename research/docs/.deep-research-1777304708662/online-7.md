# Partition 7: `extensions/git/` — Tauri Shell Plugin Research

## Scope

This document covers `extensions/git/src/git.ts` (Node `child_process` usage), the two Tauri port artifacts that already exist in `extensions/git/out/` (`tauri-shell.js` and `git/tauriProcessLauncher.js`), and focused excerpts from the Tauri 2.x shell plugin documentation needed to complete and validate those artifacts.

---

#### Tauri Shell Plugin (v2.x)

**Docs:**
- https://v2.tauri.app/plugin/shell/ (overview, permissions table)
- https://v2.tauri.app/reference/javascript/shell/ (full JS API reference)
- https://raw.githubusercontent.com/tauri-apps/plugins-workspace/v2/plugins/shell/guest-js/index.ts (authoritative TypeScript source)
- https://v2.tauri.app/develop/sidecar/ (sidecar embedding guide)

**Relevant behaviour:**

### `Command.create(program, args?, options?)` — the Node `cp.spawn` analogue

Signature (from `guest-js/index.ts`, v2 branch):

```typescript
static create<O extends IOPayload>(
  program: string,
  args: string | string[] = [],
  options?: SpawnOptions
): Command<O>
```

- `program` is a **logical name** (scope identifier), not the raw filesystem path.
  Every name must be declared in `src-tauri/capabilities/*.json` under `shell:allow-execute`.
  The actual OS binary path is bound in the capability's `allow[].cmd` field.
- `SpawnOptions` fields: `cwd?: string`, `env?: Record<string,string>`, `encoding?: string`.
  `env` set to `null` clears the inherited process environment (important for VS Code's git
  env-injection pattern at `git.ts:689–695`).
- Returns a `Command<string>` (or `Command<Uint8Array>` when `encoding: 'raw'` is passed).
  The object is an `EventEmitter` subclass; no I/O starts until `.spawn()` or `.execute()`.

### `Command.sidecar(program, args?, options?)` — bundled binary variant

```typescript
static sidecar<O extends IOPayload>(
  program: string,
  args: string | string[] = [],
  options?: SpawnOptions
): Command<O>
```

- Identical signature to `create()`. Internally sets `options.sidecar = true` before
  passing the payload to `plugin:shell|spawn`.
- The `program` string must match the value in `tauri.conf.json > bundle > externalBin`
  (e.g. `"binaries/my-sidecar"`). Tauri resolves the arch-suffixed binary
  (`my-sidecar-aarch64-apple-darwin`) at runtime; the caller never manages path suffixes.
- **Not used by the current git port** (git is a system binary, not a bundled sidecar).
  `sidecar()` is relevant if VS Code ships its own git wrapper or the `askpass` helper
  as a compiled Rust sidecar rather than a shell script.

### `Command.spawn()` — streaming / long-lived processes

```typescript
async spawn(): Promise<Child>
```

- Calls `invoke('plugin:shell|spawn', { program, args, options, onEvent })` over IPC.
- Returns `Promise<Child>` where `Child.pid: number` identifies the process.
- **Streaming events** arrive via the `onEvent` Channel:
  - `'Stdout'` payload → forwarded to `command.stdout.emit('data', payload)`
  - `'Stderr'` payload → forwarded to `command.stderr.emit('data', payload)`
  - `'Terminated'` payload `{ code: number|null, signal: number|null }` → forwarded to
    `command.emit('close', payload)`
  - `'Error'` payload → forwarded to `command.emit('error', payload)`
- The stdout/stderr are **line-buffered strings by default** (one `data` event per line),
  unlike Node streams which emit raw `Buffer` chunks. The Tauri port adapters in `out/`
  compensate by appending `'\n'` to each emitted line (see `tauri-shell.js:155–158`).

### `Command.execute()` — fire-and-forget / short-lived processes

```typescript
async execute(): Promise<ChildProcess<O>>
```

- Calls `invoke('plugin:shell|execute', { program, args, options })`.
- Collects all output before resolving; returns `{ code, signal, stdout, stderr }`.
- Analogous to `cp.exec` / `cp.execFile`. Used in `tauri-shell.js:tauriExec()`.

### `Child.write(data)` — stdin writes

```typescript
async write(data: IOPayload | number[]): Promise<void>
// IOPayload = string | Uint8Array
```

- Calls `invoke('plugin:shell|stdin_write', { pid, buffer: data })`.
- Must hold the `Child` reference returned by `spawn()`.
- In the existing Node code (`git.ts:624`), stdin writes happen via
  `child.stdin!.end(options.input, 'utf8')`. Under Tauri, `Child` has no `.stdin` stream;
  writes are explicit async calls. The current `tauriProcessLauncher.js` exposes `stdin: null`
  (`tauriProcessLauncher.js:89`), so callers relying on `options.input` must be adapted.

### `Child.kill()` — process termination

```typescript
async kill(): Promise<void>
```

- Calls `invoke('plugin:shell|kill', { cmd: 'killChild', pid: this.pid })`.
- Returns `Promise<void>`; resolution indicates the OS kill signal was sent, not that the
  process has exited. Callers should await the `'close'` event on the `Command` for
  confirmation, just as with Node's `child.kill()` + `'exit'` event pattern.
- Requires `shell:allow-kill` permission in the capability configuration.
- The Tauri adapters fire-and-forget: `this._tauriChild.kill().catch(() => {})` (see
  `tauriProcessLauncher.js:106` and `tauri-shell.js:143`).

### Permissions model (critical difference from Node)

Every program that may be executed must be pre-declared in a capabilities JSON:

```json
{
  "permissions": [
    {
      "identifier": "shell:allow-execute",
      "allow": [
        {
          "name": "run-git",
          "cmd": "git",
          "args": true
        }
      ]
    },
    "shell:allow-spawn",
    "shell:allow-kill",
    "shell:allow-stdin-write"
  ]
}
```

- `name` is the logical identifier passed to `Command.create('run-git', ...)`.
- `args: true` allows any argument list; a restricted array can validate per-arg regexes.
- There is no runtime path resolution or `PATH` lookup from the frontend — the `cmd` value
  is resolved by the Rust backend under the security policy.
- The git extension uses many ad-hoc git subcommands with varying argument lists; the
  capability scope therefore needs `args: true` for the git command entry.

---

**Where used in `extensions/git/src/git.ts`:**

| Line(s) | Node API | What it does | Tauri equivalent |
|---------|----------|--------------|-----------------|
| `git.ts:9` | `import * as cp from 'child_process'` | Module import | Replace with `Command` from `@tauri-apps/plugin-shell` |
| `git.ts:87–91` | `cp.spawn(path, ['--version'])` | One-shot version probe; collects stdout buffers, resolves on `'close'` | `Command.create('git', ['--version']).execute()` → read `result.stdout` |
| `git.ts:96` | `cp.exec('which git', ...)` | Discover git path on macOS | `Command.create('which', ['git']).execute()` (or hard-code `cmd: 'which'` in scope) |
| `git.ts:109` | `cp.exec('git --version', ...)` | Confirm git is executable on macOS | `Command.create('git', ['--version']).execute()` |
| `git.ts:124` | `cp.exec('xcode-select -p', ...)` | Check for Xcode on macOS | `Command.create('xcode-select', ['-p']).execute()` |
| `git.ts:210–246` | `exec(child, cancellationToken)` | Collects stdout/stderr buffers from a live `cp.ChildProcess`; listens for `'data'`, `'close'`, `'error'` | For short-lived calls: replace with `Command.execute()`. For streaming: subscribe to `command.stdout.on('data', ...)` + `command.on('close', ...)` as implemented in `tauriProcessLauncher.js:144–161`. |
| `git.ts:249–260` | `child.kill()` | Cancellation token triggers `child.kill()` | `child.kill()` (Tauri `Child.kill()`) — already fire-and-forget; must still wait for `'close'` event to confirm exit |
| `git.ts:604–616` | `this.spawn(args, options)` returns `cp.ChildProcess` used as stream | `stream()` method wraps a live process for callers that consume stdout line-by-line | `Command.create(...).spawn()` returns `Child`; stdout lines arrive through `command.stdout.on('data', ...)` |
| `git.ts:676–702` | `cp.spawn(this.path, args, options)` | Core spawn used by `_exec`, `exec`, `stream` | Wrapped by `TauriGitProcessLauncher.spawn()` in `tauriProcessLauncher.js` |
| `git.ts:624` | `child.stdin!.end(options.input, 'utf8')` | Writes credential/commit-msg data to git stdin | `Child.write(string)` — async; `stdin` is `null` in the Tauri adapter; callers using `options.input` need an explicit `await child.write(input)` after `spawn()` |
| `git.ts:1395–1401` | `Repository.stream()` / `Repository.spawn()` | Delegates back to `Git.stream` / `Git.spawn` | Covered by the same `TauriGitProcessLauncher` path above |

---

**Artifacts already present (`extensions/git/out/`):**

`/Users/norinlavaee/vscode-atomic/extensions/git/out/tauri-shell.js`
- Exports `tauriExec(command, args, options)` — wraps `Command.create().execute()` for
  short-lived commands (analogous to `cp.exec`).
- Exports `tauriSpawn(command, args, options)` — wraps `Command.create().spawn()` for
  streaming processes; returns a duck-typed `IChildProcessLike` with `.stdout`, `.stderr`,
  `.stdin` (null), `.on()`, `.kill()`.

`/Users/norinlavaee/vscode-atomic/extensions/git/out/git/tauriProcessLauncher.js`
- Exports `TauriGitProcessLauncher` implementing an `IGitProcessLauncher` interface.
- `.spawn(command, args, options)` returns a `TauriGitChild` that duck-types `cp.ChildProcess`
  (`.stdout`, `.stderr`, `.stdin` null, `.on('exit'|'error')`, `.kill()`).
- `kill()` delegates to `this._tauriChild.kill().catch(() => {})` (fire-and-forget).
- **Stdin gap:** `TauriGitChild.stdin` is always `null`. Any git operation that pipes input
  (e.g. `git commit` with `options.input` for the commit message, or `git credential`)
  requires a separate `await child.write(data)` call via the underlying `_tauriChild` —
  which is not yet surfaced by the current adapter's public interface.

---

**Gaps / open issues:**

1. **Stdin write support** — `options.input` in `SpawnOptions` (git.ts:204) writes to
   `child.stdin` synchronously in Node. The Tauri adapter exposes `stdin: null`; any code
   path that reaches `git.ts:624` (`child.stdin!.end(options.input, 'utf8')`) will throw.
   Fix: surface `TauriGitChild.writeStdin(data)` and call it from `_exec` when
   `options.input` is set.

2. **Line-buffering** — Tauri shell emits one `data` event per line (no partial chunks).
   The adapters append `'\n'` to restore the line terminator. However, `git.ts:238`
   accumulates raw `Buffer` chunks from Node streams; the Tauri path delivers strings.
   The `bufferResult.stdout.toString('utf8')` coercion at `git.ts:657` would fail because
   there is no `Buffer` — downstream code must work with pre-concatenated strings.

3. **Path discovery on macOS/Windows** — `findGitDarwin` / `findGitWin32` call `cp.exec('which git')` and then validate the resolved path (git.ts:96–134). Under Tauri, executing `which` requires a separate scope entry. An alternative is to hard-code common locations (`/usr/bin/git`, `/usr/local/bin/git`) and use `Command.create().execute()` for each probe, matching the current `findSpecificGit` pattern.

4. **`onSpawn` callback** — `SpawnOptions.onSpawn?(child: cp.ChildProcess)` at git.ts:207/621 passes the raw `ChildProcess` to callers (e.g., `clone()` at git.ts:451 attaches a line stream to `child.stderr`). Under Tauri, the equivalent is attaching listeners to `command.stderr.on('data', ...)` before calling `spawn()`. The `TauriGitProcessLauncher` does not yet expose an `onSpawn` hook.

5. **Cancellation / `child.killed` flag** — git.ts:611 checks `child.killed` for the log message. `TauriGitChild._killed` tracks this internally but it is not yet exposed as a public `.killed` property.

---

**Summary**

The Tauri 2.x shell plugin (`@tauri-apps/plugin-shell`) provides a `Command` class whose `create()` / `sidecar()` factory methods, `spawn()` streaming path, `execute()` fire-and-forget path, `Child.write()` for stdin, and `Child.kill()` for termination cover every Node `child_process` usage found in `extensions/git/src/git.ts`. The two existing Tauri adapter files (`tauri-shell.js` and `tauriProcessLauncher.js`) already implement the core spawn-and-stream pattern correctly for the main `Git.spawn()` / `Git._exec()` hot path (git.ts:676–702 and 210–270). The five gaps listed above — stdin writes via `options.input`, line-buffering/Buffer vs string duality, path-discovery via `which`/`xcode-select`, `onSpawn` callback exposure, and the `killed` flag — are the concrete items that remain to be resolved before the git extension can run fully under Tauri without any Node `child_process` dependency.
