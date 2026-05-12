# Pattern Research: extensions/html/

## Scope
Directory: `extensions/html/` (1 file pattern, 61 LOC relevant)

## Assessment

The `extensions/html/` directory is a pure language extension containing only grammar definitions, snippets, and language configuration. It has no relevance to porting VS Code's core IDE functionality from TypeScript/Electron to Tauri/Rust.

### Contents:
- **package.json** (92 LOC): Extension manifest declaring HTML language support, grammar definitions, and snippets
- **language-configuration.json** (54 LOC): Language behavior config (indentation, folding, bracket pairs)
- **snippets/html.code-snippets** (18 LOC): HTML boilerplate snippet
- **build/update-grammar.mjs** (62 LOC): Grammar patching utility script
- **syntaxes/html.tmLanguage.json** (2,642 LOC): TextMate grammar (scope file, not analyzed)
- **syntaxes/html-derivative.tmLanguage.json** (48 LOC): TextMate grammar derivative (scope file, not analyzed)

### Pattern Absence Confirmation
No architecture, infrastructure, runtime, or core service patterns present. This is declarative configuration and external grammar data only.

---

**Sentinel:** The `extensions/html/` partition contains exclusively grammar and snippet definitions with no patterns applicable to porting VS Code's core IDE functionality to Tauri/Rust.
