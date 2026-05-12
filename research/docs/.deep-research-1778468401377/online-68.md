(no external research applicable)

The file in scope, `src/cli.ts`, is a 26-line CLI bootstrap shim. According to the locator findings, it imports only internal VS Code bootstrap modules and delegates immediately to `vs/code/node/cli.js`. There are no third-party libraries directly imported in this file.

Because the file contains no external dependencies — no npm packages, no third-party APIs, no framework integrations — there is nothing for online research to clarify or document. The porting work for this file is purely a matter of rewriting the bootstrap delegation logic in Rust (likely as a `main.rs` or a Tauri command entry point), which is an internal architectural decision rather than a question resolvable by fetching external documentation. All relevant context is already present in the source file itself and in the broader VS Code internal module structure.

No external research is applicable here.
