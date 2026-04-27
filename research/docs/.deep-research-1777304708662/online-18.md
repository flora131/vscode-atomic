# Online Research — Partition 18: `.github/` CI Workflows and Copilot Instructions

(no external research applicable)

## Justification

The `.github/` directory contains two categories of files: GitHub Actions workflow YAML files (under `.github/workflows/`) and Copilot-related configuration files (instructions, prompts, skills, agents, and `copilot-instructions.md`).

**Why external research is not warranted for this partition:**

1. **CI workflows are build orchestration, not runtime code.** The workflow files (`pr.yml`, `pr-linux-test.yml`, `pr-darwin-test.yml`, `pr-win32-test.yml`, `pr-node-modules.yml`, `sessions-e2e.yml`, etc.) define how the project's existing TypeScript/Electron codebase is built, linted, and tested in GitHub Actions runners. They reference tooling such as `actions/checkout`, `actions/setup-node`, npm scripts, `cargo clippy`, `cargo test`, and Playwright end-to-end tests. None of these represent architectural patterns or library dependencies that need to be ported; they are the delivery pipeline for whatever codebase exists.

2. **Copilot instruction and prompt files are process documents.** Files such as `copilot-instructions.md`, the `.github/instructions/` markdowns (accessibility, disposable patterns, observables, telemetry, etc.), and `.github/prompts/` are guidance documents for AI-assisted development workflows. They describe coding conventions and patterns for the current TypeScript codebase. They do not encode runtime logic that must be translated to Rust.

3. **No third-party library imports are present.** The workflows invoke actions from `actions/` and standard toolchain setup steps. There are no imports of VS Code extension APIs, Electron APIs, or any other library whose Tauri/Rust counterpart would need to be researched.

4. **The rustup/cargo references in the workflows are already Rust-aware.** The fact that workflows reference `rustup 1.88`, `cargo clippy`, and `cargo test` indicates the project already has some Rust components in its build pipeline. This is consistent with VS Code's existing native modules and CLI tooling written in Rust, but it does not add surface area requiring external documentation lookup for a porting assessment.

5. **Playwright references are for integration testing.** Playwright is used for end-to-end UI testing of the existing Electron application. In a Tauri port, a new test harness would be needed, but the design of that harness is a build-system decision, not something that can be informed by reading the existing CI YAML files.

## Summary

The `.github/` partition is composed entirely of CI pipeline definitions and AI coding-assistant configuration files. Neither category contains runtime logic, dependency declarations, or architectural constructs that are central to evaluating what it would take to port VS Code's core IDE functionality from TypeScript/Electron to Tauri/Rust. The workflows describe how to build and test the current codebase; they do not describe the codebase itself. Fetching external documentation about GitHub Actions, rustup, cargo, or Playwright would add no analytical value to the porting question. Accordingly, this partition is correctly skipped for external online research.
