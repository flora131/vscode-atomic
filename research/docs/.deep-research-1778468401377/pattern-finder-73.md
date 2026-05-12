# DevContainer Configuration Patterns for VS Code Development

## Sentinel Note

The `.devcontainer/` scope contains only development environment configuration files (5 files, ~70 LOC total). These are infrastructure-level configs rather than codebase patterns that would inform a Tauri/Rust port. No reusable architectural patterns relevant to IDE core functionality porting are present in this directory.

## Found Configurations

### Pattern 1: Base TypeScript/Node DevContainer
**Found in**: `.devcontainer/Dockerfile:1`
**Purpose**: Foundation image for VS Code development environment

```dockerfile
FROM mcr.microsoft.com/devcontainers/typescript-node:22-bookworm

ADD install-vscode.sh /root/
RUN /root/install-vscode.sh

RUN git config --system codespaces-theme.hide-status 1

USER node
RUN npm install -g node-gyp
RUN NPM_CACHE="$(npm config get cache)" && rm -rf "$NPM_CACHE" && ln -s /vscode-dev/npm-cache "$NPM_CACHE"
RUN echo 'export DISPLAY="${DISPLAY:-:1}"' | tee -a ~/.bashrc >> ~/.zshrc

USER root
CMD chown node:node /vscode-dev && sudo -u node mkdir -p /vscode-dev/npm-cache && sleep inf
```

**Key aspects**:
- Starts from TypeScript-Node v22 image
- Installs VS Code Insiders + dependencies (libsecret, libxkbfile, libkrb5)
- npm cache shared via volume mount
- Desktop/GUI support via DISPLAY configuration

### Pattern 2: DevContainer Composition
**Found in**: `.devcontainer/devcontainer.json:1-53`
**Purpose**: VS Code-aware container orchestration

```json
{
  "name": "Code - OSS",
  "build": {
    "dockerfile": "Dockerfile"
  },
  "features": {
    "ghcr.io/devcontainers/features/desktop-lite:": {},
    "ghcr.io/devcontainers/features/rust:": {}
  },
  "containerEnv": {
    "DISPLAY": ""
  },
  "overrideCommand": false,
  "privileged": true,
  "mounts": [
    {
      "source": "vscode-dev",
      "target": "/vscode-dev",
      "type": "volume"
    }
  ],
  "postCreateCommand": "./.devcontainer/post-create.sh",
  "hostRequirements": {
    "memory": "9gb"
  }
}
```

**Key aspects**:
- Includes both Rust and Desktop-lite features (forward-looking for Tauri)
- 9GB RAM requirement for development
- Shared volume for npm cache persistence
- VNC ports (6080, 5901) for GUI access
- Extensions: ESLint, EditorConfig, GitHub PR, test provider

### Pattern 3: Post-Create Setup
**Found in**: `.devcontainer/post-create.sh:1-4`
**Purpose**: Initial build/dependency installation

```sh
#!/bin/sh

npm i
npm run electron
```

**Key aspects**:
- Minimal setup: dependencies + electron build
- Runs after container initialization

### Pattern 4: VS Code Installation
**Found in**: `.devcontainer/install-vscode.sh:1-12`
**Purpose**: Install VS Code Insiders with system dependencies

```sh
#!/bin/sh

apt update
apt install -y wget gpg

wget -qO- https://packages.microsoft.com/keys/microsoft.asc | gpg --dearmor > packages.microsoft.gpg
install -D -o root -g root -m 644 packages.microsoft.gpg /etc/apt/keyrings/packages.microsoft.gpg
sh -c 'echo "deb [arch=amd64,arm64,armhf signed-by=/etc/apt/keyrings/packages.microsoft.gpg] https://packages.microsoft.com/repos/code stable main" > /etc/apt/sources.list.d/vscode.list'
rm -f packages.microsoft.gpg

apt update
apt install -y code-insiders libsecret-1-dev libxkbfile-dev libkrb5-dev
```

**Key aspects**:
- Installs VS Code Insiders distribution
- Includes cryptography libs (libsecret, libkrb5) for secrets integration
- libxkbfile for keyboard handling

## Implications for Tauri/Rust Port

The devcontainer already includes a `rust:` feature in `devcontainer.json:8`, suggesting infrastructure awareness of Rust tooling. However, the current setup is TypeScript/Node-centric with:
- Heavy dependency on npm ecosystem
- Electron as build target
- GUI development via noVNC in container
- VS Code Insiders as the development IDE

A Tauri port would require:
1. Shift base image from `typescript-node:22` to `rust:latest` or similar
2. Replace `npm i && npm run electron` with `cargo build` equivalents
3. Adjust system dependencies (swap libxkbfile for Tauri-specific deps)
4. Reconsider GUI testing approach (Tauri uses webview, not Electron)
5. Modify npm cache volume to Rust cargo cache strategy

The current setup provides the **infrastructure baseline** but not architectural patterns for the actual IDE functionality porting task.
