# Pattern Analysis: vscode-colorize-perf-tests

## Summary

The `extensions/vscode-colorize-perf-tests/` directory is a pure test fixture extension containing performance benchmarking tests for VS Code's tokenization/colorization system. This codebase is **not relevant to a Tauri/Rust port** as it consists exclusively of:

1. Mocha test harnesses
2. TypeScript/JavaScript test fixtures for performance benchmarking
3. Launch configuration for VSCode extension host debugging
4. Large synthetic code files used only for performance testing

The test fixtures include a Game of Life simulation, TreeView UI tests, and TypeScript compiler internals—all used only to exercise tokenization performance between TextMate and TreeSitter. These are testing utilities, not core IDE functionality.

---

## Patterns Found

#### Pattern: Extension Activation Hooks
**Where:** `extensions/vscode-colorize-perf-tests/src/colorizerTestMain.ts:1-32`
**What:** Test extension configured with `onLanguage:json` activation and Mocha test runner setup for CI/CD environments.
```typescript
// Extension package metadata
"activationEvents": [
  "onLanguage:json"
],
"main": "./out/colorizerTestMain",
"engines": {
  "vscode": "*"
}

// Test runner configuration
const options: import('mocha').MochaOptions = {
  ui: 'tdd',
  color: true,
  timeout: 60000
};

if (process.env.BUILD_ARTIFACTSTAGINGDIRECTORY || process.env.GITHUB_WORKSPACE) {
  options.reporter = 'mocha-multi-reporters';
  options.reporterOptions = {
    reporterEnabled: 'spec, mocha-junit-reporter',
    // ... XML output for Azure Pipelines / GitHub Actions
  };
}
```

**Variations / call-sites:** The extension uses environment variable detection for CI/CD platforms (Azure Pipelines: `BUILD_ARTIFACTSTAGINGDIRECTORY`, GitHub Actions: `GITHUB_WORKSPACE`).

---

#### Pattern: Performance Measurement via Command Execution
**Where:** `extensions/vscode-colorize-perf-tests/src/colorizer.test.ts:95-108`
**What:** Generic command runner that executes VS Code commands repeatedly and collects timing data for best/worst comparisons.
```typescript
async function runCommand<TimesType = TreeSitterTimes | TextMateTimes>(
  command: string, 
  file: Uri, 
  times: number
): Promise<TimesType[]> {
  const results: TimesType[] = [];
  for (let i = 0; i < times; i++) {
    results.push(await commands.executeCommand(command, file));
  }
  return results;
}

async function doTest(file: Uri, times: number) {
  const treeSitterResults = await runCommand<TreeSitterTimes>(
    '_workbench.colorizeTreeSitterTokens', 
    file, 
    times
  );
  const textMateResults = await runCommand<TextMateTimes>(
    '_workbench.colorizeTextMateTokens', 
    file, 
    times
  );
}
```

**Variations / call-sites:** Tests invoke two internal commands: `_workbench.colorizeTreeSitterTokens` and `_workbench.colorizeTextMateTokens`, comparing parse/capture/metadata times (TreeSitter) vs. tokenizeTime (TextMate).

---

#### Pattern: Test Suite Lifecycle with Configuration Changes
**Where:** `extensions/vscode-colorize-perf-tests/src/colorizer.test.ts:127-146`
**What:** Mocha test suite that modifies VS Code configuration before running tests and restores it after.
```typescript
suite('Tokenization Performance', () => {
  const testPath = normalize(join(__dirname, '../test'));
  const fixturesPath = join(testPath, 'colorize-fixtures');
  let originalSettingValue: any;

  suiteSetup(async function () {
    originalSettingValue = workspace.getConfiguration('editor')
      .get('experimental.preferTreeSitter');
    await workspace.getConfiguration('editor').update(
      'experimental.preferTreeSitter', 
      ['typescript'], 
      ConfigurationTarget.Global
    );
  });
  
  suiteTeardown(async function () {
    await workspace.getConfiguration('editor').update(
      'experimental.preferTreeSitter', 
      originalSettingValue, 
      ConfigurationTarget.Global
    );
  });

  for (const fixture of fs.readdirSync(fixturesPath)) {
    test(`Full file colorize: ${fixture}`, async function () {
      await commands.executeCommand('workbench.action.closeAllEditors');
      await doTest(Uri.file(join(fixturesPath, fixture)), 6);
    });
  }
});
```

**Variations / call-sites:** Dynamically discovers test fixtures from disk and generates test cases. Uses `ConfigurationTarget.Global` to set editor preferences.

---

#### Pattern: Performance Metrics Aggregation
**Where:** `extensions/vscode-colorize-perf-tests/src/colorizer.test.ts:22-83`
**What:** Function that tracks best/worst performance across multiple runs, excluding the first run from worst-case analysis.
```typescript
interface BestsAndWorsts {
  bestParse?: number;
  bestCapture?: number;
  bestMetadata?: number;
  bestCombined: number;
  worstParse?: number;
  worstCapture?: number;
  worstMetadata?: number;
  worstCombined: number;
}

function findBestsAndWorsts(
  results: { parseTime?: number; captureTime?: number; 
             metadataTime?: number; tokenizeTime?: number }[]
): BestsAndWorsts {
  // ... tracks separate metrics for TreeSitter (parse/capture/metadata) 
  //     vs TextMate (tokenizeTime)
  // ... excludes first result (i === 0) from worst-case comparisons
}
```

**Variations / call-sites:** Distinguishes two tokenizer implementations via presence of fields; generates markdown-formatted output table for results.

---

## Non-Portable Aspects

- **Internal Command Dependencies:** Tests depend on `_workbench.colorizeTreeSitterTokens` and `_workbench.colorizeTextMateTokens`—private VS Code APIs that would need equivalent Rust implementations
- **ExtensionHost Protocol:** Mocha tests run in the VS Code ExtensionHost process; would require Tauri equivalent
- **Configuration API:** Uses `workspace.getConfiguration()` and `ConfigurationTarget.Global`—VS Code's settings system
- **Test Fixtures:** Game of Life, TypeScript compiler internals, and TreeView UI code are large synthetic files for testing, not production code

## Conclusion

This extension is **test infrastructure only** and not part of the core IDE being ported. The patterns show how VS Code's ExtensionHost-based testing works, but have no bearing on porting core functionality to Tauri/Rust. The performance comparison between TextMate and TreeSitter tokenizers is VS Code-specific and would be fully rearchitected in a Rust implementation.
