# ADR 0001: Monaco Renderer Strategy — Keep TypeScript in WebView, Host TextModel in Rust

**Status:** Accepted

**Date:** 2026-04-26

**Deciders:** flora131

---

## Context

VS Code's Monaco editor consists of two separable layers:

1. **Renderer** (`src/vs/editor/browser/`, ~1000 files, ~200 KLOC TypeScript): GPU-backed
   text layout, cursor, selections, minimap, gutter, scroll, virtual view. Tightly coupled
   to browser DOM and WebGL/Canvas APIs. Consumes ~60% of keystroke latency budget in
   Electron today.

2. **Text model** (`src/vs/editor/common/model/textModel.ts`, piece-tree buffer,
   decoration tracker, bracket-pair colorisation, undo stack): pure data structures with
   no DOM dependency. Exposed to language servers and extensions via `ITextModel`.

The research document (`research/docs/2026-04-27-research-what-it-would-take-to.md`,
"Monaco renderer" open question) identifies the renderer as a major blocker for a Rust
port: "Replacing it would invalidate decades of language-extension provider integration."

The Tauri migration is being pursued incrementally. A big-bang renderer port to Rust
(e.g., rewriting layout with wgpu or a native text-rendering crate) would:

- Block all other Tauri work for 12–18 months.
- Break 50K+ extension ecosystem that extends Monaco via the provider API
  (`src/vs/editor/common/languageFeatureRegistry.ts`).
- Lose battle-tested bidirectional-text, CJK, font-fallback, and IME handling.
- Invalidate the existing conformance suite in `extensions/vscode-api-tests/`.

However, keeping the entire `TextModel` in TypeScript inside the WebView also has costs:
Rust services (LSP bridge, VCS, diagnostics) cannot mutate document state without
crossing the JS↔Rust boundary on every read, creating round-trip latency on hot paths
like inline completion and hover.

---

## Decision

**Keep the Monaco renderer in TypeScript inside the Tauri WebView short-term.**

**Move TextModel authority to Rust (`vscode-platform/src/editor/model.rs`).**

Specifically:

1. Monaco renderer continues to run in the WebView as-is. No changes to
   `src/vs/editor/browser/`.

2. A thin TypeScript bridge (`vscode-platform-bridge.ts`) translates Monaco
   `contentChanged` events to Tauri `editor:applyEdit` events directed at the Rust
   `TextModel`.

3. Rust `TextModel` holds the single source of truth: content, version counter, and
   decorations. Language servers, diagnostics, and VCS write to Rust state only.

4. Rust pushes `editor:modelChanged` and `editor:decorationsChanged` events back to the
   WebView. The TS bridge reconciles the display model.

5. `RopeBuffer` (ropey crate) backs in-memory content for fast slicing and mutation. The
   full piece-tree from `src/vs/editor/common/model/pieceTreeTextBuffer/` may be ported
   to Rust in a later task if profiling shows ropey as a bottleneck.

6. If profiling (post-launch) shows Tauri IPC latency exceeding the 16 ms p99 typing
   budget, the renderer may be ported to a native Rust widget (e.g., xi-editor/floem or a
   custom wgpu renderer). This is explicitly deferred.

---

## Consequences

### Positive

- Renderer work unblocked immediately. All existing Monaco tests, themes, keybindings,
  and extension provider integrations remain valid.
- Rust services get a fast, lock-protected, in-process `TextModel` — no round-trips for
  language server reads.
- Decorator pipeline (LSP diagnostics, VCS blame, inlay hints) simplified: Rust owns all
  source-of-truth, pushes a single reconciled decoration set to the renderer.
- Incremental migration path: each component can be replaced independently.

### Negative / Risks

- Two TextModel copies exist during steady state (Rust authoritative + TS display).
  Version divergence must be detected and recovered (snapshot resync protocol).
- Undo/redo stack currently lives in `src/vs/editor/common/model/editStack.ts`. Migrating
  authority to Rust complicates undo while TS still drives user interactions. Deferred to
  a later task.
- Multi-cursor and overlapping edits require careful ordering (bottom-up application) not
  yet enforced in the stub.
- Tauri IPC adds ~0.5–1 ms per event round-trip. At 60 wpm (≈ 5 keystrokes/s) this is
  negligible, but burst typing could queue events. Needs measurement.

### Neutral

- No changes to any file outside `tauri/vscode-platform/src/editor/` and `tauri/docs/`
  in this task. The TS Monaco source (`src/vs/editor/`) is read-only for now.

---

## Alternatives Considered

| Option | Verdict |
|---|---|
| Port renderer to Rust now (wgpu / floem) | Rejected — 12–18 month cost, breaks extension ecosystem |
| Keep full TextModel in TypeScript | Rejected — Rust services need in-process model access for sub-millisecond reads |
| Replace Monaco with a different editor (CodeMirror, Zed GPUI) | Rejected — breaks extension provider API surface (50K+ extensions) |
| Keep current Electron, no Tauri | Out of scope — this is the Tauri port project |

---

## References

- `research/docs/2026-04-27-research-what-it-would-take-to.md` — open question on Monaco renderer
- `src/vs/editor/common/model/textModel.ts` — TS TextModel source
- `src/vs/editor/common/languageFeatureRegistry.ts` — provider registry
- `tauri/docs/monaco-integration.md` — sync protocol and architecture diagram
- `tauri/vscode-platform/src/editor/` — Rust stubs (model, buffer, decorations)
