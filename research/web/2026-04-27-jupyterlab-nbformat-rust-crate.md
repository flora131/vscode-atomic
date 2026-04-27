---
source_url: https://crates.io/crates/nbformat
fetched_at: 2026-04-27
fetch_method: crates.io API + raw GitHub
topic: Rust serde crate for Jupyter nbformat (runtimed/runtimed)
---

# nbformat Rust crate (v3.0.0)

Repository: https://github.com/runtimed/runtimed (crates/nbformat)
Downloads: 161,022
Latest version: 3.0.0 (released 2026-04-26)

## Summary

Provides serde-based parsing and serialisation of Jupyter Notebook files (.ipynb) in Rust.
Supports v4.5 notebooks via `Notebook::V4`, v4.1-v4.4 via `Notebook::Legacy`, and v3 via `Notebook::V3`.

## Key types

```rust
pub enum Notebook { V4(v4::Notebook), V4QuirksMode(V4Quirks), Legacy(legacy::Notebook), V3(v3::Notebook) }
pub fn parse_notebook(json: &str) -> Result<Notebook, NotebookError>
```

Uses `serde_json` + `serde` derive macros; `thiserror` for errors.
Cell ID validation (nbformat 4.5 requirement) is handled; missing IDs can be repaired via `V4Quirks::repair()`.
