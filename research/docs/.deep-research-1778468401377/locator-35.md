# File Locator: mermaid-chat-features Extension

**Partition:** 35 of 80  
**Scope:** `extensions/mermaid-chat-features/` (25 files, 1,261 LOC)  
**Seed Pattern:** `vscode.chat.$METHOD($$$)` — chat-API surface consumer (newer API)

---

## Implementation

### Extension Entry Point
- `extensions/mermaid-chat-features/src/extension.ts` — Main activation hook; registers chat support, command handlers, and managers
  
### Chat API Integration
- `extensions/mermaid-chat-features/src/chatOutputRenderer.ts` — Implements `vscode.ChatOutputRenderer` interface for rendering Mermaid diagrams in chat
  - **Key API Call:** `vscode.chat.registerChatOutputRenderer(viewType, renderer)` at line 160
  - Implements async `renderChatOutput()` method that manages webview lifecycle and message handling
  - Registers language model tool `renderMermaidDiagram` via `vscode.lm.registerTool()`
  - Encodes tool output with custom MIME type `text/vnd.mermaid` for router to custom renderer

### Webview Management
- `extensions/mermaid-chat-features/src/webviewManager.ts` — Centralized lifecycle and state management for both chat and editor webviews
  - Tracks active webview context
  - Provides webview registration/unregistration with disposable cleanup
  - Handles pan/zoom message routing via `webview.postMessage()`

### Editor Integration
- `extensions/mermaid-chat-features/src/editorManager.ts` — Manages persistent diagram editor panels with serialization
  - Implements `vscode.WebviewPanelSerializer` for state persistence
  - Registers webview panel serializer with `vscode.window.registerWebviewPanelSerializer()`
  - Deduplicates panels using content-based hashing

### Utility Modules
- `extensions/mermaid-chat-features/src/util/dispose.ts` — Disposable lifecycle helpers (base class and batch disposal)
- `extensions/mermaid-chat-features/src/util/html.ts` — HTML entity escaping for XSS prevention
- `extensions/mermaid-chat-features/src/util/uuid.ts` — UUID generation with crypto API fallback

### Webview Runtime (Client-Side)
- `extensions/mermaid-chat-features/chat-webview-src/index.ts` — Chat chat output item webview entry point
  - Initializes Mermaid rendering
  - Wires up "Open in Editor" button with `vscode.postMessage()`

- `extensions/mermaid-chat-features/chat-webview-src/index-editor.ts` — Editor panel webview entry point
  - Initializes pan/zoom controls and event handlers

- `extensions/mermaid-chat-features/chat-webview-src/mermaidWebview.ts` — Mermaid rendering and interactive pan/zoom
  - Wraps diagram in container with pan/zoom state management
  - Handles theme-aware re-rendering via `MutationObserver`
  - Listens for `resetPanZoom` messages from extension
  - Manages persisted state via `vscode.getState()` / `vscode.setState()`

- `extensions/mermaid-chat-features/chat-webview-src/vscodeApi.ts` — TypeScript interface for `acquireVsCodeApi()` API

---

## Configuration

### Extension Manifest
- `extensions/mermaid-chat-features/package.json` — VS Code extension metadata
  - Declares proposed API: `chatOutputRenderer`
  - Registers custom chat output renderer for MIME type `text/vnd.mermaid`
  - Registers language model tool `renderMermaidDiagram` with input schema (markup + title)
  - Defines three internal commands: `_mermaid-chat.resetPanZoom`, `_mermaid-chat.openInEditor`, `_mermaid-chat.copySource`
  - Configuration scope: `mermaid-chat.enabled` (boolean, application-scoped)
  - Activation event: `onWebviewPanel:vscode.chat-mermaid-features.preview`
  - Engines: VS Code ^1.104.0

### TypeScript Configuration
- `extensions/mermaid-chat-features/tsconfig.json` — Extension compilation settings
  - References proposed API type definitions: `vscode.proposed.chatOutputRenderer.d.ts`
  - Outdir: `./out`

- `extensions/mermaid-chat-features/chat-webview-src/tsconfig.json` — Webview compilation settings

- `extensions/mermaid-chat-features/tsconfig.browser.json` — Browser/web build settings

### Build Configuration
- `extensions/mermaid-chat-features/esbuild.webview.mts` — Webview asset bundler
  - Entry points: `index.ts` (chat), `index-editor.ts` (editor), `codicon.css`
  - Output: `chat-webview-out/`

- `extensions/mermaid-chat-features/esbuild.browser.mts` — Browser build configuration (referenced in package.json scripts)

- `extensions/mermaid-chat-features/esbuild.mts` — Extension source bundler

### Git and NPM
- `extensions/mermaid-chat-features/.gitignore` — Git ignore rules
- `extensions/mermaid-chat-features/.npmrc` — NPM configuration
- `extensions/mermaid-chat-features/.vscodeignore` — VS Code package exclusion rules
- `extensions/mermaid-chat-features/package-lock.json` — Lock file (v1.12.16, 7 dependencies)

### Localization
- `extensions/mermaid-chat-features/package.nls.json` — String resources (keys: `displayName`, `description`, `config.enabled.description`)

### Metadata
- `extensions/mermaid-chat-features/cgmanifest.json` — Compliance manifest (typically references dependencies)

---

## Documentation

- `extensions/mermaid-chat-features/README.md` — Extension description
  - Notes bundled status with VS Code
  - References Mermaid.js rendering functionality

---

## Dependencies

**Runtime:**
- `mermaid` (^11.12.3) — Diagram rendering engine
- `dompurify` (^3.4.1) — HTML sanitization

**Development:**
- `@types/node` (^22.18.10)
- `@vscode/codicons` (^0.0.36) — VS Code icon set

---

## Notable Clusters

### Chat Output Renderer Pipeline
1. **Tool Registration** (`chatOutputRenderer.ts:147`) — `vscode.lm.registerTool('renderMermaidDiagram')` outputs encoded Mermaid data
2. **Renderer Registration** (`chatOutputRenderer.ts:160`) — `vscode.chat.registerChatOutputRenderer()` hooks custom rendering
3. **Webview Lifecycle** (`chatOutputRenderer.ts:29-124`) — Chat output renders to isolated webview with CSP + nonce
4. **Webview Manager** (`webviewManager.ts`) — Centralizes state and messaging across chat/editor instances

### Pan/Zoom Interaction Model
- Chat output items: Single-button "Open in Editor" trigger
- Editor panels: Full zoom controls (in/out/reset)
- Shared state persistence via `vscode.getState()` + command routing
- Theme-aware re-rendering via `MutationObserver` on body.classList changes

### Webview HTML Generation
- CSP with nonce-based inline scripts (no external script sources)
- Local resource roots: `chat-webview-out/` directory
- Separates chat output template (single-button, compact) from editor template (full controls)
- Data attributes encode webview ID for command context routing

---

## Summary

The mermaid-chat-features extension demonstrates a modular implementation of VS Code's chat output renderer API. It captures the modern pattern: language model tools produce encoded data (MIME type + binary payload), which the chat system routes to a custom renderer via `vscode.chat.registerChatOutputRenderer()`. The renderer creates isolated webviews with strict CSP, manages state via the VS Code webview state API, and communicates bidirectionally with the extension via `postMessage()`. The webview-side code isolates Mermaid rendering logic (diagram initialization, theme handling) from interaction logic (pan/zoom state machine). Editor integration leverages the same webview manager and webview state persistence, demonstrating code reuse across chat and standalone diagram contexts. All 25 files serve specific roles: extension activation, chat API binding, webview lifecycle, rendering, UI interaction, utilities, and build orchestration.

