# Chat API Surface Consumer Patterns
## Extension: mermaid-chat-features

Research scope: `extensions/mermaid-chat-features/` (14 files, 1,261 LOC)  
Pattern seed: `ast-grep --lang ts -p 'vscode.chat.$METHOD($$$)'` — chat-API surface consumer

---

## Pattern Catalog

#### Pattern: Chat Output Renderer Registration
**Where:** `src/chatOutputRenderer.ts:160`
**What:** Registers a custom chat output renderer to handle Mermaid diagram rendering in chat.
```typescript
const renderer = new MermaidChatOutputRenderer(context.extensionUri, webviewManager);
disposables.push(vscode.chat.registerChatOutputRenderer(viewType, renderer));
```
**Variations / call-sites:** 
- Single registration per extension lifecycle at `src/chatOutputRenderer.ts:160`
- ViewType constant defined at line 20: `'vscode.chat-mermaid-features.chatOutputItem'`

---

#### Pattern: Chat Output Renderer Interface Implementation
**Where:** `src/chatOutputRenderer.ts:22-126`
**What:** Implements `vscode.ChatOutputRenderer` interface with async `renderChatOutput` method for rendering diagram data in chat output webviews.
```typescript
class MermaidChatOutputRenderer implements vscode.ChatOutputRenderer {

	constructor(
		private readonly _extensionUri: vscode.Uri,
		private readonly _webviewManager: MermaidWebviewManager
	) { }

	async renderChatOutput({ value }: vscode.ChatOutputDataItem, chatOutputWebview: vscode.ChatOutputWebview, _ctx: unknown, _token: vscode.CancellationToken): Promise<void> {
		const webview = chatOutputWebview.webview;
		const decoded = decodeMermaidData(value);
		const mermaidSource = decoded.source;
		const title = decoded.title;

		// Generate unique ID for this webview
		const webviewId = generateUuid();

		const disposables: vscode.Disposable[] = [];

		// Register and set as active
		disposables.push(this._webviewManager.registerWebview(webviewId, webview, mermaidSource, title, 'chat'));

		// Listen for messages from the webview
		disposables.push(webview.onDidReceiveMessage(message => {
			if (message.type === 'openInEditor') {
				vscode.commands.executeCommand('_mermaid-chat.openInEditor', { mermaidWebviewId: webviewId });
			}
		}));

		// Dispose resources when webview is disposed
		chatOutputWebview.onDidDispose(() => {
			disposeAll(disposables);
		});
```
**Variations / call-sites:** Single implementation at line 22-126; method called by chat framework with rendered data.

---

#### Pattern: Language Model Tool Registration
**Where:** `src/chatOutputRenderer.ts:147-154`
**What:** Registers a language model tool via `vscode.lm.registerTool` that handles Mermaid diagram generation by accepting markup and title inputs.
```typescript
disposables.push(
	vscode.lm.registerTool<{ markup: string; title?: string }>('renderMermaidDiagram', {
		invoke: async (options, _token) => {
			const sourceCode = options.input.markup;
			const title = options.input.title;
			return writeMermaidToolOutput(sourceCode, title);
		},
	})
);
```
**Variations / call-sites:** Single registration at line 147; called by LM framework when chat asks to render diagrams.

---

#### Pattern: Language Model Tool Result with Custom MIME Type
**Where:** `src/chatOutputRenderer.ts:165-182`
**What:** Returns structured tool result with custom MIME type and binary data to indicate custom renderer should be used, combining markdown fallback with binary metadata.
```typescript
function writeMermaidToolOutput(sourceCode: string, title: string | undefined): vscode.LanguageModelToolResult {
	// Expose the source code as a markdown mermaid code block
	const fence = getFenceForContent(sourceCode);
	const result = new vscode.LanguageModelToolResult([
		new vscode.LanguageModelTextPart(`${fence}mermaid\n${sourceCode}\n${fence}`)
	]);

	// And store custom data in the tool result details to indicate that a custom renderer should be used for it.
	// Encode source and optional title as JSON.
	const data = JSON.stringify({ source: sourceCode, title });
	// Add cast to use proposed API
	(result as vscode.ExtendedLanguageModelToolResult2).toolResultDetails2 = {
		mime,
		value: new TextEncoder().encode(data),
	};

	return result;
}
```
**Variations / call-sites:** Called from tool's invoke handler at line 151; MIME constant defined at line 15: `'text/vnd.mermaid'`.

---

#### Pattern: Chat Output Webview Message Handling
**Where:** `src/chatOutputRenderer.ts:44-48`
**What:** Listens for webview messages from rendered chat content and executes corresponding commands in the main extension.
```typescript
disposables.push(webview.onDidReceiveMessage(message => {
	if (message.type === 'openInEditor') {
		vscode.commands.executeCommand('_mermaid-chat.openInEditor', { mermaidWebviewId: webviewId });
	}
}));
```
**Variations / call-sites:** 
- Chat renderer webview message handler at `src/chatOutputRenderer.ts:44`
- Corresponding webview-side sender at `chat-webview-src/index.ts:18-21`: `vscode.postMessage({ type: 'openInEditor' })`

---

#### Pattern: Chat Output Webview Disposal Listener
**Where:** `src/chatOutputRenderer.ts:51-53`
**What:** Registers cleanup handler when chat output webview is disposed to free associated resources.
```typescript
chatOutputWebview.onDidDispose(() => {
	disposeAll(disposables);
});
```
**Variations / call-sites:** Single usage in renderer lifecycle at line 51.

---

#### Pattern: Command Execution from Chat Context
**Where:** `src/chatOutputRenderer.ts:137-142`
**What:** Registers private command with context parameter containing mermaidWebviewId, triggered by webview messages, to open diagram in editor.
```typescript
disposables.push(
	vscode.commands.registerCommand('_mermaid-chat.openInEditor', (ctx?: { mermaidWebviewId?: string }) => {
		const webviewInfo = ctx?.mermaidWebviewId ? webviewManager.getWebview(ctx.mermaidWebviewId) : webviewManager.activeWebview;
		if (webviewInfo) {
			editorManager.openPreview(webviewInfo.mermaidSource, webviewInfo.title);
		}
	})
);
```
**Variations / call-sites:** 
- Primary registration at `src/chatOutputRenderer.ts:137`
- Invoked from chat webview at `src/chatOutputRenderer.ts:46`
- Similar pattern for `_mermaid-chat.resetPanZoom` at `src/extension.ts:22`
- Similar pattern for `_mermaid-chat.copySource` at `src/extension.ts:28`

---

## Integration Points

### Chat API Surface Used
- `vscode.chat.registerChatOutputRenderer()` — registers custom output renderer
- `vscode.ChatOutputRenderer` — interface for rendering chat output
- `vscode.ChatOutputDataItem` — input data for rendering
- `vscode.ChatOutputWebview` — webview container for rendered content
- `webview.onDidReceiveMessage()` — listen to webview messages
- `chatOutputWebview.onDidDispose()` — cleanup on disposal

### Language Model API Integration
- `vscode.lm.registerTool()` — register tool for LM to invoke
- `vscode.LanguageModelToolResult` — structured tool output
- `vscode.LanguageModelTextPart` — text content in tool result
- `vscode.ExtendedLanguageModelToolResult2` — proposed API for custom MIME types

### Command API
- `vscode.commands.registerCommand()` — register private commands
- `vscode.commands.executeCommand()` — invoke from webview handlers

### Webview API
- `vscode.Webview.postMessage()` — send messages from extension to webview
- `vscode.window.createWebviewPanel()` — create editor webview panels
- `vscode.window.registerWebviewPanelSerializer()` — restore webview state

---

## Message Flow Diagram

```
Chat User
  ↓
[Language Model] asks for diagram
  ↓
vscode.lm.registerTool() invoked
  ↓
writeMermaidToolOutput() returns LanguageModelToolResult
  ↓
Chat renders with custom MIME type
  ↓
vscode.chat.registerChatOutputRenderer() invoked
  ↓
renderChatOutput() receives diagram data
  ↓
Webview shows diagram + "Open in Editor" button
  ↓
User clicks button
  ↓
vscode.postMessage({ type: 'openInEditor' })
  ↓
onDidReceiveMessage listener catches event
  ↓
vscode.commands.executeCommand('_mermaid-chat.openInEditor', { mermaidWebviewId })
  ↓
Opens in editor panel via editorManager
```

---

## File Summary

### Main Implementation Files
- **`src/extension.ts`** — Activates extension, registers webview serializer, registers helper commands
- **`src/chatOutputRenderer.ts`** — Core chat integration: tool registration, output renderer impl, tool result generation
- **`src/editorManager.ts`** — Editor panel creation/serialization, webview panel lifecycle
- **`src/webviewManager.ts`** — Tracks all webviews (chat and editor), manages messaging and active state

### Webview Source Files
- **`chat-webview-src/index.ts`** — Chat webview entry: initializes Mermaid, wires up "Open in Editor" button
- **`chat-webview-src/index-editor.ts`** — Editor webview entry: wires up zoom controls
- **`chat-webview-src/mermaidWebview.ts`** — Shared webview logic: pan/zoom handler, theme support, state persistence
- **`chat-webview-src/vscodeApi.ts`** — VSCode API abstraction for webview (postMessage, getState, setState)

### Utilities
- **`src/util/dispose.ts`** — Disposable resource management
- **`src/util/html.ts`** — HTML escaping for security
- **`src/util/uuid.ts`** — UUID generation

---

## Key Design Patterns

1. **Tool + Renderer Pair** — Tool generates content, renderer displays it with custom formatting
2. **MIME-Based Rendering** — Uses custom MIME type to trigger specific renderer
3. **Webview Manager Pattern** — Central registry tracking all webviews for coordinated messaging
4. **Message-Based Extension-Webview IPC** — Async message passing with typed event handling
5. **Command Dispatch** — Private commands bridge webview events to extension logic
6. **Disposal Coordination** — Tracked disposables ensure cleanup when webviews close

