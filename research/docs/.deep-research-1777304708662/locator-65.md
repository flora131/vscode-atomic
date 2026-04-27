# Partition 65: extensions/json/

## Implementation
- `extensions/json/build/update-grammars.js` (39 LOC) - Build script for grammar synchronization

## Configuration
- `extensions/json/package.json` - Extension manifest and metadata
- `extensions/json/package.nls.json` - Localization strings for package metadata
- `extensions/json/language-configuration.json` - Language-specific configuration (indentation, bracket pairs, etc.)
- `extensions/json/.vscodeignore` - Files to exclude from extension package
- `extensions/json/cgmanifest.json` - Component governance manifest

## Documentation / Examples / Fixtures
- `extensions/json/syntaxes/JSON.tmLanguage.json` - TextMate grammar for JSON syntax highlighting
- `extensions/json/syntaxes/JSONC.tmLanguage.json` - TextMate grammar for JSON with Comments syntax highlighting
- `extensions/json/syntaxes/JSONL.tmLanguage.json` - TextMate grammar for JSON Lines (newline-delimited JSON) syntax highlighting
- `extensions/json/syntaxes/snippets.tmLanguage.json` - TextMate grammar for JSON snippet syntax highlighting

## Summary
The `extensions/json/` directory contains the VS Code JSON language extension. This is a minimal, focused extension containing only grammar definitions (syntax highlighting rules), configuration for language behavior, and build tooling to synchronize grammars. No test files or TypeScript source code exist in this scope—the extension is primarily declarative grammar and configuration files.
