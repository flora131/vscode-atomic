(no external research applicable)

`src/bootstrap-meta.ts` uses only Node.js built-in modules (`module.createRequire`, `path`, `fs`) to load `product.json` and `package.json` at startup — there are no third-party npm dependencies that would require consulting external documentation or package registries.

**Prose summary:** The file is a minimal, self-contained bootstrap metadata loader (~30 lines) that calls `createRequire` from Node's standard `module` built-in to resolve and read two JSON configuration files (`product.json`, `package.json`) relative to the source tree. Because it relies exclusively on Node.js core APIs that are thoroughly covered by the Node.js standard library documentation already within the assistant's training data, and because no external libraries, frameworks, or Rust/Tauri equivalents are involved at this layer, no online research is necessary or applicable for this scope item.
