# VS Code Colorize Tests: Tokenization & Colorization Testing Patterns

## Research Focus
Patterns for fixture-driven tokenization tests, golden file validation, and TextMate grammar/TreeSitter tokenizer verification—critical for validating any Rust replacement of the tokenizer subsystem.

---

## Pattern 1: Golden File Token Capture and Assertion
**Where:** `src/vs/workbench/contrib/themes/browser/themes.test.contribution.ts:34-97`
**What:** Core token record structure (`IToken`) capturing tokenized content with scope chains and theme-aware color information. The pattern demonstrates how raw tokens are enriched with theme-specific foreground colors across multiple color schemes.

```typescript
interface IToken {
  c: string; // token content/text
  t: string; // space-separated scopes (most general to most specific)
  r: { [themeName: string]: string | undefined }; // theme -> "scope: #HEXCOLOR"
}

class ThemeDocument {
  constructor(theme: IWorkbenchColorTheme) {
    this._theme = theme;
    this._cache: { [scopes: string]: ThemeRule } = {};
    this._defaultColor = '#000000';
    // Extract default color from theme rules
    for (let i = 0; i < this._theme.tokenColors.length; i++) {
      const rule = this._theme.tokenColors[i];
      if (!rule.scope) {
        this._defaultColor = rule.settings.foreground!;
      }
    }
  }
  
  public explainTokenColor(scopes: string, color: Color): string {
    const matchingRule = this._findMatchingThemeRule(scopes);
    if (!matchingRule) {
      const expected = Color.fromHex(this._defaultColor);
      if (!color.equals(expected)) {
        throw new Error(`[${this._theme.label}]: Unexpected color ${Color.Format.CSS.formatHexA(color)} for ${scopes}. Expected default ${Color.Format.CSS.formatHexA(expected)}`);
      }
      return `default: ${Color.Format.CSS.formatHexA(color, true).toUpperCase()}`;
    }
    // Validate against theme rule and format explanation
    const expected = Color.fromHex(matchingRule.settings.foreground!);
    if (!color.equals(expected)) {
      throw new Error(`Unexpected color for ${scopes}`);
    }
    return `${matchingRule.rawSelector}: ${Color.Format.CSS.formatHexA(color, true).toUpperCase()}`;
  }
  
  private _findMatchingThemeRule(scopes: string): ThemeRule {
    if (!this._cache[scopes]) {
      this._cache[scopes] = findMatchingThemeRule(this._theme, scopes.split(' '))!;
    }
    return this._cache[scopes];
  }
}
```

**Variations / call-sites:**
- TextMate tokenization path: `_themedTokenize()` (line 110) uses `grammar.tokenizeLine2()` with metadata extraction
- TreeSitter tokenization path: `_themedTokenizeTreeSitter()` (line 140) operates on pre-tokenized arrays
- Token enrichment: `_enrichResult()` (line 260) maps theme results back to raw tokens
- Both paths validate token counts and colors match expected theme rules

---

## Pattern 2: Fixture-Driven Test Generation with Dynamic Test Suite
**Where:** `extensions/vscode-colorize-tests/src/colorizer.test.ts:63-96`
**What:** Mocha test suite dynamically generated from fixture files in directory. Each fixture file becomes a separate test case that captures syntax tokens through internal VS Code commands and compares against golden files.

```typescript
suite('colorization', () => {
  const testPath = normalize(join(__dirname, '../test'));
  const fixturesPath = join(testPath, 'colorize-fixtures');
  const resultsPath = join(testPath, 'colorize-results');
  const treeSitterResultsPath = join(testPath, 'colorize-tree-sitter-results');
  let originalSettingValues: any[];

  suiteSetup(async function () {
    // Capture original TreeSitter preferences
    originalSettingValues = [
      workspace.getConfiguration('editor.experimental').get('preferTreeSitter.typescript'),
      workspace.getConfiguration('editor.experimental').get('preferTreeSitter.ini'),
      workspace.getConfiguration('editor.experimental').get('preferTreeSitter.regex'),
      workspace.getConfiguration('editor.experimental').get('preferTreeSitter.css')
    ];
    // Enable TreeSitter tokenizers for test
    await workspace.getConfiguration('editor.experimental').update('preferTreeSitter.typescript', true, ConfigurationTarget.Global);
    await workspace.getConfiguration('editor.experimental').update('preferTreeSitter.ini', true, ConfigurationTarget.Global);
    await workspace.getConfiguration('editor.experimental').update('preferTreeSitter.regex', true, ConfigurationTarget.Global);
    await workspace.getConfiguration('editor.experimental').update('preferTreeSitter.css', true, ConfigurationTarget.Global);
  });

  suiteTeardown(async function () {
    // Restore original settings
    await workspace.getConfiguration('editor.experimental').update('preferTreeSitter.typescript', originalSettingValues[0], ConfigurationTarget.Global);
    // ... restore others
  });

  // Dynamic test generation from fixture files
  for (const fixture of fs.readdirSync(fixturesPath)) {
    test(`colorize: ${fixture}`, function (done) {
      commands.executeCommand('workbench.action.closeAllEditors').then(() => {
        assertUnchangedTokens(fixturesPath, resultsPath, treeSitterResultsPath, fixture, done);
      });
    });
  }
});
```

**Variations / call-sites:**
- Test isolation: Close all editors before each test (prevents state bleeding)
- Configuration scope: ConfigurationTarget.Global applied per-test to enable both TextMate and TreeSitter paths
- Test count: 110+ fixture files (one per test) across multiple languages (TypeScript, JavaScript, CSS, Python, Rust, etc.)

---

## Pattern 3: Golden File Assertion with Graceful Theme-Only Degradation
**Where:** `extensions/vscode-colorize-tests/src/colorizer.test.ts:12-61`
**What:** Two-phase assertion strategy: exact token match on initial comparison, but gracefully allows "theme-only" changes (color values differ, but token content and scopes remain identical). This accommodates theme definition updates without invalidating tokenizer correctness.

```typescript
async function assertUnchangedTokens(fixturesPath: string, resultsPath: string, treeSitterResultsPath: string, fixture: string, done: any) {
  const testFixurePath = join(fixturesPath, fixture);
  const tokenizers = [
    { command: '_workbench.captureSyntaxTokens', resultsPath }, 
    { command: '_workbench.captureTreeSitterSyntaxTokens', resultsPath: treeSitterResultsPath }
  ];

  try {
    await Promise.all(tokenizers.map(async (tokenizer) => {
      // Execute command to capture tokens from file
      const data = await commands.executeCommand(tokenizer.command, Uri.file(testFixurePath));

      if (!fs.existsSync(tokenizer.resultsPath)) {
        fs.mkdirSync(tokenizer.resultsPath);
      }
      const resultPath = join(tokenizer.resultsPath, fixture.replace('.', '_') + '.json');
      
      if (fs.existsSync(resultPath)) {
        const previousData = JSON.parse(fs.readFileSync(resultPath).toString());
        try {
          // Phase 1: Exact deep equality check
          assert.deepStrictEqual(data, previousData);
        } catch (e) {
          // Phase 2: Token count mismatch or structural change -> fail
          fs.writeFileSync(resultPath, JSON.stringify(data, null, '\t'), { flag: 'w' });
          if (Array.isArray(data) && Array.isArray(previousData) && data.length === previousData.length) {
            for (let i = 0; i < data.length; i++) {
              const d = data[i];
              const p = previousData[i];
              // Check token content (c) or actual scope/color change (r)
              if (d.c !== p.c || hasThemeChange(d.r, p.r)) {
                throw e;
              }
            }
            // Different but no tokenization or color change: no failure
          } else {
            throw e;
          }
        }
      } else {
        // First run: write golden file
        fs.writeFileSync(resultPath, JSON.stringify(data, null, '\t'));
      }
    }));
    done();
  } catch (e) {
    done(e);
  }
}

function hasThemeChange(d: any, p: any): boolean {
  const keys = Object.keys(d);
  for (const key of keys) {
    if (d[key] !== p[key]) {
      return true;
    }
  }
  return false;
}
```

**Variations / call-sites:**
- File naming: `fixture.replace('.', '_') + '.json'` (e.g., `test.ts` → `test_ts.json`)
- Golden file persistence: Written to `colorize-results/` and `colorize-tree-sitter-results/` separately
- Assertion tolerance: Passes if token content and scope chain unchanged, even if theme colors differ
- Both tokenizers tested in parallel via `Promise.all()`

---

## Pattern 4: TextMate Tokenization with Scope Chain Accumulation
**Where:** `src/vs/workbench/contrib/themes/browser/themes.test.contribution.ts:159-195`
**What:** Line-by-line tokenization using TextMate grammar, with state machine (`StateStack`) maintaining context across lines. Adjacent tokens with identical scope chains are coalesced into single records to reduce JSON size.

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
        // Coalesce adjacent tokens with same scopes
        result[resultLen - 1].c += tokenText;
      } else {
        lastScopes = tokenScopes;
        result[resultLen++] = {
          c: tokenText,
          t: tokenScopes,
          r: {
            dark_plus: undefined,
            light_plus: undefined,
            dark_vs: undefined,
            light_vs: undefined,
            hc_black: undefined,
          }
        };
      }
    }

    state = tokenizationResult.ruleStack;
  }
  return result;
}
```

**Variations / call-sites:**
- Grammar interface: `IGrammar` from `vscode-textmate` library
- Scope chain format: Space-separated strings from most general to most specific (e.g., `"source.ts comment.block.ts"`)
- State persistence: `StateStack` object carries line-end state to next line
- Color map: Five core themes hardcoded initially (dark_plus, light_plus, dark_vs, light_vs, hc_black)

---

## Pattern 5: TreeSitter Tokenization with Syntax Tree Traversal and Language Injection
**Where:** `src/vs/workbench/contrib/themes/browser/themes.test.contribution.ts:296-356`
**What:** Tree-based tokenization using TreeSitter parse tree. Recursively walks AST using cursor, handles multi-language injection ranges (e.g., embedded SQL in strings), and captures scope names from tokenization model's capture groups.

```typescript
private async _treeSitterTokenize(treeSitterTree: TreeSitterTree, tokenizationModel: TreeSitterTokenizationImpl, languageId: string): Promise<IToken[]> {
  const tree = await waitForState(treeSitterTree.tree);
  if (!tree) {
    return [];
  }
  const cursor = tree.walk();
  cursor.gotoFirstChild();
  let cursorResult: boolean = true;
  const tokens: IToken[] = [];

  const cursors: { cursor: Parser.TreeCursor; languageId: string; startOffset: number; endOffset: number }[] = [
    { cursor, languageId, startOffset: 0, endOffset: treeSitterTree.textModel.getValueLength() }
  ];
  
  do {
    const current = cursors[cursors.length - 1];
    const currentCursor = current.cursor;
    const currentLanguageId = current.languageId;
    const isOutsideRange: boolean = (currentCursor.currentNode.endIndex > current.endOffset);

    if (!isOutsideRange && (currentCursor.currentNode.childCount === 0)) {
      const range = new Range(
        currentCursor.currentNode.startPosition.row + 1,
        currentCursor.currentNode.startPosition.column + 1,
        currentCursor.currentNode.endPosition.row + 1,
        currentCursor.currentNode.endPosition.column + 1
      );
      
      // Check for language injection (e.g., SQL in string literal)
      const injection = treeSitterTree.getInjectionTrees(currentCursor.currentNode.startIndex, currentLanguageId);
      const treeSitterRange = injection?.ranges!.find(r => 
        r.startIndex <= currentCursor.currentNode.startIndex && 
        r.endIndex >= currentCursor.currentNode.endIndex
      );

      const injectionTree = injection?.tree.get();
      const injectionLanguageId = injection?.languageId;
      if (injectionTree && injectionLanguageId && treeSitterRange && (treeSitterRange.startIndex === currentCursor.currentNode.startIndex)) {
        // Push injection cursor onto stack for recursive processing
        const injectionCursor = injectionTree.walk();
        this._moveInjectionCursorToRange(injectionCursor, treeSitterRange);
        cursors.push({ cursor: injectionCursor, languageId: injectionLanguageId, startOffset: treeSitterRange.startIndex, endOffset: treeSitterRange.endIndex });
        while ((currentCursor.endIndex <= treeSitterRange.endIndex) && (currentCursor.gotoNextSibling() || currentCursor.gotoParent())) { }
      } else {
        // Capture scope names from tokenization model
        const capture = tokenizationModel.captureAtRangeTree(range);
        tokens.push({
          c: currentCursor.currentNode.text.replace(/\r/g, ''),
          t: capture?.map(cap => cap.name).join(' ') ?? '',
          r: {
            dark_plus: undefined,
            light_plus: undefined,
            dark_vs: undefined,
            light_vs: undefined,
            hc_black: undefined,
          }
        });
        while (!(cursorResult = currentCursor.gotoNextSibling())) {
          if (!(cursorResult = currentCursor.gotoParent())) {
            break;
          }
        }
      }

    } else {
      cursorResult = currentCursor.gotoFirstChild();
    }
    
    if (cursors.length > 1 && ((!cursorResult && currentCursor === cursors[cursors.length - 1].cursor) || isOutsideRange)) {
      current.cursor.delete();
      cursors.pop();
      cursorResult = true;
    }
  } while (cursorResult);
  
  cursor.delete();
  return tokens;
}
```

**Variations / call-sites:**
- Language injection: Detected via `treeSitterTree.getInjectionTrees()` with range validation
- Cursor stack: Maintains stack of cursors for multi-language parsing
- Node text normalization: Strips `\r` carriage returns before storage
- Scope capture: Obtained from `tokenizationModel.captureAtRangeTree(range)` (matches TreeSitter queries)

---

## Pattern 6: Multi-Theme Result Enrichment
**Where:** `src/vs/workbench/contrib/themes/browser/themes.test.contribution.ts:197-257, 260-282`
**What:** After raw tokenization, iterates through all installed default themes, applies each, captures colored tokens, then maps colors back to token records. Produces theme-agnostic scope chains with per-theme color explanations.

```typescript
private async _getThemesResult(grammar: IGrammar, lines: string[]): Promise<IThemesResult> {
  const currentTheme = this.themeService.getColorTheme();

  const getThemeName = (id: string) => {
    const part = 'vscode-theme-defaults-themes-';
    const startIdx = id.indexOf(part);
    if (startIdx !== -1) {
      return id.substring(startIdx + part.length, id.length - 5);
    }
    return undefined;
  };

  const result: IThemesResult = {};

  const themeDatas = await this.themeService.getColorThemes();
  const defaultThemes = themeDatas.filter(themeData => !!getThemeName(themeData.id));
  for (const defaultTheme of defaultThemes) {
    const themeId = defaultTheme.id;
    const success = await this.themeService.setColorTheme(themeId, undefined);
    if (success) {
      const themeName = getThemeName(themeId);
      result[themeName!] = {
        document: new ThemeDocument(this.themeService.getColorTheme()),
        tokens: this._themedTokenize(grammar, lines)
      };
    }
  }
  await this.themeService.setColorTheme(currentTheme.id, undefined);
  return result;
}

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

**Variations / call-sites:**
- Theme filtering: Only processes themes with ID pattern `vscode-theme-defaults-themes-*`
- Theme switching: Via `themeService.setColorTheme()` (async, must restore after test)
- Color capture: Through `_themedTokenize()` which tokenizes with current theme applied
- Enrichment synchronization: Tracks token index per-theme to handle scope fragmentation differences
- Current themes (line 71): dark_plus, light_plus, dark_vs, light_vs, hc_black, dark_modern, hc_light, light_modern, 2026-dark, 2026-light (10 total)

---

## Pattern 7: Public API Commands for Token Capture
**Where:** `src/vs/workbench/contrib/themes/browser/themes.test.contribution.ts:402-447`
**What:** Registers two internal test commands (`_workbench.captureSyntaxTokens`, `_workbench.captureTreeSitterSyntaxTokens`) that allow test extension to invoke tokenization on arbitrary files. Commands are used by the test harness to generate golden files.

```typescript
async function captureTokens(accessor: ServicesAccessor, resource: URI | undefined, treeSitter: boolean = false) {
  const process = (resource: URI) => {
    const fileService = accessor.get(IFileService);
    const fileName = basename(resource);
    const snapper = accessor.get(IInstantiationService).createInstance(Snapper);

    return fileService.readFile(resource).then(content => {
      if (treeSitter) {
        return snapper.captureTreeSitterSyntaxTokens(resource, content.value.toString());
      } else {
        return snapper.captureSyntaxTokens(fileName, content.value.toString());
      }
    });
  };

  if (!resource) {
    const editorService = accessor.get(IEditorService);
    const file = editorService.activeEditor ? EditorResourceAccessor.getCanonicalUri(editorService.activeEditor, { filterByScheme: Schemas.file }) : null;
    if (file) {
      process(file).then(result => {
        console.log(result);
      });
    } else {
      console.log('No file editor active');
    }
  } else {
    const processResult = await process(resource);
    return processResult;
  }
  return undefined;
}

CommandsRegistry.registerCommand('_workbench.captureSyntaxTokens', function (accessor: ServicesAccessor, resource: URI) {
  return captureTokens(accessor, resource);
});

CommandsRegistry.registerCommand('_workbench.captureTreeSitterSyntaxTokens', function (accessor: ServicesAccessor, resource?: URI) {
  if (!resource) {
    const editorService = accessor.get(IEditorService);
    resource = editorService.activeEditor?.resource;
  }
  return captureTokens(accessor, resource, true);
});
```

**Variations / call-sites:**
- Command invocation: `await commands.executeCommand('_workbench.captureSyntaxTokens', Uri.file(path))`
- Resource handling: Both file URIs and active editor detection
- Language detection: Via `languageService.guessLanguageIdByFilepathOrFirstLine()`
- Model creation: TextMate path uses file name; TreeSitter path uses full resource URI
- Return type: `Promise<IToken[]>` (tokenization results array)

---

## Pattern 8: Semantic Tokens Provider Registration (Alternative Tokenization)
**Where:** `extensions/vscode-colorize-tests/src/colorizerTestMain.ts:9-73`
**What:** Extension registers a custom `DocumentSemanticTokensProvider` for `.json` test files, demonstrating how semantic token types and modifiers are defined, registered, and populated dynamically via legends. Shows complementary tokenization mechanism to syntax highlighting.

```typescript
export function activate(context: vscode.ExtensionContext): any {

  const tokenTypes = ['type', 'struct', 'class', 'interface', 'enum', 'parameterType', 'function', 'variable', 'testToken'];
  const tokenModifiers = ['static', 'abstract', 'deprecated', 'declaration', 'documentation', 'member', 'async', 'testModifier'];

  const legend = new vscode.SemanticTokensLegend(tokenTypes, tokenModifiers);

  const outputChannel = vscode.window.createOutputChannel('Semantic Tokens Test');

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

        outputChannel.appendLine(`line: ${startLine}, character: ${startCharacter}, length ${length}, ${type} (${tokenType}), ${selectedModifiers} ${tokenModifiers.toString(2)}`);
      }

      outputChannel.appendLine('---');

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
}
```

**Variations / call-sites:**
- Legend registration: Type/modifier arrays define indices (0-based) for semantic token encoding
- Scope string parsing: `'type.static.declaration'` → tokenType index + bit flags for each modifier
- Bit-flag encoding: `tokenModifiers |= 1 << index` for multi-modifier tokens
- Provider pattern: JSON visitor pattern for DOM traversal (alternative to direct text scanning)
- Configuration target: `**/*semantic-test.json` pattern (specific test file format)

---

## Pattern Data Summary

### Golden File Structure (JSON Array)
- **File format:** One entry per contiguous token sequence with identical scopes
- **Entry shape:** `{ c: string, t: string, r: { [theme]: string | undefined } }`
- **Size example:** `test_ts.json` = 16,609 lines for Conway Game of Life TypeScript code
- **Theme keys:** dark_plus, light_plus, dark_vs, light_vs, hc_black, dark_modern, hc_light, light_modern, 2026-dark, 2026-light (10 themes)
- **Scope chain:** Space-separated, e.g., `"source.ts comment.block.ts punctuation.definition.comment.ts"`

### Test Fixture Organization
- **Fixtures directory:** 110+ files across 20+ languages
- **Result directories:** 
  - `colorize-results/` (TextMate tokenization golden files)
  - `colorize-tree-sitter-results/` (TreeSitter tokenization golden files)
- **Languages tested:** TypeScript, JavaScript, CSS, Python, Rust, YAML, SQL, PHP, CoffeeScript, R, Clojure, Julia, Dart, Swift, HTML, XML, Regex, Shell, HLSL, HBS, CSHTML, etc.
- **Fixture naming:** File suffix used as test name (e.g., `test-issue5566.ts` → test case "colorize: test-issue5566.ts")

### Assertion Strategy
1. **Exact match (Phase 1):** `assert.deepStrictEqual(data, previousData)` on full token array
2. **Token content check (Phase 2):** If arrays same length, verify `d.c !== p.c` (tokenization changed?)
3. **Color check (Phase 2):** If all tokens same, verify `hasThemeChange(d.r, p.r)` (only theme colors changed?)
4. **Pass criteria:** Passes if token content + scope chains unchanged, even if theme colors differ
5. **First run:** Creates golden file on initial execution

### Tokenizer Invocation Paths
- **TextMate (deprecated):** Grammar-based, line-by-line state machine
- **TreeSitter (modern):** Tree-based AST traversal with capture groups
- **Both tested in parallel** via `Promise.all()` in test harness
- **Configuration gates:** `editor.experimental.preferTreeSitter.*` language-specific feature flags

---

## Implementation Notes for Rust Port

1. **Token Record Structure:** Must replicate `IToken` (content, scopes, theme colors). Consider struct with HashMap for themes.

2. **Scope Chain Representation:** Space-separated strings or Vec<&str>. Maintain scope hierarchy for theme matching.

3. **Golden File Format:** JSON serialization required for compatibility with existing test framework. Consider `serde_json`.

4. **Theme Color Matching:** Need color matching algorithm similar to `findMatchingThemeRule()`. Requires theme rule precedence logic.

5. **State Machine for TextMate:** Maintain per-language state stack across lines. Consider iterator pattern with lazy evaluation.

6. **TreeSitter Integration:** Tree cursor traversal, capture group evaluation, language injection handling. Requires `tree-sitter` crate bindings.

7. **Configuration Management:** Test configuration (`preferTreeSitter.*` flags) must be queryable from Rust layer. API boundary required.

8. **Command Registration:** Test commands (`_workbench.captureSyntaxTokens`) must remain callable from TypeScript/extension side. RPC or IPC boundary needed.

9. **Parallel Theme Processing:** Multiple themes can be processed in parallel. Consider thread pool for theme switching + color capture.

10. **Test Golden Files:** 110+ fixtures * 2 tokenizers = 220+ golden files to validate. Regeneration process required on breaking changes.

