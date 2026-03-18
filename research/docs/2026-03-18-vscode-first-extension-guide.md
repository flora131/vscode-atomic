# VS Code "Your First Extension" Guide -- Research Findings

**Date**: 2026-03-18
**Sources**:
- https://code.visualstudio.com/api/get-started/your-first-extension
- https://code.visualstudio.com/api/get-started/extension-anatomy
- https://code.visualstudio.com/api/references/vscode-api#window

---

## 1. Your First Extension -- Scaffolding and Getting Started

**Source**: [Your First Extension](https://code.visualstudio.com/api/get-started/your-first-extension)

### Prerequisites

- [Node.js](https://nodejs.org) installed
- [Git](https://git-scm.com/) installed

### Scaffolding with Yeoman Generator

The official guide uses [Yeoman](https://yeoman.io/) and the [VS Code Extension Generator](https://www.npmjs.com/package/generator-code) to scaffold a new extension project.

**Option A -- Without installing Yeoman globally (one-off):**

```bash
npx --package yo --package generator-code -- yo code
```

**Option B -- Install Yeoman globally for repeated use:**

```bash
npm install --global yo generator-code
yo code
```

### Generator Prompts (TypeScript Project)

When running the generator, fill out the following fields:

```
# ? What type of extension do you want to create? New Extension (TypeScript)
# ? What's the name of your extension? HelloWorld
### Press <Enter> to choose default for all options below ###
# ? What's the identifier of your extension? helloworld
# ? What's the description of your extension? LEAVE BLANK
# ? Initialize a git repository? Y
# ? Which bundler to use? unbundled
# ? Which package manager to use? npm
# ? Do you want to open the new folder with Visual Studio Code? Open with `code`
```

### Running the Extension

1. Open `src/extension.ts` inside the editor.
2. Press **F5** or run the command **Debug: Start Debugging** from the Command Palette (Ctrl+Shift+P / Cmd+Shift+P).
3. This compiles and runs the extension in a new **Extension Development Host** window.
4. Run the **Hello World** command from the Command Palette in the new window.
5. You should see a `Hello World from HelloWorld!` notification.

**Troubleshooting**: If the Hello World command is not visible in the debug window, check that `engines.vscode` in `package.json` is compatible with the installed version of VS Code.

### Developing the Extension

To make changes:

1. Change the message from `"Hello World from HelloWorld!"` to `"Hello VS Code"` in `extension.ts`.
2. Run **Developer: Reload Window** in the Extension Development Host window.
3. Run the **Hello World** command again to see the updated message.

**Ideas for experimentation** (from the guide):

- Give the Hello World command a new name in the Command Palette.
- [Contribute](https://code.visualstudio.com/api/references/contribution-points) another command that displays the current time in an information message.
- Replace `vscode.window.showInformationMessage` with another [VS Code API](https://code.visualstudio.com/api/references/vscode-api) call to show a warning message.

### Debugging the Extension

- Set breakpoints by clicking the gutter next to a line.
- Hover over variables in the editor to inspect values.
- Use the **Run and Debug** view on the left to check variable values.
- The Debug Console allows evaluating expressions.
- More info: [Node.js Debugging Topic](https://code.visualstudio.com/docs/nodejs/nodejs-debugging)

### Sample Source Code

The official Hello World sample is available at:
https://github.com/microsoft/vscode-extension-samples/tree/main/helloworld-sample

A minimal JavaScript version is at:
https://github.com/microsoft/vscode-extension-samples/tree/main/helloworld-minimal-sample

---

## 2. Extension Anatomy -- Structure and Manifest

**Source**: [Extension Anatomy](https://code.visualstudio.com/api/get-started/extension-anatomy)

### Three Core Concepts

The Hello World extension demonstrates three fundamental concepts:

1. **Activation Events**: Events upon which your extension becomes active. The Hello World extension registers `onCommand:helloworld.helloWorld`.
   > **Note**: Starting with [VS Code 1.74.0](https://code.visualstudio.com/updates/v1_74#_implicit-activation-events-for-declared-extension-contributions), commands declared in the `commands` section of `package.json` automatically activate the extension when invoked, without requiring an explicit `onCommand` entry in `activationEvents`.

2. **Contribution Points**: Static declarations made in the `package.json` [Extension Manifest](https://code.visualstudio.com/api/references/extension-manifest) to extend VS Code. The Hello World extension uses [`contributes.commands`](https://code.visualstudio.com/api/references/contribution-points#contributes.commands) to make the command available in the Command Palette and bind it to command ID `helloworld.helloWorld`.

3. **VS Code API**: JavaScript APIs you can invoke in your extension code. The Hello World extension uses [`commands.registerCommand`](https://code.visualstudio.com/api/references/vscode-api#commands.registerCommand) to bind a function to the registered command ID.

### Extension File Structure

```
.
├── .vscode
│   ├── launch.json     // Config for launching and debugging the extension
│   └── tasks.json      // Config for build task that compiles TypeScript
├── .gitignore           // Ignore build output and node_modules
├── README.md            // Readable description of your extension's functionality
├── src
│   └── extension.ts     // Extension source code
├── package.json         // Extension manifest
├── tsconfig.json        // TypeScript configuration
```

**Configuration files**:
- `launch.json` -- used to configure VS Code [Debugging](https://code.visualstudio.com/docs/debugtest/debugging)
- `tasks.json` -- for defining VS Code [Tasks](https://code.visualstudio.com/docs/debugtest/tasks)
- `tsconfig.json` -- consult the TypeScript [Handbook](https://www.typescriptlang.org/docs/handbook/tsconfig-json.html)

### Extension Manifest (`package.json`)

Each VS Code extension must have a `package.json` as its [Extension Manifest](https://code.visualstudio.com/api/references/extension-manifest). It contains a mix of Node.js fields (`scripts`, `devDependencies`) and VS Code-specific fields (`publisher`, `activationEvents`, `contributes`).

**Most important fields**:

| Field | Description |
|-------|-------------|
| `name` + `publisher` | VS Code uses `<publisher>.<name>` as a unique ID for the extension (e.g., `vscode-samples.helloworld-sample`). |
| `main` | The extension entry point (e.g., `./out/extension.js`). |
| `activationEvents` | [Activation Events](https://code.visualstudio.com/api/references/activation-events) that trigger extension activation. |
| `contributes` | [Contribution Points](https://code.visualstudio.com/api/references/contribution-points) -- static declarations to extend VS Code. |
| `engines.vscode` | Specifies the minimum version of VS Code API that the extension depends on. |

**Full example `package.json`:**

```json
{
  "name": "helloworld-sample",
  "displayName": "helloworld-sample",
  "description": "HelloWorld example for VS Code",
  "version": "0.0.1",
  "publisher": "vscode-samples",
  "repository": "https://github.com/microsoft/vscode-extension-samples/helloworld-sample",
  "engines": {
    "vscode": "^1.51.0"
  },
  "categories": ["Other"],
  "activationEvents": [],
  "main": "./out/extension.js",
  "contributes": {
    "commands": [
      {
        "command": "helloworld.helloWorld",
        "title": "Hello World"
      }
    ]
  },
  "scripts": {
    "vscode:prepublish": "npm run compile",
    "compile": "tsc -p ./",
    "watch": "tsc -watch -p ./"
  },
  "devDependencies": {
    "@types/node": "^8.10.25",
    "@types/vscode": "^1.51.0",
    "tslint": "^5.16.0",
    "typescript": "^3.4.5"
  }
}
```

> **Note**: If your extension targets a VS Code version prior to 1.74, you must explicitly list `onCommand:helloworld.helloWorld` in `activationEvents`.

### Extension Entry File (`src/extension.ts`)

The extension entry file exports two functions: `activate` and `deactivate`.

- **`activate`**: Executed when your registered Activation Event happens.
- **`deactivate`**: Gives you a chance to clean up before your extension is deactivated. For many extensions, explicit cleanup may not be required and the `deactivate` method can be removed. However, if an extension needs to perform an operation when VS Code is shutting down or the extension is disabled/uninstalled, this is the method to do so.

The VS Code extension API is declared in the [@types/vscode](https://www.npmjs.com/package/@types/vscode) type definitions. The version of the `vscode` type definitions is controlled by the value in the `engines.vscode` field in `package.json`.

**Full example `extension.ts`:**

```typescript
// The module 'vscode' contains the VS Code extensibility API
// Import the module and reference it with the alias vscode in your code below
import * as vscode from 'vscode';

// this method is called when your extension is activated
// your extension is activated the very first time the command is executed
export function activate(context: vscode.ExtensionContext) {
  // Use the console to output diagnostic information (console.log) and errors (console.error)
  // This line of code will only be executed once when your extension is activated
  console.log('Congratulations, your extension "helloworld-sample" is now active!');

  // The command has been defined in the package.json file
  // Now provide the implementation of the command with registerCommand
  // The commandId parameter must match the command field in package.json
  let disposable = vscode.commands.registerCommand('helloworld.helloWorld', () => {
    // The code you place here will be executed every time your command is executed
    // Display a message box to the user
    vscode.window.showInformationMessage('Hello World!');
  });

  context.subscriptions.push(disposable);
}

// this method is called when your extension is deactivated
export function deactivate() {}
```

---

## 3. VS Code API Reference -- `window` Namespace

**Source**: [VS Code API -- window](https://code.visualstudio.com/api/references/vscode-api#window)

### Window Namespace Overview

> Namespace for dealing with the current window of the editor. That is visible and active editors, as well as, UI elements to show messages, selections, and asking for user input.

### Key Variables

| Variable | Type | Description |
|----------|------|-------------|
| `activeColorTheme` | `ColorTheme` | The currently active color theme. |
| `activeNotebookEditor` | `NotebookEditor \| undefined` | The currently active notebook editor. |
| `activeTerminal` | `Terminal \| undefined` | The currently active terminal. |
| `activeTextEditor` | `TextEditor \| undefined` | The currently active text editor. |
| `state` | `WindowState` | The current window state. |
| `tabGroups` | `TabGroups` | Tab group access. |
| `terminals` | `readonly Terminal[]` | All open terminals. |
| `visibleNotebookEditors` | `readonly NotebookEditor[]` | All visible notebook editors. |
| `visibleTextEditors` | `readonly TextEditor[]` | All visible text editors. |

### Key Terminal Events

| Event | Type |
|-------|------|
| `onDidChangeActiveTerminal` | `Event<Terminal \| undefined>` |
| `onDidChangeTerminalShellIntegration` | `Event<TerminalShellIntegrationChangeEvent>` |
| `onDidChangeTerminalState` | `Event<Terminal>` |
| `onDidCloseTerminal` | `Event<Terminal>` |
| `onDidEndTerminalShellExecution` | `Event<TerminalShellExecutionEndEvent>` |
| `onDidOpenTerminal` | `Event<Terminal>` |
| `onDidStartTerminalShellExecution` | `Event<TerminalShellExecutionStartEvent>` |

### QuickPick API

#### `window.createQuickPick<T extends QuickPickItem>(): QuickPick<T>`

Creates a `QuickPick` to let the user pick an item from a list of items of type `T`. The items can be filtered through a filter text field and there is an option `canSelectMany` to allow for selecting multiple items.

> **Note**: In many cases the more convenient `window.showQuickPick` is easier to use. `window.createQuickPick` should be used when `window.showQuickPick` does not offer the required flexibility.

#### `window.showQuickPick` (convenience overloads)

```typescript
showQuickPick(items: readonly string[] | Thenable<readonly string[]>,
  options: QuickPickOptions & {canPickMany: true},
  token?: CancellationToken): Thenable<string[] | undefined>

showQuickPick(items: readonly string[] | Thenable<readonly string[]>,
  options?: QuickPickOptions,
  token?: CancellationToken): Thenable<string | undefined>

showQuickPick<T extends QuickPickItem>(
  items: readonly T[] | Thenable<readonly T[]>,
  options: QuickPickOptions & {canPickMany: true},
  token?: CancellationToken): Thenable<T[] | undefined>

showQuickPick<T extends QuickPickItem>(
  items: readonly T[] | Thenable<readonly T[]>,
  options?: QuickPickOptions,
  token?: CancellationToken): Thenable<T | undefined>
```

#### `QuickPick<T>` Interface

**Events**:
| Event | Type |
|-------|------|
| `onDidAccept` | `Event<void>` |
| `onDidChangeActive` | `Event<readonly T[]>` |
| `onDidChangeSelection` | `Event<readonly T[]>` |
| `onDidChangeValue` | `Event<string>` |
| `onDidHide` | `Event<void>` |
| `onDidTriggerButton` | `Event<QuickInputButton>` |
| `onDidTriggerItemButton` | `Event<QuickPickItemButtonEvent<T>>` |

**Properties**:
| Property | Type |
|----------|------|
| `activeItems` | `readonly T[]` |
| `busy` | `boolean` |
| `buttons` | `readonly QuickInputButton[]` |
| `canSelectMany` | `boolean` |
| `enabled` | `boolean` |
| `ignoreFocusOut` | `boolean` |
| `items` | `readonly T[]` |
| `keepScrollPosition` | `boolean` (optional) |
| `matchOnDescription` | `boolean` |
| `matchOnDetail` | `boolean` |
| `placeholder` | `string` |
| `prompt` | `string` |
| `selectedItems` | `readonly T[]` |
| `step` | `number` |
| `title` | `string` |
| `totalSteps` | `number` |
| `value` | `string` |

**Methods**: `dispose()`, `hide()`, `show()`

#### `QuickPickItem` Interface

| Property | Type | Required |
|----------|------|----------|
| `label` | `string` | Yes |
| `alwaysShow` | `boolean` | No |
| `buttons` | `readonly QuickInputButton[]` | No |
| `description` | `string` | No |
| `detail` | `string` | No |
| `iconPath` | `IconPath` | No |
| `kind` | `QuickPickItemKind` | No |
| `picked` | `boolean` | No |
| `resourceUri` | `Uri` | No |

#### `QuickPickOptions` Interface

| Property | Type |
|----------|------|
| `canPickMany` | `boolean` (optional) |
| `ignoreFocusOut` | `boolean` (optional) |
| `matchOnDescription` | `boolean` (optional) |
| `matchOnDetail` | `boolean` (optional) |
| `placeHolder` | `string` (optional) |
| `prompt` | `string` (optional) |
| `title` | `string` (optional) |
| `onDidSelectItem` | `(item: string \| QuickPickItem) => any` (event) |

#### `QuickPickItemKind` Enum

| Member | Value |
|--------|-------|
| `Separator` | `-1` |
| `Default` | `0` |

#### `QuickInputButton` Interface

| Property | Type |
|----------|------|
| `iconPath` | `IconPath` |
| `location` | `QuickInputButtonLocation` (optional) |
| `toggle` | `{checked: boolean}` (optional) |
| `tooltip` | `string` (optional) |

#### `QuickInputButtonLocation` Enum

| Member | Value |
|--------|-------|
| `Title` | `1` |
| `Inline` | `2` |
| `Input` | `3` |

#### `QuickInputButtons` (Predefined buttons)

| Static | Type |
|--------|------|
| `Back` | `QuickInputButton` |

---

### Terminal API

#### `window.createTerminal` (three overloads)

```typescript
createTerminal(name?: string, shellPath?: string,
  shellArgs?: string | readonly string[]): Terminal

createTerminal(options: TerminalOptions): Terminal

createTerminal(options: ExtensionTerminalOptions): Terminal
```

#### `Terminal` Interface

> An individual terminal instance within the integrated terminal.

**Properties**:
| Property | Type |
|----------|------|
| `creationOptions` | `Readonly<TerminalOptions \| ExtensionTerminalOptions>` |
| `exitStatus` | `TerminalExitStatus` |
| `name` | `string` |
| `processId` | `Thenable<number>` |
| `shellIntegration` | `TerminalShellIntegration` |
| `state` | `TerminalState` |

**Methods**:
| Method | Signature |
|--------|-----------|
| `dispose` | `(): void` |
| `hide` | `(): void` |
| `sendText` | `(text: string, shouldExecute?: boolean): void` |
| `show` | `(preserveFocus?: boolean): void` |

#### `TerminalOptions` Interface

| Property | Type |
|----------|------|
| `color` | `ThemeColor` (optional) |
| `cwd` | `string \| Uri` (optional) |
| `env` | `object` (optional) |
| `hideFromUser` | `boolean` (optional) |
| `iconPath` | `IconPath` (optional) |
| `isTransient` | `boolean` (optional) |
| `location` | `TerminalEditorLocationOptions \| TerminalSplitLocationOptions \| TerminalLocation` (optional) |
| `message` | `string` (optional) |
| `name` | `string` (optional) |
| `shellArgs` | `string \| string[]` (optional) |
| `shellIntegrationNonce` | `string` (optional) |
| `shellPath` | `string` (optional) |
| `strictEnv` | `boolean` (optional) |

#### `TerminalExitStatus` Interface

| Property | Type |
|----------|------|
| `code` | `number` |
| `reason` | `TerminalExitReason` |

#### `TerminalExitReason` Enum

| Member | Value |
|--------|-------|
| `Unknown` | `0` |
| `Shutdown` | `1` |
| `Process` | `2` |
| `User` | `3` |
| `Extension` | `4` |

#### `TerminalLocation` Enum

| Member | Value |
|--------|-------|
| `Panel` | `1` |
| `Editor` | `2` |

#### `TerminalDimensions` Interface

| Property | Type |
|----------|------|
| `columns` | `number` |
| `rows` | `number` |

#### Terminal-Related Registration Functions

```typescript
registerTerminalLinkProvider(provider: TerminalLinkProvider<TerminalLink>): Disposable

registerTerminalProfileProvider(id: string, provider: TerminalProfileProvider): Disposable
```

---

## 4. Other Useful Window Functions (Message Dialogs)

These are commonly used in extensions and demonstrated in the Hello World example:

```typescript
// Information message
showInformationMessage<T extends string>(message: string, ...items: T[]): Thenable<T | undefined>
showInformationMessage<T extends string>(message: string, options: MessageOptions, ...items: T[]): Thenable<T | undefined>
showInformationMessage<T extends MessageItem>(message: string, ...items: T[]): Thenable<T | undefined>
showInformationMessage<T extends MessageItem>(message: string, options: MessageOptions, ...items: T[]): Thenable<T | undefined>

// Warning message
showWarningMessage<T extends string>(message: string, ...items: T[]): Thenable<T | undefined>

// Error message
showErrorMessage<T extends string>(message: string, ...items: T[]): Thenable<T | undefined>

// Input box
showInputBox(options?: InputBoxOptions, token?: CancellationToken): Thenable<string | undefined>

// Open/save dialogs
showOpenDialog(options?: OpenDialogOptions): Thenable<Uri[] | undefined>
showSaveDialog(options?: SaveDialogOptions): Thenable<Uri | undefined>

// Progress
withProgress<R>(options: ProgressOptions, task: (progress: Progress<{increment: number, message: string}>, token: CancellationToken) => Thenable<R>): Thenable<R>
```

---

## 5. Additional Resources

- [Extension Guides](https://code.visualstudio.com/api/extension-guides/overview) -- Other samples illustrating different VS Code APIs or Contribution Points
- [UX Guidelines](https://code.visualstudio.com/api/ux-guidelines/overview) -- Best practices for designing extension UIs
- [Extension Manifest Reference](https://code.visualstudio.com/api/references/extension-manifest) -- Full reference for `package.json` fields
- [Activation Events Reference](https://code.visualstudio.com/api/references/activation-events) -- All available activation events
- [Contribution Points Reference](https://code.visualstudio.com/api/references/contribution-points) -- All available contribution points
- [VS Code API Reference](https://code.visualstudio.com/api/references/vscode-api) -- Complete API reference
- [Wrapping Up](https://code.visualstudio.com/api/get-started/wrapping-up) -- Next steps after getting started
- [Extension Capabilities Overview](https://code.visualstudio.com/api/extension-capabilities/overview) -- Helps find the right Contribution Point and VS Code API

---

## 6. Gaps and Limitations

- The official "Your First Extension" page is relatively concise and focuses on getting a Hello World running. It does not cover testing, publishing, bundling, or more advanced patterns.
- The `package.json` example in the Extension Anatomy page references older dependency versions (`typescript ^3.4.5`, `tslint ^5.16.0`). Modern scaffolding from `yo code` will produce updated versions (ESLint instead of TSLint, newer TypeScript).
- The window API reference page is extremely large (the full page text exceeds 160,000 characters). The QuickPick and Terminal sections documented above were extracted specifically; there are many more APIs in the `window` namespace not covered here.
- The `ExtensionTerminalOptions` interface (used for virtual/pseudoterminals) was referenced but not fully extracted in this research.
