# Partition 40 of 79 — Findings

## Scope
`resources/` (20 files, 600 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
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

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
_(no analysis produced)_

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Partition 40: Resource Configuration & Host Integration Patterns

## Overview

The `resources/` directory contains non-code artifacts essential for packaging, platform integration, and CLI bridging across desktop environments (Windows, macOS, Linux) and server deployments. While primarily static assets (icons, images), it includes critical **host-integration** patterns that a Tauri port would need to replicate, particularly for:

1. **Platform-specific bootstrapping and entry points**
2. **Package manifest declarations** (Windows AppX, Linux Snap, Debian)
3. **System integration** (desktop shortcuts, CLI wrappers, environment setup)
4. **Server-mode operation** (glibc versioning, node runtime bridging)

---

## Patterns

#### Pattern: Unix Shell Bootstrapper with Environment Detection

**Where:** `resources/linux/bin/code.sh:1-64`, `resources/darwin/bin/code.sh:1-40`

**What:** Unix shells scripts that detect runtime environment, locate the application binary, and bootstrap the CLI via Node.js/Electron with proper environment variables and path resolution.

```bash
#!/usr/bin/env sh

# Remote terminal detection (for server scenarios)
if [ -n "$VSCODE_IPC_HOOK_CLI" ]; then
    REMOTE_CLI="$(which -a '@@APPNAME@@' | grep /remote-cli/)"
    if [ -n "$REMOTE_CLI" ]; then
        "$REMOTE_CLI" "$@"
        exit $?
    fi
fi

# Security check: prevent running as root without explicit flags
if [ "$(id -u)" = "0" ]; then
    for i in "$@"; do
        case "$i" in
            --user-data-dir | --user-data-dir=* | --file-write | tunnel | serve-web )
                CAN_LAUNCH_AS_ROOT=1
            ;;
        esac
    done
    if [ -z $CAN_LAUNCH_AS_ROOT ]; then
        echo "You are trying to start @@PRODNAME@@ as a super user..." 1>&2
        exit 1
    fi
fi

# Path resolution: handle symlinks and fallback to standard locations
if [ ! -L "$0" ]; then
    VSCODE_PATH="$(dirname "$0")/.."
else
    if command -v readlink >/dev/null; then
        VSCODE_PATH="$(dirname "$(readlink -f "$0")")/.."
    else
        VSCODE_PATH="/usr/share/@@APPNAME@@"
    fi
fi

ELECTRON="$VSCODE_PATH/@@APPNAME@@"
CLI="$VSCODE_PATH/resources/app/out/cli.js"
ELECTRON_RUN_AS_NODE=1 "$ELECTRON" "$CLI" "$@"
exit $?
```

**Key aspects:**
- Detects remote CLI context via `VSCODE_IPC_HOOK_CLI` environment variable
- Enforces privilege escalation safety (rejects root unless `--user-data-dir`, `--file-write`, `tunnel`, or `serve-web` provided)
- Handles symlink resolution with fallback to system install paths
- Uses `ELECTRON_RUN_AS_NODE=1` to run Node CLI instead of Electron GUI
- Passes all arguments through to the CLI entry point

---

#### Pattern: macOS .app Bundle Resolver

**Where:** `resources/darwin/bin/code.sh:15-39`

**What:** macOS-specific path resolution that unwraps `.app` bundle structure using a custom `app_realpath` function.

```bash
function app_realpath() {
    SOURCE=$1
    while [ -h "$SOURCE" ]; do
        DIR=$(dirname "$SOURCE")
        SOURCE=$(readlink "$SOURCE")
        [[ $SOURCE != /* ]] && SOURCE=$DIR/$SOURCE
    done
    SOURCE_DIR="$( cd -P "$( dirname "$SOURCE" )" >/dev/null 2>&1 && pwd )"
    echo "${SOURCE_DIR%%${SOURCE_DIR#*.app}}"
}

APP_PATH="$(app_realpath "${BASH_SOURCE[0]}")"
if [ -z "$APP_PATH" ]; then
    echo "Unable to determine app path from symlink : ${BASH_SOURCE[0]}"
    exit 1
fi
CONTENTS="$APP_PATH/Contents"
ELECTRON="$CONTENTS/MacOS/@@NAME@@"
CLI="$CONTENTS/Resources/app/out/cli.js"
```

**Key aspects:**
- Recursive symlink following with relative path handling
- `.app` bundle structure awareness (`Contents/MacOS/`, `Contents/Resources/`)
- Preserves Node environment variables via `VSCODE_NODE_OPTIONS` and `VSCODE_NODE_REPL_EXTERNAL_MODULE`
- Error handling for malformed bundle paths

---

#### Pattern: Windows Batch Entry Point (Desktop)

**Where:** `resources/win32/bin/code.cmd:1-8`

**What:** Minimal Windows batch wrapper that sets up Node.js execution environment and delegates to CLI.

```batch
@echo off
setlocal
set VSCODE_DEV=
set ELECTRON_RUN_AS_NODE=1
"%~dp0..\@@NAME@@.exe" "%~dp0..\resources\app\out\cli.js" %*
IF %ERRORLEVEL% NEQ 0 EXIT /b %ERRORLEVEL%
endlocal
```

**Key aspects:**
- Uses `%~dp0` (batch relative path) for platform-independent directory resolution
- Clears `VSCODE_DEV` flag to distinguish from dev mode
- Propagates exit codes with `IF %ERRORLEVEL%`
- Minimal overhead (no feature detection, as Windows is single-path)

---

#### Pattern: Windows Server Bootstrap with Argument Parsing

**Where:** `resources/server/bin/code-server.cmd:1-25`

**What:** Server-mode Windows entry point with `--inspect` flag extraction and argument forwarding.

```batch
@echo off
setlocal

set ROOT_DIR=%~dp0..

set _FIRST_ARG=%1
if "%_FIRST_ARG:~0,9%"=="--inspect" (
    set INSPECT=%1
    shift
) else (
    set INSPECT=
)

:loop1
if "%~1"=="" goto after_loop
set RESTVAR=%RESTVAR% %1
shift
goto loop1

:after_loop

"%ROOT_DIR%\node.exe" %INSPECT% "%ROOT_DIR%\out\server-main.js" %RESTVAR%

endlocal
```

**Key aspects:**
- Parses `--inspect` flag before other arguments
- Uses labeled loops for remaining argument collection (batch lacks arrays)
- Separates inspection tooling from application bootstrap
- Delegates to `server-main.js` instead of `cli.js`

---

#### Pattern: WSL/Cygwin Path Translation in Entry Point

**Where:** `resources/win32/bin/code.sh:1-63`

**What:** Windows .sh wrapper that bridges WSL/Cygwin environments, detects WSL builds, and converts paths appropriately.

```bash
#!/usr/bin/env sh
COMMIT="@@COMMIT@@"
APP_NAME="@@APPNAME@@"
QUALITY="@@QUALITY@@"
NAME="@@NAME@@"
SERVERDATAFOLDER="@@SERVERDATAFOLDER@@"
VSCODE_PATH="$(dirname "$(dirname "$(realpath "$0")")")"
ELECTRON="$VSCODE_PATH/$NAME.exe"

IN_WSL=false
if [ -n "$WSL_DISTRO_NAME" ]; then
    IN_WSL=true
else
    WSL_BUILD=$(uname -r | sed -E 's/^[0-9.]+-([0-9]+)-Microsoft.*|.*/\1/')
    if [ -n "$WSL_BUILD" ]; then
        if [ "$WSL_BUILD" -ge 17063 ]; then
            IN_WSL=true
        else
            "$ELECTRON" "$@"
            exit $?
        fi
    fi
fi

if [ $IN_WSL = true ]; then
    export WSLENV="ELECTRON_RUN_AS_NODE/w:$WSLENV"
    CLI=$(wslpath -m "$VSCODE_PATH/resources/app/out/cli.js")
    WSL_EXT_ID="ms-vscode-remote.remote-wsl"
    ELECTRON_RUN_AS_NODE=1 "$ELECTRON" "$CLI" --locate-extension $WSL_EXT_ID >/tmp/remote-wsl-loc.txt 2>/dev/null
    WSL_EXT_WLOC=$(cat /tmp/remote-wsl-loc.txt)
    if [ -n "$WSL_EXT_WLOC" ]; then
        WSL_CODE=$(wslpath -u "${WSL_EXT_WLOC%%[[:cntrl:]]}")/scripts/wslCode.sh
        "$WSL_CODE" "$COMMIT" "$QUALITY" "$ELECTRON" "$APP_NAME" "$SERVERDATAFOLDER" "$@"
        exit $?
    fi
elif [ -x "$(command -v cygpath)" ]; then
    CLI=$(cygpath -m "$VSCODE_PATH/resources/app/out/cli.js")
else
    CLI="$VSCODE_PATH/resources/app/out/cli.js"
fi
ELECTRON_RUN_AS_NODE=1 "$ELECTRON" "$CLI" "$@"
```

**Key aspects:**
- Detects WSL via `WSL_DISTRO_NAME` or `uname -r` parsing
- Checks WSL build number (≥17063 required for `WSLENV` support)
- Uses `wslpath -m` (Windows path) and `wslpath -u` (Unix path) for conversions
- Falls back to Cygpath if WSL unavailable
- Attempts to locate and delegate to Remote WSL extension if present

---

#### Pattern: Linux System Requirements Verification

**Where:** `resources/server/bin/helpers/check-requirements-linux.sh:1-162`

**What:** Comprehensive glibc and libstdc++ version checking for server mode, with distro-specific handling (Alpine, NixOS, musl).

```bash
#!/usr/bin/env sh
set -e

# Skip check if flagged or custom glibc provided
if [ -f "/tmp/vscode-skip-server-requirements-check" ] || [ -n "$VSCODE_SERVER_CUSTOM_GLIBC_LINKER" ]; then
    echo "!!! WARNING: Skipping server pre-requisite check !!!"
    exit 0
fi

ARCH=$(uname -m)
MIN_GLIBCXX_VERSION="3.4.25"

# Detect distro via /etc/os-release
if [ -f /etc/os-release ]; then
    OS_ID="$(cat /etc/os-release | grep -Eo 'ID=([^"]+)' | sed -n '1s/ID=//p')"
    if [ "$OS_ID" = "nixos" ]; then
        echo "Warning: NixOS detected, skipping GLIBC check"
        exit 0
    fi
fi

# Map architecture to ldconfig pattern
case $ARCH in
    x86_64) LDCONFIG_ARCH="x86-64";;
    armv7l | armv8l)
        MIN_GLIBCXX_VERSION="3.4.26"
        LDCONFIG_ARCH="hard-float"
        ;;
    arm64 | aarch64)
        LDCONFIG_ARCH="AArch64"
        ;;
esac

# Query installed libstdc++ version from ldconfig
if [ "$OS_ID" != "alpine" ]; then
    libstdcpp_paths=$(/sbin/ldconfig -p | grep 'libstdc++.so.6')
    libstdcpp_version=$(grep -ao 'GLIBCXX_[0-9]*\.[0-9]*\.[0-9]*' "$libstdcpp_real_path" | sort -V | tail -1)
fi

# Similar checks for glibc (libc.so.6) and musl on Alpine
if [ "$OS_ID" = "alpine" ]; then
    MUSL_RTLDLIST="/lib/ld-musl-aarch64.so.1 /lib/ld-musl-x86_64.so.1"
    for rtld in ${MUSL_RTLDLIST}; do
        if [ -x $rtld ]; then
            musl_version=$("$rtld" --version 2>&1 | grep "Version" | awk '{print $NF}')
        fi
    done
fi

# Final verdict: exit 99 if requirements not met
if [ "$found_required_glibc" = "0" ] || [ "$found_required_glibcxx" = "0" ]; then
    echo "Error: Missing required dependencies."
    exit 99
fi
```

**Key aspects:**
- Exit code 99 signals unsupported OS (distinct from general errors)
- Architecture-aware ldconfig queries (x86-64, AArch64, ARM hard-float)
- Distro detection: skip checks for Alpine (musl) and NixOS
- Version comparison using `sort -V` (semantic versioning)
- Custom glibc override via `VSCODE_SERVER_CUSTOM_GLIBC_PATH` and `patchelf`

---

#### Pattern: Linux Desktop Integration (`.desktop` file)

**Where:** `resources/linux/code.desktop:1-29`

**What:** Freedesktop.org format for desktop launcher, MIME type registration, and context menu actions.

```ini
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
Keywords=vscode;

[Desktop Action new-empty-window]
Name=New Empty Window
Name[cs]=Nové prázdné okno
Name[de]=Neues leeres Fenster
Name[es]=Nueva ventana vacía
Name[fr]=Nouvelle fenêtre vide
Name[it]=Nuova finestra vuota
Name[ja]=新しい空のウィンドウ
Name[ko]=새 빈 창
Name[ru]=Новое пустое окно
Name[zh_CN]=新建空窗口
Name[zh_TW]=開新空視窗
Exec=@@EXEC@@ --new-window %F
Icon=@@ICON@@
```

**Key aspects:**
- Localized menu labels (cs, de, es, fr, it, ja, ko, ru, zh_CN, zh_TW)
- MIME type registration for workspace files
- `StartupWMClass` for X11 window manager hints
- Desktop actions for context menu integration
- Parameterized via template substitution (`@@EXEC@@`, `@@ICON@@`)

---

#### Pattern: Snap Confinement & Runtime Dependencies

**Where:** `resources/linux/snap/snapcraft.yaml:1-86`

**What:** Snapcraft manifest declaring dependencies, build settings, confinement level, and runtime entry points.

```yaml
name: @@NAME@@
version: '@@VERSION@@'
summary: Code editing. Redefined.
description: Visual Studio Code combined with edit-build-debug...
architectures:
  - build-on: amd64
    run-on: @@ARCHITECTURE@@
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
      - libatk1.0-0
      - libatspi2.0-0
      - libcairo2
      - libcanberra-gtk3-module
      - libcurl3-gnutls
      - libcurl3-nss
      - libcurl4
      - libegl1
      - libdrm2
      - libgbm1
      - libgl1
      - libgles2
      - libglib2.0-0
      - libgtk-3-0
      - libibus-1.0-5
      - libnss3
      - libpango-1.0-0
      - libsecret-1-0
      - libwayland-egl1
      - libxcomposite1
      - libxdamage1
      - libxfixes3
      - libxkbcommon0
      - libxkbfile1
      - libxrandr2
      - libxss1
      - locales-all
      - packagekit-gtk3-module
      - xdg-utils
    prime:
      - -usr/share/doc
      - -usr/share/fonts
      - -usr/share/icons
      - -usr/share/lintian
      - -usr/share/man
    override-build: |
      snapcraftctl build
      patchelf --force-rpath --set-rpath '$ORIGIN/../../lib/x86_64-linux-gnu:$ORIGIN:/snap/core20/current/lib/x86_64-linux-gnu' $SNAPCRAFT_PART_INSTALL/usr/share/@@NAME@@/chrome_crashpad_handler
      chmod 0755 $SNAPCRAFT_PART_INSTALL/usr/share/@@NAME@@/chrome-sandbox
  cleanup:
    after: [code]
    plugin: nil
    override-prime: |
      set -eux
      for snap in "core20"; do
        cd "/snap/$snap/current" && find . -type f,l -exec rm -f "$SNAPCRAFT_PRIME/{}" \;
      done

apps:
  @@NAME@@:
    command: electron-launch $SNAP/usr/share/@@NAME@@/bin/@@NAME@@ --no-sandbox
    common-id: @@NAME@@.desktop
  url-handler:
    command: electron-launch $SNAP/usr/share/@@NAME@@/bin/@@NAME@@ --open-url --no-sandbox
```

**Key aspects:**
- `confinement: classic` allows full system access (required for IDE functionality)
- `base: core20` uses Ubuntu 20.04 LTS runtime
- Stage packages list all direct GTK/audio/GL dependencies for Electron/Chromium
- Prime filters drop docs/fonts/icons to reduce snap size
- `patchelf` rewrites rpath for vendored libraries in sandbox
- `chrome-sandbox` requires explicit execute permission in snap

---

#### Pattern: Windows AppX Manifest with Capabilities

**Where:** `resources/win32/appx/AppxManifest.xml:1-90`

**What:** UWP/AppX manifest declaring application identity, capabilities, file associations, and shell extensions for Windows Store distribution.

```xml
<?xml version="1.0" encoding="utf-8"?>
<Package xmlns:desktop="http://schemas.microsoft.com/appx/manifest/desktop/windows10"
         xmlns:desktop4="http://schemas.microsoft.com/appx/manifest/desktop/windows10/4"
         xmlns:desktop5="http://schemas.microsoft.com/appx/manifest/desktop/windows10/5"
         xmlns:rescap="http://schemas.microsoft.com/appx/manifest/foundation/windows10/restrictedcapabilities"
         xmlns:com="http://schemas.microsoft.com/appx/manifest/com/windows10">
  <Identity Name="@@AppxPackageName@@"
            Publisher="CN=Microsoft Corporation, O=Microsoft Corporation, L=Redmond, S=Washington, C=US"
            Version="@@AppxPackageVersion@@"
            ProcessorArchitecture="neutral" />
  <Properties>
    <DisplayName>@@AppxPackageDisplayName@@</DisplayName>
    <PublisherDisplayName>Microsoft Corporation</PublisherDisplayName>
    <Logo>resources\app\resources\win32\code_150x150.png</Logo>
    <uap10:AllowExternalContent>true</uap10:AllowExternalContent>
    <desktop6:RegistryWriteVirtualization>disabled</desktop6:RegistryWriteVirtualization>
    <desktop6:FileSystemWriteVirtualization>disabled</desktop6:FileSystemWriteVirtualization>
  </Properties>
  <Resources>
    <Resource Language="en-us" />
    <Resource Language="es-es" />
    <Resource Language="de-de" />
    <Resource Language="fr-fr" />
    <Resource Language="hu-hu" />
    <Resource Language="it-it" />
    <Resource Language="ja-jp" />
    <Resource Language="ko-kr" />
    <Resource Language="pt-br" />
    <Resource Language="ru-ru" />
    <Resource Language="tr-tr" />
    <Resource Language="zh-cn" />
    <Resource Language="zh-tw" />
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
    <Application Id="@@ApplicationIdShort@@"
      Executable="@@ApplicationExe@@"
      uap10:TrustLevel="mediumIL"
      uap10:RuntimeBehavior="win32App">
      <uap:VisualElements AppListEntry="none" ... />
      <Extensions>
        <desktop4:Extension Category="windows.fileExplorerContextMenus">
          <desktop4:FileExplorerContextMenus>
            <desktop5:ItemType Type="Directory">
              <desktop5:Verb Id="@@FileExplorerContextMenuID@@" Clsid="@@FileExplorerContextMenuCLSID@@" />
            </desktop5:ItemType>
          </desktop4:FileExplorerContextMenus>
        </desktop4:Extension>
        <com:Extension Category="windows.comServer">
          <com:ComServer>
            <com:SurrogateServer DisplayName="@@AppxPackageDisplayName@@">
              <com:Class Id="@@FileExplorerContextMenuCLSID@@" Path="@@FileExplorerContextMenuDLL@@" />
            </com:SurrogateServer>
          </com:ComServer>
        </com:Extension>
      </Extensions>
    </Application>
  </Applications>
</Package>
```

**Key aspects:**
- `ProcessorArchitecture="neutral"` allows deployment across CPU architectures
- `runFullTrust` capability required for IDE file/process access
- `unvirtualizedResources` disables AppX registry/filesystem isolation
- `TrustLevel="mediumIL"` allows medium integrity level (no admin required)
- 13 language resources (en-us, es, de, fr, hu, it, ja, ko, pt-br, ru, tr, zh-cn, zh-tw)
- File Explorer context menu integration via COM surrogate
- Explicit microphone capability for voice/audio features

---

#### Pattern: Linux Debian Package Control File

**Where:** `resources/linux/debian/control.template:1-19`

**What:** Debian package metadata declaring dependencies, maintainer, and package relationships.

```
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
 Visual Studio Code is a new choice of tool that combines the simplicity of
 a code editor with what developers need for the core edit-build-debug cycle.
 See https://code.visualstudio.com/docs/setup/linux for installation
 instructions and FAQ.
```

**Key aspects:**
- `Provides:`, `Conflicts:`, `Replaces:` enable parallel installation management
- `Installed-Size` in KiB (used for disk space warnings)
- Separate `Depends` and `Recommends` for required vs. optional packages
- Architecture-specific builds (amd64, arm64, armhf, etc.)
- Microsoft Corporation as maintainer with contact email

---

#### Pattern: PWA Web Manifest for Server Mode

**Where:** `resources/server/manifest.json:1-20`

**What:** Web App Manifest for server-mode web UI, declaring PWA properties and app icons.

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

**Key aspects:**
- `display: "standalone"` hides browser chrome (full-screen mode)
- `display_override: ["window-controls-overlay"]` uses web platform window controls on desktop
- 192x192 and 512x512 icons for mobile and desktop
- `start_url: "/"` for home navigation after installation

---

#### Pattern: Bash Shell Completion Definition

**Where:** `resources/completions/bash/code:1-50`

**What:** Bash completion script for the `code` command, supporting file/flag completion and context-sensitive suggestions.

```bash
_@@APPNAME@@()
{
    local cur prev words cword split
    _init_completion -s || return
    _expand || return

    case $prev in
        -d|--diff)
            _filedir
            return
            ;;
        -a|--add|--user-data-dir|--extensions-dir)
            _filedir -d
            return
            ;;
        -g|--goto)
            compopt -o nospace
            _filedir
            return
            ;;
        --locale)
            COMPREPLY=( $( compgen -W 'de en en-US es fr it ja ko ru zh-CN zh-TW bg hu pt-br tr' ) )
            return
            ;;
        --install-extension|--uninstall-extension)
            _filedir vsix
            return
            ;;
        --log)
            COMPREPLY=( $( compgen -W 'critical error warn info debug trace off' ) )
            return
            ;;
    esac

    $split && return

    if [[ $cur == -* ]]; then
        COMPREPLY=( $( compgen -W '-d --diff --folder-uri -a --add -g
            --goto -n --new-window -r --reuse-window -w --wait --locale=
            --user-data-dir -v --version -h --help --extensions-dir
            --list-extensions --show-versions --install-extension
            --uninstall-extension --enable-proposed-api --verbose --log...' ) )
    fi
}
```

**Key aspects:**
- Context-aware completions: directories for `--user-data-dir`, `.vsix` files for extensions
- Locale hardcoding (de, en, en-US, es, fr, it, ja, ko, ru, zh-CN, zh-TW, bg, hu, pt-br, tr)
- Log level enums (critical, error, warn, info, debug, trace, off)
- Uses bash-completion builtins (`_filedir`, `compgen`, `compopt`)

---

## Non-Pattern Assets

The remaining ~600 files in `resources/` are binary assets:
- **Icons** (`.ico`, `.png`, `.xpm`): 50+ file type icons + app branding
- **Installer bitmaps** (`.bmp`): Inno Setup dialog backgrounds
- **Apple bundle resources** (`.icns`): macOS app icon

These have no code patterns and do not require porting analysis.

## Key Takeaways for Tauri Port

A Tauri-based port would need to replace or adapt:

1. **CLI Bootstrapping**: Tauri's Rust CLI entry point must replicate environment detection, symlink resolution, and path normalization (especially for WSL, Cygwin).

2. **Package Manifests**: Rewrite Snap/Debian/AppX templates for Tauri-based packaging (e.g., AppX manifest with Tauri.exe instead of Electron).

3. **System Integration**: Desktop entries, MIME types, and shell context menus require Tauri-native equivalents (possibly via plugins).

4. **Server Requirements**: The glibc/libstdc++ checks and custom glibc patching would need a Rust-based replacement to verify runtime compatibility.

5. **Platform Entry Points**: The batch/shell wrappers encapsulate critical platform-specific setup (WSL path translation, root check, remote CLI detection) that must map to Tauri's initialization phases.

6. **Shell Completions**: These remain valid as-is if the CLI interface and flags remain stable.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
