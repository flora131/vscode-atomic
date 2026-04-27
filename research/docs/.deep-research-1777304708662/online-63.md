# Online Research: src/bootstrap-meta.ts — Tauri/Rust Port

## Verdict

(no external research applicable)

## Justification

`src/bootstrap-meta.ts` is a 55-line configuration metadata loader. Its entire surface area consists of Node.js and JavaScript language/runtime primitives — none of which require consulting external library documentation to understand or replace in a Tauri/Rust context:

### 1. `node:module` / `createRequire` / `import.meta.url`

`createRequire` is a Node.js built-in that lets an ESM module use CommonJS `require()`, anchored to the calling module's URL via `import.meta.url`. Both are Node.js core / ESM spec features, not third-party libraries. In a Tauri/Rust port there is no equivalent need: JSON files are loaded in Rust either statically (`include_str!` + `serde_json::from_str`) or dynamically (`std::fs::read_to_string` + `serde_json`). Neither requires external documentation research beyond standard Rust crate knowledge.

### 2. `process.env` / `process.isEmbeddedApp`

`process` is a Node.js global. Its replacement in Tauri is `std::env::var` on the Rust side, or `import.meta.env` in the Vite-bundled frontend. Both are well-known idioms with no ambiguity requiring web lookup.

### 3. Build-time sentinel patching (`BUILD_INSERT_PRODUCT_CONFIGURATION` / `BUILD_INSERT_PACKAGE_CONFIGURATION`)

The file uses a build-time string-replacement trick to embed JSON at compile time. In Tauri/Rust this pattern maps directly to Cargo build scripts (`build.rs`) or Vite plugin transforms. No external library documentation is central to implementing or understanding this transformation.

### 4. JSON merging / `Object.assign` logic

The merging of `product.json`, `product.sub.json`, `product.overrides.json`, and their package equivalents is pure JavaScript object manipulation. The Rust equivalent is `serde_json::Value` merging, a standard pattern in the `serde_json` crate that requires no web research.

### 5. Exported `product` and `pkg` values

These are plain data exports. The Tauri equivalent would be a Rust struct deserialized from JSON and exposed to the frontend via a Tauri command or injected into the webview context. This is standard Tauri architecture requiring no additional documentation beyond what is already covered in existing Tauri research files.

## Summary

`src/bootstrap-meta.ts` uses exclusively Node.js built-ins (`node:module`, `import.meta`, `process`) and standard JavaScript patterns. There are no third-party libraries involved. When porting to Tauri/Rust, every concern in this file — JSON loading, environment variable access, build-time embedding, and data merging — maps to standard Rust/Tauri idioms that are already well-understood or covered by existing research. No external web documentation fetch is warranted for this file.
