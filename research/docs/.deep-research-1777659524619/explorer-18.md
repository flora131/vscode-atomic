# Partition 18 of 79 — Findings

## Scope
`.github/` (9 files, 4,624 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
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

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
## Analysis: Partition 18 — `.github/` (CI / Operational Metadata)

### Files Analysed

(none — partition is non-runtime CI/operational metadata)

### Notes

The `.github/` directory was briefly verified via a glob traversal. Its contents fall exclusively into the following non-runtime categories:

**GitHub Actions workflows** (`workflows/`): YAML pipeline definitions for PR testing on Linux, macOS, and Windows (`pr-linux-test.yml`, `pr-darwin-test.yml`, `pr-win32-test.yml`), CLI tests, component fixture tests, Monaco editor publishing, API proposal version checks, chat performance runs, and repo hygiene guards (`check-clean-git-state.sh`, `no-engineering-system-changes.yml`).

**Issue and PR templates** (`ISSUE_TEMPLATE/`, `pull_request_template.md`): Structured markdown forms for bug reports (including a Copilot-specific variant), feature requests, and pull request descriptions — GitHub UI metadata only.

**Copilot / AI assistant instructions and prompts** (`instructions/`, `prompts/`, `agents/`, `skills/`): Markdown files that configure GitHub Copilot's in-repo behaviour (coding style guidance, accessibility, telemetry, observable patterns, notebook architecture, agent sessions, etc.) and reusable prompt templates (`implement.prompt.md`, `fix-error.prompt.md`, `migrate.prompt.md`, etc.). These are read by the GitHub Copilot service at authoring time, not at application runtime.

**Dependabot configuration** (`dependabot.yml`): Automated dependency-update scheduling for npm and GitHub Actions ecosystems.

**Repository ownership and notification routing** (`CODENOTIFY`, `similarity.yml`, `classifier.json`, `commands.json`, `hooks/hooks.json`): Files consumed by GitHub bots and internal tooling to route notifications, label PRs, and trigger repository commands — none of these are loaded by the VS Code or extension host runtime.

**Skills scratchpads and helper scripts** (`skills/auto-perf-optimize/scripts/`, `skills/heap-snapshot-analysis/helpers/`): Standalone TypeScript/MJS utility scripts used offline by internal contributors for profiling and heap analysis; they import no application modules and are not part of any build target.

None of these files contain TypeScript source modules, Rust crates, WebAssembly bindings, IPC contracts, or extension APIs. They carry zero runtime code that would need to be analysed or ported as part of a Tauri/Rust migration of VS Code's core. Partition 18 is therefore out of scope for the porting question and requires no further investigation.

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Pattern Finder Research: VS Code TypeScript/Electron to Tauri/Rust Porting

**Research Question:** What it would take to port VS Code's core IDE functionality from TypeScript/Electron to Tauri/Rust.

**Scope Analysis:** `.github/` directory (CI workflows, configuration, metadata)

## Out of Scope Determination

The `.github/` partition is **OUT OF SCOPE** for this research question.

This directory contains:
- **CI/CD Workflows** (`.github/workflows/`) - GitHub Actions configuration for testing and deployment (pr-linux-test.yml, pr-darwin-test.yml, pr-win32-test.yml, etc.)
- **Repository Configuration** - Dependabot settings, code notification rules, PR templates
- **Development Metadata** - Issue templates, contributor instructions, Copilot instruction guides
- **Build Infrastructure Scripts** - Test runners and deployment helpers
- **Documentation and Skills** - Agent instructions, skill definitions, prompt templates

None of these artifacts represent **runtime code**, architectural patterns, or implementation details that would inform a TypeScript/Electron-to-Tauri/Rust migration strategy.

### Why This Partition Cannot Answer the Research Question

To understand porting requirements, we need to analyze:
1. **Core language bindings** (TypeScript → Rust type systems)
2. **UI framework architecture** (Electron/web → Tauri/native UI)
3. **Platform abstraction layers** (OS-specific modules)
4. **IPC and process models** (main/renderer → Tauri frontend/backend)
5. **Extension system** (plugin architecture portability)
6. **Build and compilation strategies**

The `.github/` directory provides none of this information. It shows HOW the project is tested and deployed, not HOW it is structured or implemented.

### What Would Be Relevant

To answer this research question properly, analysis would focus on:
- `/src/` - Core TypeScript implementation, architecture layers
- `/src/vs/code/` - Electron main process code
- `/src/vs/base/` - Cross-platform abstractions
- `/src/vs/platform/` - Platform service implementations
- `/build/` - Build tooling and compilation strategy
- Type definitions, module boundaries, and dependency graphs

---

**Sentinel:** This partition documents CI/CD infrastructure and metadata, not runtime code architecture. Porting feasibility analysis requires source code examination.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
