# Partition 40: Resources Directory

Comprehensive inventory of VS Code's platform-specific assets, scripts, and configuration files (115 files across 20 directories).

## Implementation

### Launcher Scripts (10 files)
Platform-specific entry-point scripts that invoke VS Code:
- `/resources/linux/bin/code.sh` - Linux CLI launcher; handles WSL detection, root checks, symlink resolution
- `/resources/darwin/bin/code.sh` - macOS launcher script
- `/resources/win32/bin/code.cmd` - Windows batch launcher
- `/resources/win32/bin/code.sh` - Windows Unix-like launcher variant
- `/resources/win32/versioned/bin/code.cmd` - Versioned Windows launcher
- `/resources/win32/versioned/bin/code.sh` - Versioned Unix variant
- `/resources/server/bin/code-server-darwin.sh` - Code Server macOS launcher
- `/resources/server/bin/code-server-linux.sh` - Code Server Linux launcher
- `/resources/server/bin/code-server.cmd` - Code Server Windows launcher
- `/resources/server/bin-dev/remote-cli/code.sh` - Development remote CLI launcher

### Remote CLI Scripts (7 files)
Scripts for remote development and headless modes:
- `/resources/server/bin/remote-cli/code-darwin.sh` - Remote CLI for macOS
- `/resources/server/bin/remote-cli/code-linux.sh` - Remote CLI for Linux
- `/resources/server/bin/remote-cli/code.cmd` - Remote CLI for Windows
- `/resources/server/bin-dev/remote-cli/code.cmd` - Development mode remote CLI (Windows)
- `/resources/server/bin-dev/remote-cli/code.sh` - Development mode remote CLI (Unix)

### Browser Helper Scripts (5 files)
Utilities for opening browsers and checking system requirements:
- `/resources/server/bin/helpers/browser-darwin.sh` - Browser launch helper (macOS)
- `/resources/server/bin/helpers/browser-linux.sh` - Browser launch helper (Linux)
- `/resources/server/bin/helpers/browser.cmd` - Browser launch helper (Windows)
- `/resources/server/bin-dev/helpers/browser.sh` - Development mode browser helper (Unix)
- `/resources/server/bin-dev/helpers/browser.cmd` - Development mode browser helper (Windows)
- `/resources/server/bin/helpers/check-requirements-linux.sh` - Linux system requirement verification

### Snap Launcher
- `/resources/linux/snap/electron-launch` - Launcher wrapper for Snap environment

## Configuration

### Platform Manifests (4 XML files)

**Windows (2 files):**
- `/resources/win32/VisualElementsManifest.xml` - Windows Start menu tile configuration (background color, logo paths, display name)
- `/resources/win32/appx/AppxManifest.xml` - Windows App Store manifest; declares app identity, capabilities (microphone), file explorer context menu extensions, COM server configuration for file association handlers

**Linux (2 files):**
- `/resources/linux/code.desktop` - FreeDesktop.org desktop entry; defines app launch, MIME types (workspace files), desktop categories, actions (new-empty-window)
- `/resources/linux/code-url-handler.desktop` - Desktop entry for URL protocol handler registration

### Linux Packaging Templates (5 template files)
Located in `/resources/linux/debian/` and `/resources/linux/rpm/`:

**Debian (5 templates):**
- `control.template` - Debian package metadata (version, dependencies, maintainer, homepage)
- `postinst.template` - Post-installation script
- `postrm.template` - Post-removal script
- `prerm.template` - Pre-removal script
- `templates.template` - Debconf template for interactive prompts

**RPM (1 template):**
- `/resources/linux/rpm/code.spec.template` - RPM spec file; includes build configuration, install directives, post-installation mime database updates, desktop database updates, bash/zsh completion installation

### Linux Metadata
- `/resources/linux/code.appdata.xml` - AppStream metadata for software centers
- `/resources/linux/code-workspace.xml` - MIME type definition for VS Code workspace files

### Snap Configuration
- `/resources/linux/snap/snapcraft.yaml` - Snap package manifest; defines build process, stage packages (GTK3, X11, Wayland libs), architecture, confinement (classic), prime directives for cleanup

### Web Manifest
- `/resources/server/manifest.json` - PWA manifest for browser-based Code Server; defines app name, icons (192x192, 512x512), display mode (standalone), display override (window-controls-overlay)

## Documentation

### Shell Completions (2 files)
- `/resources/completions/bash/code` - Bash completion script for `code` command; includes options like `--diff`, `--goto`, `--locale`, `--install-extension`, etc.
- `/resources/completions/zsh/_code` - Zsh completion function

## Types / Interfaces

Not applicable. This partition contains binary assets and configuration templates, not source code or type definitions.

## Examples / Fixtures

### Application Icons

**macOS Icons (.icns format, 29 files):**
Located in `/resources/darwin/`, covering file type associations:
- `code.icns` - Main application icon
- `default.icns` - Generic file icon
- Language/framework icons: `javascript.icns`, `typescript.icns`, `python.icns`, `java.icns`, `cpp.icns`, `c.icns`, `csharp.icns`, `go.icns`, `ruby.icns`, `php.icns`, `powershell.icns`, `html.icns`, `css.icns`, `json.icns`, `yaml.icns`, `xml.icns`, `markdown.icns`, `less.icns`, `sass.icns`, `config.icns`, `vue.icns`, `react.icns`, `jade.icns`, `shell.icns`, `sql.icns`, `bower.icns`, `bat.icns`

**Windows Icons (.ico format, 29 files):**
Located in `/resources/win32/`, same file type associations as macOS:
- `code.ico` - Main application icon
- `default.ico` - Generic file icon
- Same language/framework icons as macOS (in .ico format)

**UI Icons for Windows:**
- `code_150x150.png` - Tile icon for Windows Start menu
- `code_70x70.png` - Small tile icon
- `code_150x150.png` - also used in AppxManifest.xml

**Linux Icons:**
- `/resources/linux/code.png` - PNG application icon
- `/resources/linux/rpm/code.xpm` - XPM format icon for RPM

### Server/Web Icons
- `/resources/server/code-192.png` - PWA icon (192x192)
- `/resources/server/code-512.png` - PWA icon (512x512)
- `/resources/server/favicon.ico` - Browser favicon

### Windows Installer Assets (14 bitmap files for Inno Setup)
Located in `/resources/win32/`:
- `inno-big-100.bmp` through `inno-big-250.bmp` (7 files) - Large installer graphics at 100%, 125%, 150%, 175%, 200%, 225%, 250% DPI
- `inno-small-100.bmp` through `inno-small-250.bmp` (7 files) - Small installer graphics at same DPI scales

## Notable Clusters

### Platform-Specific Directory Structure
```
resources/
├── completions/          (2 files) - Shell autocompletion scripts (bash, zsh)
├── darwin/              (31 files) - macOS: 29 .icns app icons + 1 launcher script + 1 app icon
├── linux/               (38 files) - Linux packaging and desktop integration
│   ├── bin/             (1 file) - Linux launcher script
│   ├── debian/          (5 files) - Debian/Ubuntu packaging templates
│   ├── rpm/             (2 files) - RPM packaging template + icon
│   ├── snap/            (2 files) - Snap packaging config + electron launcher
│   └── desktop + metadata (4 files) - .desktop entries and AppStream metadata
├── win32/               (43 files) - Windows: 29 .ico icons + 14 installer bitmaps + manifests + launchers
│   ├── appx/            (1 file) - Windows Store AppxManifest
│   ├── bin/             (2 files) - Windows launchers
│   ├── versioned/       (2 files) - Versioned launchers
│   └── manifests + icons
└── server/              (15 files) - Code Server / web assets
    ├── bin/             (7 files) - Launchers and helpers for production
    ├── bin-dev/         (4 files) - Development launchers and helpers
    └── web assets       (4 files) - manifest.json, favicon, icons (192/512)
```

### Packaging Integration Points
1. **Debian (5 templates)**: `control.template`, `postinst.template`, `postrm.template`, `prerm.template`, `templates.template` - Full lifecycle control for deb packaging
2. **RPM (1 template)**: `code.spec.template` - Handles installation, desktop database updates, mime type registration
3. **Snap (1 yaml)**: `snapcraft.yaml` - Stage packages include GTK3, X11, Wayland, audio libraries
4. **Windows AppX (1 manifest)**: `AppxManifest.xml` - Declares file association context menu handlers, COM server configuration, microphone capability

### Platform-Specific Build Assets
- **Windows Installer**: 14 bitmap images at 7 DPI scales (100%-250%) for Inno Setup dialogs
- **macOS**: 30 icon files (.icns format) for app and file associations
- **Linux (Desktop)**: Desktop entries for MIME association, shell completions, appdata metadata
- **Web/Server**: PWA manifest, web icons, browser helpers

---

**Summary**: The `resources/` directory (115 files) contains platform-specific application assets and build configuration essential for distribution across Windows (AppX/Inno Setup), macOS (native app), Linux (deb/rpm/snap), and web (Code Server). A Tauri port would require equivalent platform-specific assets: Windows NSIS/WiX installers instead of Inno Setup, macOS app bundle signing certificates, Linux packaging templates, and web launcher assets. All icon formats, desktop integration files, and installation scripts are tightly coupled to their respective distribution mechanisms and would need Tauri-compatible equivalents.
