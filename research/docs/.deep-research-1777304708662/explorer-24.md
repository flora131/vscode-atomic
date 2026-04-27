# Partition 24 of 79 — Findings

## Scope
`extensions/github-authentication/` (24 files, 3,104 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# GitHub Authentication Extension - File Locator Report

## Overview
The `extensions/github-authentication/` contains VS Code's OAuth authentication provider for GitHub with multiple authentication flows. Total: 24 files, 3,046 LOC of TypeScript source code.

## Implementation

### Core Extension Logic
- `extensions/github-authentication/src/extension.ts` - Extension entry point, registers GitHub authentication providers
- `extensions/github-authentication/src/github.ts` - GitHubAuthenticationProvider class, session management, UriEventHandler
- `extensions/github-authentication/src/githubServer.ts` - IGitHubServer interface and GitHubServer class for OAuth communication
- `extensions/github-authentication/src/flows.ts` - Authentication flow orchestration (multiple flow types), GitHubTarget enum, ExtensionHost enum
- `extensions/github-authentication/src/config.ts` - OAuth client ID/secret configuration

### Node Runtime Implementation (Desktop)
- `extensions/github-authentication/src/node/authServer.ts` - LoopbackAuthServer class using Node.js `http.createServer()` for localhost OAuth callback
- `extensions/github-authentication/src/node/fetch.ts` - Node.js fetch implementation wrapper
- `extensions/github-authentication/src/node/crypto.ts` - Node.js crypto utilities (PKCE, hashing)
- `extensions/github-authentication/src/node/buffer.ts` - Node.js buffer utilities (base64 encoding)

### Browser Runtime Implementation (Web)
- `extensions/github-authentication/src/browser/authServer.ts` - Browser stub (throws "Not implemented") - placeholder for browser flow
- `extensions/github-authentication/src/browser/fetch.ts` - Browser fetch implementation
- `extensions/github-authentication/src/browser/crypto.ts` - Browser crypto utilities (SubtleCrypto API)
- `extensions/github-authentication/src/browser/buffer.ts` - Browser buffer utilities

### Common Utilities
- `extensions/github-authentication/src/common/keychain.ts` - Keychain/secret storage abstraction using VS Code's ExtensionContext.secrets
- `extensions/github-authentication/src/common/utils.ts` - Utility functions (PromiseAdapter, arrayEquals, promiseFromEvent)
- `extensions/github-authentication/src/common/logger.ts` - Logging abstraction
- `extensions/github-authentication/src/common/experimentationService.ts` - Telemetry and experimentation service
- `extensions/github-authentication/src/common/errors.ts` - Error constants for OAuth flow
- `extensions/github-authentication/src/common/env.ts` - Environment detection utilities

## Tests

### Unit Tests
- `extensions/github-authentication/src/test/node/authServer.test.ts` - LoopbackAuthServer tests (server startup, redirect handling, state management)
- `extensions/github-authentication/src/test/node/fetch.test.ts` - Fetch implementation tests
- `extensions/github-authentication/src/test/flows.test.ts` - OAuth flow selection logic tests (tests IFlowQuery, ExtensionHost conditions, client secret scenarios)

## Configuration

### Build Configuration
- `extensions/github-authentication/tsconfig.json` - TypeScript compiler options (output to `./out`, WebWorker + ES2024 lib)
- `extensions/github-authentication/tsconfig.browser.json` - Browser-specific TypeScript configuration
- `extensions/github-authentication/esbuild.mts` - esbuild configuration for Node.js target
- `extensions/github-authentication/esbuild.browser.mts` - esbuild configuration for browser target

### Extension Configuration
- `extensions/github-authentication/package.json` - Extension manifest with API contributions (authentication provider registration), configuration schema (github-enterprise.uri, github-authentication.useElectronFetch, github-authentication.preferDeviceCodeFlow)
- `extensions/github-authentication/package.nls.json` - Localization strings
- `extensions/github-authentication/tsconfig.browser.json` - Browser-specific TypeScript settings

### Ignore Patterns
- `extensions/github-authentication/.gitignore` - Git ignore rules
- `extensions/github-authentication/.vscodeignore` - VS Code extension packaging ignore rules
- `extensions/github-authentication/.npmrc` - NPM configuration

## Types / Interfaces

### Core Interfaces (from implementation files)
- `IOAuthResult` - OAuth authorization code and state (authServer.ts)
- `ILoopbackServer` - Localhost server interface with port, nonce, state (authServer.ts)
- `IGitHubDeviceCodeResponse` - Device flow response structure (flows.ts)
- `IFlowOptions` - Authentication flow feature flags (flows.ts)
- `IFlowQuery` - Query parameters for flow selection (flows.ts)
- `GitHubTarget` - Enum: DotCom | Enterprise | HostedEnterprise (flows.ts)
- `ExtensionHost` - Enum: Local | Remote | WebWorker (flows.ts)
- `SessionData` - Token session with id, account, scopes, accessToken (github.ts)
- `AuthProviderType` - Enum: github | github-enterprise (github.ts)
- `GitHubAuthenticationProviderOptions` - Options with optional provider (GitHubSocialSignInProvider) and extraAuthorizeParameters (github.ts)
- `IGitHubServer` - Server interface with login, logout, getUserInfo, sendAdditionalTelemetryInfo (githubServer.ts)
- `IConfig` - Configuration interface with gitHubClientId, gitHubClientSecret (config.ts)

## Examples / Fixtures

### Web UI
- `extensions/github-authentication/media/index.html` - OAuth callback success/error page with dynamic redirect logic and fallback link handling
- `extensions/github-authentication/media/auth.css` - Styling for authentication UI
- `extensions/github-authentication/media/code-icon.svg` - VS Code icon asset
- `extensions/github-authentication/media/sessions-icon.svg` - Sessions icon asset
- `extensions/github-authentication/media/favicon.ico` - Favicon

### Icons
- `extensions/github-authentication/images/icon.png` - Extension icon
- `extensions/github-authentication/media/icon.png` - Media folder icon

## Documentation

- `extensions/github-authentication/README.md` - Extension overview ("This extension is bundled with Visual Studio Code", provides GitHub Authentication Provider)

## Notable Clusters

### OAuth Flow Architecture
- **Localhost Server Pattern**: `src/node/authServer.ts` implements HTTP localhost server for OAuth redirect callbacks using Node.js http module. Browser stub at `src/browser/authServer.ts` indicates placeholder for web-based flow.

### Runtime Abstraction Layers
- **Node/Browser Duality**: Crypto (`src/node/crypto.ts` vs `src/browser/crypto.ts`), Fetch (`src/node/fetch.ts` vs `src/browser/fetch.ts`), and Buffer (`src/node/buffer.ts` vs `src/browser/buffer.ts`) implementations provide runtime-specific abstractions for desktop and web environments.

### Authentication Flows
- **Multi-flow Support**: `src/flows.ts` orchestrates multiple authentication strategies (local server, URL handler, device code, personal access token) with runtime and environment-specific constraints via `IFlowQuery` and `IFlowOptions`.

### Configuration Management
- **Enterprise Support**: Separate authentication providers registered in `package.json` for GitHub.com and GitHub Enterprise Server, with configuration schema in `github-enterprise.uri` setting.

### Credential Storage
- **VS Code Integration**: `src/common/keychain.ts` delegates to VS Code's ExtensionContext.secrets API for secure token storage, abstracting away platform-specific credential management.

---

## Porting Assessment: TypeScript/Electron to Tauri/Rust

Porting this OAuth extension from TypeScript/Electron to Tauri/Rust would require:

1. **Localhost HTTP Server Rewrite** - The core `http.createServer()` based LoopbackAuthServer would need a Rust HTTP server implementation (e.g., `actix-web`, `tokio`). The Node.js native http module is deeply integrated.

2. **Crypto & Encoding Layer** - Node.js `crypto` module usage (PKCE, SHA256, random bytes) and Buffer handling require Rust equivalents (`ring`, `sha2`, `base64` crates).

3. **Fetch/HTTP Client** - Current use of `node-fetch` and Electron fetch would need replacement with Rust HTTP client (e.g., `reqwest`).

4. **Secret Storage** - Keychain abstraction currently uses VS Code's native secret storage API; would need direct OS keychain access via Rust (e.g., `keyring` crate) or Tauri's secret storage plugin.

5. **Extension Manifest Compatibility** - The `package.json` extension contribution points (authentication provider registration) are VS Code-specific and have no Tauri equivalent; would require reimplementation as Tauri plugin/command interface.

6. **Experimentation Service** - The telemetry and experimentation service (ExperimentationTelemetry) uses VS Code's telemetry API; would need custom implementation or removal.

7. **Flow Orchestration Logic** - The multi-flow branching logic in `flows.ts` is platform-agnostic but depends on VS Code runtime detection (ExtensionHost enum) which is specific to VS Code's extension system.

8. **UI/Callback Handling** - The `media/index.html` success/error page and redirect logic would need adaptation for Tauri's webview; URI handling would shift from VS Code's UriEventHandler to Tauri's deep-linking or custom protocol handling.

9. **No Direct Browser Build** - The `browser/` stub implementations indicate web execution is not fully supported; a complete Rust rewrite would not use browser runtime abstractions.

The OAuth flow logic itself (PKCE validation, state management, token exchange) is framework-agnostic and would port cleanly, but the infrastructure—servers, crypto, storage, and VS Code API integration—constitutes the bulk of the implementation complexity.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
# Analyzer-24: `extensions/github-authentication/` — GitHub Authentication Extension

## Files Analysed

1. `/Users/norinlavaee/vscode-atomic/extensions/github-authentication/src/extension.ts` (103 lines)
2. `/Users/norinlavaee/vscode-atomic/extensions/github-authentication/src/github.ts` (453 lines)
3. `/Users/norinlavaee/vscode-atomic/extensions/github-authentication/src/githubServer.ts` (371 lines)
4. `/Users/norinlavaee/vscode-atomic/extensions/github-authentication/src/flows.ts` (673 lines)
5. `/Users/norinlavaee/vscode-atomic/extensions/github-authentication/src/node/authServer.ts` (209 lines)
6. `/Users/norinlavaee/vscode-atomic/extensions/github-authentication/src/node/fetch.ts` (263 lines)
7. `/Users/norinlavaee/vscode-atomic/extensions/github-authentication/src/node/crypto.ts` (8 lines)
8. `/Users/norinlavaee/vscode-atomic/extensions/github-authentication/src/node/buffer.ts` (8 lines)
9. `/Users/norinlavaee/vscode-atomic/extensions/github-authentication/src/browser/authServer.ts` (18 lines)
10. `/Users/norinlavaee/vscode-atomic/extensions/github-authentication/src/browser/crypto.ts` (6 lines)
11. `/Users/norinlavaee/vscode-atomic/extensions/github-authentication/src/common/keychain.ts` (48 lines)
12. `/Users/norinlavaee/vscode-atomic/extensions/github-authentication/src/common/utils.ts` (118 lines)
13. `/Users/norinlavaee/vscode-atomic/extensions/github-authentication/src/common/env.ts` (41 lines)
14. `/Users/norinlavaee/vscode-atomic/extensions/github-authentication/src/common/experimentationService.ts` (96 lines)
15. `/Users/norinlavaee/vscode-atomic/extensions/github-authentication/src/config.ts` (19 lines)
16. `/Users/norinlavaee/vscode-atomic/extensions/github-authentication/src/common/errors.ts` (14 lines)

---

## Per-File Notes (file:line)

### `extension.ts`

**Role**: Extension entry point. Registers one or two `AuthenticationProvider` instances depending on configuration and watches for runtime config changes.

**Key symbols**:
- `extension.ts:12` — `NullAuthProvider` class: a stub `vscode.AuthenticationProvider` registered under `'github-enterprise'` when the GHES URI setting is absent or invalid. `createSession` and `removeSession` throw immediately; `getSessions` returns `[]`.
- `extension.ts:39` — `initGHES(context, uriHandler)`: reads `github-enterprise.uri` from workspace config, validates it as a `vscode.Uri`, then instantiates a real `GitHubAuthenticationProvider` (or a `NullAuthProvider` on failure). Returns a `vscode.Disposable`.
- `extension.ts:63` — `activate(context)`: the VS Code activation entrypoint.
  - `extension.ts:64` — Creates a single shared `UriEventHandler` and registers it with `vscode.window.registerUriHandler`.
  - `extension.ts:68` — Constructs the primary `GitHubAuthenticationProvider` (GitHub.com, no GHES URI).
  - `extension.ts:71` — Calls `initGHES` and stores its result so it can be disposed on config change.
  - `extension.ts:72–81` — `onDidChangeConfiguration` listener re-creates the GHES provider whenever `github-enterprise.uri` changes.
  - `extension.ts:84–102` — Second `onDidChangeConfiguration` listener detects changes to `github-authentication.useElectronFetch` and prompts the user to reload the window via `workbench.action.reloadWindow`.

**Control flow**: `activate` → two auth providers registered → config-change watchers armed for live reconfiguration.

---

### `github.ts`

**Role**: Defines `GitHubAuthenticationProvider` (the concrete `vscode.AuthenticationProvider` implementation) and `UriEventHandler` (the OAuth redirect receiver).

**Key symbols**:

- `github.ts:17–28` — `SessionData` interface: shape of persisted session JSON (`id`, `account.{label,displayName,id}`, `scopes[]`, `accessToken`).
- `github.ts:30–33` — `AuthProviderType` enum: `github` | `githubEnterprise`.
- `github.ts:35–48` — `GitHubAuthenticationProviderOptions` interface extending `vscode.AuthenticationProviderSessionOptions`, adding optional `provider: GitHubSocialSignInProvider` and `extraAuthorizeParameters: Record<string,string>`.
- `github.ts:50–68` — `isGitHubAuthenticationProviderOptions`: runtime type-guard that validates option shape, throwing descriptive `Error` on malformed input.
- `github.ts:70–128` — `UriEventHandler extends vscode.EventEmitter<vscode.Uri> implements vscode.UriHandler`:
  - `github.ts:71` — `_pendingNonces: Map<string, string[]>`: maps scope-string → array of accepted nonces for concurrent login flows.
  - `github.ts:72` — `_codeExchangePromises: Map<string, {promise, cancel}>`: deduplicates concurrent waiters for the same scope.
  - `github.ts:74` — `handleUri(uri)`: fires the event immediately (called by VS Code runtime on redirect).
  - `github.ts:78–99` — `waitForCode(logger, scopes, nonce, token)`: registers the nonce, starts/reuses a `promiseFromEvent`, then races against a 300 s hardcoded timeout and a cancellation token. Cleans up nonce and promise regardless of outcome (the `finally` block at `github.ts:94`).
  - `github.ts:101–127` — `handleEvent`: the `PromiseAdapter` passed to `promiseFromEvent`. Parses `code` and `nonce` from the redirect URI's query string; rejects on absence; silently returns (skips) if the nonce is not in the accepted set.

- `github.ts:130–453` — `GitHubAuthenticationProvider`:
  - **Constructor** (`github.ts:141–190`):
    - Reads `aiKey` from `package.json` and wraps it in `ExperimentationTelemetry` + `TelemetryReporter`.
    - Determines `AuthProviderType` from whether `ghesUri` is present.
    - Constructs `Keychain` with service ID `"github.auth"` (dotcom) or `"<authority><path>.ghes.auth"` (enterprise).
    - Constructs `GitHubServer` passing logger, telemetry, `uriHandler`, extension kind, and optional GHES URI.
    - Calls `readSessions()` immediately and stores the resulting promise in `_sessionsPromise`; fires telemetry for each session after a 1 s delay.
    - Registers the provider via `vscode.authentication.registerAuthenticationProvider` with `supportsMultipleAccounts: true`.
    - Subscribes to `context.secrets.onDidChange` → `checkForUpdates()`.

  - **`getSessions`** (`github.ts:200–213`): Sorts requested scopes, awaits `_sessionsPromise`, filters by account label (if `options.account` given), then filters by exact sorted-scope equality.

  - **`checkForUpdates`** (`github.ts:224–253`): Diff between previous and freshly read sessions; fires `_sessionChangeEmitter` with added/removed arrays. Called when the OS secret store signals a change (another window mutated sessions).

  - **`readSessions`** (`github.ts:255–337`): Reads JSON from keychain, deduplicates by sorted scope string, verifies tokens against the GitHub API if `session.account` is absent, migrates numeric account IDs to strings. Returns only successfully verified sessions; re-stores if any migration/pruning occurred.

  - **`storeSessions`** (`github.ts:339–344`): Updates `_sessionsPromise` synchronously (in-memory), serialises sessions to JSON, persists via `Keychain.setToken`.

  - **`createSession`** (`github.ts:346–408`):
    1. Validates options via `isGitHubAuthenticationProviderOptions`.
    2. Delegates OAuth to `this._githubServer.login(scopeString, signInProvider, extraAuthorizeParameters, loginWith)`.
    3. Exchanges returned token for a full session via `tokenToSession` (calls `getUserInfo`).
    4. Upserts the session into the in-memory list and persists.
    5. Fires `_sessionChangeEmitter` with `{added, removed, changed:[]}`.
    6. Error paths: silently re-throws `'Cancelled'`; shows `vscode.window.showErrorMessage` and emits telemetry for all other errors.

  - **`tokenToSession`** (`github.ts:410–418`): Calls `getUserInfo`, generates a 64-bit hex session ID using `crypto.getRandomValues`, returns a `vscode.AuthenticationSession` literal.

  - **`removeSession`** (`github.ts:420–452`): Finds session by ID, splices from array, calls `storeSessions`, then calls `_githubServer.logout(session)`, fires event.

---

### `githubServer.ts`

**Role**: `GitHubServer` implements `IGitHubServer`, orchestrating OAuth flow selection, token-server communication, user-info retrieval, and token revocation.

**Key symbols**:

- `githubServer.ts:21–27` — `IGitHubServer` interface: `login`, `logout`, `getUserInfo`, `sendAdditionalTelemetryInfo`, `friendlyName`.

- `githubServer.ts:30–46` — Constructor: sets `_type` and `friendlyName`; stores GHES URI and extension kind.

- `githubServer.ts:48–53` — `baseUri` getter: `https://github.com/` for dotcom; otherwise `_ghesUri`.

- `githubServer.ts:55–77` — `getRedirectEndpoint()`: lazily resolves to `https://vscode.dev/redirect` or `https://insiders.vscode.dev/redirect` for dotcom by calling the internal VS Code command `workbench.getCodeExchangeProxyEndpoints`. For GHES always uses `https://vscode.dev/redirect`.

- `githubServer.ts:80–88` — `isNoCorsEnvironment()`: determines if running in a no-CORS-constrained environment (web-based vscode.dev / github.dev / localhost) by round-tripping a dummy URI through `vscode.env.asExternalUri`.

- `githubServer.ts:90–153` — `login(scopes, signInProvider?, extraAuthorizeParameters?, existingLogin?)`:
  1. Generates a random `nonce` via `crypto.getRandomValues`.
  2. Constructs the `callbackUri` via `vscode.env.asExternalUri`.
  3. Classifies runtime: `GitHubTarget` (DotCom / HostedEnterprise / Enterprise) and `ExtensionHost` (Local / Remote / WebWorker) based on `process.versions.node` presence and `_extensionKind`.
  4. Calls `getFlows(query)` to get an ordered list of applicable flows.
  5. Iterates flows; on first failure shows an optional "try a different way?" prompt. Throws `CANCELLATION_ERROR` if all flows exhaust or user declines retry.

- `githubServer.ts:155–208` — `logout(session)`: only deletes from the server if `Config.gitHubClientSecret` is set and token starts with `gho_` and target is supported. Calls GitHub REST `DELETE /applications/{client_id}/token` with Basic auth header constructed via `base64Encode`.

- `githubServer.ts:210–218` — `getServerUri(path)`: builds the correct API base URL (`api.github.com` vs `<host>/api/v3`) based on target type.

- `githubServer.ts:220–261` — `getUserInfo(token)`: `GET /user` with `Authorization: token <token>` header; returns `{id: string, accountName: string}`.

- `githubServer.ts:263–279` — `sendAdditionalTelemetryInfo(session)`: skips in no-CORS or telemetry-disabled environments; dispatches to `checkUserDetails` (dotcom) or `checkEnterpriseVersion` (GHES).

- `githubServer.ts:364–370` — `processLoginError`: distinguishes user-cancellation from other errors; re-throws `CANCELLATION_ERROR` immediately (does not return a boolean for those).

---

### `flows.ts`

**Role**: Defines four concrete OAuth flows as Strategy-pattern classes; `getFlows` selects and orders applicable strategies for the current runtime context.

**Key symbols**:

- `flows.ts:25–43` — `IFlowOptions`: nine boolean capability flags per flow (target support, runtime support, client support, client-secret requirement).
- `flows.ts:45–55` — `GitHubTarget` and `ExtensionHost` const enums.
- `flows.ts:57–61` — `IFlowQuery`: runtime context bag passed to `getFlows`.
- `flows.ts:63–108` — `IFlowTriggerOptions`: all parameters a flow's `trigger` method receives (scopes, URIs, nonce, provider, logger, handler).
- `flows.ts:110–114` — `IFlow` interface: `label: string`, `options: IFlowOptions`, `trigger(options): Promise<string>`.

- `flows.ts:121–128` — `generateRandomString(length)`: builds a hex string from `crypto.getRandomValues(Uint8Array)`.
- `flows.ts:135–146` — `generateCodeChallenge(codeVerifier)`: SHA-256 via `crypto.subtle.digest`, then base64url-encodes the digest for PKCE.
- `flows.ts:148–195` — `exchangeCodeForToken(logger, endpointUri, redirectUri, code, codeVerifier, enterpriseUri?)`: POSTs to `/login/oauth/access_token` with code + PKCE verifier + client secret. Returns `access_token` from JSON response.

- `flows.ts:197–285` — `UrlHandlerFlow` (label `'url handler'`):
  - Supported targets: DotCom + HostedEnterprise (not plain GHES). Requires client secret, supported client, works in Remote and WebWorker extension hosts.
  - `trigger` (`flows.ts:215–284`): Generates PKCE challenge, calls `uriHandler.waitForCode`, opens external browser to `/login/oauth/authorize`, awaits redirect code, then exchanges via proxy endpoint or direct `/login/oauth/access_token`.

- `flows.ts:287–385` — `LocalServerFlow` (label `'local server'`):
  - Supported targets: DotCom + HostedEnterprise. Local extension host only (no Remote, no WebWorker). Requires client secret; supports unsupported clients.
  - `trigger` (`flows.ts:307–384`): Generates PKCE challenge, instantiates `LoopbackAuthServer` (serving static HTML from `../media`), opens `http://127.0.0.1:{port}/signin`, races `server.waitForOAuthResponse()` against timeout and cancellation. Stops server after 5 s delay. Exchanges code via `exchangeCodeForToken`.

- `flows.ts:387–520` — `DeviceCodeFlow` (label `'device code'`):
  - Supports all targets including on-prem GHES. No client secret required. Remote + Local hosts; not WebWorker (CORS constraint).
  - `trigger` (`flows.ts:400–454`): POSTs to `/login/device/code`, shows modal with `user_code`, copies it to clipboard, opens browser to `verification_uri`.
  - `waitForDeviceCodeAccessToken` (`flows.ts:456–519`): polls `/login/oauth/access_token` at `json.interval`-second intervals up to 120 s, watching for `authorization_pending` vs actual token.

- `flows.ts:522–605` — `PatFlow` (label `'personal access token'`):
  - All targets; no client secret required. Both hosts. Only for unsupported clients (explicitly disabled for supported clients on DotCom to prevent Settings Sync breakage).
  - `trigger` (`flows.ts:537–578`): Directs user to GitHub's token creation page, shows an input box for the PAT, validates requested scopes against the token's actual `X-OAuth-Scopes` response header.

- `flows.ts:607–612` — `allFlows` array: `[LocalServerFlow, UrlHandlerFlow, DeviceCodeFlow, PatFlow]` (priority order).

- `flows.ts:614–661` — `getFlows(query: IFlowQuery)`:
  1. Filters `allFlows` by matching target, extension host, client-secret availability, and client type.
  2. Reads `github-authentication.preferDeviceCodeFlow` setting; if true, moves all `DeviceCodeFlow` instances to the front.
  3. Returns ordered array of valid flows.

- `flows.ts:666–673` — `GitHubSocialSignInProvider` const enum: `Google = 'google'`, `Apple = 'apple'`. `isSocialSignInProvider` type guard checks value equality.

---

### `node/authServer.ts`

**Role**: Implements `LoopbackAuthServer` — an ephemeral HTTP server on a random `127.0.0.1` port that handles the OAuth local redirect.

**Key symbols**:

- `authServer.ts:31–34` — `IOAuthResult`: `{code: string; state: string}`.
- `authServer.ts:36–69` — `ILoopbackServer` interface with `port`, `nonce`, `state`, `start()`, `stop()`, `waitForOAuthResponse()`.
- `authServer.ts:71–153` — `LoopbackAuthServer` constructor:
  - `authServer.ts:76` — `nonce` = `randomBytes(16).toString('base64')` (Node.js `crypto`).
  - `authServer.ts:99` — Creates a `Promise<IOAuthResult>` whose `deferred` resolver is captured in the closure.
  - HTTP handler routes:
    - `/signin` (`authServer.ts:106–114`): verifies nonce (replacing spaces with `+`), then 302-redirects to `_startingRedirect` (the GitHub authorize URL).
    - `/callback` (`authServer.ts:116–142`): verifies state and nonce; resolves `deferred` with `{code, state}`; 302-redirects to completion page.
    - `/` and static files (`authServer.ts:145–153`): serves files from `serveRoot` (the `../media` directory).
  - `authServer.ts:101–102` — Appends `app_name` and `app_is_sessions` query params to error redirect URLs using `env.appName` and `workspace.isAgentSessionsWorkspace`.

- `authServer.ts:156–189` — `start()`: listens on `0.0.0.0:0` (random port); on `'listening'` reads the port from `server.address()`, sets `this.state` to `http://127.0.0.1:{port}/callback?nonce=...`, resolves with port. Rejects after 5 s timeout.

- `authServer.ts:191–208` — `stop()`: calls `server.close()` wrapped in a promise.
- `authServer.ts:206–208` — `waitForOAuthResponse()`: returns `_resultPromise`.

---

### `node/fetch.ts`

**Role**: Multi-fetcher abstraction with automatic fallback ordering. Exports `fetching: Fetch` as the singleton fetch function used throughout the extension.

**Key symbols**:

- `fetch.ts:12–33` — `FetchOptions`, `FetchHeaders`, `FetchResponse`, `Fetch` type alias.
- `fetch.ts:37–66` — `_fetchers` array assembly:
  1. `fetch.ts:43–50` — Attempts `require('electron').net.fetch` (Electron fetch); silently ignores if unavailable.
  2. `fetch.ts:52–55` — `nodeFetch` object using global `fetch` (Node.js built-in).
  3. `fetch.ts:56–61` — Orders Electron fetch first (or Node fetch first) based on `github-authentication.useElectronFetch` config setting.
  4. `fetch.ts:63–66` — Always appends `nodeHTTP` (raw Node.js `http`/`https` module) as last fallback.

- `fetch.ts:68–77` — `createFetch()`: returns a closure that calls `fetchWithFallbacks` and may update the `fetchers` array locally (promoting a successful fallback fetcher).

- `fetch.ts:79–87` — `shouldNotRetry(status)`: returns `true` for 401, 403, 404, 429 — application-level errors where changing fetcher won't help.

- `fetch.ts:89–124` — `fetchWithFallbacks`: tries fetchers in order; promotes a later fetcher to front if it succeeds while the first fails (sticky preference). Returns first result for non-retryable status codes immediately.

- `fetch.ts:126–151` — `tryFetch`: wraps a single fetcher call; for JSON expectations eagerly reads response text and tries `JSON.parse` to confirm parseability; returns typed `{ok, response}` or `{ok:false, err}`.

- `fetch.ts:153` — `export const fetching = createFetch()` — module-level singleton.

- `fetch.ts:169–199` — `nodeHTTP(url, options)`: wraps Node.js `http.request`/`https.request`. Sets 60 s timeout for receiving data. Handles `AbortSignal` via `signal.addEventListener('abort', ...)`.

- `fetch.ts:201–258` — `NodeFetcherResponse`: adapts `http.IncomingMessage` to `FetchResponse`; implements streaming text via `'data'`/`'end'` events.

---

### `node/crypto.ts` and `node/buffer.ts`

- `node/crypto.ts:7` — Re-exports Node.js `webcrypto` as `crypto`. This is the platform-specific crypto import used in Node environments.
- `browser/crypto.ts:6` — Re-exports `globalThis.crypto` as `crypto`. Used in browser/web worker contexts where `webcrypto` is not a module import.
- `node/buffer.ts:6–8` — `base64Encode(text)`: uses Node.js `Buffer.from(text, 'binary').toString('base64')`. Used exclusively by `githubServer.ts` to construct the Basic auth header for token deletion.

---

### `common/keychain.ts`

**Role**: Thin wrapper around `vscode.ExtensionContext.secrets` (the VS Code secret storage API). Does not interact with the OS keychain directly.

**Key symbols**:

- `keychain.ts:10–14` — Constructor takes `context: vscode.ExtensionContext`, `serviceId: string`, `Logger: Log`.
- `keychain.ts:16–23` — `setToken(token)`: calls `context.secrets.store(serviceId, token)`.
- `keychain.ts:25–37` — `getToken()`: calls `context.secrets.get(serviceId)`; returns `null`/`undefined` (not throwing) on error.
- `keychain.ts:39–47` — `deleteToken()`: calls `context.secrets.delete(serviceId)`.

All operations silently swallow errors and log them.

---

### `common/utils.ts`

**Role**: Shared utility functions for event handling and data comparison.

**Key symbols**:

- `utils.ts:8–10` — `filterEvent<T>`: creates a filtered VS Code event (used in event wiring).
- `utils.ts:12–21` — `onceEvent<T>`: creates a once-only VS Code event.
- `utils.ts:24–32` — `PromiseAdapter<T,U>` interface: three-argument callback `(value, resolve, reject)`.
- `utils.ts:50–78` — `promiseFromEvent<T,U>(event, adapter?)`: converts a VS Code `Event<T>` to a `{promise, cancel}` pair. The `cancel` emitter lets callers abort. Used by `UriEventHandler.waitForCode` and `LocalServerFlow.trigger` for cancellation-token integration.
- `utils.ts:80–100` — `arrayEquals<T>`: element-wise equality check. Used for session scope matching.
- `utils.ts:103–118` — `StopWatch`: simple `Date.now()`-based timer.

---

### `common/env.ts`

**Role**: Helper predicates for classifying the runtime client and target.

**Key symbols**:

- `env.ts:8–20` — `VALID_DESKTOP_CALLBACK_SCHEMES`: array of acceptable VS Code URI schemes for the callback. Notably excludes `code-oss` with a comment about Windows browser redirect issues.
- `env.ts:22–30` — `isSupportedClient(uri)`: returns `true` if the callback URI uses a known desktop scheme or matches `*.vscode.dev` / `*.github.dev` authority patterns.
- `env.ts:32–37` — `isSupportedTarget(type, gheUri?)`: returns `true` for dotcom or hosted GHE (`.ghe.com`). On-prem GHES returns `false`.
- `env.ts:39–41` — `isHostedGitHubEnterprise(uri)`: tests authority against `/\.ghe\.com$/`.

---

### `config.ts`

**Role**: Holds OAuth client credentials. `gitHubClientSecret` is explicitly absent from the committed version and injected at publish time.

- `config.ts:6–10` — `IConfig` interface.
- `config.ts:17–19` — `Config` export: `gitHubClientId = '01ab8ac9400c4e429b23'`. Comment at `config.ts:14–16` explains the intentional inclusion of client ID in source code and that the secret is added pre-publish.

---

### `common/experimentationService.ts`

**Role**: Wraps `@vscode/extension-telemetry`'s `TelemetryReporter` with the `vscode-tas-client` A/B testing service.

**Key symbols**:

- `experimentationService.ts:10` — `ExperimentationTelemetry implements IExperimentationTelemetry`.
- `experimentationService.ts:16–41` — `createExperimentationService()`: maps `vscode.env.uriScheme` to a `TargetPopulation` (Public / Insiders / Internal / Team), then calls `getExperimentationService(id, version, population, this, globalState)` and awaits `initialFetch`.
- `experimentationService.ts:46–60` — `sendTelemetryEvent`: lazily initialises the experimentation service on first call, then delegates to `baseReporter.sendTelemetryEvent` merging `sharedProperties`.
- `experimentationService.ts:81–92` — `postEvent`: bridge for the `IExperimentationTelemetry` contract, converts a `Map<string,string>` to an object and calls `sendTelemetryEvent`.

---

## Cross-Cutting Synthesis

The GitHub Authentication extension is a layered OAuth provider built entirely on top of the VS Code Extension API (`vscode.AuthenticationProvider`). It does not directly own credential storage; instead it mediates between the VS Code secret store (`context.secrets`, which VS Code itself backs by the OS keychain via Electron's `safeStorage`) and the GitHub OAuth endpoints.

The core architectural pattern is a **Strategy chain** for OAuth flows. `flows.ts` encodes four strategies — `LocalServerFlow`, `UrlHandlerFlow`, `DeviceCodeFlow`, `PatFlow` — each carrying a capability matrix (`IFlowOptions`) that the `getFlows` selector filters against the runtime context (`GitHubTarget` × `ExtensionHost` × client type × client-secret availability). This means the login path is not hardcoded: the same `createSession` call can follow entirely different network paths depending on whether it runs in a local Electron window, a remote extension host, a web worker, or against on-prem GHES.

All HTTP I/O uses a bespoke `fetching` facade (`node/fetch.ts`) that wraps three possible fetch backends — Electron `net.fetch`, Node.js built-in `fetch`, and Node.js `http`/`https` — with automatic fallback promotion and explicit no-retry rules for application-level HTTP errors.

Platform divergence is handled by two parallel directory trees: `node/` vs `browser/`. `node/crypto.ts` re-exports `webcrypto` from Node's `crypto` module; `browser/crypto.ts` re-exports `globalThis.crypto`. `browser/authServer.ts` is a throw-stub since the local loopback server cannot run in a browser context. Both crypto variants expose the same surface (`getRandomValues`, `subtle.digest`), allowing `flows.ts` to import from `./node/crypto` without any conditional branching.

Session state is held in a single lazily-populated promise (`_sessionsPromise` in `GitHubAuthenticationProvider`) that acts as an in-memory cache. Cross-window synchronisation is handled by subscribing to `context.secrets.onDidChange` and diffing the fresh read against the cached version, firing `_sessionChangeEmitter` with added/removed arrays.

The 5-minute timeout hardcoded at `github.ts:91` and mirrored at `flows.ts:365` (local server) and `flows.ts:364` (URL handler) represents the maximum wall-clock time allotted for the user to complete browser-based authentication.

---

## Out-of-Partition References

The following cross-partition dependencies are invoked or imported:

- **`vscode` (VS Code Extension API)** — consumed pervasively. All authentication registration (`vscode.authentication.registerAuthenticationProvider`), secret storage (`context.secrets`), URI handling (`vscode.window.registerUriHandler`, `vscode.env.asExternalUri`), and progress UI (`vscode.window.withProgress`) are VS Code host APIs, not implemented within this extension.

- **`workbench.getCodeExchangeProxyEndpoints`** (VS Code internal command, `githubServer.ts:60`) — resolves the proxy endpoint for GitHub code exchange on vscode.dev. Implemented in VS Code workbench, outside this extension.

- **`workbench.action.reloadWindow`** (VS Code command, `extension.ts:98`) — reloads the entire workbench when the fetch setting changes.

- **`@vscode/extension-telemetry`** (npm package, `github.ts:7`, `experimentationService.ts:7`) — `TelemetryReporter` class; published by Microsoft separately.

- **`vscode-tas-client`** (npm package, `experimentationService.ts:8`) — `getExperimentationService`, `IExperimentationService`, `TargetPopulation`; the A/B testing client.

- **`electron` module** (`node/fetch.ts:45`) — `require('electron').net.fetch` for the Electron-native network stack. Only available in the Electron renderer/main process; caught with a try/catch when absent.

- **`../media/` directory** (static HTML/SVG, `flows.ts:357`, `authServer.ts:12`) — the loopback server serves these files as the post-redirect landing page. This directory is bundled with the extension but exists outside `src/`.

- **Node.js built-ins**: `http`, `https`, `fs`, `path`, `url`, `crypto` (from Node.js), `stream` — used in `node/authServer.ts`, `node/fetch.ts`, `node/buffer.ts`. All unavailable in browser/web worker contexts.

- **`workspace.isAgentSessionsWorkspace`** (`authServer.ts:102`) — a VS Code API property that indicates whether the workspace is an Agent Sessions workspace, used to append a query parameter to the loopback server redirect URLs. This is a VS Code workbench concept outside this partition.

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# VS Code GitHub Authentication Extension: Tauri/Rust Port Patterns

## Research Question
What patterns exist in the VS Code GitHub authentication extension that would need to be ported from TypeScript/Electron to Tauri/Rust?

## Scope
`extensions/github-authentication/` — OAuth flow with localhost server (24 files, ~3,104 LOC)

---

#### Pattern: Localhost HTTP Server for OAuth Callback

**Where:** `extensions/github-authentication/src/node/authServer.ts:103-153`

**What:** Creates an HTTP server listening on 127.0.0.1 that handles OAuth callback routes and serves static files.

```typescript
this._server = http.createServer((req, res) => {
    const reqUrl = new URL(req.url!, `http://${req.headers.host}`);
    switch (reqUrl.pathname) {
        case '/signin': {
            const receivedNonce = (reqUrl.searchParams.get('nonce') ?? '').replace(/ /g, '+');
            if (receivedNonce !== this.nonce) {
                res.writeHead(302, { location: `/?error=${encodeURIComponent('Nonce does not match.')}...` });
                res.end();
            }
            res.writeHead(302, { location: this._startingRedirect.toString() });
            res.end();
            break;
        }
        case '/callback': {
            const code = reqUrl.searchParams.get('code') ?? undefined;
            const state = reqUrl.searchParams.get('state') ?? undefined;
            const nonce = (reqUrl.searchParams.get('nonce') ?? '').replace(/ /g, '+');
            if (!code || !state || !nonce) {
                res.writeHead(400);
                res.end();
                return;
            }
            deferred.resolve({ code, state });
            res.writeHead(302, { location: `/?redirect_uri=${encodeURIComponent(callbackUri)}...` });
            res.end();
            break;
        }
        case '/':
            sendFile(res, path.join(serveRoot, 'index.html'));
            break;
        default:
            sendFile(res, path.join(serveRoot, reqUrl.pathname.substring(1)));
            break;
    }
});
```

**Variations / call-sites:**
- Also used in test mock server at `src/test/node/fetch.test.ts:21` with similar pattern
- Browser variant at `src/browser/authServer.ts:6-12` throws "Not implemented" (web workers cannot run servers)

---

#### Pattern: Dynamic Port Binding and Startup

**Where:** `extensions/github-authentication/src/node/authServer.ts:156-189`

**What:** Server binds to port 0 (OS-assigned), retrieves actual port via 'listening' event, and establishes nonce-based state URL for redirect.

```typescript
public start(): Promise<number> {
    return new Promise<number>((resolve, reject) => {
        if (this._server.listening) {
            throw new Error('Server is already started');
        }
        const portTimeout = setTimeout(() => {
            reject(new Error('Timeout waiting for port'));
        }, 5000);
        this._server.on('listening', () => {
            const address = this._server.address();
            if (typeof address === 'string') {
                this.port = parseInt(address);
            } else if (address instanceof Object) {
                this.port = address.port;
            } else {
                throw new Error('Unable to determine port');
            }

            clearTimeout(portTimeout);

            // set state which will be used to redirect back to vscode
            this.state = `http://127.0.0.1:${this.port}/callback?nonce=${encodeURIComponent(this.nonce)}`;

            resolve(this.port);
        });
        this._server.on('error', err => {
            reject(new Error(`Error listening to server: ${err}`));
        });
        this._server.on('close', () => {
            reject(new Error('Closed'));
        });
        this._server.listen(0, '127.0.0.1');
    });
}
```

**Variations / call-sites:**
- Port number wrapped in `Promise` for async startup
- 5-second timeout for port acquisition
- Used in LocalServerFlow at `src/flows.ts:358` where port is exposed to user

---

#### Pattern: PKCE Code Challenge Generation

**Where:** `extensions/github-authentication/src/flows.ts:121-146`

**What:** Generates cryptographically secure random strings and SHA-256-based code challenges for PKCE OAuth flow.

```typescript
function generateRandomString(length: number): string {
    const array = new Uint8Array(length);
    crypto.getRandomValues(array);
    return Array.from(array)
        .map(b => b.toString(16).padStart(2, '0'))
        .join('')
        .substring(0, length);
}

async function generateCodeChallenge(codeVerifier: string): Promise<string> {
    const encoder = new TextEncoder();
    const data = encoder.encode(codeVerifier);
    const digest = await crypto.subtle.digest('SHA-256', data);

    // Base64url encode the digest
    const base64String = btoa(String.fromCharCode(...new Uint8Array(digest)));
    return base64String
        .replace(/\+/g, '-')
        .replace(/\//g, '_')
        .replace(/=+$, '');
}
```

**Variations / call-sites:**
- Used in UrlHandlerFlow at `src/flows.ts:239-240`
- Used in LocalServerFlow at `src/flows.ts:329-330`
- Crypto module from Node.js `webcrypto` at `src/node/crypto.ts:6-8`

---

#### Pattern: OAuth Token Exchange via HTTP POST

**Where:** `extensions/github-authentication/src/flows.ts:148-195`

**What:** Exchanges OAuth authorization code for access token using URLSearchParams and HTTP POST with PKCE verifier.

```typescript
async function exchangeCodeForToken(
    logger: Log,
    endpointUri: Uri,
    redirectUri: Uri,
    code: string,
    codeVerifier: string,
    enterpriseUri?: Uri
): Promise<string> {
    logger.info('Exchanging code for token...');

    const clientSecret = Config.gitHubClientSecret;
    if (!clientSecret) {
        throw new Error('No client secret configured for GitHub authentication.');
    }

    const body = new URLSearchParams([
        ['code', code],
        ['client_id', Config.gitHubClientId],
        ['redirect_uri', redirectUri.toString(true)],
        ['client_secret', clientSecret],
        ['code_verifier', codeVerifier]
    ]);
    if (enterpriseUri) {
        body.append('github_enterprise', enterpriseUri.toString(true));
    }
    const result = await fetching(endpointUri.toString(true), {
        logger,
        retryFallbacks: true,
        expectJSON: true,
        method: 'POST',
        headers: {
            Accept: 'application/json',
            'Content-Type': 'application/x-www-form-urlencoded',
        },
        body: body.toString()
    });

    if (result.ok) {
        const json = await result.json();
        logger.info('Token exchange success!');
        return json.access_token;
    } else {
        const text = await result.text();
        const error = new Error(text);
        error.name = 'GitHubTokenExchangeError';
        throw error;
    }
}
```

**Variations / call-sites:**
- Called from UrlHandlerFlow at `src/flows.ts:281`
- Called from LocalServerFlow at `src/flows.ts:375-381`
- Uses `fetching()` abstraction from `src/node/fetch.ts` for multiple fallback implementations

---

#### Pattern: Multi-Flow OAuth Strategy Selection

**Where:** `extensions/github-authentication/src/flows.ts:287-385`

**What:** System tries multiple OAuth flows in sequence (LocalServer, UrlHandler, DeviceCode, PAT) based on environment constraints, with fallback prompts.

```typescript
class LocalServerFlow implements IFlow {
    label = l10n.t('local server');
    options: IFlowOptions = {
        supportsGitHubDotCom: true,
        supportsGitHubEnterpriseServer: false,
        supportsRemoteExtensionHost: false,  // Can't open port on remote
        supportsWebWorkerExtensionHost: false,  // Can't open port in browser
        supportsNoClientSecret: false,
        supportsSupportedClients: true,
        supportsUnsupportedClients: true
    };
    async trigger({...}: IFlowTriggerOptions): Promise<string> {
        // Implementation creates LoopbackAuthServer and waits for OAuth response
    }
}

export function getFlows(query: IFlowQuery) {
    const validFlows = allFlows.filter(flow => {
        let useFlow: boolean = true;
        switch (query.target) {
            case GitHubTarget.DotCom:
                useFlow &&= flow.options.supportsGitHubDotCom;
                break;
            // ... more conditions
        }
        return useFlow;
    });
    // Respects user preference for device code flow
    const preferDeviceCodeFlow = workspace.getConfiguration('github-authentication').get<boolean>('preferDeviceCodeFlow', false);
    if (preferDeviceCodeFlow) {
        return [
            ...validFlows.filter(flow => flow instanceof DeviceCodeFlow),
            ...validFlows.filter(flow => !(flow instanceof DeviceCodeFlow))
        ];
    }
    return validFlows;
}
```

**Variations / call-sites:**
- LocalServerFlow at `src/flows.ts:287-385`
- UrlHandlerFlow at `src/flows.ts:197-285`
- DeviceCodeFlow at `src/flows.ts:387-520`
- PatFlow at `src/flows.ts:522-605`
- Flow selection in GitHubServer.login() at `src/githubServer.ts:90-150`

---

#### Pattern: Secure Credential Storage via Secrets API

**Where:** `extensions/github-authentication/src/common/keychain.ts:9-48`

**What:** Wraps VS Code extension secrets storage for persisting OAuth sessions and tokens.

```typescript
export class Keychain {
    constructor(
        private readonly context: vscode.ExtensionContext,
        private readonly serviceId: string,
        private readonly Logger: Log
    ) { }

    async setToken(token: string): Promise<void> {
        try {
            return await this.context.secrets.store(this.serviceId, token);
        } catch (e) {
            this.Logger.error(`Setting token failed: ${e}`);
        }
    }

    async getToken(): Promise<string | null | undefined> {
        try {
            const secret = await this.context.secrets.get(this.serviceId);
            if (secret && secret !== '[]') {
                this.Logger.trace('Token acquired from secret storage.');
            }
            return secret;
        } catch (e) {
            this.Logger.error(`Getting token failed: ${e}`);
            return Promise.resolve(undefined);
        }
    }

    async deleteToken(): Promise<void> {
        try {
            return await this.context.secrets.delete(this.serviceId);
        } catch (e) {
            this.Logger.error(`Deleting token failed: ${e}`);
            return Promise.resolve(undefined);
        }
    }
}
```

**Variations / call-sites:**
- Initialized in GitHubAuthenticationProvider at `src/github.ts:153-158`
- Uses service ID format: `github.auth` or `{authority}{path}.ghes.auth` for GitHub Enterprise
- Session data persisted as JSON string

---

#### Pattern: Nonce-Based CSRF Protection

**Where:** `extensions/github-authentication/src/node/authServer.ts:76-177` and `src/flows.ts:99-111`

**What:** Generates and validates nonces to prevent cross-site request forgery in OAuth flow; state parameter includes nonce in callback URL.

```typescript
// In LoopbackAuthServer constructor
public nonce = randomBytes(16).toString('base64');

// In request handler
case '/signin': {
    const receivedNonce = (reqUrl.searchParams.get('nonce') ?? '').replace(/ /g, '+');
    if (receivedNonce !== this.nonce) {
        res.writeHead(302, { location: `/?error=${encodeURIComponent('Nonce does not match.')}...` });
        res.end();
    }
    // ... redirect to GitHub
}

case '/callback': {
    const nonce = (reqUrl.searchParams.get('nonce') ?? '').replace(/ /g, '+');
    if (this.nonce !== nonce) {
        res.writeHead(302, { location: `/?error=${encodeURIComponent('Nonce does not match.')}...` });
        res.end();
        throw new Error('Nonce does not match.');
    }
    deferred.resolve({ code, state });
    // ... continue
}

// In GitHubServer.login()
const nonce: string = crypto.getRandomValues(new Uint32Array(2)).reduce((prev, curr) => prev += curr.toString(16), '');
// Nonce passed through: callbackUri → state parameter → UriEventHandler → waitForCode()
```

**Variations / call-sites:**
- Nonce generation at `src/githubServer.ts:111`
- Validation in multiple flows: local server, URL handler
- Nonce validation in UriEventHandler at `src/github.ts:78-128`

---

## Summary

The GitHub authentication extension demonstrates seven critical patterns for Tauri/Rust porting:

1. **HTTP Server Architecture**: Node.js `http.createServer()` for localhost OAuth callback handling with route matching and file serving
2. **Dynamic Port Management**: OS-assigned port binding with event-driven port discovery and timeout handling
3. **Cryptographic PKCE**: SHA-256 hashing, base64url encoding, and random byte generation for secure OAuth
4. **Token Exchange Protocol**: URLSearchParams form data construction and JSON response parsing for OAuth token endpoint
5. **Pluggable Auth Flows**: Strategy pattern supporting 4 different OAuth flows with capability-based selection
6. **Secure Storage**: Abstraction over platform-native credential storage (VS Code secrets) for tokens and sessions
7. **CSRF Protection**: Nonce generation and validation throughout the redirect flow chain

Key constraints for Tauri port:
- Must provide HTTP server library (or use Tauri's built-in HTTP functionality)
- Cryptographic APIs (SHA-256, base64url) are well-supported in Rust ecosystem (`sha2`, `base64` crates)
- Secure storage requires platform integration (system keychain on macOS/Windows, secret-service on Linux)
- Flow selection logic can map directly to Rust trait implementations
- URL parsing and construction patterns are similar in Rust (`url` crate)

The architecture is fundamentally sound for cross-platform porting but requires careful attention to the HTTP server component, which is Tauri-specific, and the credential storage abstraction, which must bridge Tauri's plugin system to platform keychains.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
