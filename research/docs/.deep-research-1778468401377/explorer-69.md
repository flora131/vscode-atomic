# Partition 69 of 80 — Findings

## Scope
`extensions/cpp/` (1 files, 23 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
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

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
### Files Analysed

- `/home/norinlavaee/projects/vscode-atomic/extensions/cpp/package.json` (137 lines)
- `/home/norinlavaee/projects/vscode-atomic/extensions/cpp/package.nls.json` (4 lines)
- `/home/norinlavaee/projects/vscode-atomic/extensions/cpp/language-configuration.json` (132 lines)
- `/home/norinlavaee/projects/vscode-atomic/extensions/cpp/snippets/c.code-snippets` (16 lines)
- `/home/norinlavaee/projects/vscode-atomic/extensions/cpp/snippets/cpp.code-snippets` (16 lines)
- `/home/norinlavaee/projects/vscode-atomic/extensions/cpp/cgmanifest.json` (95 lines)
- `/home/norinlavaee/projects/vscode-atomic/extensions/cpp/.vscodeignore` (3 lines)
- `/home/norinlavaee/projects/vscode-atomic/extensions/cpp/build/update-grammars.js` (23 lines)
- `/home/norinlavaee/projects/vscode-atomic/extensions/cpp/syntaxes/c.tmLanguage.json` (header+patterns read)
- `/home/norinlavaee/projects/vscode-atomic/extensions/cpp/syntaxes/cpp.tmLanguage.json` (header+patterns read)
- `/home/norinlavaee/projects/vscode-atomic/extensions/cpp/syntaxes/cpp.embedded.macro.tmLanguage.json` (header+patterns read)
- `/home/norinlavaee/projects/vscode-atomic/extensions/cpp/syntaxes/cuda-cpp.tmLanguage.json` (header+patterns read)
- `/home/norinlavaee/projects/vscode-atomic/extensions/cpp/syntaxes/platform.tmLanguage.json` (header+patterns read)

---

### Per-File Notes

#### `package.json`

This is the extension manifest. It contributes three language IDs at lines 16-72:

- `c` — mapped to extensions `.c`, `.i` (`package.json:19-21`), configuration pointer to `./language-configuration.json` (`package.json:27`).
- `cpp` — mapped to 20 extensions covering all standard C++ source, header, module-interface, Arduino, and template file suffixes (`package.json:32-53`), same language-configuration file (`package.json:60`).
- `cuda-cpp` — mapped to `.cu` and `.cuh` (`package.json:65-68`), same language-configuration file (`package.json:71`).

Five TextMate grammars are registered at lines 74-98:

| Grammar entry | `scopeName` | File |
|---|---|---|
| `package.json:75-79` | `source.c` | `syntaxes/c.tmLanguage.json` |
| `package.json:80-84` | `source.cpp.embedded.macro` | `syntaxes/cpp.embedded.macro.tmLanguage.json` |
| `package.json:85-89` | `source.cpp` | `syntaxes/cpp.tmLanguage.json` |
| `package.json:90-93` | `source.c.platform` | `syntaxes/platform.tmLanguage.json` (no `language` field, injected/referenced by other grammars) |
| `package.json:94-98` | `source.cuda-cpp` | `syntaxes/cuda-cpp.tmLanguage.json` |

A problem pattern named `nvcc-location` is defined at `package.json:100-109`. Its regex `^(.*)\\((\\d+)\\):\\s+(warning|error):\\s+(.*)` captures file path as group 1, line as group 2, severity as group 3, and message as group 4. A problem matcher named `nvcc` at `package.json:111-121` consumes this pattern with `owner: "cuda-cpp"` and `fileLocation: ["relative", "${workspaceFolder}"]`. Two snippet files are registered at `package.json:122-131`, one each for `c` and `cpp`.

The `scripts` field at `package.json:11-13` exposes a single `update-grammar` command that delegates to `node ./build/update-grammars.js`.

#### `package.nls.json`

Contains only two localizable strings (`package.nls.json:2-4`): `displayName` resolves to `"C/C++ Language Basics"` and `description` to `"Provides snippets, syntax highlighting, bracket matching and folding in C/C++ files."`.

#### `language-configuration.json`

Shared by all three language IDs. Key sections:

- **Comments** (`language-configuration.json:2-8`): `//` for line comments, `/* */` for block comments.
- **Brackets** (`language-configuration.json:9-20`): `{}`, `[]`, `()`.
- **Auto-closing pairs** (`language-configuration.json:23-65`): `[`, `{`, `(`, single quote (not in string/comment contexts), double quote (not in string), `/*` → `*/` (not in string/comment), `/**` → ` */` (not in string).
- **Surrounding pairs** (`language-configuration.json:67-92`): same bracket set plus `<>`.
- **Word pattern** (`language-configuration.json:93`): a regex that captures floating-point literals and identifier characters, excluding all operator and punctuation characters.
- **Folding markers** (`language-configuration.json:94-99`): start on `#pragma region`, end on `#pragma endregion`.
- **Indentation rules** (`language-configuration.json:100-106`): decrease on lines beginning with `}`, `]`, or `)`; increase on lines ending with an unclosed `{`, `(`, or `[`.
- **On-enter rules** (`language-configuration.json:108-130`): two rules. The first outdents after single-line `if`/`else if`/`else`/`for`/`while` statements (`language-configuration.json:110-117`). The second continues a line comment by appending `// ` on the new line when the cursor is inside a `//` comment that has non-whitespace text after it (`language-configuration.json:119-130`).

#### `snippets/c.code-snippets` and `snippets/cpp.code-snippets`

Both files are identical in content. Each provides exactly two snippets:

- `"Region Start"` (`c.code-snippets:2-8`, `cpp.code-snippets:2-8`): prefix `#region`, body expands to `#pragma region $0`.
- `"Region End"` (`c.code-snippets:9-15`, `cpp.code-snippets:9-15`): prefix `#endregion`, body expands to `#pragma endregion`.

These snippets map the `#region`/`#endregion` shorthand prefix notation to the underlying `#pragma region`/`#pragma endregion` directives that the folding markers in `language-configuration.json:94-99` recognise.

#### `cgmanifest.json`

Component governance manifest listing four upstream third-party sources at `cgmanifest.json:1-94`:

1. `jeff-hykin/better-cpp-syntax` v1.17.4, commit `f1d127a8`, MIT license — source for `cpp.tmLanguage.json` and `cpp.embedded.macro.tmLanguage.json`.
2. `jeff-hykin/better-c-syntax` v1.13.2, commit `34712a6`, MIT license — source for `c.tmLanguage.json`.
3. `textmate/c.tmbundle` v0.0.0, commit `60daf83b`, TextMate Bundle License — source for `platform.tmLanguage.json`.
4. `NVIDIA/cuda-cpp-grammar` v0.0.0, commit `81e88ea`, MIT license — source for `cuda-cpp.tmLanguage.json`.

The description at `cgmanifest.json:39` and `cgmanifest.json:52` traces the grammar lineage: all C/C++ grammars ultimately derive from the TextMate `c.tmbundle`, via Atom's `language-c`.

#### `.vscodeignore`

Three lines excluding `build/**`, `test/**`, and `cgmanifest.json` from the packaged extension at `.vscodeignore:1-3`. The grammar updater script and governance manifest are therefore not shipped in extension bundles.

#### `build/update-grammars.js`

The updater script imports `vscode-grammar-updater` (`build/update-grammars.js:7`) and defines a single async function `updateGrammars` at `build/update-grammars.js:9-20`. It makes four `updateGrammar.update()` calls:

1. `build/update-grammars.js:10` — fetches `jeff-hykin/better-c-syntax` branch `master`, file `autogenerated/c.tmLanguage.json`, writes to `./syntaxes/c.tmLanguage.json`.
2. Lines 13-14 are commented out — the `better-cpp-syntax` updates for `cpp.tmLanguage.json` and `cpp.embedded.macro.tmLanguage.json` are frozen. The comment at `build/update-grammars.js:12` states the upstream license changed to one incompatible with the VS Code license, so those grammars are pinned at commit `f1d127a8`.
3. `build/update-grammars.js:16` — fetches `NVIDIA/cuda-cpp-grammar` branch `master`, file `syntaxes/cuda-cpp.tmLanguage.json`, writes to `./syntaxes/cuda-cpp.tmLanguage.json`.
4. `build/update-grammars.js:19` — fetches `textmate/c.tmbundle` (default branch), file `Syntaxes/Platform.tmLanguage`, converts to JSON, writes to `./syntaxes/platform.tmLanguage.json`.

The function is called immediately at `build/update-grammars.js:22`.

#### Grammar files (syntaxes/)

**`c.tmLanguage.json`** — scope `source.c`, sourced from `jeff-hykin/better-c-syntax` at commit `34712a6` (recorded in `c.tmLanguage.json:7`). Top-level `patterns` array includes named sections such as `#preprocessor-rule-enabled`, `#preprocessor-rule-disabled`, `#preprocessor-rule-conditional`, `#predefined_macros`, `#comments`, `#switch_statement`, `#storage_types`, `#operators`, `#numbers`, `#strings`.

**`cpp.tmLanguage.json`** — scope `source.cpp`, sourced from `jeff-hykin/better-cpp-syntax` at commit `f1d127a8` (`cpp.tmLanguage.json:7`). Top-level patterns include `#ever_present_context`, `#constructor_root`, `#destructor_root`, `#function_definition`, `#operator_overload`, `#using_namespace`, `#type_alias`, `#using_name`, `#namespace_alias`, `#namespace_block`, `#extern_block`, `#typedef_class`, `#typedef_struct`, `#typedef_union`, `#misc_keywords`, `#standard_declares`, `#class_block`.

**`cpp.embedded.macro.tmLanguage.json`** — scope `source.cpp.embedded.macro`, same upstream commit as `cpp.tmLanguage.json` (`cpp.embedded.macro.tmLanguage.json:7`). It mirrors the `cpp.tmLanguage.json` pattern list with a notable difference: several includes cross-reference `source.cpp` directly using the `source.cpp#<name>` notation (e.g., `"include": "source.cpp#type_alias"` at `cpp.embedded.macro.tmLanguage.json:31`), meaning those rules are resolved from the sibling grammar rather than defined locally.

**`cuda-cpp.tmLanguage.json`** — scope `source.cuda-cpp`, sourced from `NVIDIA/cuda-cpp-grammar` at commit `81e88ea` (`cuda-cpp.tmLanguage.json:7`). Pattern structure mirrors `cpp.tmLanguage.json` with CUDA-specific kernel syntax additions (not visible in the first 40 lines but implied by the upstream NVIDIA repo).

**`platform.tmLanguage.json`** — scope `source.c.platform`, sourced from `textmate/c.tmbundle` (`platform.tmLanguage.json:7`). Its comment at `platform.tmLanguage.json:10` states it was generated using `clang-C` and `MacOSX10.15.sdk`. The patterns mark deprecated Apple SDK symbols (Audio Unit, CoreFoundation, LaunchServices constants and types) using scopes like `invalid.deprecated.10.10.support.constant.c` (`platform.tmLanguage.json:13`) and `invalid.deprecated.10.11.support.constant.c` (`platform.tmLanguage.json:33`). This grammar has no `language` field in its `package.json` registration (`package.json:90-93`), making it an injected/referenceable grammar rather than a primary language grammar.

---

### Cross-Cutting Synthesis

The `extensions/cpp` extension is a pure declarative configuration package: it contains no TypeScript or JavaScript runtime code that executes at IDE runtime. The extension manifest (`package.json`) registers three language IDs (`c`, `cpp`, `cuda-cpp`) that all share a single `language-configuration.json` governing editor behaviours (bracket matching, auto-close, indentation, folding). Syntax highlighting is handled by five TextMate grammars sourced from four upstream open-source repositories tracked in `cgmanifest.json`. The `cpp.embedded.macro` grammar cross-references rule repositories defined in the sibling `source.cpp` grammar via `source.cpp#<rule>` notation. The `platform` grammar is not bound to any language ID—it functions as a shared rule library for deprecated macOS SDK symbols, included by C and C++ grammars via scope reference. The build script (`build/update-grammars.js`) automates upstream grammar synchronisation using `vscode-grammar-updater`, but the two `better-cpp-syntax`-derived grammars (`cpp.tmLanguage.json`, `cpp.embedded.macro.tmLanguage.json`) are deliberately frozen due to a license incompatibility recorded in comments at `build/update-grammars.js:12-14`. The NVCC problem matcher (`package.json:100-121`) is the only CUDA-specific non-grammar contribution. For a Tauri/Rust port, this entire extension is data-only and requires no TypeScript-to-Rust translation—only the host's extension-point loading mechanism for language IDs, TextMate grammars, problem matchers, and snippets needs to exist on the new platform.

---

### Out-of-Partition References

- `vscode-grammar-updater` npm package — imported at `build/update-grammars.js:7`; not present inside `extensions/cpp/`.
- `source.cpp` scope — cross-referenced directly by `cpp.embedded.macro.tmLanguage.json` (e.g., line 31); the authoritative repository for that scope is `syntaxes/cpp.tmLanguage.json` in this same extension, but the reference is resolved at grammar-engine runtime by VS Code's TextMate tokeniser.
- VS Code's TextMate grammar engine — the `grammars` contribution point at `package.json:74-98` is consumed by the extension host's grammar registry (outside this partition).
- VS Code's problem matcher host — `contributes.problemMatchers` at `package.json:111-121` is consumed by the task runner subsystem (outside this partition).
- `${workspaceFolder}` variable at `package.json:117` — resolved by VS Code's variable substitution service at task execution time.

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Pattern Finder - Partition 69: extensions/cpp/

## Sentinel

The `extensions/cpp/` scope contains only language grammar definitions, snippet configurations, and metadata files with no architectural patterns relevant to porting VS Code core IDE to Tauri/Rust.

## Summary

Partition 69 consists entirely of declarative extension configuration and TextMate grammar files:
- Grammar definitions (`*.tmLanguage.json`) for C/C++ syntax highlighting
- Snippet files (`*.code-snippets`) for code template support  
- Extension package metadata and language registration
- Build scripts for grammar updates

These are extension interface specifications, not core IDE implementation patterns. No code patterns related to IDE architecture, UI frameworks, state management, or Rust/Tauri integration exist in this scope.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
