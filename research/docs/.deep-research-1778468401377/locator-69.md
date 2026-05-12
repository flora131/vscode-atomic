# C++ Extension Grammar & Snippets

## Configuration

- `/home/norinlavaee/projects/vscode-atomic/extensions/cpp/package.json` — Extension manifest defining language support for C, C++, and CUDA C++. Registers grammars, snippets, language configurations, and problem matchers for NVCC compiler errors.

- `/home/norinlavaee/projects/vscode-atomic/extensions/cpp/language-configuration.json` — Language config with comments, bracket pairs, auto-closing pairs, folding markers (`#pragma region`/`endregion`), and indentation rules for C/C++.

- `/home/norinlavaee/projects/vscode-atomic/extensions/cpp/package.nls.json` — Localization strings.

- `/home/norinlavaee/projects/vscode-atomic/extensions/cpp/.vscodeignore` — Ignore patterns for packaging.

- `/home/norinlavaee/projects/vscode-atomic/extensions/cpp/cgmanifest.json` — Component governance manifest.

## Grammars

- `/home/norinlavaee/projects/vscode-atomic/extensions/cpp/syntaxes/c.tmLanguage.json` — C language grammar (TextMate format).

- `/home/norinlavaee/projects/vscode-atomic/extensions/cpp/syntaxes/cpp.tmLanguage.json` — C++ language grammar.

- `/home/norinlavaee/projects/vscode-atomic/extensions/cpp/syntaxes/cpp.embedded.macro.tmLanguage.json` — Embedded macro grammar for C++.

- `/home/norinlavaee/projects/vscode-atomic/extensions/cpp/syntaxes/cuda-cpp.tmLanguage.json` — CUDA C++ language grammar.

- `/home/norinlavaee/projects/vscode-atomic/extensions/cpp/syntaxes/platform.tmLanguage.json` — Platform-specific grammar.

## Snippets

- `/home/norinlavaee/projects/vscode-atomic/extensions/cpp/snippets/cpp.code-snippets` — C++ code snippets: `#region` and `#endregion` pragma markers for code folding.

- `/home/norinlavaee/projects/vscode-atomic/extensions/cpp/snippets/c.code-snippets` — C code snippets: `#region` and `#endregion` pragma markers for code folding.

## Build

- `/home/norinlavaee/projects/vscode-atomic/extensions/cpp/build/update-grammars.js` — Build script invoked via `npm run update-grammar` to regenerate or refresh grammar files.

---

## Summary

The C++ extension provides minimal built-in snippets (folding pragmas only) and focuses on grammar definitions and language configuration. Five TextMate grammar files support syntax highlighting for C, C++, C++ macros, CUDA C++, and platform-specific code. Language configuration enables bracket pairing, auto-closing, indentation rules, and block comment handling. A build script exists to update grammars, likely from external sources. The extension also registers NVCC problem matchers for CUDA compilation errors.
