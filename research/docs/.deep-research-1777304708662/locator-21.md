# File Location Report: Microsoft Authentication Extension
## Research Partition 21 of 79

**Scope**: `extensions/microsoft-authentication/` (31 files, 3,561 LOC)
**Research Question**: Porting VS Code IDE core functionality (editing, language intelligence, debugging, source control, terminal, navigation) from TypeScript/Electron to Tauri/Rust.
**Partition Focus**: Auth flow; uses secrets/keytar. Relevant for Tauri (no Electron `safeStorage`).

---

### Implementation

- `extensions/microsoft-authentication/src/extension.ts` — Entry point; registers authentication provider via `vscode.authentication.registerAuthenticationProvider()`; uses `context.secrets` and `context.globalState` APIs
- `extensions/microsoft-authentication/src/node/authProvider.ts` — Core authentication provider implementation; handles session lifecycle, token management, credential refresh; uses `PublicClientApplication` from MSAL; interfaces with `ExtensionContext`
- `extensions/microsoft-authentication/src/node/cachedPublicClientApplication.ts` — Wraps MSAL `PublicClientApplication` with caching layer; manages token cache and account access
- `extensions/microsoft-authentication/src/node/flows.ts` — OAuth 2.0 authorization code flow implementation with PKCE; handles token acquisition and refresh
- `extensions/microsoft-authentication/src/node/publicClientCache.ts` — Token cache management for MSAL; persists credentials
- `extensions/microsoft-authentication/src/common/publicClientCache.ts` — Shared token cache plugin for MSAL integration
- `extensions/microsoft-authentication/src/common/cachePlugin.ts` — Cache plugin interface for MSAL persistence
- `extensions/microsoft-authentication/src/betterSecretStorage.ts` — Wrapper around `context.secrets` API (ExtensionContext SecretStorage); provides token storage with change notification support; exports `BetterTokenStorage<T>` interface
- `extensions/microsoft-authentication/src/UriEventHandler.ts` — Handles URI protocol events for OAuth redirect callback (deep linking); integrates with VS Code URI event system
- `extensions/microsoft-authentication/src/cryptoUtils.ts` — Cryptographic utilities for token handling (hash, sign operations)
- `extensions/microsoft-authentication/src/node/buffer.ts` — Buffer utilities for cryptographic operations
- `extensions/microsoft-authentication/src/node/loopbackTemplate.ts` — HTML template for loopback server redirect endpoint; serves as OAuth callback receiver on localhost
- `extensions/microsoft-authentication/src/node/fetch.ts` — HTTP fetch wrapper abstraction
- `extensions/microsoft-authentication/src/node/loopbackTemplate.ts` — Loopback HTTP server (140 LOC) that captures authorization code from redirect URI
- `extensions/microsoft-authentication/src/common/loopbackClientAndOpener.ts` — Manages loopback server lifecycle; binds to localhost and opens browser for OAuth flow
- `extensions/microsoft-authentication/src/common/accountAccess.ts` — Account access control and session filtering
- `extensions/microsoft-authentication/src/common/scopeData.ts` — OAuth scope management and parsing
- `extensions/microsoft-authentication/src/common/config.ts` — Configuration constants (client ID, authority, scopes)
- `extensions/microsoft-authentication/src/common/env.ts` — Environment-specific settings
- `extensions/microsoft-authentication/src/common/event.ts` — Event emitter utilities
- `extensions/microsoft-authentication/src/common/async.ts` — Async utility functions
- `extensions/microsoft-authentication/src/common/uri.ts` — URI parsing and handling
- `extensions/microsoft-authentication/src/common/telemetryReporter.ts` — Telemetry reporting
- `extensions/microsoft-authentication/src/common/loggerOptions.ts` — Logger configuration
- `extensions/microsoft-authentication/src/common/experimentation.ts` — Experimentation/feature flag support
- `extensions/microsoft-authentication/src/logger.ts` — Logging utility

### Tests

- `extensions/microsoft-authentication/src/common/test/loopbackClientAndOpener.test.ts` — Tests for loopback server initialization and browser opening
- `extensions/microsoft-authentication/src/common/test/scopeData.test.ts` — Tests for OAuth scope parsing and management
- `extensions/microsoft-authentication/src/node/test/flows.test.ts` — Tests for OAuth 2.0 flow implementation

### Configuration

- `extensions/microsoft-authentication/package.json` — Extension manifest; declares `activationEvents: []` (lazy), `enabledApiProposals: ["nativeWindowHandle", "authIssuers", "authenticationChallenges"]`; dependencies include `@azure/msal-node`, `keytar`, `uuid`
- `extensions/microsoft-authentication/tsconfig.json` — TypeScript compilation configuration
- `extensions/microsoft-authentication/esbuild.mts` — Build configuration
- `extensions/microsoft-authentication/.npmrc` — NPM configuration
- `extensions/microsoft-authentication/.vscodeignore` — VS Code extension packaging rules

### Package Mocks

- `extensions/microsoft-authentication/packageMocks/keytar/index.js` — Mock implementation of native keytar module (system keychain abstraction)
- `extensions/microsoft-authentication/packageMocks/keytar/package.json` — Mock keytar package metadata
- `extensions/microsoft-authentication/packageMocks/dpapi/dpapi.js` — Mock DPAPI (Data Protection API) module for Windows credential encryption

### Documentation

- `extensions/microsoft-authentication/README.md` — Extension documentation

### Media / UI

- `extensions/microsoft-authentication/media/index.html` — OAuth authorization UI template
- `extensions/microsoft-authentication/media/auth.css` — Authorization UI styles
- `extensions/microsoft-authentication/media/icon.png` — Extension icon
- `extensions/microsoft-authentication/media/favicon.ico` — Favicon for auth UI

### Internationalization

- `extensions/microsoft-authentication/package.nls.json` — Localization strings

---

### Notable Clusters

- **`src/node/`** — 9 files; Node.js-specific OAuth 2.0 implementation (MSAL-based authentication, loopback redirect server, token caching, public client application management); **Tauri-relevant**: HTTP server, PKCE flow, token lifecycle
- **`src/common/`** — 12 files (includes `/test/` subdirectory); Shared authentication utilities across platforms (URI handling, scope management, cache plugins, event emission, telemetry); **Tauri-relevant**: Protocol handling, cache abstraction
- **`packageMocks/`** — 2 subdirectories; Mock implementations of platform-specific credential storage (keytar for system keychain, DPAPI for Windows); **Tauri-relevant**: These are Electron native module mocks; Tauri would need Rust-native equivalents

---

## Summary

The `microsoft-authentication` extension (38 files, 3,561 LOC) implements a complete OAuth 2.0 authentication provider for Microsoft accounts using MSAL (Microsoft Authentication Library). Key Tauri migration surfaces:

1. **SecretStorage API** (`context.secrets`) — Backed by keytar/Electron safeStorage; Tauri requires OS keychain integration (Rust: `keyring-rs`, macOS Keychain, Windows Credential Manager, Linux Secret Service)
2. **Authentication Provider Registration** — Extension registers via `vscode.authentication.registerAuthenticationProvider()`; Tauri host must replicate this VS Code API
3. **Loopback OAuth Server** — Localhost HTTP server captures authorization codes; Tauri needs HTTP server abstraction (e.g., `tauri::http` or embedded Rust server)
4. **Native Credential Storage Mocks** — `packageMocks/{keytar,dpapi}` mask platform-specific credential APIs; Tauri host needs unified Rust credential store
5. **Token Caching** — MSAL cache persistence via `context.secrets`; requires encrypted storage in Tauri
6. **URI Event Handling** — Deep-link protocol registration for OAuth redirect; Tauri needs protocol handler registration (platform-specific: Windows registry, macOS Info.plist, Linux .desktop file)

**Critical Dependencies**: `@azure/msal-node`, `keytar`, `uuid`; Node.js HTTP and crypto APIs; VS Code ExtensionContext and SecretStorage interfaces.

