(no external research applicable)

`src/bootstrap-cli.ts` is an 11-line environment-cleanup shim that solely reads and deletes the `VSCODE_CWD` process environment variable before the CLI launches. It has no meaningful third-party library dependencies — only Node.js built-ins — so external library documentation is irrelevant to analyzing or porting it.
