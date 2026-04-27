# Partition 40 of 79 — Findings

## Scope
`resources/` (20 files, 600 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
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

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
_(no analysis produced)_

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Pattern Finder Report: Partition 40 - Resources Directory
## Executable Scripts and Platform-Specific Configuration Patterns

**Scope:** `resources/` directory (executable scripts, desktop entries, package manifests, snap configuration)

**Finding:** The resources directory contains extensive platform-specific executable and configuration patterns for packaging, distribution, and launch across macOS, Windows, and Linux. These patterns establish CLI wrappers, package metadata, system integration hooks, and environment configuration that a Tauri/Rust port would need to adapt.

---

#### Pattern: Cross-Platform CLI Launcher Script Template

**Where:** `resources/darwin/bin/code.sh:1-40` (macOS), `resources/linux/bin/code.sh:1-64` (Linux), `resources/win32/bin/code.cmd:1-8` (Windows)

**What:** Shell/batch scripts that wrap the Electron executable to invoke the CLI bootstrapper (`cli.js`) with ELECTRON_RUN_AS_NODE mode, handling symlink resolution and runtime environment setup differently per platform.

```bash
# macOS pattern (code.sh)
APP_PATH="$(app_realpath "${BASH_SOURCE[0]}")"
CONTENTS="$APP_PATH/Contents"
ELECTRON="$CONTENTS/MacOS/@@NAME@@"
CLI="$CONTENTS/Resources/app/out/cli.js"
export VSCODE_NODE_REPL_EXTERNAL_MODULE=$NODE_REPL_EXTERNAL_MODULE
unset NODE_OPTIONS
ELECTRON_RUN_AS_NODE=1 "$ELECTRON" "$CLI" "$@"

# Linux pattern (code.sh)
if [ "$(id -u)" = "0" ]; then
  for i in "$@"; do
    case "$i" in
      --user-data-dir | --user-data-dir=* | --file-write | tunnel | serve-web )
        CAN_LAUNCH_AS_ROOT=1 ;;
    esac
  done
fi
ELECTRON="$VSCODE_PATH/@@APPNAME@@"
CLI="$VSCODE_PATH/resources/app/out/cli.js"
ELECTRON_RUN_AS_NODE=1 "$ELECTRON" "$CLI" "$@"

# Windows pattern (code.cmd)
@echo off
set ELECTRON_RUN_AS_NODE=1
"%~dp0..\@@NAME@@.exe" "%~dp0..\resources\app\out\cli.js" %*
```

**Variations / call-sites:**
- Linux variant includes root privilege checks and WSL detection (`grep -qi Microsoft /proc/version`)
- macOS uses symlink resolution with `readlink` and app bundle structure detection
- Windows uses batch variable expansion (`%~dp0`, `%*`) for path and argument handling
- All variants suppress/reset Node.js-specific env vars before launching the runtime
- Remote CLI override pattern: checks `VSCODE_IPC_HOOK_CLI` environment variable for remote terminal scenarios

---

#### Pattern: Linux Desktop Entry and URL Handler Registration

**Where:** `resources/linux/code.desktop:1-29`, `resources/linux/code-url-handler.desktop:1-13`

**What:** FreeDesktop desktop entry files (.desktop) with parameterized template placeholders (`@@NAME_LONG@@`, `@@EXEC@@`, `@@ICON@@`) for desktop environment integration, file associations, and context menu actions.

```desktop
[Desktop Entry]
Name=@@NAME_LONG@@
Comment=Code Editing. Redefined.
GenericName=Text Editor
Exec=@@EXEC@@ %F
Icon=@@ICON@@
Type=Application
StartupNotify=false
StartupWMClass=@@NAME_SHORT@@
Categories=TextEditor;Development;IDE;
MimeType=application/x-@@NAME@@-workspace;
Actions=new-empty-window;

[Desktop Action new-empty-window]
Name=New Empty Window
Exec=@@EXEC@@ --new-window %F
Icon=@@ICON@@

# URL Handler variant
[Desktop Entry]
Name=@@NAME_LONG@@ - URL Handler
Exec=@@EXEC@@ --open-url %U
NoDisplay=true
MimeType=x-scheme-handler/@@URLPROTOCOL@@;
```

**Variations / call-sites:**
- Main entry: file associations (`MimeType=application/x-@@NAME@@-workspace`), new window action
- URL handler entry: `NoDisplay=true`, URL scheme handler (`x-scheme-handler/@@URLPROTOCOL@@`), uses `%U` for URI args
- Both use i18n-ready Name fields with language variants

---

#### Pattern: Snap Container Package Configuration (snapcraft.yaml)

**Where:** `resources/linux/snap/snapcraft.yaml:1-86`

**What:** Snapcraft YAML manifest with plugin configuration, stage packages (glibc, graphics libraries), patchelf binary patching, and app integration points for snap containerization.

```yaml
name: @@NAME@@
version: '@@VERSION@@'
grade: stable
confinement: classic
base: core20
compression: lzo

parts:
  code:
    plugin: dump
    source: .
    stage-packages:
      - ca-certificates
      - libasound2
      - libatk-bridge2.0-0
      - libcairo2
      - libglib2.0-0
      - libgtk-3-0
    override-build: |
      snapcraftctl build
      patchelf --force-rpath --set-rpath '$ORIGIN/../../lib/x86_64-linux-gnu:$ORIGIN:/snap/core20/current/lib/x86_64-linux-gnu' $SNAPCRAFT_PART_INSTALL/usr/share/@@NAME@@/chrome_crashpad_handler
      chmod 0755 $SNAPCRAFT_PART_INSTALL/usr/share/@@NAME@@/chrome-sandbox

apps:
  @@NAME@@:
    command: electron-launch $SNAP/usr/share/@@NAME@@/bin/@@NAME@@ --no-sandbox
    common-id: @@NAME@@.desktop
```

**Variations / call-sites:**
- Uses `electron-launch` wrapper (snap-specific launcher script) with `--no-sandbox` flag
- Patchelf rpath manipulation for binary compatibility
- Two app entries: main and url-handler variants
- Stage-packages include Wayland, X11, accessibility, audio libraries

---

#### Pattern: Debian Package Lifecycle Scripts (postinst, postrm, prerm)

**Where:** `resources/linux/debian/postinst.template:1-156`, `resources/linux/debian/postrm.template:1-35`, `resources/linux/debian/prerm.template:1-7`

**What:** Debian package hooks for symlink creation, desktop database updates, apt repository registration, and GPG key installation with debconf interactive prompts.

```bash
# postinst: symlink, alternatives registration, repo setup
rm -f /usr/bin/@@NAME@@
ln -s /usr/share/@@NAME@@/bin/@@NAME@@ /usr/bin/@@NAME@@
update-alternatives --install /usr/bin/editor editor /usr/bin/@@NAME@@ 0

if hash update-desktop-database 2>/dev/null; then
  update-desktop-database
fi
if hash update-mime-database 2>/dev/null; then
  update-mime-database /usr/share/mime
fi

# Repository registration with debconf prompts
db_get @@NAME@@/add-microsoft-repo || true
# DEB822 format source writing
echo "Types: deb
URIs: https://packages.microsoft.com/repos/code
Signed-By: $CODE_TRUSTED_PART" > "$CODE_SOURCE_PART_DEB822"

# postrm: symlink removal, database cleanup, repo removal
rm -f /usr/bin/@@NAME@@
update-desktop-database
update-mime-database /usr/share/mime
rm -f $CODE_SOURCE_PART

# prerm: alternatives deregistration
update-alternatives --remove editor /usr/bin/@@NAME@@
```

**Variations / call-sites:**
- postinst: conditional repository setup (checks for Raspberry Pi OS), DEB822 format preference, embedded PGP key
- Uses debconf module for interactive prompts (`db_input`, `db_go`)
- Handles both classic APT sources and new DEB822 format
- postrm: purge-mode cleanup (db_purge)
- prerm: only removes from alternatives system

---

#### Pattern: Debian Package Metadata (control.template)

**Where:** `resources/linux/debian/control.template:1-19`

**What:** Debian control file with parameterized dependencies, architecture, and package relationships (Provides, Conflicts, Replaces).

```text
Package: @@NAME@@
Version: @@VERSION@@
Section: devel
Depends: @@DEPENDS@@
Recommends: @@RECOMMENDS@@
Priority: optional
Architecture: @@ARCHITECTURE@@
Maintainer: Microsoft Corporation <vscode-linux@microsoft.com>
Homepage: https://code.visualstudio.com/
Installed-Size: @@INSTALLEDSIZE@@
Provides: visual-studio-@@NAME@@
Conflicts: visual-studio-@@NAME@@
Replaces: visual-studio-@@NAME@@
Description: Code editing. Redefined.
```

**Variations / call-sites:**
- Virtual package provision: `Provides/Conflicts/Replaces` all map to `visual-studio-@@NAME@@`
- Dependencies and recommends are parameterized per build variant

---

#### Pattern: Server Launcher Scripts (Node.js-based server)

**Where:** `resources/server/bin/code-server-linux.sh:1-23`, `resources/server/bin/code-server-darwin.sh:1-22`, `resources/server/bin/code-server.cmd:1-25`

**What:** Entrypoint scripts for headless code-server that resolve runtime paths and optionally patch glibc via patchelf, delegating to node with server-main.js bootstrap.

```bash
# Linux: custom glibc patching for compatibility
case "$1" in
  --inspect*) INSPECT="$1"; shift;;
esac

if [ -n "$VSCODE_SERVER_CUSTOM_GLIBC_LINKER" ] && [ -n "$VSCODE_SERVER_CUSTOM_GLIBC_PATH" ]; then
  "$VSCODE_SERVER_PATCHELF_PATH" --set-rpath "$VSCODE_SERVER_CUSTOM_GLIBC_PATH" "$ROOT/node"
  "$VSCODE_SERVER_PATCHELF_PATH" --set-interpreter "$VSCODE_SERVER_CUSTOM_GLIBC_LINKER" "$ROOT/node"
fi

"$ROOT/node" ${INSPECT:-} "$ROOT/out/server-main.js" "$@"

# Windows: batch variant
set ROOT_DIR=%~dp0..
if "%_FIRST_ARG:~0,9%"=="--inspect" (set INSPECT=%1; shift)
"%ROOT_DIR%\node.exe" %INSPECT% "%ROOT_DIR%\out\server-main.js" %RESTVAR%
```

**Variations / call-sites:**
- Linux script includes patchelf customization (glibc 2.28+ compatibility via environment variables)
- Darwin variant uses realpath function for symlink resolution (similar to CLI launchers)
- Windows version parses `--inspect` flag and accumulates remaining arguments in loop

---

#### Pattern: Windows App Package Manifest (AppxManifest.xml)

**Where:** `resources/win32/appx/AppxManifest.xml:1-90`

**What:** UWP app manifest with capabilities (runFullTrust, unvirtualizedResources), file explorer context menu COM integration, and multi-language resource declarations for Windows Store deployment.

```xml
<?xml version="1.0" encoding="utf-8"?>
<Package xmlns="http://schemas.microsoft.com/appx/manifest/foundation/windows10" ...>
  <Identity Name="@@AppxPackageName@@"
    Publisher="CN=Microsoft Corporation, O=Microsoft Corporation, L=Redmond, S=Washington, C=US"
    Version="@@AppxPackageVersion@@" />
  <Properties>
    <DisplayName>@@AppxPackageDisplayName@@</DisplayName>
    <Logo>resources\app\resources\win32\code_150x150.png</Logo>
    <desktop6:RegistryWriteVirtualization>disabled</desktop6:RegistryWriteVirtualization>
    <desktop6:FileSystemWriteVirtualization>disabled</desktop6:FileSystemWriteVirtualization>
  </Properties>
  <Resources>
    <Resource Language="en-us" />
    <Resource Language="es-es" />
    <Resource Language="zh-cn" />
  </Resources>
  <Dependencies>
    <TargetDeviceFamily Name="Windows.Desktop" MinVersion="10.0.19041.0" MaxVersionTested="10.0.26100.0" />
  </Dependencies>
  <Capabilities>
    <rescap:Capability Name="runFullTrust" />
    <rescap:Capability Name="unvirtualizedResources"/>
    <DeviceCapability Name="microphone"/>
  </Capabilities>
  <Applications>
    <Application Id="@@ApplicationIdShort@@" Executable="@@ApplicationExe@@">
      <Extensions>
        <desktop4:Extension Category="windows.fileExplorerContextMenus">
          <desktop4:FileExplorerContextMenus>
            <desktop5:ItemType Type="Directory">
              <desktop5:Verb Id="@@FileExplorerContextMenuID@@" Clsid="@@FileExplorerContextMenuCLSID@@" />
            </desktop5:ItemType>
```

**Variations / call-sites:**
- Win32 trust level: `uap10:TrustLevel="mediumIL"`, `RuntimeBehavior="win32App"`
- Virtualization disabled for registry and filesystem (requires runFullTrust)
- File Explorer context menu via COM surrogate with CLSID registration
- Language resources declared for Windows Store localization

---

#### Pattern: Snap Environment Launcher (electron-launch wrapper)

**Where:** `resources/linux/snap/electron-launch:1-279`

**What:** Comprehensive snap environment setup script that manages XDG paths, GTK/GIO module compilation, Wayland detection, font/mime caching, and ibus/fcitx integration before delegating to application.

```bash
function ensure_dir_exists() {
  [ -d "$1" ] || mkdir -p "$@"
}

# Architecture triplet detection for snap
if [ "$SNAP_ARCH" == "amd64" ]; then
  ARCH="x86_64-linux-gnu"
elif [ "$SNAP_ARCH" == "arm64" ]; then
  ARCH="aarch64-linux-gnu"
fi

# XDG Config setup
prepend_dir XDG_CONFIG_DIRS "$SNAP/etc/xdg"
prepend_dir XDG_DATA_DIRS "$SNAP/usr/share"
export XDG_DATA_HOME="$SNAP_USER_DATA/.local/share"

# Version detection for incremental updates
source "$SNAP_USER_DATA/.last_revision" 2>/dev/null || true
if [ "$SNAP_DESKTOP_LAST_REVISION" = "$SNAP_VERSION" ]; then
  needs_update=false
fi

# GTK/GIO module compilation on update
if [ "$needs_update" = true ]; then
  compile_giomodules "/snap/core20/current/usr/lib/$ARCH"
  compile_schemas "/snap/core20/current/usr/lib/$ARCH/glib-2.0/glib-compile-schemas"
fi

# Wayland detection and compat symlink
if [ -S "$wayland_sockpath" ]; then
  ln -s "$wayland_sockpath" "$wayland_snappath"
fi

exec "$@" --ozone-platform=x11
```

**Variations / call-sites:**
- Per-arch lib paths and GDK/GTK module directories
- Async execution pattern for schema compilation (background tasks)
- Conditional Wayland/X11 support with fallback
- Migration of legacy `.cache` from per-user to common storage
- GDK pixbuf loaders and GTK input module compilation

---

#### Pattern: Server Manifest (Web App Metadata)

**Where:** `resources/server/manifest.json:1-21`

**What:** PWA web app manifest for code-server with icon declarations, standalone display mode, and window controls overlay support.

```json
{
  "name": "Code - OSS",
  "short_name": "Code- OSS",
  "start_url": "/",
  "lang": "en-US",
  "display": "standalone",
  "display_override": ["window-controls-overlay"],
  "icons": [
    {
      "src": "code-192.png",
      "type": "image/png",
      "sizes": "192x192"
    },
    {
      "src": "code-512.png",
      "type": "image/png",
      "sizes": "512x512"
    }
  ]
}
```

**Variations / call-sites:**
- Standalone display for borderless window experience
- Window controls overlay for custom frame
- Icon sizes: 192px and 512px (standard PWA sizes)

---

#### Pattern: Linux System Requirements Validation Script

**Where:** `resources/server/bin/helpers/check-requirements-linux.sh:1-162`

**What:** Proactive glibc/libstdc++ version checking with ldconfig integration, distro detection (Alpine, NixOS), and architecture-specific library path resolution for server compatibility.

```bash
# Exit codes: 0 = OK, 99 = Unsupported OS
MIN_GLIBCXX_VERSION="3.4.25"

# Architecture detection for lib path selection
case $ARCH in
  x86_64) LDCONFIG_ARCH="x86-64";;
  aarch64) LDCONFIG_ARCH="AArch64";;
esac

# GLIBC version extraction from binary
if [ -f /sbin/ldconfig ]; then
  libstdcpp_paths=$(/sbin/ldconfig -p | grep 'libstdc++.so.6')
  libstdcpp_version=$(grep -ao 'GLIBCXX_[0-9]*\.[0-9]*\.[0-9]*' "$libstdcpp_real_path" | sort -V | tail -1)
fi

# Distro-specific overrides (Alpine, NixOS skip checks)
if [ "$OS_ID" = "nixos" ]; then
  echo "Warning: NixOS detected, skipping GLIBC check"
  exit 0
fi

# MUSL detection for Alpine
if [ "$OS_ID" = "alpine" ]; then
  for rtld in /lib/ld-musl-aarch64.so.1 /lib/ld-musl-x86_64.so.1; do
    if [ -x $rtld ]; then
      musl_version=$("$rtld" --version 2>&1 | grep "Version" | awk '{print $NF}')
    fi
  done
fi

if [ "$found_required_glibc" = "0" ] || [ "$found_required_glibcxx" = "0" ]; then
  exit 99  # Unsupported OS
fi
```

**Variations / call-sites:**
- Skip mechanism via `/tmp/vscode-skip-server-requirements-check` file
- Custom glibc linker override via `VSCODE_SERVER_CUSTOM_GLIBC_LINKER` environment variable
- Distro detection via `/etc/os-release` (nixos ID)
- MUSL detection for Alpine, GLIBC 2.28+ requirement for others

---

#### Pattern: Mimetype and AppData Metadata (Linux)

**Where:** `resources/linux/code-workspace.xml:1-8`, `resources/linux/code.appdata.xml:1-19`

**What:** FreeDesktop MIME type definition and AppData metadata for Linux app repositories.

```xml
<!-- MIME type for workspace files -->
<?xml version="1.0" encoding="UTF-8"?>
<mime-info xmlns="http://www.freedesktop.org/standards/shared-mime-info">
  <mime-type type="application/x-@@NAME@@-workspace">
    <comment>@@NAME_LONG@@ Workspace</comment>
    <glob pattern="*.code-workspace"/>
  </mime-type>
</mime-info>

<!-- AppData metadata for Linux app stores -->
<?xml version="1.0" encoding="UTF-8"?>
<component type="desktop">
  <id>@@NAME@@.desktop</id>
  <metadata_license>@@LICENSE@@</metadata_license>
  <project_license>@@LICENSE@@</project_license>
  <name>@@NAME_LONG@@</name>
  <url type="homepage">https://code.visualstudio.com</url>
  <summary>Visual Studio Code. Code editing. Redefined.</summary>
  <description>
    <p>Visual Studio Code is a new choice of tool...</p>
  </description>
  <screenshots>
    <screenshot type="default">
      <image>https://code.visualstudio.com/home/home-screenshot-linux-lg.png</image>
    </screenshot>
  </screenshots>
</component>
```

**Variations / call-sites:**
- MIME type: glob pattern matching for workspace files
- AppData: project license, homepage URL, description, screenshot URLs
- Parameterized for localization and branding

---

### Summary

The `resources/` directory contains **production-ready platform distribution patterns** including:

1. **CLI Launchers**: Cross-platform shell/batch wrappers with platform-specific symlink resolution, privilege checks, and environment variable management
2. **Desktop Integration**: FreeDesktop .desktop entries with MIME type association and context menu actions
3. **Package Metadata**: Debian control templates, AppData XML, and Windows AppX manifests for system packaging
4. **Package Hooks**: Debian postinst/postrm/prerm scripts managing symlinks, database updates, APT repository registration, and GPG keys
5. **Container Config**: Snapcraft YAML with patchelf binary patching, GTK/GIO module compilation, and environment isolation
6. **Snap Launcher**: Comprehensive environment setup with XDG path management, Wayland/X11 detection, and version-triggered schema compilation
7. **Server Launchers**: Node.js-based bootstraps for headless code-server with optional glibc patching
8. **Requirements Validation**: Pre-flight glibc/libstdc++ version checks with distro-specific overrides for Linux server deployment
9. **Web Manifests**: PWA metadata for web-based server variant

A Tauri/Rust port would need to **translate** these patterns into Rust-native equivalents: Tauri's built-in installer/updater system, custom IPC protocols replacing Node.js/Electron, platform-specific build configuration, and native system integration (desktop entries, package scripts) as build-time templates rather than runtime wrappers. The parameterized template approach (@@NAME@@, @@VERSION@@, etc.) is consistent across all package configurations.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
