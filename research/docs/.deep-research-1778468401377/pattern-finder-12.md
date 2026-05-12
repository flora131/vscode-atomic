# Partition 12: Language Feature Registration Patterns in markdown-language-features

Research question: What APIs does VS Code's core IDE require to support language-specific features like editing, language intelligence, diagnostics, commands, and workspace integration?

## Key Finding

The markdown-language-features extension demonstrates that VS Code's extensibility is built on **language registration APIs** that a Rust/Tauri port must replicate to achieve feature parity. These patterns show how language services connect to:
- Code actions and quick fixes
- Paste/drop edit providers  
- Document paste/drop editing
- Language status items
- Commands with telemetry
- Workspace file events
- Configuration management
- Language client protocol (LSP)

---

## Pattern Examples

### Pattern 1: Code Actions Provider Registration

**Where:** `extensions/markdown-language-features/src/languageFeatures/diagnostics.ts:31`

**What:** Registers a code action provider that supplies quick fixes for diagnosed issues in the document.

```typescript
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
```

**Variations / call-sites:** `extension.shared.ts:61` (registered via `registerDiagnosticSupport`).

**Core APIs needed:**
- `vscode.languages.registerCodeActionsProvider(selector, provider, metadata)`
- `CodeActionProviderMetadata` with `providedCodeActionKinds`
- `vscode.CodeAction` class with `kind` and `command` fields
- `vscode.workspace.getConfiguration()` and `config.update()`
- `vscode.ConfigurationTarget` enum

---

### Pattern 2: Document Paste Edit Provider Registration

**Where:** `extensions/markdown-language-features/src/languageFeatures/copyFiles/pasteUrlProvider.ts:82-87`

**What:** Registers a provider that intercepts paste operations and offers document edits.

```typescript
export function registerPasteUrlSupport(selector: vscode.DocumentSelector, parser: IMdParser) {
    return vscode.languages.registerDocumentPasteEditProvider(selector, new PasteUrlEditProvider(parser), {
        providedPasteEditKinds: [PasteUrlEditProvider.kind],
        pasteMimeTypes: PasteUrlEditProvider.pasteMimeTypes,
    });
}
```

**Variations / call-sites:** 
- `dropOrPasteResource.ts:296` — `registerDocumentPasteEditProvider` with `ResourcePasteOrDropProvider`
- `updateLinksOnPaste.ts:86` — `registerDocumentPasteEditProvider` with `UpdatePastedLinksEditProvider`

**Core APIs needed:**
- `vscode.languages.registerDocumentPasteEditProvider(selector, provider, options)`
- `DocumentPasteEditProvider` interface with `provideDocumentPasteEdits()` method
- `DocumentPasteEdit` class with `snippet`, `label`, `kind`, `additionalEdit`, `yieldTo`
- `DocumentPasteEditContext` with `only` filter
- `DataTransfer` for clipboard mime types
- `WorkspaceEdit` for additional edits

---

### Pattern 3: Document Drop Edit Provider Registration

**Where:** `extensions/markdown-language-features/src/languageFeatures/copyFiles/dropOrPasteResource.ts:300-304`

**What:** Registers a provider for drag-and-drop operations into the editor.

```typescript
return vscode.Disposable.from(
    vscode.languages.registerDocumentPasteEditProvider(selector, new ResourcePasteOrDropProvider(parser), {
        providedPasteEditKinds: providedEditKinds,
        pasteMimeTypes: ResourcePasteOrDropProvider.mimeTypes,
    }),
    vscode.languages.registerDocumentDropEditProvider(selector, new ResourcePasteOrDropProvider(parser), {
        providedDropEditKinds: providedEditKinds,
        dropMimeTypes: ResourcePasteOrDropProvider.mimeTypes,
    }),
);
```

**Variations / call-sites:** `dropOrPasteResource.ts:52-73` implements `DocumentDropEditProvider.provideDocumentDropEdits()`.

**Core APIs needed:**
- `vscode.languages.registerDocumentDropEditProvider(selector, provider, options)`
- `DocumentDropEditProvider` interface with `provideDocumentDropEdits()` method
- `DocumentDropEdit` class with `snippet`, `title`, `kind`, `additionalEdit`, `yieldTo`
- `DataTransfer` for mime type inspection
- Media/file detection from data transfer

---

### Pattern 4: Language Status Item Registration

**Where:** `extensions/markdown-language-features/src/languageFeatures/diagnostics.ts:76-123`

**What:** Creates a language-specific status bar item that reflects current validation state.

```typescript
function registerMarkdownStatusItem(selector: vscode.DocumentSelector, commandManager: CommandManager): vscode.Disposable {
    const statusItem = vscode.languages.createLanguageStatusItem('markdownStatus', selector);

    const enabledSettingId = 'validate.enabled';
    const commandId = '_markdown.toggleValidation';

    const commandSub = commandManager.register({
        id: commandId,
        execute: (enabled: boolean) => {
            vscode.workspace.getConfiguration('markdown').update(enabledSettingId, enabled);
        }
    });

    const update = () => {
        const activeDoc = vscode.window.activeTextEditor?.document;
        const markdownDoc = activeDoc && isMarkdownFile(activeDoc) ? activeDoc : undefined;

        const enabled = vscode.workspace.getConfiguration('markdown', markdownDoc).get(enabledSettingId);
        if (enabled) {
            statusItem.text = vscode.l10n.t('Markdown link validation enabled');
            statusItem.command = {
                command: commandId,
                arguments: [false],
                title: vscode.l10n.t('Disable'),
                tooltip: vscode.l10n.t('Disable validation of Markdown links'),
            };
        }
    };
    update();
    // ... event subscriptions to keep statusItem.command updated
}
```

**Variations / call-sites:** `extension.shared.ts:61` (registered via `registerDiagnosticSupport`).

**Core APIs needed:**
- `vscode.languages.createLanguageStatusItem(id, selector)`
- `LanguageStatusItem` with `text`, `command`, `severity`, `detail` properties
- `vscode.l10n.t()` for localization
- Configuration management to read/update settings
- Window event subscriptions for state updates

---

### Pattern 5: File Rename Event Handler with Workspace Edit Application

**Where:** `extensions/markdown-language-features/src/languageFeatures/linkUpdater.ts:34-62`

**What:** Listens for file rename events and applies bulk workspace edits via LSP.

```typescript
this._register(vscode.workspace.onDidRenameFiles(async (e) => {
    await Promise.all(e.files.map(async (rename) => {
        if (await this.#shouldParticipateInLinkUpdate(rename.newUri)) {
            this.#pendingRenames.add(rename);
        }
    }));

    if (this.#pendingRenames.size) {
        this.#delayer.trigger(() => {
            vscode.window.withProgress({
                location: vscode.ProgressLocation.Window,
                title: vscode.l10n.t("Checking for Markdown links to update")
            }, () => this.#flushRenames());
        });
    }
}));
```

And then applies edits:

```typescript
async #flushRenames(): Promise<void> {
    const renames = Array.from(this.#pendingRenames);
    this.#pendingRenames.clear();

    const result = await this.#getEditsForFileRename(renames, noopToken);

    if (result?.edit.size) {
        if (await this.#confirmActionWithUser(result.resourcesBeingRenamed)) {
            await vscode.workspace.applyEdit(result.edit);
        }
    }
}
```

**Variations / call-sites:** `linkUpdater.ts:46-61` shows full event subscription pattern.

**Core APIs needed:**
- `vscode.workspace.onDidRenameFiles()` event
- `FileRenameEvent` with `files` array
- `vscode.workspace.applyEdit(WorkspaceEdit)`
- `vscode.window.withProgress()` for long-running operations
- `vscode.window.showInformationMessage()` for user confirmation
- LSP client integration via `MdLanguageClient.getEditForFileRenames()`

---

### Pattern 6: Command Manager with Telemetry

**Where:** `extensions/markdown-language-features/src/commands/showPreview.ts:53-75`

**What:** Implements command pattern that reports telemetry on execution.

```typescript
export class ShowPreviewCommand implements Command {
    public readonly id = 'markdown.showPreview';

    readonly #webviewManager: MarkdownPreviewManager;
    readonly #telemetryReporter: TelemetryReporter;

    public constructor(
        webviewManager: MarkdownPreviewManager,
        telemetryReporter: TelemetryReporter
    ) {
        this.#webviewManager = webviewManager;
        this.#telemetryReporter = telemetryReporter;
    }

    public execute(mainUri?: vscode.Uri, allUris?: vscode.Uri[], previewSettings?: DynamicPreviewSettings) {
        for (const uri of Array.isArray(allUris) ? allUris : [mainUri]) {
            showPreview(this.#webviewManager, this.#telemetryReporter, uri, {
                sideBySide: false,
                locked: previewSettings?.locked
            });
        }
    }
}
```

**Variations / call-sites:** `commandManager.ts:24-37` shows registration wrapper.

**Core APIs needed:**
- `vscode.commands.registerCommand(id, impl, thisArg)`
- `vscode.commands.executeCommand()` for invoking other commands
- Telemetry reporter integration
- URI and configuration handling

---

### Pattern 7: Language Client Protocol Integration

**Where:** `extensions/markdown-language-features/src/client/client.ts:64-102`

**What:** Initializes language client with document selector, file watchers, and sync options.

```typescript
const clientOptions: LanguageClientOptions = {
    documentSelector: markdownLanguageIds,
    synchronize: {
        configurationSection: ['markdown'],
        fileEvents: vscode.workspace.createFileSystemWatcher(mdFileGlob),
    },
    initializationOptions: {
        markdownFileExtensions,
        i10lLocation: vscode.l10n.uri?.toJSON(),
    },
    diagnosticPullOptions: {
        onChange: true,
        onTabs: true,
        match(_documentSelector, resource) {
            return looksLikeMarkdownPath(resource);
        },
    },
    markdown: {
        supportHtml: true,
    }
};

const client = factory('markdown', vscode.l10n.t("Markdown Language Server"), clientOptions);
```

**Variations / call-sites:** `extension.ts:29-56` shows server startup with debug options.

**Core APIs needed:**
- `LanguageClientOptions` configuration object
- `vscode.workspace.createFileSystemWatcher()` for monitoring files
- `DocumentSelector` based on language IDs
- Diagnostic pull/push synchronization options
- NotebookDocumentSyncRegistration for notebook support

---

## Summary

The markdown-language-features extension demonstrates that porting VS Code to Tauri/Rust requires implementing these critical subsystems:

1. **Provider Registration APIs**: Code actions, document edit providers (paste/drop), language status items
2. **Event/Subscription System**: File system watches, workspace events (rename, configuration changes), window/editor events
3. **Configuration Management**: Per-resource configuration with scoped updates (global/workspace/folder)
4. **Data Transfer & Edit Orchestration**: DataTransfer protocol for clipboard/drag-drop, WorkspaceEdit for bulk edits
5. **Language Client Protocol**: LSP initialization, document synchronization, diagnostic collection
6. **Command System**: Command registration, execution, and telemetry reporting
7. **Localization**: Built-in i18n via `vscode.l10n.t()`

Each pattern shows VS Code's extensibility model: providers implement interfaces, register themselves with document selectors, and communicate via events and LSP. A Rust port must replicate this contract-based architecture.

