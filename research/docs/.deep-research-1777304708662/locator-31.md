# Partition 31: Media Preview Extension — Tauri/Rust Port Research

## Overview
This partition covers the media preview extension (image, audio, video custom editors) which demonstrates core VS Code webview infrastructure patterns critical to understanding what a Tauri/Rust port would need to replicate.

---

## Implementation

### Core Extension Logic
- `extensions/media-preview/src/extension.ts` — Extension activation entry point; registers three custom editor providers (image, audio, video) and initializes status bar entries

### Custom Editor Providers & Infrastructure
- `extensions/media-preview/src/imagePreview/index.ts` — ImagePreviewManager implementing `vscode.CustomReadonlyEditorProvider`; manages multiple image preview instances, handles zoom/size state via status bar; webview message passing for zoom/pan interactions
- `extensions/media-preview/src/audioPreview.ts` — AudioPreviewProvider implementing `vscode.CustomReadonlyEditorProvider`; lightweight webview-based audio player with HTML5 audio element
- `extensions/media-preview/src/videoPreview.ts` — VideoPreviewProvider implementing `vscode.CustomReadonlyEditorProvider`; webview-based video player with configurable autoplay/loop settings from workspace config

### Base Classes & Patterns
- `extensions/media-preview/src/mediaPreview.ts` — Abstract MediaPreview class extending Disposable; shared webview lifecycle management (onDidChangeViewState, onDidDispose), file system watching, webview options configuration with CSP, enableScripts, localResourceRoots
- `extensions/media-preview/src/ownedStatusBarEntry.ts` — PreviewStatusBarEntry abstract base class; status bar item lifecycle management
- `extensions/media-preview/src/binarySizeStatusBarEntry.ts` — BinarySizeStatusBarEntry for displaying file size; extends PreviewStatusBarEntry with binary size formatting (B/KB/MB/GB/TB)

### Image Preview Specific
- `extensions/media-preview/src/imagePreview/sizeStatusBarEntry.ts` — SizeStatusBarEntry displays image dimensions (width × height)
- `extensions/media-preview/src/imagePreview/zoomStatusBarEntry.ts` — ZoomStatusBarEntry with Scale type (number | 'fit'); manages zoom level selection via quick pick, fires onDidChangeScale events, registers select zoom level command

### Utilities
- `extensions/media-preview/src/util/dispose.ts` — Disposable base class for resource lifecycle management; `disposeAll()` utility function
- `extensions/media-preview/src/util/dom.ts` — `escapeAttribute()` function for XSS prevention when building webview HTML
- `extensions/media-preview/src/util/uuid.ts` — `generateUuid()` function using crypto.randomUUID or fallback Uint8Array implementation; used for CSP nonce generation

### Webview Content (JavaScript)
- `extensions/media-preview/media/imagePreview.js` — Client-side image viewer: zoom (10x to 0.1x, fit-to-window), pan, keyboard shortcuts (ctrl/cmd+scroll, arrow keys), state persistence via `vscode.setState()`, `vscode.postMessage()` for reopen-as-text command, `window.addEventListener('message')` for state updates from extension
- `extensions/media-preview/media/audioPreview.js` — Client-side audio player; minimal logic, mainly HTML5 audio element with error handling and fallback to text editor
- `extensions/media-preview/media/videoPreview.js` — Client-side video player; HTML5 video element with error handling, configuration-driven autoplay/loop settings

### Webview Content (CSS)
- `extensions/media-preview/media/imagePreview.css` — Styles for image viewer (fullscreen container, loading indicator, pixelation at scale > 3x)
- `extensions/media-preview/media/audioPreview.css` — Styles for audio player (centered container, dark theme support)
- `extensions/media-preview/media/videoPreview.css` — Styles for video player (responsive container, loading indicator)

---

## Configuration

### Extension Manifest
- `extensions/media-preview/package.json` — Declares three customEditors with viewTypes (imagePreview.previewEditor, vscode.audioPreview, vscode.videoPreview), priority "builtin", file selector patterns; contributes zoom/copy/reopen commands; settings for video autoPlay and loop; supports virtual/untrusted workspaces

### TypeScript Build Configuration
- `extensions/media-preview/tsconfig.json` — Extends tsconfig.base.json; targets Node.js runtime; includes vscode.d.ts type definitions
- `extensions/media-preview/tsconfig.browser.json` — Extends tsconfig.json (no additional changes in this project)

### Build Scripts
- `extensions/media-preview/esbuild.mts` — esbuild configuration for Node.js platform; compiles src/extension.ts to dist/extension
- `extensions/media-preview/esbuild.browser.mts` — esbuild configuration for browser platform; compiles src/extension.ts to dist/browser/extension.js using tsconfig.browser.json

### Localization
- `extensions/media-preview/package.nls.json` — i18n strings for display names, command titles, status messages (Image/Audio/Video Preview, zoom commands, reopen actions)

### Development Configuration
- `extensions/media-preview/.npmrc` — Legacy peer deps mode; 180s timeout; min release age 1
- `extensions/media-preview/.vscodeignore` — Excludes dev files from bundled extension

---

## Types / Interfaces

### TypeScript Type Definitions
- `extensions/media-preview/src/imagePreview/zoomStatusBarEntry.ts` — `type Scale = number | 'fit'` (zoom level union type)
- `extensions/media-preview/src/mediaPreview.ts` — `const enum PreviewState { Disposed, Visible, Active }` (lifecycle states)

### VS Code API Interfaces Used (Not Defined Here)
- `vscode.CustomReadonlyEditorProvider` — Implemented by ImagePreviewManager, AudioPreviewProvider, VideoPreviewProvider
- `vscode.WebviewPanel` — Type passed to resolveCustomEditor methods
- `vscode.ExtensionContext` — Type passed to activate function and register functions
- `vscode.CustomDocument` — Type passed to openCustomDocument and resolveCustomEditor
- `vscode.Disposable` — Base type for lifecycle management (imported from vscode module)

---

## Documentation

- `extensions/media-preview/README.md` — User-facing documentation; lists supported image formats (jpg, png, bmp, gif, ico, webp, avif, svg), audio formats (mp3, wav, ogg), video formats (mp4, webm); notes bundled extension status, no uninstall capability

---

## Notable Clusters

### `extensions/media-preview/src/imagePreview/` — 3 files
- Implements full image preview with zoom/pan; demonstrates complex webview state management, status bar integration, and command handling

### `extensions/media-preview/media/` — 6 files (CSS + JS + SVG loaders)
- Client-side webview implementations for image/audio/video players; shows postMessage API, event handling, DOM manipulation within webview context; loading spinner SVG assets (loading.svg, loading-dark.svg, loading-hc.svg)

### `extensions/media-preview/src/util/` — 3 files
- Cross-cutting utility patterns: lifecycle management (Disposable), DOM safety (escapeAttribute), cryptographic UUID generation; importable throughout extension

---

## Tauri/Rust Port Relevance

This extension is highly relevant to a Tauri/Rust port because:

1. **Webview Architecture Pattern**: Demonstrates core VS Code pattern of webview-based custom editors with message passing (`postMessage`/`onDidReceiveMessage`) for asynchronous editor ↔ webview communication

2. **CSP & Security**: Shows how VS Code enforces Content Security Policy in webviews (nonce-based inline scripts, localResourceRoots restrictions), critical to replicate in Tauri's webview security model

3. **State Management**: Demonstrates vscode.setState/getState persistence API and how previews maintain state across visibility changes—Tauri would need equivalent serialization layer

4. **Status Bar Integration**: Shows command/event-driven UI updates to status bar from webview interactions (zoom level, image size)—indicates rich extension ↔ IDE communication needs

5. **Resource Loading**: Illustrates asWebviewUri URI rewriting for webview resource access with localResourceRoots isolation—a critical security boundary in both Electron and Tauri

6. **Configuration Access**: Uses vscode.workspace.getConfiguration for runtime settings (video autoplay/loop), showing extension-level config binding—Tauri would need config provider API

7. **Workspace FS API**: Uses vscode.workspace.fs.stat and createFileSystemWatcher for file monitoring and metadata—indicates Tauri needs robust async file I/O layer

8. **Disposable Pattern**: Core Disposable base class shows how VS Code manages resource cleanup; would map to Rust's Drop trait and Arc<Mutex<>> patterns

9. **Localization (l10n)**: Uses vscode.l10n.t() for i18n strings—Tauri port needs i18n integration at API level

10. **Zero Test Files**: No unit tests in this partition indicates test infrastructure would need to be designed for Tauri model (unlike Electron's Jest/Mocha integration)

