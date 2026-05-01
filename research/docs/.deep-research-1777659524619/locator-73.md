# Locator Partition 73: extensions/objective-c/

## Scope
Objective-C grammar extension (1 directory, 8 files, ~14 LOC in build scripts)

## Relevance Assessment
**SKIP** - This partition contains only grammar and language configuration files for Objective-C syntax highlighting. Not relevant to porting VS Code core IDE functionality from TypeScript/Electron to Tauri/Rust.

## File Locations

### Configuration Files
- `extensions/objective-c/package.json` - Extension manifest defining Objective-C and Objective-C++ language support
- `extensions/objective-c/language-configuration.json` - Syntax and indentation configuration
- `extensions/objective-c/package.nls.json` - Localization strings
- `extensions/objective-c/.vscodeignore` - Build artifact exclusions
- `extensions/objective-c/cgmanifest.json` - Component governance manifest

### Grammar Files
- `extensions/objective-c/syntaxes/objective-c.tmLanguage.json` - TextMate grammar for .m files
- `extensions/objective-c/syntaxes/objective-c++.tmLanguage.json` - TextMate grammar for .mm files

### Build/Maintenance
- `extensions/objective-c/build/update-grammars.js` - Grammar update script (~14 LOC)

## Summary
This extension provides TextMate grammar rules and language configuration for Objective-C and Objective-C++ syntax highlighting in VS Code. It contains no runtime logic, no IDE core functionality, and no platform-specific code relevant to an Electron-to-Tauri port. This is a pure language grammar extension and can be skipped for the core IDE porting effort.
