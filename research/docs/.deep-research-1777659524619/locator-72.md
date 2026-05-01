# Partition 72: `.devcontainer/` (Dev Container Metadata)

## Configuration

- `.devcontainer/devcontainer.json` - Main dev container configuration
  - Specifies TypeScript/Node.js 22 base environment
  - Includes Rust feature (relevant for Tauri exploration)
  - Desktop-lite feature for GUI support (VNC via ports 6080, 5901)
  - Post-creation hook to `.devcontainer/post-create.sh`
  - VSCode extensions: ESLint, EditorConfig, GitHub PR, Test Provider, Resource Monitor
  - Memory requirement: 9GB
  - Mounts vscode-dev volume for npm cache

- `.devcontainer/Dockerfile` - Container image definition
  - Base: mcr.microsoft.com/devcontainers/typescript-node:22-bookworm
  - Installs VS Code via install-vscode.sh
  - Configures npm cache in shared volume
  - Sets up DISPLAY environment for GUI applications
  - User context: node (non-root)

- `.devcontainer/devcontainer-lock.json` - Dependency lock file (auto-generated)

## Scripts

- `.devcontainer/install-vscode.sh` - VS Code installation script
- `.devcontainer/post-create.sh` - Post-container creation setup

## Documentation

- `.devcontainer/README.md` - Development container guide
  - Quick start instructions for local Docker and GitHub Codespaces
  - VNC connection details (port 5901, default password: vscode)
  - Build requirements and RAM specifications
  - Debugging workflow with VSCode launcher (F5)
  - Notes on using VS Code Insiders and Fluxbox window manager

## Summary

This partition contains minimal but complete dev container infrastructure for VS Code development. The configuration establishes a TypeScript/Node.js environment with Rust support already provisioned (via devcontainers/features/rust), which is directly relevant to Tauri exploration. The container prioritizes GUI development through VNC, supports npm-based builds, and enables VSCode debugging workflows. All files are metadata and orchestration-focused, with no application code present.

