# Partition 59: `extensions/php/` — Online Research

(no external research applicable)

This partition contains only static grammar and snippet assets (a TextMate tmLanguage grammar for PHP/HTML, a language-configuration JSON, and a `.code-snippets` file) with no third-party library dependencies that are central to porting VS Code to Tauri/Rust.

## Summary

The `extensions/php/` directory is a purely declarative, data-only extension: it ships a bundled TextMate grammar (`syntaxes/php.tmLanguage.json`, `syntaxes/html.tmLanguage.json`), a language configuration file (`language-configuration.json`), and a PHP snippet file (`snippets/php.code-snippets`). There is no TypeScript or JavaScript runtime code, no npm runtime dependencies, and no platform-specific integration points. The grammar JSON files are vendored snapshots derived from the `php/php-src` TextMate grammar and require no external fetching or Rust/Tauri bridging work. Porting this partition to a Tauri-hosted VS Code amounts to carrying these static JSON files as-is; the only build-time artifact to be aware of is `build/update-grammar.mjs`, a maintenance script used to refresh the bundled grammar from upstream, which has no bearing on the runtime port.
