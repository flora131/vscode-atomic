# Partition 29 of 79 — Findings

## Scope
`extensions/vscode-colorize-tests/` (69 files, 1,946 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Colorize/Grammar Tests Locator (Partition 29)

**Scope:** `extensions/vscode-colorize-tests/` (341 files, ~13MB, 502 LOC of test harness code)

---

## Implementation

### Test Harness & Orchestration
- `/home/norinlavaee/projects/vscode-atomic/extensions/vscode-colorize-tests/src/index.ts`
  - Extension activation hook; registers semantic token provider
  - Semantic tokens legend (types: type, struct, class, interface, enum, parameterType, function, variable, testToken)
  - Semantic tokens modifiers (static, abstract, deprecated, declaration, documentation, member, async, testModifier)
  - JSON-based semantic token test provider for pattern matching
  - ~73 LOC

- `/home/norinlavaee/projects/vscode-atomic/extensions/vscode-colorize-tests/src/colorizerTestMain.ts`
  - Test runner configuration entry point
  - Mocha orchestration (ui: 'tdd', timeout: 60s)
  - Multi-reporter setup for CI (mocha-junit-reporter)
  - JUnit XML test result output to `BUILD_ARTIFACTSTAGINGDIRECTORY` or `GITHUB_WORKSPACE`
  - Platform/arch tagged result files
  - ~32 LOC

### Test Implementation
- `/home/norinlavaee/projects/vscode-atomic/extensions/vscode-colorize-tests/src/colorizer.test.ts`
  - Core test suite: `suite('colorization')`
  - Two tokenizer variants per fixture: TextMate (`_workbench.captureSyntaxTokens`) + Tree-sitter (`_workbench.captureTreeSitterSyntaxTokens`)
  - Regression test loop: iterates all fixtures in `test/colorize-fixtures/`
  - Token capture -> JSON serialization -> deep equality assertion
  - Diff handling: accepts same colorization even if theme or position changed (line 34-37)
  - Result persistence: writes JSON snapshots to `colorize-results/` and `colorize-tree-sitter-results/`
  - ~97 LOC

---

## Tests

### Colorization Fixture Corpus
- `/home/norinlavaee/projects/vscode-atomic/extensions/vscode-colorize-tests/test/colorize-fixtures/`
  - **108 test fixtures** across 40+ language modes
  - Cover edge cases and regression issues tied to GitHub issue numbers (issue-1550.yaml, issue-6303.yaml, issue-224862.yaml, test-23850.cpp, test-166781.rs, test-241001.ts, etc.)
  - Language families tested:
    - **C/C++/C#:** test.c, test.cpp, test.cc, test.cu, test-23850.cpp, test-23630.cpp, test-80644.cpp, test-78769.cpp
    - **TypeScript/JavaScript:** test.ts, test.tsx, test.js, test.jsx, test-241001.ts, test-issue241715.ts, test-issue5465.ts, test-object-literals.ts, test-strings.ts, test-issue5431.ts, test-issue11.ts, test-jsdoc-multiline-type.ts, test-function-inv.ts, test-issue5566.ts, test-keywords.ts, test-members.ts, test-this.ts, test6916.js
    - **Web:** test.html, test.xml, test.css, test.less, test.scss, test-embedding.html, test-variables.css, test-cssvariables.less, test-cssvariables.scss, test-4287.pug, test-variables.css
    - **Python:** test.py, test-freeze-56377.py
    - **Rust:** test.rs, test-166781.rs, test-6611.rs
    - **Go:** test.go, test-13777.go
    - **Shell/Scripting:** test.sh, test-173216.sh, test-173336.sh, test-173224.sh
    - **Config/Data:** test.json, test.yaml, issue-6303.yaml, issue-224862.yaml, issue-4008.yaml, issue-1550.yaml, tsconfig_off.json, test.ini, test.env
    - **Markup:** test.md, md-math.md, test-33886.md, test.rst, test.tex, test.sty, test.bib
    - **Other:** test.php, test.java, test.go, test.rb, test.r, test.pl, test2.pl, test.lua, test.clj, test.fs, test.dart, test.swift, test.m, test.mm, test.p6, test.handlebars, test.hbs, test.cshtml, test.bat, test.ps1, test-freeze-56476.ps1, test.shader, test.hlsl, test.sql, test.log, test.bib, test.coffee, test-regex.coffee, test.diff, Dockerfile, git-rebase-todo, COMMIT_EDITMSG, makefile, test.code-snippets, test-13777.go, test.jl, test.groovy, test.vb

### Result Snapshots
- `/home/norinlavaee/projects/vscode-atomic/extensions/vscode-colorize-tests/test/colorize-results/`
  - **108 JSON snapshot files** (one per fixture)
  - Each snapshot contains token array: `[{c: int, r: {fg?, bg?, fontStyle?}}, ...]`
  - c = token class/scopes, r = theme color info
  - Files named matching fixture inputs: test_cpp.json, test-7115_xml.json, test_mm.json, etc.

- `/home/norinlavaee/projects/vscode-atomic/extensions/vscode-colorize-tests/test/colorize-tree-sitter-results/`
  - **108 JSON snapshot files** (parallel to colorize-results)
  - Tree-sitter tokenization baseline (experimental feature under `editor.experimental.preferTreeSitter.*` config)
  - Allows parallel validation of both TextMate and Tree-sitter engines

### Semantic Token Test
- `/home/norinlavaee/projects/vscode-atomic/extensions/vscode-colorize-tests/test/semantic-test/`
  - `semantic-test.json` - JSON document with semantic token decoration markers
  - `.vscode/settings.json` - Local workspace config for semantic test isolation

---

## Configuration

### Build & Deployment
- `/home/norinlavaee/projects/vscode-atomic/extensions/vscode-colorize-tests/package.json`
  - Entry point: `./out/colorizerTestMain` (compiled TS)
  - Activation: `onLanguage:json` (semantic test provider trigger)
  - Build: `gulp compile-extension:vscode-colorize-tests` (tsconfig.json driven)
  - CI: Mocha + multi-reporter (spec + JUnit XML)
  - Contributes semantic token types/modifiers/scopes, product icon theme

- `/home/norinlavaee/projects/vscode-atomic/extensions/vscode-colorize-tests/tsconfig.json`
  - Extends `tsconfig.base.json` (root)
  - rootDir: `./src`, outDir: `./out`
  - Includes `../../src/vscode-dts/vscode.d.ts` (API types)

### IDE/Debugger
- `/home/norinlavaee/projects/vscode-atomic/extensions/vscode-colorize-tests/.vscode/launch.json`
  - Launch config: "Launch Tests" 
  - Type: `extensionHost` (runs tests in VS Code host)
  - Args: Opens workspace root + test folder with extension enabled
  - Sourcemaps + out/ folder breakpoint support

- `/home/norinlavaee/projects/vscode-atomic/extensions/vscode-colorize-tests/.vscode/tasks.json`
  - Pre-launch task: `npm` (compile step)

### Misc
- `/home/norinlavaee/projects/vscode-atomic/extensions/vscode-colorize-tests/.npmrc` - NPM config
- `/home/norinlavaee/projects/vscode-colorize-tests/package-lock.json` - Locked deps (jsonc-parser 3.2.0, @types/node 22.x)

---

## Examples / Fixtures

**Fixture Naming Patterns:**

1. **Regression Issue-driven:** issue-XXX.{yaml,php,xml}
   - issue-1550.yaml, issue-4008.yaml, issue-6303.yaml, issue-224862.yaml, issue-28354.php, issue-76997.php, test-7115.xml

2. **Feature Test-driven:** test-XXX.{ts,js,cpp,rs,py,sh}
   - test-241001.ts, test-23850.cpp, test-166781.rs, test-freeze-56377.py, test-173216.sh, test-freeze-56476.ps1

3. **Language General:** test.{ext}
   - Basic corpus (100+ files, one or two per language)

4. **Special Cases:** test-embedding.html (nested language), test-jsdoc-multiline-type.ts (JSDoc edge cases), md-math.md (Markdown with math), test-brackets.tsx (bracket pairing)

**Product Icon Theme:**
- `/home/norinlavaee/projects/vscode-atomic/extensions/vscode-colorize-tests/producticons/test-product-icon-theme.json`
  - Icon theme contribution test fixture
- `/home/norinlavaee/projects/vscode-atomic/extensions/vscode-colorize-tests/producticons/ElegantIcons.woff` - Test icon font

---

## Notable Clusters

### Dual-Engine Tokenization Pipeline
The test suite captures and compares outputs from **two syntactic analysis engines:**

1. **TextMate (default):** `_workbench.captureSyntaxTokens` command
   - Snapshot results → `test/colorize-results/`
   - Grammar-based (XML TextMate grammar files in VS Code core)
   - Used for all 108 fixtures

2. **Tree-sitter (experimental):** `_workbench.captureTreeSitterSyntaxTokens` command
   - Snapshot results → `test/colorize-tree-sitter-results/`
   - Parallel parser library (WASM/native bindings in VS Code core)
   - Controlled by `editor.experimental.preferTreeSitter.*` config flags (typescript, ini, regex, css)
   - Line 74-80 in colorizer.test.ts

**Tauri/Rust Port Implication:**
- A Rust-based VS Code port must replicate both pipelines or choose one:
  - TextMate path: Integrate an existing TextMate grammar parser + theme applier (Rust crate: `text-mate` or similar)
  - Tree-sitter path: Leverage existing Rust Tree-sitter bindings, port grammar queries
  - Semantic tokens: Implement `SemanticTokensProvider` API (language-agnostic, defined in vscode.d.ts)

### Test Data Regression Coverage
- 108 fixtures = broad language coverage (40+ modes)
- 216 snapshots (TextMate + Tree-sitter) = two-engine baseline
- Test comparison: token sequence + color theme info (lines 34-37: tolerates color theme changes but fails on token boundary/scope shifts)

### CI/CD Integration
- Mocha suite runs in VS Code host (no standalone runner)
- JUnit XML export to `test-results/{platform}-{arch}-{suite}.xml`
- Used for build verification in Azure Pipelines / GitHub Actions

---

## Summary

**Partition 29 is primarily a **grammar fixture corpus + test harness** with minimal IDE-runtime logic.**

The 502 lines of TypeScript code comprise:
- **~73 LOC:** Semantic token provider (index.ts) - VSCode API contract
- **~97 LOC:** Core regression test loop (colorizer.test.ts) - snapshot comparison
- **~32 LOC:** Mocha orchestration (colorizerTestMain.ts) - CI plumbing
- **~300 LOC:** Remaining stubs/config

**Key insight for Tauri/Rust port:** This partition does NOT contain IDE runtime code. However, it documents the **textual colorization pipeline** that a Rust host must reproduce:

1. **Grammar Engines:** TextMate (regex-based) + Tree-sitter (parsed AST)
2. **Token Output Format:** Scopes string (e.g., "source.ts keyword.control") → token type/modifiers → theme colors
3. **Semantic Tokens API:** Parallel mechanism for LSP-driven highlighting (language servers emit token ranges + types)
4. **Test Infrastructure:** Uses VS Code commands (`_workbench.captureSyntaxTokens`, `_workbench.captureTreeSitterSyntaxTokens`) to validate output

**For a Rust rewrite, prioritize:**
- Grammar parser integration (TextMate or Tree-sitter, ideally both)
- Semantic token provider registration mechanism
- Theme color application layer (maps token scope → theme variable)
- Regression test harness (corpus + snapshot comparison still valid)

The fixture corpus itself (108 files across 40+ languages) is reusable as golden data for validating colorization parity.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
_(no analysis produced)_

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Colorization Pipeline Patterns for Tauri/Rust Port

## Research Context
This scope partition contains colorize/grammar fixtures with the test harness for VS Code's tokenization pipeline. The fixtures are test source files (TypeScript, JavaScript, CSS, etc.), but the patterns found reveal the **tokenization harness invocation**, **snapshot testing strategy**, and **dual tokenizer architecture** (TextMate + Tree-sitter).

---

## Pattern 1: Dual Tokenizer Snapshot Test Harness
**Where:** `/home/norinlavaee/projects/vscode-atomic/extensions/vscode-colorize-tests/src/colorizer.test.ts:12-46`

**What:** Core test infrastructure that captures token output from two parallel tokenizers (TextMate and Tree-sitter), persists snapshots as JSON, and compares with stored baselines while tolerating theme-only changes.

```typescript
async function assertUnchangedTokens(fixturesPath: string, resultsPath: string, treeSitterResultsPath: string, fixture: string, done: any) {
	const testFixurePath = join(fixturesPath, fixture);
	const tokenizers = [{ command: '_workbench.captureSyntaxTokens', resultsPath }, { command: '_workbench.captureTreeSitterSyntaxTokens', resultsPath: treeSitterResultsPath }];

	try {
		await Promise.all(tokenizers.map(async (tokenizer) => {
			const data = await commands.executeCommand(tokenizer.command, Uri.file(testFixurePath));

			if (!fs.existsSync(tokenizer.resultsPath)) {
				fs.mkdirSync(tokenizer.resultsPath);
			}
			const resultPath = join(tokenizer.resultsPath, fixture.replace('.', '_') + '.json');
			if (fs.existsSync(resultPath)) {
				const previousData = JSON.parse(fs.readFileSync(resultPath).toString());
				try {
					assert.deepStrictEqual(data, previousData);
				} catch (e) {
					fs.writeFileSync(resultPath, JSON.stringify(data, null, '\t'), { flag: 'w' });
					if (Array.isArray(data) && Array.isArray(previousData) && data.length === previousData.length) {
						for (let i = 0; i < data.length; i++) {
							const d = data[i];
							const p = previousData[i];
							if (d.c !== p.c || hasThemeChange(d.r, p.r)) {
								throw e;
							}
						}
						// different but no tokenization or color change: no failure
					} else {
						throw e;
					}
				}
			} else {
				fs.writeFileSync(resultPath, JSON.stringify(data, null, '\t'));
			}
		}));
		done();
	} catch (e) {
		done(e);
	}
}
```

**Variations:**
- Two separate result directories: `colorize-results/` for TextMate, `colorize-tree-sitter-results/` for Tree-sitter
- Snapshot files named `fixture_name_replaced_dots.json`
- Deep equality check with fallback logic: rejects if token content (`d.c`) or theme colors (`d.r`) differ, but tolerates color format/theme changes alone

---

## Pattern 2: Snapshot JSON Token Format
**Where:** `/home/norinlavaee/projects/vscode-atomic/test/colorize-results/test_css.json:1-50`

**What:** Standardized token snapshot structure with three fields: character content, scope chain (TextMate-style), and per-theme color mapping.

```json
[
	{
		"c": "/*",
		"t": "source.css comment.block.css punctuation.definition.comment.begin.css",
		"r": {
			"dark_plus": "comment: #6A9955",
			"light_plus": "comment: #008000",
			"dark_vs": "comment: #6A9955",
			"light_vs": "comment: #008000",
			"hc_black": "comment: #7CA668",
			"dark_modern": "comment: #6A9955",
			"hc_light": "comment: #515151",
			"light_modern": "comment: #008000",
			"2026-dark": "punctuation.definition.comment: #8B949E",
			"2026-light": "punctuation.definition.comment: #6E7781"
		}
	}
]
```

**Variations:**
- `c`: Raw token character content (contiguous same-scoped tokens merged)
- `t`: Space-separated scope chain from general to specific (e.g., `source.css > comment.block.css > punctuation.definition.comment`)
- `r`: Per-theme color overrides; maps theme name to `"scope.selector: #HEXCOLOR"` string or undefined

---

## Pattern 3: TextMate Tokenizer Implementation (Snapper._tokenize)
**Where:** `/home/norinlavaee/projects/vscode-atomic/src/vs/workbench/contrib/themes/browser/themes.test.contribution.ts:159-195`

**What:** State-machine tokenization using TextMate grammar with line-by-line processing, scope tracking, and adjacent token merging.

```typescript
private _tokenize(grammar: IGrammar, lines: string[]): IToken[] {
	let state: StateStack | null = null;
	const result: IToken[] = [];
	let resultLen = 0;
	for (let i = 0, len = lines.length; i < len; i++) {
		const line = lines[i];

		const tokenizationResult = grammar.tokenizeLine(line, state);
		let lastScopes: string | null = null;

		for (let j = 0, lenJ = tokenizationResult.tokens.length; j < lenJ; j++) {
			const token = tokenizationResult.tokens[j];
			const tokenText = line.substring(token.startIndex, token.endIndex);
			const tokenScopes = token.scopes.join(' ');

			if (lastScopes === tokenScopes) {
				result[resultLen - 1].c += tokenText;
			} else {
				lastScopes = tokenScopes;
				result[resultLen++] = {
					c: tokenText,
					t: tokenScopes,
					r: { dark_plus: undefined, light_plus: undefined, dark_vs: undefined, light_vs: undefined, hc_black: undefined }
				};
			}
		}

		state = tokenizationResult.ruleStack;
	}
	return result;
}
```

**Variations:**
- Maintains `StateStack` across lines (stateful parsing required for multi-line constructs)
- Token merging logic: consecutive tokens with identical scopes are concatenated
- Scope array converted to space-separated string

---

## Pattern 4: Tree-sitter Tokenizer Implementation (Snapper._treeSitterTokenize)
**Where:** `/home/norinlavaee/projects/vscode-atomic/src/vs/workbench/contrib/themes/browser/themes.test.contribution.ts:296-356`

**What:** Tree-sitter AST-based tokenization using cursor traversal, support for language injections, and metadata aggregation.

```typescript
private async _treeSitterTokenize(treeSitterTree: TreeSitterTree, tokenizationModel: TreeSitterTokenizationImpl, languageId: string): Promise<IToken[]> {
	const tree = await waitForState(treeSitterTree.tree);
	if (!tree) {
		return [];
	}
	// [cursor navigation and injection handling omitted for brevity]
	// Cursor-based tree traversal, child/sibling navigation
	const tokens: IToken[] = [];
	// Loop through tree nodes, extract scopes, aggregate text
	cursor.delete();
	return tokens;
}
```

**Variations:**
- Asynchronous initialization waiting for tree state
- Injection cursor handling: navigates tree to stay within injection range
- Returns raw token array without state carryover (AST is self-contained)

---

## Pattern 5: Theme-Aware Token Enrichment
**Where:** `/home/norinlavaee/projects/vscode-atomic/src/vs/workbench/contrib/themes/browser/themes.test.contribution.ts:260-282`

**What:** Post-processing loop that matches tokenized output against all default themes and enriches snapshot with per-theme color explanations.

```typescript
private _enrichResult(result: IToken[], themesResult: IThemesResult): void {
	const index: { [themeName: string]: number } = {};
	const themeNames = Object.keys(themesResult);
	for (const themeName of themeNames) {
		index[themeName] = 0;
	}

	for (let i = 0, len = result.length; i < len; i++) {
		const token = result[i];

		for (const themeName of themeNames) {
			const themedToken = themesResult[themeName].tokens[index[themeName]];

			themedToken.text = themedToken.text.substr(token.c.length);
			if (themedToken.color) {
				token.r[themeName] = themesResult[themeName].document.explainTokenColor(token.t, themedToken.color);
			}
			if (themedToken.text.length === 0) {
				index[themeName]++;
			}
		}
	}
}
```

**Variations:**
- Parallel indices tracking for each theme to synchronize with token stream
- Color lookup via `ThemeDocument.explainTokenColor()` which resolves scope → rule → hex + label
- Handles partial token consumption (text.substr advances through themed token buffer)

---

## Pattern 6: Dual Snapshot Execution Strategy
**Where:** `/home/norinlavaee/projects/vscode-atomic/extensions/vscode-colorize-tests/src/colorizer.test.ts:89-95`

**What:** Dynamic test generation from fixture directory, executing both tokenizers in parallel and asserting against independent snapshots.

```typescript
suite('colorization', () => {
	const testPath = normalize(join(__dirname, '../test'));
	const fixturesPath = join(testPath, 'colorize-fixtures');
	const resultsPath = join(testPath, 'colorize-results');
	const treeSitterResultsPath = join(testPath, 'colorize-tree-sitter-results');
	// ... suite setup/teardown ...

	for (const fixture of fs.readdirSync(fixturesPath)) {
		test(`colorize: ${fixture}`, function (done) {
			commands.executeCommand('workbench.action.closeAllEditors').then(() => {
				assertUnchangedTokens(fixturesPath, resultsPath, treeSitterResultsPath, fixture, done);
			});
		});
	}
});
```

**Variations:**
- Test suite generated by directory scan (fixture discovery)
- Editor cleanup before tokenization to ensure consistent state
- Configuration setup in suiteSetup enables Tree-sitter for TS, INI, regex, CSS

---

## Pattern 7: Semantic Tokens Provider Registration
**Where:** `/home/norinlavaee/projects/vscode-atomic/extensions/vscode-colorize-tests/src/colorizerTestMain.ts:9-71`

**What:** Extension manifest pattern for registering custom semantic token types/modifiers and language-specific tokenization providers.

```typescript
const tokenTypes = ['type', 'struct', 'class', 'interface', 'enum', 'parameterType', 'function', 'variable', 'testToken'];
const tokenModifiers = ['static', 'abstract', 'deprecated', 'declaration', 'documentation', 'member', 'async', 'testModifier'];
const legend = new vscode.SemanticTokensLegend(tokenTypes, tokenModifiers);

const documentSemanticHighlightProvider: vscode.DocumentSemanticTokensProvider = {
	provideDocumentSemanticTokens(document: vscode.TextDocument): vscode.ProviderResult<vscode.SemanticTokens> {
		const builder = new vscode.SemanticTokensBuilder();
		// ... token aggregation via builder.push(line, char, length, tokenType, tokenModifiers)
		return builder.build();
	}
};

context.subscriptions.push(vscode.languages.registerDocumentSemanticTokensProvider(
	{ pattern: '**/*semantic-test.json' }, 
	documentSemanticHighlightProvider, 
	legend
));
```

**Variations:**
- Token type/modifier arrays define extensible legend
- SemanticTokensBuilder for batch token registration
- Pattern-based provider registration by file glob

---

## Summary: Rust Port Implications

The colorize test extension reveals **three tiers** of tokenization:

1. **TextMate Grammar Layer**: Stateful line-by-line tokenization with incremental scope tracking. Requires regex engine and rule stacking. ~159-195 lines to port.

2. **Tree-sitter AST Layer**: Parallel cursor-based traversal with injection support. Requires WASM Tree-sitter bindings or native Rust port. ~296-356 lines to port.

3. **Snapshot & Theme Infrastructure**: Persistence layer matching tokens against multiple themes. Requires theme resolution and scope-to-color mapping. ~260-282 lines for enrichment alone.

**Key architectural requirements for Rust port:**
- Dual tokenizer backends must operate independently and produce comparable IPC/JSON output
- Snapshot format (c/t/r fields) is canonical; theme color lookup must replicate via scope matching
- State machine for TextMate (StateStack) maps to Rust enum-based parser state
- Tree-sitter cursor API translates to Rust tree traversal with optional injection handling
- Test harness can spawn two independent tokenizer processes and compare JSON snapshots

**Fixture corpus size:** ~69 files covering TypeScript, JavaScript, CSS, JSON, HTML, XML, Python, Go, Rust, and other major languages—representative of grammar complexity.

---

**Note:** This partition is primarily fixtures corpus. Actionable patterns are concentrated in the test harness implementation and the Snapper tokenizer logic, which establish the interface contract between tokenizer backends and the snapshot testing framework.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
