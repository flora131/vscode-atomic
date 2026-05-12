# Partition 60: extensions/html/

**Status**: SKIP - Grammar/Snippets Package

The `extensions/html/` directory contains only declarative HTML language support (TextMate grammar, snippets, language configuration) with no implementation logic relevant to core IDE porting. This is a built-in language extension, not core functionality.

**Files**: 9 total
- Grammar definitions (`.tmLanguage.json`)
- Language configuration metadata
- Code snippets
- Build utilities for grammar updates
- Package manifest

**Relevance to Tauri/Rust Port**: None — Language syntax support would be handled by LSP integrations or UI extensions, not core IDE architecture.
