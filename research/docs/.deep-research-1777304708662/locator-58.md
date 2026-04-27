# File Locations: extensions/typescript-basics/

## Implementation

- `extensions/typescript-basics/syntaxes/TypeScript.tmLanguage.json` - TextMate grammar for TypeScript syntax highlighting (derived from TypeScript-TmLanguage repository)
- `extensions/typescript-basics/syntaxes/TypeScriptReact.tmLanguage.json` - TextMate grammar for TypeScript JSX/React syntax highlighting
- `extensions/typescript-basics/syntaxes/jsdoc.ts.injection.tmLanguage.json` - JSDoc documentation syntax injection for TypeScript
- `extensions/typescript-basics/syntaxes/jsdoc.js.injection.tmLanguage.json` - JSDoc documentation syntax injection for JavaScript
- `extensions/typescript-basics/language-configuration.json` - Language configuration defining brackets, auto-closing pairs, folding rules, indentation rules, and editor behaviors for TypeScript files
- `extensions/typescript-basics/snippets/typescript.code-snippets` - Code snippets for TypeScript and TypeScript React languages

## Configuration

- `extensions/typescript-basics/package.json` - Extension manifest with language and grammar contributions, semantic token scopes, and build scripts
- `extensions/typescript-basics/package.nls.json` - Localization strings for display name and description
- `extensions/typescript-basics/.vscodeignore` - Files to exclude from packaging
- `extensions/typescript-basics/cgmanifest.json` - Component governance manifest for tracking dependencies

## Build & Maintenance

- `extensions/typescript-basics/build/update-grammars.mjs` - Build script for updating grammar files from TypeScript-TmLanguage repository

## Documentation

- `extensions/typescript-basics/syntaxes/Readme.md` - Documentation on grammar source, update procedures, and migration notes for scope naming improvements

## Summary

The `typescript-basics` extension is a grammar and snippet-only language contribution for VS Code. It provides TextMate grammars for TypeScript syntax highlighting (both standard and JSX variants), language configuration for editor behaviors (bracket matching, auto-closing, indentation), and code snippets. The extension has no executable code or language server integration—it focuses entirely on static syntax highlighting and editor features. The grammars are derived from and maintained synchronously with the external TypeScript-TmLanguage repository via the update-grammars build script.
