# Partition 20: `.vscode/` — Dev-Time Configuration and Selfhost Development Tooling

## Summary

The `.vscode/` directory in the `vscode-atomic` repository is exclusively a developer workspace configuration layer. It contains editor settings, launch/task configurations, code snippets, saved searches, GitHub issue notebook queries, an MCP server manifest, and a suite of internal selfhost extensions used only when contributors are actively developing VS Code itself. None of this content ships in the released product or participates in the runtime architecture being analyzed.

## Detailed Findings

### Scope Determination

**Relevance to architectural research**: None. The task partition explicitly marks this directory as out of scope, and inspection of the directory tree confirms that assessment. Every artifact here is either a local editor preference file (`settings.json`, `launch.json`, `tasks.json`, `extensions.json`, `shared.code-snippets`) or a selfhost-only VS Code extension that exists solely to improve the contributor development experience inside this very repository.

### Contents Overview

The `.vscode/` tree breaks down into the following categories, all confirmed by direct directory inspection:

- **Root config files** (`settings.json`, `launch.json`, `tasks.json`, `extensions.json`, `mcp.json`, `shared.code-snippets`, `cglicenses.schema.json`): Standard VS Code workspace configuration. These control editor behavior, debugger launch profiles, build tasks, recommended extensions, MCP server connections, and editor snippets for contributors. They are not read at product runtime.

- **Saved searches** (`searches/ts36031.code-search`, `searches/no-any-casts.code-search`): Persisted VS Code search queries for contributor convenience (e.g., tracking a specific TypeScript error or auditing `any` casts). No architectural relevance.

- **GitHub Issues notebooks** (`notebooks/*.github-issues`): Saved GitHub Issues query notebooks (inbox, API issues, grooming, endgame, my-work, verification, papercuts, vscode-dev). These are query definitions for the GitHub Issues VS Code extension, used by the team to triage work. No runtime relevance.

- **Selfhost extensions** (`extensions/vscode-selfhost-test-provider`, `extensions/vscode-selfhost-import-aid`, `extensions/vscode-extras`, `extensions/vscode-pr-pinger`): Four private VS Code extensions installed only in the development instance of VS Code that the contributor is using to build VS Code. They provide test result surfacing, import assistance, npm version checking, and PR pinging workflows — all dev-time utilities.

### Selfhost Extensions (brief characterization)

| Extension | Purpose |
|---|---|
| `vscode-selfhost-test-provider` | Surfaces Mocha test results inside the VS Code Test Explorer when running unit/integration tests during development; includes V8 coverage wrangling and stack-trace parsing |
| `vscode-selfhost-import-aid` | Assists contributors with correct import path resolution within the monorepo |
| `vscode-extras` | Miscellaneous contributor tooling including an npm-up-to-date check feature |
| `vscode-pr-pinger` | Automates PR review reminders for the VS Code team |

None of these extensions are bundled into the VS Code distribution; they exist only inside `.vscode/extensions/` to be loaded when a contributor opens this workspace.

## Additional Resources

- `.vscode/launch.json` — `/home/norinlavaee/projects/vscode-atomic/.vscode/launch.json`
- `.vscode/settings.json` — `/home/norinlavaee/projects/vscode-atomic/.vscode/settings.json`
- `.vscode/tasks.json` — `/home/norinlavaee/projects/vscode-atomic/.vscode/tasks.json`
- `.vscode/extensions/vscode-selfhost-test-provider/src/extension.ts` — `/home/norinlavaee/projects/vscode-atomic/.vscode/extensions/vscode-selfhost-test-provider/src/extension.ts`

## Gaps or Limitations

No external research was applicable or required for this partition. The `.vscode/` directory is a self-contained, well-understood VS Code workspace convention; its contents are fully legible from local inspection alone, and no public documentation beyond the VS Code workspace configuration reference would add meaningful insight to an architectural analysis of the product itself.

---

The `.vscode/` directory is developer scaffolding that exists entirely to support contributors working inside this repository. Because it contains no shipped code, no runtime modules, and no artifacts that influence the behavior of the released VS Code product, it is correctly classified as out of scope for this architectural research effort, and no further investigation is warranted.
