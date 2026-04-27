# .devcontainer/ - Development Container Configuration

## Overview
The `.devcontainer/` directory contains Docker-based development environment configuration for VS Code - OSS. This partition is not a porting target but rather infrastructure for local and cloud-based development workflows.

### Configuration
- `.devcontainer/devcontainer.json` — Main dev container configuration; specifies TypeScript-Node base image (22-bookworm), desktop-lite and rust features, 9GB memory requirement, VNC forwarding ports (6080, 5901), ESLint/EditorConfig/GitHub PR extensions, and post-create hook
- `.devcontainer/Dockerfile` — Customization layer building on mcr.microsoft.com/devcontainers/typescript-node:22; installs VS Code Insiders, configures npm cache volume mount, sets up display variables
- `.devcontainer/devcontainer-lock.json` — Locked feature versions (desktop-lite@1.2.8, rust@1.5.0) with SHA256 integrity hashes
- `.devcontainer/README.md` — User-facing documentation with local Docker setup instructions, GitHub Codespaces quickstart, VNC access details, debugging instructions, and resource requirements (4-8GB minimum, 9GB recommended)

### Implementation
- `.devcontainer/install-vscode.sh` — Apt package installation script; adds Microsoft package repository and installs code-insiders plus native build dependencies (libsecret-1-dev, libxkbfile-dev, libkrb5-dev)
- `.devcontainer/post-create.sh` — Shell script executed after container creation; runs `npm i` and `npm run electron` to bootstrap the development environment

## Porting Relevance
While this partition is development infrastructure rather than a porting target, it demonstrates the current development stack requirements:
- Base runtime: TypeScript-Node 22 on Debian Bookworm
- Desktop capability: Fluxbox window manager via desktop-lite feature
- Build toolchain: Electron (native module compilation with node-gyp)
- IDE infrastructure: VS Code Insiders with ESLint/GitHub integration
- Optional feature: Rust environment pre-configured (relevant for potential Tauri/Rust porting)

The presence of the Rust feature in the dev container suggests forward-thinking about Rust-based tooling, though currently the post-create hook only builds the TypeScript/Electron stack. Any Tauri-based reimplementation would require substantial modifications to the base image, build commands, and dependency chain.
