# Pattern Research: `.devcontainer/` Analysis

## Scope Assessment

The `.devcontainer/` directory contains Docker-based development environment configuration (2 files, 16 LOC of shell scripts; JSON config files are infrastructure, not application code). This scope is explicitly a **development environment setup**, not a porting target or core IDE functionality.

## Relevant Findings for Porting Research

While the `.devcontainer/` directory is not a porting target, it reveals important infrastructure dependencies and build patterns:

#### Pattern: Electron + Node.js Build Pipeline
**Where:** `.devcontainer/post-create.sh:3-4`
**What:** Development environment assumes Electron-based build with npm dependency management.
```bash
npm i
npm run electron
```
**Variations / call-sites:** The `post-create.sh` directly invokes npm scripts without conditional logic. The README (line 89) shows alternative build path via `bash scripts/code.sh`.

#### Pattern: Desktop GUI Delivery via VNC
**Where:** `.devcontainer/devcontainer.json:39-48`
**What:** VS Code development environment forwards X11 display through VNC (web and TCP), not native windowing.
```json
"forwardPorts": [6080, 5901],
"portsAttributes": {
  "6080": {
    "label": "VNC web client (noVNC)",
    "onAutoForward": "silent"
  },
  "5901": {
    "label": "VNC TCP port",
    "onAutoForward": "silent"
  }
}
```
**Variations / call-sites:** Display forwarding configured conditionally in Dockerfile (line 11): `export DISPLAY="${DISPLAY:-:1}"`. README notes (line 32-33) Wayland support alongside X11.

#### Pattern: System Library Dependencies
**Where:** `.devcontainer/install-vscode.sh:12`
**What:** Native code requires libsecret, xkbfile, and Kerberos libraries.
```bash
apt install -y code-insiders libsecret-1-dev libxkbfile-dev libkrb5-dev
```
**Variations / call-sites:** These dependencies are installed only in the Docker image build, not managed by npm. Indicates tight coupling to system libraries for credential storage and keyboard handling.

#### Pattern: Base Image Selection
**Where:** `.devcontainer/Dockerfile:1`
**What:** Development container built on TypeScript + Node.js base, with Rust features added.
```dockerfile
FROM mcr.microsoft.com/devcontainers/typescript-node:22-bookworm
```
**Where:** `.devcontainer/devcontainer.json:6-9`
**What:** Rust toolchain explicitly included as a dev feature.
```json
"features": {
  "ghcr.io/devcontainers/features/desktop-lite:": {},
  "ghcr.io/devcontainers/features/rust:": {}
}
```
**Variations / call-sites:** Rust is a development tool feature, not the primary build language. No Cargo or Rust-specific build invocations present in the devcontainer setup.

#### Pattern: Resource Constraints
**Where:** `.devcontainer/devcontainer.json:50-52`
**What:** Development environment has explicit memory minimum to accommodate npm builds.
```json
"hostRequirements": {
  "memory": "9gb"
}
```
**Variations / call-sites:** README (line 15-16) emphasizes 9GB RAM as recommended and warns of slow npm operations on macOS/Hyper-V.

## Summary

The `.devcontainer/` scope shows current development infrastructure is entirely **Node.js/npm/Electron-based** with no native Rust or Tauri build patterns present. The setup assumes:

1. **Electron as the runtime**: Post-create hook runs `npm run electron`, not a Tauri build command.
2. **Native system libraries**: Credential storage (libsecret), keyboard handling (libxkbfile), and Kerberos require system-level libraries compiled for the container OS.
3. **GUI delivery through X11/VNC**: Not native window rendering, which Tauri would handle differently.
4. **Heavy npm dependency**: Large npm cache volume mount and npm-gyp for native modules.
5. **Rust is optional tooling**: Included as a dev feature but not integrated into the build pipeline.

**Relevance to Porting:** This infrastructure would need significant restructuring for a Tauri port—different build commands, cargo-based dependency management, different native library bindings, and potentially different display/GUI handling mechanisms depending on target platform.

