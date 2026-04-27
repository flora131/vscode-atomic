# Analyzer-74: extensions/latex — Grammar-Only LaTeX/TeX/BibTeX Extension

> Scope: `extensions/latex/` (grammar-only partition)
> Research question: What would it take to port VS Code's core IDE functionality from TypeScript/Electron to Tauri/Rust?
> Note: This partition contains no TypeScript/Electron application logic. All portability observations are therefore derived exclusively from the declarative configuration and build tooling present in scope.

---

### Files Analysed

| File | Lines | Read in full? |
|---|---|---|
| `/Users/norinlavaee/vscode-atomic/extensions/latex/package.json` | 120 | Yes |
| `/Users/norinlavaee/vscode-atomic/extensions/latex/build/update-grammars.js` | 14 | Yes |
| `/Users/norinlavaee/vscode-atomic/extensions/latex/latex-language-configuration.json` | 120 | Yes |
| `/Users/norinlavaee/vscode-atomic/extensions/latex/latex-cpp-embedded-language-configuration.json` | 33 | Yes |
| `/Users/norinlavaee/vscode-atomic/extensions/latex/markdown-latex-combined-language-configuration.json` | 126 | Yes |
| `/Users/norinlavaee/vscode-atomic/extensions/latex/package.nls.json` | 4 | Yes |
| `/Users/norinlavaee/vscode-atomic/extensions/latex/cgmanifest.json` | 38 | Yes |
| `/Users/norinlavaee/vscode-atomic/extensions/latex/.vscodeignore` | 3 | Yes |

Large TextMate grammar JSONs (`LaTeX.tmLanguage.json` 4446 LOC, `cpp-grammar-bailout.tmLanguage.json` 20054 LOC, `Bibtex.tmLanguage.json`, `TeX.tmLanguage.json`, `markdown-latex-combined.tmLanguage.json`) were not read per scope instructions.

---

### Per-File Notes

#### `/Users/norinlavaee/vscode-atomic/extensions/latex/package.json`

- **Role:** VS Code extension manifest. Declares the five language IDs contributed by this extension and binds each to a grammar file and a language-configuration file. Acts as the sole integration contract between this extension and the VS Code extension host.
- **Key symbols:**
  - `contributes.languages` (lines 16–64): array of five language descriptors — `tex`, `latex`, `bibtex`, `cpp_embedded_latex`, `markdown_latex_combined`.
  - `contributes.grammars` (lines 65–114): array of five grammar descriptors, one per language, linking language IDs to TextMate scope names and `.tmLanguage.json` paths.
  - `scripts["update-grammar"]` (line 12): single npm script that delegates to `./build/update-grammars.js`.
  - `engines.vscode: "*"` (line 9): declares compatibility with all VS Code engine versions.
- **Control flow:** No runtime control flow; the manifest is consumed statically by the VS Code extension host at activation time. There is no `main` entry point, confirming the extension is purely declarative.
- **Data flow:**
  - `contributes.languages[*].configuration` fields (lines 29, 42, 57, 62) point to the three language-configuration JSON files in the extension root.
  - `contributes.grammars[*].path` fields (lines 69, 81, 103, 107, 111) point into `./syntaxes/`.
  - `contributes.grammars[1].embeddedLanguages` (lines 83–97) maps 13 TextMate scope names to VS Code language IDs, enabling embedded-language tokenization inside LaTeX documents (e.g., `source.cpp` → `cpp_embedded_latex`, `source.python` → `python`).
  - `contributes.grammars[0].unbalancedBracketScopes` and `contributes.grammars[1].unbalancedBracketScopes` (lines 70–73, 79–82) suppress false bracket-match highlights for two TeX-specific scopes.
- **Dependencies:** `vscode` engine (host); no npm runtime dependencies declared; dev dependency is the external `vscode-grammar-updater` package (not present in the repo tree, resolved at build time via node_modules).

---

#### `/Users/norinlavaee/vscode-atomic/extensions/latex/build/update-grammars.js`

- **Role:** Build-time utility script. Fetches the five upstream TextMate grammar files from the `jlelong/vscode-latex-basics` GitHub repository and writes them into `syntaxes/`, overwriting local copies.
- **Key symbols:**
  - `updateGrammar` (line 7): the single imported module, `vscode-grammar-updater`, which provides the `update()` function.
  - Five `updateGrammar.update()` calls (lines 9–13), one per grammar file.
- **Control flow:** Linear, top-to-bottom. Each `update()` call is independent; they are not chained or conditioned on each other. The script terminates after all five calls are initiated (the `vscode-grammar-updater` package handles async HTTP internally).
- **Data flow:**
  - Each `update()` invocation receives four arguments: upstream repository slug (`'jlelong/vscode-latex-basics'`), source path within that repo, local destination path (both identical in all five calls), an `undefined` transform function (no post-processing), and the branch name `'main'`.
  - The function fetches the raw file from the GitHub API and writes it to the local `syntaxes/` path.
  - No data is returned or piped between calls.
- **Dependencies:** `vscode-grammar-updater` (external npm package, not vendored in the repository tree). Requires network access to `api.github.com` at build time. No other imports.

---

#### `/Users/norinlavaee/vscode-atomic/extensions/latex/latex-language-configuration.json`

- **Role:** Editor behavior configuration for the `tex` and `latex` language IDs (both share this file via `package.json` lines 29, 42). Consumed by the VS Code extension host to configure editor features without any runtime code.
- **Key symbols:**
  - `comments.lineComment` (line 3): `"%"` — designates `%` as the TeX/LaTeX line comment character.
  - `brackets` (lines 5–71): 70 bracket pairs, covering standard ASCII pairs (`{}`, `[]`, `()`) plus an extensive set of LaTeX math delimiters (`\left(` / `\right)`, `\bigl` / `\bigr` variants at all four size levels, `\langle` / `\rangle`, `\lvert` / `\rvert`, etc.).
  - `autoClosingPairs` (lines 72–95): 22 pairs; a subset of the bracket list used for automatic closing. Includes the backtick/apostrophe pair `` ` `` / `'` (line 94) for TeX-style quoting.
  - `surroundingPairs` (lines 96–103): 7 pairs including `$`/`$` (line 103) for math-mode toggling.
  - `indentationRules` (lines 105–108): two regex patterns that increase indent after `\begin{...}` and decrease it before `\end{...}`, with an explicit exception for the `document` environment.
  - `folding.markers` (lines 110–113): regex-based fold start/end for `%region` / `%endregion` comments and `\begingroup` / `\endgroup` commands.
  - `autoCloseBefore` (line 115): string of characters after which auto-closing is suppressed.
  - `wordPattern` (lines 116–119): Unicode-aware regex (`\p{Alphabetic}|\p{Number}|\p{Nonspacing_Mark}`) with flag `"u"` for double-click word selection.
- **Control flow:** No control flow; purely declarative JSON consumed by the host.
- **Data flow:** Read once by the extension host at language activation; values are wired into editor subsystems (bracket matching, auto-closing, indentation, folding, word selection). No runtime mutations.
- **Dependencies:** Consumed by VS Code extension host. References no external files. Shared by both `tex` and `latex` language IDs.

---

#### `/Users/norinlavaee/vscode-atomic/extensions/latex/latex-cpp-embedded-language-configuration.json`

- **Role:** Editor behavior configuration for the synthetic `cpp_embedded_latex` language ID — the virtual language used when C++ is embedded inside a LaTeX document via the `\begin{cppcode}` family of environments.
- **Key symbols:**
  - `comments` (lines 2–5): C++ comment styles (`//` line comment, `/* */` block comment).
  - `brackets` (lines 6–10): three standard ASCII pairs only (`{}`, `[]`, `()`).
  - `autoClosingPairs` (lines 11–17): five pairs including single/double-quote pairs with `notIn` guards for `string` and `comment` contexts (lines 15–16).
  - `surroundingPairs` (lines 18–25): six pairs including `<>`.
  - `wordPattern` (line 26): C++ identifier regex matching numeric literals and non-operator sequences.
  - `folding.markers` (lines 27–32): `#pragma region` / `#pragma endregion` patterns.
- **Control flow:** No control flow; purely declarative.
- **Data flow:** Consumed by the extension host when the tokenizer encounters the `source.cpp.embedded.latex` scope and switches to the `cpp_embedded_latex` virtual language for editor features. Values govern bracket matching, auto-close, and folding for the C++ sub-document.
- **Dependencies:** Consumed by VS Code extension host. Linked from `package.json` line 57.

---

#### `/Users/norinlavaee/vscode-atomic/extensions/latex/markdown-latex-combined-language-configuration.json`

- **Role:** Editor behavior configuration for the `markdown_latex_combined` virtual language ID — used for Markdown documents with embedded LaTeX math (as used by Markdown-Math or similar extensions that embed LaTeX grammars into Markdown).
- **Key symbols:**
  - `comments.blockComment` (lines 3–6): HTML block comment `<!-- -->`.
  - `brackets` (lines 7–74): the full LaTeX math delimiter set (identical to `latex-language-configuration.json`) plus the three ASCII pairs.
  - `autoClosingPairs` (lines 75–98): identical to `latex-language-configuration.json`.
  - `surroundingPairs` (lines 99–108): extends the LaTeX set with Markdown-specific pairs: backtick (line 106), underscore (line 107), asterisk (line 108).
  - `indentationRules` (lines 110–113): identical LaTeX `\begin{}`/`\end{}` patterns.
  - `folding` (lines 114–119): uses `offSide: true` (line 115) and HTML comment region markers (`<!-- #region -->` / `<!-- #endregion -->`), contrasting with the `%region` markers in the pure LaTeX config.
  - `autoCloseBefore` (line 121): same string as the LaTeX config.
  - `wordPattern` (lines 122–125): extends the LaTeX Unicode word pattern to optionally wrap the match in Markdown emphasis delimiters (`[*_]{1,2}`).
- **Control flow:** No control flow; purely declarative.
- **Data flow:** Consumed by the extension host for the `markdown_latex_combined` embedded virtual language. Provides combined Markdown-and-LaTeX editor behaviors in a single configuration object.
- **Dependencies:** Consumed by VS Code extension host. Linked from `package.json` line 62.

---

#### `/Users/norinlavaee/vscode-atomic/extensions/latex/package.nls.json`

- **Role:** Localization string table for the extension manifest. Provides the English display strings for the `%displayName%` and `%description%` tokens in `package.json` lines 3–4.
- **Key symbols:**
  - `displayName` (line 2): `"LaTeX Language Basics"`
  - `description` (line 3): `"Provides syntax highlighting and bracket matching for TeX, LaTeX and BibTeX."`
- **Control flow:** None; consumed statically by VS Code's extension NLS loader.
- **Data flow:** Values substitute into `package.json` at extension-host load time for display in the Extensions panel and marketplace.
- **Dependencies:** VS Code extension host NLS subsystem.

---

#### `/Users/norinlavaee/vscode-atomic/extensions/latex/cgmanifest.json`

- **Role:** Component Governance manifest. Registers the single upstream open-source component (`jlelong/vscode-latex-basics`) that supplies all five grammar files, for legal/license-tracking tooling used by Microsoft's supply-chain compliance process.
- **Key symbols:**
  - `registrations[0].component.git.repositoryUrl` (line 8): `https://github.com/jlelong/vscode-latex-basics`
  - `registrations[0].component.git.commitHash` (line 9): `76dc409348227db00f6779772f7763dc90cdf22e` — the pinned upstream commit.
  - `registrations[0].version` (line 12): `"1.16.0"`
  - `registrations[0].licenseDetail` (lines 13–33): describes the MIT base license and two grammar-file-specific license overrides (the original LaTeX.tmbundle permissive license and the `cpp-grammar-bailout` file's separate license described in `cpp-bailout-license.txt`).
- **Control flow:** None; consumed by external Microsoft Component Governance tooling, not by VS Code itself.
- **Data flow:** Provides audit metadata linking the local `syntaxes/` files to their upstream source at a specific commit. The commit hash in this file should correspond to what `update-grammars.js` would fetch from the `main` branch at the time of the last grammar update.
- **Dependencies:** External Microsoft CG tooling. Not referenced at runtime.

---

#### `/Users/norinlavaee/vscode-atomic/extensions/latex/.vscodeignore`

- **Role:** Packaging exclusion list. Specifies paths omitted when the extension is packaged into a `.vsix` file by `vsce`.
- **Key symbols:**
  - `cgmanifest.json` (line 1): excluded from the packaged extension.
  - `build/**` (line 2): the entire `build/` directory (containing `update-grammars.js`) is excluded from the package.
- **Control flow:** None; consumed by the `vsce` packaging tool.
- **Data flow:** Ensures that build-time tooling (`update-grammars.js`) and compliance metadata (`cgmanifest.json`) are not shipped inside the published extension artifact.
- **Dependencies:** `vsce` (VS Code Extension packaging tool).

---

### Cross-Cutting Synthesis

The `extensions/latex/` partition is a purely declarative, grammar-only VS Code extension. It contributes five language IDs (`tex`, `latex`, `bibtex`, `cpp_embedded_latex`, `markdown_latex_combined`) to the VS Code extension host. There is no TypeScript, no `main` activation entry point, and no runtime code whatsoever — the extension consists entirely of JSON manifests, JSON language-configuration files, and the five TextMate grammar files in `syntaxes/`. The only imperative code is `build/update-grammars.js`, which is a one-time build-time script (explicitly excluded from the packaged extension by `.vscodeignore`) that pulls upstream grammar files from `jlelong/vscode-latex-basics` via the `vscode-grammar-updater` npm package.

From a Tauri/Rust porting perspective, this partition presents essentially zero porting complexity on its own terms. The TextMate grammars, language configurations, and manifest structure are consumed by VS Code's extension host JavaScript runtime. In a Tauri/Rust host, the equivalent subsystem would need to implement or embed a TextMate grammar engine (such as `syntect` in Rust, which already supports `.tmLanguage` grammars) and a language-configuration loader. The five `.tmLanguage.json` files themselves are format-portable. The `embeddedLanguages` map in `package.json` lines 83–97 is a VS Code-specific extension API concept that a Tauri host would need to re-implement independently. The `build/update-grammars.js` script has no dependency on Electron or VS Code internals and would remain usable as-is for maintaining grammar freshness regardless of the target editor platform.

---

### Out-of-Partition References

- `/Users/norinlavaee/vscode-atomic/extensions/markdown-math/package.json` — references the `latex` language ID (confirmed by grep), indicating a cross-extension dependency where markdown-math embeds LaTeX grammar scopes.
- `/Users/norinlavaee/vscode-atomic/extensions/markdown-basics/package.json` — also references the `latex` language ID, indicating the markdown-basics extension integrates with the LaTeX grammar for combined rendering.
- The `cpp_embedded_latex` and `markdown_latex_combined` virtual language IDs are private to this extension (aliases are empty arrays, `package.json` lines 57, 62), meaning they are not directly user-selectable but exist purely as embedded-language targets inside the LaTeX tokenizer pipeline.
- The `vscode-grammar-updater` npm package is resolved at build time and is not present in the repository tree; its API contract (four-argument `update(repo, srcPath, destPath, transform, branch)`) is the only external interface exercised by this partition.

---

This partition is a sentinel case for the Tauri/Rust porting research question: because the extension contains no TypeScript application logic and no Electron API calls, it represents the easiest category of VS Code extension to carry forward to an alternative host. The full porting burden for language support of this kind shifts entirely onto the host platform's ability to load and execute TextMate grammars and honor the declarative language-configuration contract — both of which are well-defined, documented, and already partially addressed by Rust ecosystem libraries such as `syntect`.
