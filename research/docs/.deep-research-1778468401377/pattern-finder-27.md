# Pattern Finder: VS Code npm Extension (Partition 27)

## Scope
Extensions: `extensions/npm/` (14 files, ~2,372 LOC)

## Task Provider Registration Pattern

#### Pattern: Task Provider Registration via Extension Activation
**Where:** `extensions/npm/src/npmMain.ts:124-141`
**What:** Registers an npm task provider that discovers and provides npm scripts as executable tasks.

```typescript
let taskProvider: NpmTaskProvider;
function registerTaskProvider(context: vscode.ExtensionContext): vscode.Disposable | undefined {
	if (vscode.workspace.workspaceFolders) {
		const watcher = vscode.workspace.createFileSystemWatcher('**/package.json');
		watcher.onDidChange((_e) => invalidateScriptCaches());
		watcher.onDidDelete((_e) => invalidateScriptCaches());
		watcher.onDidCreate((_e) => invalidateScriptCaches());
		context.subscriptions.push(watcher);

		const workspaceWatcher = vscode.workspace.onDidChangeWorkspaceFolders((_e) => invalidateScriptCaches());
		context.subscriptions.push(workspaceWatcher);

		taskProvider = new NpmTaskProvider(context);
		const disposable = vscode.tasks.registerTaskProvider('npm', taskProvider);
		context.subscriptions.push(disposable);
		return disposable;
	}
	return undefined;
}
```

**Variations / call-sites:** `npmMain.ts:36` (called from activate function)

---

## Task Provider Implementation Pattern

#### Pattern: TaskProvider Interface Implementation with Task Discovery
**Where:** `extensions/npm/src/tasks.ts:46-87`
**What:** Implements VS Code's TaskProvider interface with async task discovery and resolution.

```typescript
export class NpmTaskProvider implements TaskProvider {

	constructor(private context: ExtensionContext) {
	}

	get tasksWithLocation(): Promise<ITaskWithLocation[]> {
		return provideNpmScripts(this.context, false);
	}

	public async provideTasks() {
		const tasks = await provideNpmScripts(this.context, true);
		return tasks.map(task => task.task);
	}

	public async resolveTask(_task: Task): Promise<Task | undefined> {
		const npmTask = _task.definition.script;
		if (npmTask) {
			const kind = _task.definition as INpmTaskDefinition;
			// ... task resolution logic
			return task;
		}
		return undefined;
	}
}
```

**Variations / call-sites:** `npmMain.ts:145` (instantiated), `npmView.ts:144` (used in TreeDataProvider)

---

## Shell Execution Pattern

#### Pattern: Task Creation with ShellExecution
**Where:** `extensions/npm/src/tasks.ts:334-360`
**What:** Creates executable npm script tasks with shell execution, proper argument escaping, and task grouping.

```typescript
export async function createScriptRunnerTask(context: ExtensionContext, script: string, folder: WorkspaceFolder, packageJsonUri: Uri, scriptValue?: string, showWarning?: boolean): Promise<Task> {
	const kind: INpmTaskDefinition = { type: 'npm', script };

	const relativePackageJson = getRelativePath(folder.uri, packageJsonUri);
	if (relativePackageJson.length && !kind.path) {
		kind.path = relativePackageJson.substring(0, relativePackageJson.length - 1);
	}
	const taskName = getTaskName(script, relativePackageJson);
	const cwd = path.dirname(packageJsonUri.fsPath);
	const args = await getRunScriptCommand(script, folder.uri, context, showWarning);
	const scriptRunner = args.shift()!;
	const task = new Task(kind, folder, taskName, 'npm', new ShellExecution(scriptRunner, escapeCommandLine(args), { cwd: cwd }));
	task.detail = scriptValue;

	const lowerCaseTaskName = script.toLowerCase();
	if (isBuildTask(lowerCaseTaskName)) {
		task.group = TaskGroup.Build;
	} else if (isTestTask(lowerCaseTaskName)) {
		task.group = TaskGroup.Test;
	} else if (canHavePrePostScript(lowerCaseTaskName)) {
		task.group = TaskGroup.Clean;
	} else if (scriptValue && isDebugScript(scriptValue)) {
		task.group = TaskGroup.Rebuild;
	}
	return task;
}
```

**Variations / call-sites:** `tasks.ts:371-387` (installation task variant), `commands.ts:25`, `npmView.ts:185`, `scriptHover.ts:117`

---

## Command Registration Pattern

#### Pattern: Command Registration with Multiple Commands
**Where:** `extensions/npm/src/npmMain.ts:56-77`
**What:** Multiple commands registered via vscode.commands.registerCommand, some returning contextual values.

```typescript
context.subscriptions.push(vscode.commands.registerCommand('npm.runSelectedScript', runSelectedScript));

if (await hasPackageJson()) {
	vscode.commands.executeCommand('setContext', 'npm:showScriptExplorer', true);
}

context.subscriptions.push(vscode.commands.registerCommand('npm.runScriptFromFolder', selectAndRunScriptFromFolder));
context.subscriptions.push(vscode.commands.registerCommand('npm.refresh', () => {
	invalidateScriptCaches();
}));
context.subscriptions.push(vscode.commands.registerCommand('npm.scriptRunner', (args) => {
	if (args instanceof vscode.Uri) {
		return getScriptRunner(args, context, true);
	}
	return '';
}));
context.subscriptions.push(vscode.commands.registerCommand('npm.packageManager', (args) => {
	if (args instanceof vscode.Uri) {
		return getPackageManager(args, context, true);
	}
	return '';
}));
```

**Variations / call-sites:** `npmView.ts:147-150` (tree explorer commands), `scriptHover.ts:37-38` (hover provider commands)

---

## Child Process Execution Pattern

#### Pattern: Process Execution via child_process.execFile
**Where:** `extensions/npm/src/features/packageJSONContribution.ts:303-323`
**What:** Async npm command execution with cwd resolution, environment variable setup for corepack compatibility, and platform-specific shell handling.

```typescript
private async runNpmCommand(npmCommandPath: string, args: string[], resource: Uri | undefined): Promise<string | undefined> {
	const cp = await import('child_process');
	return new Promise((resolve, _reject) => {
		const cwd = resource && resource.scheme === 'file' ? dirname(resource.fsPath) : undefined;

		// corepack npm wrapper would automatically update package.json. disable that behavior.
		const env = { ...process.env, COREPACK_ENABLE_AUTO_PIN: '0', COREPACK_ENABLE_PROJECT_SPEC: '0' };
		let options: cp.ExecFileOptions = { cwd, env };
		let commandPath: string = npmCommandPath;
		if (process.platform === 'win32') {
			options = { cwd, env, shell: true };
			commandPath = `"${npmCommandPath}"`;
		}
		cp.execFile(commandPath, args, options, (error, stdout) => {
			resolve(error ? undefined : stdout);
		});
	});
}
```

**Variations / call-sites:** `packageJSONContribution.ts:325-344` (npmView variant), `packageJSONContribution.ts:369-381` (npmListInstalledVersion variant)

---

## JSON Parsing Pattern for Script Discovery

#### Pattern: Declarative JSON Visitor Pattern with jsonc-parser
**Where:** `extensions/npm/src/readScripts.ts:21-73`
**What:** Uses jsonc-parser's visitor pattern to traverse package.json AST and extract npm scripts with position tracking.

```typescript
export const readScripts = (document: TextDocument, buffer = document.getText()): INpmScriptInfo | undefined => {
	let start: Position | undefined;
	let end: Position | undefined;
	let inScripts = false;
	let buildingScript: { name: string; nameRange: Range } | void;
	let level = 0;

	const scripts: INpmScriptReference[] = [];
	const visitor: JSONVisitor = {
		onError() { /* no-op */ },
		onObjectBegin() { level++; },
		onObjectEnd(offset) {
			if (inScripts) {
				end = document.positionAt(offset);
				inScripts = false;
			}
			level--;
		},
		onLiteralValue(value: unknown, offset: number, length: number) {
			if (buildingScript && typeof value === 'string') {
				scripts.push({
					...buildingScript,
					value,
					valueRange: new Range(document.positionAt(offset), document.positionAt(offset + length)),
				});
				buildingScript = undefined;
			}
		},
		onObjectProperty(property: string, offset: number, length: number) {
			if (level === 1 && property === 'scripts') {
				inScripts = true;
				start = document.positionAt(offset);
			} else if (inScripts) {
				buildingScript = {
					name: property,
					nameRange: new Range(document.positionAt(offset), document.positionAt(offset + length))
				};
			}
		},
	};

	visit(buffer, visitor);

	if (start === undefined) {
		return undefined;
	}

	return { location: new Location(document.uri, new Range(start, end ?? start)), scripts };
};
```

**Variations / call-sites:** `npmView.ts:164`, `scriptHover.ts:60`, `npmScriptLens.ts:69`

---

## Hover Provider Pattern

#### Pattern: Hover Provider with Markdown Commands
**Where:** `extensions/npm/src/scriptHover.ts:33-74, 112-119`
**What:** Implements HoverProvider with markdown-based command links and hover lifecycle management.

```typescript
export class NpmScriptHoverProvider implements HoverProvider {
	private enabled: boolean;

	constructor(private context: ExtensionContext) {
		context.subscriptions.push(commands.registerCommand('npm.runScriptFromHover', this.runScriptFromHover, this));
		context.subscriptions.push(commands.registerCommand('npm.debugScriptFromHover', this.debugScriptFromHover, this));
		context.subscriptions.push(workspace.onDidChangeTextDocument((e) => {
			invalidateHoverScriptsCache(e.document);
		}));

		const isEnabled = () => workspace.getConfiguration('npm').get<boolean>('scriptHover', true);
		this.enabled = isEnabled();
		context.subscriptions.push(workspace.onDidChangeConfiguration((e) => {
			if (e.affectsConfiguration('npm.scriptHover')) {
				this.enabled = isEnabled();
			}
		}));
	}

	public provideHover(document: TextDocument, position: Position, _token: CancellationToken): ProviderResult<Hover> {
		if (!this.enabled) {
			return;
		}

		let hover: Hover | undefined = undefined;

		if (!cachedDocument || cachedDocument.fsPath !== document.uri.fsPath) {
			cachedScripts = readScripts(document);
			cachedDocument = document.uri;
		}

		cachedScripts?.scripts.forEach(({ name, nameRange }) => {
			if (nameRange.contains(position)) {
				const contents: MarkdownString = new MarkdownString();
				contents.isTrusted = true;
				contents.appendMarkdown(this.createRunScriptMarkdown(name, document.uri));
				contents.appendMarkdown(this.createDebugScriptMarkdown(name, document.uri));
				hover = new Hover(contents);
			}
		});
		return hover;
	}

	public async runScriptFromHover(args: any) {
		const script = args.script;
		const documentUri = args.documentUri;
		const folder = workspace.getWorkspaceFolder(documentUri);
		if (folder) {
			const task = await createScriptRunnerTask(this.context, script, folder, documentUri);
			await tasks.executeTask(task);
		}
	}
}
```

**Variations / call-sites:** `npmMain.ts:153-164` (registration), `npmMain.ts:54` (registration call)

---

## Configuration-Driven Package Manager Detection

#### Pattern: Auto-Detection with Configuration Fallback
**Where:** `extensions/npm/src/tasks.ts:130-166`
**What:** Detects package manager from lock files or configuration, with multi-PM detection warnings.

```typescript
export async function getScriptRunner(folder: Uri, context?: ExtensionContext, showWarning?: boolean): Promise<string> {
	let scriptRunner = workspace.getConfiguration('npm', folder).get<string>('scriptRunner', 'npm');

	if (scriptRunner === 'auto') {
		scriptRunner = await detectPackageManager(folder, context, showWarning);
	}

	return scriptRunner;
}

export async function detectPackageManager(folder: Uri, extensionContext?: ExtensionContext, showWarning: boolean = false): Promise<string> {
	const { name, multipleLockFilesDetected: multiplePMDetected } = await findPreferredPM(folder.fsPath);
	const neverShowWarning = 'npm.multiplePMWarning.neverShow';
	if (showWarning && multiplePMDetected && extensionContext && !extensionContext.globalState.get<boolean>(neverShowWarning)) {
		const multiplePMWarning = l10n.t('Using {0} as the preferred package manager. Found multiple lockfiles for {1}...', name, folder.fsPath);
		const neverShowAgain = l10n.t("Do not show again");
		const learnMore = l10n.t("Learn more");
		window.showInformationMessage(multiplePMWarning, learnMore, neverShowAgain).then(result => {
			switch (result) {
				case neverShowAgain: extensionContext.globalState.update(neverShowWarning, true); break;
				case learnMore: env.openExternal(Uri.parse('https://docs.npmjs.com/cli/v9/configuring-npm/package-lock-json'));
			}
		});
	}

	return name;
}
```

**Variations / call-sites:** `tasks.ts:140-148` (getPackageManager variant), `npmView.ts:155` (usage in explorer)

---

## Tree Data Provider Pattern

#### Pattern: Hierarchical Tree Data Provider with Filtering
**Where:** `extensions/npm/src/npmView.ts:138-334`
**What:** Custom TreeDataProvider that builds hierarchical task tree from flat task list with exclusion pattern support.

```typescript
export class NpmScriptsTreeDataProvider implements TreeDataProvider<TreeItem> {
	private taskTree: TaskTree | null = null;
	private extensionContext: ExtensionContext;
	private _onDidChangeTreeData: EventEmitter<TreeItem | null> = new EventEmitter<TreeItem | null>();
	readonly onDidChangeTreeData: Event<TreeItem | null> = this._onDidChangeTreeData.event;

	constructor(private context: ExtensionContext, public taskProvider: NpmTaskProvider) {
		const subscriptions = context.subscriptions;
		this.extensionContext = context;
		subscriptions.push(commands.registerCommand('npm.runScript', this.runScript, this));
		subscriptions.push(commands.registerCommand('npm.debugScript', this.debugScript, this));
		subscriptions.push(commands.registerCommand('npm.openScript', this.openScript, this));
		subscriptions.push(commands.registerCommand('npm.runInstall', this.runInstall, this));
	}

	private async runScript(script: NpmScript) {
		await detectPackageManager(script.getFolder().uri, this.context, true);
		tasks.executeTask(script.task);
	}

	async getChildren(element?: TreeItem): Promise<TreeItem[]> {
		if (!this.taskTree) {
			const taskItems = await this.taskProvider.tasksWithLocation;
			if (taskItems) {
				const taskTree = this.buildTaskTree(taskItems);
				this.taskTree = this.sortTaskTree(taskTree);
				if (this.taskTree.length === 0) {
					let message = l10n.t("No scripts found.");
					if (!isAutoDetectionEnabled()) {
						message = l10n.t('The setting "npm.autoDetect" is "off".');
					}
					this.taskTree = [new NoScripts(message)];
				}
			}
		}
		if (element instanceof Folder) {
			return element.packages;
		}
		if (element instanceof PackageJSON) {
			return element.scripts;
		}
		if (!element) {
			if (this.taskTree) {
				return this.taskTree;
			}
		}
		return [];
	}

	private buildTaskTree(tasks: ITaskWithLocation[]): TaskTree {
		const folders: Map<String, Folder> = new Map();
		const packages: Map<String, PackageJSON> = new Map();

		tasks.forEach(each => {
			const location = each.location;
			if (location && !excludeConfig.has(location.uri.toString())) {
				const regularExpressionsSetting = workspace.getConfiguration('npm', location.uri).get<string[]>('scriptExplorerExclude', []);
				excludeConfig.set(location.uri.toString(), regularExpressionsSetting?.map(value => RegExp(value)));
			}
			const regularExpressions = (location && excludeConfig.has(location.uri.toString())) ? excludeConfig.get(location.uri.toString()) : undefined;

			if (regularExpressions && regularExpressions.some((regularExpression) => (<INpmTaskDefinition>each.task.definition).script.match(regularExpression))) {
				return; // skip excluded scripts
			}
			// ... build tree structure
		});
		return [...folders.values()];
	}
}
```

**Variations / call-sites:** `npmMain.ts:143-150` (tree view creation and registration)

---

## CodeLens Provider Pattern

#### Pattern: Configuration-Driven CodeLens with Debug Integration
**Where:** `extensions/npm/src/npmScriptLens.ts:32-107`
**What:** CodeLens provider that conditionally displays debug links based on configuration location setting.

```typescript
export class NpmScriptLensProvider implements CodeLensProvider, Disposable {
	private lensLocation = getFreshLensLocation();
	private readonly changeEmitter = new EventEmitter<void>();
	private subscriptions: Disposable[] = [];

	public readonly onDidChangeCodeLenses = this.changeEmitter.event;

	constructor() {
		this.subscriptions.push(
			this.changeEmitter,
			workspace.onDidChangeConfiguration(evt => {
				if (evt.affectsConfiguration(Constants.ConfigKey)) {
					this.lensLocation = getFreshLensLocation();
					this.changeEmitter.fire();
				}
			}),
			languages.registerCodeLensProvider(
				{
					language: 'json',
					pattern: '**/package.json',
				},
				this,
			)
		);
	}

	public async provideCodeLenses(document: TextDocument): Promise<CodeLens[]> {
		if (this.lensLocation === 'never') {
			return [];
		}

		const tokens = readScripts(document);
		if (!tokens) {
			return [];
		}

		const title = '$(debug-start) ' + l10n.t("Debug");
		const cwd = path.dirname(document.uri.fsPath);
		if (this.lensLocation === 'top') {
			return [
				new CodeLens(
					tokens.location.range,
					{
						title,
						command: 'extension.js-debug.npmScript',
						arguments: [cwd],
					},
				),
			];
		}

		if (this.lensLocation === 'all') {
			const folder = Uri.joinPath(document.uri, '..');
			return Promise.all(tokens.scripts.map(
				async ({ name, nameRange }) => {
					const runScriptCommand = await getRunScriptCommand(name, folder);
					return new CodeLens(
						nameRange,
						{
							title,
							command: 'extension.js-debug.createDebuggerTerminal',
							arguments: [runScriptCommand.join(' '), workspace.getWorkspaceFolder(document.uri), { cwd }],
						},
					);
				},
			));
		}

		return [];
	}
}
```

**Variations / call-sites:** `npmMain.ts:78` (subscription push in activate)

---

## Extension Activation Lifecycle

#### Pattern: Full Extension Activation with Async Setup and Subscriptions
**Where:** `extensions/npm/src/npmMain.ts:26-103`
**What:** Comprehensive activation showing configuration listeners, provider registration, command setup, and subscription management.

```typescript
export async function activate(context: vscode.ExtensionContext): Promise<void> {
	configureHttpRequest();
	context.subscriptions.push(vscode.workspace.onDidChangeConfiguration(e => {
		if (e.affectsConfiguration('http.proxy') || e.affectsConfiguration('http.proxyStrictSSL')) {
			configureHttpRequest();
		}
	}));

	const npmCommandPath = await getNPMCommandPath();
	context.subscriptions.push(addJSONProviders(httpRequest.xhr, npmCommandPath));
	registerTaskProvider(context);

	treeDataProvider = registerExplorer(context);

	context.subscriptions.push(vscode.workspace.onDidChangeConfiguration((e) => {
		if (e.affectsConfiguration('npm.exclude') || e.affectsConfiguration('npm.autoDetect') || e.affectsConfiguration('npm.scriptExplorerExclude') || e.affectsConfiguration('npm.runSilent') || e.affectsConfiguration('npm.packageManager') || e.affectsConfiguration('npm.scriptRunner')) {
			invalidateTasksCache();
			if (treeDataProvider) {
				treeDataProvider.refresh();
			}
		}
		if (e.affectsConfiguration('npm.scriptExplorerAction')) {
			if (treeDataProvider) {
				treeDataProvider.refresh();
			}
		}
	}));

	registerHoverProvider(context);

	context.subscriptions.push(vscode.commands.registerCommand('npm.runSelectedScript', runSelectedScript));

	if (await hasPackageJson()) {
		vscode.commands.executeCommand('setContext', 'npm:showScriptExplorer', true);
	}

	// ... more command registrations

	context.subscriptions.push(vscode.window.registerTerminalQuickFixProvider('ms-vscode.npm-command', {
		provideTerminalQuickFixes({ outputMatch }) {
			if (!outputMatch) {
				return;
			}

			const lines = outputMatch.regexMatch[1];
			const fixes: vscode.TerminalQuickFixTerminalCommand[] = [];
			for (const line of lines.split('\n')) {
				const begin = line.indexOf('npm', 1);
				if (begin === -1) {
					continue;
				}

				const end = line.lastIndexOf('#');
				fixes.push({ terminalCommand: line.slice(begin, end === -1 ? undefined : end - 1) });
			}

			return fixes;
		},
	}));
}
```

---

## Summary

The `extensions/npm/` codebase demonstrates integrated patterns for a complete npm script management system:

1. **Task Provider Pattern**: Full TaskProvider implementation with file watching, caching, and dynamic task discovery
2. **Process Execution**: Platform-aware subprocess execution with environment variable configuration
3. **JSON Parsing**: Declarative visitor-based parsing for extracting npm scripts with position tracking
4. **Command Integration**: Multi-level command registration (UI commands, hover commands, explorer commands)
5. **Tree Exploration**: Hierarchical TreeDataProvider with dynamic filtering and exclusion patterns
6. **Configuration-Driven Behavior**: Extensive use of workspace configuration with reactive updates
7. **Hover/CodeLens Integration**: Rich editor integration with markdown-based command links
8. **Package Manager Detection**: Auto-detection with fallback and multi-PM warnings

**Key architectural insights for porting:**
- Task execution uses `ShellExecution` abstraction (high-level API, not direct child_process)
- Configuration drives behavior (package manager, auto-detection, script exclusions, hover settings)
- Script discovery decoupled from execution via TaskProvider interface
- JSON parsing uses declarative visitors, not regex
- Process spawning wrapped in Promise-based utilities
- Subscription-based lifecycle management for all resources
