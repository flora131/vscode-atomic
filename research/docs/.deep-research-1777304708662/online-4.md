# Partition 4: `extensions/terminal-suggest/` — External Research Assessment

(no external research applicable)

## Justification

### Package.json audit

`extensions/terminal-suggest/package.json` declares **no runtime npm dependencies**. Its only entry under `devDependencies` is `@types/node`. There are no third-party npm packages to research.

### VS Code proposed-API surface

The `package.json` `enabledApiProposals` field lists two VS Code internal proposed APIs:

- `terminalCompletionProvider` — used via `vscode.window.registerTerminalCompletionProvider`, `vscode.TerminalCompletionContext`, `vscode.TerminalCompletionItem`, `vscode.TerminalCompletionList`, `vscode.TerminalCompletionItemKind`.
- `terminalShellEnv` — used via `terminal.shellIntegration?.env?.value` to read the shell environment.

Both are VS Code-internal proposals defined in the VS Code core itself; their contracts are entirely determined by the codebase being ported, not by any external library or standard. No external documentation is needed to understand them — the type definitions are resolved through the standard `@types/vscode` stubs already present in the workspace.

### Fig spec system

The extension bundles a large, self-contained copy of Fig's autocomplete spec system (under `src/fig/` and `src/completions/`). The type declarations for the `Fig` namespace live in `src/completions/index.d.ts`, which is checked into this repository. The file is authored entirely by the VS Code team (with minor edits noted inline); it does not reference any external npm package such as `@fig/autocomplete` or `@withfig/autocomplete`. All Fig spec logic is application-level TypeScript and does not require consulting external documentation to port.

### Shell-parsing logic

The remaining source under `src/fig/shell-parser/`, `src/shell/`, `src/tokens.ts`, `src/helpers/`, and `src/env/` implements command-line tokenisation, shell-specific global-command enumeration (bash, zsh, fish, pwsh), and PATH-executable caching. This is pure application logic with no external library dependencies beyond Node.js built-ins (`child_process`, `fs`, `path`).

## Summary

The `extensions/terminal-suggest/` partition is almost entirely self-contained application logic: it bundles a local copy of the Fig spec type system, implements its own shell parser, and drives completions through two VS Code-internal proposed APIs (`terminalCompletionProvider` and `terminalShellEnv`). There are no external npm runtime dependencies and no third-party libraries whose documentation would be central to planning a Tauri/Rust port. The porting challenge is purely a matter of (a) re-implementing the VS Code proposed-API contract in a Tauri host layer and (b) translating the TypeScript shell-parsing and Fig-spec-walking logic to Rust — neither of which requires external web research.
