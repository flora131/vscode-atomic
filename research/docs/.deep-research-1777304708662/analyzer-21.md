### Files Analysed

1. `extensions/microsoft-authentication/src/extension.ts`
2. `extensions/microsoft-authentication/src/node/authProvider.ts`
3. `extensions/microsoft-authentication/src/node/cachedPublicClientApplication.ts`
4. `extensions/microsoft-authentication/src/node/flows.ts`
5. `extensions/microsoft-authentication/src/node/publicClientCache.ts`
6. `extensions/microsoft-authentication/src/common/cachePlugin.ts`
7. `extensions/microsoft-authentication/src/betterSecretStorage.ts`
8. `extensions/microsoft-authentication/src/UriEventHandler.ts`
9. `extensions/microsoft-authentication/src/common/loopbackClientAndOpener.ts`
10. `extensions/microsoft-authentication/src/common/publicClientCache.ts`

---

### Per-File Notes

#### `extensions/microsoft-authentication/src/extension.ts`

- **Role:** Extension entry point. Bootstraps both the primary `microsoft` authentication provider and optionally a `microsoft-sovereign-cloud` provider. Registers them with VS Code's authentication API. Also listens for configuration changes to allow runtime provider switching.
- **Key symbols:**
  - `activate` (`extension.ts:72`) — async activation function; creates `UriEventHandler`, calls `MsalAuthProvider.create()`, then calls `authentication.registerAuthenticationProvider()`
  - `initMicrosoftSovereignCloudAuthProvider` (`extension.ts:16`) — conditionally creates a sovereign cloud provider based on `microsoft-sovereign-cloud.environment` workspace config; handles the `custom` environment variant via `@azure/ms-rest-azure-env`
  - `getImplementation` (`extension.ts:14`) — reads `microsoft-authentication.implementation` setting to determine `'msal'` vs `'msal-no-broker'`
  - `deactivate` (`extension.ts:142`) — minimal; logs shutdown
- **Control flow:**
  - `activate` reads the `implementation` setting at line 74
  - Registers a `onDidChangeConfiguration` handler at line 75 that prompts a window reload when the `microsoft-authentication` configuration changes
  - Instantiates `UriEventHandler` at line 110
  - Awaits `MsalAuthProvider.create(...)` at line 112
  - Calls `authentication.registerAuthenticationProvider('microsoft', 'Microsoft', authProvider, {...})` at line 118, passing `supportedAuthorizationServers` pointing to `https://login.microsoftonline.com/*`
  - Optionally initializes sovereign cloud provider at line 132 and re-initializes on config changes via second `onDidChangeConfiguration` handler at line 134
- **Data flow:** `context.extension.packageJSON.aiKey` → telemetry reporter; `context.secrets` / `context.globalState` → forwarded to `MsalAuthProvider`; `UriEventHandler` shared between both providers for OAuth redirect
- **Dependencies:** `vscode` (authentication, commands, workspace, window, Uri, env); `@azure/ms-rest-azure-env`; `./node/authProvider`; `./UriEventHandler`; `./common/telemetryReporter`

---

#### `extensions/microsoft-authentication/src/node/authProvider.ts`

- **Role:** Core `AuthenticationProvider` implementation registered with VS Code. Manages the complete Microsoft identity session lifecycle: listing sessions, creating sessions (interactive auth), removing sessions, and challenge-based session creation. Coordinates with MSAL via `ICachedPublicClientApplicationManager`.
- **Key symbols:**
  - `MsalAuthProvider` (`authProvider.ts:39`) — class implementing `AuthenticationProvider`
  - `MsalAuthProvider.create` (`authProvider.ts:99`) — static factory; creates `CachedPublicClientApplicationManager`, then calls `new MsalAuthProvider(...)` and `authProvider.initialize()`
  - `initialize` (`authProvider.ts:141`) — runs migration (`_migrateSessions`) if `globalState.get('msalMigration')` is false
  - `_migrateSessions` (`authProvider.ts:117`) — reads legacy sessions from `BetterTokenStorage<IStoredSession>` keyed at `'microsoft.login.keylist'`; groups them by `clientId:tenant`; calls `publicClientManager.getOrCreate(clientId, { refreshTokensToMigrate })` to port refresh tokens into MSAL
  - `getSessions` (`authProvider.ts:171`) — handles two cases: (1) `scopes === undefined` returns one synthetic session per unique `homeAccountId` across all PCAs; (2) specific scopes use `getOrCreate` then `getAllSessionsForPca`
  - `createSession` (`authProvider.ts:202`) — iterates flows returned by `getMsalFlows()`; calls `flow.trigger(...)` in order; on success calls `sessionFromAuthenticationResult`
  - `removeSession` (`authProvider.ts:278`) — iterates all PCAs, finds matching `homeAccountId`, calls `cachedPca.removeAccount(account)`
  - `getSessionsFromChallenges` / `createSessionFromChallenges` (`authProvider.ts:306`, `authProvider.ts:328`) — challenge-based auth; parses `Bearer` challenge for `scope` and `claims` (base64 decoded at line 431)
  - `getAllSessionsForPca` (`authProvider.ts:448`) — for each account in the PCA: determines whether to force-refresh based on tenant mismatch or id-token expiry; calls `cachedPca.acquireTokenSilent`; buffers events via `_eventBufferer`
  - `sessionFromAuthenticationResult` (`authProvider.ts:541`) — maps `AuthenticationResult` → `AuthenticationSession`; uses `result.account?.homeAccountId` as session `id`
  - `sessionFromAccountInfo` (`authProvider.ts:554`) — creates a synthetic session with placeholder `accessToken: '1234'` for account-list display
- **Control flow:**
  - Constructor wires `_publicClientManager.onDidAccountsChange` through `EventBufferer.wrapEvent` to deduplicate rapid events before firing `_onDidChangeSessionsEmitter`
  - `createSession` builds a `callbackUri` using `env.asExternalUri`, determines `ExtensionHost` from `extensionKind`, calls `getMsalFlows` to get an ordered list of auth flows, then tries each in sequence with user prompting between attempts
  - macOS broker redirect URI is conditionally included at line 512 when `cachedPca.isBrokerAvailable && process.platform === 'darwin'`
- **Data flow:** `ScopeData` class wraps raw scopes, extracting `clientId`, `tenant`, `tenantId`, `scopesToSend`, `originalScopes`, and `claims`; `AuthenticationResult` from MSAL → `AuthenticationSession` returned to VS Code host
- **Dependencies:** `@azure/msal-node`; `vscode`; `./publicClientCache`; `../UriEventHandler`; `../common/publicClientCache`; `../common/telemetryReporter`; `../common/scopeData`; `../common/event`; `../betterSecretStorage`; `./flows`; `./buffer`; `../common/config`; `../common/env`

---

#### `extensions/microsoft-authentication/src/node/cachedPublicClientApplication.ts`

- **Role:** Wraps MSAL's `PublicClientApplication` with VS Code-specific concerns: secret-storage-backed token cache, native broker integration, a sequencer for serializing MSAL operations, and event emission for account changes.
- **Key symbols:**
  - `CachedPublicClientApplication` (`cachedPublicClientApplication.ts:16`) — implements `ICachedPublicClientApplication`
  - Constructor (`cachedPublicClientApplication.ts:39`) — initializes `SecretStorageCachePlugin` at line 46, configures `NativeBrokerPlugin` if not in web UI and not disabled via settings (lines 54–65), builds `PublicClientApplication` at line 66 wiring the `cachePlugin` and optional broker
  - `acquireTokenSilent` (`cachedPublicClientApplication.ts:111`) — queues via `_sequencer`; after result, checks id-token expiry at 5-min threshold (line 118); broker-specific workaround at line 127 injects claims `'{ "id_token": {}}'` instead of `forceRefresh: true` because broker doesn't support `forceRefresh`
  - `acquireTokenInteractive` (`cachedPublicClientApplication.ts:173`) — wraps in `window.withProgress` with cancellable notification; uses 5-minute timeout via `raceCancellationAndTimeoutError`; calls `_accountAccess.setAllowedAccess` post-success if broker available
  - `acquireTokenByDeviceCode` (`cachedPublicClientApplication.ts:231`) — uses `DeferredPromise` to race MSAL device code flow against user cancellation; `_deviceCodeCallback` at line 258 shows modal with code, copies to clipboard via `env.clipboard.writeText`, opens browser
  - `acquireTokenByRefreshToken` (`cachedPublicClientApplication.ts:211`) — migration path only; queues via sequencer, calls `_pca.acquireTokenByRefreshToken`
  - `removeAccount` (`cachedPublicClientApplication.ts:306`) — for broker: delegates to `_accountAccess.setAllowedAccess(account, false)`; otherwise calls `_pca.getTokenCache().removeAccount(account)`
  - `_update` (`cachedPublicClientApplication.ts:348`) — clears in-memory MSAL cache, calls `_pca.getAllAccounts()`, filters by broker access, computes added/deleted diff, fires `_onDidAccountsChangeEmitter`
  - `_registerOnSecretStorageChanged` (`cachedPublicClientApplication.ts:313`) — for broker: listens to `_accountAccess.onDidAccountAccessChange`; otherwise: listens to `_secretStorageCachePlugin.onDidChange`
  - `_verifyIfUsingBroker` (`cachedPublicClientApplication.ts:321`) — tracks `iat` timestamps per `nativeAccountId` to deduplicate broker-sourced account-change events
  - `Sequencer` (`cachedPublicClientApplication.ts:377`) — simple promise chain: `queue(task)` appends to `this.current`; both resolve and reject paths execute the next task
- **Control flow:** All mutating MSAL operations pass through `_sequencer.queue()` to serialize requests; `_update()` is called after every interactive or refresh-token acquisition to sync the in-memory account list with the cache
- **Data flow:** `SecretStorage` → `SecretStorageCachePlugin` → MSAL `TokenCache` (`beforeCacheAccess`/`afterCacheAccess`); MSAL `AuthenticationResult` → returned to caller; `AccountInfo[]` → `_onDidAccountsChangeEmitter`
- **Dependencies:** `@azure/msal-node`; `@azure/msal-node-extensions` (NativeBrokerPlugin); `vscode`; `../common/async`; `../common/cachePlugin`; `../common/loggerOptions`; `../common/publicClientCache`; `../common/accountAccess`; `../common/telemetryReporter`

---

#### `extensions/microsoft-authentication/src/node/flows.ts`

- **Role:** Defines the three concrete OAuth interactive flows and the `getMsalFlows` selector that filters them based on runtime environment characteristics.
- **Key symbols:**
  - `IMsalFlow` (`flows.ts:40`) — interface with `label: string`, `options: IMsalFlowOptions`, and `trigger(options): Promise<AuthenticationResult>`
  - `IMsalFlowOptions` (`flows.ts:21`) — capability flags: `supportsRemoteExtensionHost`, `supportsUnsupportedClient`, `supportsBroker`, `supportsPortableMode`
  - `DefaultLoopbackFlow` (`flows.ts:46`) — uses MSAL's built-in loopback: calls `cachedPca.acquireTokenInteractive` with `openBrowser: url => env.openExternal(...)` and passes `loopbackTemplate` for success/error HTML; conditionally sets macOS broker redirect URI at line 58
  - `UrlHandlerFlow` (`flows.ts:76`) — uses `UriHandlerLoopbackClient` as the loopback client; passes `callbackUri` appended as `state` param to the auth URL; suited for remote extension host scenarios; `supportsRemoteExtensionHost: true`, `supportsBroker: false`
  - `DeviceCodeFlow` (`flows.ts:106`) — delegates entirely to `cachedPca.acquireTokenByDeviceCode`; fallback for environments where loopback and URI handler are unavailable; `supportsUnsupportedClient: true`, `supportsPortableMode: true`
  - `allFlows` (`flows.ts:125`) — module-level array: `[DefaultLoopbackFlow, UrlHandlerFlow, DeviceCodeFlow]`
  - `getMsalFlows` (`flows.ts:138`) — filters `allFlows` by comparing query fields against each flow's `options`; remote host excludes flows with `supportsRemoteExtensionHost: false`; broker exclusion at line 145: `flow.options.supportsBroker || !query.isBrokerSupported`
- **Control flow:** `getMsalFlows` returns an ordered slice of `allFlows`; `authProvider.ts` tries them in returned order, prompting user between attempts on failures
- **Data flow:** `IMsalFlowTriggerOptions` (containing `cachedPca`, `authority`, `scopes`, `callbackUri`, `loginHint`, `windowHandle`, `logger`, `uriHandler`, `claims`) → each flow's `trigger()` → `AuthenticationResult`
- **Dependencies:** `@azure/msal-node`; `vscode`; `../common/publicClientCache`; `../common/loopbackClientAndOpener`; `../UriEventHandler`; `./loopbackTemplate`; `../common/config`

---

#### `extensions/microsoft-authentication/src/node/publicClientCache.ts`

- **Role:** Manages a registry of `CachedPublicClientApplication` instances keyed by `clientId`. Persists the list of known client IDs in `SecretStorage`. Handles cross-window synchronization by listening for changes to the stored client ID list.
- **Key symbols:**
  - `CachedPublicClientApplicationManager` (`publicClientCache.ts:21`) — implements `ICachedPublicClientApplicationManager`; holds `_pcas: Map<string, ICachedPublicClientApplication>` and `_pcaDisposables: Map<string, Disposable>`
  - `CachedPublicClientApplicationManager.create` (`publicClientCache.ts:47`) — creates `PublicClientApplicationsSecretStorage`, runs old-format migration, creates `ScopedAccountAccess`, then instantiates manager and calls `initialize()`
  - `initialize` (`publicClientCache.ts:66`) — reads stored client IDs, calls `_doCreatePublicClientApplication` for each; prunes empty PCAs and re-saves
  - `getOrCreate` (`publicClientCache.ts:117`) — cache-hit: returns existing PCA; cache-miss: creates via `_doCreatePublicClientApplication` and stores updated list; migration path: calls `pca.acquireTokenByRefreshToken` for each old refresh token at line 140
  - `_doCreatePublicClientApplication` (`publicClientCache.ts:158`) — creates `CachedPublicClientApplication`, wires `onDidAccountsChange` to bubble up to manager's emitter, wires `onDidRemoveLastAccount` to self-dispose and remove from maps, fires initial add event if accounts exist
  - `_handleSecretStorageChange` (`publicClientCache.ts:186`) — reconciles in-memory PCA map against the stored client ID list; creates PCAs for IDs added in other windows
  - `PublicClientApplicationsSecretStorage` (`publicClientCache.ts:243`) — inner class that stores a JSON array of clientId strings under the key `publicClients-{cloudName}` in `SecretStorage`; migrates from old key `publicClientApplications-{cloudName}` that stored serialized objects with `clientId+authority`
- **Control flow:** Secret storage change on `publicClients-{cloudName}` → `_handleSecretStorageChange` → diff against in-memory PCAs → create new ones or dispose removed ones
- **Data flow:** `SecretStorage` → JSON array of clientId strings → `CachedPublicClientApplication` instances; `onDidAccountsChange` bubbles from individual PCAs through manager to `MsalAuthProvider`
- **Dependencies:** `@azure/msal-node`; `vscode`; `../common/publicClientCache`; `./cachedPublicClientApplication`; `../common/accountAccess`; `../common/telemetryReporter`; `@azure/ms-rest-azure-env`; `../common/config`; `../common/env`

---

#### `extensions/microsoft-authentication/src/common/cachePlugin.ts`

- **Role:** MSAL `ICachePlugin` adapter that bridges MSAL's token cache serialization API to VS Code's `SecretStorage`. Fires a change event when the underlying secret changes so `CachedPublicClientApplication` can trigger account refresh.
- **Key symbols:**
  - `SecretStorageCachePlugin` (`cachePlugin.ts:9`) — implements `ICachePlugin` and `Disposable`
  - `beforeCacheAccess` (`cachePlugin.ts:35`) — reads the current secret value via `_secretStorage.get(this._key)`, deserializes it into the MSAL `TokenCacheContext`
  - `afterCacheAccess` (`cachePlugin.ts:43`) — if `tokenCacheContext.cacheHasChanged`, serializes the cache and stores only if the new value differs from the last-read value `_value`
  - `_registerChangeHandler` (`cachePlugin.ts:27`) — listens to `_secretStorage.onDidChange`; fires `_onDidChange` when the key matches
- **Control flow:** MSAL calls `beforeCacheAccess` before any cache read, `afterCacheAccess` after any mutating operation; change handler fires asynchronously when another window updates the cached secret
- **Data flow:** MSAL `TokenCache.serialize()` → JSON string → `SecretStorage.store(key, value)`; `SecretStorage.get(key)` → `TokenCache.deserialize(data)`
- **Dependencies:** `@azure/msal-node`; `vscode`

---

#### `extensions/microsoft-authentication/src/betterSecretStorage.ts`

- **Role:** Serialized-access wrapper over VS Code's `SecretStorage` for storing typed token collections. Maintains an in-memory `Map<string, T>` as a cache and a separate keylist entry in `SecretStorage` to enumerate all known keys on startup. Emits cross-window change events.
- **Key symbols:**
  - `BetterTokenStorage<T>` (`betterSecretStorage.ts:15`) — generic typed secret storage
  - `_tokensPromise` (`betterSecretStorage.ts:20`) — serialization primitive; all operations await and replace this promise
  - `_operationInProgress` (`betterSecretStorage.ts:17`) — flag checked in `getTokens()` spin loop to handle concurrent mutations
  - `initialize` (`betterSecretStorage.ts:39`) — reads `keylistKey` to get list of keys, then fetches all values in parallel; builds initial `Map<string, T>`
  - `store` (`betterSecretStorage.ts:95`) — writes value, conditionally updates keylist if it's a new key
  - `delete` (`betterSecretStorage.ts:122`) — removes value and updates keylist atomically
  - `handleSecretChange` (`betterSecretStorage.ts:194`) — detects whether change originated in another window; fires `_didChangeInOtherWindow` with `added`, `updated`, or `removed` payloads
  - `getTokens` (`betterSecretStorage.ts:177`) — spin-loops on `_operationInProgress` flag before returning `_tokensPromise`
- **Control flow:** Every mutating method (`store`, `delete`) creates a new `_tokensPromise` that chains off the previous one, ensuring serial execution; `handleSecretChange` similarly chains via `_tokensPromise`
- **Data flow:** `SecretStorage` → JSON strings → parsed as `T` via `parseSecret`; typed objects → `serializeSecret` → JSON → `SecretStorage`; cross-window mutations → `onDidChangeInOtherWindow` event
- **Dependencies:** `vscode`; `./logger`

---

#### `extensions/microsoft-authentication/src/UriEventHandler.ts`

- **Role:** Bridges VS Code's URI handler protocol (deep links) to an `EventEmitter<Uri>` so OAuth redirect callbacks arriving via `vscode://vscode.microsoft-authentication` are converted to events.
- **Key symbols:**
  - `UriEventHandler` (`UriEventHandler.ts:8`) — extends `vscode.EventEmitter<vscode.Uri>`, implements `vscode.UriHandler`
  - `handleUri` (`UriEventHandler.ts:11`) — called by VS Code when a URI matching this extension arrives; fires the inherited emitter with the `Uri`
  - Constructor implicitly registers via `vscode.window.registerUriHandler(this)` at line 9
  - `dispose` (`UriEventHandler.ts:15`) — unregisters the URI handler
- **Control flow:** VS Code runtime calls `handleUri(uri)` on any incoming URI for this extension; `UriHandlerLoopbackClient.listenForAuthCode()` subscribes to `this._uriHandler.event` via `toPromise` to capture exactly one URI event
- **Data flow:** OS/browser redirect to `vscode://vscode.microsoft-authentication?code=...&state=...` → `handleUri` → emitter fires `Uri` → `UriHandlerLoopbackClient.listenForAuthCode` returns `ServerAuthorizationCodeResponse` parsed from URL search params
- **Dependencies:** `vscode`

---

#### `extensions/microsoft-authentication/src/common/loopbackClientAndOpener.ts`

- **Role:** Implements `ILoopbackClient` (MSAL interface) using VS Code's URI handler instead of a real loopback HTTP server. Used by `UrlHandlerFlow` for remote host scenarios.
- **Key symbols:**
  - `UriHandlerLoopbackClient` (`loopbackClientAndOpener.ts:15`) — implements `ILoopbackClientAndOpener`
  - `listenForAuthCode` (`loopbackClientAndOpener.ts:23`) — awaits one URI from `_uriHandler.event` via `toPromise`; parses `code`, `state`, `error`, `error_description`, `error_uri` from URL search params
  - `getRedirectUri` (`loopbackClientAndOpener.ts:36`) — always returns the constant `DEFAULT_REDIRECT_URI` (`'https://vscode.dev/redirect'`)
  - `closeServer` (`loopbackClientAndOpener.ts:42`) — no-op; no actual server to close
  - `openBrowser` (`loopbackClientAndOpener.ts:46`) — appends `&state={callbackUri}` to the auth URL; opens via `env.openExternal`; the state causes the identity provider to redirect back through `vscode.dev/redirect` which then triggers the local URI handler
- **Control flow:** MSAL calls `openBrowser(url)` then `listenForAuthCode()`; `openBrowser` encodes `callbackUri` as `state` so identity server redirects to `vscode.dev/redirect?code=...&state={callbackUri}` → vscode.dev redirects to local VS Code URI handler
- **Data flow:** MSAL auth URL → browser opens to Azure AD → auth code returned as query params to URI handler → `ServerAuthorizationCodeResponse` returned to MSAL to complete PKCE exchange
- **Dependencies:** `@azure/msal-node`; `vscode`; `../UriEventHandler`; `./async`

---

#### `extensions/microsoft-authentication/src/common/publicClientCache.ts`

- **Role:** Defines shared TypeScript interfaces for the public client application abstractions, decoupling the auth provider from the concrete Node.js MSAL implementation.
- **Key symbols:**
  - `ICachedPublicClientApplication` (`publicClientCache.ts:8`) — interface declaring all token acquisition methods (`acquireTokenSilent`, `acquireTokenInteractive`, `acquireTokenByDeviceCode`, `acquireTokenByRefreshToken`, `removeAccount`) plus `accounts`, `clientId`, `isBrokerAvailable`
  - `ICachedPublicClientApplicationManager` (`publicClientCache.ts:21`) — interface declaring `onDidAccountsChange`, `getOrCreate`, `getAll`
- **Control flow:** No logic; pure interface declarations used throughout the auth stack
- **Data flow:** Typed contracts; `AccountInfo`, `AuthenticationResult`, `SilentFlowRequest`, etc. imported from `@azure/msal-node`
- **Dependencies:** `@azure/msal-node`; `vscode`

---

### Cross-Cutting Synthesis

The Microsoft Authentication extension is a layered OAuth/OIDC stack built entirely on top of VS Code's extension host APIs and Microsoft's `@azure/msal-node` library. The entry point (`extension.ts`) instantiates `MsalAuthProvider` and registers it with `vscode.authentication.registerAuthenticationProvider`, giving VS Code's core the ability to request and revoke Microsoft identity sessions. `MsalAuthProvider` (`authProvider.ts`) implements the `AuthenticationProvider` contract and delegates all MSAL operations to the manager tier. The manager tier (`publicClientCache.ts` node) maintains a per-clientId registry of `CachedPublicClientApplication` instances, persisting the known client IDs in `SecretStorage` under `publicClients-{cloudName}` and reacting to cross-window changes. Each `CachedPublicClientApplication` (`cachedPublicClientApplication.ts`) holds a single MSAL `PublicClientApplication` instance, serializes all MSAL calls through a `Sequencer` promise chain, and uses `SecretStorageCachePlugin` (`cachePlugin.ts`) as the MSAL cache adapter — writing serialized token cache JSON into `SecretStorage` after each mutation and restoring it before each read. Interactive authentication runs through three pluggable flows (`flows.ts`): a default loopback (browser opens, MSAL hosts local redirect), a URI-handler flow (`loopbackClientAndOpener.ts`) suited for remote extension hosts where no localhost is reachable, and a device code flow for headless environments. The redirect callback in the URI-handler flow is received by `UriEventHandler` (`UriEventHandler.ts`) via `vscode.window.registerUriHandler` and surfaced as a one-shot event. Legacy pre-MSAL sessions stored in `BetterTokenStorage` (`betterSecretStorage.ts`) are migrated by re-exchanging old refresh tokens via `acquireTokenByRefreshToken`. For a Tauri/Rust port, every integration point here is a VS Code platform API (`SecretStorage`, `authentication.registerAuthenticationProvider`, `UriHandler`, `window.registerUriHandler`, `env.openExternal`, `env.asExternalUri`, `window.withProgress`, `context.secrets`, `context.globalState`) that would need Tauri equivalents, and the `@azure/msal-node` and `@azure/msal-node-extensions` Node.js libraries would need either a Rust MSAL crate or a WebAssembly wrapper.

---

### Out-of-Partition References

- `vscode` host APIs used throughout:
  - `vscode.authentication.registerAuthenticationProvider` — core IDE auth registry
  - `vscode.SecretStorage` — encrypted credential store; implementation is in VS Code core (`src/vs/workbench/services/secrets/`)
  - `vscode.window.registerUriHandler` — URI protocol dispatch; implemented in workbench core
  - `vscode.env.openExternal`, `vscode.env.asExternalUri`, `vscode.env.clipboard` — platform services
  - `vscode.window.withProgress`, `vscode.window.showInformationMessage`, `vscode.window.showWarningMessage` — UI notification APIs
  - `vscode.workspace.getConfiguration` — settings access
  - `context.globalState` (`Memento`) — global persisted state
- `@azure/msal-node` — Microsoft's OAuth2/OIDC client library for Node.js; source outside this repo
- `@azure/msal-node-extensions` (`NativeBrokerPlugin`) — Windows/macOS native credential broker; native binary, outside repo
- `@azure/ms-rest-azure-env` — Azure environment configuration library; outside repo
- `../common/scopeData` (`ScopeData` class) — scope parsing/normalization utility within the extension (`extensions/microsoft-authentication/src/common/scopeData.ts`)
- `../common/telemetryReporter` — telemetry helpers (`extensions/microsoft-authentication/src/common/telemetryReporter.ts`)
- `../common/accountAccess` (`ScopedAccountAccess`) — broker account allow-list stored in SecretStorage (`extensions/microsoft-authentication/src/common/accountAccess.ts`)
- `../common/event` (`EventBufferer`) — event deduplication utility (`extensions/microsoft-authentication/src/common/event.ts`)
- `../common/config` (`Config.macOSBrokerRedirectUri`) — static configuration constants (`extensions/microsoft-authentication/src/common/config.ts`)
- `../common/env` (`isSupportedClient`, `DEFAULT_REDIRECT_URI`) — runtime environment checks (`extensions/microsoft-authentication/src/common/env.ts`)
- `../common/async` (`DeferredPromise`, `raceCancellationAndTimeoutError`, `toPromise`) — async utilities (`extensions/microsoft-authentication/src/common/async.ts`)
- `./buffer` (`base64Decode`) — buffer utilities in node tier (`extensions/microsoft-authentication/src/node/buffer.ts`)
- `./loopbackTemplate` — HTML string for loopback redirect page (`extensions/microsoft-authentication/src/node/loopbackTemplate.ts`)
