# Configuration Editing Extension - File Locator

## Scope
`extensions/configuration-editing/` (11 files, ~1,450 LOC)

---

### Implementation
- `extensions/configuration-editing/src/configurationEditingMain.ts` - Main extension entry point
- `extensions/configuration-editing/src/settingsDocumentHelper.ts` - Settings document utilities
- `extensions/configuration-editing/src/extensionsProposals.ts` - Extension proposals handling
- `extensions/configuration-editing/src/importExportProfiles.ts` - Profile import/export functionality
- `extensions/configuration-editing/src/browser/net.ts` - Browser-side network utilities
- `extensions/configuration-editing/src/node/net.ts` - Node-side network utilities

### Tests
- `extensions/configuration-editing/src/test/index.ts` - Test suite entry point
- `extensions/configuration-editing/src/test/completion.test.ts` - Completion tests

### Types / Interfaces
- `extensions/configuration-editing/src/typings/ref.d.ts` - TypeScript type definitions

### Configuration
- `extensions/configuration-editing/package.json` - Extension manifest and dependencies
- `extensions/configuration-editing/package-lock.json` - Locked dependency versions
- `extensions/configuration-editing/package.nls.json` - Localization/translation strings
- `extensions/configuration-editing/tsconfig.json` - TypeScript configuration
- `extensions/configuration-editing/tsconfig.browser.json` - Browser-specific TypeScript config
- `extensions/configuration-editing/.npmrc` - NPM configuration
- `extensions/configuration-editing/.vscodeignore` - Packaging ignore rules
- `extensions/configuration-editing/esbuild.mts` - Node build configuration
- `extensions/configuration-editing/esbuild.browser.mts` - Browser build configuration

### Examples / Fixtures
- `extensions/configuration-editing/schemas/devContainer.vscode.schema.json` - Dev container schema
- `extensions/configuration-editing/schemas/devContainer.codespaces.schema.json` - Codespaces container schema
- `extensions/configuration-editing/schemas/attachContainer.schema.json` - Attach container schema

### Notable Clusters
- **Source Code**: `extensions/configuration-editing/src/` contains 9 TypeScript files with platform-specific code paths (browser/, node/) and test suite
- **Build Configuration**: Dual esbuild configurations support both Node and browser execution paths
- **Container Schemas**: Three JSON schema files in `extensions/configuration-editing/schemas/` for dev container specifications
- **Assets**: `extensions/configuration-editing/images/icon.png` - Extension icon

---

## Summary

The configuration-editing extension provides TypeScript implementation split across a main module and utility modules for settings, profiles, and proposals. It features dual-platform support (browser/node) with separate build configurations, comprehensive TypeScript configurations, and schema definitions for container specifications. The test structure includes a dedicated test suite with completion tests.
