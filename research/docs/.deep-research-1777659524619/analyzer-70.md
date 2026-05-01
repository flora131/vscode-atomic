# Partition: extensions/less/

**Status: GRAMMAR-ONLY — Not relevant to porting core IDE functionality.**

This partition contains only the LESS TextMate grammar (`syntaxes/less.tmLanguage.json`), a language configuration file (`language-configuration.json`), the extension manifest (`package.json`), and a build script. There is no editor runtime logic, no TypeScript source, and no VS Code API surface to port.

TextMate grammars are host-agnostic JSON (or XML plist) data structures that describe tokenization rules as regex-based scope patterns. They carry no dependency on any particular host runtime — a Rust-side TextMate or grammar engine would consume the `less.tmLanguage.json` file directly and unchanged, requiring no translation or adaptation work as part of an atomic IDE port.

The LESS extension is therefore inert from a porting perspective. Its entire contribution is the grammar file and the language configuration, both of which are declarative data that any conforming TextMate grammar consumer can load without modification. No analysis of runtime behavior, API bindings, or execution paths is warranted here, and this partition can be safely excluded from further deep-research passes targeting core IDE functionality.
