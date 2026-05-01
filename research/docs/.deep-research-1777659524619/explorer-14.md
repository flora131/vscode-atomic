# Partition 14 of 79 — Findings

## Scope
`extensions/markdown-language-features/` (86 files, 8,704 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# File Location Index: Markdown Language Features Extension

**Scope**: `extensions/markdown-language-features/` (110 files, ~8,704 LOC)

**Research Question**: Porting VS Code's core IDE functionality (webview-based preview system) from TypeScript/Electron to Tauri/Rust.

**Key Hot Spots Identified**:
- Custom editor provider registration (`registerCustomEditorProvider`)
- MarkdownPreview class (webview-based preview implementation)
- Webview messaging protocol (extension ↔ webview IPC)

---

## Implementation

### Extension Core & Activation
- `extensions/markdown-language-features/src/extension.ts` — Main entry point; activates language server and shared features
- `extensions/markdown-language-features/src/extension.shared.ts` — Shared activation logic across desktop/browser; registers preview manager and language features
- `extensions/markdown-language-features/src/extension.browser.ts` — Browser-specific extension behavior

### Preview System (Primary Porting Hot Spot)
- `extensions/markdown-language-features/src/preview/preview.ts` — **MarkdownPreview class**: Core webview-based preview implementation; handles webview panel lifecycle, message routing, scroll sync, file watching
- `extensions/markdown-language-features/src/preview/previewManager.ts` — **MarkdownPreviewManager**: Manages preview instances; implements `vscode.WebviewPanelSerializer` and `vscode.CustomTextEditorProvider`; registers custom editor (`vscode.markdown.preview.editor`)
- `extensions/markdown-language-features/src/preview/documentRenderer.ts` — **MdDocumentRenderer**: Generates HTML content from markdown; injects CSP, initial data, styling; prepares webview payload
- `extensions/markdown-language-features/src/preview/previewConfig.ts` — Configuration management for preview behavior (scroll sync, font, theme)
- `extensions/markdown-language-features/src/preview/security.ts` — Content Security Policy arbitration for preview rendering
- `extensions/markdown-language-features/src/preview/scrolling.ts` — Scroll synchronization logic between editor and preview
- `extensions/markdown-language-features/src/preview/topmostLineMonitor.ts` — Tracks scroll position across previews

### Webview-Side Code (Client-Side Preview Script)
- `extensions/markdown-language-features/preview-src/index.ts` — **Main webview entry point**: Initializes DOM, handles messages from extension, manages content updates with morphdom
- `extensions/markdown-language-features/preview-src/messaging.ts` — Creates message poster for extension communication; handles `FromWebviewMessage` protocol
- `extensions/markdown-language-features/preview-src/scroll-sync.ts` — Scroll synchronization on webview side
- `extensions/markdown-language-features/preview-src/activeLineMarker.ts` — Visual indicator for active line in preview
- `extensions/markdown-language-features/preview-src/events.ts` — Event helpers (onceDocumentLoaded)
- `extensions/markdown-language-features/preview-src/settings.ts` — Retrieves and caches preview settings from initial data
- `extensions/markdown-language-features/preview-src/loading.ts` — Loading state management
- `extensions/markdown-language-features/preview-src/csp.ts` — Content Security Policy alerts in webview
- `extensions/markdown-language-features/preview-src/strings.ts` — String utilities
- `extensions/markdown-language-features/preview-src/pre.ts` — Pre-processing script

### Markdown Rendering Engine
- `extensions/markdown-language-features/src/markdownEngine.ts` — **MarkdownItEngine**: Wraps markdown-it library; provides parsing and HTML generation
- `extensions/markdown-language-features/src/markdownExtensions.ts` — Plugin/contribution system for markdown parsing

### Command System
- `extensions/markdown-language-features/src/commandManager.ts` — Command registration and dispatch
- `extensions/markdown-language-features/src/commands/index.ts` — Markdown command registration hub
- `extensions/markdown-language-features/src/commands/showPreview.ts` — Open preview in current column
- `extensions/markdown-language-features/src/commands/showPreviewToSide.ts` (implied) — Open preview in side column
- `extensions/markdown-language-features/src/commands/showSource.ts` — Navigate from preview back to source
- `extensions/markdown-language-features/src/commands/refreshPreview.ts` — Manually refresh preview
- `extensions/markdown-language-features/src/commands/toggleLock.ts` — Lock/unlock preview tracking
- `extensions/markdown-language-features/src/commands/reopenAsPreview.ts` — Switch editor to custom editor view
- `extensions/markdown-language-features/src/commands/showPreviewSecuritySelector.ts` — CSP override dialog
- `extensions/markdown-language-features/src/commands/copyImage.ts` — Copy image from preview
- `extensions/markdown-language-features/src/commands/openImage.ts` — Open image file
- `extensions/markdown-language-features/src/commands/reloadPlugins.ts` — Reload markdown extensions
- `extensions/markdown-language-features/src/commands/renderDocument.ts` — API: render markdown to HTML
- `extensions/markdown-language-features/src/commands/insertResource.ts` — API: insert resource references

### Language Server Integration
- `extensions/markdown-language-features/src/client/client.ts` — LanguageClient initialization and configuration
- `extensions/markdown-language-features/src/client/fileWatchingManager.ts` — File watcher for markdown files
- `extensions/markdown-language-features/src/client/protocol.ts` — Protocol definitions for language server
- `extensions/markdown-language-features/src/client/workspace.ts` — Workspace abstraction for language server
- `extensions/markdown-language-features/src/client/inMemoryDocument.ts` — In-memory document representation

### Language Features
- `extensions/markdown-language-features/src/languageFeatures/diagnostics.ts` — Link validation diagnostics
- `extensions/markdown-language-features/src/languageFeatures/fileReferences.ts` — Find file references
- `extensions/markdown-language-features/src/languageFeatures/linkUpdater.ts` — Update links on file rename
- `extensions/markdown-language-features/src/languageFeatures/updateLinksOnPaste.ts` — Auto-fix links when pasting
- `extensions/markdown-language-features/src/languageFeatures/copyFiles/copyFiles.ts` — Handle copy/paste of files
- `extensions/markdown-language-features/src/languageFeatures/copyFiles/dropOrPasteResource.ts` — Drag-drop and paste resources
- `extensions/markdown-language-features/src/languageFeatures/copyFiles/pasteUrlProvider.ts` — Paste URL as markdown link
- `extensions/markdown-language-features/src/languageFeatures/copyFiles/newFilePathGenerator.ts` — Generate paths for copied files
- `extensions/markdown-language-features/src/languageFeatures/copyFiles/snippets.ts` — File-to-markdown snippet generation
- `extensions/markdown-language-features/src/languageFeatures/copyFiles/shared.ts` — Shared copy/paste utilities

### Utilities
- `extensions/markdown-language-features/src/util/resources.ts` — Webview resource URI conversion
- `extensions/markdown-language-features/src/util/openDocumentLink.ts` — Link opening (markdown links in preview)
- `extensions/markdown-language-features/src/util/document.ts` — Document helpers
- `extensions/markdown-language-features/src/util/dom.ts` — DOM utilities (script evaluation)
- `extensions/markdown-language-features/src/util/file.ts` — File type detection
- `extensions/markdown-language-features/src/util/schemes.ts` — URI scheme handling
- `extensions/markdown-language-features/src/util/url.ts` — URL parsing and conversion
- `extensions/markdown-language-features/src/util/uuid.ts` — UUID generation (CSP nonce)
- `extensions/markdown-language-features/src/util/arrays.ts` — Array utilities
- `extensions/markdown-language-features/src/util/dispose.ts` — Disposable pattern utilities
- `extensions/markdown-language-features/src/util/mimes.ts` — MIME type mapping
- `extensions/markdown-language-features/src/util/cancellation.ts` — Cancellation token helpers
- `extensions/markdown-language-features/src/util/resourceMap.ts` — URI-keyed map
- `extensions/markdown-language-features/src/util/async.ts` — Async utilities
- `extensions/markdown-language-features/src/util/uriList.ts` — URI list parsing (drag-drop)

### Logging & Telemetry
- `extensions/markdown-language-features/src/logging.ts` — **ILogger interface**: Output channel logging
- `extensions/markdown-language-features/src/telemetryReporter.ts` — Telemetry event reporting
- `extensions/markdown-language-features/src/slugify.ts` — Heading slug generation

### Notebook Renderer
- `extensions/markdown-language-features/notebook/index.ts` — Notebook markdown renderer entry point

---

## Tests

### Unit Tests
- `extensions/markdown-language-features/src/test/engine.test.ts` — Markdown engine tests
- `extensions/markdown-language-features/src/test/documentLink.test.ts` — Document link resolution tests
- `extensions/markdown-language-features/src/test/urlToUri.test.ts` — URL-to-URI conversion tests
- `extensions/markdown-language-features/src/test/pasteUrl.test.ts` — Paste URL feature tests
- `extensions/markdown-language-features/src/test/copyFile.test.ts` — Copy/paste file handling tests

### Test Utilities
- `extensions/markdown-language-features/src/test/index.ts` — Test runner/setup
- `extensions/markdown-language-features/src/test/engine.ts` — Test markdown engine instance
- `extensions/markdown-language-features/src/test/nulLogging.ts` — No-op logger for tests
- `extensions/markdown-language-features/src/test/util.ts` — Test utilities

---

## Types / Interfaces

### Message Protocol (Critical for Tauri IPC Porting)
- `extensions/markdown-language-features/types/previewMessaging.d.ts` — TypeScript types for webview ↔ extension messaging
  - **FromWebviewMessage**: Messages sent by webview to extension
    - `CacheImageSizes` — Image dimensions for layout
    - `RevealLine` — Scroll position in preview
    - `DidClick` — Click event with line number
    - `ClickLink` — Link click from preview
    - `ShowPreviewSecuritySelector` — CSP override request
    - `PreviewStyleLoadError` — Failed stylesheet load
  - **ToWebviewMessage**: Messages sent by extension to webview
    - `OnDidChangeTextEditorSelection` — Highlight line in preview
    - `UpdateView` — Update view with line
    - `UpdateContent` — Full HTML content update
    - `CopyImageContent` — Copy image action
    - `OpenImageContent` — Open image action

### Type Definitions
- `extensions/markdown-language-features/src/typings/ref.d.ts` — Reference type definitions

---

## Configuration

### Build Configuration
- `extensions/markdown-language-features/tsconfig.json` — TypeScript configuration (main extension)
- `extensions/markdown-language-features/tsconfig.browser.json` — TypeScript configuration (browser/web)
- `extensions/markdown-language-features/preview-src/tsconfig.json` — TypeScript configuration (webview)
- `extensions/markdown-language-features/notebook/tsconfig.json` — TypeScript configuration (notebook renderer)

### Build Scripts (esbuild)
- `extensions/markdown-language-features/esbuild.mts` — Main extension bundling
- `extensions/markdown-language-features/esbuild.webview.mts` — **Webview bundling** (preview-src → media/)
- `extensions/markdown-language-features/esbuild.browser.mts` — Browser-specific bundling
- `extensions/markdown-language-features/esbuild.notebook.mts` — Notebook renderer bundling

### Extension Manifest
- `extensions/markdown-language-features/package.json` — Extension metadata, commands, menus, configuration, activation events
- `extensions/markdown-language-features/package.nls.json` — Localization strings

### JSON Schemas
- `extensions/markdown-language-features/schemas/package.schema.json` — Schema validation

---

## Examples / Fixtures

### Test Workspace
- `extensions/markdown-language-features/test-workspace/a.md`
- `extensions/markdown-language-features/test-workspace/b.md`
- `extensions/markdown-language-features/test-workspace/sub/c.md`
- `extensions/markdown-language-features/test-workspace/sub/d.md`
- `extensions/markdown-language-features/test-workspace/sub/file with space.md`
- `extensions/markdown-language-features/test-workspace/sub with space/file.md`

---

## Documentation

- `extensions/markdown-language-features/README.md` — Extension overview and configuration

---

## Assets

### Styling
- `extensions/markdown-language-features/media/markdown.css` — Preview styling
- `extensions/markdown-language-features/media/highlight.css` — Syntax highlighting (highlight.js)

### Icons
- `extensions/markdown-language-features/media/preview-light.svg` — Light theme icon
- `extensions/markdown-language-features/media/preview-dark.svg` — Dark theme icon

---

## Notable Clusters

### Webview-Based Preview System (Core Porting Area)
The preview system is split across three layers:
1. **Extension Host** (`src/preview/`): Manages webview lifecycle, handles content generation, routes messages
2. **Webview Client** (`preview-src/`): Renders content, syncs scroll, sends user interactions back
3. **Protocol** (`types/previewMessaging.d.ts`): Typed message contract for IPC

### Message-Driven Architecture
Key for Tauri porting: The entire preview ↔ extension communication is message-based:
- Extension sends `ToWebviewMessage` types: content updates, selection changes, image actions
- Webview sends `FromWebviewMessage` types: scroll events, clicks, CSP overrides
- Uses `vscode.postMessage()` on webview side, `webview.postMessage()` on extension side

### Custom Editor Provider Pattern
`MarkdownPreviewManager` implements `vscode.CustomTextEditorProvider`:
- Provides preview when `.md` file opened in custom editor view
- Alternative to dynamic panel previews
- Critical for integrating preview into editor tabs

### Build Separation
Three separate builds:
- **Extension**: `esbuild.mts` → bundles extension code
- **Webview**: `esbuild.webview.mts` → bundles preview-src/ → media/index.js
- **Browser**: `esbuild.browser.mts` → web version of extension

### File Watching & Auto-Refresh
- `FileWatchingManager`: Language server file watcher
- `MarkdownPreview`: Local file system watcher for referenced assets
- Triggers refresh on content or dependency changes

### Scroll Synchronization
Bidirectional scroll sync between editor and preview:
- `TopmostLineMonitor`: Tracks visible line in preview
- `scrolling.ts`: Calculates line-to-scroll-position mapping
- `scroll-sync.ts` (webview): Maps scroll events back to line numbers

### Content Security Policy (CSP)
Security model for preview rendering:
- `ExtensionContentSecurityPolicyArbiter`: Decides CSP level (strict/moderate/allowLocal/allowScripts)
- CSP injected as `<meta>` tag in generated HTML
- User can override via command (`showPreviewSecuritySelector`)

---

## Summary for Tauri/Rust Porting

**Total Extension Files**: 110

**Critical for Porting**:
1. **Preview Manager** (`previewManager.ts`) — Implements editor provider registration and lifecycle
2. **MarkdownPreview Class** (`preview.ts`) — Core webview container, message routing, state management
3. **Message Protocol** (`previewMessaging.d.ts`) — Must be recreated in Rust/Tauri; defines IPC contract
4. **Document Renderer** (`documentRenderer.ts`) — HTML generation pipeline; CSS/CSP injection
5. **Webview Scripts** (`preview-src/`) — Client-side rendering logic; must be ported or integrated into Tauri window

**Key Ports Required**:
- Webview panel lifecycle (create, show, hide, dispose, restore from state)
- Message routing (typed IPC between Tauri backend and webview)
- HTML content generation (markdown → HTML with styling and CSP)
- Scroll synchronization (line-to-scroll mapping in both directions)
- File watching (detect markdown and asset changes)
- Security model (CSP enforcement in webview)

**Estimated Complexity**: High. The webview-based preview is deeply integrated with VS Code's webview API. Tauri's webview differs in architecture (native OS webview vs. Chromium). Message protocol can be reused, but webview lifecycle, resource loading, and security model require careful mapping.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
_(no analysis produced)_

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# VS Code Markdown Language Features - Pattern Analysis

This document analyzes concrete code patterns found in `extensions/markdown-language-features/` (86 files, 8,704 LOC) relevant to porting VS Code's IDE functionality from TypeScript/Electron to Tauri/Rust.

## Overview

The markdown language features extension demonstrates key VS Code architectural patterns:
- Webview-based preview rendering
- Custom editor provider registration
- Language feature registration and management
- Bidirectional messaging between extension and webviews
- Resource handling and URI mapping
- Document lifecycle management

---

## Patterns

#### Pattern: Custom Editor Provider Registration

**Where:** `src/preview/previewManager.ts:102-104`

**What:** Registers a custom text editor provider that handles markdown file previews as native VS Code editors.

```typescript
this._register(vscode.window.registerCustomEditorProvider(StaticMarkdownPreview.customEditorViewType, this, {
	webviewOptions: { enableFindWidget: true }
}));
```

**Variations / call-sites:**
- `src/preview/preview.ts:497` - Defines `StaticMarkdownPreview.customEditorViewType = 'vscode.markdown.preview.editor'`
- `src/preview/previewManager.ts:100` - Also registers webview panel serializer for dynamic previews

**Key aspects:**
- Single registration point in manager constructor
- Implements `vscode.CustomTextEditorProvider` interface (line 72 of previewManager.ts)
- Webview options enable find widget for better UX
- Registration stored via `_register()` for lifecycle management

---

#### Pattern: Preview Class Hierarchy with Composition

**Where:** `src/preview/preview.ts:43-173`

**What:** Core `MarkdownPreview` class that handles rendering logic, scrolling, and event management, composed by both static and dynamic preview types.

```typescript
class MarkdownPreview extends Disposable implements WebviewResourceProvider {

	static readonly #unwatchedImageSchemes = new Set(['https', 'http', 'data']);

	readonly #delay = 300;
	readonly #resource: vscode.Uri;
	readonly #webviewPanel: vscode.WebviewPanel;

	constructor(
		webview: vscode.WebviewPanel,
		resource: vscode.Uri,
		startingScroll: StartingScrollLocation | undefined,
		delegate: MarkdownPreviewDelegate,
		contentProvider: MdDocumentRenderer,
		previewConfigurations: MarkdownPreviewConfigurationManager,
		logger: ILogger,
		contributionProvider: MarkdownContributionProvider,
		opener: MdLinkOpener,
	) {
		super();
		// ... initialization
		this._register(contributionProvider.onContributionsChanged(() => {
			setTimeout(() => this.refresh(true), 0);
		}));

		this._register(vscode.workspace.onDidChangeTextDocument(event => {
			if (this.isPreviewOf(event.document.uri)) {
				this.refresh();
			}
		}));
	}

	public refresh(forceUpdate: boolean = false) {
		if (!this.#throttleTimer) {
			if (this.#firstUpdate) {
				this.#updatePreview(true);
			} else {
				this.#throttleTimer = setTimeout(() => this.#updatePreview(forceUpdate), this.#delay);
			}
		}
		this.#firstUpdate = false;
	}
}
```

**Variations / call-sites:**
- `src/preview/preview.ts:495` - `StaticMarkdownPreview` wraps this class (one preview per file)
- `src/preview/preview.ts:621` - `DynamicMarkdownPreview` wraps this class (switches between files)

**Key aspects:**
- Private fields prevent external mutation
- Event emitters for scroll location tracking
- Debounced refresh with 300ms delay to avoid thrashing
- Delegate pattern for preview-specific behavior
- File watcher management for image invalidation

---

#### Pattern: Static vs Dynamic Preview Types

**Where:** `src/preview/preview.ts:495-612` (StaticMarkdownPreview) and `src/preview/preview.ts:621-830` (DynamicMarkdownPreview)

**What:** Two preview implementations with different lifecycle semantics.

```typescript
export class StaticMarkdownPreview extends Disposable implements IManagedMarkdownPreview {

	public static readonly customEditorViewType = 'vscode.markdown.preview.editor';

	public static revive(
		resource: vscode.Uri,
		webview: vscode.WebviewPanel,
		contentProvider: MdDocumentRenderer,
		previewConfigurations: MarkdownPreviewConfigurationManager,
		topmostLineMonitor: TopmostLineMonitor,
		logger: ILogger,
		contributionProvider: MarkdownContributionProvider,
		opener: MdLinkOpener,
		scrollLine?: number,
	): StaticMarkdownPreview {
		webview.iconPath = contentProvider.iconPath;
		return new StaticMarkdownPreview(webview, resource, contentProvider, previewConfigurations, topmostLineMonitor, logger, contributionProvider, opener, scrollLine);
	}

	private constructor(...) {
		super();
		const topScrollLocation = scrollLine ? new StartingScrollLine(scrollLine) : undefined;
		this.#preview = this._register(new MarkdownPreview(this.#webviewPanel, resource, topScrollLocation, {
			getAdditionalState: () => { return {}; },
			openPreviewLinkToMarkdownFile: (markdownLink, fragment) => {
				return vscode.commands.executeCommand('vscode.openWith', markdownLink.with({
					fragment
				}), StaticMarkdownPreview.customEditorViewType, this.#webviewPanel.viewColumn);
			}
		}, contentProvider, previewConfigurations, logger, contributionProvider, opener));
	}

	public matchesResource(
		_otherResource: vscode.Uri,
		_otherPosition: vscode.ViewColumn | undefined,
		_otherLocked: boolean
	): boolean {
		return false;  // One preview per file
	}
}

export class DynamicMarkdownPreview extends Disposable implements IManagedMarkdownPreview {

	public static readonly viewType = 'markdown.preview';

	public static create(
		input: DynamicPreviewInput,
		previewColumn: vscode.ViewColumn,
		contentProvider: MdDocumentRenderer,
		previewConfigurations: MarkdownPreviewConfigurationManager,
		logger: ILogger,
		topmostLineMonitor: TopmostLineMonitor,
		contributionProvider: MarkdownContributionProvider,
		opener: MdLinkOpener,
	): DynamicMarkdownPreview {
		const webview = vscode.window.createWebviewPanel(
			DynamicMarkdownPreview.viewType,
			DynamicMarkdownPreview.#getPreviewTitle(input.resource, input.locked),
			previewColumn, { enableFindWidget: true, });

		webview.iconPath = contentProvider.iconPath;

		return new DynamicMarkdownPreview(webview, input,
			contentProvider, previewConfigurations, logger, topmostLineMonitor, contributionProvider, opener);
	}

	public matchesResource(
		otherResource: vscode.Uri,
		otherPosition: vscode.ViewColumn | undefined,
		otherLocked: boolean
	): boolean {
		return this.#preview.isPreviewOf(otherResource)
			&& this.#resourceColumn === otherPosition
			&& this.#locked === otherLocked;
	}

	public update(newResource: vscode.Uri, scrollLocation?: StartingScrollLocation) {
		if (this.#preview.isPreviewOf(newResource)) {
			// Handle scroll location if same resource
			return;
		}
		this.#preview.dispose();
		this.#preview = this.#createPreview(newResource, scrollLocation);
	}
}
```

**Key aspects:**
- **StaticMarkdownPreview**: Tied to specific file editor tab (custom editor view type)
- **DynamicMarkdownPreview**: Follows active text editor, can switch between files
- Both implement `IManagedMarkdownPreview` interface for uniform management
- Factory methods (`create`, `revive`) separate construction from initialization
- `matchesResource()` determines preview reuse vs creation of new instance

---

#### Pattern: Webview Panel Creation and Configuration

**Where:** `src/preview/preview.ts:657-660`

**What:** Creating webview panels with specific options and lifecycle handling.

```typescript
const webview = vscode.window.createWebviewPanel(
	DynamicMarkdownPreview.viewType,
	DynamicMarkdownPreview.#getPreviewTitle(input.resource, input.locked),
	previewColumn, { enableFindWidget: true, });
```

**Variations / call-sites:**
- `src/preview/preview.ts:375` - Sets webview HTML after rendering
- `src/preview/preview.ts:228` - Posts messages to webview
- `src/preview/preview.ts:372` - Configures webview options (enableScripts, localResourceRoots)

**Key aspects:**
- View type identifies the preview kind globally
- Title includes resource name and lock state
- Find widget enabled for user search capability
- Webview disposed automatically when tab closed
- HTML content loaded after security policy validation

---

#### Pattern: Bidirectional Webview Messaging

**Where:** `types/previewMessaging.d.ts:1-87` and `src/preview/preview.ts:141-172`

**What:** Typed message contracts for extension↔webview communication.

```typescript
// From Webview to Extension
export namespace FromWebviewMessage {
	export interface CacheImageSizes extends BaseMessage {
		readonly type: 'cacheImageSizes';
		readonly imageData: ReadonlyArray<{ id: string; width: number; height: number }>;
	}

	export interface RevealLine extends BaseMessage {
		readonly type: 'revealLine';
		readonly line: number;
	}

	export interface DidClick extends BaseMessage {
		readonly type: 'didClick';
		readonly line: number;
	}

	export interface ClickLink extends BaseMessage {
		readonly type: 'openLink';
		readonly href: string;
	}

	export type Type =
		| CacheImageSizes
		| RevealLine
		| DidClick
		| ClickLink
		| ShowPreviewSecuritySelector
		| PreviewStyleLoadError
		;
}

// Extension handler
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

**Variations / call-sites:**
- `preview-src/messaging.ts:20-33` - Webview poster that adds source field automatically
- `preview-src/index.ts:336, 365, 386` - Webview sending messages
- `src/preview/preview.ts:226-229` - Extension posting messages to webview

**Key aspects:**
- Discriminated union types for type safety
- Source field prevents cross-resource message confusion
- Async event-driven architecture
- Clear separation of concerns (ExtensionHost vs Webview)

---

#### Pattern: Document Rendering Pipeline

**Where:** `src/preview/documentRenderer.ts:43-128`

**What:** Rendering markdown documents to secure HTML with CSP, theming, and resource mapping.

```typescript
export class MdDocumentRenderer {

	readonly #engine: MarkdownItEngine;
	readonly #context: vscode.ExtensionContext;
	readonly #cspArbiter: ContentSecurityPolicyArbiter;
	readonly #contributionProvider: MarkdownContributionProvider;
	readonly #logger: ILogger;

	constructor(
		engine: MarkdownItEngine,
		context: vscode.ExtensionContext,
		cspArbiter: ContentSecurityPolicyArbiter,
		contributionProvider: MarkdownContributionProvider,
		logger: ILogger
	) {
		this.#engine = engine;
		this.#context = context;
		this.#cspArbiter = cspArbiter;
		this.#contributionProvider = contributionProvider;
		this.#logger = logger;
		this.iconPath = {
			dark: vscode.Uri.joinPath(this.#context.extensionUri, 'media', 'preview-dark.svg'),
			light: vscode.Uri.joinPath(this.#context.extensionUri, 'media', 'preview-light.svg'),
		};
	}

	public async renderDocument(
		markdownDocument: vscode.TextDocument,
		resourceProvider: WebviewResourceProvider,
		previewConfigurations: MarkdownPreviewConfigurationManager,
		initialLine: number | undefined,
		selectedLine: number | undefined,
		state: any | undefined,
		imageInfo: readonly ImageInfo[],
		token: vscode.CancellationToken
	): Promise<MarkdownContentProviderOutput> {
		const sourceUri = markdownDocument.uri;
		const config = previewConfigurations.loadAndCacheConfiguration(sourceUri);
		const initialData = {
			source: sourceUri.toString(),
			fragment: state?.fragment || markdownDocument.uri.fragment || undefined,
			line: initialLine,
			selectedLine,
			scrollPreviewWithEditor: config.scrollPreviewWithEditor,
			scrollEditorWithPreview: config.scrollEditorWithPreview,
			doubleClickToSwitchToEditor: config.doubleClickToSwitchToEditor,
			disableSecurityWarnings: this.#cspArbiter.shouldDisableSecurityWarnings(),
			webviewResourceRoot: resourceProvider.asWebviewUri(markdownDocument.uri).toString(),
		};

		const nonce = generateUuid();
		const csp = this.#getCsp(resourceProvider, sourceUri, nonce);

		const body = await this.renderBody(markdownDocument, resourceProvider);

		const html = `<!DOCTYPE html>
			<html style="${escapeAttribute(this.#getSettingsOverrideStyles(config))}">
			<head>
				<meta http-equiv="Content-type" content="text/html;charset=UTF-8">
				<meta http-equiv="Content-Security-Policy" content="${escapeAttribute(csp)}">
				<meta id="vscode-markdown-preview-data"
					data-settings="${escapeAttribute(JSON.stringify(initialData))}"
					data-strings="${escapeAttribute(JSON.stringify(previewStrings))}"
					data-state="${escapeAttribute(JSON.stringify(state || {}))}"
					data-initial-md-content="${escapeAttribute(body.html)}">
				<script src="${this.#extensionResourcePath(resourceProvider, 'pre.js')}" nonce="${nonce}"></script>
				${this.#getStyles(resourceProvider, sourceUri, config, imageInfo)}
				<base href="${resourceProvider.asWebviewUri(markdownDocument.uri)}">
			</head>
			<body class="vscode-body ${config.scrollBeyondLastLine ? 'scrollBeyondLastLine' : ''} ${config.wordWrap ? 'wordWrap' : ''} ${config.markEditorSelection ? 'showEditorSelection' : ''}">
				${this.#getScripts(resourceProvider, nonce)}
			</body>
			</html>`;
		return {
			html,
			containingImages: body.containingImages,
		};
	}

	public async renderBody(
		markdownDocument: vscode.TextDocument,
		resourceProvider: WebviewResourceProvider,
	): Promise<MarkdownContentProviderOutput> {
		const rendered = await this.#engine.render(markdownDocument, resourceProvider);
		const html = `<div class="markdown-body" dir="auto">${rendered.html}<div class="code-line" data-line="${markdownDocument.lineCount}"></div></div>`;
		return {
			html,
			containingImages: rendered.containingImages
		};
	}
}
```

**Key aspects:**
- Dependency injection of engine, context, and arbiter
- HTML generation with embedded configuration as meta tags
- Content Security Policy enforcement with nonce
- Resource URI mapping for local files
- Image tracking for file watching
- Cancellation token support for async operations

---

#### Pattern: Language Feature Registration

**Where:** `src/extension.shared.ts:53-68`

**What:** Centralized registration of language features for markdown documents.

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

**Variations / call-sites:**
- `src/languageFeatures/diagnostics.ts:125` - Diagnostic support registration
- `src/languageFeatures/copyFiles/dropOrPasteResource.ts:31` - Drop/paste handler registration

**Key aspects:**
- Document selector identifies target files (.md, .markdown)
- Composable feature registration using Disposable.from()
- Language server client integration
- Parser access for document analysis

---

#### Pattern: Code Action Provider with Commands

**Where:** `src/languageFeatures/diagnostics.ts:20-73`

**What:** Quick fix provider that registers commands and handles code actions.

```typescript
class AddToIgnoreLinksQuickFixProvider implements vscode.CodeActionProvider {

	static readonly #addToIgnoreLinksCommandId = '_markdown.addToIgnoreLinks';

	static readonly #metadata: vscode.CodeActionProviderMetadata = {
		providedCodeActionKinds: [
			vscode.CodeActionKind.QuickFix
		],
	};

	public static register(selector: vscode.DocumentSelector, commandManager: CommandManager): vscode.Disposable {
		const reg = vscode.languages.registerCodeActionsProvider(selector, new AddToIgnoreLinksQuickFixProvider(), AddToIgnoreLinksQuickFixProvider.#metadata);
		const commandReg = commandManager.register({
			id: AddToIgnoreLinksQuickFixProvider.#addToIgnoreLinksCommandId,
			execute(resource: vscode.Uri, path: string) {
				const settingId = 'validate.ignoredLinks';
				const config = vscode.workspace.getConfiguration('markdown', resource);
				const paths = new Set(config.get<string[]>(settingId, []));
				paths.add(path);
				config.update(settingId, [...paths], vscode.ConfigurationTarget.WorkspaceFolder);
			}
		});
		return vscode.Disposable.from(reg, commandReg);
	}

	provideCodeActions(document: vscode.TextDocument, _range: vscode.Range | vscode.Selection, context: vscode.CodeActionContext, _token: vscode.CancellationToken): vscode.ProviderResult<(vscode.CodeAction | vscode.Command)[]> {
		const fixes: vscode.CodeAction[] = [];

		for (const diagnostic of context.diagnostics) {
			switch (diagnostic.code) {
				case DiagnosticCode.link_noSuchReferences:
				case DiagnosticCode.link_noSuchHeaderInOwnFile:
				case DiagnosticCode.link_noSuchFile:
				case DiagnosticCode.link_noSuchHeaderInFile: {
					const hrefText = (diagnostic as unknown as Record<string, any>).data?.hrefText;
					if (hrefText) {
						const fix = new vscode.CodeAction(
							vscode.l10n.t("Exclude '{0}' from link validation.", hrefText),
							vscode.CodeActionKind.QuickFix);

						fix.command = {
							command: AddToIgnoreLinksQuickFixProvider.#addToIgnoreLinksCommandId,
							title: '',
							arguments: [document.uri, hrefText],
						};
						fixes.push(fix);
					}
					break;
				}
			}
		}

		return fixes;
	}
}
```

**Key aspects:**
- Meta-declaration of supported code action kinds
- Factory pattern for registration and disposal
- Commands registered separately from provider
- Access to workspace configuration for settings
- Scope-aware configuration (workspace folder level)

---

#### Pattern: Drop/Paste Edit Providers

**Where:** `src/languageFeatures/copyFiles/dropOrPasteResource.ts:31-95`

**What:** Unified handler for both drop and paste operations with data transfer.

```typescript
class ResourcePasteOrDropProvider implements vscode.DocumentPasteEditProvider, vscode.DocumentDropEditProvider {

	public static readonly mimeTypes = [
		Mime.textUriList,
		'files',
		...Object.values(rootMediaMimesTypes).map(type => `${type}/*`),
	];

	readonly #yieldTo = [
		vscode.DocumentDropOrPasteEditKind.Text,
		vscode.DocumentDropOrPasteEditKind.Empty.append('markdown', 'link', 'image', 'attachment'),
	];

	readonly #parser: IMdParser;

	constructor(parser: IMdParser) {
		this.#parser = parser;
	}

	public async provideDocumentDropEdits(
		document: vscode.TextDocument,
		position: vscode.Position,
		dataTransfer: vscode.DataTransfer,
		token: vscode.CancellationToken,
	): Promise<vscode.DocumentDropEdit | undefined> {
		const edit = await this.#createEdit(document, [new vscode.Range(position, position)], dataTransfer, {
			insert: this.#getEnabled(document, 'editor.drop.enabled'),
			copyIntoWorkspace: vscode.workspace.getConfiguration('markdown', document).get<CopyFilesSettings>('editor.drop.copyIntoWorkspace', CopyFilesSettings.MediaFiles)
		}, undefined, token);

		if (!edit || token.isCancellationRequested) {
			return;
		}

		const dropEdit = new vscode.DocumentDropEdit(edit.snippet);
		dropEdit.title = edit.label;
		dropEdit.kind = edit.kind;
		dropEdit.additionalEdit = edit.additionalEdits;
		dropEdit.yieldTo = [...this.#yieldTo, ...edit.yieldTo];
		return dropEdit;
	}

	public async provideDocumentPasteEdits(
		document: vscode.TextDocument,
		ranges: readonly vscode.Range[],
		dataTransfer: vscode.DataTransfer,
		context: vscode.DocumentPasteEditContext,
		token: vscode.CancellationToken,
	): Promise<vscode.DocumentPasteEdit[] | undefined> {
		const edit = await this.#createEdit(document, ranges, dataTransfer, {
			insert: this.#getEnabled(document, 'editor.paste.enabled'),
			copyIntoWorkspace: vscode.workspace.getConfiguration('markdown', document).get<CopyFilesSettings>('editor.paste.copyIntoWorkspace', CopyFilesSettings.MediaFiles)
		}, context, token);

		if (!edit || token.isCancellationRequested) {
			return;
		}

		const pasteEdit = new vscode.DocumentPasteEdit(edit.snippet, edit.label, edit.kind);
		pasteEdit.additionalEdit = edit.additionalEdits;
		pasteEdit.yieldTo = [...this.#yieldTo, ...edit.yieldTo];
		return [pasteEdit];
	}
}
```

**Key aspects:**
- Single class handles both drop and paste operations
- MIME type enumeration for supported data
- `yieldTo` preference ordering for competing edits
- Configuration-driven behavior (insert strategy, copy vs link)
- Cancellation token support for async file operations

---

#### Pattern: Extension Lifecycle and Activation

**Where:** `src/extension.ts:15-27`

**What:** Extension activation entry point with dependency setup.

```typescript
export async function activate(context: vscode.ExtensionContext) {
	const contributions = getMarkdownExtensionContributions(context);
	context.subscriptions.push(contributions);

	const logger = new VsCodeOutputLogger();
	context.subscriptions.push(logger);

	const engine = new MarkdownItEngine(contributions, githubSlugifier, logger);

	const client = await startServer(context, engine);
	context.subscriptions.push(client);
	activateShared(context, client, engine, logger, contributions);
}
```

**Variations / call-sites:**
- `src/extension.shared.ts:26-51` - Shared activation logic
- `src/extension.ts:29-56` - Language server startup

**Key aspects:**
- Contributions loaded first (enables dynamic features)
- Logger created and pushed to subscriptions
- Engine instantiated with logger
- Language server started asynchronously
- All disposables tracked in context.subscriptions

---

## Architectural Insights

### Key Patterns for Tauri/Rust Porting

1. **Custom Editor Types**: VS Code's custom editor system requires per-document state management. A Tauri equivalent would need per-window or per-document state handling in Rust.

2. **Webview IPC Pattern**: The typed messaging system (FromWebviewMessage, ToWebviewMessage) would map cleanly to Tauri's invoke/listen patterns with serde serialization.

3. **Disposable Pattern**: The hierarchical resource disposal using `_register()` chains would require explicit Drop implementation or Arc<Mutex<>> patterns in Rust.

4. **Provider Registration**: Language features registered via callbacks would need dependency injection or service registry patterns in Rust.

5. **URI-based Resource Identity**: Resources tracked by VS Code URI (file:///, vscode://) would need URI type safety in Rust (e.g., `std::path::PathBuf` or dedicated URI type).

6. **Async Rendering**: The async rendering pipeline with cancellation tokens would map to Rust async/await with cancellation support via tokio::sync::CancellationToken.

7. **Configuration Management**: Workspace-scoped configuration retrieval would require a configuration service with scope awareness (workspace folder, global, etc.).

### Critical Extension Points

- **Custom Editor Provider**: `registerCustomEditorProvider()` - Single entry point for editor integration
- **Webview Serialization**: `registerWebviewPanelSerializer()` - Persistence of preview state across sessions
- **Language Features**: Multiple provider types (CodeActions, DropEdit, PasteEdit, Diagnostics)
- **File System Watching**: Image file changes trigger preview refresh
- **URI Schemes**: Support for multiple URI schemes (file, vscode, http, https)

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
