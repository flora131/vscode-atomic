# Partition 18 of 80 — Findings

## Scope
`.github/` (9 files, 4,624 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
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

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
_(no analysis produced)_

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Pattern Finder 18: VS Code → Tauri/Rust Port — CI/Build Architecture

**Scope:** `.github/` directory (workflows, instructions, skills).

**Sentinel Finding:** This partition documents **build orchestration and test harness patterns** that reveal the multi-platform architecture any Tauri port must replicate.

---

## Core Patterns from CI/Build

#### Pattern: Multi-Platform Test Matrix
**Where:** `.github/workflows/pr.yml:115-176`
**What:** Parallel test execution across three rendering engines (Electron, Browser, Remote) and three platforms (Linux, macOS, Windows). Each platform runs the same test suite permutations.

```yaml
linux-electron-tests:
  name: Linux
  uses: ./.github/workflows/pr-linux-test.yml
  with:
    job_name: Electron
    electron_tests: true

linux-browser-tests:
  name: Linux
  uses: ./.github/workflows/pr-linux-test.yml
  with:
    job_name: Browser
    browser_tests: true

linux-remote-tests:
  name: Linux
  uses: ./.github/workflows/pr-linux-test.yml
  with:
    job_name: Remote
    remote_tests: true

# Pattern repeats for macos-*-tests and windows-*-tests
```

**Variations:** `.github/workflows/pr-linux-test.yml:282-378` (Electron/Browser/Remote conditional execution), `pr-darwin-test.yml`, `pr-win32-test.yml`.

---

#### Pattern: CLI Rust Build Pipeline
**Where:** `.github/workflows/pr-linux-cli-test.yml:23-48`
**What:** Dedicated Rust toolchain setup and cargo-based build/test. Shows existing Rust infrastructure for CLI tools, distinct from TypeScript/Electron main codebase.

```yaml
- name: Install Rust
  run: |
    set -e
    curl https://sh.rustup.rs -sSf | sh -s -- -y --profile minimal --default-toolchain $RUSTUP_TOOLCHAIN
    echo "$HOME/.cargo/bin" >> $GITHUB_PATH

- name: Set Rust version
  run: |
    set -e
    rustup default $RUSTUP_TOOLCHAIN
    rustup update $RUSTUP_TOOLCHAIN
    rustup component add clippy

- name: Clippy lint
  run: cargo clippy -- -D warnings
  working-directory: cli

- name: 🧪 Run unit tests
  run: cargo test
  working_directory: cli
```

**Key aspects:**
- Separate `cli/` workspace directory
- Rustup pinned to input `toolchain` version (e.g. 1.88)
- Clippy linting required before tests pass
- Cargo workspaces isolated from Node.js build

**Variations:** Same pattern invoked from main workflow at `.github/workflows/pr.yml:108-113`.

---

#### Pattern: Transpile vs. Bundle Phases
**Where:** `.github/instructions/buildNext.instructions.md:6-120`
**What:** Two-stage build system splitting fast iteration (transpile, type-check only) from production (bundle, minify). Critical for understanding how Tauri would need separate dev and release builds.

**Transpile (Dev/Watch):**
```typescript
// node build/next/index.ts transpile --out out-test
// Fast TS → JS via esbuild.transform()
// Copies ALL non-TS files from src/
// Type-checking deferred to separate tsgo process
```

**Bundle (Production):**
```typescript
// node build/next/index.ts bundle --nls --target server-web --out out-vscode-reh-web-test
// TS → bundled JS via esbuild.build()
// Applies minification, NLS placeholder injection, source maps
// Curated resource lists (not catch-all)
```

**Variations:** Desktop, web, server-web targets have different entry points (`.github/instructions/buildNext.instructions.md:88-96`).

---

#### Pattern: Node Modules Caching Strategy
**Where:** `.github/workflows/pr.yml:34-46`, `pr-linux-test.yml:58-70`
**What:** Hash-based caching of `node_modules` across workflow runs using computed package-lock digest. Shows how to avoid expensive reinstalls on platforms with native builds.

```bash
# Compute cache key from package-lock + platform + arch
node build/azure-pipelines/common/computeNodeModulesCacheKey.ts linux $VSCODE_ARCH $(node -p process.arch) > .build/packagelockhash

# Restore from cache
uses: actions/cache/restore@v5
with:
  path: .build/node_modules_cache
  key: "node_modules-linux-${{ hashFiles('.build/packagelockhash') }}"

# Extract and skip npm ci if cache hit
if: steps.cache-node-modules.outputs.cache-hit == 'true'
  run: tar -xzf .build/node_modules_cache/cache.tgz
```

**Variations:** Built-in extensions cached separately (`.github/workflows/pr.yml:86-92`), Copilot extension build cache (`.github/workflows/pr.yml:194-204`).

---

#### Pattern: Resilient Network Operations
**Where:** `.github/workflows/pr-linux-test.yml:261-278`, `pr.yml:54-64`
**What:** Exponential backoff retry loops for npm ci and Electron/Playwright downloads. Critical for flaky network in CI environments.

```bash
for i in {1..5}; do # try 5 times
  npm ci && break
  if [ $i -eq 5 ]; then
    echo "Npm install failed too many times" >&2
    exit 1
  fi
  echo "Npm install failed $i, trying again..."
done

for i in {1..3}; do # Electron/Playwright: 3 retries
  if npm exec -- npm-run-all2 -lp "electron ${{ env.VSCODE_ARCH }}" "playwright-install"; then
    echo "Download successful on attempt $i"
    break
  fi
  if [ $i -eq 3 ]; then
    echo "Download failed after 3 attempts" >&2
    exit 1
  fi
  echo "Download failed on attempt $i, retrying..."
  sleep 5
done
```

**Key aspects:**
- npm ci has 5 retries (compilation-heavy)
- Electron/Playwright has 3 retries with 5s delay
- Clear failure messages and exit codes

---

#### Pattern: Cross-Platform System Setup
**Where:** `.github/workflows/pr-linux-test.yml:36-56` (Linux), `pr-darwin-test.yml:50-76` (macOS)
**What:** Platform-specific dependency installation and environment setup. Shows surface area a Tauri port must handle.

**Linux:**
```bash
./build/azure-pipelines/linux/apt-retry.sh sudo apt-get update
./build/azure-pipelines/linux/apt-retry.sh sudo apt-get install -y pkg-config \
  xvfb libgtk-3-0 libxkbfile-dev libkrb5-dev libgbm1 rpm bubblewrap socat
sudo service xvfb start
```

**macOS:**
```bash
c++ --version
xcode-select -print-path
python3 -m pip install --break-system-packages setuptools
# GYP_DEFINES: "kerberos_use_rtld=false" (Kerberos dlopen workaround)
```

**Windows:** (implied from `pr-win32-test.yml` invocation) — PowerShell-based, 7z archives, native build tools.

---

#### Pattern: Test Suite Organization
**Where:** `.github/workflows/pr-linux-test.yml:282-378`
**What:** Layered test execution: unit tests → integration tests → smoke tests, with platform-specific runners and artifact collection on failure.

```bash
# Unit tests (Electron + node.js)
./scripts/test.sh --tfs "Unit Tests"
npm run test-node

# Unit tests (Browser)
npm run test-browser-no-install -- --browser chromium --tfs "Browser Unit Tests"

# Integration tests (compile first)
npm run gulp compile-extension:* ...
./scripts/test-integration.sh --tfs "Integration Tests"

# Smoke tests
npm run smoketest-no-compile -- --tracing

# Artifact collection on failure
- name: Publish Crash Reports
  uses: actions/upload-artifact@v7
  if: failure()
  with:
    name: crash-dump-linux-${{ env.VSCODE_ARCH }}-...
    path: .build/crashes
```

**Variations:** Browser tests use `--headless` flag, remote tests use `--remote` flag, all test scripts source from `./scripts/test*.sh` or `npm run` commands.

---

#### Pattern: Hygiene and Type Checking
**Where:** `.github/workflows/pr.yml:100-106`
**What:** Pre-compilation validation gates: TypeScript type checking on build scripts, ESLint, dependency cycle detection.

```yaml
- name: Compile & Hygiene
  run: npm exec -- npm-run-all2 -lp core-ci hygiene eslint valid-layers-check define-class-fields-check vscode-dts-compile-check tsec-compile-check test-build-scripts

- name: Check cyclic dependencies
  run: node build/lib/checkCyclicDependencies.ts out-build
```

**Key commands:**
- `core-ci` — main esbuild transpile
- `hygiene` — code style/formatting
- `eslint` — linting
- `valid-layers-check` — architecture validation
- `checkCyclicDependencies` — graph analysis

---

## Architecture Insights for Porting

#### 1. **Multiple Runtime Targets Must Be Testable**
The workflows enforce parity across Electron (desktop), Browser (web), and Remote (SSH/tunnel). A Tauri port would need:
- Separate test matrix for native Tauri desktop builds
- Continued support for web/browser targets
- Remote testing infrastructure (if keeping tunneling)

#### 2. **Rust Toolchain Is Already Isolated**
The `cli/` Rust workspace is tested separately from the main Node.js build. A Tauri port could extend this pattern: keep Rust builds in a designated workspace, with separate rustup toolchain pinning and cargo-based CI.

#### 3. **Build Stages Are Distinct**
Transpile (fast, type-check deferred) vs. Bundle (slow, minified, resource-curated) suggests a Tauri port needs:
- Fast dev iteration via transpile-like Rust incremental builds
- Separate production bundling step
- Type checking in parallel (not blocking transpile)

#### 4. **Platform-Specific Setup Is Inevitable**
Linux (apt + xvfb), macOS (Xcode + GYP), Windows (PowerShell + 7z) require per-OS CI branches. Tauri's native layer would require similar or more platform-specific logic.

#### 5. **Crash Reporting and Diagnostics Are Built-in**
Workflow artifacts capture crash dumps, node modules, and logs on failure. A Tauri port would need equivalent crash reporter integration and artifact collection.

#### 6. **Retry Logic for Flaky Operations Is Essential**
Network operations (npm ci, downloads) use exponential backoff. Any Tauri port CI should inherit this pattern for downloading Rust deps, Tauri CLI, etc.

---

## Summary

The `.github/` directory reveals a **multi-platform, multi-stage CI architecture** with:
- 9 concurrent test jobs (3 platforms × 3 runtimes)
- Separate Rust CLI build pipeline (already in use)
- Two-phase TypeScript build (transpile + bundle)
- Extensive caching and retry logic
- Platform-specific system dependencies

Any Tauri port must replicate these patterns, likely by extending the existing Rust pipeline and introducing native Tauri-specific targets to the test matrix.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
