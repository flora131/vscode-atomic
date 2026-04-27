(no external research applicable)

<justification>
The `extensions/npm/` partition (14 files, 2,372 LOC) provides npm script task integration for VS Code. Its only external runtime dependency is the `which` npm package, which is a trivial ~50-line utility used solely to locate the `npm` binary on the host PATH. It has no API surface relevant to porting decisions.

All substantive logic in the extension is expressed through the VS Code extension host APIs:

- `vscode.TaskDefinition` / `vscode.TaskProvider` — the task registration contract
- `vscode.ShellExecution` — the mechanism for running npm scripts as shell commands
- `vscode.workspace`, `vscode.window`, `vscode.commands` — standard VS Code host surface

These APIs are defined entirely within the vscode-dts type declarations that live inside this repository (typically `src/vscode-dts/vscode.d.ts`). They are not external library contracts that require fetching third-party documentation.

From a Tauri/Rust porting perspective, the relevant question for this extension is: "how do you replace the VS Code Tasks/ShellExecution API with an equivalent in a Tauri-based IDE?" That is an architectural question about the target platform's internal task/process-execution subsystem — not a question that external npm or Rust crate documentation for `which` (or its Rust equivalent, `which = "6.x"` on crates.io) can meaningfully inform at this stage of scoping. The `which` crate's API is a one-liner (`which::which("npm")`) and introduces no porting complexity.

Therefore no external library or framework documentation is central to answering the porting question for this partition.
</justification>
