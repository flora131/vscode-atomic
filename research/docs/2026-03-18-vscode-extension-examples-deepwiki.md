# VS Code Extension Examples -- DeepWiki Research

**Date**: 2026-03-18
**Sources**: DeepWiki queries against `microsoft/vscode` and `microsoft/vscode-extension-samples`

---

## Table of Contents

1. [QuickPick Examples in Built-in Extensions](#1-quickpick-examples-in-built-in-extensions)
2. [Programmatic Terminal Creation in Built-in Extensions](#2-programmatic-terminal-creation-in-built-in-extensions)
3. [VS Code Extension Project Structure and Lifecycle](#3-vs-code-extension-project-structure-and-lifecycle)
4. [QuickPick and Terminal API Samples from vscode-extension-samples](#4-quickpick-and-terminal-api-samples-from-vscode-extension-samples)

---

## 1. QuickPick Examples in Built-in Extensions

### 1.1 Git Extension -- Checkout with QuickPick

**Source file**: `extensions/git/src/commands.ts`

The Git extension's `_checkout` method is one of the most comprehensive QuickPick implementations in the codebase. It is decorated with `@command('git.checkout', { repository: true })`.

**Pattern overview**:

1. A `QuickPick` instance is created via `window.createQuickPick()`.
2. Properties are configured: `busy`, `sortByLabel`, `matchOnDetail`, `placeholder`.
3. Items are populated from `createCheckoutItems()`, which retrieves branches and tags via `repository.getRefs`.
4. Special items are injected: `CreateBranchItem`, `CreateBranchFromItem`, `CheckoutDetachedItem`.
5. Event listeners handle user selection and dismissal.

**Special QuickPick items**:

- `CreateBranchItem` -- labeled "Create new branch...", triggers the `_branch` method.
- `CreateBranchFromItem` -- labeled "Create new branch from...", triggers `_branch` with `from: true`.
- `CheckoutDetachedItem` -- labeled "Checkout detached...", recursively calls `_checkout` with `detached: true`.

**Branch and tag listing** via `createCheckoutItems`:

- Local branches are represented by `BranchItem` / `CheckoutItem`.
- Remote branches are represented by `CheckoutRemoteHeadItem`.
- Tags are represented by `CheckoutTagItem` with commit hash and date in the description.
- `RefItemSeparator` visually groups different reference types.
- The `git.checkoutType` configuration controls which reference types are shown.

**Selection handling**:

```
User selects item
  -> CreateBranchItem:         calls _branch()
  -> CreateBranchFromItem:     calls _branch(from=true)
  -> CheckoutDetachedItem:     calls _checkout(detached=true)
  -> BranchItem/TagItem:       calls choice.run() -> repository.checkout() or repository.checkoutTracking()
```

Error handling covers `GitErrorCodes.DirtyWorkTree` and `GitErrorCodes.WorktreeBranchAlreadyUsed`, offering stash, migrate, or force-checkout options.

**Event listeners used**:

- `onDidAccept` -- captures user selection.
- `onDidHide` -- handles QuickPick dismissal.
- `onDidTriggerItemButton` -- handles actions on item-specific buttons.

### 1.2 Git Extension -- SSH Askpass

**Source file**: `extensions/git/src/askpass.ts`

The `handleSSHAskpass` function uses `window.showQuickPick` to ask for user confirmation regarding SSH host authenticity.

### 1.3 Debug Extension -- Loaded Scripts Picker

**Source file**: `src/vs/workbench/contrib/debug/common/loadedScriptsPicker.ts`

The `showLoadedScriptMenu` function demonstrates an advanced QuickPick with separators and dynamic filtering.

**Implementation**:

1. **Creation**: `quickInputService.createQuickPick<IPickerDebugItem>({ useSeparators: true })`
2. **Configuration**: `matchOnLabel`, `matchOnDescription`, `matchOnDetail`, `sortByLabel`, `placeholder`.
3. **Population**: `_getPicks` iterates active debug sessions; `_getPicksFromSession` creates items per session.
4. **Separators**: Each debug session gets a separator: `{ type: 'separator', label: session.name }`.
5. **Dynamic filtering**: `onDidChangeValue` re-calls `_getPicks` with the current filter. Uses `matchesFuzzy` for fuzzy matching against label (basename) and description (dirname).
6. **Acceptance**: `onDidAccept` calls `item.accept()` which opens the script in an editor.
7. **Disposal**: `onDidHide` disposes of all resources.

The `IPickerDebugItem` interface extends `IQuickPickItem` with an `accept()` method. The command is registered in `src/vs/workbench/contrib/debug/browser/debugCommands.ts` under the ID `SHOW_LOADED_SCRIPTS_ID`.

### 1.4 Other QuickPick Usages in the Codebase

| Feature Area | Source File | Description |
|---|---|---|
| Extensions management | `src/vs/workbench/contrib/extensions/browser/extensionsActions.ts` | `AbstractInstallExtensionsInServerAction` uses QuickPick for selecting extensions to install |
| Task system | `src/vs/workbench/contrib/tasks/browser/abstractTaskService.ts` | `_showQuickPick` presents a list of tasks |
| Snippet picker | `src/vs/workbench/contrib/snippets/browser/snippetPicker.ts` | QuickPick for selecting code snippets |
| Theme selection | `src/vs/workbench/contrib/themes/browser/themes.contribution.ts` | QuickPick for theme selection, including marketplace search |
| Editor resolver | `src/vs/workbench/services/editor/browser/editorResolverService.ts` | QuickPick for selecting an editor for a resource |

### 1.5 General QuickPick Patterns in VS Code

The two main APIs:

- **Extension API**: `window.createQuickPick()` -- used in built-in extensions.
- **Workbench service**: `IQuickInputService.createQuickPick()` -- used in workbench contributions.

Common pattern:

```typescript
// 1. Create the QuickPick
const quickPick = window.createQuickPick<MyQuickPickItem>();

// 2. Configure
quickPick.placeholder = 'Select an item...';
quickPick.matchOnDescription = true;
quickPick.matchOnDetail = true;
quickPick.sortByLabel = false;

// 3. Populate items
quickPick.items = myItems;

// 4. Handle events
quickPick.onDidAccept(() => {
    const selected = quickPick.selectedItems[0];
    // Handle selection
    quickPick.hide();
});

quickPick.onDidHide(() => {
    quickPick.dispose();
});

// 5. Show
quickPick.show();
```

Key `QuickPickItem` properties: `label`, `description`, `detail`, `picked`, `alwaysShow`.

---

## 2. Programmatic Terminal Creation in Built-in Extensions

### 2.1 Two Terminal Creation APIs

VS Code provides two ways to create terminals programmatically:

#### Shell-backed terminals (`vscode.TerminalOptions`)

```typescript
const terminal = vscode.window.createTerminal({
    name: 'My Shell Terminal',
    shellPath: '/bin/bash',
    shellArgs: ['-l'],
    cwd: vscode.Uri.file('/path/to/workspace'),
    env: { MY_VAR: 'my_value' },
    hideFromUser: false
});
terminal.show();
terminal.sendText('echo Hello from my terminal!');
```

Flow: Extension Host (`ExtHostTerminalService.createTerminalFromOptions`) -> RPC `$createTerminal` -> `MainThreadTerminalService` -> constructs `IShellLaunchConfig` -> `ITerminalService.createTerminal` -> Pty Host spawns shell process.

#### Pseudoterminal-backed terminals (`vscode.ExtensionTerminalOptions`)

```typescript
class MyPseudoterminal implements vscode.Pseudoterminal {
    private writeEmitter = new EventEmitter<string>();
    onDidWrite = this.writeEmitter.event;
    private closeEmitter = new EventEmitter<number | void>();
    onDidClose = this.closeEmitter.event;

    open(initialDimensions: vscode.TerminalDimensions | undefined): void {
        this.writeEmitter.fire('Welcome to my custom terminal!\r\n');
    }

    close(): void {
        this.closeEmitter.fire();
    }

    handleInput(data: string): void {
        this.writeEmitter.fire(`You typed: ${data}\r\n`);
        if (data.includes('exit')) {
            this.closeEmitter.fire(0);
        }
    }
}

const pty = new MyPseudoterminal();
const terminal = vscode.window.createTerminal({
    name: 'My Custom Terminal',
    pty: pty
});
terminal.show();
```

Flow: Extension Host (`BaseExtHostTerminalService.createExtensionTerminal`) -> creates `ExtHostPseudoterminal` (implements `ITerminalChildProcess`, wraps the extension's `Pseudoterminal`) -> `MainThreadTerminalService` receives `TerminalProcessExtHostProxy` as `customPtyImplementation` -> forwards data/events between the extension's `Pseudoterminal` and `TerminalProcessManager`.

### 2.2 Debug Extension -- Terminal Creation

**Source file**: `src/vs/workbench/contrib/debug/` (debug terminal implementation)

The Debug extension creates terminals to run debug processes using `this._terminalService.createTerminalFromOptions`. Key characteristics:

- Passes `shellPath`, `shellArgs`, `cwd`, `name` via options.
- Terminal is marked as a "feature terminal".
- Shell integration is forced on.

### 2.3 Tasks Extension -- Terminal Creation

**Source file**: `src/vs/workbench/contrib/tasks/` (task terminal implementation)

The Tasks extension creates terminals for task execution:

- For `CustomExecution` runtime: provides a `customPtyImplementation` to `IShellLaunchConfig`, giving the task full control over terminal I/O.
- For other runtimes: constructs an `IShellLaunchConfig` with the resolved command and arguments.

### 2.4 Terminal Architecture

The multi-process terminal architecture:

```
Extension Host                     Renderer Process              Pty Host
+-----------------------+         +---------------------+       +------------------+
| ExtHostTerminalService|  --RPC->| MainThreadTerminal  | -IPC->| PtyService       |
| (creates ExtHostTerminal)       | Service             |       | (shell processes)|
+-----------------------+         +---------------------+       +------------------+
```

- `PtyService` in the Pty Host manages underlying shell processes.
- Terminal processes are isolated from the main renderer process.
- Communication uses RPC (Extension Host <-> Renderer) and IPC (Renderer <-> Pty Host).

---

## 3. VS Code Extension Project Structure and Lifecycle

### 3.1 Recommended Project Structure

```
my-extension/
├── package.json          # Extension manifest
├── tsconfig.json         # TypeScript configuration
├── src/
│   ├── extension.ts      # Main entry point (Node.js / Electron)
│   └── extension.browser.ts  # Browser entry point (optional, for web)
├── out/                  # Compiled JavaScript output
│   └── extension.js
├── dist/                 # Bundled output (optional)
│   └── browser/
│       └── extension.js
└── test/                 # Tests
```

Real-world examples from the VS Code repo:

- `typescript-language-features`: main = `./out/extension`, browser = `./dist/browser/extension`
- `css-language-features`: main = `./client/out/node/cssClientMain`, browser = `./client/dist/browser/cssClientMain`

### 3.2 package.json Structure

#### Entry Points

```json
{
  "main": "./out/extension.js",
  "browser": "./dist/browser/extension.js"
}
```

- `main`: Entry point for Node.js environments (desktop VS Code).
- `browser`: Entry point for browser environments (VS Code for the Web).

#### activationEvents

Declares when the extension will be activated. Common events:

| Event | Description | Example Extension |
|---|---|---|
| `onLanguage:<languageId>` | File of specified language opened | `typescript-language-features` (`onLanguage:javascript`, `onLanguage:typescript`) |
| `onCommand:<commandId>` | Specific command invoked | `typescript-language-features` (`typescript.restartTsServer`) |
| `onStartupFinished` | After VS Code startup is complete | `merge-conflict` |
| `*` | On VS Code startup (use sparingly) | `git` |
| `onTaskType:<taskType>` | Tasks of a type need listing/resolving | `jake` (`onTaskType:jake`) |
| `onTerminalProfile:<profileId>` | Terminal profile requested | `terminal-sample` (`onTerminalProfile:terminalTest.terminal-profile`) |

#### contributes.commands

```json
{
  "contributes": {
    "commands": [
      {
        "command": "myExtension.myCommand",
        "title": "My Command Title",
        "category": "My Extension",
        "icon": "$(icon-name)",
        "enablement": "editorLangId == typescript"
      }
    ]
  }
}
```

Fields:
- `command`: Unique identifier string.
- `title`: Human-readable label for the Command Palette.
- `category`: Optional grouping category.
- `icon`: Optional icon (ThemeIcon or path).
- `enablement`: Optional `when`-clause for enabling/disabling.

### 3.3 Extension Lifecycle

```
VS Code Startup
    |
    v
Activation Event Occurs?
    |
    +--(yes)--> Call activate(context: ExtensionContext)
    |               |
    |               v
    |           Extension initializes:
    |             - Register commands
    |             - Register providers
    |             - Subscribe to events
    |             - Push disposables to context.subscriptions
    |               |
    |               v
    |           Extension Running
    |               |
    |               v
    |           VS Code Shutdown / Extension Disabled
    |               |
    |               v
    |           Call deactivate() (if exported)
    |               |
    |               v
    |           Cleanup resources
    |               |
    |               v
    |           Extension Deactivated
    |
    +--(no)---> Extension remains inactive
```

The `activate` function:
- Receives an `ExtensionContext` with `subscriptions`, `extensionUri`, `globalState`, `workspaceState`, etc.
- Should register all commands, providers, and event handlers.
- Should push all disposables to `context.subscriptions` for automatic cleanup.

The `deactivate` function:
- Called on shutdown or when the extension is disabled.
- Should perform any final cleanup not handled by disposable subscriptions.

**Example** from `typescript-language-features`:
- `extension.ts` (Electron) and `extension.browser.ts` (Web) are entry points.
- Activation creates a `PluginManager`, `CommandManager`, calls `createLazyClientHost()`, and `registerBaseCommands()`.

---

## 4. QuickPick and Terminal API Samples from vscode-extension-samples

### 4.1 quickinput-sample

**Repository**: `microsoft/vscode-extension-samples`
**Path**: `quickinput-sample/`

#### package.json

Key fields:

```json
{
  "main": "./out/extension.js",
  "activationEvents": ["onCommand:samples.quickInput"],
  "contributes": {
    "commands": [
      {
        "command": "samples.quickInput",
        "title": "Quick Input Samples"
      }
    ]
  },
  "devDependencies": {
    "@eslint/js": "...",
    "@stylistic/eslint-plugin": "...",
    "@types/node": "...",
    "@types/vscode": "...",
    "eslint": "...",
    "typescript": "...",
    "typescript-eslint": "..."
  }
}
```

No runtime dependencies -- only `devDependencies`.

#### extension.ts

The `activate()` function registers the `samples.quickInput` command, which presents a QuickPick menu with four options:

- `showQuickPick` -- basic quick pick demo
- `showInputBox` -- basic input box demo
- `multiStepInput` -- multi-step wizard demo
- `quickOpen` -- dynamic file filtering demo

Each option calls a function from its respective module.

#### Source Files

| File | Purpose |
|---|---|
| `src/extension.ts` | Entry point; registers command, shows top-level menu |
| `src/basicInput.ts` | Demonstrates `window.showQuickPick()` and `window.showInputBox()` |
| `src/multiStepInput.ts` | Multi-step wizard with `window.createQuickPick()` and `window.createInputBox()` |
| `src/quickOpen.ts` | Dynamic file filtering with `window.createQuickPick()` |

#### basicInput.ts -- Simple QuickPick and InputBox

**`showQuickPick()`**:
```typescript
// Basic usage with string array
const result = await window.showQuickPick(['one', 'two', 'three'], {
    placeHolder: 'Select an option',
    onDidSelectItem: item => {
        window.showInformationMessage(`Focused: ${item}`);
    }
});
window.showInformationMessage(`Selected: ${result}`);
```

**`showInputBox()`** with validation:
```typescript
const result = await window.showInputBox({
    value: 'abcdef',
    valueSelection: [2, 4],  // Select characters at positions 2-4
    placeHolder: 'Enter a value',
    validateInput: text => {
        return text === '123' ? 'Not 123!' : null;
    }
});
```

The `validateInput` callback returns an error string or `null` (valid). It can also be async.

#### multiStepInput.ts -- Multi-Step Wizard

This is the most sophisticated example. It implements a multi-step wizard using the `InputFlowAction` pattern and a `MultiStepInput` helper class.

**State interface**:
```typescript
interface State {
    title: string;
    step: number;
    totalSteps: number;
    resourceGroup: QuickPickItem | string;
    name: string;
    runtime: QuickPickItem;
}
```

**Step functions**:

1. `pickResourceGroup` -- uses `input.showQuickPick` with a custom `createResourceGroupButton`. If button pressed, transitions to `inputResourceGroupName`; otherwise to `inputName`.
2. `inputResourceGroupName` -- uses `input.showInputBox` with `validateNameIsUnique`.
3. `inputName` -- uses `input.showInputBox` for application service name.
4. `pickRuntime` -- uses `input.showQuickPick` with a dynamically loaded list.

**InputFlowAction pattern**:

`InputFlowAction` is an enum-like class with three values:
- `InputFlowAction.back` -- go to previous step (pops current and previous from stack, re-executes previous).
- `InputFlowAction.cancel` -- cancel entire wizard (sets step to undefined).
- `InputFlowAction.resume` -- resume current step after interruption (pops current, re-executes).

The `MultiStepInput.run()` method iterates through steps, and `stepThrough` catches `InputFlowAction` rejections to manage navigation.

The `showQuickPick` and `showInputBox` methods within `MultiStepInput` listen for:
- `onDidTriggerButton` (specifically `QuickInputButtons.Back`) -- rejects with `InputFlowAction.back`.
- `onDidHide` -- rejects with `InputFlowAction.cancel`.

#### quickOpen.ts -- Dynamic File Filtering

Demonstrates live search using `window.createQuickPick()` with asynchronous results:

```typescript
async function pickFile() {
    const disposables: Disposable[] = [];
    try {
        return await new Promise<Uri | undefined>((resolve) => {
            const input = window.createQuickPick<FileItem | MessageItem>();
            input.placeholder = 'Type to search for files';
            let rgs: cp.ChildProcess[] = [];
            disposables.push(
                input.onDidChangeValue(value => {
                    rgs.forEach(rg => rg.kill());
                    if (!value) {
                        input.items = [];
                        return;
                    }
                    input.busy = true;
                    const cwds = workspace.workspaceFolders
                        ? workspace.workspaceFolders.map(f => f.uri.fsPath)
                        : [process.cwd()];
                    const q = process.platform === 'win32' ? '"' : "'";
                    rgs = cwds.map(cwd => {
                        const rg = cp.exec(
                            `rg --files -g ${q}*${value}*${q}`,
                            { cwd },
                            (err, stdout) => {
                                // Process results, concat FileItem instances
                                // Set input.busy = false when all processes complete
                            }
                        );
                        return rg;
                    });
                }),
                input.onDidChangeSelection(items => {
                    const item = items[0];
                    if (item instanceof FileItem) {
                        resolve(item.uri);
                        input.hide();
                    }
                }),
                input.onDidHide(() => {
                    rgs.forEach(rg => rg.kill());
                    resolve(undefined);
                    input.dispose();
                })
            );
            input.show();
        });
    } finally {
        disposables.forEach(d => d.dispose());
    }
}
```

Key patterns:
- `input.busy = true` shows a loading indicator while searching.
- `onDidChangeValue` debounces by killing previous `rg` child processes.
- Results are capped at 50 items per workspace folder.
- Proper cleanup in `finally` block disposes all event listener subscriptions.

### 4.2 terminal-sample

**Repository**: `microsoft/vscode-extension-samples`
**Path**: `terminal-sample/`

#### package.json

Key fields:

```json
{
  "enabledApiProposals": [
    "terminalDataWriteEvent",
    "terminalDimensions"
  ],
  "activationEvents": [
    "onTerminalProfile:terminalTest.terminal-profile"
  ],
  "contributes": {
    "commands": [
      { "command": "terminalTest.createAndSend", "title": "Terminal API: Create Terminal and Immediately Send" },
      { "command": "terminalTest.createTerminal", "title": "Terminal API: Create Terminal" },
      { "command": "terminalTest.createTerminalHideFromUser", "title": "Terminal API: Create Terminal (hideFromUser)" },
      { "command": "terminalTest.createZshLoginShell", "title": "Terminal API: Create Terminal (zsh login shell)" },
      { "command": "terminalTest.dimensions", "title": "Terminal API: Set dimensions" },
      { "command": "terminalTest.dispose", "title": "Terminal API: Dispose" },
      { "command": "terminalTest.hide", "title": "Terminal API: Hide" },
      { "command": "terminalTest.onDidWriteTerminalData", "title": "Terminal API: Attach data listener" },
      { "command": "terminalTest.onDidChangeTerminalDimensions", "title": "Terminal API: Attach dimensions listener" },
      { "command": "terminalTest.processId", "title": "Terminal API: Get process ID" },
      { "command": "terminalTest.sendText", "title": "Terminal API: Send Text" },
      { "command": "terminalTest.sendTextNoNewLine", "title": "Terminal API: Send Text (no implied \\n)" },
      { "command": "terminalTest.show", "title": "Terminal API: Show" },
      { "command": "terminalTest.showPreserveFocus", "title": "Terminal API: Show (preserving focus)" },
      { "command": "terminalTest.terminals", "title": "Terminal API: View terminals" },
      { "command": "terminalTest.updateEnvironment", "title": "Terminal API: Update environment" },
      { "command": "terminalTest.clearEnvironment", "title": "Terminal API: Clear environment" },
      { "command": "terminalTest.registerTerminalLinkProvider", "title": "Terminal API: Register link provider" }
    ]
  }
}
```

18 commands total covering terminal creation, visibility, text sending, environment variables, event listeners, link providers, and terminal profiles.

#### extension.ts -- activate() Function

The full `activate()` function demonstrates every major Terminal API feature:

**Terminal creation patterns**:

```typescript
// Basic terminal with name
vscode.window.createTerminal(`Ext Terminal #${NEXT_TERM_ID++}`);

// Hidden terminal
vscode.window.createTerminal({
    name: `Ext Terminal #${NEXT_TERM_ID++}`,
    hideFromUser: true
});

// Create and immediately send text
const terminal = vscode.window.createTerminal(`Ext Terminal #${NEXT_TERM_ID++}`);
terminal.sendText("echo 'Sent text immediately after creating'");

// Custom shell with arguments
vscode.window.createTerminal(`Ext Terminal #${NEXT_TERM_ID++}`, '/bin/zsh', ['-l']);
```

**Terminal lifecycle events**:

```typescript
// Open event
vscode.window.onDidOpenTerminal(terminal => {
    vscode.window.showInformationMessage(`onDidOpenTerminal, name: ${terminal.name}`);
});

// Active terminal changed
vscode.window.onDidChangeActiveTerminal(e => {
    console.log(`Active terminal changed, name=${e ? e.name : 'undefined'}`);
});

// Close event
vscode.window.onDidCloseTerminal(terminal => {
    vscode.window.showInformationMessage(`onDidCloseTerminal, name: ${terminal.name}`);
});
```

**Terminal operations**:

```typescript
// Show / hide
terminal.show();          // Show and take focus
terminal.show(true);      // Show but preserve focus
terminal.hide();

// Send text
terminal.sendText("echo 'Hello world!'");       // With trailing newline
terminal.sendText("echo 'Hello world!'", false); // Without trailing newline

// Dispose
terminal.dispose();

// Get process ID
const processId = await terminal.processId;
```

**Environment variable management**:

```typescript
const collection = context.environmentVariableCollection;
collection.replace('FOO', 'BAR');
collection.append('PATH', '/test/path');
collection.clear();
```

**Terminal link provider**:

```typescript
type CustomTerminalLink = vscode.TerminalLink & { data: string };

vscode.window.registerTerminalLinkProvider(new class implements vscode.TerminalLinkProvider<CustomTerminalLink> {
    provideTerminalLinks(context: vscode.TerminalLinkContext, _token: vscode.CancellationToken) {
        const startIndex = (context.line as string).indexOf('link');
        if (startIndex === -1) { return []; }
        return [{
            startIndex,
            length: 'link'.length,
            tooltip: 'Show a notification',
            data: 'Example data'
        }];
    }
    handleTerminalLink(link: CustomTerminalLink) {
        vscode.window.showInformationMessage(`Link activated (data = ${link.data})`);
    }
});
```

**Terminal profile provider**:

```typescript
context.subscriptions.push(
    vscode.window.registerTerminalProfileProvider('terminalTest.terminal-profile', {
        provideTerminalProfile(_token: vscode.CancellationToken): vscode.ProviderResult<vscode.TerminalProfile> {
            return {
                options: {
                    name: 'Terminal API',
                    shellPath: process.title || 'C:/Windows/System32/cmd.exe'
                }
            };
        }
    })
);
```

#### Helper Functions

**`selectTerminal()`** -- presents a QuickPick of open terminals:

```typescript
function selectTerminal(): Thenable<vscode.Terminal | undefined> {
    interface TerminalQuickPickItem extends vscode.QuickPickItem {
        terminal: vscode.Terminal;
    }
    const terminals = vscode.window.terminals;
    const items: TerminalQuickPickItem[] = terminals.map(t => ({
        label: `name: ${t.name}`,
        terminal: t
    }));
    return vscode.window.showQuickPick(items).then(item => {
        return item ? item.terminal : undefined;
    });
}
```

This is a good example of combining QuickPick with Terminal APIs -- it extends `QuickPickItem` with a `terminal` property.

**`ensureTerminalExists()`** -- guard function:

```typescript
function ensureTerminalExists(): boolean {
    if (vscode.window.terminals.length === 0) {
        vscode.window.showErrorMessage('No active terminals');
        return false;
    }
    return true;
}
```

Used as a guard before terminal operations:

```typescript
context.subscriptions.push(vscode.commands.registerCommand('terminalTest.hide', () => {
    if (ensureTerminalExists()) {
        selectTerminal().then(terminal => {
            if (terminal) {
                terminal.hide();
            }
        });
    }
}));
```

---

## Summary of Key API Patterns

### QuickPick API Summary

| API | Use Case | Complexity |
|---|---|---|
| `window.showQuickPick(items)` | Simple one-shot selection from a list | Low |
| `window.createQuickPick()` | Full control: dynamic items, buttons, separators, busy state | Medium-High |
| `IQuickInputService.createQuickPick()` | Workbench-internal (not extension API), supports separators | Internal |

### Terminal API Summary

| API | Use Case |
|---|---|
| `window.createTerminal(name)` | Basic terminal with a name |
| `window.createTerminal(options: TerminalOptions)` | Terminal with shell path, args, cwd, env |
| `window.createTerminal(options: ExtensionTerminalOptions)` | Pseudoterminal with full I/O control |
| `terminal.sendText(text, addNewLine?)` | Send commands to terminal |
| `terminal.show(preserveFocus?)` | Show terminal panel |
| `terminal.hide()` | Hide terminal from UI |
| `terminal.dispose()` | Close and destroy terminal |
| `window.onDidOpenTerminal` | React to terminal creation |
| `window.onDidCloseTerminal` | React to terminal closure |
| `window.onDidChangeActiveTerminal` | React to active terminal changes |
| `window.registerTerminalLinkProvider` | Add clickable links in terminal output |
| `window.registerTerminalProfileProvider` | Add custom terminal profile |
| `context.environmentVariableCollection` | Manage environment variables for terminals |

### Extension Lifecycle Summary

```
package.json (manifest)
  -> activationEvents (when to wake up)
  -> contributes.commands (what commands to register)
  -> main (where the code lives)

extension.ts
  -> export function activate(context) { ... }
  -> export function deactivate() { ... }

context.subscriptions
  -> Push all disposables here for automatic cleanup
```

---

## Source References

### microsoft/vscode (built-in extensions and workbench)

- `extensions/git/src/commands.ts` -- Git checkout QuickPick implementation
- `extensions/git/src/askpass.ts` -- SSH askpass QuickPick
- `src/vs/workbench/contrib/debug/common/loadedScriptsPicker.ts` -- Debug loaded scripts picker
- `src/vs/workbench/contrib/debug/browser/debugCommands.ts` -- Debug command registration
- `src/vs/workbench/contrib/extensions/browser/extensionsActions.ts` -- Extensions QuickPick
- `src/vs/workbench/contrib/tasks/browser/abstractTaskService.ts` -- Task QuickPick
- `src/vs/workbench/contrib/snippets/browser/snippetPicker.ts` -- Snippet QuickPick
- `src/vs/workbench/contrib/themes/browser/themes.contribution.ts` -- Theme QuickPick
- `src/vs/workbench/services/editor/browser/editorResolverService.ts` -- Editor resolver QuickPick

### microsoft/vscode-extension-samples

- `quickinput-sample/package.json` -- QuickInput sample manifest
- `quickinput-sample/src/extension.ts` -- QuickInput sample entry point
- `quickinput-sample/src/basicInput.ts` -- Basic showQuickPick/showInputBox
- `quickinput-sample/src/multiStepInput.ts` -- Multi-step wizard with InputFlowAction
- `quickinput-sample/src/quickOpen.ts` -- Dynamic file filtering with ripgrep
- `terminal-sample/package.json` -- Terminal sample manifest
- `terminal-sample/src/extension.ts` -- Terminal sample with 18 commands

---

## DeepWiki Search Links

- [QuickPick in built-in extensions](https://deepwiki.com/search/show-me-examples-of-builtin-ex_5b4fb346-c7f3-47f5-a807-c408d15513cf)
- [Terminal creation in built-in extensions](https://deepwiki.com/search/show-me-examples-of-builtin-ex_5ea7735a-af8b-4bbb-97dc-ab06b2ad02d2)
- [Extension project structure](https://deepwiki.com/search/what-is-the-recommended-projec_a4fdc196-4ece-46bf-9939-2e4b73de8f17)
- [QuickPick and Terminal API samples](https://deepwiki.com/search/what-quickpick-and-terminal-ap_6e19f030-b3f0-4b3f-8692-aa18be79ddb6)
- [quickinput-sample package.json details](https://deepwiki.com/search/show-me-the-exact-packagejson_6b7e592e-2cb0-4df9-ae27-ac5012ca4eb8)
- [terminal-sample package.json and activate()](https://deepwiki.com/search/show-me-the-exact-packagejson_bb02be92-e7d4-4307-b24e-059aba56ef53)
- [multiStepInput.ts details](https://deepwiki.com/search/show-the-multistepinputts-file_066b004f-3f31-45a4-a1bc-f3fa15ff92de)
- [Git checkout QuickPick details](https://deepwiki.com/search/how-does-the-git-extensions-ch_32c61732-d879-4bd0-8dc5-1c0548b67509)
- [basicInput.ts details](https://deepwiki.com/search/show-me-the-basicinputts-file_676e113a-d485-442c-b3bd-449651cb7c12)
- [Loaded scripts picker details](https://deepwiki.com/search/show-me-the-loaded-scripts-pic_266cc4bc-d1d2-4fd5-8567-142c4ec4f5dc)
- [selectTerminal and ensureTerminalExists helpers](https://deepwiki.com/search/show-me-the-helper-functions-s_6af0cd02-1650-4d32-87c1-e8c93eab5ec9)

---

## Gaps and Limitations

- DeepWiki did not return the complete verbatim `package.json` for either sample extension; the fields documented above are reconstructed from the described properties.
- The exact `activate()` function from `quickinput-sample/src/extension.ts` was described but not returned in full code form by DeepWiki.
- Proposed API details (`terminalDataWriteEvent`, `terminalDimensions`) were mentioned but not elaborated upon -- these may change or graduate to stable API.
- The Debug extension terminal creation code paths were described at the service level but specific file paths within the debug contribution were not pinpointed beyond the general `src/vs/workbench/contrib/debug/` directory.
