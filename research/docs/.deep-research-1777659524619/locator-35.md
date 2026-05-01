# Chat API Surface Analysis - mermaid-chat-features Extension

## Research Context
The partition seed referenced `vscode.chat.createChatParticipant($$$)` as a reference for chat-API consumers and the Copilot surface. This analysis examined the mermaid-chat-features extension (14 files, ~1,261 LOC) to identify how VS Code's chat platform integrates with extension code.

**Key Finding:** This extension does NOT use `createChatParticipant()`. Instead, it implements two complementary chat APIs:
1. **Chat Output Renderer** (`vscode.chat.registerChatOutputRenderer`) - for rendering structured output
2. **Language Model Tools** (`vscode.lm.registerTool`) - for AI tool invocation

---

## Implementation

- `extensions/mermaid-chat-features/src/chatOutputRenderer.ts` — Core chat integration; registers both chat output renderer and language model tool; demonstrates tool result encoding with MIME type
- `extensions/mermaid-chat-features/src/extension.ts` — Entry point; activates chat support and command handlers
- `extensions/mermaid-chat-features/src/webviewManager.ts` — Manages webview lifecycle for chat output items and editor previews
- `extensions/mermaid-chat-features/src/editorManager.ts` — Handles editor panel serialization and preview state persistence

---

## Types / Interfaces

- `extensions/mermaid-chat-features/src/webviewManager.ts` — `MermaidWebviewInfo` interface tracking webview identity, content, and type
- `extensions/mermaid-chat-features/src/chatOutputRenderer.ts` — `MermaidData` interface encoding diagram source + optional title as JSON in tool result details
- `extensions/mermaid-chat-features/chat-webview-src/vscodeApi.ts` — `VsCodeApi` interface wrapping state/messaging API for webview layer

---

## Configuration

- `extensions/mermaid-chat-features/package.json` — Declares `enabledApiProposals: ["chatOutputRenderer"]`; registers chat output renderer in contributes; defines `renderMermaidDiagram` language model tool with input schema; setting `mermaid-chat.enabled` gates tool availability
- `extensions/mermaid-chat-features/tsconfig.json` — Includes proposed API type definitions: `vscode.proposed.chatOutputRenderer.d.ts`, `vscode.proposed.chatParticipantAdditions.d.ts`

---

## Examples / Fixtures

- `extensions/mermaid-chat-features/src/chatOutputRenderer.ts` (lines 147-154) — Language model tool registration pattern with input options and async invoke callback
- `extensions/mermaid-chat-features/src/chatOutputRenderer.ts` (lines 165-181) — Tool result encoding: wraps markdown fence + embedded MIME data in `toolResultDetails2` using `ExtendedLanguageModelToolResult2`
- `extensions/mermaid-chat-features/src/chatOutputRenderer.ts` (lines 199-213) — Forward/backward compatible Mermaid data decoding (JSON with fallback to legacy plain text)

---

## Notable Clusters

- `extensions/mermaid-chat-features/src/` — 5 files; extension activation, chat support registration, webview lifecycle management
- `extensions/mermaid-chat-features/chat-webview-src/` — 5 files; webview runtime layer with Mermaid rendering, pan/zoom interaction, theme switching
- `extensions/mermaid-chat-features/src/util/` — 3 files; reusable disposal pattern, HTML escaping, UUID generation

---

## Summary

The mermaid-chat-features extension exemplifies how chat-aware extensions integrate without creating custom chat participants. Instead of `createChatParticipant()`, it:

1. **Registers a Language Model Tool** (`renderMermaidDiagram`) callable by Copilot and other AI agents
2. **Implements a Chat Output Renderer** to customize display of tool results with a custom MIME type
3. **Bridges Extension ↔ Webview** messaging for interactive pan/zoom and state persistence
4. **Encodes Structured Data** in tool results using `ExtendedLanguageModelToolResult2.toolResultDetails2` (MIME + binary payload)

This pattern supports **Tauri/Rust porting** by showing how the API boundary separates:
- **Host Extension Layer** (registration, lifecycle, IPC) — currently TypeScript/Electron-specific
- **Webview Layer** (Mermaid rendering, interaction) — runtime-agnostic (reusable in Tauri)

The proposed API surface (`chatOutputRenderer`, `languageModelTools`) would need first-class Rust bindings in a Tauri architecture.
