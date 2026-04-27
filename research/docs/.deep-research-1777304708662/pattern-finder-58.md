# Pattern Research: TypeScript Basics Extension
## Partition 58/79 - Language Grammar, Snippets, and Configuration

---

## Pattern 1: Extension Configuration and Language Declaration

**Found in**: `/extensions/typescript-basics/package.json:1-64`  
**Used for**: Declaring TypeScript and TypeScript React language support with grammar bindings

```json
{
  "name": "typescript",
  "description": "%description%",
  "displayName": "%displayName%",
  "version": "10.0.0",
  "author": "vscode",
  "publisher": "vscode",
  "license": "MIT",
  "engines": {
    "vscode": "*"
  },
  "scripts": {
    "update-grammar": "node ./build/update-grammars.mjs"
  },
  "categories": ["Programming Languages"],
  "contributes": {
    "languages": [
      {
        "id": "typescript",
        "aliases": ["TypeScript", "ts", "typescript"],
        "extensions": [".ts", ".cts", ".mts"],
        "firstLine": "^#!.*\\b(deno|bun|ts-node)\\b",
        "configuration": "./language-configuration.json"
      },
      {
        "id": "typescriptreact",
        "aliases": ["TypeScript JSX", "TypeScript React", "tsx"],
        "extensions": [".tsx"],
        "configuration": "./language-configuration.json"
      }
    ]
  }
}
```

**Key aspects**:
- Multi-variant language support: `typescript`, `typescriptreact`, `jsonc`, `json`
- File extension mapping with multiple variants (`.ts`, `.cts`, `.mts`)
- Shebang-based language detection for runtime-specific files
- Centralized language configuration reference
- Grammar contributions for syntax highlighting

---

## Pattern 2: Grammar Registration and Token Mapping

**Found in**: `/extensions/typescript-basics/package.json:65-127`  
**Used for**: Registering TextMate grammars with semantic token scope mappings

```json
"grammars": [
  {
    "language": "typescript",
    "scopeName": "source.ts",
    "path": "./syntaxes/TypeScript.tmLanguage.json",
    "unbalancedBracketScopes": [
      "keyword.operator.relational",
      "storage.type.function.arrow",
      "keyword.operator.bitwise.shift",
      "meta.brace.angle",
      "punctuation.definition.tag",
      "keyword.operator.assignment.compound.bitwise.ts"
    ],
    "tokenTypes": {
      "punctuation.definition.template-expression": "other",
      "entity.name.type.instance.jsdoc": "other",
      "entity.name.function.tagged-template": "other",
      "meta.import string.quoted": "other",
      "variable.other.jsdoc": "other"
    }
  },
  {
    "language": "typescriptreact",
    "scopeName": "source.tsx",
    "path": "./syntaxes/TypeScriptReact.tmLanguage.json",
    "unbalancedBracketScopes": [
      "keyword.operator.relational",
      "storage.type.function.arrow",
      "keyword.operator.bitwise.shift",
      "punctuation.definition.tag",
      "keyword.operator.assignment.compound.bitwise.ts"
    ],
    "embeddedLanguages": {
      "meta.tag.tsx": "jsx-tags",
      "meta.tag.without-attributes.tsx": "jsx-tags",
      "meta.tag.attributes.tsx": "typescriptreact",
      "meta.embedded.expression.tsx": "typescriptreact"
    }
  }
]
```

**Key aspects**:
- Grammar files derived from external source (microsoft/TypeScript-TmLanguage)
- Unbalanced bracket scopes for angle bracket matching
- Language-specific embedded language support (JSX)
- Token type overrides for semantic accuracy

---

## Pattern 3: Code Snippets with Placeholder Substitution

**Found in**: `/extensions/typescript-basics/snippets/typescript.code-snippets:1-50`  
**Used for**: Defining reusable code templates with variable placeholders

```json
{
  "Constructor": {
    "prefix": "ctor",
    "body": [
      "/**",
      " *",
      " */",
      "constructor() {",
      "\tsuper();",
      "\t$0",
      "}",
    ],
    "description": "Constructor"
  },
  "Class Definition": {
    "prefix": "class",
    "isFileTemplate": true,
    "body": [
      "class ${1:name} {",
      "\tconstructor(${2:parameters}) {",
      "\t\t$0",
      "\t}",
      "}"
    ],
    "description": "Class Definition"
  },
  "Import Statement": {
    "prefix": "import",
    "body": [
      "import { $0 } from \"${1:module}\";"
    ],
    "description": "Import external module"
  },
  "Property getter": {
    "prefix": "get",
    "body": [
      "",
      "public get ${1:value}() : ${2:string} {",
      "\t${3:return $0}",
      "}",
      ""
    ],
    "description": "Property getter"
  }
}
```

**Key aspects**:
- Numbered placeholder syntax: `${1:name}`, `${2:parameters}`, `${3:element}`
- Final cursor position marker: `$0`
- File template flag for class definitions
- Multi-line bodies with tab-based indentation
- Descriptions for IDE tooltips

**Variations**: Console logging, loops, async functions, promise patterns:

```json
{
  "Log to the console": {
    "prefix": "log",
    "body": ["console.log($1);", "$0"],
    "description": "Log to the console"
  },
  "For Loop": {
    "prefix": "for",
    "body": [
      "for (let ${1:index} = 0; ${1:index} < ${2:array}.length; ${1:index}++) {",
      "\tconst ${3:element} = ${2:array}[${1:index}];",
      "\t$TM_SELECTED_TEXT$0",
      "}"
    ],
    "description": "For Loop"
  },
  "Async Function Statement": {
    "prefix": "async function",
    "body": [
      "async function ${1:name}(${2:params}:${3:type}) {",
      "\t$TM_SELECTED_TEXT$0",
      "}"
    ],
    "description": "Async Function Statement"
  }
}
```

---

## Pattern 4: Language Configuration with Bracket Matching and Auto-Closing

**Found in**: `/extensions/typescript-basics/language-configuration.json:1-75`  
**Used for**: Configuring editor behavior for bracket matching, auto-closing, and auto-indentation

```json
{
  "comments": {
    "lineComment": "//",
    "blockComment": ["/*", "*/"]
  },
  "brackets": [
    ["${", "}"],
    ["{", "}"],
    ["[", "]"],
    ["(", ")"]
  ],
  "autoClosingPairs": [
    { "open": "${", "close": "}" },
    { "open": "{", "close": "}" },
    { "open": "[", "close": "]" },
    { "open": "(", "close": ")" },
    { "open": "'", "close": "'", "notIn": ["string", "comment"] },
    { "open": "\"", "close": "\"", "notIn": ["string"] },
    { "open": "`", "close": "`", "notIn": ["string", "comment"] },
    { "open": "/**", "close": " */", "notIn": ["string"] }
  ],
  "surroundingPairs": [
    ["${", "}"],
    ["$", ""],
    ["{", "}"],
    ["[", "]"],
    ["(", ")"],
    ["'", "'"],
    ["\"", "\""],
    ["`", "`"],
    ["<", ">"]
  ],
  "colorizedBracketPairs": [
    ["(", ")"],
    ["[", "]"],
    ["{", "}"],
    ["<", ">"]
  ],
  "autoCloseBefore": ";:.,=}])>` \n\t"
}
```

**Key aspects**:
- Template expression brackets: `${...}`
- Context-aware auto-closing (not in strings/comments)
- JSDoc block comment pattern
- Surrounding pairs for wrapping selection
- Colorized bracket pair highlighting
- Close-before characters to prevent double insertion

---

## Pattern 5: OnEnter Rules for Comment and Indentation Behavior

**Found in**: `/extensions/typescript-basics/language-configuration.json:157-271`  
**Used for**: Configuring automatic indentation and text insertion on Enter key press

```json
"onEnterRules": [
  {
    // e.g. /** | */
    "beforeText": {
      "pattern": "^\\s*/\\*\\*(?!/)([^\\*]|\\*(?!/))*$"
    },
    "afterText": {
      "pattern": "^\\s*\\*/$"
    },
    "action": {
      "indent": "indentOutdent",
      "appendText": " * "
    }
  },
  {
    // e.g.  * ...|
    "beforeText": {
      "pattern": "^(\\t|[ ])*\\*([ ]([^\\*]|\\*(?!/))*)?$"
    },
    "previousLineText": {
      "pattern": "(?=^(\\s*(/\\*\\*|\\*)).*)(?=(?!(\\s*\\*/))"
    },
    "action": {
      "indent": "none",
      "appendText": "* "
    }
  },
  {
    "beforeText": {
      "pattern": "^\\s*(\\bcase\\s.+:|\\bdefault:)$"
    },
    "afterText": {
      "pattern": "^(?!\\s*(\\bcase\\b|\\bdefault\\b))"
    },
    "action": {
      "indent": "indent"
    }
  },
  {
    // Indent when pressing enter from inside ()
    "beforeText": "^.*\\([^\\)]*$",
    "afterText": "^\\s*\\).*$",
    "action": {
      "indent": "indentOutdent",
      "appendText": "\t"
    }
  },
  {
    // Add // when pressing enter from inside line comment
    "beforeText": "(?<!\\\\|\\w:)\/\/\\s*\\S",
    "afterText": "^(?!\\s*$).+",
    "action": {
      "indent": "none",
      "appendText": "// "
    }
  }
]
```

**Key aspects**:
- Multi-condition rules (beforeText, afterText, previousLineText)
- JSDoc continuation pattern matching
- Switch/case indentation handling
- Context-aware actions: indent, indentOutdent, outdent
- Comment prefix continuation
- Parenthesis-aware indentation

---

## Pattern 6: Grammar File Patching and Adaptation Build Script

**Found in**: `/extensions/typescript-basics/build/update-grammars.mjs:1-96`  
**Used for**: Automating grammar file updates from upstream and removing unnecessary patterns

```javascript
import { update } from 'vscode-grammar-updater';

function removeDom(grammar) {
  grammar.repository['support-objects'].patterns = grammar.repository['support-objects'].patterns.filter(pattern => {
    if (pattern.match && (
      /\b(HTMLElement|ATTRIBUTE_NODE|stopImmediatePropagation)\b/g.test(pattern.match)
      || /\bJSON\b/g.test(pattern.match)
      || /\bMath\b/g.test(pattern.match)
    )) {
      return false;
    }
    if (pattern.name?.startsWith('support.class.error.')
      || pattern.name?.startsWith('support.class.builtin.')
      || pattern.name?.startsWith('support.function.')
    ) {
      return false;
    }
    return true;
  });
  return grammar;
}

function removeNodeTypes(grammar) {
  grammar.repository['support-objects'].patterns = grammar.repository['support-objects'].patterns.filter(pattern => {
    if (pattern.name) {
      if (pattern.name.startsWith('support.variable.object.node') || pattern.name.startsWith('support.class.node.')) {
        return false;
      }
    }
    if (pattern.captures) {
      if (Object.values(pattern.captures).some(capture =>
        capture.name && (capture.name.startsWith('support.variable.object.process')
          || capture.name.startsWith('support.class.console'))
      )) {
        return false;
      }
    }
    return true;
  });
  return grammar;
}

function patchJsdoctype(grammar) {
  grammar.repository['jsdoctype'].patterns = grammar.repository['jsdoctype'].patterns.filter(pattern => {
    if (pattern.name && pattern.name.includes('illegal')) {
      return false;
    }
    return true;
  });
  return grammar;
}

function patchGrammar(grammar) {
  return removeNodeTypes(removeDom(patchJsdoctype(grammar)));
}

var tsGrammarRepo = 'microsoft/TypeScript-TmLanguage';
update(tsGrammarRepo, 'TypeScript.tmLanguage', './syntaxes/TypeScript.tmLanguage.json', grammar => patchGrammar(grammar));
update(tsGrammarRepo, 'TypeScriptReact.tmLanguage', './syntaxes/TypeScriptReact.tmLanguage.json', grammar => patchGrammar(grammar));
update(tsGrammarRepo, 'TypeScriptReact.tmLanguage', '../javascript/syntaxes/JavaScript.tmLanguage.json', grammar => adaptToJavaScript(patchGrammar(grammar), '.js'));
```

**Key aspects**:
- Grammar updater from external source (microsoft/TypeScript-TmLanguage)
- Filtering pipelines for removing bloat (DOM objects, Node.js types)
- Name-based scope filtering for built-in types
- Composition of patch functions
- Batch updates for multiple output files

---

## Pattern 7: Semantic Token Scope Mapping for Language Intelligence

**Found in**: `/extensions/typescript-basics/package.json:128-187`  
**Used for**: Mapping semantic token types to TextMate scopes for IDE features

```json
"semanticTokenScopes": [
  {
    "language": "typescript",
    "scopes": {
      "property": [
        "variable.other.property.ts"
      ],
      "property.readonly": [
        "variable.other.constant.property.ts"
      ],
      "variable": [
        "variable.other.readwrite.ts"
      ],
      "variable.readonly": [
        "variable.other.constant.object.ts"
      ],
      "function": [
        "entity.name.function.ts"
      ],
      "namespace": [
        "entity.name.type.module.ts"
      ],
      "variable.defaultLibrary": [
        "support.variable.ts"
      ],
      "function.defaultLibrary": [
        "support.function.ts"
      ]
    }
  },
  {
    "language": "typescriptreact",
    "scopes": {
      "property": [
        "variable.other.property.tsx"
      ],
      "property.readonly": [
        "variable.other.constant.property.tsx"
      ],
      "variable": [
        "variable.other.readwrite.tsx"
      ],
      "variable.readonly": [
        "variable.other.constant.object.tsx"
      ],
      "function": [
        "entity.name.function.tsx"
      ],
      "namespace": [
        "entity.name.type.module.tsx"
      ]
    }
  }
]
```

**Key aspects**:
- Bidirectional mapping: semantic tokens → scopes
- Read-only property differentiation
- Default library distinction for built-in APIs
- Language-specific scope variants (ts vs tsx)
- Supports IDE features like semantic highlighting and symbol navigation

---

## Summary

The `typescript-basics` extension demonstrates the foundational patterns for language support in VS Code, structured as static assets and configuration rather than runtime logic. Key patterns include:

1. **Extension Metadata Pattern**: Clean separation of language declaration, grammar registration, and snippet contributions in `package.json`

2. **Grammar Management Pattern**: Derived TextMate grammars from upstream source with systematic patching to customize scope names, remove bloat, and maintain synchronization

3. **Snippet Definition Pattern**: Template-based code generation with numbered placeholders, context awareness, and IDE integration hooks

4. **Editor Behavior Configuration Pattern**: Declarative rules for bracket matching, auto-closing, auto-indentation, and comment continuation without code

5. **Semantic Token Bridge Pattern**: Mapping semantic language features to TextMate scopes to connect language intelligence with syntax highlighting

6. **Build Automation Pattern**: Composition of filter functions for grammar patching, enabling reproducible updates from upstream sources

These patterns are relevant for a Tauri/Rust port because:
- Language grammar systems could be abstracted and ported independently
- Configuration schema could be replicated in Rust data structures
- Snippet and scope mapping systems are language-agnostic
- The grammar patching pipeline demonstrates data transformation patterns applicable to Rust
- Semantic token mapping shows how to bridge language servers to UI rendering across architectures

