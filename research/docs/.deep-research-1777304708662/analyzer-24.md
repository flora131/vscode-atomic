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
