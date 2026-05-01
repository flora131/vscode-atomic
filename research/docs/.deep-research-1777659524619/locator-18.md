# Partition Analysis: .github/ (Out of Scope)

**Status**: Out of Scope for VS Code Tauri/Rust Porting Research

## Summary

The `.github/` directory contains 9 files across 4,624 LOC and consists entirely of GitHub repository operational infrastructure, not runtime code relevant to IDE functionality porting:

- **Workflows** (`.github/workflows/*.yml`): CI/CD pipeline definitions for testing, building, and deployment automation
- **GitHub Actions metadata** (`dependabot.yml`, `similarity.yml`, configuration files): Build system and dependency management automation
- **Issue templates** (`.github/ISSUE_TEMPLATE/`): Bug report and feature request templates for GitHub
- **AI automation** (`.github/agents/`, `.github/instructions/`, `.github/prompts/`, `.github/skills/`): Copilot instructions, AI agent configurations, and analysis scripts for development assistance
- **Pull request templates** (`.github/pull_request_template.md`): Contributor workflow templates
- **Code notification rules** (`.github/CODENOTIFY`, `.github/classifier.json`, `.github/hooks/`): CODEOWNERS and notification automation

## Conclusion

This partition is skipped. It contains no IDE implementation code, architectural patterns, type definitions, or core functionality relevant to understanding how to port VS Code's editor, language services, UI framework, or runtime from TypeScript/Electron to Tauri/Rust. This is pure CI/operational metadata.
