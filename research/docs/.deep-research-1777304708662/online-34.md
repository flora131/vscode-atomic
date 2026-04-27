<!--research-sentinel: NO_EXTERNAL_RESEARCH_APPLICABLE-->

# Partition 34: extensions/extension-editing/ — deps: jsonc-parser, markdown-it, parse5

(no external research applicable) The three libraries scoped to this partition — jsonc-parser, markdown-it, and parse5 — are peripheral tooling used by the extension-editing linter to parse JSON-with-comments config files and to validate README markup; they have no bearing on the core porting decision and each has a direct, well-maintained Rust equivalent (tree-sitter-jsonc or the `jsonc` crate for JSON-with-comments, pulldown-cmark for Markdown rendering/parsing, and the `scraper`/`html5ever` crates for HTML5 parsing), so documentation research for these TypeScript libraries would not inform the Tauri/Rust migration strategy in any meaningful way.

## Summary

The `extensions/extension-editing/` partition contributes extension-authoring tooling — README linting and configuration-file validation — that is entirely optional from the perspective of porting the core VS Code IDE shell to Tauri and Rust. Its three JavaScript dependencies (jsonc-parser, markdown-it, parse5) are well-understood, narrowly scoped libraries with clear Rust-ecosystem analogues. No external documentation research is warranted for this partition; the porting effort here reduces to a straightforward library substitution rather than any architectural investigation.
