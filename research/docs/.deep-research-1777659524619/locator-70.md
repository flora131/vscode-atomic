# Locator Partition 70: extensions/less/

## Scope
- Directory: `extensions/less/`
- Files: 7 total
- LOC: ~19 (grammar/config files)

## Classification
**LESS grammar extension** — language syntax definition only

## Contents
- `syntaxes/less.tmLanguage.json` — TextMate grammar for LESS syntax highlighting
- `language-configuration.json` — Language configuration for brackets, comments, etc.
- `package.json` — Extension metadata
- `package.nls.json` — Localization strings
- `cgmanifest.json` — Component governance manifest
- `.vscodeignore` — Files to ignore in published extension
- `build/update-grammar.js` — Build script for grammar updates

## Relevance Assessment
**Not relevant to Tauri/Rust porting** — This is a pure language grammar extension providing LESS syntax highlighting support. No runtime code, no IDE core functionality, no state management, no editor logic.

## Recommendation
**SKIP** — Grammar extensions are orthogonal to the core IDE porting effort. They define syntax highlighting rules via TextMate grammar, not functional code. Can be maintained separately or ported later if needed.

---
**Status:** Sentinel confirmed — partition 70 skipped per orientation briefing.
