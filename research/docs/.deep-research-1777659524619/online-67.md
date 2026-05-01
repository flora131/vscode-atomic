# Online Research: src/cli.ts Bootstrap Shim

(no external research applicable)

The file `src/cli.ts` is a 26-line ESM bootstrap shim whose sole responsibility is to sequence four local side-effectful imports — `bootstrap-cli.js`, `bootstrap-node.js`, `bootstrap-esm.js`, and `bootstrap-meta.js` — resolve an NLS configuration, set two environment variables (`VSCODE_NLS_CONFIG`, `VSCODE_CLI`), and then dynamically import the actual server-CLI entry point at `./vs/code/node/cli.js`. Every symbol it references is resolved entirely within the VS Code source tree itself; there is no dependency on any third-party library whose external documentation would inform how this shim should be ported or rewritten for a Tauri/Rust target.
