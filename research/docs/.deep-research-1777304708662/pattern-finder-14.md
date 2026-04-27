# Webview API Integration Patterns: Markdown Language Features Extension

## Overview
This analysis documents concrete code patterns used in the markdown-language-features extension (85 files, 8,661 LOC) that demonstrate VS Code's Webview API usage. These patterns are critical for understanding the abstraction layer that would need to be ported when migrating from TypeScript/Electron to Tauri/Rust.

---

## Pattern 1: Webview Panel Lifecycle Management

**Where:** `extensions/markdown-language-features/src/preview/preview.ts:655-658`
**What:** Creating a webview panel with configuration and title management for dynamic markdown previews.

```typescript
const webview = vscode.window.createWebviewPanel(
	DynamicMarkdownPreview.viewType,
	DynamicMarkdownPreview.#getPreviewTitle(input.resource, input.locked),
	previewColumn, { enableFindWidget: true, });

webview.iconPath = contentProvider.iconPath;
```

**Key aspects:**
- `createWebviewPanel()` is the primary constructor for panel-based webviews
- Accepts viewType identifier, title, column position, and options object
- Options include `enableFindWidget`, `enableScripts`, `enableForms`, `localResourceRoots`
- Panel lifecycle tied to disposal and view state changes

**Call sites:**
- `src/preview/preview.ts:645-664` - `DynamicMarkdownPreview.create()`
- Registered with serializer at `src/preview/previewManager.ts:100`
- Custom editor registration at `src/preview/previewManager.ts:102`

---

## Pattern 2: Bidirectional Message Passing Between Extension and Webview

**Where:** `extensions/markdown-language-features/src/preview/preview.ts:141-172`
**What:** Receiving and dispatching messages from webview to extension, with source routing.

```typescript
this._register(this.#webviewPanel.webview.onDidReceiveMessage((e: FromWebviewMessage.Type) => {
	if (e.source !== this.#resource.toString()) {
		return;
	}

	switch (e.type) {
		case 'cacheImageSizes':
			this.#imageInfo = e.imageData;
			break;

		case 'revealLine':
			this.#onDidScrollPreview(e.line);
			break;

		case 'didClick':
			this.#onDidClickPreview(e.line);
			break;

		case 'openLink':
			this.#onDidClickPreviewLink(e.href);
			break;

		case 'showPreviewSecuritySelector':
			vscode.commands.executeCommand('markdown.showPreviewSecuritySelector', e.source);
			break;

		case 'previewStyleLoadError':
			vscode.window.showWarningMessage(
				vscode.l10n.t("Could not load 'markdown.styles': {0}", e.unloadedStyles.join(', ')));
			break;
	}
}));
```

**Key aspects:**
- `onDidReceiveMessage()` returns an event emitter for webview→extension messages
- Source field routes messages to correct preview instance
- Type-safe message handling via discriminated union
- Each message type handles different interaction (scroll, click, link, error)

**Message types defined:** `types/previewMessaging.d.ts:10-49`
- `CacheImageSizes`, `RevealLine`, `DidClick`, `ClickLink`, `ShowPreviewSecuritySelector`, `PreviewStyleLoadError`

---

## Pattern 3: Sending Messages to Webview from Extension

**Where:** `extensions/markdown-language-features/src/preview/preview.ts:226-230`
**What:** Wrapper method to post messages from extension to webview with disposal check.

```typescript
public postMessage(msg: ToWebviewMessage.Type) {
	if (!this.#disposed) {
		this.#webviewPanel.webview.postMessage(msg);
	}
}
```

**Usage examples:**
- Scroll position updates at `src/preview/preview.ts:244-248`
- Content updates at `src/preview/preview.ts:377-381`
- Image copy commands at `src/preview/preview.ts:567-571`
- Text editor selection at `src/preview/preview.ts:715-720`

**Key aspects:**
- Disposal check prevents sending to destroyed panels
- Type-safe message objects sent to webview
- Messages include source identifier for routing
- Part of larger `ToWebviewMessage` namespace with types: `UpdateView`, `UpdateContent`, `CopyImageContent`, `OnDidChangeTextEditorSelection`

---

## Pattern 4: Webview Options and Security Configuration

**Where:** `extensions/markdown-language-features/src/preview/preview.ts:408-430`
**What:** Setting webview options including script execution, form handling, and resource roots.

```typescript
#getWebviewOptions(): vscode.WebviewOptions {
	return {
		enableScripts: true,
		enableForms: false,
		localResourceRoots: this.#getLocalResourceRoots()
	};
}

#getLocalResourceRoots(): ReadonlyArray<vscode.Uri> {
	const baseRoots = Array.from(this.#contributionProvider.contributions.previewResourceRoots);

	const folder = vscode.workspace.getWorkspaceFolder(this.#resource);
	if (folder) {
		const workspaceRoots = vscode.workspace.workspaceFolders?.map(folder => folder.uri);
		if (workspaceRoots) {
			baseRoots.push(...workspaceRoots);
		}
	} else {
		baseRoots.push(uri.Utils.dirname(this.#resource));
	}

	return baseRoots;
}
```

**Applied at:** `src/preview/preview.ts:372`

**Key aspects:**
- `enableScripts: true` for interactive content
- `enableForms: false` restricts form submission
- `localResourceRoots` restricts file access to specific directories
- Roots include workspace folders, extension contributions, and document directory
- Security-critical configuration for sandboxed webview

---

## Pattern 5: Resource URI Transformation for Webview Serving

**Where:** `extensions/markdown-language-features/src/util/resources.ts:8-12`
**What:** Interface abstraction for converting file URIs to webview-accessible URIs.

```typescript
export interface WebviewResourceProvider {
	asWebviewUri(resource: vscode.Uri): vscode.Uri;

	readonly cspSource: string;
}
```

**Implementation at:** `src/preview/preview.ts:454-470`
```typescript
asWebviewUri(resource: vscode.Uri) {
	return this.#webviewPanel.webview.asWebviewUri(resource);
}

get cspSource() {
	return [
		this.#webviewPanel.webview.cspSource,

		// On web, we also need to allow loading of resources from contributed extensions
		...this.#contributionProvider.contributions.previewResourceRoots
			.filter(root => root.scheme === 'http' || root.scheme === 'https')
			.map(root => {
				const dirRoot = root.path.endsWith('/') ? root : root.with({ path: root.path + '/' });
				return dirRoot.toString();
			}),
	].join(' ');
}
```

**Usage sites:**
- Image loading at `src/markdownEngine.ts:371, 393`
- Base href at `src/preview/documentRenderer.ts:118`
- Extension resources at `src/preview/documentRenderer.ts:154-156`
- Style sheet links at `src/preview/documentRenderer.ts:222`
- Script inclusion at `src/preview/documentRenderer.ts:234`

**Key aspects:**
- Transforms `file://` to webview-safe URIs
- CSP source generation for inline security policies
- Handles both local files and web-based resources
- Essential for serving extension media files

---

## Pattern 6: File System Watching with Targeted Refresh

**Where:** `extensions/markdown-language-features/src/preview/preview.ts:128-139`
**What:** Watching document directory for changes and triggering preview updates.

```typescript
if (vscode.workspace.fs.isWritableFileSystem(resource.scheme)) {
	const watcher = this._register(vscode.workspace.createFileSystemWatcher(new vscode.RelativePattern(resource, '*')));
	this._register(watcher.onDidChange(uri => {
		if (this.isPreviewOf(uri)) {
			// Only use the file system event when VS Code does not already know about the file.
			// This is needed to avoid duplicate refreshes
			if (!vscode.workspace.textDocuments.some(doc => areUrisEqual(doc.uri, uri))) {
				this.refresh();
			}
		}
	}));
}
```

**Related patterns:**
- Image watcher at `src/preview/preview.ts:396-406`
- Global markdown watcher at `src/client/workspace.ts:29-40`
- Client-level file watcher at `src/client/client.ts:72`
- Manager-level watchers at `src/client/fileWatchingManager.ts:36-54`

**Key aspects:**
- `createFileSystemWatcher()` with glob patterns
- `RelativePattern` for workspace-relative watching
- Deduplication to prevent redundant refreshes
- Scheme checking for writable file systems
- Events: `onDidChange`, `onDidCreate`, `onDidDelete`

---

## Pattern 7: Webview Panel Serialization and Restoration

**Where:** `extensions/markdown-language-features/src/preview/previewManager.ts:188-244`
**What:** Implementing `WebviewPanelSerializer` to restore webview state across VS Code restarts.

```typescript
public async deserializeWebviewPanel(
	webview: vscode.WebviewPanel,
	state: any
): Promise<void> {
	try {
		const resource = vscode.Uri.parse(state.resource);
		const locked = state.locked;
		const line = state.line;
		const resourceColumn = state.resourceColumn;

		const preview = DynamicMarkdownPreview.revive(
			{ resource, locked, line, resourceColumn },
			webview,
			this.#contentProvider,
			this.#previewConfigurations,
			this.#logger,
			this.#topmostLineMonitor,
			this.#contributions,
			this.#opener);

		this.#registerDynamicPreview(preview);
	} catch (e) {
		console.error(e);

		webview.webview.html = /* html */`<!DOCTYPE html>
		<html lang="en">
		<head>
			<meta charset="UTF-8">

			<!-- Disable pinch zooming -->
			<meta name="viewport"
				content="width=device-width, initial-scale=1.0, maximum-scale=1.0, minimum-scale=1.0, user-scalable=no">

			<title>Markdown Preview</title>

			<style>
				html, body {
					min-height: 100%;
					height: 100%;
				}

				.error-container {
					display: flex;
					justify-content: center;
					align-items: center;
					text-align: center;
				}
			</style>

			<meta http-equiv="Content-Security-Policy" content="default-src 'none';">
		</head>
		<body class="error-container">
			<p>${vscode.l10n.t("An unexpected error occurred while restoring the Markdown preview.")}</p>
		</body>
		</html>`;
	}
}
```

**Registration at:** `src/preview/previewManager.ts:100`
```typescript
this._register(vscode.window.registerWebviewPanelSerializer(DynamicMarkdownPreview.viewType, this));
```

**State preservation at:** `src/preview/preview.ts:195-202`
```typescript
public get state() {
	return {
		resource: this.#resource.toString(),
		line: this.#line,
		fragment: this.#scrollToFragment,
		...this.#delegate.getAdditionalState(),
	};
}
```

**Key aspects:**
- Serializable state includes resource URI, scroll position, and view settings
- Error handling with fallback UI
- Type-safe deserialization from stored state
- Revive pattern reconstitutes full object from serialized data
- WebviewPanelSerializer interface implementation

---

## Pattern 8: Custom Editor Provider for Text Document Editing

**Where:** `extensions/markdown-language-features/src/preview/previewManager.ts:246-264`
**What:** Implementing custom editor provider for static markdown preview editing.

```typescript
public async resolveCustomTextEditor(
	document: vscode.TextDocument,
	webview: vscode.WebviewPanel
): Promise<void> {
	const lineNumber = this.#topmostLineMonitor.getPreviousStaticTextEditorLineByUri(document.uri);
	const preview = StaticMarkdownPreview.revive(
		document.uri,
		webview,
		this.#contentProvider,
		this.#previewConfigurations,
		this.#topmostLineMonitor,
		this.#logger,
		this.#contributions,
		this.#opener,
		lineNumber
	);
	this.#registerStaticPreview(preview);
	this.#activePreview = preview;
}
```

**Registration at:** `src/preview/previewManager.ts:102-104`
```typescript
this._register(vscode.window.registerCustomEditorProvider(StaticMarkdownPreview.customEditorViewType, this, {
	webviewOptions: { enableFindWidget: true }
}));
```

**Key aspects:**
- `CustomTextEditorProvider` interface for editor-like webviews
- Receives `vscode.TextDocument` for content synchronization
- Webview panel provided by VS Code
- State restoration via line number monitor
- Enablement of find widget for search/replace

---

## Pattern 9: Event-Driven Synchronization Between Editor and Preview

**Where:** `extensions/markdown-language-features/src/preview/preview.ts:713-733`
**What:** Listening to editor events and propagating changes to webview and vice versa.

```typescript
this._register(vscode.window.onDidChangeTextEditorSelection(event => {
	if (this.#preview.isPreviewOf(event.textEditor.document.uri)) {
		this.#preview.postMessage({
			type: 'onDidChangeTextEditorSelection',
			line: event.selections[0].active.line,
			source: this.#preview.resource.toString()
		});
	}
}));

this._register(vscode.window.onDidChangeActiveTextEditor(editor => {
	// Only allow previewing normal text editors which have a viewColumn: See #101514
	if (typeof editor?.viewColumn === 'undefined') {
		return;
	}

	if (isMarkdownFile(editor.document) && !this.#locked && !this.#preview.isPreviewOf(editor.document.uri)) {
		const line = getVisibleLine(editor);
		this.update(editor.document.uri, line ? new StartingScrollLine(line) : undefined);
	}
}));
```

**Related handlers:**
- Scroll sync at `src/preview/preview.ts:116-120, 308-330`
- Document change at `src/preview/preview.ts:116-126`
- Configuration changes at `src/extension.shared.ts:48-50`

**Key aspects:**
- Event listeners bridge editor and webview state
- Message posting updates preview based on editor changes
- Prevents redundant synchronization with state checks
- Handles file open, active editor change, selection change
- Scroll position bidirectional sync

---

## Pattern 10: Command Registration and Execution Framework

**Where:** `extensions/markdown-language-features/src/commandManager.ts:14-38`
**What:** Centralized command registration system for extension commands.

```typescript
export class CommandManager {
	readonly #commands = new Map<string, vscode.Disposable>();

	public dispose() {
		for (const registration of this.#commands.values()) {
			registration.dispose();
		}
		this.#commands.clear();
	}

	public register<T extends Command>(command: T): vscode.Disposable {
		this.#registerCommand(command.id, command.execute, command);
		return new vscode.Disposable(() => {
			this.#commands.delete(command.id);
		});
	}

	#registerCommand(id: string, impl: (...args: any[]) => void, thisArg?: any) {
		if (this.#commands.has(id)) {
			return;
		}

		this.#commands.set(id, vscode.commands.registerCommand(id, impl, thisArg));
	}
}
```

**Usage in commands:**
- `src/commands/showPreview.ts:67-74` - Preview display commands
- Execution at `src/preview/preview.ts:164, 334`

**Key aspects:**
- Deduplication of command registrations
- Command object interface for type safety
- Centralized disposal management
- Wrapper around `vscode.commands.registerCommand()`

---

## Pattern 11: Language Feature Provider Registration

**Where:** `extensions/markdown-language-features/src/extension.shared.ts:53-68`
**What:** Registering multiple language intelligence providers for markdown.

```typescript
function registerMarkdownLanguageFeatures(
	client: MdLanguageClient,
	commandManager: CommandManager,
	parser: IMdParser,
): vscode.Disposable {
	const selector: vscode.DocumentSelector = markdownLanguageIds;
	return vscode.Disposable.from(
		// Language features
		registerDiagnosticSupport(selector, commandManager),
		registerFindFileReferenceSupport(commandManager, client),
		registerResourceDropOrPasteSupport(selector, parser),
		registerPasteUrlSupport(selector, parser),
		registerUpdateLinksOnRename(client),
		registerUpdatePastedLinks(selector, client),
	);
}
```

**Specific provider registrations:**
- Code actions at `src/languageFeatures/diagnostics.ts:31`
- Document paste at `src/languageFeatures/copyFiles/pasteUrlProvider.ts:83`
- Document drop/paste at `src/languageFeatures/copyFiles/dropOrPasteResource.ts:296-303`

**Key aspects:**
- Document selector for language scoping
- Multiple provider types: diagnostics, code actions, paste/drop edits
- Composable registration via `vscode.Disposable.from()`
- Providers execute synchronously or asynchronously
- Declarative selector for language-specific features

---

## Pattern 12: Configuration Management with Caching

**Where:** `extensions/markdown-language-features/src/preview/previewConfig.ts:75-99`
**What:** Loading and caching configuration per workspace with change detection.

```typescript
export class MarkdownPreviewConfigurationManager {
	readonly #previewConfigurationsForWorkspaces = new Map<string, MarkdownPreviewConfiguration>();

	public loadAndCacheConfiguration(
		resource: vscode.Uri
	): MarkdownPreviewConfiguration {
		const config = MarkdownPreviewConfiguration.getForResource(resource);
		this.#previewConfigurationsForWorkspaces.set(this.#getKey(resource), config);
		return config;
	}

	public hasConfigurationChanged(resource: vscode.Uri): boolean {
		const key = this.#getKey(resource);
		const currentConfig = this.#previewConfigurationsForWorkspaces.get(key);
		const newConfig = MarkdownPreviewConfiguration.getForResource(resource);
		return !currentConfig?.isEqualTo(newConfig);
	}

	#getKey(
		resource: vscode.Uri
	): string {
		const folder = vscode.workspace.getWorkspaceFolder(resource);
		return folder ? folder.uri.toString() : '';
	}
}
```

**Configuration keys accessed:** `src/preview/previewConfig.ts:31-57`
- `editor.scrollBeyondLastLine`, `editor.wordWrap`
- `markdown.preview.*` settings (scrolling, line breaks, linkify, typographer)
- `markdown.styles` (custom CSS)

**Key aspects:**
- Per-workspace caching to minimize repeated reads
- Equality checking to detect changes
- Workspace-relative keying for multi-folder support
- Settings accessed via `vscode.workspace.getConfiguration()`

---

## Summary

These 12 patterns demonstrate the key integration points between VS Code's extension API and Webview/UI layer:

1. **Panel Lifecycle** - Creation, configuration, disposal
2. **Bidirectional Messaging** - Type-safe message contracts with source routing
3. **Resource Transformation** - Converting file URIs for sandboxed webview access
4. **File System Integration** - Watching files and triggering updates
5. **State Persistence** - Serialization/deserialization across sessions
6. **Editor Synchronization** - Two-way event propagation
7. **Command Framework** - Registering and executing IDE commands
8. **Language Features** - Multiple provider types for IDE intelligence
9. **Configuration Management** - Workspace-aware settings caching

**Critical for Tauri migration:**
- Message passing architecture must map to Tauri's invoke/listen system
- Webview panel lifecycle has no direct Tauri equivalent (requires new abstraction)
- File watching can use Tauri's file system watchers
- Resource URIs require different serving mechanism (Tauri asset protocol)
- Configuration system can remain similar with workspace awareness

