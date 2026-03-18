# VS Code Extension API Research: QuickPick, Terminal, Activation & Commands

**Date**: 2026-03-18
**Source**: DeepWiki (microsoft/vscode repository)
**Purpose**: Research findings for building a VS Code extension that uses QuickPick for menu selection and Terminal API for spawning CLI processes.

---

## Table of Contents

1. [QuickPick API](#1-quickpick-api)
2. [Terminal API](#2-terminal-api)
3. [Extension Activation](#3-extension-activation)
4. [Command Registration](#4-command-registration)

---

## 1. QuickPick API

The VS Code extension API provides two primary ways to interact with QuickPick menus: `vscode.window.showQuickPick` (simplified) and `vscode.window.createQuickPick` (advanced).

### 1.1 `vscode.window.showQuickPick`

A simplified function that displays a QuickPick and returns a `Promise` resolving to the selected item(s) or `undefined` if dismissed. Accepts an array of `QuickPickItem` objects (or strings) and optional `QuickPickOptions`.

**Usage Pattern:**

```typescript
// Simple string list
window.showQuickPick(['eins', 'zwei', 'drei']).then(selection => {
    if (selection) {
        console.log(`Selected: ${selection}`);
    }
});

// With QuickPickItem objects and multi-selection
window.showQuickPick(items, { canPickMany: true });
```

### 1.2 `vscode.window.createQuickPick`

Returns a `QuickPick` object providing advanced control: dynamic updates, event handling, and multi-step input scenarios. You instantiate the object, set properties, register event listeners, then call `show()`.

**Usage Pattern:**

```typescript
const quickPick = window.createQuickPick();
quickPick.items = [{ label: 'Item 1' }, { label: 'Item 2' }];
quickPick.placeholder = 'Select an item';
quickPick.onDidAccept(() => {
    const selection = quickPick.selectedItems[0];
    console.log(`Selected: ${selection.label}`);
    quickPick.dispose();
});
quickPick.show();
```

### 1.3 `QuickPickItem` Interface

```typescript
export interface QuickPickItem {
    label: string;
    kind?: QuickPickItemKind;
    iconPath?: IconPath;
    description?: string;
    detail?: string;
    resourceUri?: Uri;
    picked?: boolean;
    alwaysShow?: boolean;
    buttons?: readonly QuickInputButton[];
    tooltip?: string | MarkdownString; // Proposed API
}
```

**Property descriptions:**

| Property | Type | Description |
|---|---|---|
| `label` | `string` | Primary human-readable string, prominently rendered. Supports theme icons. |
| `kind` | `QuickPickItemKind` | Rendering type. `Default` for selectable items, `Separator` for visual grouping. |
| `iconPath` | `IconPath` | Optional icon for the item. |
| `description` | `string` | Less prominent string on the same line as the label. |
| `detail` | `string` | Less prominent string on a separate line below. |
| `resourceUri` | `Uri` | Used to derive label, description, and icon if not explicitly provided. |
| `picked` | `boolean` | Initially selected flag (honored when `canPickMany` is true). |
| `alwaysShow` | `boolean` | Keeps item visible even when filtered out. |
| `buttons` | `readonly QuickInputButton[]` | Buttons rendered on the item, triggering `QuickPickItemButtonEvent`. |
| `tooltip` | `string \| MarkdownString` | Tooltip on hover (proposed API, may require opt-in). |

### 1.4 `QuickPickOptions` Interface

```typescript
export interface QuickPickOptions {
    title?: string;
    matchOnDescription?: boolean;
    matchOnDetail?: boolean;
    placeHolder?: string;
    prompt?: string;
    ignoreFocusOut?: boolean;
    canPickMany?: boolean;
    onDidSelectItem?(item: QuickPickItem | string): any;
}
```

| Property | Type | Description |
|---|---|---|
| `title` | `string` | Optional title displayed at the top. |
| `matchOnDescription` | `boolean` | Include description when filtering. |
| `matchOnDetail` | `boolean` | Include detail when filtering. |
| `placeHolder` | `string` | Placeholder text in the input box. |
| `prompt` | `string` | Instructions displayed below the input box. |
| `ignoreFocusOut` | `boolean` | Keep picker open when focus is lost. |
| `canPickMany` | `boolean` | Allow multiple selections (returns array). |
| `onDidSelectItem` | `function` | Callback invoked when an item is focused/selected. |

### 1.5 `QuickPick<T>` Object (from `createQuickPick`)

```typescript
export interface QuickPick<T extends QuickPickItem> extends QuickInput {
    readonly type: QuickInputType.QuickPick;
    value: string;
    filterValue: (value: string) => string;
    ariaLabel: string | undefined;
    placeholder: string | undefined;
    prompt: string | undefined;
    readonly onDidChangeValue: Event<string>;
    readonly onWillAccept: Event<IQuickPickWillAcceptEvent>;
    readonly onDidAccept: Event<IQuickPickDidAcceptEvent>;
    canAcceptInBackground: boolean;
    ok: boolean | 'default';
    okLabel: string | undefined;
    readonly onDidCustom: Event<void>;
    customButton: boolean;
    customLabel: string | undefined;
    customHover: string | undefined;
    customButtonSecondary?: boolean;
    readonly onDidTriggerItemButton: Event<IQuickPickItemButtonEvent<T>>;
    readonly onDidTriggerSeparatorButton: Event<IQuickPickSeparatorButtonEvent>;
    items: ReadonlyArray<T | IQuickPickSeparator>;
    canSelectMany: boolean;
    matchOnDescription: boolean;
    matchOnDetail: boolean;
    matchOnLabel: boolean;
    sortByLabel: boolean; // Proposed API
}
```

**Key events:**

- `onDidChangeValue` -- fired when input text changes
- `onWillAccept` -- fired before accepting (can veto)
- `onDidAccept` -- fired when user accepts selection
- `onDidTriggerItemButton` -- fired when an item button is clicked
- `onDidTriggerSeparatorButton` -- fired when a separator button is clicked

### 1.6 Multi-Step QuickPick Wizards

Multi-step wizards reuse a single `IQuickPick` instance, updating its properties (`items`, `placeholder`, `title`, `buttons`, `step`, `totalSteps`) in a loop based on the current wizard step.

**Key properties for multi-step:**

- `step: number | undefined` -- Current step number, displayed in the title bar.
- `totalSteps: number | undefined` -- Total number of steps, displayed alongside `step` (e.g. "1/3").
- `buttons: IQuickInputButton[]` -- Add back button via `quickInputService.backButton`.

**Back button handling pattern:**

1. Detect if the triggered button is the `backButton`.
2. Revert the `step` counter and restore the state of the previous step.

**Pattern from built-in extensions (Chat hooks, MCP elicitation):**

```typescript
// Pseudocode pattern from VS Code built-in extensions
const picker = quickInputService.createQuickPick();
picker.totalSteps = totalSteps;
const stepHistory: Step[] = [];

while (true) {
    picker.step = currentStep;
    picker.items = getItemsForStep(currentStep);
    picker.buttons = currentStep > 1 ? [backButton] : [];

    const result = await awaitPick(picker);

    if (result === 'back') {
        currentStep = stepHistory.pop();
    } else {
        stepHistory.push(currentStep);
        currentStep = nextStep;
    }
}
```

**Real examples in the VS Code codebase:**

- `src/vs/workbench/contrib/chat/browser/promptSyntax/hookActions.ts` -- `showConfigureHooksQuickPick` uses `enum Step`, `while(true)` loop, `stepHistory` array and `goBack` function
- `src/vs/workbench/contrib/mcp/browser/mcpElicitationService.ts` -- `_doElicitForm` uses `backSnapshots` to store/restore state per step
- `src/vs/workbench/contrib/mcp/browser/mcpPromptArgumentPick.ts` -- `McpPromptArgumentPick` class with `backSnapshots`

### 1.7 Source File References

| File | Role |
|---|---|
| `src/vs/workbench/api/common/extHostQuickOpen.ts` | Extension host implementation of `showQuickPick` and `createQuickPick` |
| `src/vs/workbench/api/browser/mainThreadQuickOpen.ts` | Main thread side that interfaces with `IQuickInputService` |
| `src/vs/platform/quickinput/common/quickInput.ts` | Core `IQuickPick`, `IQuickInput`, `IQuickInputService` interfaces |
| `src/vs/workbench/api/common/extHost.protocol.ts` | RPC protocol definitions for extension host <-> main thread |
| `src/vs/vscode.proposed.quickPickItemTooltip.d.ts` | Proposed API for `tooltip` on QuickPickItem |
| `src/vs/workbench/contrib/chat/browser/promptSyntax/hookActions.ts` | Multi-step QuickPick example |
| `src/vs/workbench/contrib/mcp/browser/mcpElicitationService.ts` | Multi-step QuickPick example |
| `src/vs/workbench/contrib/mcp/browser/mcpPromptArgumentPick.ts` | Multi-step QuickPick example |

---

## 2. Terminal API

The VS Code Terminal API allows extensions to create and manage integrated terminals programmatically. Extensions interact with terminals primarily through `vscode.window.createTerminal`.

### 2.1 `Terminal` Interface

```typescript
export interface Terminal {
    readonly name: string;
    readonly processId: Thenable<number | undefined>;
    readonly creationOptions: Readonly<TerminalOptions | ExtensionTerminalOptions>;
    readonly exitStatus: TerminalExitStatus | undefined;
    readonly state: TerminalState;
    readonly shellIntegration: TerminalShellIntegration | undefined;
    sendText(text: string, shouldExecute?: boolean): void;
    show(preserveFocus?: boolean): void;
    hide(): void;
    dispose(): void;
}
```

**Key members:**

| Member | Type | Description |
|---|---|---|
| `name` | `string` | Human-readable terminal name |
| `processId` | `Thenable<number \| undefined>` | PID of the shell process |
| `creationOptions` | `TerminalOptions \| ExtensionTerminalOptions` | Read-only options used to create the terminal |
| `exitStatus` | `TerminalExitStatus \| undefined` | Exit status after terminal closes |
| `state` | `TerminalState` | Current terminal state |
| `shellIntegration` | `TerminalShellIntegration \| undefined` | Shell integration info |
| `sendText(text, shouldExecute?)` | method | Send text to terminal stdin. `shouldExecute` defaults to `true` (appends newline). |
| `show(preserveFocus?)` | method | Show the terminal panel. `preserveFocus` prevents stealing focus. |
| `hide()` | method | Hide the terminal panel. |
| `dispose()` | method | Destroy the terminal. |

### 2.2 `vscode.window.createTerminal` Overloads

```typescript
// Overload 1: Simple creation
createTerminal(name?: string, shellPath?: string, shellArgs?: readonly string[] | string): Terminal;

// Overload 2: Full options
createTerminal(options: TerminalOptions): Terminal;

// Overload 3: Extension-controlled terminal (Pseudoterminal)
createTerminal(options: ExtensionTerminalOptions): Terminal;
```

### 2.3 `TerminalOptions` Interface

```typescript
export interface TerminalOptions {
    name?: string;
    shellPath?: string;
    shellArgs?: string[] | string;
    cwd?: string | Uri;
    env?: { [key: string]: string | null | undefined };
    hideFromUser?: boolean;
    titleTemplate?: string; // Proposed API
}
```

| Property | Type | Description |
|---|---|---|
| `name` | `string` | Human-readable name for the terminal tab |
| `shellPath` | `string` | Path to the shell executable (e.g. `/bin/bash`, `cmd.exe`) |
| `shellArgs` | `string[] \| string` | Command line arguments for the shell |
| `cwd` | `string \| Uri` | Working directory for the terminal |
| `env` | `object` | Environment variables added to the terminal. `null` deletes a variable. |
| `hideFromUser` | `boolean` | If true, terminal is not shown in the UI |
| `titleTemplate` | `string` | Title template string (proposed API) |

### 2.4 `ExtensionTerminalOptions` Interface

```typescript
export interface ExtensionTerminalOptions {
    name?: string;
    pty: Pseudoterminal;
    titleTemplate?: string; // Proposed API
}
```

Used for creating terminals fully controlled by the extension via a `Pseudoterminal`.

### 2.5 `Pseudoterminal` Interface

```typescript
export interface Pseudoterminal {
    onDidWrite: Event<string>;
    onDidClose?: Event<number | void>;
    onDidOverrideDimensions?: Event<TerminalDimensions | undefined>;
    onDidChangeName?: Event<string>;
    open(initialDimensions?: TerminalDimensions): void;
    close(): void;
    handleInput?(data: string): void;
    setDimensions?(dimensions: TerminalDimensions): void;
}
```

| Member | Type | Description |
|---|---|---|
| `onDidWrite` | `Event<string>` | Fires when the pty writes output data |
| `onDidClose` | `Event<number \| void>` | Fires when the pty is closed (optional exit code) |
| `onDidOverrideDimensions` | `Event<TerminalDimensions \| undefined>` | Fires when dimensions are overridden |
| `onDidChangeName` | `Event<string>` | Fires when the terminal name changes |
| `open(initialDimensions?)` | method | Called when the terminal is opened/ready |
| `close()` | method | Called when the terminal is closed/disposed |
| `handleInput?(data)` | method | Called when user types input |
| `setDimensions?(dimensions)` | method | Called when terminal dimensions change |

### 2.6 Terminal Event Handlers

```typescript
// Fires when a terminal is opened
vscode.window.onDidOpenTerminal: Event<Terminal>;

// Fires when a terminal is closed
vscode.window.onDidCloseTerminal: Event<Terminal>;
```

### 2.7 `sendText` Method -- Key Usage Pattern

The `sendText` method sends text to the terminal's stdin. By default, a newline is appended (simulating Enter), which causes the shell to execute the text as a command.

```typescript
const terminal = vscode.window.createTerminal('My Terminal');
terminal.show();

// Execute a command (shouldExecute defaults to true, appends newline)
terminal.sendText('npm install');

// Send text without executing (no newline appended)
terminal.sendText('partial command', false);
```

### 2.8 Terminal Profiles

Extensions can contribute custom terminal profiles using `vscode.window.registerTerminalProfileProvider`. A `TerminalProfileProvider` implements the `provideTerminalProfile` method, which returns a `TerminalProfile` containing `TerminalOptions` or `ExtensionTerminalOptions`.

```typescript
vscode.window.registerTerminalProfileProvider('myext.customTerminal', {
    provideTerminalProfile(token: CancellationToken): TerminalProfile {
        return new TerminalProfile({
            name: 'Custom Shell',
            shellPath: '/usr/bin/zsh',
            shellArgs: ['--login']
        });
    }
});
```

### 2.9 Architecture Notes

The terminal system uses a multi-process architecture:
- **Extension Host** (`ExtHostTerminalService`) -- Handles `createTerminal` calls from extensions.
- **Main Thread** (`MainThreadTerminalService`) -- Receives RPC calls and manages actual terminal instances in the renderer.
- **Pty Host** -- Manages shell processes in a separate process.

For `Pseudoterminal`-backed terminals, `ExtHostPseudoterminal` acts as an `ITerminalChildProcess`, bridging the extension's pty events to the main thread via `TerminalProcessExtHostProxy`.

### 2.10 Source File References

| File | Role |
|---|---|
| `src/vs/workbench/api/common/extHostTerminalService.ts` | Extension host terminal service implementation |
| `src/vs/workbench/api/browser/mainThreadTerminalService.ts` | Main thread terminal service |
| `src/vs/workbench/contrib/terminal/` | Core terminal contribution |
| `src/vs/platform/terminal/common/terminal.ts` | Terminal interfaces and types |
| `src/vs/vscode.d.ts` | API type definitions for Terminal, TerminalOptions, etc. |

---

## 3. Extension Activation

VS Code extension activation is managed by `AbstractExtHostExtensionService` in the extension host process. It loads extension modules and calls their `activate()` function based on `activationEvents` declared in `package.json`.

### 3.1 Activation Events in `package.json`

The `activationEvents` property declares events that trigger extension activation.

| Event | Description | Example |
|---|---|---|
| `onCommand:<commandId>` | Activates when a specific command is invoked | `"onCommand:myext.doSomething"` |
| `onLanguage:<languageId>` | Activates when a file of the specified language is opened | `"onLanguage:json"` |
| `workspaceContains:<filePattern>` | Activates if the workspace contains a matching file | `"workspaceContains:**/.gitignore"` |
| `onStartupFinished` | Activates after all eager (`*`) extensions finish activating | -- |
| `*` | Activates on VS Code startup (discouraged for performance) | Used by the `git` extension |

**Implicit activation events**: VS Code automatically infers activation events from manifest contributions. For example, contributing a command in `contributes.commands` implicitly adds an `onCommand` activation event for that command. This means you often do not need to explicitly declare `onCommand` events if you contribute the command.

```json
{
  "activationEvents": [
    "onCommand:myExtension.runCommand",
    "onLanguage:javascript"
  ]
}
```

### 3.2 `activate()` Function

Every VS Code extension **must** export an `activate()` function. This is the entry point called when the extension is activated.

```typescript
import * as vscode from 'vscode';

export function activate(context: vscode.ExtensionContext) {
    // Register commands, providers, event listeners, etc.
    const disposable = vscode.commands.registerCommand('myext.hello', () => {
        vscode.window.showInformationMessage('Hello!');
    });
    context.subscriptions.push(disposable);
}
```

The `_doActivateExtension` method in `AbstractExtHostExtensionService` is responsible for loading the extension module and calling `activate`.

### 3.3 `deactivate()` Function

An **optional** export for cleanup when the extension is deactivated.

```typescript
export function deactivate() {
    // Clean up resources
}
```

Example: The `html-language-features` extension disposes of its language client in `deactivate()`.

### 3.4 `ExtensionContext` Object

The `ExtensionContext` is passed to `activate()` and provides lifecycle resources.

| Property/Method | Type | Description |
|---|---|---|
| `subscriptions` | `Disposable[]` | Disposables automatically disposed when the extension deactivates |
| `extensionUri` | `Uri` | URI of the extension's installation directory |
| `extensionPath` | `string` | Absolute file path to the extension directory |
| `asAbsolutePath(relativePath)` | method | Returns absolute path to a resource within the extension |
| `globalState` | `Memento` | Persistent key-value storage across VS Code sessions |
| `workspaceState` | `Memento` | Key-value storage scoped to the current workspace |
| `globalStorageUri` | `Uri` | URI for global persistent storage |
| `storageUri` | `Uri` | URI for workspace-specific storage |
| `logUri` | `Uri` | URI for extension-specific log directory |

### 3.5 Contribution Points in `package.json`

The `contributes` section declares what the extension adds to VS Code.

#### `contributes.commands`

Defines commands visible in the Command Palette and other UI surfaces:

```json
{
  "contributes": {
    "commands": [
      {
        "command": "myext.runTool",
        "title": "Run Tool",
        "category": "My Extension",
        "icon": "$(play)"
      }
    ]
  }
}
```

#### `contributes.menus`

Defines where commands appear in menus:

```json
{
  "contributes": {
    "menus": {
      "editor/context": [
        {
          "command": "myext.runTool",
          "when": "editorHasSelection",
          "group": "navigation"
        }
      ],
      "commandPalette": [
        {
          "command": "myext.runTool",
          "when": "editorIsOpen"
        }
      ]
    }
  }
}
```

Common menu locations:
- `editor/context` -- Editor right-click context menu
- `editor/title` -- Editor title bar
- `explorer/context` -- File explorer context menu
- `commandPalette` -- Command Palette (with optional `when` clause to filter visibility)
- `view/title` -- View title actions
- `view/item/context` -- View item context menu

### 3.6 Source File References

| File | Role |
|---|---|
| `src/vs/workbench/api/common/extHostExtensionService.ts` | `AbstractExtHostExtensionService` -- manages extension loading and activation |
| `src/vs/workbench/services/extensions/common/extensions.ts` | Extension description interfaces and activation events |
| `src/vs/workbench/api/common/extHost.api.impl.ts` | Creates the `vscode` namespace API object exposed to extensions |
| `src/vs/workbench/services/extensions/common/extensionsRegistry.ts` | Extension contribution point registry |

---

## 4. Command Registration

Commands are the primary way extensions expose functionality to users. They connect programmatic registration via `vscode.commands.registerCommand` with declarative registration in `package.json`.

### 4.1 `vscode.commands.registerCommand`

Registers a command handler that will be invoked when the command is executed.

```typescript
vscode.commands.registerCommand(id: string, callback: (...args: any[]) => any, thisArg?: any): Disposable;
```

Returns a `Disposable` that unregisters the command when disposed.

### 4.2 `vscode.commands.registerTextEditorCommand`

Registers a command that operates on the active text editor. The callback receives `TextEditor` and `TextEditorEdit` as its first two arguments.

```typescript
vscode.commands.registerTextEditorCommand(
    id: string,
    callback: (textEditor: TextEditor, edit: TextEditorEdit, ...args: any[]) => void,
    thisArg?: any
): Disposable;
```

### 4.3 Registration Patterns

#### Basic registration in `activate()`:

```typescript
export function activate(context: vscode.ExtensionContext) {
    const disposable = vscode.commands.registerCommand('myext.sayHello', () => {
        vscode.window.showInformationMessage('Hello World!');
    });
    context.subscriptions.push(disposable);
}
```

#### Registration with arguments:

```typescript
vscode.commands.registerCommand('myext.openFile', (uri: vscode.Uri) => {
    vscode.window.showTextDocument(uri);
});
```

#### CommandManager pattern (from typescript-language-features):

The `typescript-language-features` extension uses a `CommandManager` class to centralize command registration:

```typescript
class CommandManager {
    private readonly commands = new Map<string, vscode.Disposable>();

    register(id: string, callback: (...args: any[]) => any): void {
        const disposable = vscode.commands.registerCommand(id, callback);
        this.commands.set(id, disposable);
    }

    dispose(): void {
        for (const disposable of this.commands.values()) {
            disposable.dispose();
        }
        this.commands.clear();
    }
}
```

### 4.4 Connecting to `package.json` `contributes.commands`

The `contributes.commands` section in `package.json` declares commands to VS Code. This allows:
- Commands to appear in the Command Palette
- Commands to be displayed in menus
- Implicit `onCommand` activation events to be generated
- Icons and categories to be associated with commands

```json
{
  "contributes": {
    "commands": [
      {
        "command": "myext.sayHello",
        "title": "Say Hello",
        "category": "My Extension"
      },
      {
        "command": "myext.openFile",
        "title": "Open File",
        "category": "My Extension",
        "icon": {
          "light": "resources/light/open.svg",
          "dark": "resources/dark/open.svg"
        }
      }
    ]
  }
}
```

**Important**: The `command` string in `contributes.commands` must exactly match the `id` string passed to `vscode.commands.registerCommand`. The `title` is what the user sees in the Command Palette.

### 4.5 How It Works Internally

1. VS Code reads `contributes.commands` from all installed extensions' `package.json` files at startup.
2. The `commandsExtensionPoint` in VS Code core processes these declarations and registers them with the `MenuRegistry`.
3. The `activationEventsGenerator` uses the `command` field to generate implicit `onCommand` activation events.
4. When a user invokes the command (via palette, menu, keybinding, or API), VS Code first activates the owning extension (if not already active), then calls the registered handler.
5. In the extension host, `extHost.api.impl.ts` maps `vscode.commands.registerCommand` to the `ExtHostCommands` service.
6. `extHostCommands.ts` contains the actual logic for registering and unregistering commands.

### 4.6 Real-World Examples from Built-in Extensions

- **npm extension**: Registers `npm.runSelectedScript`, `npm.refresh`, and others.
- **vscode-test-resolver**: Registers `vscode-testresolver.newWindow`.
- **github extension**: Declares `github.publish` and `github.copyVscodeDevLink` in `package.json`.

### 4.7 Source File References

| File | Role |
|---|---|
| `src/vs/workbench/api/common/extHost.api.impl.ts` | Maps `vscode.commands.registerCommand` to internal services |
| `src/vs/workbench/api/common/extHostCommands.ts` | `ExtHostCommands` -- actual command registration logic |
| `src/vs/workbench/services/extensions/common/extensionsRegistry.ts` | Processes `contributes.commands` extension point |
| `src/vs/platform/actions/common/actions.ts` | `MenuRegistry` where commands are registered for UI visibility |

---

## 5. Putting It All Together: Extension Structure for QuickPick + Terminal

Based on the research above, here is how the pieces connect for an extension that uses QuickPick menus to select options and then spawns CLI processes in the terminal.

### 5.1 `package.json` Manifest

```json
{
  "name": "my-cli-extension",
  "activationEvents": [],
  "contributes": {
    "commands": [
      {
        "command": "myCli.selectAndRun",
        "title": "Select and Run CLI Tool",
        "category": "My CLI"
      }
    ],
    "menus": {
      "commandPalette": [
        {
          "command": "myCli.selectAndRun"
        }
      ]
    }
  }
}
```

Note: `activationEvents` can be left empty because contributing commands implicitly generates `onCommand` activation events.

### 5.2 Extension Entry Point Pattern

```typescript
import * as vscode from 'vscode';

export function activate(context: vscode.ExtensionContext) {
    // 1. Register command
    const disposable = vscode.commands.registerCommand('myCli.selectAndRun', async () => {

        // 2. Show QuickPick for user selection
        const selection = await vscode.window.showQuickPick(
            [
                { label: 'Build', description: 'Run build process' },
                { label: 'Test', description: 'Run test suite' },
                { label: 'Deploy', description: 'Deploy to staging' }
            ],
            { placeHolder: 'Select a CLI command to run' }
        );

        if (!selection) return;

        // 3. Create terminal and execute selected command
        const terminal = vscode.window.createTerminal({
            name: `CLI: ${selection.label}`,
            cwd: vscode.workspace.workspaceFolders?.[0]?.uri
        });
        terminal.show();
        terminal.sendText(`npm run ${selection.label.toLowerCase()}`);
    });

    context.subscriptions.push(disposable);
}

export function deactivate() {}
```

### 5.3 Key Architectural Notes

- **Extension Host to Main Thread**: QuickPick calls go from `ExtHostQuickOpen` to `MainThreadQuickOpen` via RPC. Terminal calls go from `ExtHostTerminalService` to `MainThreadTerminalService` via RPC.
- **Disposables**: Always push command registrations and event subscriptions to `context.subscriptions` for automatic cleanup.
- **Terminal lifecycle**: Use `onDidCloseTerminal` to detect when a terminal is closed and react accordingly.
- **Multi-step workflows**: Use `createQuickPick()` instead of `showQuickPick()` for wizard-style multi-step selection flows. Set `step` and `totalSteps` properties, and handle the back button via `onDidTriggerButton`.

---

## 6. Additional DeepWiki Resources

- [VS Code Codebase Overview](https://deepwiki.com/wiki/microsoft/vscode#1)
- [Extension System](https://deepwiki.com/wiki/microsoft/vscode#3)
- [Integrated Terminal](https://deepwiki.com/wiki/microsoft/vscode#6)
- [TypeScript and JavaScript Language Features](https://deepwiki.com/wiki/microsoft/vscode#15)
- [Source Control](https://deepwiki.com/wiki/microsoft/vscode#5)

---

## 7. Gaps and Limitations

- **`tooltip` on `QuickPickItem`**: This is a proposed API and may require enabling proposed API features in the extension's `package.json` (`"enabledApiProposals": ["quickPickItemTooltip"]`).
- **`titleTemplate` on `TerminalOptions`**: This is also a proposed API requiring opt-in.
- **`sortByLabel` on `QuickPick`**: Proposed API for controlling label-based sorting during filtering.
- **Shell Integration API**: The `TerminalShellIntegration` property on `Terminal` provides deeper integration but may have varying support across shells and platforms.
- **Pseudoterminal limitations**: While powerful, the `Pseudoterminal` interface requires the extension to manage all I/O, which is more complex than simply using `sendText` on a standard terminal.
