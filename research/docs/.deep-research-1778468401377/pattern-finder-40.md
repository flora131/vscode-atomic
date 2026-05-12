# Packaging Artifact Patterns in VS Code Resources

**Scope:** `resources/` (115 files; ~600 LOC in non-binary artifacts)

A Tauri bundler porting VS Code would need to reproduce these recurring platform-specific packaging artifact patterns. The following patterns define how the Electron app is currently distributed across Windows, Linux (Debian/RPM/Snap), and macOS.

---

## Pattern 1: Windows AppxManifest Configuration

**Where:** `resources/win32/appx/AppxManifest.xml:1-89`

**What:** Declarative Microsoft Store app packaging manifest that registers capabilities, file handlers, context menu extensions, and localization.

```xml
<?xml version="1.0" encoding="utf-8"?>
<Package
  xmlns="http://schemas.microsoft.com/appx/manifest/foundation/windows10"
  xmlns:uap="http://schemas.microsoft.com/appx/manifest/uap/windows10"
  xmlns:desktop="http://schemas.microsoft.com/appx/manifest/desktop/windows10"
  xmlns:desktop4="http://schemas.microsoft.com/appx/manifest/desktop/windows10/4"
  IgnorableNamespaces="uap uap2 uap3 rescap desktop desktop4 desktop5 desktop6 desktop10 uap10 com">
  <Identity
    Name="@@AppxPackageName@@"
    Publisher="CN=Microsoft Corporation, O=Microsoft Corporation, L=Redmond, S=Washington, C=US"
    Version="@@AppxPackageVersion@@"
    ProcessorArchitecture="neutral" />
  <Properties>
    <DisplayName>@@AppxPackageDisplayName@@</DisplayName>
    <Logo>resources\app\resources\win32\code_150x150.png</Logo>
    <desktop6:RegistryWriteVirtualization>disabled</desktop6:RegistryWriteVirtualization>
    <desktop6:FileSystemWriteVirtualization>disabled</desktop6:FileSystemWriteVirtualization>
  </Properties>
  <Resources>
    <Resource Language="en-us" />
    <Resource Language="es-es" />
    <Resource Language="fr-fr" />
    <Resource Language="ja-jp" />
  </Resources>
  <Capabilities>
    <rescap:Capability Name="runFullTrust" />
    <DeviceCapability Name="microphone"/>
  </Capabilities>
  <Applications>
    <Application Id="@@ApplicationIdShort@@" Executable="@@ApplicationExe@@">
      <desktop4:Extension Category="windows.fileExplorerContextMenus">
        <desktop5:ItemType Type="Directory">
          <desktop5:Verb Id="@@FileExplorerContextMenuID@@" />
        </desktop5:ItemType>
      </desktop4:Extension>
    </Application>
  </Applications>
</Package>
```

**Variations / siblings:**
- `resources/win32/VisualElementsManifest.xml` — Tileset icons (150x150, 70x70 PNG references)
- Schema versioning requires desktop4/desktop5/desktop10 namespaces for context menu, shell integration, and trust level declarations

---

## Pattern 2: Linux Desktop Entry & AppData

**Where:** `resources/linux/code.desktop:1-29`

**What:** Freedesktop .desktop file that registers the application with system desktop environments, declaring executable, icon, MIME types, and localized actions.

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
Name[de]=Neues leeres Fenster
Name[es]=Nueva ventana vacía
Name[ja]=新しい空のウィンドウ
Exec=@@EXEC@@ --new-window %F
Icon=@@ICON@@
```

**Variations / siblings:**
- `resources/linux/code-url-handler.desktop` — Registers URL protocol handler (x-scheme-handler/@@URLPROTOCOL@@)
- `resources/linux/code-workspace.xml` — MIME type definition for .code-workspace files
- `resources/linux/code.appdata.xml` — AppStream metadata (homepage, description, screenshots) for app store discovery
- Localization strings embedded in template variables (@@EXEC@@, @@ICON@@, @@NAME_LONG@@)

---

## Pattern 3: Debian Package Control & Installation Hooks

**Where:** `resources/linux/debian/control.template:1-19`

**What:** Package metadata template for Debian/Ubuntu that declares dependencies, version, architecture, maintainer, and alternative-editor registration.

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
```

**Variations / siblings:**
- `resources/linux/debian/postinst.template` — Post-install script (169 lines):
  - Creates /usr/bin symlink
  - Registers editor alternative (update-alternatives)
  - Updates desktop database (update-desktop-database)
  - Updates MIME database (update-mime-database)
  - Manages Microsoft APT repository (deb822 format with GPG key management)
- `resources/linux/debian/prerm.template` — Pre-remove hook for alternatives deregistration
- `resources/linux/debian/postrm.template` — Post-remove cleanup (symlink removal, repo cleanup, database updates)

---

## Pattern 4: RPM Package Specification with Install Directives

**Where:** `resources/linux/rpm/code.spec.template:1-93`

**What:** RPM spec file template defining build layout, dependencies, file ownership, and post-install hooks for Fedora/RHEL/OpenSUSE.

```spec
Name:     @@NAME@@
Version:  @@VERSION@@
Release:  @@RELEASE@@.el8
Summary:  Code editing. Redefined.
Group:    Development/Tools
Vendor:   Microsoft Corporation
License:  @@LICENSE@@
URL:      https://code.visualstudio.com/
Requires: @@DEPENDENCIES@@
AutoReq:  0

%global __provides_exclude_from ^%{_datadir}/%{name}/.*\\.so.*$
%global __brp_strip %{nil}

%install
mkdir -p %{buildroot}%{_bindir}
mkdir -p %{buildroot}%{_datadir}/%{name}
mkdir -p %{buildroot}%{_datadir}/applications
mkdir -p %{buildroot}%{_datadir}/appdata
mkdir -p %{buildroot}%{_datadir}/mime/packages
mkdir -p %{buildroot}%{_datadir}/bash-completion/completions
mkdir -p %{buildroot}%{_datadir}/zsh/site-functions
cp -r usr/share/%{name}/* %{buildroot}%{_datadir}/%{name}
ln -s %{_datadir}/%{name}/bin/%{name} %{buildroot}%{_bindir}/%{name}
cp -r usr/share/applications/%{name}.desktop %{buildroot}%{_datadir}/applications
cp -r usr/share/appdata/%{name}.appdata.xml %{buildroot}%{_datadir}/appdata

%post
update-desktop-database &> /dev/null || :
update-mime-database %{_datadir}/mime &> /dev/null || :

%postun
update-desktop-database &> /dev/null || :
update-mime-database %{_datadir}/mime &> /dev/null || :

%files
%attr(4755, root, root) %{_datadir}/%{name}/chrome-sandbox
%{_bindir}/%{name}
%{_datadir}/%{name}/
%{_datadir}/applications/%{name}.desktop
%{_datadir}/bash-completion/completions/%{name}
%{_datadir}/zsh/site-functions/_%{name}
```

**Variations / siblings:**
- Defines SETUID bit on chrome-sandbox executable (4755)
- Disables ELF stripping (%global __brp_strip) to preserve binary integrity
- Includes shell completion integration
- %files section declares artifact ownership and permissions

---

## Pattern 5: Snap Package Configuration with Build Patching

**Where:** `resources/linux/snap/snapcraft.yaml:1-86`

**What:** Snapcraft manifest declaring plugin strategy, runtime dependencies, build-time patching directives (patchelf for glibc/rpath), and electron launcher wrapper.

```yaml
name: @@NAME@@
version: '@@VERSION@@'
summary: Code editing. Redefined.
description: |
  Visual Studio Code is a new choice of tool that combines the
  simplicity of a code editor with what developers need for the core
  edit-build-debug cycle.

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
      - libgtk-3-0
      - libnss3
      - libxkbcommon0
    prime:
      - -usr/share/doc
      - -usr/share/fonts
    override-build: |
      snapcraftctl build
      patchelf --force-rpath --set-rpath '$ORIGIN/../../lib/x86_64-linux-gnu:$ORIGIN:/snap/core20/current/lib/x86_64-linux-gnu' $SNAPCRAFT_PART_INSTALL/usr/share/@@NAME@@/chrome_crashpad_handler

apps:
  @@NAME@@:
    command: electron-launch $SNAP/usr/share/@@NAME@@/bin/@@NAME@@ --no-sandbox
    common-id: @@NAME@@.desktop
```

**Variations / siblings:**
- Runtime dependency list (20+ libraries for GTK, audio, SSL, X11)
- Build-time filtering of documentation/fonts (prime section)
- patchelf invocation to rewrite binary rpath/interpreter for confinement
- electron-launch wrapper to handle sandbox restrictions
- Cleanup part removes core20 base snap remnants

---

## Pattern 6: Platform-Specific Shell Launchers (CLI Entrypoints)

**Where:** `resources/win32/bin/code.cmd:1-8` (Windows), `resources/darwin/bin/code.sh:1-40` (macOS), `resources/linux/bin/code.sh:1-64` (Linux)

**What:** OS-specific command-line launcher scripts that set environment variables (ELECTRON_RUN_AS_NODE), detect remote/WSL contexts, and invoke the Electron binary with cli.js.

Windows CMD:
```cmd
@echo off
setlocal
set VSCODE_DEV=
set ELECTRON_RUN_AS_NODE=1
"%~dp0..\@@NAME@@.exe" "%~dp0..\resources\app\out\cli.js" %*
IF %ERRORLEVEL% NEQ 0 EXIT /b %ERRORLEVEL%
endlocal
```

macOS Bash:
```bash
if [ -n "$VSCODE_IPC_HOOK_CLI" ]; then
    REMOTE_CLI="$(which -a '@@APPNAME@@' | grep /remote-cli/)"
    if [ -n "$REMOTE_CLI" ]; then
        "$REMOTE_CLI" "$@"
        exit $?
    fi
fi

function app_realpath() {
    SOURCE=$1
    while [ -h "$SOURCE" ]; do
        DIR=$(dirname "$SOURCE")
        SOURCE=$(readlink "$SOURCE")
    done
    echo "${SOURCE_DIR%%${SOURCE_DIR#*.app}}"
}

APP_PATH="$(app_realpath "${BASH_SOURCE[0]}")"
CONTENTS="$APP_PATH/Contents"
ELECTRON="$CONTENTS/MacOS/@@NAME@@"
ELECTRON_RUN_AS_NODE=1 "$ELECTRON" "$CONTENTS/Resources/app/out/cli.js" "$@"
```

Linux Sh (partial):
```bash
if grep -qi Microsoft /proc/version && [ -z "$DONT_PROMPT_WSL_INSTALL" ]; then
    echo "To use @@PRODNAME@@ with WSL, install in Windows..."
    read -r YN
fi

if [ "$(id -u)" = "0" ]; then
    for i in "$@"; do
        case "$i" in
            --user-data-dir | --user-data-dir=* | --file-write | tunnel | serve-web )
                CAN_LAUNCH_AS_ROOT=1 ;;
        esac
    done
    if [ -z $CAN_LAUNCH_AS_ROOT ]; then
        echo "You are trying to start @@PRODNAME@@ as a super user..."
        exit 1
    fi
fi
```

**Variations / siblings:**
- Windows variant: %ERRORLEVEL% checking, relative path resolution with %~dp0
- macOS variant: .app directory structure parsing (Contents/MacOS), symlink resolution with readlink
- Linux variant: WSL detection, root permission checks, symlink vs. direct path branching (56+ lines)
- Common: ELECTRON_RUN_AS_NODE environment variable, remote CLI context detection

---

## Pattern 7: Shell Completions (Bash & Zsh)

**Where:** `resources/completions/bash/code:1-61` (Bash), `resources/completions/zsh/_code:1-40` (Zsh)

**What:** Platform shell completion scripts that provide flag/argument suggestions and context-sensitive file/directory completion.

Bash completion:
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
    
    if [[ $cur == -* ]]; then
        COMPREPLY=( $( compgen -W '-d --diff --folder-uri -a --add -g
            --goto -n --new-window -r --reuse-window --locale=
            --list-extensions --install-extension --uninstall-extension
            --disable-gpu' -- "$cur") )
    fi
    
    _filedir
} &&
complete -F _@@APPNAME@@ @@APPNAME@@
```

Zsh completion (compdef style):
```zsh
#compdef @@APPNAME@@

arguments=(
    '(-d --diff)'{-d,--diff}'[compare two files]:file:_files:file to compare:_files'
    '(-g --goto)'{-g,--goto}'[open file at position]:file\:line[\:column]:_files -r \:'
    '(-n --new-window -r --reuse-window)'{-n,--new-window}'[force new window]'
    '(-w --wait)'{-w,--wait}'[wait for files to close]'
    '--locale=[locale to use]:locale:(de en en-US es fr it ja ko ru zh-CN zh-TW bg hu pt-br tr)'
    '*:file or directory:_files'
)

_arguments -s -S $arguments
```

**Variations / siblings:**
- Bash uses compgen with case statement branching
- Zsh uses #compdef with argument array and _arguments helper
- Both support file/directory/vsix file filtering, enum completions (locales, log levels)
- Installed to `/usr/share/bash-completion/completions/@@NAME@@` and `/usr/share/zsh/site-functions/_@@NAME@@`

---

## Pattern 8: Server-Specific Bootstrap Scripts

**Where:** `resources/server/bin/code-server-linux.sh:1-23`

**What:** Launcher for headless server variant that performs runtime glibc/linker patching (patchelf) for glibc compatibility across distributions.

```bash
#!/usr/bin/env sh

case "$1" in
    --inspect*) INSPECT="$1"; shift;;
esac

ROOT="$(dirname "$(dirname "$(readlink -f "$0")")")"

if [ -n "$VSCODE_SERVER_CUSTOM_GLIBC_LINKER" ] && [ -n "$VSCODE_SERVER_CUSTOM_GLIBC_PATH" ] && [ -n "$VSCODE_SERVER_PATCHELF_PATH" ]; then
    echo "Patching glibc from $VSCODE_SERVER_CUSTOM_GLIBC_PATH with $VSCODE_SERVER_PATCHELF_PATH..."
    "$VSCODE_SERVER_PATCHELF_PATH" --set-rpath "$VSCODE_SERVER_CUSTOM_GLIBC_PATH" "$ROOT/node"
    echo "Patching linker from $VSCODE_SERVER_CUSTOM_GLIBC_LINKER with $VSCODE_SERVER_PATCHELF_PATH..."
    "$VSCODE_SERVER_PATCHELF_PATH" --set-interpreter "$VSCODE_SERVER_CUSTOM_GLIBC_LINKER" "$ROOT/node"
    echo "Patching complete."
fi

"$ROOT/node" ${INSPECT:-} "$ROOT/out/server-main.js" "$@"
```

**Variations / siblings:**
- `resources/server/bin/code-server.cmd` — Windows variant (delegates to cmd)
- `resources/server/bin/code-server-darwin.sh` — macOS variant
- `resources/server/manifest.json` — PWA manifest with icon assets (192x192, 512x512 PNG)
- glibc patching allows a single Linux binary to run across glibc versions

---

## Pattern 9: Windows Visual Elements Manifest

**Where:** `resources/win32/VisualElementsManifest.xml:1-10`

**What:** Windows-specific branding manifest for taskbar/live tile appearance, referencing logo assets.

```xml
<Application xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
    <VisualElements
        BackgroundColor="#2D2D30"
        ShowNameOnSquare150x150Logo="on"
        Square150x150Logo="@@VERSIONFOLDER@@resources\app\resources\win32\code_150x150.png"
        Square70x70Logo="@@VERSIONFOLDER@@resources\app\resources\win32\code_70x70.png"
        ForegroundText="light"
        ShortDisplayName="Code - OSS" />
</Application>
```

---

## Summary

The `resources/` directory contains **9 distinct artifact patterns** that a Tauri bundler must reproduce to port VS Code:

1. **Windows AppxManifest** — UWP app packaging, capabilities, file type associations, COM server registration
2. **Linux Desktop Entries** — Freedesktop .desktop files + MIME type definitions + AppData
3. **Debian Control/Hooks** — Package metadata, symlink creation, repository management, alternative editor registration (169-line postinst)
4. **RPM Spec** — Architecture, build layout, file permissions (SETUID chrome-sandbox), completion integration
5. **Snap Configuration** — Runtime dependency graph, patchelf build-time patching, confinement-aware launcher
6. **CLI Launchers** — OS-specific shell/cmd stubs for ELECTRON_RUN_AS_NODE invocation, remote context detection
7. **Shell Completions** — Bash & Zsh format definitions with context-sensitive file/flag suggestions
8. **Server Bootstrap** — Headless variant with glibc/linker patching for compatibility
9. **Windows Branding** — Taskbar/tile customization via VisualElementsManifest

**Icon/asset artifacts** (115 files total, mostly binary):
- Windows: .ico files (language file icons: json, python, c++, etc.), .bmp installer graphics (6 sizes @ 100-250 DPI)
- Linux: .xpm icon (RPM), PNG icons (appdata)
- Completions, server manifests, and scripts comprise ~30 text files

A Tauri rust-based bundler would need to **template and substitute** variable placeholders (@@NAME@@, @@ARCHITECTURE@@, @@VERSION@@) in all control files, **manage multi-platform asset registration** (deb/rpm/appx/snap), **handle SETUID/permissions** for sandbox binaries, and **integrate system completion databases** for cross-platform CLI discoverability.
