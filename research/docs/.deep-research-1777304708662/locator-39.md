# Partition 39: Simple Browser Extension (WebView-Based Browser)

## Implementation

Extension entry point and core browser management:
- `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/src/extension.ts` - Extension activation, command registration (simpleBrowser.api.open, simpleBrowser.show), external URI opener registration for http/https schemes targeting localhost and IPv4/IPv6 loopback addresses
- `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/src/simpleBrowserManager.ts` - Manager class handling lifecycle of browser views, show/restore functionality
- `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/src/simpleBrowserView.ts` - Core WebView panel implementation. Generates HTML with iframe sandbox, Content-Security-Policy headers, manages webview options (enableScripts, enableForms), handles configuration changes, and communication between extension and webview
- `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/src/dispose.ts` - Base Disposable class for resource cleanup pattern
- `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/src/uuid.ts` - UUID generation for nonce values in CSP headers

Webview frontend implementation:
- `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/preview-src/index.ts` - Client-side browser logic: iframe navigation, history (back/forward/reload), URL input handling, focus lock indicator toggle, message passing via vscode.postMessage()
- `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/preview-src/events.ts` - DOM ready helper function

## Configuration

Extension manifest and metadata:
- `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/package.json` - Extension metadata, version 10.0.0, activation events (onCommand, onOpenExternalUri, onWebviewPanel), configuration schema for focusLockIndicator.enabled, build scripts
- `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/package.nls.json` - Localization strings

Build configuration:
- `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/tsconfig.json` - TypeScript config for src/ (Node types, VSCode API definitions)
- `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/tsconfig.browser.json` - Browser-specific TypeScript config extending base
- `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/preview-src/tsconfig.json` - TypeScript config for webview source

Build scripts:
- `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/esbuild.mts` - Main extension build (references common build runner)
- `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/esbuild.webview.mts` - Webview bundle build (entry point: preview-src/index.ts, output: media/, includes codicon.css)
- `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/esbuild.browser.mts` - Browser platform extension build

Utility configs:
- `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/.vscodeignore` - Extension packaging exclusions
- `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/.npmrc` - NPM configuration
- `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/.gitignore` - Git exclusions

## Documentation

- `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/README.md` - Basic overview describing Simple Browser as iframe-embedded preview for other extensions

## Examples / Fixtures

Media assets:
- `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/media/main.css` - Webview UI styling: header with navigation controls (back/forward/reload/open-external buttons), URL input field, iframe container, focus lock indicator alert box
- `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/media/icon.png` - Extension icon
- `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/media/preview-light.svg` - Light theme preview
- `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/media/preview-dark.svg` - Dark theme preview

Lock file:
- `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/package-lock.json` - NPM dependency lock

## Notable Clusters

**WebView HTML Generation** (`simpleBrowserView.ts` lines 113-174): Dynamic HTML construction with:
- CSP enforcement: `default-src 'none'`, `frame-src *`, script nonce injection
- Settings injection via data attribute (`simple-browser-settings`)
- Iframe sandbox attributes: `allow-scripts allow-forms allow-same-origin allow-downloads`
- Codicon icon font integration

**Focus Lock Detection** (implemented across components): Periodic DOM polling (50ms interval) checking if iframe is focused element, toggling CSS class for visual indicator

**Message Bridge**: Two-way communication between extension and webview:
- Extension → Webview: `focus`, `didChangeFocusLockIndicatorEnabled` messages
- Webview → Extension: `openExternal` message to open URLs in system browser

**Build Output Targets**: 
- Desktop/Node: `out/extension.js` (from src/)
- Browser/Web: `dist/browser/extension.js` (from src/)
- Webview: `media/index.js` (from preview-src/)

## Summary

The Simple Browser extension is a lightweight webview-based browser preview component (636 LOC across 10 primary source files) that embeds HTML content in iframe sandboxes with strict CSP policies. It demonstrates VS Code's webview API for iframe embedding, command and URI handler registration, bidirectional messaging between extension and webview contexts, and configuration-driven UI behavior (focus lock indicator). The architecture separates extension logic (TypeScript), webview frontend (TypeScript compiled to client-side JavaScript), and build pipeline optimization for both desktop and web platforms. This serves as a reference implementation for Tauri's webview embedding capabilities, showing how to securely instantiate nested web contexts with controlled CSP headers and cross-context messaging.
