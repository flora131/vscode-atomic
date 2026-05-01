(no external research applicable)

`src/bootstrap-fork.ts` relies exclusively on Node.js built-in APIs (`process.send`, `process.on`, `process.kill`, `process.exit`, `process.stdout`/`stderr`), native ESM dynamic `import()`, and a single defensively-guarded Electron `process['crashReporter'].addExtraParameter` call that is already gated behind a runtime check — none of these constitute third-party libraries with external documentation that is central to understanding or porting this file.
