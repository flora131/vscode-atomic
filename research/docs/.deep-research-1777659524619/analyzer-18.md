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
