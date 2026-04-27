# Objective-C Extension - File Locator Report

## Configuration
- `extensions/objective-c/package.json` - Extension manifest with language contributions
- `extensions/objective-c/language-configuration.json` - Language configuration for Objective-C
- `extensions/objective-c/package.nls.json` - Localization strings
- `extensions/objective-c/cgmanifest.json` - Component governance manifest
- `extensions/objective-c/.vscodeignore` - VSCode packaging ignore file

## Implementation
- `extensions/objective-c/build/update-grammars.js` - Build script for grammar updates

## Examples / Fixtures
- `extensions/objective-c/syntaxes/objective-c.tmLanguage.json` - TextMate grammar for Objective-C
- `extensions/objective-c/syntaxes/objective-c++.tmLanguage.json` - TextMate grammar for Objective-C++

---

The `extensions/objective-c/` directory contains a grammar-only language extension with no runtime code to port. It defines syntax highlighting for `.m` and `.mm` files through TextMate grammar files and language configuration, along with build automation for grammar updates. This is a declarative extension that provides syntax support without any executable logic—no TypeScript/JavaScript runtime components, no Electron integration, and no core IDE functionality implementation to migrate to Tauri/Rust.
