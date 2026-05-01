# GitHub Authentication Extension Architecture

## Research Question
What it would take to port VS Code's core IDE functionality from TypeScript/Electron to Tauri/Rust — examining the authentication provider infrastructure.

## Implementation Files

### Core Authentication Provider
- `/extensions/github-authentication/src/github.ts` — Main `GitHubAuthenticationProvider` class implementing `vscode.AuthenticationProvider` interface. Handles session lifecycle: creation (line 346), retrieval (line 200), and removal (line 420). Integrates with VS Code's `authentication.registerAuthenticationProvider()` API (line 179).
- `/extensions/github-authentication/src/extension.ts` — Extension activation point. Registers two authentication providers: the main GitHub provider (line 68) and an optional GitHub Enterprise provider with config-driven initialization (lines 39–82).
- `/extensions/github-authentication/src/githubServer.ts` — `GitHubServer` class implementing `IGitHubServer`. Encapsulates OAuth server interaction, token exchange (line 155), user info fetching (line 220), and telemetry (line 263).
- `/extensions/github-authentication/src/flows.ts` — Four authentication flow implementations as `IFlow` classes:
  - `LocalServerFlow` (line 287): Loopback server on localhost, requires client secret
  - `UrlHandlerFlow` (line 197): URI handler redirect without local server, requires client secret
  - `DeviceCodeFlow` (line 387): Device authorization grant, no client secret needed
  - `PatFlow` (line 522): Personal access token manual entry, no client secret needed
  
  Flow selection logic in `getFlows()` (line 614) determines valid flows based on GitHub target (DotCom, Enterprise, HostedEnterprise), extension host (Local, Remote, WebWorker), and client capabilities.

### OAuth & Session Management
- `/extensions/github-authentication/src/common/keychain.ts` — `Keychain` class wrapping `vscode.ExtensionContext.secrets` for encrypted token storage. Three methods: `setToken()`, `getToken()`, `deleteToken()` (lines 16–47).
- `/extensions/github-authentication/src/github.ts` (lines 70–127): `UriEventHandler` class intercepts OAuth callback URIs via `vscode.UriHandler`. Manages pending nonce validation (line 79), prevents replay attacks, and chains promise resolution for multiple concurrent auth flows.
- `/extensions/github-authentication/src/node/authServer.ts` — `LoopbackAuthServer` HTTP server listening on 127.0.0.1. Implements OAuth callback endpoints `/signin` and `/callback`, serves static HTML/CSS/SVG media (line 146), handles state/nonce validation (lines 125–134).
- `/extensions/github-authentication/src/browser/authServer.ts` — Stub implementation; throws `Not implemented` for web context.

### PKCE & Cryptography
- `/extensions/github-authentication/src/flows.ts` (lines 121–146):
  - `generateRandomString()`: Creates random hex string for PKCE code verifier
  - `generateCodeChallenge()`: Computes SHA-256 hash, base64url-encodes result
  - `exchangeCodeForToken()`: Exchanges authorization code for token using client credentials (lines 148–195)

- `/extensions/github-authentication/src/node/crypto.ts` — Exports Node's `webcrypto` API
- `/extensions/github-authentication/src/browser/crypto.ts` — Exports browser's `globalThis.crypto`

### Network & Runtime Abstraction
- `/extensions/github-authentication/src/node/fetch.ts` — Dual-fetch strategy with fallback. Tries Electron `net.fetch()` first, falls back to Node's built-in fetch (lines 43–60). Configurable via `github-authentication.useElectronFetch` setting.
- `/extensions/github-authentication/src/browser/fetch.ts` — Direct proxy to browser `fetch()`.
- `/extensions/github-authentication/src/node/buffer.ts` — Base64 encoding using Node's `Buffer` class
- `/extensions/github-authentication/src/browser/buffer.ts` — Base64 encoding using `btoa()`

### Configuration & Utilities
- `/extensions/github-authentication/src/config.ts` — Hardcoded GitHub OAuth client ID (`01ab8ac9400c4e429b23`), optional client secret injection point.
- `/extensions/github-authentication/src/common/env.ts` — Runtime environment detection:
  - `isSupportedClient()`: Whitelists callback URI schemes (vscode, vscode-insiders, vscode-wsl) and domains (vscode.dev, github.dev)
  - `isSupportedTarget()`: Determines if target GitHub instance supports token revocation
  - `isHostedGitHubEnterprise()`: Detects GHE Cloud instances (`.ghe.com` authority)
- `/extensions/github-authentication/src/common/errors.ts` — Standardized error strings: `TIMED_OUT_ERROR`, `USER_CANCELLATION_ERROR`, `CANCELLATION_ERROR`, `NETWORK_ERROR`
- `/extensions/github-authentication/src/common/logger.ts` — `Log` class wrapping `vscode.window.createOutputChannel()` with trace/debug/info/error/warn methods.
- `/extensions/github-authentication/src/common/utils.ts` — Helper functions:
  - `promiseFromEvent()`: Convert VS Code events to promises with cancellation
  - `arrayEquals()`: Scope comparison utility
  - `StopWatch`: Timing utility

### Telemetry
- `/extensions/github-authentication/src/common/experimentationService.ts` — `ExperimentationTelemetry` wrapping `@vscode/extension-telemetry` reporter
- Telemetry events: `login`, `loginCancelled`, `loginFailed`, `logout`, `logoutFailed`, `session` (EDU status), `ghe-session` (version tracking)

## Tests

### Unit Tests
- `/extensions/github-authentication/src/test/node/authServer.test.ts` — 10+ test cases for `LoopbackAuthServer`:
  - `/signin` endpoint redirect validation (line 23)
  - `/callback` state/nonce/code parameter validation (lines 45–65)
  - Portable mode detection (lines 68–80)
  
- `/extensions/github-authentication/src/test/node/fetch.test.ts` — Fetch fallback behavior
- `/extensions/github-authentication/src/test/flows.test.ts` — Flow selection logic tests across target/host/client combinations (lines 29–100+)

## Types / Interfaces

### Session Management
- `SessionData` (github.ts line 17): Stores id, account info, scopes, accessToken
- `vscode.AuthenticationSession`: Public API contract with id, account (label, id), scopes, accessToken
- `vscode.AuthenticationProvider`: Interface implemented by `GitHubAuthenticationProvider`
- `vscode.AuthenticationProviderAuthenticationSessionsChangeEvent`: Session change notification

### OAuth Flow
- `IFlowOptions` (flows.ts line 25): Capability matrix (supports GitHub.com, Enterprise, Hosted; web worker, remote, etc.)
- `IFlowTriggerOptions` (flows.ts line 63): Input parameters for flow execution (scopes, baseUri, nonce, callbacks)
- `IFlow` (flows.ts line 110): Flow interface with label, options, trigger method
- `IFlowQuery` (flows.ts line 57): Query shape for `getFlows()` selection

### Server & Network
- `IGitHubServer` (githubServer.ts line 21): Contract for server interaction (login, logout, getUserInfo, telemetry)
- `FetchOptions`, `FetchResponse`, `FetchHeaders` (fetch.ts lines 12–32): Fetch abstraction layer
- `ILoopbackServer` (authServer.ts line 36): Port, nonce, state management, start/stop, waitForOAuthResponse

### GitHub-Specific
- `AuthProviderType` enum: `github` vs `github-enterprise`
- `GitHubTarget` enum: `DotCom`, `Enterprise`, `HostedEnterprise` (flows.ts line 45)
- `ExtensionHost` enum: `WebWorker`, `Remote`, `Local` (flows.ts line 51)
- `GitHubSocialSignInProvider` enum: `Google`, `Apple` (flows.ts line 666)
- `GitHubAuthenticationProviderOptions`: Provider-specific options extending `vscode.AuthenticationProviderSessionOptions`

## Configuration

### Extension Manifest
- `/extensions/github-authentication/package.json`:
  - **Entry points**: `main: ./out/extension.js` (Node), `browser: ./dist/browser/extension.js` (Web)
  - **API proposals**: `authIssuers`, `authProviderSpecific`, `agentSessionsWorkspace`
  - **Contributes**:
    - Two authentication providers: `github` (ID: github) and `github-enterprise` (ID: github-enterprise)
    - Settings: `github-enterprise.uri`, `github-authentication.useElectronFetch`, `github-authentication.preferDeviceCodeFlow`
  - **Dependencies**: `node-fetch@2.6.7`, `@vscode/extension-telemetry@^0.9.8`, `vscode-tas-client@^0.1.84`
  - **Capabilities**: Virtual workspaces, untrusted workspaces (limited support)

### VS Code API Proposals Used
- `vscode.authentication.registerAuthenticationProvider()`: Core registration (github.ts line 179)
- `vscode.ExtensionContext.secrets`: Encrypted token storage (keychain.ts line 18)
- `vscode.window.registerUriHandler()`: OAuth callback handling (extension.ts line 66)
- `vscode.env.asExternalUri()`: URI scheme translation for web contexts
- `vscode.env.openExternal()`: Open browser for auth flow (flows.ts line 272)
- `vscode.workspace.getConfiguration()`: Settings access (multiple files)
- `vscode.commands.executeCommand()`: Proxy endpoint discovery (githubServer.ts line 60)

## Examples / Fixtures

### OAuth Flow Sequence
1. Extension calls `GitHubAuthenticationProvider.createSession(scopes)` (github.ts line 346)
2. `GitHubServer.login()` determines available flows based on runtime (githubServer.ts line 90)
3. For `LoopbackAuthServer` flow:
   - Server starts on 127.0.0.1 with random port
   - Nonce generated from `crypto.getRandomValues()`
   - User redirected to `https://github.com/login/oauth/authorize?client_id=...&state=...&code_challenge=...`
   - Browser redirects to `http://127.0.0.1:{port}/callback?code=...&state=...&nonce=...`
   - PKCE code verifier exchanged for token at `/login/oauth/access_token`
4. Token converted to session via `getUserInfo()` API call

### Configuration Example
```json
{
  "github-enterprise.uri": "https://ghe.company.com",
  "github-authentication.useElectronFetch": true,
  "github-authentication.preferDeviceCodeFlow": false
}
```

### Session Storage
Sessions stored in VS Code's secret storage as JSON array:
```json
[
  {
    "id": "abc123def456",
    "account": { "label": "username", "id": "12345" },
    "scopes": ["user", "repo"],
    "accessToken": "gho_xxxxx..."
  }
]
```

## Documentation

- `/extensions/github-authentication/README.md` — Brief overview: bundled extension providing `github` authentication provider for Settings Sync and other extensions.
- Inline comments document GDPR telemetry classification (lines 312–318 in githubServer.ts, 352–357 in github.ts)
- Flow-specific documentation in class comments explaining capability matrices and trade-offs (e.g., LocalServerFlow doesn't work on web workers)

## Notable Clusters

### Authentication Flow Architecture
The extension implements a **capability-driven flow selector** pattern. Each flow declares supported targets, runtimes, and client types via `IFlowOptions`. The `getFlows()` function (flows.ts line 614) returns a filtered, ordered list of flows matching the current environment. This design accommodates:
- **Desktop (Local)**: LocalServerFlow preferred, URL handler and device code as fallbacks
- **Web (Remote/WebWorker)**: URL handler and device code only
- **On-Prem GHES**: Device code and PAT flows (no token revocation support)
- **Hosted GHE**: All flows available

### Platform Abstraction
The extension uses **dual implementations** for platform-specific APIs:
- `src/node/` — Node.js/Electron implementations (webcrypto, node-fetch, Buffer, HTTP server)
- `src/browser/` — Browser implementations (globalThis.crypto, fetch(), btoa(), stub authServer)
- `src/common/` — Shared logic (keychain, logger, utilities, error definitions)

Build scripts compile separate bundles: `out/extension.js` for Node and `dist/browser/extension.js` for web via esbuild.

### Secret Storage & Lifecycle
Sessions are stored as encrypted JSON in `vscode.ExtensionContext.secrets` (managed by VS Code's secret storage backend — system keychain on desktop, IndexedDB on web). The `Keychain` class provides CRUD operations. Session validation occurs on every activation: invalid/expired tokens are removed (github.ts lines 255–337).

### OAuth & Security
- **PKCE**: Code verifiers (64 bytes random hex) and SHA-256 challenges for all code-exchange flows
- **Nonce**: Random nonces prevent authorization code injection; validated in URI handler and loopback server
- **State Parameter**: Encodes callback URI for post-auth redirect
- **No Client Secret in Native Client**: Client ID publicly visible in code; secret only used in backend proxies for GitHub.com to comply with OAuth best practices

### Extension Kind & Capability Detection
`ExtensionKind` (UI vs Workspace) determines whether extension runs locally or remotely. For local extensions, more flows are available. The extension also detects if running in supported clients (vscode.dev, github.dev, desktop variants) to enable URL handler flow.

### Telemetry Integration
Telemetry via `@vscode/extension-telemetry` reports:
- Login attempts and failures
- User education status (student, faculty, none) for github.com
- GitHub Enterprise version for tracking adoption
- Managed account detection (username contains underscore)

---

## Summary

The GitHub authentication extension is a **sophisticated OAuth provider** that demonstrates VS Code's authentication plugin architecture. It encompasses:

1. **Provider Registration**: Implements `vscode.AuthenticationProvider` with session lifecycle methods, registered via `vscode.authentication.registerAuthenticationProvider()`
2. **Multi-Flow Strategy**: Four complementary OAuth flows (loopback server, URL handler, device code, PAT) automatically selected based on runtime capabilities and GitHub target type
3. **Secure Token Storage**: Leverages VS Code's encrypted `ExtensionContext.secrets` API with session persistence and validation
4. **Platform Abstraction**: Separate Node/Browser implementations with shared core logic, enabling both Electron and web deployment
5. **PKCE & Replay Attack Prevention**: Code verifier/challenge pairs and cryptographic nonce validation for security
6. **GitHub Enterprise Support**: Conditional feature support (token revocation, token scope validation) based on server type detection
7. **Runtime Capability Detection**: Introspects extension host type, client scheme/domain, and GitHub instance to enable appropriate flows

**Porting to Tauri/Rust** would require:
- **Rust OAuth client library** (e.g., `oauth2` crate) with PKCE support
- **Native HTTP server** for loopback flow (e.g., `tokio`, `hyper`)
- **Cryptography primitives** (SHA-256, PBKDF2 if PAT validation needed)
- **Secret storage integration** (native keychain libraries; Tauri provides `tauri-plugin-store` for web storage)
- **URI scheme handling** for deep links (Tauri's `tauri::api::shell::open()` for external URLs, custom protocol handler for OAuth callbacks)
- **Multi-flow selector logic** mapping Tauri's runtime environment to flow capabilities
- **IPC bridge** between Rust backend and frontend for cryptographic operations and HTTP server management
- **Session serialization/deserialization** (serde for JSON session data)
- **Telemetry forwarding** to avoid tight coupling with VS Code's telemetry SDK
- **Configuration management** via Tauri's preferences/config system instead of VS Code settings

The architecture itself is **portable**; the challenge is replacing VS Code APIs with Tauri equivalents and ensuring Rust-based OAuth flows achieve feature parity with the TypeScript implementations.
