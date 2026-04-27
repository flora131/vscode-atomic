# External Library Research — `extensions/ipynb/` Tauri/Rust Port

Scope: `extensions/ipynb/` (25 files, 4,925 LOC) — Jupyter notebook serializer.

---

## Decision: External Research IS Applicable

Three of the four named dependencies are central to the Tauri/Rust port question:

1. `@jupyterlab/nbformat` — defines every type that drives de/serialisation; a Rust serde equivalent is the core porting task.
2. `detect-indent` — has a direct Rust port on crates.io.
3. `@enonic/fnv-plus` — the FNV-1a hash algorithm has first-class Rust support.

`node:worker_threads` / Web Workers are a concurrency mechanism, not a library; they are addressed in the prose summary rather than as a separate library entry.

---

## Detailed Findings

### 1. `@jupyterlab/nbformat` (devDependency ^3.2.9)

**Role in the extension.** Used as a TypeScript type-only import (`import type * as nbformat from '@jupyterlab/nbformat'`) throughout `serializers.ts`, `deserializers.ts`, and `common.ts`. Every interface that describes the on-disk `.ipynb` JSON structure comes from this package:

- `INotebookContent`, `ICell`, `ICodeCell`, `IMarkdownCell`, `IRawCell`
- Output types: `IExecuteResult`, `IDisplayData`, `IStream`, `IError`, `IUnrecognizedOutput`, `OutputType`
- Auxiliary types: `MultilineString` (`string | string[]`), `IMimeBundle`, `IAttachments`, `ExecutionCount` (`number | null`)

Because it is a **devDependency**, the package ships zero runtime code into the bundle; it is pure schema documentation expressed as TypeScript interfaces.

**Source URL:** https://github.com/jupyterlab/jupyterlab/blob/main/packages/nbformat/src/index.ts
**npm URL:** https://www.npmjs.com/package/@jupyterlab/nbformat

**Rust/serde equivalent — `nbformat` crate, v3.0.0**

A production-quality Rust crate exists that mirrors this schema:

- **crates.io:** https://crates.io/crates/nbformat
- **Repository:** https://github.com/runtimed/runtimed (subdirectory `crates/nbformat`)
- **Downloads:** 161,022 (actively used)
- **Version cadence:** 3.0.0 released 2026-04-26 (actively maintained)

The crate exposes:

```rust
pub enum Notebook {
    V4(v4::Notebook),
    V4QuirksMode(V4Quirks),
    Legacy(legacy::Notebook),   // v4.1–v4.4
    V3(v3::Notebook),
}

pub fn parse_notebook(json: &str) -> Result<Notebook, NotebookError>
```

Internally it uses `serde` + `serde_json` derive macros plus `thiserror` for structured errors. It handles the nbformat 4.5 cell-ID uniqueness constraint (missing IDs are caught as `Quirk::MissingCellId`; callers call `V4Quirks::repair()` to mint fresh UUIDs).

**Port mapping.** The TypeScript code that calls `nbformat.ICodeCell`, `nbformat.IStream`, etc., would map directly to `v4::Cell`, `v4::Output`, etc. in the Rust crate. The `MultilineString` alias (`string | string[]`) maps to a Rust enum that serde deserialises from either a JSON string or a JSON array of strings — the `nbformat` crate already models this. The `sortObjectPropertiesRecursively` function used before `JSON.stringify` would become a custom `serde::Serializer` wrapper or a BTree-based intermediate representation to guarantee alphabetical key order on output (the Rust crate does not currently sort keys; this would need to be added for round-trip fidelity with Jupyter Lab's convention).

---

### 2. `detect-indent` (runtime dependency ^6.0.0)

**Role in the extension.** Called once per `deserializeNotebook` invocation:

```typescript
// notebookSerializer.ts, line 54
const indentAmount = contents
    ? detectIndent(contents.substring(0, 1_000)).indent
    : ' ';
```

The result (`indent` — the detected indent string, e.g. `"  "` or `"\t"`) is stored in `data.metadata.indentAmount` and later passed as the third argument to `JSON.stringify(sorted, undefined, indentAmount)` at serialisation time. This preserves the original file's indentation style across edits, preventing spurious SCM diffs.

**Algorithm.** The library scans only the first 1,000 characters of the file. It counts the most-common indent-size change between consecutive non-empty lines, disambiguates spaces vs. tabs, and returns `{ amount: number, type: 'space'|'tab'|undefined, indent: string }`. The full algorithm is ~160 lines of pure JS with no dependencies.

**Source URL:** https://github.com/sindresorhus/detect-indent (npm), local copy at `extensions/ipynb/node_modules/detect-indent/index.js`

**Rust equivalent.** The `detect-indent` readme explicitly links to its Rust port:

- **crate name:** `detect-indent`
- **crates.io:** https://crates.io/crates/detect-indent (v0.1.0)
- **Repository:** https://github.com/stefanpenner/detect-indent-rs

The crate exposes `detect_indent(s: &str) -> Indent` with the same algorithm. Downloads (23,908) indicate light but real-world usage. The crate's v0.1.0 may not track the JS library's v6 API changes exactly; this should be verified for edge-case parity (e.g., the `ignoreSingleSpaces` two-pass behaviour added in detect-indent v6). In a Rust port, this is a trivial reimplementation risk — the algorithm is short enough to vendor inline if parity gaps exist.

---

### 3. `@enonic/fnv-plus` (runtime dependency ^1.3.0)

**Role in the extension.** Used in exactly one place, `notebookSerializer.ts` line 38:

```typescript
import * as fnv from '@enonic/fnv-plus';
// ...
const fileHash = fnv.fast1a32hex(backupId) as string;
```

This hashes a `__webview_backup` key (a string ID stored in the notebook JSON by the Jupyter VS Code extension) to derive a filesystem path for a Jupyter backup file. The call path executes only when `json.__webview_backup` is truthy — a backward-compatibility code path for reading Jupyter extension backup files. The hash is used purely as a deterministic filename component; cryptographic properties are irrelevant.

**Algorithm.** `fast1a32hex` computes FNV-1a 32-bit hash with the default seed and returns a lowercase hex string (e.g. `"d58b3fa7"`). No external dependencies; the entire fast variant is ~20 lines of JS.

**Source URL:** https://github.com/tjwebb/fnv-plus (upstream), `@enonic/fnv-plus` is a fork pinned to v1.3.0.
**npm URL:** https://www.npmjs.com/package/@enonic/fnv-plus

**Rust equivalent.** The `fnv` crate (v1.0.7) is the canonical Rust FNV-1a implementation:

- **crates.io:** https://crates.io/crates/fnv
- **Usage:** `fnv::FnvHasher` or the free `fnv::hash` functions

For the exact `fast1a32hex` behaviour (32-bit FNV-1a → lowercase hex string), a direct Rust equivalent is:

```rust
use fnv::FnvHasher;
use std::hash::{Hash, Hasher};

fn fast1a32hex(s: &str) -> String {
    let mut h = FnvHasher::default(); // 64-bit; for 32-bit, fold manually
    s.hash(&mut h);
    format!("{:08x}", h.finish() as u32)
}
```

For strict 32-bit FNV-1a byte-level parity with the JS implementation (which operates on char codes, not UTF-8 bytes), a ~10-line inline implementation is safer than relying on `fnv` crate's hasher (which hashes bytes via `Write`). The `fnv_rs` crate (v0.4.4, https://crates.io/crates/fnv_rs) supports explicit 32/64/128-bit variants and may be a cleaner match.

---

### 4. `node:worker_threads` / Web Workers (concurrency mechanism)

**Role in the extension.** The `NotebookSerializer.node.ts` uses `node:worker_threads.Worker` to offload `serializeNotebookToString` to a background thread when `ipynb.experimental.serialization` is enabled. `notebookSerializer.web.ts` uses the browser `Worker` API for the same purpose. The worker scripts (`notebookSerializerWorker.ts`, `notebookSerializerWorker.web.ts`) contain only a message listener that calls `serializeNotebookToString` and posts back a `Uint8Array`.

**Port implications.** In a Tauri/Rust port:

- The serialisation call itself would move to Rust (synchronous or async), removing the need for a JS worker thread entirely.
- Tauri's `invoke` mechanism handles cross-boundary calls asynchronously on a Tokio thread pool; the worker-offloading pattern would be replaced by a standard `#[tauri::command] async fn serialize_notebook(data: NotebookData) -> Result<Vec<u8>, String>` handler.
- No external crate is required for this substitution.

---

## Additional Resources

- Jupyter nbformat v4 JSON Schema: https://github.com/jupyter/nbformat/blob/master/nbformat/v4/nbformat.v4.schema.json — authoritative schema against which both the TS types and any Rust structs should be validated.
- `nbformat` Rust crate docs: https://docs.rs/nbformat/latest/nbformat/
- `detect-indent-rs` repo: https://github.com/stefanpenner/detect-indent-rs
- `fnv` crate docs: https://docs.rs/fnv/latest/fnv/
- `fnv_rs` crate (multi-width): https://crates.io/crates/fnv_rs

---

## Gaps and Limitations

1. **Key-ordering on serialisation.** The JS code calls `sortObjectPropertiesRecursively` before `JSON.stringify` to alphabetise all JSON keys (matching Jupyter Lab's convention). The `nbformat` Rust crate does not currently guarantee alphabetical key ordering on serialisation. A port would need to add this, either via a custom `serde::Serializer` or by serialising through `BTreeMap` wrappers.

2. **`detect-indent-rs` version parity.** The Rust crate is at v0.1.0 and may not reflect the v6 two-pass behaviour of the JS library. Functional testing against real `.ipynb` files with mixed indentation is recommended before adoption.

3. **`fast1a32hex` exact byte semantics.** The JS `fnv-plus` implementation operates on JavaScript character codes (UTF-16 code units for the fast variant), while Rust's `fnv` crate hashes raw bytes. For ASCII backup IDs (the only observed call site) this makes no difference, but the discrepancy should be noted for any round-trip compatibility test.

4. **vscode API surface.** The extension is heavily coupled to `vscode.NotebookData`, `vscode.NotebookCellOutput`, and `vscode.NotebookCellOutputItem`. A Tauri port would need to define equivalent Rust structs (matching the same JSON shape) to serve as the de/serialisation boundary. The `nbformat` crate already defines the `.ipynb` on-disk shape; the VS Code internal representation would need separate Rust structs and a translation layer.

---

## Summary

All three runtime/devDependency libraries have well-maintained Rust equivalents. The most significant porting work is around `@jupyterlab/nbformat`: the `nbformat` crate (v3.0.0, runtimed/runtimed) provides serde structs for all cell and output types, but a custom JSON key-sorting serialiser must be added to match Jupyter Lab's alphabetical convention. `detect-indent` maps cleanly to `detect-indent-rs` (verify v6 two-pass parity). `@enonic/fnv-plus`'s single call site (`fast1a32hex`) is trivially reimplemented with the `fnv` or `fnv_rs` crate. The Node `worker_threads` / Web Worker offloading pattern dissolves entirely in a Tauri context, replaced by async Tauri command handlers on Tokio threads. The partition has no opaque binary dependencies and its logic is well-bounded JSON parsing and transformation, making it one of the more straightforward partitions to port.
