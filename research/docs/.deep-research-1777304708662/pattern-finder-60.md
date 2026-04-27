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

