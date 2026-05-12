# Partition 26: eslint.config.js — ESLint Configuration

(no external research applicable)

**Justification:** The `eslint.config.js` file is a project-internal linting configuration that encodes VS Code's own code-style rules, plugin selections, and per-directory overrides; when porting to Tauri/Rust the file is either carried over as-is for any remaining TypeScript/JavaScript layers or simply replaced by the new project's own linting setup, making it a mechanical migration decision that requires no external research.
