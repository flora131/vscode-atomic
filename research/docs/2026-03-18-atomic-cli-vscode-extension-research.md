---
date: 2026-03-18 07:06:38 PDT
researcher: Claude Code
git_commit: f7b13184617f5ae87af497ce0664fcfbc11fd832
branch: main
repository: vscode-atomic
topic: "VS Code Extension for Atomic CLI: Agent Backend Selector + Terminal Spawner"
tags: [research, codebase, vscode-extension, quickpick, terminal-api, atomic-cli, coding-agents]
status: complete
last_updated: 2026-03-18
last_updated_by: Claude Code
---

# Research: VS Code Extension for Atomic CLI with Agent Backend Selection

## Research Question

How to create a VS Code extension for the atomic CLI that asks the user to select their coding agent backend through a menu selector, then spawns the CLI with their coding agent of choice in the integrated terminal.

## Summary

This research covers everything needed to build a VS Code extension that:
1. Registers a command (e.g., `atomic.startAgent`) accessible from the Command Palette
2. Displays a QuickPick menu listing available coding agent backends
3. Spawns the `atomic` CLI in VS Code's integrated terminal with the selected agent as an argument

The extension follows standard VS Code extension patterns: scaffold via `yo code`, declare commands in `package.json`, register a command handler in `activate()`, use `vscode.window.showQuickPick` for selection, and `vscode.window.createTerminal` + `sendText` to run the CLI.

---

## Detailed Findings

### 1. Atomic CLI Context

The `.atomic/settings.json` file in the repo references an external atomic tool:

```json
{
  "scm": "github",
  "version": 1,
  "lastUpdated": "2026-03-18T13:55:42.632Z",
  "$schema": "https://raw.githubusercontent.com/flora131/atomic/main/assets/settings.schema.json"
}
```

- The schema is hosted at `flora131/atomic` on GitHub
- The CLI appears to be an external tool (not built into this VS Code fork)
- The `scm` setting suggests it integrates with source control

### 2. Extension Scaffolding

**Official method** (from [Your First Extension](https://code.visualstudio.com/api/get-started/your-first-extension)):

```bash
npx --package yo --package generator-code -- yo code
```

Generator prompts for a TypeScript extension:
- Type: New Extension (TypeScript)
- Name, identifier, description
- Bundler: esbuild (recommended) or unbundled
- Package manager: npm

**Generated project structure:**

```
atomic-vscode/
├── .vscode/
│   ├── launch.json          # F5 to debug in Extension Development Host
│   └── tasks.json           # Build task (tsc or esbuild)
├── src/
│   └── extension.ts         # activate() and deactivate() exports
├── package.json             # Extension manifest
├── tsconfig.json            # TypeScript config
└── README.md
```

### 3. Extension Manifest (`package.json`)

Key fields for this extension:

```json
{
  "name": "atomic-vscode",
  "displayName": "Atomic CLI",
  "description": "Launch Atomic CLI with your preferred coding agent backend",
  "version": "0.0.1",
  "publisher": "your-publisher-id",
  "engines": {
    "vscode": "^1.74.0"
  },
  "categories": ["Other"],
  "activationEvents": [],
  "main": "./out/extension.js",
  "contributes": {
    "commands": [
      {
        "command": "atomic.startAgent",
        "title": "Start Agent",
        "category": "Atomic"
      }
    ]
  },
  "scripts": {
    "vscode:prepublish": "npm run compile",
    "compile": "tsc -p ./",
    "watch": "tsc -watch -p ./"
  },
  "devDependencies": {
    "@types/node": "^20.x",
    "@types/vscode": "^1.74.0",
    "typescript": "^5.x"
  }
}
```

**Important notes:**
- `activationEvents` can be left empty — since VS Code 1.74, contributing commands implicitly generates `onCommand` activation events
- The `command` string in `contributes.commands` must exactly match the `id` in `registerCommand()`
- `category` groups the command in the Command Palette (shows as "Atomic: Start Agent")

### 4. QuickPick API for Agent Selection

Two approaches exist. For this use case, the simple `showQuickPick` is sufficient.

#### Simple Approach: `vscode.window.showQuickPick`

```typescript
const selection = await vscode.window.showQuickPick(
    [
        { label: 'Claude', description: 'Anthropic Claude' },
        { label: 'GPT', description: 'OpenAI GPT' },
        { label: 'Copilot', description: 'GitHub Copilot' },
        { label: 'Gemini', description: 'Google Gemini' }
    ],
    {
        placeHolder: 'Select your coding agent backend',
        title: 'Atomic: Choose Agent'
    }
);
```

**`QuickPickItem` interface** (key properties):

| Property | Type | Description |
|---|---|---|
| `label` | `string` | Primary text (required) |
| `description` | `string` | Secondary text on same line |
| `detail` | `string` | Additional text on line below |
| `kind` | `QuickPickItemKind` | `Default` (selectable) or `Separator` |
| `picked` | `boolean` | Pre-selected (for `canPickMany`) |

**`QuickPickOptions`** (key properties):

| Property | Description |
|---|---|
| `title` | Title displayed at top |
| `placeHolder` | Placeholder text in input |
| `ignoreFocusOut` | Keep open when focus is lost |
| `matchOnDescription` | Filter includes description |

#### Advanced Approach: `vscode.window.createQuickPick`

Use this if you need dynamic items, separators, buttons, or multi-step wizards:

```typescript
const quickPick = vscode.window.createQuickPick();
quickPick.items = agents;
quickPick.placeholder = 'Select your coding agent backend';
quickPick.onDidAccept(() => {
    const selected = quickPick.selectedItems[0];
    // spawn terminal
    quickPick.dispose();
});
quickPick.onDidHide(() => quickPick.dispose());
quickPick.show();
```

**Source references for QuickPick patterns:**
- `src/vs/workbench/api/common/extHostQuickOpen.ts` — Extension host implementation
- `src/vs/platform/quickinput/common/quickInput.ts` — Core interfaces
- `extensions/git/src/commands.ts` — Git checkout QuickPick (real-world example)
- `src/vs/sessions/contrib/chat/browser/modePicker.ts` — Mode picker in agent sessions
- `src/vs/sessions/contrib/chat/browser/modelPicker.ts` — Model picker in agent sessions

### 5. Terminal API for Spawning the CLI

#### Creating a Terminal and Running a Command

```typescript
const terminal = vscode.window.createTerminal({
    name: `Atomic: ${selection.label}`,
    cwd: vscode.workspace.workspaceFolders?.[0]?.uri
});
terminal.show();
terminal.sendText(`atomic --agent ${selection.label.toLowerCase()}`);
```

#### `TerminalOptions` Interface

| Property | Type | Description |
|---|---|---|
| `name` | `string` | Terminal tab name |
| `shellPath` | `string` | Shell executable path |
| `shellArgs` | `string[] \| string` | Shell arguments |
| `cwd` | `string \| Uri` | Working directory |
| `env` | `{ [key: string]: string \| null }` | Environment variables (`null` deletes) |
| `hideFromUser` | `boolean` | Hidden terminal |
| `message` | `string` | Initial message shown in terminal |
| `iconPath` | `IconPath` | Terminal tab icon |
| `color` | `ThemeColor` | Terminal tab color |

#### `Terminal` Interface (key methods)

| Method | Description |
|---|---|
| `sendText(text, shouldExecute?)` | Send text to stdin. `shouldExecute` defaults to `true` (appends newline to execute) |
| `show(preserveFocus?)` | Show terminal panel. `preserveFocus` prevents stealing focus |
| `hide()` | Hide terminal from UI |
| `dispose()` | Close and destroy terminal |

#### Three `createTerminal` Overloads

```typescript
// 1. Simple
createTerminal(name?: string, shellPath?: string, shellArgs?: string[]): Terminal;

// 2. Full options
createTerminal(options: TerminalOptions): Terminal;

// 3. Extension-controlled (Pseudoterminal)
createTerminal(options: ExtensionTerminalOptions): Terminal;
```

For the atomic CLI, **Overload 2 (TerminalOptions)** is the right choice — it provides control over name, cwd, and environment without the complexity of Pseudoterminals.

#### Terminal Events

```typescript
vscode.window.onDidOpenTerminal(terminal => { /* ... */ });
vscode.window.onDidCloseTerminal(terminal => { /* ... */ });
vscode.window.onDidChangeActiveTerminal(terminal => { /* ... */ });
```

**Source references for Terminal patterns:**
- `src/vs/workbench/api/common/extHostTerminalService.ts` — Extension host terminal service
- `extensions/terminal-suggest/` — Built-in terminal extension (good scaffolding reference)
- `src/vs/sessions/contrib/terminal/browser/sessionsTerminalContribution.ts` — Sessions terminal

### 6. Command Registration Pattern

```typescript
import * as vscode from 'vscode';

export function activate(context: vscode.ExtensionContext) {
    const disposable = vscode.commands.registerCommand('atomic.startAgent', async () => {
        // 1. Show QuickPick
        // 2. Create terminal
        // 3. Send command
    });
    context.subscriptions.push(disposable);
}

export function deactivate() {}
```

**Key points:**
- The `command` ID in `registerCommand` must match `contributes.commands[].command` in `package.json`
- Always push disposables to `context.subscriptions` for automatic cleanup
- Use `async` handler for `await`-based QuickPick interaction
- `deactivate()` can be empty if no cleanup is needed

**Source references:**
- `src/vs/workbench/api/common/extHostCommands.ts` — Command registration internals
- `src/vs/workbench/api/common/extHost.api.impl.ts` — API namespace creation

### 7. Existing Patterns in This Repository

This vscode-atomic repo contains several highly relevant patterns:

#### Agent Session Pickers (QuickPick patterns for agent selection)
- `src/vs/sessions/contrib/chat/browser/modePicker.ts` — Mode selection picker
- `src/vs/sessions/contrib/chat/browser/modelPicker.ts` — Model selection picker
- `src/vs/sessions/contrib/chat/browser/sessionTargetPicker.ts` — Session target picker
- `src/vs/workbench/contrib/chat/browser/agentSessions/agentSessionsPicker.ts` — Agent session picker

#### Terminal Integration
- `src/vs/sessions/contrib/terminal/browser/sessionsTerminalContribution.ts` — Terminal in agent sessions
- `src/vs/workbench/contrib/terminal/browser/terminalProfileQuickpick.ts` — Terminal profile picker (combines QuickPick + Terminal)
- `extensions/terminal-suggest/` — Full built-in extension example with `package.json`, `src/`, build config

#### Agent/Coding Agent Configuration
- `product.json` — Contains `defaultChatAgent` config referencing `GitHub.copilot` and `GitHub.copilot-chat`
- Dependencies: `@anthropic-ai/sandbox-runtime`, `@github/copilot`, `@github/copilot-sdk`
- Proposed APIs: `vscode.proposed.remoteCodingAgents.d.ts`, `vscode.proposed.agentSessionsWorkspace.d.ts`

### 8. Extension Samples Reference

The `microsoft/vscode-extension-samples` repo contains two directly relevant samples:

#### quickinput-sample
- Demonstrates `showQuickPick`, `showInputBox`, multi-step wizard (`MultiStepInput` class), and dynamic file search
- Multi-step wizard uses `InputFlowAction` pattern for back/cancel/resume navigation
- **Key file**: `src/multiStepInput.ts` — reusable multi-step wizard pattern

#### terminal-sample
- 18 commands covering all Terminal API features
- Includes `selectTerminal()` helper that combines QuickPick + Terminal (extends `QuickPickItem` with `terminal` property)
- Terminal profile provider registration via `registerTerminalProfileProvider`
- **Key file**: `src/extension.ts` — comprehensive Terminal API examples

---

## Code References

### VS Code Extension API Internals
- `src/vs/workbench/api/common/extHostQuickOpen.ts` — QuickPick extension host implementation
- `src/vs/workbench/api/browser/mainThreadQuickOpen.ts` — QuickPick main thread bridge
- `src/vs/platform/quickinput/common/quickInput.ts` — Core QuickPick interfaces
- `src/vs/workbench/api/common/extHostTerminalService.ts` — Terminal extension host implementation
- `src/vs/workbench/api/browser/mainThreadTerminalService.ts` — Terminal main thread bridge
- `src/vs/workbench/api/common/extHostExtensionService.ts` — Extension activation lifecycle
- `src/vs/workbench/api/common/extHostCommands.ts` — Command registration
- `src/vs/workbench/api/common/extHost.api.impl.ts` — Full `vscode` namespace API

### Built-in Extension Examples
- `extensions/git/src/commands.ts` — Git checkout QuickPick (comprehensive real-world example)
- `extensions/terminal-suggest/package.json` — Terminal extension manifest
- `extensions/terminal-suggest/src/terminalSuggestMain.ts` — Terminal extension entry point

### Agent Sessions (This Repo)
- `src/vs/sessions/contrib/chat/browser/modePicker.ts` — Mode picker
- `src/vs/sessions/contrib/chat/browser/modelPicker.ts` — Model picker
- `src/vs/sessions/contrib/terminal/browser/sessionsTerminalContribution.ts` — Sessions terminal
- `src/vs/workbench/contrib/terminal/browser/terminalProfileQuickpick.ts` — Terminal profile picker

### Configuration
- `.atomic/settings.json` — Atomic CLI settings (schema at `flora131/atomic`)

---

## Architecture Documentation

### Extension Architecture Flow

```
User invokes "Atomic: Start Agent" (Cmd+Shift+P)
    |
    v
VS Code activates extension (implicit onCommand activation)
    |
    v
activate(context) called → registerCommand('atomic.startAgent', handler)
    |
    v
handler() executes:
    |
    ├── 1. vscode.window.showQuickPick(agents) → User selects agent
    |
    ├── 2. vscode.window.createTerminal({ name, cwd }) → Terminal created
    |
    ├── 3. terminal.show() → Terminal panel visible
    |
    └── 4. terminal.sendText('atomic --agent <selected>') → CLI spawned
```

### Key Design Decisions

1. **`showQuickPick` vs `createQuickPick`**: Use `showQuickPick` for simplicity — the agent selection is a single-step, static list. Use `createQuickPick` only if you need dynamic items, separators between agent categories, or multi-step configuration.

2. **`sendText` vs `shellPath`**: Use `sendText` to invoke the atomic CLI rather than setting `shellPath` to the atomic binary. This keeps the user's default shell (bash/zsh/etc.) and lets them interact with the terminal after the CLI runs.

3. **Activation**: Leave `activationEvents` empty and rely on implicit `onCommand` activation (VS Code 1.74+). This is the recommended modern approach.

4. **Disposables**: Push all command registrations to `context.subscriptions` for automatic cleanup on deactivation.

---

## Historical Context (from research/)

No prior research documents existed in this repository. The `research/` directory was created during this research session.

### Supporting Research Documents Created

- `research/docs/2026-03-18-vscode-extension-api-deepwiki.md` — Full QuickPick, Terminal, Activation, and Command Registration API research from DeepWiki (750 lines)
- `research/docs/2026-03-18-vscode-first-extension-guide.md` — Official "Your First Extension" guide content scraped via Playwright (526 lines)
- `research/docs/2026-03-18-vscode-extension-examples-deepwiki.md` — Practical extension examples from DeepWiki (843 lines)

---

## Related Research

- [VS Code Extension API](https://code.visualstudio.com/api/references/vscode-api)
- [Your First Extension](https://code.visualstudio.com/api/get-started/your-first-extension)
- [Extension Anatomy](https://code.visualstudio.com/api/get-started/extension-anatomy)
- [QuickPick Sample](https://github.com/microsoft/vscode-extension-samples/tree/main/quickinput-sample)
- [Terminal Sample](https://github.com/microsoft/vscode-extension-samples/tree/main/terminal-sample)
- [Atomic CLI Schema](https://raw.githubusercontent.com/flora131/atomic/main/assets/settings.schema.json)

---

## Open Questions

1. **Atomic CLI invocation format**: What is the exact CLI command and argument format for specifying the agent backend? (e.g., `atomic --agent claude`, `atomic start --backend gpt`, etc.) The `.atomic/settings.json` schema at `flora131/atomic` should document this.

2. **Available agent backends**: What are the specific agent backends supported by the atomic CLI? The extension should either hardcode the known list or dynamically discover available agents (perhaps from `atomic --list-agents` or from the settings schema).

3. **Extension distribution**: Will this be published to the VS Code Marketplace, distributed as a `.vsix`, or bundled as a built-in extension in this vscode-atomic fork?

4. **Configuration settings**: Should the extension contribute VS Code settings (via `contributes.configuration`) for default agent, custom CLI path, or additional CLI arguments?

5. **Terminal reuse**: Should the extension reuse an existing "Atomic" terminal if one is already open, or always create a new one?
