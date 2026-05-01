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
