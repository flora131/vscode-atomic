# Partition 18: `.github/` Directory

**Scope verdict: OUT OF SCOPE**

(no external research applicable)

The `.github/` directory contains exclusively CI/CD pipeline definitions, GitHub Actions workflow files, issue and pull-request templates, Dependabot configuration, and other repository-operational metadata. None of these artifacts describe application behavior, extension APIs, UI surface, IPC contracts, or build-system logic that would need to be reproduced or adapted in a Tauri/Rust port of VS Code. Workflow YAML files orchestrate linting, packaging, and release automation that is inherently platform-specific to GitHub Actions and has no meaningful analog in a Tauri build pipeline at the source-code level.
