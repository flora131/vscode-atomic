# File Locations for Partition 40 — `resources/`

## Implementation / Configuration / Examples

### Launcher Scripts (Platform-Specific Entry Points)

**Linux Desktop Environment Integration**
- `resources/linux/code.desktop` — Desktop entry file for GNOME/KDE application menus; defines Exec, Icon, MimeType, and category metadata
- `resources/linux/code-url-handler.desktop` — URL protocol handler registration for custom scheme (`x-scheme-handler/code://`)
- `resources/linux/code-workspace.xml` — MIME type definition for VS Code workspace files (`.code-workspace`)
- `resources/linux/code.appdata.xml` — AppData metadata for Linux software centers (screenshots, description, homepage)

**macOS App Bundle Integration**
- `resources/darwin/bin/code.sh` — Bash launcher for macOS; resolves `.app` bundle path, sets up Electron environment variables, runs CLI via `ELECTRON_RUN_AS_NODE=1`

**Windows Package Integration**
- `resources/win32/bin/code.cmd` — CMD batch launcher for Windows; sets `ELECTRON_RUN_AS_NODE=1`, invokes Electron with CLI script
- `resources/win32/bin/code.sh` — POSIX shell launcher for Windows Subsystem for Linux (WSL); includes WSL detection and warnings

**Server/Remote CLI**
- `resources/server/bin/code-server.cmd` — Windows server launcher
- `resources/server/bin/code-server-darwin.sh` — macOS server launcher
- `resources/server/bin/code-server-linux.sh` — Linux server launcher
- `resources/server/bin/helpers/browser.cmd` — Windows browser helper script
- `resources/server/bin/helpers/browser-darwin.sh` — macOS browser helper
- `resources/server/bin/helpers/browser-linux.sh` — Linux browser helper
- `resources/server/bin/helpers/check-requirements-linux.sh` — Linux system requirements validation
- `resources/server/bin/remote-cli/code.cmd` — Remote CLI for Windows
- `resources/server/bin/remote-cli/code-darwin.sh` — Remote CLI for macOS
- `resources/server/bin/remote-cli/code-linux.sh` — Remote CLI for Linux
- `resources/server/bin-dev/` — Development variants of server helpers

### Package Management & Distribution

**Debian/Ubuntu Packaging**
- `resources/linux/debian/control.template` — Debian package metadata (version, architecture, dependencies, maintainer, homepage)
- `resources/linux/debian/postinst.template` — Post-install script; creates `/usr/bin/code` symlink, registers desktop entry, updates MIME database, configures apt repository with GPG key validation
- `resources/linux/debian/prerm.template` — Pre-removal script
- `resources/linux/debian/postrm.template` — Post-removal script
- `resources/linux/debian/templates.template` — Debconf user prompts template

**RPM/Fedora Packaging**
- `resources/linux/rpm/code.spec.template` — RPM specfile; defines build, install, post/postun hooks, sets permissions (chrome-sandbox as 4755), disables ELF stripping
- `resources/linux/rpm/code.xpm` — Icon file for RPM packages

**Snap Packaging**
- `resources/linux/snap/snapcraft.yaml` — Snap confinement & build config; specifies core20 base, dumps plugin, sets stage-packages (GTK3, Wayland, NSS, libsecret, audio), patchelf override-build for chrome_crashpad_handler, classic confinement with electron-launch wrapper
- `resources/linux/snap/electron-launch` — Snap environment setup launcher; handles Fedora `/var/lib/snapd` path remapping, GIO/GSettings module compilation, Wayland detection, GTK IM module setup, XDG paths, font config, cache management

**Windows Store (AppX)**
- `resources/win32/appx/AppxManifest.xml` — UWP manifest for Microsoft Store; declares capabilities (runFullTrust, unvirtualizedResources, microphone), file explorer context menu handler registration with COM surrogate, display names, logos, target Windows 10.0.19041.0+

**Windows Desktop Customization**
- `resources/win32/VisualElementsManifest.xml` — Windows taskbar/tile appearance; background color, square logos (150x150, 70x70), shortcut name

### Shell Completions

- `resources/completions/bash/code` — Bash command-line completion script; supports options like `--diff`, `--add`, `--goto`, `--locale`, `--install-extension`, `--log`, `--enable-proposed-api`
- `resources/completions/zsh/_code` — Zsh completion script (stub reference only; full file not shown)

### Web App Manifest

- `resources/server/manifest.json` — PWA manifest for web-based VS Code server; defines name, start_url, display mode (window-controls-overlay), icon sizes (192x192, 512x512)

## Summary

The `resources/` directory contains 115 files, predominantly binary assets (icons: `.ico`, `.icns`, `.png`, `.bmp`), but also houses critical non-binary configuration:

- **8 Platform-Specific Launcher Scripts** (Linux shell, macOS bash, Windows batch/POSIX)
- **3 Linux Desktop Integration Files** (`.desktop`, `.appdata.xml`, MIME registration)
- **3 Debian Packaging Templates** (control metadata, post-install with APG key management, pre/post-remove hooks)
- **2 RPM/Fedora Packaging** (spec file with ELF stripping disabled, icon)
- **2 Snap Packaging** (snapcraft.yaml with Wayland/GTK environment setup, electron-launch bootstrap)
- **1 UWP/Store Manifest** (AppX with COM handler + microphone capability)
- **1 Windows Branding** (taskbar tile manifest)
- **2 Shell Completion** (Bash with full option coverage, Zsh reference)
- **1 PWA Manifest** (web server variant)

For a Tauri/Rust port, the key artifacts needing Tauri equivalents are:

1. **Launcher Scripts**: Replace Electron entry points with Tauri binary invocation across OS platforms
2. **Desktop/Appdata Files**: Keep Linux `.desktop` and `.appdata.xml` (mostly format-compatible); update `Exec` paths
3. **Package Metadata**: Adapt control.template, spec.template for Tauri binary size/dependencies; snapcraft.yaml requires core22+ base and new plugin instead of dump
4. **AppX Manifest**: Preserve Windows Store integration structure but reference Tauri runtime instead of Electron executable
5. **Launcher Wrappers**: electron-launch snap wrapper needs Tauri equivalent environment setup
6. **Shell Completions**: Retain as-is (CLI interface typically unchanged)
7. **PWA Manifest**: Keep for web server variant

The snap/electron-launch script is particularly noteworthy: it compiles GIO modules, sets up GTK IM modules, handles Wayland detection, and manages font/cache paths—all environment concerns a Tauri snap would inherit.
