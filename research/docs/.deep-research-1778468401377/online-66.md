(no external research applicable)

`src/server-cli.ts` is a 31-LOC internal bootstrap shim. It configures NLS, initialises the ESM loader, sets a handful of environment variables, and then delegates entirely to `vs/server/node/server.cli.js`. Every import it touches is a VS Code-internal bootstrap module; there are no third-party libraries central to this file, so no external library documentation is relevant to the Tauri/Rust porting question for this scope.
