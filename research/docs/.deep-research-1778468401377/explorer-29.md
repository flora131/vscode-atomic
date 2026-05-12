# Partition 29 of 80 — Findings

## Scope
`extensions/vscode-colorize-tests/` (69 files, 1,946 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# vscode-colorize-tests: File Locator Index

## Overview

The `extensions/vscode-colorize-tests/` directory (69 files, ~1,946 LOC) contains TextMate grammar and tokenization integration test infrastructure for VS Code's syntax highlighting layer. This extension validates that the editor's colorization/tokenization pipeline produces consistent output across different languages and grammar implementations.

## Implementation

- `extensions/vscode-colorize-tests/src/index.ts` - Test runner entry point; configures Mocha test framework and outputs JUnit XML for CI/CD environments
- `extensions/vscode-colorize-tests/src/colorizerTestMain.ts` - Main test suite entry; orchestrates test execution
- `extensions/vscode-colorize-tests/src/colorizer.test.ts` - Core test logic; compares current tokenization output against baseline results using `_workbench.captureSyntaxTokens` and `_workbench.captureTreeSitterSyntaxTokens` commands

## Configuration

- `extensions/vscode-colorize-tests/package.json` - Extension manifest defining semantic token types, modifiers, and product icon theme contribution
- `extensions/vscode-colorize-tests/tsconfig.json` - TypeScript configuration
- `extensions/vscode-colorize-tests/.npmrc` - NPM configuration
- `extensions/vscode-colorize-tests/package-lock.json` - Dependency lock file
- `extensions/vscode-colorize-tests/.vscode/launch.json` - Debug launch configuration
- `extensions/vscode-colorize-tests/.vscode/tasks.json` - VSCode build tasks

## Examples / Fixtures

### Source Fixtures (108 files in `test/colorize-fixtures/`)
Language-specific source files used as input for tokenization tests, covering 40+ languages:

- C/C++/C#: `test.c`, `test.cc`, `test.cpp`, `test-*.cpp` (multiple issue-specific fixtures)
- TypeScript/JavaScript: `test.ts`, `test.js`, `test.tsx`, `test-*.ts` (issue-specific variants), `test-issue*.ts`, `test.jsx`
- Python: `test.py`, `test-freeze-56377.py`
- Rust: `test.rs`, `test-6611.rs`, `test-166781.rs`
- Go: `test.go`, `test-13777.go`
- Java: `basic.java`
- C#: `test.cs`
- VB.NET: `test.vb`
- F#: `test.fs`
- PHP: `test.php`, `issue-28354.php`, `issue-76997.php`
- Web: `test.html`, `test.embedding.html`, `test-embedding.html`
- Styling: `test.css`, `test.scss`, `test.less`, `test-cssvariables.scss`, `test-cssvariables.less`, `test-variables.css`
- Markup: `test.xml`, `test-7115.xml`, `test.json`, `test-embedding.html`
- Templates: `test.handlebars`, `test.pug`, `test-4287.pug`, `test.cshtml`
- Scripting: `test.py`, `test.rb`, `test.lua`, `test.perl`, `test.sh`, `test-*.sh` (multiple shell variants), `test.bat`
- YAML: `test.yaml`, `issue-*.yaml` (multiple variants)
- Other: `test.dart`, `test.coffee`, `test-regex.coffee`, `test.clj`, `test.rs`, `test.m`, `test.mm`, `test.pl`, `test2.pl`, `test.p6`, `test.r`, `test.jl`, `test.rst`, `test.bib`, `test.log`, `test.ini`, `test.diff`, `Dockerfile`, `makefile`, `git-rebase-todo`, `COMMIT_EDITMSG`, `test.env`, `test.code-snippets`, `md-math.md`, `test-33886.md`
- Special: Issue-specific test cases like `test-241001.ts`, `test-function-inv.ts`, `test-jsdoc-multiline-type.ts`, `test-keywords.ts`, `test-members.ts`, `test-object-literals.ts`, `test-strings.ts`, `test-this.ts`, `test-brackets.tsx`, `test-issue11.ts`, `test-issue5431.ts`, `test-issue5465.ts`, `test-issue5566.ts`

### Expected Output Baselines

- `test/colorize-results/` (180+ JSON files) - TextMate tokenizer baseline outputs; one JSON file per fixture file
- `test/colorize-tree-sitter-results/` (180+ JSON files) - Tree-sitter tokenizer baseline outputs; parallel structure to TextMate results
- `test/semantic-test/semantic-test.json` - Semantic token highlighting test data

Format: Each JSON file contains an array of token objects with colorization and scope information, used for regression detection.

## Documentation

- `extensions/vscode-colorize-tests/producticons/mit_license.txt` - License for product icons

## Notable Clusters

**Dual Tokenization Tracking**
- The extension maintains separate baseline directories for two tokenization backends: TextMate (traditional) and Tree-sitter (modern). This dual-path architecture documents VS Code's shift toward Tree-sitter support for certain languages (TypeScript, CSS, regex, INI).

**Language Coverage**
- Comprehensive fixture library spanning 40+ programming and markup languages. Each language has at least one generic fixture (`test.*`) and multiple issue-specific variants capturing edge cases and past bug reports.

**Product Icon Theme Testing**
- `producticons/` directory with custom product icon theme for semantic token visual testing (`test-product-icon-theme.json`)

## Summary

This extension serves as a regression test harness for VS Code's tokenization layer. It captures syntax highlighting output for hundreds of language samples using two different tokenization engines (TextMate and Tree-sitter) and validates consistency against stored baselines. For a Tauri/Rust port, this directory documents the exact tokenization contract the editor must honor: what token types and scopes every supported language must produce. The dual-tokenization architecture signals that a production implementation would need either TextMate grammar support (complex, language-specific) or Tree-sitter integration (modern, unified parsing), or both. The fixture files themselves are valuable specifications of language syntax that must continue to tokenize correctly.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
_(no analysis produced)_

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Pattern Finder Report: TextMate Tokenizer Test Harness
## Porting VS Code's Colorization from TypeScript/Electron to Tauri/Rust

### Summary

The `extensions/vscode-colorize-tests/` extension (69 files, ~502 LOC test/fixture code) is a **fixture-driven test suite** with minimal test harness code. It validates TextMate grammar tokenization across VS Code's internal tokenizer and the newer Tree-Sitter parser, comparing outputs against baseline JSON snapshots. The patterns found are those directly relevant to what a Rust/Tauri port must maintain for compatibility.

---

## Patterns Found

### Pattern 1: Dual-Tokenizer Fixture-Based Test Harness
**Where:** `src/colorizer.test.ts:12-51`
**What:** Tests both TextMate (internal tokenizer) and Tree-Sitter tokenizers against identical fixtures, capturing token sequences and comparing to baseline snapshots.

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

**Key aspects:**
- Executes two commands: `_workbench.captureSyntaxTokens` (TextMate) and `_workbench.captureTreeSitterSyntaxTokens` (Tree-Sitter) against the same fixture file.
- Stores results separately (`colorize-results/` and `colorize-tree-sitter-results/`).
- Deep-equality comparison; tolerates theme-only changes but fails on tokenization differences (allowing theme-color mismatches when token boundaries remain identical).
- Dynamically creates baseline files if missing, updates them on assertion failure.

---

### Pattern 2: Token Output Format
**Where:** `test/colorize-results/test_ts.json` (representative example lines 1-60)
**What:** Token data structure with character content, TextMate scope chain, and per-theme color mappings.

```json
[
	{
		"c": "/*",
		"t": "source.ts comment.block.ts punctuation.definition.comment.ts",
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

**Key aspects:**
- `"c"` field: actual character content (substring of source).
- `"t"` field: space-separated TextMate scope chain (e.g., `source.ts comment.block.ts punctuation.definition.comment.ts`).
- `"r"` field: object mapping theme names to `"scope-name: #HEX"` color assignments (8+ theme variants tested: dark_plus, light_plus, dark_vs, light_vs, hc_black, dark_modern, hc_light, light_modern, 2026-dark, 2026-light).

---

### Pattern 3: Configuration-Driven Parser Selection
**Where:** `src/colorizer.test.ts:70-87`
**What:** Test suite setup/teardown manages VS Code experimental configuration to toggle between TextMate and Tree-Sitter parsers for specific languages.

```typescript
suiteSetup(async function () {
	originalSettingValues = [
		workspace.getConfiguration('editor.experimental').get('preferTreeSitter.typescript'),
		workspace.getConfiguration('editor.experimental').get('preferTreeSitter.ini'),
		workspace.getConfiguration('editor.experimental').get('preferTreeSitter.regex'),
		workspace.getConfiguration('editor.experimental').get('preferTreeSitter.css')
	];
	await workspace.getConfiguration('editor.experimental').update('preferTreeSitter.typescript', true, ConfigurationTarget.Global);
	await workspace.getConfiguration('editor.experimental').update('preferTreeSitter.ini', true, ConfigurationTarget.Global);
	await workspace.getConfiguration('editor.experimental').update('preferTreeSitter.regex', true, ConfigurationTarget.Global);
	await workspace.getConfiguration('editor.experimental').update('preferTreeSitter.css', true, ConfigurationTarget.Global);
});
suiteTeardown(async function () {
	await workspace.getConfiguration('editor.experimental').update('preferTreeSitter.typescript', originalSettingValues[0], ConfigurationTarget.Global);
	await workspace.getConfiguration('editor.experimental').update('preferTreeSitter.ini', originalSettingValues[1], ConfigurationTarget.Global);
	await workspace.getConfiguration('editor.experimental').update('preferTreeSitter.regex', originalSettingValues[2], ConfigurationTarget.Global);
	await workspace.getConfiguration('editor.experimental').update('preferTreeSitter.css', originalSettingValues[3], ConfigurationTarget.Global);
});
```

**Key aspects:**
- Settings keys: `editor.experimental.preferTreeSitter.[language]` (supports ts, ini, regex, css at minimum).
- Preserves original values and restores after test suite.
- Enables Tree-Sitter for specific languages to test both tokenization paths.

---

### Pattern 4: Fixture-to-Results Mapping
**Where:** `src/colorizer.test.ts:63-96`
**What:** Dynamic test generation iterating over fixture files and comparing against baseline results.

```typescript
suite('colorization', () => {
	const testPath = normalize(join(__dirname, '../test'));
	const fixturesPath = join(testPath, 'colorize-fixtures');
	const resultsPath = join(testPath, 'colorize-results');
	const treeSitterResultsPath = join(testPath, 'colorize-tree-sitter-results');
	let originalSettingValues: any[];

	// ... setup/teardown ...

	for (const fixture of fs.readdirSync(fixturesPath)) {
		test(`colorize: ${fixture}`, function (done) {
			commands.executeCommand('workbench.action.closeAllEditors').then(() => {
				assertUnchangedTokens(fixturesPath, resultsPath, treeSitterResultsPath, fixture, done);
			});
		});
	}
});
```

**Key aspects:**
- Fixture files in `test/colorize-fixtures/` (any extension: `.ts`, `.js`, `.json`, `.xml`, `.cpp`, etc.).
- One test case per fixture: `colorize: <fixture-filename>`.
- Baseline results stored in `test/colorize-results/` and `test/colorize-tree-sitter-results/` with names like `test_js.json`, `test-241001_ts.json` (dots replaced with underscores in JSON filenames).
- Closes all editors before each test to ensure clean state.

---

### Pattern 5: Semantic Tokens Provider Registration
**Where:** `src/index.ts:9-72`
**What:** Extension registers a semantic tokens provider parsing JSON and emitting token ranges with types and modifiers.

```typescript
const tokenTypes = ['type', 'struct', 'class', 'interface', 'enum', 'parameterType', 'function', 'variable', 'testToken'];
const tokenModifiers = ['static', 'abstract', 'deprecated', 'declaration', 'documentation', 'member', 'async', 'testModifier'];

const legend = new vscode.SemanticTokensLegend(tokenTypes, tokenModifiers);

const documentSemanticHighlightProvider: vscode.DocumentSemanticTokensProvider = {
	provideDocumentSemanticTokens(document: vscode.TextDocument): vscode.ProviderResult<vscode.SemanticTokens> {
		const builder = new vscode.SemanticTokensBuilder();

		function addToken(value: string, startLine: number, startCharacter: number, length: number) {
			const [type, ...modifiers] = value.split('.');
			const selectedModifiers = [];
			let tokenType = legend.tokenTypes.indexOf(type);
			if (tokenType === -1) {
				if (type === 'notInLegend') {
					tokenType = tokenTypes.length + 2;
				} else {
					return;
				}
			}
			let tokenModifiers = 0;
			for (const modifier of modifiers) {
				const index = legend.tokenModifiers.indexOf(modifier);
				if (index !== -1) {
					tokenModifiers = tokenModifiers | 1 << index;
					selectedModifiers.push(modifier);
				} else if (modifier === 'notInLegend') {
					tokenModifiers = tokenModifiers | 1 << (legend.tokenModifiers.length + 2);
					selectedModifiers.push(modifier);
				}
			}
			builder.push(startLine, startCharacter, length, tokenType, tokenModifiers);
		}

		const visitor: jsoncParser.JSONVisitor = {
			onObjectProperty: (property: string, _offset: number, _length: number, startLine: number, startCharacter: number) => {
				addToken(property, startLine, startCharacter, property.length + 2);
			},
			onLiteralValue: (value: any, _offset: number, length: number, startLine: number, startCharacter: number) => {
				if (typeof value === 'string') {
					addToken(value, startLine, startCharacter, length);
				}
			}
		};
		jsoncParser.visit(document.getText(), visitor);
		return builder.build();
	}
};

context.subscriptions.push(vscode.languages.registerDocumentSemanticTokensProvider({ pattern: '**/*semantic-test.json' }, documentSemanticHighlightProvider, legend));
```

**Key aspects:**
- Semantic tokens orthogonal to TextMate grammars; uses separate token type/modifier legend.
- Bitfield encoding: modifiers as bit flags (`1 << index`).
- Matches files with pattern `**/*semantic-test.json`.
- Handles undefined tokens (`notInLegend`) by mapping to synthetic IDs beyond the legend size.

---

### Pattern 6: Test Runner Configuration
**Where:** `src/colorizerTestMain.ts:9-32`
**What:** Mocha test runner setup with platform-specific CI reporting.

```typescript
const suite = 'Integration Colorize Tests';

const options: import('mocha').MochaOptions = {
	ui: 'tdd',
	color: true,
	timeout: 60000
};

if (process.env.BUILD_ARTIFACTSTAGINGDIRECTORY) {
	options.reporter = 'mocha-multi-reporters';
	options.reporterOptions = {
		reporterEnabled: 'spec, mocha-junit-reporter',
		mochaJunitReporterReporterOptions: {
			testsuitesTitle: `${suite} ${process.platform}`,
			mochaFile: path.join(
				process.env.BUILD_ARTIFACTSTAGINGDIRECTORY || process.env.GITHUB_WORKSPACE || __dirname,
				`test-results/${process.platform}-${process.arch}-${suite.toLowerCase().replace(/[^\w]/g, '-')}-results.xml`)
		}
	};
}

testRunner.configure(options);
export = testRunner;
```

**Key aspects:**
- 60-second timeout per test (colorization can be slow).
- Conditional CI reporting: generates JUnit XML when `BUILD_ARTIFACTSTAGINGDIRECTORY` or `GITHUB_WORKSPACE` is set.
- Platform/architecture included in test result filenames for multi-platform CI.

---

## Fixture Inventory

The scope contains 20+ fixture source files representing diverse languages and edge cases:

| Fixture | Language | Purpose |
|---------|----------|---------|
| `test.js` | JavaScript | General syntax |
| `test.ts` | TypeScript | Type annotations, generics |
| `test-241001.ts` | TypeScript | Issue #241001 regression |
| `test-issue11.ts` | TypeScript | Issue #11 regression |
| `test-issue5431.ts` | TypeScript | Issue #5431 regression |
| `test-issue5465.ts` | TypeScript | Issue #5465 regression |
| `test-issue5566.ts` | TypeScript | Issue #5566 regression |
| `test-issue241715.ts` | TypeScript | Issue #241715 regression |
| `test-function-inv.ts` | TypeScript | Function invocation edge case |
| `test-jsdoc-multiline-type.ts` | TypeScript | JSDoc multiline types |
| `test-keywords.ts` | TypeScript | Keyword tokenization |
| `test-members.ts` | TypeScript | Member access patterns |
| `test-object-literals.ts` | TypeScript | Object literal syntax |
| `test-strings.ts` | TypeScript | String escaping & templates |
| `test-this.ts` | TypeScript | `this` binding contexts |
| `test.regexp.ts` | TypeScript | Regular expression literals |
| `test.json` | JSON | JSON syntax |
| `test6916.js` | JavaScript | Issue #6916 regression |
| Plus 30+ additional baselines in `colorize-results/` and `colorize-tree-sitter-results/` covering C++, C#, CSS, HTML, Markdown, XML, Python, Ruby, Go, Rust, Java, etc. |

---

## Key Design Requirements for Rust/Tauri Port

Based on the patterns observed, a Rust implementation must:

1. **Dual Tokenizer Path:**  
   Support both TextMate grammar-based tokenization (via OnigurumaRegistry or equivalent) and Tree-Sitter parsing (tree-sitter Rust bindings).

2. **Token Structure Compatibility:**  
   Output tokens with `{ character, scopes, theme_colors }` semantics:
   - **Scopes:** Space-separated TextMate scope chains (e.g., `source.ts comment.block.ts`).
   - **Theme mappings:** Per-theme color assignments from a fixed theme set (dark_plus, light_plus, dark_vs, light_vs, hc_black, dark_modern, hc_light, light_modern, 2026-dark, 2026-light).

3. **Configuration Management:**  
   Implement `editor.experimental.preferTreeSitter.[language]` settings to allow runtime parser switching per language.

4. **Fixture Validation Framework:**  
   Snapshot-based testing with:
   - Automatic baseline creation if missing.
   - Tolerance for theme-color-only changes (same scopes, different colors OK).
   - Strict failure on tokenization boundary changes.

5. **TextMate Grammar Compatibility:**  
   The tokenizer must faithfully implement TextMate grammar matching to produce identical scope chains as VS Code's internal tokenizer.

---

## Related Files & Line References

- **Test harness:** `/home/norinlavaee/projects/vscode-atomic/extensions/vscode-colorize-tests/src/colorizer.test.ts`
- **Extension activation:** `/home/norinlavaee/projects/vscode-atomic/extensions/vscode-colorize-tests/src/colorizerTestMain.ts`
- **Semantic provider:** `/home/norinlavaee/projects/vscode-colorize-tests/src/index.ts`
- **Fixtures:** `/home/norinlavaee/projects/vscode-atomic/extensions/vscode-colorize-tests/test/colorize-fixtures/` (20+ `.ts`, `.js`, `.json` files)
- **Baselines:** `/home/norinlavaee/projects/vscode-atomic/extensions/vscode-colorize-tests/test/colorize-results/` and `colorize-tree-sitter-results/` (40+ `.json` snapshots)

---

## Conclusion

The `vscode-colorize-tests` extension is **primarily a fixture repository** with a small but critical test harness. The harness validates tokenization determinism and theme color application without modification of token boundaries. For a Rust/Tauri port of VS Code's IDE, the core deliverable is ensuring that both the TextMate and Tree-Sitter tokenization paths produce identical token boundaries (scopes) across all these fixtures, while tolerating theme color variations that arise from color scheme changes. The snapshot-based validation pattern provides a concrete acceptance test for tokenizer correctness.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
