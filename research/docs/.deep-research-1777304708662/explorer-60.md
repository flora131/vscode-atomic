# Partition 60 of 79 — Findings

## Scope
`extensions/php/` (1 files, 75 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# PHP Extension File Locations

## Implementation Files

- `extensions/php/package.json` - Extension manifest declaring language registration, grammar, and snippet contributions
- `extensions/php/language-configuration.json` - Language-specific configuration (comments, bracket matching, indentation, auto-closing pairs, folding markers)
- `extensions/php/syntaxes/php.tmLanguage.json` - TextMate grammar for PHP language syntax highlighting (4191 lines)
- `extensions/php/syntaxes/html.tmLanguage.json` - TextMate grammar for HTML with embedded PHP (97 lines)
- `extensions/php/snippets/php.code-snippets` - Code snippet definitions (50+ snippets for PHP constructs)

## Build/Tooling

- `extensions/php/build/update-grammar.mjs` - Grammar update utility using vscode-grammar-updater, manages adaptation of injection scopes and regex fixes for PHP grammar synchronization with external grammar source

## Configuration

- `extensions/php/package.nls.json` - Localization strings (display name and description)
- `extensions/php/cgmanifest.json` - Component governance manifest declaring external dependency (language-php v0.49.0 from GitHub)

## Development Configuration

- `extensions/php/.vscode/launch.json` - VS Code debug launch configuration for extension host testing
- `extensions/php/.vscode/tasks.json` - Build task configuration (npm compile)
- `extensions/php/.vscodeignore` - Package exclusion patterns (test/, build/, src/, tsconfig files, etc.)

## Notable Clusters

**Grammar and Syntax Architecture:**
- Two TextMate grammar files provide PHP syntax highlighting: standalone PHP and PHP embedded in HTML with language injections for SQL, XML, JavaScript, JSON, and CSS

**Snippet Coverage:**
- Comprehensive snippet set covering PHP 8+ features (attributes, enums, match expressions, spaceship operator) and classic constructs (classes, functions, loops, try-catch, traits)

**Grammar Synchronization:**
- Automated grammar update pipeline pulls from external KapitanOczywisty/language-php repository and applies custom patches (scope resolution, HTML grammar derivatives, regex case-sensitivity fixes)

## Directory Structure

```
extensions/php/
├── .vscode/                    # VS Code development configuration
├── build/                      # Build utilities (grammar updater)
├── snippets/                   # PHP code snippets
├── syntaxes/                   # TextMate grammar files
├── package.json                # Extension manifest
├── language-configuration.json # Language configuration
├── package.nls.json            # Localization
├── cgmanifest.json             # Component governance
└── .vscodeignore               # Package exclusions
```

## Summary

The PHP extension is a grammar-only language contribution providing syntax highlighting, bracket matching, and code snippets. It contains no implementation code or tests. The extension relies entirely on external TextMate grammar definitions (sourced from KapitanOczywisty/language-php) with automated synchronization and custom patches. Snippets cover modern PHP syntax including PHP 8+ features like attributes, enums, and match expressions alongside traditional PHP constructs.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
### Files Analysed

| File | Lines | Role |
|---|---|---|
| `extensions/php/package.json` | 69 | Extension manifest — language/grammar/snippet contribution points |
| `extensions/php/language-configuration.json` | 174 | Editor behaviour rules (comments, brackets, indentation, folding, on-enter) |
| `extensions/php/build/update-grammar.mjs` | 75 | Build-time grammar updater with upstream patches |
| `extensions/php/snippets/php.code-snippets` | 371 | ~35 tabstop-based code templates |
| `extensions/php/syntaxes/php.tmLanguage.json` | 4191 | Pure PHP TextMate grammar (`source.php`) |
| `extensions/php/syntaxes/html.tmLanguage.json` | 97 | HTML+PHP mixed-file TextMate grammar (`text.html.php`) |
| `extensions/php/package.nls.json` | 4 | Localisation strings for `displayName` and `description` |
| `extensions/php/cgmanifest.json` | 16 | Component governance — pins upstream `language-php` at commit `cd607a5` |

---

### Per-File Notes

#### `extensions/php/package.json` (lines 1–69)

- **Language registration** (lines 13–32): declares language id `"php"`, maps file extensions `.php`, `.php4`, `.php5`, `.phtml`, `.ctp`, the shebang first-line pattern `^#!\s*/.*\bphp\b` (line 27), and MIME type `application/x-php` (line 29). Points to `language-configuration.json` at line 31.
- **Grammar registrations** (lines 34–53): two grammars are contributed.
  - `source.php` scope at `./syntaxes/php.tmLanguage.json` (line 38) — standalone PHP scope used when the file is pure PHP.
  - `text.html.php` scope at `./syntaxes/html.tmLanguage.json` (lines 43–53) — HTML-embedded PHP. The `embeddedLanguages` map at lines 44–52 binds seven inner scopes (`text.html`, `source.php`, `source.sql`, `text.xml`, `source.js`, `source.json`, `source.css`) to their respective language ids, enabling tokenisation-based language switching inside `.php` HTML templates.
- **Snippet registration** (lines 55–59): `./snippets/php.code-snippets` linked to language id `"php"`.
- **Build script** (line 63): `"update-grammar": "node ./build/update-grammar.mjs"` — the only developer-facing script in the extension.

#### `extensions/php/language-configuration.json` (lines 1–174)

- **Comments** (lines 2–8): line comment `//` (with inline annotation noting `#` is also valid in PHP), block comment `/* … */`.
- **Brackets** (lines 9–22): three pairs `{}`, `[]`, `()`.
- **Auto-closing pairs** (lines 23–67): `{`, `[`, `(` excluded in `"string"` scope; single/double quotes excluded in both `"string"` and `"comment"` scopes; `/**` closes to ` */` outside strings (lines 62–67).
- **Surrounding pairs** (lines 69–94): same three bracket pairs plus `'`, `"`, and backtick.
- **Indentation rules** (lines 95–101):
  - `increaseIndentPattern` (line 96) matches `if`, `else`, `for`, `foreach`, `while`, `switch`, `case` followed by `:` on a line-end (covering PHP alternative syntax).
  - `decreaseIndentPattern` (line 97) matches `endif`, `endfor`, `endforeach`, `endwhile`, `endswitch` terminator keywords.
  - `unIndentedLinePattern` (line 99) handles continuation lines inside block comments.
  - `indentNextLinePattern` (line 100) applies single-level indent after braceless `if`/`else`/`while`/`for`/`foreach` lines.
- **Folding markers** (lines 102–107): start `#region` / `// #region`, end `#endregion` / `// #endregion`.
- **Word pattern** (line 108): captures floating-point literals and identifiers; excludes most punctuation characters.
- **On-enter rules** (lines 109–173): six rules.
  - Lines 110–117: inside `/** … */` (both before and after on same line) → `indentOutdent` + append ` * `.
  - Lines 118–124: at end of `/**` line → `none` + append ` * `.
  - Lines 125–134: inside ` * …` continuation → `none` + append `* `.
  - Lines 135–143: after ` */` → `none` + `removeText: 1`.
  - Lines 144–150: after ` *…*/` (inline close) → `none` + `removeText: 1`.
  - Lines 151–159: after single-line braceless control construct → `outdent`.
  - Lines 160–172: enter pressed within a `//` line comment when text follows cursor → `none` + append `// `.

#### `extensions/php/build/update-grammar.mjs` (lines 1–75)

The module imports `vscode-grammar-updater` (line 7), a shared VS Code build utility that fetches CSON grammar files from GitHub, converts them to JSON, applies a callback, and writes to the local `syntaxes/` path.

Three patch functions are defined before the two `update()` calls:

1. **`adaptInjectionScope`** (lines 9–22): modifies the injection scope key in the HTML grammar's `injections` map. The old key (line 12) used `source.js.embedded.html`; the new key (line 13) replaces it with `source.js` and adds an additional `source.css` injection scope. This enables PHP injection to apply inside `<script>` and `<style>` tags as they appear in VS Code's HTML grammar. The function throws if the expected old key is absent (line 18), acting as a structural assertion.

2. **`includeDerivativeHtml`** (lines 24–30): walks the `patterns` array of the HTML grammar and replaces any `include: 'text.html.basic'` with `include: 'text.html.derivative'`, directing the grammar to use VS Code's derivative HTML scope instead of the upstream TextMate HTML scope.

3. **`fixBadRegex`** (lines 32–69): works around issues filed as vscode#40279 and vscode-textmate#59. Both issues relate to the upstream grammar using `(?i)` inline case-insensitive flags in PCRE regex syntax which vscode-textmate's Oniguruma binding does not accept in the same way.
   - Patches `scope-resolution` (lines 39–49): replaces `(?i)([a-z_…)` with an explicit `[A-Za-z_…)` character class.
   - Patches `function-call` patterns[0].begin (lines 51–58): removes `(?xi)` and replaces `[a-z…` ranges with `[a-zA-Z…` ranges.
   - Patches `function-call` patterns[1].begin (lines 60–65): removes `(?i)` and expands character classes to include uppercase letters.

Two `vscodeGrammarUpdater.update()` calls (lines 71–75) drive the actual fetch:
- Line 71: fetches `grammars/php.cson` from `KapitanOczywisty/language-php`, writes `./syntaxes/php.tmLanguage.json`, applying `fixBadRegex`.
- Lines 72–75: fetches `grammars/html.cson`, writes `./syntaxes/html.tmLanguage.json`, applying both `adaptInjectionScope` and `includeDerivativeHtml`.

#### `extensions/php/snippets/php.code-snippets` (lines 1–371)

~35 named snippets for PHP 8.x idioms. Key entries:

- Lines 2–5: ternary `if?` → `$retVal = (condition) ? a : b ;`
- Lines 60–70: `class` snippet uses `TM_FILENAME_BASE` variable, optional `final`/`readonly`, `extends`, `implements`.
- Lines 71–78: `construct` generates `__construct` with `$this->var = $var` body.
- Lines 94–100: `enum` covers PHP 8.1 enum syntax.
- Lines 194–203: `match` generates PHP 8.0 match expression.
- Lines 214–246: `doc_class` / `doc_fun` snippets generate PHPDoc blocks with `@param`, `@return`, `@throws`.
- Lines 277–289: `#region` / `#endregion` markers matching the folding rules in `language-configuration.json`.
- Lines 329–339: `try` generates `try { } catch (\Throwable $th) { }` — uses the `\Throwable` base type.
- Lines 341–361: `use_fun`, `use_const`, `use_group`, `use_as` — namespace use statement variants.

#### `extensions/php/syntaxes/php.tmLanguage.json` (4191 lines)

- Header (lines 0–6): upstream commit `cd607a522b79c457fe78bf7a1b04511d1c49f693` from `KapitanOczywisty/language-php`, scope name `source.php`.
- Top-level `patterns` array (lines 8–onward) includes entries for `#attribute`, `#comments`, `#namespace`, and all other PHP constructs, delegating to the `repository` section via include references.
- The `scope-resolution` and `function-call` repository entries are the targets of the `fixBadRegex` patches in `update-grammar.mjs`.

#### `extensions/php/syntaxes/html.tmLanguage.json` (97 lines)

- Upstream commit `ff64523c94c014d68f5dec189b05557649c5872a`, scope name `text.html.php` (lines 6–8).
- `injections` (lines 9–27): two injection entries. The first (lines 10–18) injects `text.html.basic` and the Blade template grammar into `meta.embedded.php.blade` contexts. The second (lines 20–26) is the patched key from `adaptInjectionScope`, applying `#php-tag` patterns in HTML/JS/CSS regions.
- `patterns` (line 28+): includes `text.html.derivative` (patched by `includeDerivativeHtml`) and PHP-specific overrides.

---

### Cross-Cutting Synthesis

The `extensions/php` extension is a pure declarative language contribution with no runtime TypeScript code. Its entire surface consists of three categories of static data: TextMate grammars for tokenisation, a language configuration for editor mechanics, and code snippets for template insertion. The grammar layer is split into two scopes — `source.php` for standalone PHP files and `text.html.php` for HTML-embedded PHP — with the latter using the `embeddedLanguages` map in `package.json` to hand off inner regions (JavaScript, CSS, SQL, XML, JSON) to their respective language servers and tokenisers. The build script `update-grammar.mjs` acts as a thin integration layer: it pulls the upstream CSON grammar from the `KapitanOczywisty/language-php` repository, converts it, and applies three deterministic patches to fix regex syntax incompatibilities with vscode-textmate's Oniguruma engine and to align injection scope keys with VS Code's own HTML grammar scope names. No language intelligence (hover, completion, diagnostics) is provided here; that is delegated to the separate `extensions/php-language-features` extension which implements the PHP language server integration. In a Tauri/Rust port, the grammar data files (`.tmLanguage.json`) and snippet files are engine-agnostic and reusable directly; the language configuration rules would need to be parsed and applied by the Rust editor engine; and the build script's `vscode-grammar-updater` dependency would need a Rust or Node equivalent for grammar update workflows.

---

### Out-of-Partition References

- `extensions/php-language-features/` — the companion extension that provides IntelliSense, diagnostics, hover, and go-to-definition for PHP; consumes the `"php"` language id registered here.
- `extensions/html/syntaxes/` — registers `text.html.derivative` and `text.html.basic` scopes that `html.tmLanguage.json` includes after the `includeDerivativeHtml` patch (`update-grammar.mjs:25-29`).
- `extensions/vscode-colorize-tests/test/colorize-results/test_php.json`, `issue-28354_php.json`, `issue-76997_php.json` — integration colorisation test fixtures that validate token output from `source.php` and `text.html.php` grammars.
- `extensions/razor/build/update-grammar.mjs` — parallel build script using the same `vscode-grammar-updater` pattern for the Razor/C# HTML grammar, and also references `text.html.derivative`.
- `extensions/search-result/syntaxes/generateTMLanguage.js` — references `source.php` scope as one of the known language scopes in search result highlighting.
- `extensions/theme-monokai-dimmed/themes/dimmed-monokai-color-theme.json` and `extensions/theme-tomorrow-night-blue/themes/tomorrow-night-blue-color-theme.json` — both contain per-scope colour rules targeting `source.php` and `text.html.php` token scopes.

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# VS Code Language Extension Architecture Patterns
## PHP Extension Analysis (extensions/php/)

---

## Pattern Examples: Language Extension Architecture

### Pattern 1: Extension Manifest with Language Declaration
**Found in**: `extensions/php/package.json:1-70`
**Used for**: Declaring language support with multiple file extensions and grammar bindings

```json
{
  "name": "php",
  "displayName": "%displayName%",
  "description": "%description%",
  "version": "10.0.0",
  "publisher": "vscode",
  "license": "MIT",
  "engines": {
    "vscode": "0.10.x"
  },
  "categories": ["Programming Languages"],
  "contributes": {
    "languages": [
      {
        "id": "php",
        "extensions": [
          ".php",
          ".php4",
          ".php5",
          ".phtml",
          ".ctp"
        ],
        "aliases": [
          "PHP",
          "php"
        ],
        "firstLine": "^#!\\s*/.*\\bphp\\b",
        "mimetypes": [
          "application/x-php"
        ],
        "configuration": "./language-configuration.json"
      }
    ],
    "grammars": [
      {
        "language": "php",
        "scopeName": "source.php",
        "path": "./syntaxes/php.tmLanguage.json"
      },
      {
        "language": "php",
        "scopeName": "text.html.php",
        "path": "./syntaxes/html.tmLanguage.json",
        "embeddedLanguages": {
          "text.html": "html",
          "source.php": "php",
          "source.sql": "sql",
          "text.xml": "xml",
          "source.js": "javascript",
          "source.json": "json",
          "source.css": "css"
        }
      }
    ],
    "snippets": [
      {
        "language": "php",
        "path": "./snippets/php.code-snippets"
      }
    ]
  }
}
```

**Key aspects**:
- Multiple file extensions (.php, .php4, .php5, .phtml, .ctp)
- Shebang-based file detection via `firstLine`
- Two grammar scopes: pure PHP and HTML+PHP embedding
- Embedded language definitions for SQL, XML, JavaScript, JSON, CSS
- Centralized language configuration file reference
- Version targeting with `engines`

---

### Pattern 2: Language Configuration with Bracket Matching and Indentation Rules
**Found in**: `extensions/php/language-configuration.json:1-175`
**Used for**: Editor behavior specification including auto-closing, bracket matching, indentation

```json
{
	"comments": {
		"lineComment": "//",
		"blockComment": [
			"/*",
			"*/"
		]
	},
	"brackets": [
		[
			"{",
			"}"
		],
		[
			"[",
			"]"
		],
		[
			"(",
			")"
		]
	],
	"autoClosingPairs": [
		{
			"open": "{",
			"close": "}",
			"notIn": [
				"string"
			]
		},
		{
			"open": "[",
			"close": "]",
			"notIn": [
				"string"
			]
		},
		{
			"open": "(",
			"close": ")",
			"notIn": [
				"string"
			]
		},
		{
			"open": "'",
			"close": "'",
			"notIn": [
				"string",
				"comment"
			]
		},
		{
			"open": "\"",
			"close": "\"",
			"notIn": [
				"string",
				"comment"
			]
		}
	],
	"indentationRules": {
		"increaseIndentPattern": "((else\\s?)?if|else|for(each)?|while|switch|case).*:\\s*((/[/*].*|)?$|\\?>)",
		"decreaseIndentPattern": "^(.*\\*\\/)?\\s*(\\b(else:)|\\bend(if|for(each)?|while|switch);)",
		"unIndentedLinePattern": "^(\\t|[ ])*[ ]\\*[^/]*\\*/\\s*$|^(\\t|[ ])*[ ]\\*/\\s*$|^(\\t|[ ])*\\*([ ]([^\\*]|\\*(?!/))*)?$",
		"indentNextLinePattern": "^\\s*(((if|else ?if|while|for|foreach)\\s*\\(.*\\)\\s*)|else\\s*)$"
	},
	"folding": {
		"markers": {
			"start": "^\\s*(#|\/\/|\/\/ #)region\\b",
			"end": "^\\s*(#|\/\/|\/\/ #)endregion\\b"
		}
	}
}
```

**Key aspects**:
- Comment syntax declarations (line and block)
- Bracket pair declarations for matching/jumping
- Auto-closing pair rules with context filtering (notIn arrays)
- Regex-based indentation rules for language structures
- Custom folding markers (region/endregion)
- OnEnter rules for special formatting (doc comment continuation, indentation adjustments)

---

### Pattern 3: TextMate Grammar with Namespace and Use Statement Parsing
**Found in**: `extensions/php/syntaxes/php.tmLanguage.json:1-85`
**Used for**: Fine-grained syntax highlighting with scope names and pattern matching

```json
{
	"information_for_contributors": [
		"This file has been converted from https://github.com/KapitanOczywisty/language-php/blob/master/grammars/php.cson",
		"If you want to provide a fix or improvement, please create a pull request against the original repository.",
		"Once accepted there, we are happy to receive an update request."
	],
	"version": "https://github.com/KapitanOczywisty/language-php/commit/cd607a522b79c457fe78bf7a1b04511d1c49f693",
	"scopeName": "source.php",
	"patterns": [
		{
			"include": "#attribute"
		},
		{
			"include": "#comments"
		},
		{
			"match": "(?i)(?:^|(?<=<\\?php))\\s*(namespace)\\s+([a-z0-9_\\x{7f}-\\x{10ffff}\\\\]+)(?=\\s*;)",
			"name": "meta.namespace.php",
			"captures": {
				"1": {
					"name": "keyword.other.namespace.php"
				},
				"2": {
					"name": "entity.name.type.namespace.php",
					"patterns": [
						{
							"match": "\\\\",
							"name": "punctuation.separator.inheritance.php"
						}
					]
				}
			}
		},
		{
			"begin": "(?i)\\buse\\b",
			"beginCaptures": {
				"0": {
					"name": "keyword.other.use.php"
				}
			},
			"end": "(?<=})|(?=;)|(?=\\?>)",
			"name": "meta.use.php",
			"patterns": [
				{
					"match": "\\b(const|function)\\b"
				}
			]
		}
	]
}
```

**Key aspects**:
- Attribution to upstream source (KapitanOczywisty/language-php)
- Scope name hierarchy (meta.namespace.php, entity.name.type.namespace.php)
- Pattern includes and self-references
- Unicode character range support (\x{7f}-\x{10ffff})
- Named captures for granular scope assignment
- Begin/end block patterns for multi-line constructs

---

### Pattern 4: HTML+PHP Embedded Grammar with Language Injection
**Found in**: `extensions/php/syntaxes/html.tmLanguage.json:1-70`
**Used for**: Handling PHP embedded within HTML markup

```json
{
	"information_for_contributors": [
		"This file has been converted from https://github.com/KapitanOczywisty/language-php/blob/master/grammars/html.cson",
		"If you want to provide a fix or improvement, please create a pull request against the original repository.",
		"Once accepted there, we are happy to receive an update request."
	],
	"version": "https://github.com/KapitanOczywisty/language-php/commit/ff64523c94c014d68f5dec189b05557649c5872a",
	"name": "PHP",
	"scopeName": "text.html.php",
	"injections": {
		"L:meta.embedded.php.blade": {
			"patterns": [
				{
					"include": "text.html.basic"
				},
				{
					"include": "text.html.php.blade#blade"
				}
			]
		},
		"text.html.php - (meta.embedded | meta.tag), L:((text.html.php meta.tag) - (meta.embedded.block.php | meta.embedded.line.php)), L:(source.js - (meta.embedded.block.php | meta.embedded.line.php)), L:(source.css - (meta.embedded.block.php | meta.embedded.line.php))": {
			"patterns": [
				{
					"include": "#php-tag"
				}
			]
		}
	},
	"patterns": [
		{
			"begin": "\\A#!",
			"beginCaptures": {
				"0": {
					"name": "punctuation.definition.comment.php"
				}
			},
			"end": "$",
			"name": "comment.line.shebang.php"
		},
		{
			"include": "text.html.derivative"
		}
	],
	"repository": {
		"php-tag": {
			"patterns": [
				{
					"begin": "<\\?(?i:php|=)?(?![^?]*\\?>)",
					"beginCaptures": {
						"0": {
							"name": "punctuation.section.embedded.begin.php"
						}
					},
					"end": "(\\?)>",
					"endCaptures": {
						"0": {
							"name": "punctuation.section.embedded.end.php"
						}
					},
					"name": "meta.embedded.block.php",
					"contentName": "source.php",
					"patterns": [
						{
							"include": "source.php"
						}
					]
				}
			]
		}
	}
}
```

**Key aspects**:
- Grammar injections for language scoping (L: prefix denotes injection)
- Blade templating framework support
- Shebang detection for executable PHP files
- Repository pattern for reusable grammar rules
- Lookahead/lookbehind assertions for PHP tag boundary detection
- Nested pattern inclusion (source.php within HTML context)

---

### Pattern 5: Code Snippets with TextMate Variables and Placeholders
**Found in**: `extensions/php/snippets/php.code-snippets:1-250`
**Used for**: IntelliSense completions with parameter templates

```json
{
	"$… = ( … ) ? … : …": {
		"prefix": "if?",
		"body": "$${1:retVal} = (${2:condition}) ? ${3:a} : ${4:b} ;",
		"description": "Ternary conditional assignment"
	},
	"$… = array (…)": {
		"prefix": "array",
		"body": "$${1:arrayName} = array($0);",
		"description": "Array initializer"
	},
	"class …": {
		"prefix": "class",
		"body": [
			"${1:${2|final ,readonly |}}class ${3:${TM_FILENAME_BASE}}${4: extends ${5:AnotherClass}} ${6:implements ${7:Interface}}",
			"{",
			"\t$0",
			"}",
			""
		],
		"description": "Class definition"
	},
	"class __construct": {
		"prefix": "construct",
		"body": [
			"${1|public,private,protected|} function __construct(${2:${3:Type} $${4:var}${5: = ${6:null}}}$7) {",
			"\t\\$this->${4:var} = $${4:var};$0",
			"}"
		]
	},
	"class function …": {
		"prefix": "class_fun",
		"body": [
			"${1|public ,private ,protected |}${2: static }function ${3:FunctionName}(${4:${5:${6:Type} }$${7:var}${8: = ${9:null}}}$10) : ${11:Returntype}",
			"{",
			"\t${0:# code...}",
			"}"
		],
		"description": "Function for classes, traits and enums"
	},
	"do … while …": {
		"prefix": "do",
		"body": [
			"do {",
			"\t${0:# code...}",
			"} while (${1:$${2:a} <= ${3:10}});"
		],
		"description": "Do-While loop"
	},
	"for …": {
		"prefix": "for",
		"body": [
			"for ($${1:i}=${2:0}; $${1:i} < $3; $${1:i}++) { ",
			"\t${0:# code...}",
			"}"
		],
		"description": "For-loop"
	},
	"foreach …": {
		"prefix": "foreach",
		"body": [
			"foreach ($${1:variable} as $${2:key}${3: => $${4:value}}) {",
			"\t${0:# code...}",
			"}"
		],
		"description": "Foreach loop"
	},
	"PHPDoc function …": {
		"prefix": "doc_fun",
		"body": [
			"/**",
			" * ${1:undocumented function summary}",
			" *",
			" * ${2:Undocumented function long description}",
			" *",
			"${3: * @param ${4:Type} $${5:var} ${6:Description}}",
			"${7: * @return ${8:type}}",
			"${9: * @throws ${10:conditon}}",
			" **/",
			"${11:public }function ${12:FunctionName}(${13:${14:${4:Type} }$${5:var}${15: = ${16:null}}}17)",
			"{",
			"\t${0:# code...}",
			"}"
		],
		"description": "Documented function"
	}
}
```

**Key aspects**:
- Placeholder numbering for tab stops (${1:default}, ${2:...})
- Choice syntax with pipes (${1|option1,option2|})
- Variable substitution (${TM_FILENAME_BASE})
- Multi-line snippet bodies
- Descriptive labels for IntelliSense
- Reference to placeholders across lines (${4:var} reused multiple times)

---

### Pattern 6: Grammar Adaptation and Post-Processing Build Script
**Found in**: `extensions/php/build/update-grammar.mjs:1-76`
**Used for**: Updating grammar from upstream source with custom fixes

```javascript
/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/
//@ts-check

import * as vscodeGrammarUpdater from 'vscode-grammar-updater';

function adaptInjectionScope(grammar) {
	// we're using the HTML grammar from https://github.com/textmate/html.tmbundle which has moved away from source.js.embedded.html
	// also we need to add source.css scope for PHP code in <style> tags, which are handled differently in atom
	const oldInjectionKey = "text.html.php - (meta.embedded | meta.tag), L:((text.html.php meta.tag) - (meta.embedded.block.php | meta.embedded.line.php)), L:(source.js.embedded.html - (meta.embedded.block.php | meta.embedded.line.php))";
	const newInjectionKey = "text.html.php - (meta.embedded | meta.tag), L:((text.html.php meta.tag) - (meta.embedded.block.php | meta.embedded.line.php)), L:(source.js - (meta.embedded.block.php | meta.embedded.line.php)), L:(source.css - (meta.embedded.block.php | meta.embedded.line.php))";

	const injections = grammar.injections;
	const injection = injections[oldInjectionKey];
	if (!injection) {
		throw new Error("Can not find PHP injection to patch");
	}
	delete injections[oldInjectionKey];
	injections[newInjectionKey] = injection;
}

function includeDerivativeHtml(grammar) {
	grammar.patterns.forEach(pattern => {
		if (pattern.include === 'text.html.basic') {
			pattern.include = 'text.html.derivative';
		}
	});
}

// Workaround for https://github.com/microsoft/vscode/issues/40279
// and https://github.com/microsoft/vscode-textmate/issues/59
function fixBadRegex(grammar) {
	function fail(msg) {
		throw new Error(`fixBadRegex callback couldn't patch ${msg}. It may be obsolete`);
	}

	const scopeResolution = grammar.repository['scope-resolution'];
	if (scopeResolution) {
		const match = scopeResolution.patterns[0].match;
		if (match === '(?i)([a-z_\\x{7f}-\\x{10ffff}\\\\][a-z0-9_\\x{7f}-\\x{10ffff}\\\\]*)(?=\\s*::)') {
			scopeResolution.patterns[0].match = '([A-Za-z_\\x{7f}-\\x{10ffff}\\\\][A-Za-z0-9_\\x{7f}-\\x{10ffff}\\\\]*)(?=\\s*::)';
		} else {
			fail('scope-resolution.match');
		}
	} else {
		fail('scope-resolution');
	}

	const functionCall = grammar.repository['function-call'];
	if (functionCall) {
		const begin0 = functionCall.patterns[0].begin;
		if (begin0 === '(?xi)\n(\n  \\\\?(?<![a-z0-9_\\x{7f}-\\x{10ffff}])                            # Optional root namespace\n  [a-z_\\x{7f}-\\x{10ffff}][a-z0-9_\\x{7f}-\\x{10ffff}]*          # First namespace\n  (?:\\\\[a-z_\\x{7f}-\\x{10ffff}][a-z0-9_\\x{7f}-\\x{10ffff}]*)+ # Additional namespaces\n)\\s*(\\()') {
			functionCall.patterns[0].begin = '(?x)\n(\n  \\\\?(?<![a-zA-Z0-9_\\x{7f}-\\x{10ffff}])                            # Optional root namespace\n  [a-zA-Z_\\x{7f}-\\x{10ffff}][a-zA-Z0-9_\\x{7f}-\\x{10ffff}]*          # First namespace\n  (?:\\\\[a-zA-Z_\\x{7f}-\\x{10ffff}][a-zA-Z0-9_\\x{7f}-\\x{10ffff}]*)+ # Additional namespaces\n)\\s*(\\()';
		} else {
			fail('function-call.begin0');
		}

		const begin1 = functionCall.patterns[1].begin;
		if (begin1 === '(?i)(\\\\)?(?<![a-z0-9_\\x{7f}-\\x{10ffff}])([a-z_\\x{7f}-\\x{10ffff}][a-z0-9_\\x{7f}-\\x{10ffff}]*)\\s*(\\()') {
			functionCall.patterns[1].begin = '(\\\\)?(?<![a-zA-Z0-9_\\x{7f}-\\x{10ffff}])([a-zA-Z_\\x{7f}-\\x{10ffff}][a-zA-Z0-9_\\x{7f}-\\x{10ffff}]*)\\s*(\\()';
		} else {
			fail('function-call.begin1');
		}
	} else {
		fail('function-call');
	}
}

vscodeGrammarUpdater.update('KapitanOczywisty/language-php', 'grammars/php.cson', './syntaxes/php.tmLanguage.json', fixBadRegex);
vscodeGrammarUpdater.update('KapitanOczywisty/language-php', 'grammars/html.cson', './syntaxes/html.tmLanguage.json', grammar => {
	adaptInjectionScope(grammar);
	includeDerivativeHtml(grammar);
});
```

**Key aspects**:
- Grammar updater abstraction from vscode-grammar-updater package
- Callback-based grammar transformation pattern
- Validation/error-checking before modification (fail() guards)
- Case sensitivity fixes for unicode regex patterns
- Injection scope mapping and pattern include updates
- Upstream attribution and contribution guidelines embedded in code

---

### Pattern 7: Development and Deployment Configuration
**Found in**: `extensions/php/.vscode/launch.json:1-18` and `.vscode/tasks.json:1-11`
**Used for**: Extension development environment setup

```json
{
	"version": "0.2.0",
	"configurations": [
		{
			"name": "Launch Extension",
			"type": "extensionHost",
			"request": "launch",
			"runtimeExecutable": "${execPath}",
			"args": [
				"--extensionDevelopmentPath=${workspaceFolder}"
			],
			"stopOnEntry": false,
			"sourceMaps": true,
			"preLaunchTask": "npm"
		}
	]
}
```

```json
{
	"version": "2.0.0",
	"command": "npm",
	"type": "shell",
	"presentation": {
		"reveal": "silent"
	},
	"args": ["run", "compile"],
	"isBackground": true,
	"problemMatcher": "$tsc-watch"
}
```

**Key aspects**:
- extensionHost launch type for VS Code extension debugging
- Pre-launch task integration (npm compile)
- Source map support
- Background compilation with TypeScript watch problem matcher
- Silent output presentation

---

## Summary

The PHP extension demonstrates the foundational architecture for VS Code language support:

1. **Language Declaration Pattern**: Extensions declare supported file extensions, MIME types, and shebang patterns via `package.json` contributions.

2. **Configuration Layering**: Three-tier configuration model:
   - `package.json`: Extension metadata and contribution points
   - `language-configuration.json`: Editor behavior (indentation, brackets, comments, folding)
   - TextMate `.tmLanguage.json`: Syntax highlighting with scope names and regex patterns

3. **Grammar Composition**: Grammars leverage pattern inclusion and embedding to handle multi-language scenarios (PHP within HTML, HTML within PHP). The `embeddedLanguages` map enables proper syntax highlighting for nested code.

4. **Upstream Maintenance**: The build script (`update-grammar.mjs`) demonstrates a pattern for maintaining synchronization with upstream grammar sources while applying VS Code-specific fixes.

5. **Snippet System**: Code snippets use TextMate variable syntax with tab stops, choices, and placeholders to provide structured code completions.

6. **Development Infrastructure**: Standard VS Code extension development setup with extensionHost debugging and npm-based build tasks.

For a Tauri/Rust port, these patterns suggest the need for:
- A language registry system that maps file extensions to language IDs
- Pluggable grammar/tokenization engine (TextMate grammars or equivalent)
- Configuration schema for editor behavior (bracket matching, indentation rules)
- Code completion/snippet system with variable substitution
- Support for multi-language document embedding (especially HTML+PHP scenarios)

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
