# File Locations: Objective-C Extension

## Configuration

- `extensions/objective-c/package.json` — Extension manifest with language registrations and grammar paths
- `extensions/objective-c/language-configuration.json` — Language-specific editor behaviors (comments, brackets, auto-closing pairs)
- `extensions/objective-c/package.nls.json` — Localization strings
- `extensions/objective-c/.vscodeignore` — Files to exclude from packaging

## Grammar / Syntax

- `extensions/objective-c/syntaxes/objective-c.tmLanguage.json` — TextMate grammar for .m files
- `extensions/objective-c/syntaxes/objective-c++.tmLanguage.json` — TextMate grammar for .mm files

## Build Scripts

- `extensions/objective-c/build/update-grammars.js` — Script to update grammar files

## Metadata

- `extensions/objective-c/cgmanifest.json` — Component governance manifest

---

The Objective-C extension is a minimal syntax and language support package. It registers two language identifiers (objective-c for .m files, objective-cpp for .mm files), references external TextMate grammars for syntax highlighting, and provides editor configuration for brackets, comments, and auto-closing behavior. There are no implementation files, tests, or runtime logic—only declarative configuration and static grammar definitions.
