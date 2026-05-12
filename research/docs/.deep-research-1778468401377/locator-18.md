# File Locations in `.github/` Directory

## Configuration
- `.github/classifier.json` - Issue/PR classification rules
- `.github/commands.json` - Command definitions
- `.github/copilot-instructions.md` - Copilot-specific instructions
- `.github/dependabot.yml` - Dependabot configuration
- `.github/insiders.yml` - Insiders build configuration
- `.github/endgame/insiders.yml` - Endgame phase insiders configuration
- `.github/similarity.yml` - Code similarity detection configuration
- `.github/CODENOTIFY` - Code notification rules
- `.github/pull_request_template.md` - PR template

## Workflows (CI/CD)
- `.github/workflows/api-proposal-version-check.yml`
- `.github/workflows/chat-lib-package.yml`
- `.github/workflows/chat-perf.yml`
- `.github/workflows/check-clean-git-state.sh` - Shell script for git state validation
- `.github/workflows/component-fixtures.yml`
- `.github/workflows/copilot-setup-steps.yml`
- `.github/workflows/monaco-editor.yml`
- `.github/workflows/no-engineering-system-changes.yml`
- `.github/workflows/pr-darwin-test.yml` - macOS testing
- `.github/workflows/pr-linux-cli-test.yml` - Linux CLI testing
- `.github/workflows/pr-linux-test.yml` - Linux testing
- `.github/workflows/pr-node-modules.yml` - Node modules validation
- `.github/workflows/pr-win32-test.yml` - Windows testing
- `.github/workflows/pr.yml` - Primary PR workflow
- `.github/workflows/sessions-e2e.yml` - End-to-end sessions testing
- `.github/workflows/telemetry.yml` - Telemetry workflow

## Issue Templates
- `.github/ISSUE_TEMPLATE/bug_report.md`
- `.github/ISSUE_TEMPLATE/config.yml`
- `.github/ISSUE_TEMPLATE/copilot_bug_report.md`
- `.github/ISSUE_TEMPLATE/feature_request.md`

## Agent Documentation
- `.github/agents/data.md` - Agent data documentation
- `.github/agents/demonstrate.md` - Agent demonstration docs
- `.github/agents/sessions.md` - Agent sessions documentation

## Commands
- `.github/commands/codespaces_issue.yml` - Codespaces issue command

## Instructions / Subsystem Catalogs (15 markdown files)
- `.github/instructions/accessibility.instructions.md`
- `.github/instructions/agentHostTesting.instructions.md`
- `.github/instructions/ai-customization.instructions.md`
- `.github/instructions/api-version.instructions.md`
- `.github/instructions/buildNext.instructions.md`
- `.github/instructions/chat.instructions.md`
- `.github/instructions/disposable.instructions.md`
- `.github/instructions/interactive.instructions.md` - Interactive feature subsystem
- `.github/instructions/kusto.instructions.md` - Kusto query language integration
- `.github/instructions/learnings.instructions.md`
- `.github/instructions/notebook.instructions.md` - Notebook subsystem
- `.github/instructions/observables.instructions.md`
- `.github/instructions/oss.instructions.md` - Open-source considerations
- `.github/instructions/sessions.instructions.md` - Sessions subsystem
- `.github/instructions/telemetry.instructions.md` - Telemetry subsystem
- `.github/instructions/tree-widgets.instructions.md` - Tree widget subsystem

## Instruction Resources (Diagrams)
- `.github/instructions/resources/interactive/interactive.eh.drawio.svg`
- `.github/instructions/resources/interactive/interactive.editor.drawio.svg`
- `.github/instructions/resources/interactive/interactive.model.resolution.drawio.svg`
- `.github/instructions/resources/notebook/cell-resize-above-viewport.drawio.svg`
- `.github/instructions/resources/notebook/hybrid-find.drawio.svg`
- `.github/instructions/resources/notebook/viewport-rendering.drawio.svg`

## Prompts (17 markdown files)
- `.github/prompts/build-champ.prompt.md`
- `.github/prompts/codenotify.prompt.md`
- `.github/prompts/component.prompt.md`
- `.github/prompts/doc-comments.prompt.md`
- `.github/prompts/find-duplicates.prompt.md`
- `.github/prompts/find-issue.prompt.md`
- `.github/prompts/fix-error.prompt.md`
- `.github/prompts/fixIssueNo.prompt.md`
- `.github/prompts/implement.prompt.md`
- `.github/prompts/issue-grouping.prompt.md`
- `.github/prompts/micro-perf.prompt.md`
- `.github/prompts/migrate.prompt.md`
- `.github/prompts/no-any.prompt.md`
- `.github/prompts/plan-deep.prompt.md`
- `.github/prompts/plan-fast.prompt.md`
- `.github/prompts/setup-environment.prompt.md`
- `.github/prompts/update-instructions.prompt.md`

## Skills / Subsystem Implementations (31 skills)

### Core Skills (SKILL.md files)
- `.github/skills/accessibility/SKILL.md`
- `.github/skills/add-policy/SKILL.md`
- `.github/skills/agent-sessions-layout/SKILL.md` - Sessions layout implementation
- `.github/skills/author-contributions/SKILL.md`
- `.github/skills/auto-perf-optimize/SKILL.md`
- `.github/skills/azure-pipelines/SKILL.md`
- `.github/skills/chat-customizations-editor/SKILL.md` - Chat customization feature
- `.github/skills/chat-perf/SKILL.md`
- `.github/skills/code-oss-logs/SKILL.md`
- `.github/skills/component-fixtures/SKILL.md`
- `.github/skills/cpu-profile-analysis/SKILL.md`
- `.github/skills/fix-ci-failures/SKILL.md`
- `.github/skills/fix-errors/SKILL.md`
- `.github/skills/heap-snapshot-analysis/SKILL.md`
- `.github/skills/hygiene/SKILL.md`
- `.github/skills/integration-tests/SKILL.md`
- `.github/skills/memory-leak-audit/SKILL.md`
- `.github/skills/otel/SKILL.md` - OpenTelemetry integration
- `.github/skills/sessions/SKILL.md`
- `.github/skills/tool-rename-deprecation/SKILL.md`
- `.github/skills/unit-tests/SKILL.md`
- `.github/skills/update-screenshots/SKILL.md`
- `.github/skills/vscode-dev-workbench/SKILL.md`

### Skills with TypeScript Helpers
- `.github/skills/azure-pipelines/azure-pipeline.ts`
- `.github/skills/heap-snapshot-analysis/helpers/compareSnapshots.ts`
- `.github/skills/heap-snapshot-analysis/helpers/findRetainers.ts`
- `.github/skills/heap-snapshot-analysis/helpers/parseSnapshot.ts`
- `.github/skills/heap-snapshot-analysis/helpers/streamSnapshot.mjs`

### Skills with Test Scripts
- `.github/skills/auto-perf-optimize/scripts/chat-memory-smoke.mts`
- `.github/skills/auto-perf-optimize/scripts/chat-session-switch-smoke.mts`
- `.github/skills/auto-perf-optimize/scripts/userDataProfile.mts`

### Skills Support Files
- `.github/skills/auto-perf-optimize/scratchpad/.gitignore`
- `.github/skills/auto-perf-optimize/scratchpad/README.md`
- `.github/skills/heap-snapshot-analysis/scratchpad/.gitignore`
- `.github/skills/heap-snapshot-analysis/scratchpad/README.md`

## Notable Clusters

### Instructions Directory (15 subsystems)
The `.github/instructions/` folder documents major VS Code subsystems through markdown instruction files and diagrams:
- Interactive/editor components
- Notebook functionality (cell resizing, hybrid find)
- Sessions management
- Chat integration
- Accessibility features
- Telemetry collection

### Skills Directory (31 expertise domains)
The `.github/skills/` folder contains domain-specific implementations:
- Performance analysis (CPU profiles, heap snapshots, memory leaks)
- Testing frameworks (integration tests, unit tests, component fixtures)
- CI/CD tools (Azure pipelines, Copilot setup)
- Feature implementations (sessions layout, chat customizations)
- Code quality (hygiene, tool deprecation)

### Workflows Directory (16 CI/CD files)
Platform-specific test workflows:
- Linux (CLI and standard)
- macOS (darwin)
- Windows (win32)
- Pull request validation, endgame telemetry, sessions e2e

### Prompts Directory (17 templates)
AI-driven development prompts for code generation and analysis tasks.

## Summary

The `.github/` directory contains 108 files organized into 6 main categories:
1. **Configuration** (7 files) - Project settings, automation rules, release configs
2. **Workflows** (16 files) - GitHub Actions CI/CD pipelines for multi-platform testing
3. **Issue Templates** (4 files) - Bug reports and feature request templates
4. **Agent Documentation** (3 files) - Agentic system documentation
5. **Instructions** (21 files + 6 diagrams) - Subsystem catalogs documenting interactive, notebook, sessions, chat, accessibility, and telemetry subsystems
6. **Prompts** (17 files) - AI prompt templates for development automation
7. **Skills** (31 skills + 5 implementation files + 4 support files) - Domain-specific expertise modules covering performance analysis, testing, CI/CD, features, and code quality

Key subsystems documented as instructions include: accessibility, agent host testing, AI customization, API versioning, chat, interactive components, notebooks, sessions, telemetry, and tree widgets. The skills section identifies performance optimization, testing infrastructure, memory analysis, and feature-specific implementations as major development concerns.
