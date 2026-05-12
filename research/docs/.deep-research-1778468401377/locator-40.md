# Locator 40: Packaging & Static Assets (resources/)

## Overview
The `resources/` directory contains 115 static files organized by platform (darwin/macOS, linux, win32, server) and by artifact type (icons, manifests, scripts, completion specs, installers). These are critical Tauri bundler targets that must be reproduced for cross-platform distribution.

---

## Implementation

### Platform-Specific Asset Trees

#### macOS (resources/darwin/) — 32 icns files + launcher
- `resources/darwin/code.icns` — Main application icon
- `resources/darwin/default.icns` — Generic file icon
- `resources/darwin/bin/code.sh` — macOS launch wrapper script
- `resources/darwin/{bat,bower,c,cpp,csharp,css,go,html,jade,java,javascript,json,less,markdown,php,powershell,python,react,ruby,sass,shell,sql,typescript,vue,xml,yaml}.icns` — Language/framework-specific file type icons (26 files)

#### Linux (resources/linux/) — Desktop entries, AppData, package templates, PNG icon
- `resources/linux/code.desktop` — Desktop entry for application launcher
- `resources/linux/code-url-handler.desktop` — URL protocol handler registration (vscode:// links)
- `resources/linux/code-workspace.xml` — MIME type definition for .code-workspace files
- `resources/linux/code.appdata.xml` — AppStream metadata for software stores/discovery
- `resources/linux/code.png` — 256x256 PNG icon for desktop environments
- `resources/linux/bin/code.sh` — Linux launch wrapper script
- `resources/linux/debian/` — Debian/Ubuntu packaging (5 templates)
  - `control.template` — Package metadata (name, version, dependencies, maintainer)
  - `postinst.template` — Post-installation script (symlink creation, update-alternatives setup)
  - `postrm.template` — Post-removal script (cleanup, alternative unregistration)
  - `prerm.template` — Pre-removal script (pre-removal tasks)
  - `templates.template` — Debconf template for interactive prompts
- `resources/linux/rpm/` — RPM/Fedora packaging (2 files)
  - `code.spec.template` — RPM spec file (build instructions, dependencies, files, scripts)
  - `code.xpm` — X11 pixmap icon for RPM metadata
- `resources/linux/snap/` — Snap packaging (2 files)
  - `snapcraft.yaml` — Snap manifest (confinement, plugs, hooks, commands)
  - `electron-launch` — Electron wrapper for snap execution environment

#### Windows (resources/win32/) — ICO files, installer graphics, manifests, scripts (57 files)
- `resources/win32/code.ico` — Main application icon (ICO format, multiple resolutions)
- `resources/win32/default.ico` — Generic file icon
- `resources/win32/code_150x150.png` — Tile icon for Windows Start menu
- `resources/win32/code_70x70.png` — Smaller tile variant
- `resources/win32/VisualElementsManifest.xml` — Windows 8+ Modern App metadata (tile colors, icons)
- `resources/win32/appx/AppxManifest.xml` — Windows Store/MSIX app manifest (app identity, capabilities, visual assets)
- `resources/win32/bin/code.cmd` — Windows command script launcher
- `resources/win32/bin/code.sh` — Windows bash/WSL launcher script
- `resources/win32/versioned/bin/` — Version-specific launcher variants (2 files)
  - `code.cmd` — Versioned command wrapper
  - `code.sh` — Versioned bash wrapper
- `resources/win32/{bat,bower,c,cpp,csharp,css,go,html,jade,java,javascript,json,less,markdown,php,powershell,python,react,ruby,sass,shell,sql,typescript,vue,xml,yaml}.ico` — Language/framework file type icons (26 files)
- `resources/win32/inno-{big,small}-{100,125,150,175,200,225,250}.bmp` — Inno Setup installer graphics (14 BMP files for various DPI scaling scenarios)

#### Server/Remote (resources/server/) — Web server launcher, manifest, icons
- `resources/server/manifest.json` — Web app manifest (name, icons, app metadata)
- `resources/server/code-192.png` — PWA icon 192x192
- `resources/server/code-512.png` — PWA icon 512x512
- `resources/server/favicon.ico` — Favicon for web interface
- `resources/server/bin/` — Production server launchers
  - `code-server.cmd` — Windows server launcher
  - `code-server-darwin.sh` — macOS server launcher
  - `code-server-linux.sh` — Linux server launcher
  - `helpers/` — Helper scripts (browser.cmd, browser-darwin.sh, browser-linux.sh, check-requirements-linux.sh)
  - `remote-cli/` — Remote CLI launchers (code.cmd, code-darwin.sh, code-linux.sh)
- `resources/server/bin-dev/` — Development server variants (mirrors production structure)
  - `remote-cli/` — Dev remote CLI scripts
  - `helpers/` — Dev helper scripts

#### Shell Completions (resources/completions/)
- `resources/completions/bash/code` — Bash completion spec for `code` command
- `resources/completions/zsh/_code` — Zsh completion spec for `code` command

---

## Configuration

### Installer & Distribution Metadata
- **Debian/Ubuntu**: `resources/linux/debian/control.template` defines package dependencies, description, architecture
- **RPM/Fedora**: `resources/linux/rpm/code.spec.template` defines build rules, file installation, scriptlets
- **Snap**: `resources/linux/snap/snapcraft.yaml` defines sandboxing (confinement: classic), plugs (hardware access)
- **Windows Installer**: `resources/win32/inno-*.bmp` graphics for Inno Setup (visual branding during installation)
- **Windows Store**: `resources/win32/appx/AppxManifest.xml` and `VisualElementsManifest.xml` for Microsoft Store metadata

### Platform Launch & Integration
- **macOS**: `resources/darwin/bin/code.sh` — Launch script for app bundle
- **Linux**: `resources/linux/code.desktop` — Freedesktop.org desktop entry for GUI launcher; `code-url-handler.desktop` for protocol routing
- **Windows**: `resources/win32/bin/code.cmd` and `.sh` — CLI entry points; `appx/AppxManifest.xml` for OS integration
- **Web/Server**: `resources/server/manifest.json` — PWA manifest for web deployment

---

## Examples / Fixtures

### Package Configuration Templates
- `resources/linux/debian/control.template` — Real Debian control file template (shows structure for OS integration)
- `resources/linux/rpm/code.spec.template` — Real RPM spec template
- `resources/linux/snap/snapcraft.yaml` — Real snap confinement + hook definitions
- `resources/win32/appx/AppxManifest.xml` — Real UWP app manifest with capabilities

### Shell Scripts
- `resources/darwin/bin/code.sh` — Shell script showing macOS app bundle launching
- `resources/linux/bin/code.sh` — Linux CLI launcher
- `resources/server/bin/code-server-*.sh` — Multi-platform server startup
- `resources/completions/bash/code` and `resources/completions/zsh/_code` — Shell completion examples

### Visual Assets (Icon/Branding)
- macOS icons: `.icns` format, 32x32 to 1024x1024 resolution variants per icon
- Windows icons: `.ico` format, multiple resolution variants embedded
- Windows installer DPI graphics: `inno-{big,small}-{100,125,150,175,200,225,250}.bmp` (7 size variants for 100%-250% scaling)
- Linux: Single `code.png` (256x256) + `code.xpm` (for RPM)
- Web: `code-{192,512}.png` for PWA manifest

---

## Notable Clusters

### Platform-Specific Icon Sets (File Type Association)
All platforms maintain 26+ language/framework icons (bat, bower, c, cpp, csharp, css, go, html, jade, java, javascript, json, less, markdown, php, powershell, python, react, ruby, sass, shell, sql, typescript, vue, xml, yaml):
- **darwin/**: `.icns` format (32 files)
- **win32/**: `.ico` format (26 files)
- **linux/**: No individual language icons (single generic icon)

### Linux Packaging Matrix
Three distinct packaging ecosystems, each with own manifest/script templates:
- **Debian/Ubuntu** (5 files): control metadata + 3 shell script templates (postinst/prerm/postrm) + debconf template
- **RPM/Fedora** (2 files): spec file template + pixmap icon
- **Snap** (2 files): YAML manifest + electron-launch wrapper

### Installer Graphics (Windows-Specific)
14 BMP files covering DPI scaling (100%, 125%, 150%, 175%, 200%, 225%, 250%) in two size variants (big/small) — all for Inno Setup visual branding.

### Cross-Platform Launcher Scripts
- **Platform-specific launchers**: `darwin/bin/code.sh`, `linux/bin/code.sh`, `win32/bin/code.cmd`, `server/bin/code-server-*.{sh,cmd}`
- **Completion specs**: Bash and Zsh completion definitions for CLI UX
- **URL handlers**: `linux/code-url-handler.desktop` for protocol-based app invocation

### Web/PWA Assets
- `server/manifest.json` — Web app manifest
- `server/code-{192,512}.png` — Progressive web app icons
- `server/favicon.ico` — Browser tab icon
- `server/bin/code-server-*.sh` — Web server launch scripts

---

## Summary

The `resources/` directory (115 files) is a platform-distribution layer: macOS icns bundles, Linux desktop/AppData/package templates (Debian/RPM/Snap), Windows UWP/installer manifests and DPI-scaled graphics, server manifests for web deployment, shell completions, and cross-platform launcher scripts. A Tauri port must reconstruct this entire asset tree — mapping Electron-era installer logic (Inno Setup, Debian postinst, Snap plugs) to Tauri's bundler configuration, while preserving per-platform icon formats, shell integration (desktop entries, completion specs), and package metadata semantics (MSIX capabilities, Snap confinement, Debian dependencies).
