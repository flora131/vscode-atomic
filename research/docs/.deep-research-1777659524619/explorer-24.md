# Partition 24 of 79 — Findings

## Scope
`extensions/github-authentication/` (24 files, 3,104 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
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

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
_(no analysis produced)_

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# VS Code GitHub Authentication Patterns for Tauri/Rust Porting

## Overview
This analysis examines the GitHub authentication extension patterns in VS Code, focusing on authentication provider registration, secret storage, OAuth flows, and protocol handling. These patterns are critical for understanding what would be needed to port VS Code's authentication infrastructure to Tauri/Rust.

---

## Pattern 1: Authentication Provider Registration

**Where:** `extensions/github-authentication/src/github.ts:179-189`

**What:** The provider registers itself with the vscode.authentication API, declaring supported authorization servers and multi-account support. This is the entry point for all authentication flows.

```typescript
vscode.authentication.registerAuthenticationProvider(
    type,
    this._githubServer.friendlyName,
    this,
    {
        supportsMultipleAccounts: true,
        supportedAuthorizationServers
    }
)
```

**Variations:**
- `extensions/github-authentication/src/extension.ts:19` - GitHub Enterprise provider registration
- Fallback null provider when configuration is invalid (lines 12-37 in extension.ts)

**Porting Implications:**
- Tauri/Rust requires equivalent provider interface with session management callbacks
- Must support multiple account types (GitHub.com, Enterprise, Hosted Enterprise)
- Authorization server declarations need platform-equivalent mechanism for URL scheme handling

---

## Pattern 2: Session Storage via ExtensionContext Secrets

**Where:** `extensions/github-authentication/src/common/keychain.ts:16-47`

**What:** Credentials are stored/retrieved through the vscode ExtensionContext secrets API, which abstracts platform-specific secure storage (macOS Keychain, Windows Credential Manager, Linux secret-tool).

```typescript
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
```

**Key Aspects:**
- Service ID format: `type.auth` (e.g., `github.auth`) or `authority.path.ghes.auth`
- Stores JSON-stringified session arrays
- Gracefully handles missing secrets and parsing errors
- Listens for external changes via `context.secrets.onDidChange()` for multi-window sync

**Porting Implications:**
- Rust requires bindings to native credential managers (keyring crate, tauri-plugin-keyring)
- Must maintain same JSON session format for compatibility
- Need equivalent cross-window secret change events

---

## Pattern 3: Multi-Flow OAuth Architecture

**Where:** `extensions/github-authentication/src/flows.ts:197-285, 287-385, 387-454`

**What:** Three fallback OAuth flows ordered by platform capabilities and client environment.

### Flow 3.1: URL Handler Flow (No Local Server)
**Location:** `flows.ts:197-285`

```typescript
class UrlHandlerFlow implements IFlow {
    label = l10n.t('url handler');
    options: IFlowOptions = {
        supportsGitHubDotCom: true,
        supportsGitHubEnterpriseServer: false,
        supportsHostedGitHubEnterprise: true,
        supportsRemoteExtensionHost: true,
        supportsWebWorkerExtensionHost: true,
        supportsNoClientSecret: false,
        supportsSupportedClients: true,
        supportsUnsupportedClients: false
    };

    async trigger({...}: IFlowTriggerOptions): Promise<string> {
        const codeVerifier = generateRandomString(64);
        const codeChallenge = await generateCodeChallenge(codeVerifier);
        const promise = uriHandler.waitForCode(logger, scopes, nonce, token);
        
        const searchParams = new URLSearchParams([
            ['client_id', Config.gitHubClientId],
            ['redirect_uri', redirectUri.toString(true)],
            ['scope', scopes],
            ['state', encodeURIComponent(callbackUri.toString(true))],
            ['code_challenge', codeChallenge],
            ['code_challenge_method', 'S256']
        ]);
        
        const uri = Uri.parse(baseUri.with({
            path: '/login/oauth/authorize',
            query: searchParams.toString()
        }).toString(true));
        await env.openExternal(uri);
        
        const code = await promise;
        const endpointUrl = proxyEndpoints?.github
            ? Uri.parse(`${proxyEndpoints.github}login/oauth/access_token`)
            : baseUri.with({ path: '/login/oauth/access_token' });
        
        const accessToken = await exchangeCodeForToken(logger, endpointUrl, redirectUri, code, codeVerifier, enterpriseUri);
        return accessToken;
    }
}
```

**Key Aspects:**
- Uses PKCE (Proof Key for Public Clients) with SHA-256 challenges
- Opens OAuth authorize URL in default browser
- Waits for redirect via protocol handler (vscode://vscode.github-authentication/did-authenticate)
- No client secret required for token exchange (PKCE-protected)
- Supports web environments (vscode.dev)

### Flow 3.2: Local Server Flow (Loopback Redirect)
**Location:** `flows.ts:287-385`

```typescript
class LocalServerFlow implements IFlow {
    label = l10n.t('local server');
    options: IFlowOptions = {
        supportsGitHubDotCom: true,
        supportsGitHubEnterpriseServer: false,
        supportsHostedGitHubEnterprise: true,
        supportsRemoteExtensionHost: false,
        supportsWebWorkerExtensionHost: false,
        supportsNoClientSecret: false,
        supportsSupportedClients: true,
        supportsUnsupportedClients: true
    };

    async trigger({...}: IFlowTriggerOptions): Promise<string> {
        const codeVerifier = generateRandomString(64);
        const codeChallenge = await generateCodeChallenge(codeVerifier);
        
        const loginUrl = baseUri.with({
            path: '/login/oauth/authorize',
            query: searchParams.toString()
        });
        const server = new LoopbackAuthServer(
            path.join(__dirname, '../media'),
            loginUrl.toString(true),
            callbackUri.toString(true),
            env.isAppPortable
        );
        const port = await server.start();
        
        try {
            env.openExternal(Uri.parse(`http://127.0.0.1:${port}/signin?nonce=${encodeURIComponent(server.nonce)}`));
            const { code } = await Promise.race([
                server.waitForOAuthResponse(),
                new Promise<any>((_, reject) => setTimeout(() => reject(TIMED_OUT_ERROR), 300_000)),
                promiseFromEvent<any, any>(token.onCancellationRequested, (_, __, reject) => { reject(USER_CANCELLATION_ERROR); }).promise
            ]);
            codeToExchange = code;
        } finally {
            setTimeout(() => { void server.stop(); }, 5000);
        }
        
        const accessToken = await exchangeCodeForToken(
            logger,
            baseUri.with({ path: '/login/oauth/access_token' }),
            redirectUri,
            codeToExchange,
            codeVerifier,
            enterpriseUri
        );
        return accessToken;
    }
}
```

### Flow 3.3: Device Code Flow (No UI Redirect Required)
**Location:** `flows.ts:387-454`

```typescript
class DeviceCodeFlow implements IFlow {
    label = l10n.t('device code');
    options: IFlowOptions = {
        supportsGitHubDotCom: true,
        supportsGitHubEnterpriseServer: true,
        supportsHostedGitHubEnterprise: true,
        supportsRemoteExtensionHost: true,
        supportsWebWorkerExtensionHost: false,
        supportsNoClientSecret: true,
        supportsSupportedClients: true,
        supportsUnsupportedClients: true
    };

    async trigger({ scopes, baseUri, signInProvider, extraAuthorizeParameters, logger }: IFlowTriggerOptions) {
        const uri = baseUri.with({
            path: '/login/device/code',
            query: `client_id=${Config.gitHubClientId}&scope=${scopes}`
        });
        const result = await fetching(uri.toString(true), {
            logger,
            retryFallbacks: true,
            expectJSON: true,
            method: 'POST',
            headers: { Accept: 'application/json' }
        });
        
        const json = await result.json() as IGitHubDeviceCodeResponse;
        const button = l10n.t('Copy & Continue to Browser');
        const modalResult = await window.showInformationMessage(
            l10n.t({ message: 'Your Code: {0}', args: [json.user_code], comment: ['The {0} will be a code, e.g. 123-456'] }),
            {
                modal: true,
                detail: l10n.t('To finish authenticating, navigate to GitHub and paste in the above one-time code.')
            }, button
        );
        
        if (modalResult !== button) {
            throw new Error(USER_CANCELLATION_ERROR);
        }
        
        await env.clipboard.writeText(json.user_code);
        await env.openExternal(uriToOpen);
        
        return await this.waitForDeviceCodeAccessToken(logger, baseUri, json);
    }

    private async waitForDeviceCodeAccessToken(
        logger: Log,
        baseUri: Uri,
        json: IGitHubDeviceCodeResponse,
    ): Promise<string> {
        const refreshTokenUri = baseUri.with({
            path: '/login/oauth/access_token',
            query: `client_id=${Config.gitHubClientId}&device_code=${json.device_code}&grant_type=urn:ietf:params:oauth:grant-type:device_code`
        });
        
        const attempts = 120 / json.interval;
        for (let i = 0; i < attempts; i++) {
            await new Promise(resolve => setTimeout(resolve, json.interval * 1000));
            if (token.isCancellationRequested) {
                throw new Error(USER_CANCELLATION_ERROR);
            }
            // Poll for token with exponential backoff support
        }
    }
}
```

**Porting Implications:**
- Three OAuth flows must be reimplemented with feature parity
- PKCE implementation requires SHA-256 hashing capability
- Protocol handler (vscode://) needs replacement with Tauri deep linking
- Local server requires native HTTP server implementation
- Device code polling requires background task support
- Flow selection logic must consider Tauri runtime capabilities

---

## Pattern 4: Protocol Handler Registration and URI Event Handling

**Where:** `extensions/github-authentication/src/extension.ts:64-66`

**What:** Registers a URI handler for protocol scheme and manages URI events from OAuth redirects.

```typescript
const uriHandler = new UriEventHandler();
context.subscriptions.push(uriHandler);
context.subscriptions.push(vscode.window.registerUriHandler(uriHandler));
```

**Where:** `extensions/github-authentication/src/github.ts:70-128`

```typescript
export class UriEventHandler extends vscode.EventEmitter<vscode.Uri> implements vscode.UriHandler {
    private readonly _pendingNonces = new Map<string, string[]>();
    private readonly _codeExchangePromises = new Map<string, { promise: Promise<string>; cancel: vscode.EventEmitter<void> }>();

    public handleUri(uri: vscode.Uri) {
        this.fire(uri);
    }

    public async waitForCode(logger: Log, scopes: string, nonce: string, token: vscode.CancellationToken) {
        const existingNonces = this._pendingNonces.get(scopes) || [];
        this._pendingNonces.set(scopes, [...existingNonces, nonce]);

        let codeExchangePromise = this._codeExchangePromises.get(scopes);
        if (!codeExchangePromise) {
            codeExchangePromise = promiseFromEvent(this.event, this.handleEvent(logger, scopes));
            this._codeExchangePromises.set(scopes, codeExchangePromise);
        }

        try {
            return await Promise.race([
                codeExchangePromise.promise,
                new Promise<string>((_, reject) => setTimeout(() => reject(TIMED_OUT_ERROR), 300_000)), // 5min timeout
                promiseFromEvent<void, string>(token.onCancellationRequested, (_, __, reject) => { reject(USER_CANCELLATION_ERROR); }).promise
            ]);
        } finally {
            this._pendingNonces.delete(scopes);
            codeExchangePromise?.cancel.fire();
            this._codeExchangePromises.delete(scopes);
        }
    }

    private handleEvent: (logger: Log, scopes: string) => PromiseAdapter<vscode.Uri, string> =
        (logger: Log, scopes) => (uri, resolve, reject) => {
            const query = new URLSearchParams(uri.query);
            const code = query.get('code');
            const nonce = query.get('nonce');
            if (!code) {
                reject(new Error('No code'));
                return;
            }
            if (!nonce) {
                reject(new Error('No nonce'));
                return;
            }

            const acceptedNonces = this._pendingNonces.get(scopes) || [];
            if (!acceptedNonces.includes(nonce)) {
                logger.info('Nonce not found in accepted nonces. Skipping this execution...');
                return;
            }

            resolve(code);
        };
}
```

**Key Aspects:**
- Nonce-based replay attack prevention
- Manages concurrent OAuth flows with scope-based deduplication
- 5-minute timeout for user action
- Cancellation token support for user-initiated cancellation
- Multiple concurrent scopes supported per handler instance

**Porting Implications:**
- Tauri deep linking must route to equivalent event handler
- Nonce validation logic must be preserved exactly
- Timeout and cancellation mechanisms need Rust async equivalents
- Protocol URI format: `vscode://vscode.github-authentication/did-authenticate?nonce=...&code=...`

---

## Pattern 5: Local Loopback Server for OAuth Redirect

**Where:** `extensions/github-authentication/src/node/authServer.ts:71-210`

**What:** HTTP server listening on 127.0.0.1 to receive OAuth redirects, used when protocol handlers are unavailable.

```typescript
export class LoopbackAuthServer implements ILoopbackServer {
    private readonly _server: http.Server;
    private readonly _resultPromise: Promise<IOAuthResult>;
    private _startingRedirect: URL;

    public nonce = randomBytes(16).toString('base64');
    public port: number | undefined;

    constructor(serveRoot: string, startingRedirect: string, callbackUri: string, isPortable: boolean) {
        this._startingRedirect = new URL(startingRedirect);
        let deferred: { resolve: (result: IOAuthResult) => void; reject: (reason: any) => void };
        this._resultPromise = new Promise<IOAuthResult>((resolve, reject) => deferred = { resolve, reject });

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
                    if (this.state !== state) {
                        res.writeHead(302, { location: `/?error=${encodeURIComponent('State does not match.')}...` });
                        res.end();
                        throw new Error('State does not match.');
                    }
                    if (this.nonce !== nonce) {
                        res.writeHead(302, { location: `/?error=${encodeURIComponent('Nonce does not match.')}...` });
                        res.end();
                        throw new Error('Nonce does not match.');
                    }
                    deferred.resolve({ code, state });
                    if (isPortable) {
                        res.writeHead(302, { location: `/?app_name=${encodeURIComponent(env.appName)}...` });
                    } else {
                        res.writeHead(302, { location: `/?redirect_uri=${encodeURIComponent(callbackUri)}...` });
                    }
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
    }

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

    public stop(): Promise<void> {
        return new Promise<void>((resolve, reject) => {
            if (!this._server.listening) {
                throw new Error('Server is not started');
            }
            this._server.close((err) => {
                if (err) {
                    reject(err);
                } else {
                    resolve();
                }
            });
        });
    }

    public waitForOAuthResponse(): Promise<IOAuthResult> {
        return this._resultPromise;
    }
}
```

**Key Aspects:**
- Dynamic port selection (OS-assigned port)
- Base64 nonce generation for replay protection
- Serves static HTML/SVG files from media directory
- State parameter validation against OAuth spec
- Nonce mismatch handling with user-facing error pages
- 5-second startup timeout
- Graceful shutdown with cleanup

**Porting Implications:**
- Tauri needs native HTTP server (actix-web, axum, or warp crate)
- Static file serving from embedded resources
- Same nonce/state validation logic required
- Async Promise-based interface requires Tokio runtime
- Error page HTML must be bundled with Tauri app

---

## Pattern 6: OAuth Token Exchange with PKCE

**Where:** `extensions/github-authentication/src/flows.ts:148-195`

**What:** Authorization code exchange for access token using PKCE, client secret, and basic auth.

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

**Key Aspects:**
- POST to `/login/oauth/access_token` endpoint
- PKCE code verifier in body (required for public clients)
- Optional `github_enterprise` parameter for GHES instances
- Error handling with response text extraction
- Retry fallback mechanism via fetching abstraction

**Porting Implications:**
- HTTP client library (reqwest crate)
- PKCE implementation with SHA-256 hashing
- Config management for client ID/secret
- Enterprise Server URI validation and parameter handling

---

## Pattern 7: Session Management and Lifecycle

**Where:** `extensions/github-authentication/src/github.ts:346-453`

**What:** Creating, storing, and removing sessions with event emission for session changes.

```typescript
public async createSession(scopes: string[], options?: GitHubAuthenticationProviderOptions): Promise<vscode.AuthenticationSession> {
    try {
        const sortedScopes = [...scopes].sort();

        this._telemetryReporter?.sendTelemetryEvent('login', {
            scopes: JSON.stringify(scopes),
        });

        if (options && !isGitHubAuthenticationProviderOptions(options)) {
            throw new Error('Invalid options');
        }
        const sessions = await this._sessionsPromise;
        const loginWith = options?.account?.label;
        const signInProvider = options?.provider;
        this._logger.info(`Logging in with${signInProvider ? ` ${signInProvider}, ` : ''} '${loginWith ? loginWith : 'any'}' account...`);
        const scopeString = sortedScopes.join(' ');
        const token = await this._githubServer.login(scopeString, signInProvider, options?.extraAuthorizeParameters, loginWith);
        const session = await this.tokenToSession(token, scopes);
        this.afterSessionLoad(session);

        const sessionIndex = sessions.findIndex(s => s.account.id === session.account.id && arrayEquals([...s.scopes].sort(), sortedScopes));
        const removed = new Array<vscode.AuthenticationSession>();
        if (sessionIndex > -1) {
            removed.push(...sessions.splice(sessionIndex, 1, session));
        } else {
            sessions.push(session);
        }
        await this.storeSessions(sessions);

        this._sessionChangeEmitter.fire({ added: [session], removed, changed: [] });

        this._logger.info('Login success!');

        return session;
    } catch (e) {
        if (e === 'Cancelled' || e.message === 'Cancelled') {
            this._telemetryReporter?.sendTelemetryEvent('loginCancelled');
            throw e;
        }

        this._telemetryReporter?.sendTelemetryEvent('loginFailed');

        vscode.window.showErrorMessage(vscode.l10n.t('Sign in failed: {0}', `${e}`));
        this._logger.error(e);
        throw e;
    }
}

public async removeSession(id: string) {
    try {
        this._telemetryReporter?.sendTelemetryEvent('logout');
        this._logger.info(`Logging out of ${id}`);

        const sessions = await this._sessionsPromise;
        const sessionIndex = sessions.findIndex(session => session.id === id);
        if (sessionIndex > -1) {
            const session = sessions[sessionIndex];
            sessions.splice(sessionIndex, 1);

            await this.storeSessions(sessions);
            await this._githubServer.logout(session);

            this._sessionChangeEmitter.fire({ added: [], removed: [session], changed: [] });
        } else {
            this._logger.error('Session not found');
        }
    } catch (e) {
        this._telemetryReporter?.sendTelemetryEvent('logoutFailed');
        vscode.window.showErrorMessage(vscode.l10n.t('Sign out failed: {0}', `${e}`));
        this._logger.error(e);
        throw e;
    }
}
```

**Key Aspects:**
- Session deduplication by account ID and sorted scopes
- Scope ordering normalization (order-independent matching)
- Telemetry tracking for login/logout flows
- Event emission for external subscription
- Local token deletion plus optional server-side revocation
- Token generation with random IDs

**Porting Implications:**
- Session storage must maintain JSON compatibility
- Scope normalization required for API contracts
- Event emitter pattern for session changes
- Telemetry integration points
- Server-side token revocation API calls

---

## Pattern 8: Multi-Window Secret Synchronization

**Where:** `extensions/github-authentication/src/github.ts:224-253`

**What:** Detects secret storage changes from other windows and syncs session state.

```typescript
private async checkForUpdates() {
    const previousSessions = await this._sessionsPromise;
    this._sessionsPromise = this.readSessions();
    const storedSessions = await this._sessionsPromise;

    const added: vscode.AuthenticationSession[] = [];
    const removed: vscode.AuthenticationSession[] = [];

    storedSessions.forEach(session => {
        const matchesExisting = previousSessions.some(s => s.id === session.id);
        if (!matchesExisting) {
            this._logger.info('Adding session found in keychain');
            added.push(session);
        }
    });

    previousSessions.forEach(session => {
        const matchesExisting = storedSessions.some(s => s.id === session.id);
        if (!matchesExisting) {
            this._logger.info('Removing session no longer found in keychain');
            removed.push(session);
        }
    });

    if (added.length || removed.length) {
        this._sessionChangeEmitter.fire({ added, removed, changed: [] });
    }
}
```

Triggered by:
```typescript
this.context.secrets.onDidChange(() => this.checkForUpdates())
```

**Key Aspects:**
- Listens to ExtensionContext secrets change event
- Diff-based update detection (added vs removed)
- Event emission after verification
- Handles cross-window session sharing

**Porting Implications:**
- Tauri/Rust needs system secret storage change notifications
- Event listener pattern for credential manager changes
- Session diff algorithm must match exactly

---

## Pattern 9: User Credential Verification and Account Info Retrieval

**Where:** `extensions/github-authentication/src/githubServer.ts:220-261`

**What:** Validates access tokens by fetching user information from GitHub API.

```typescript
public async getUserInfo(token: string): Promise<{ id: string; accountName: string }> {
    let result;
    try {
        this._logger.info('Getting user info...');
        result = await fetching(this.getServerUri('/user').toString(), {
            logger: this._logger,
            retryFallbacks: true,
            expectJSON: true,
            headers: {
                Authorization: `token ${token}`,
                'User-Agent': `${vscode.env.appName} (${vscode.env.appHost})`
            }
        });
    } catch (ex) {
        this._logger.error(ex.message);
        throw new Error(NETWORK_ERROR);
    }

    if (result.ok) {
        try {
            const json = await result.json() as { id: number; login: string };
            this._logger.info('Got account info!');
            return { id: `${json.id}`, accountName: json.login };
        } catch (e) {
            this._logger.error(`Unexpected error parsing response from GitHub: ${e.message ?? e}`);
            throw e;
        }
    } else {
        let errorMessage = result.statusText;
        try {
            const json = await result.json();
            if (json.message) {
                errorMessage = json.message;
            }
        } catch (err) {
            // noop
        }
        this._logger.error(`Getting account info failed: ${errorMessage}`);
        throw new Error(errorMessage);
    }
}
```

**Key Aspects:**
- Token-based Bearer authentication to GitHub API
- Dynamic server URI construction (github.com vs GHES)
- Response parsing with fallback error extraction
- User-Agent header inclusion
- Network error abstraction

**Porting Implications:**
- GitHub REST API v3 integration
- Dynamic endpoint construction for enterprise servers
- HTTP header management
- Error message extraction and handling

---

## Pattern 10: Server-Side Token Revocation

**Where:** `extensions/github-authentication/src/githubServer.ts:155-208`

**What:** Deletes OAuth tokens from GitHub servers using client credentials and Basic auth.

```typescript
public async logout(session: vscode.AuthenticationSession): Promise<void> {
    this._logger.trace(`Deleting session (${session.id}) from server...`);

    if (!Config.gitHubClientSecret) {
        this._logger.warn('No client secret configured for GitHub authentication. The token has been deleted with best effort on this system, but we are unable to delete the token on server without the client secret.');
        return;
    }

    // Only attempt to delete OAuth tokens. They are always prefixed with `gho_`.
    if (!session.accessToken.startsWith('gho_')) {
        this._logger.warn('The token being deleted is not an OAuth token. It has been deleted locally, but we cannot delete it on server.');
        return;
    }

    if (!isSupportedTarget(this._type, this._ghesUri)) {
        this._logger.trace('GitHub.com and GitHub hosted GitHub Enterprise are the only options that support deleting tokens on the server. Skipping.');
        return;
    }

    const authHeader = 'Basic ' + base64Encode(`${Config.gitHubClientId}:${Config.gitHubClientSecret}`);
    const uri = this.getServerUri(`/applications/${Config.gitHubClientId}/token`);

    try {
        const result = await fetching(uri.toString(true), {
            logger: this._logger,
            retryFallbacks: true,
            expectJSON: false,
            method: 'DELETE',
            headers: {
                Accept: 'application/vnd.github+json',
                Authorization: authHeader,
                'X-GitHub-Api-Version': '2022-11-28',
                'User-Agent': `${vscode.env.appName} (${vscode.env.appHost})`
            },
            body: JSON.stringify({ access_token: session.accessToken }),
        });

        if (result.status === 204) {
            this._logger.trace(`Successfully deleted token from session (${session.id}) from server.`);
            return;
        }

        try {
            const body = await result.text();
            throw new Error(body);
        } catch (e) {
            throw new Error(`${result.status} ${result.statusText}`);
        }
    } catch (e) {
        this._logger.warn('Failed to delete token from server.' + (e.message ?? e));
    }
}
```

**Key Aspects:**
- Basic authentication with client credentials
- Token type detection (OAuth tokens have `gho_` prefix)
- Server-side deletion not supported for on-prem GHES
- 204 No Content success status
- GitHub API versioning header
- Graceful degradation if client secret unavailable

**Porting Implications:**
- Base64 encoding for Basic auth
- Token classification logic
- Target server capability detection
- GitHub API version header handling
- Error recovery and logging

---

## Pattern 11: Configuration Management and Extension Initialization

**Where:** `extensions/github-authentication/src/extension.ts:39-82`

**What:** Dynamic GitHub Enterprise provider initialization based on workspace configuration with hot reload support.

```typescript
function initGHES(context: vscode.ExtensionContext, uriHandler: UriEventHandler): vscode.Disposable {
    const settingValue = vscode.workspace.getConfiguration().get<string>('github-enterprise.uri');
    if (!settingValue) {
        const provider = new NullAuthProvider(settingNotSent);
        context.subscriptions.push(provider);
        return provider;
    }

    let uri: vscode.Uri;
    try {
        uri = vscode.Uri.parse(settingValue, true);
    } catch (e) {
        vscode.window.showErrorMessage(vscode.l10n.t('GitHub Enterprise Server URI is not a valid URI: {0}', e.message ?? e));
        const provider = new NullAuthProvider(settingInvalid);
        context.subscriptions.push(provider);
        return provider;
    }

    const githubEnterpriseAuthProvider = new GitHubAuthenticationProvider(context, uriHandler, uri);
    context.subscriptions.push(githubEnterpriseAuthProvider);
    return githubEnterpriseAuthProvider;
}

export function activate(context: vscode.ExtensionContext) {
    const uriHandler = new UriEventHandler();
    context.subscriptions.push(uriHandler);
    context.subscriptions.push(vscode.window.registerUriHandler(uriHandler));

    context.subscriptions.push(new GitHubAuthenticationProvider(context, uriHandler));

    let before = vscode.workspace.getConfiguration().get<string>('github-enterprise.uri');
    let githubEnterpriseAuthProvider = initGHES(context, uriHandler);
    context.subscriptions.push(vscode.workspace.onDidChangeConfiguration(e => {
        if (e.affectsConfiguration('github-enterprise.uri')) {
            const after = vscode.workspace.getConfiguration().get<string>('github-enterprise.uri');
            if (before !== after) {
                githubEnterpriseAuthProvider?.dispose();
                before = after;
                githubEnterpriseAuthProvider = initGHES(context, uriHandler);
            }
        }
    }));

    const beforeFetchSetting = vscode.workspace.getConfiguration().get<boolean>('github-authentication.useElectronFetch', true);
    context.subscriptions.push(vscode.workspace.onDidChangeConfiguration(async e => {
        if (e.affectsConfiguration('github-authentication.useElectronFetch')) {
            const afterFetchSetting = vscode.workspace.getConfiguration().get<boolean>('github-authentication.useElectronFetch', true);
            if (beforeFetchSetting !== afterFetchSetting) {
                const selection = await vscode.window.showInformationMessage(
                    vscode.l10n.t('GitHub Authentication - Reload required'),
                    {
                        modal: true,
                        detail: vscode.l10n.t('A reload is required for the fetch setting change to take effect.')
                    },
                    vscode.l10n.t('Reload Window')
                );
                if (selection) {
                    await vscode.commands.executeCommand('workbench.action.reloadWindow');
                }
            }
        }
    }));
}
```

**Key Aspects:**
- Lazy initialization based on configuration
- URI validation with error recovery (NullAuthProvider fallback)
- Configuration change listeners with provider replacement
- Hot reload support with window reload prompts
- Subscription management for resource cleanup

**Porting Implications:**
- Configuration file system (Tauri config or local storage)
- Provider factory pattern
- Lifecycle management with cleanup
- Settings change notification system
- Error recovery with graceful degradation

---

## Pattern 12: Flow Selection Logic

**Where:** `extensions/github-authentication/src/flows.ts` (lines showing getFlows call) and `extensions/github-authentication/src/test/flows.test.ts:29-100`

**What:** Dynamic OAuth flow selection based on environment, runtime, and client capabilities.

```typescript
const flows = getFlows({
    target: this._type === AuthProviderType.github
        ? GitHubTarget.DotCom
        : supportedTarget ? GitHubTarget.HostedEnterprise : GitHubTarget.Enterprise,
    extensionHost: isNodeEnvironment
        ? this._extensionKind === vscode.ExtensionKind.UI ? ExtensionHost.Local : ExtensionHost.Remote
        : ExtensionHost.WebWorker,
    isSupportedClient: supportedClient
});

for (const flow of flows) {
    try {
        if (flow !== flows[0]) {
            await promptToContinue(flow.label);
        }
        return await flow.trigger({...});
    } catch (e) {
        userCancelled = this.processLoginError(e);
    }
}
```

**Test Cases** (`flows.test.ts:29-100`):
- VS Code Desktop + local filesystem + GitHub.com: [LocalServer, UrlHandler, DeviceCode]
- VS Code Desktop + local filesystem + Hosted Enterprise: [LocalServer, UrlHandler, DeviceCode, PAT]
- VS Code Desktop + local filesystem + Enterprise Server: [DeviceCode, PAT]
- vscode.dev + remote + GitHub.com: [UrlHandler, DeviceCode]
- vscode.dev + remote + Hosted Enterprise: [UrlHandler, DeviceCode, PAT]
- vscode.dev + remote + Enterprise Server: [DeviceCode, PAT]
- Web worker (unsupported client): [DeviceCode, PAT]

**Key Aspects:**
- Flow ordering by success probability
- Target detection (DotCom vs HostedEnterprise vs Enterprise)
- Runtime detection (Node vs WebWorker vs Remote)
- Client capability detection (supported vs unsupported)
- Fallback mechanism with user prompts

**Porting Implications:**
- Flow registry with capabilities matching
- Runtime environment detection in Tauri
- Client capability matrix
- User prompt system for fallback flows
- Comprehensive test coverage for all combinations

---

## Summary: Critical Porting Challenges for Tauri/Rust

1. **Authentication Provider Interface**: Tauri needs equivalent API for registering auth providers with session callbacks
2. **Secret Storage**: Native keyring integration (keyring, tauri-plugin-keyring) replacing vscode.ExtensionContext.secrets
3. **Protocol Handler**: Deep linking replacement for vscode:// protocol scheme
4. **HTTP Server**: Native HTTP server (actix-web, axum) for loopback OAuth redirect
5. **OAuth Flows**: Three separate implementations (URL handler, Local server, Device code) with PKCE support
6. **URI Event Handling**: Nonce/state validation and timeout management in Rust async context
7. **Configuration System**: Workspace/user settings replacement in Tauri
8. **Cross-Window Synchronization**: System credential change notifications for multi-window secret sync
9. **HTTP Client**: HTTP library (reqwest) with error handling and retry logic
10. **Crypto Operations**: SHA-256 hashing, random value generation, Base64url encoding

---

## Related Files in Extension

- `src/extension.ts` - Provider activation and configuration
- `src/github.ts` - GitHubAuthenticationProvider implementation and session management
- `src/githubServer.ts` - OAuth server communication and API calls
- `src/flows.ts` - OAuth flow implementations (3 variants)
- `src/common/keychain.ts` - Secret storage abstraction
- `src/node/authServer.ts` - Loopback HTTP server
- `src/config.ts` - Client ID/secret configuration
- `package.json` - Extension manifest with authentication capabilities
- `src/test/flows.test.ts` - Flow selection test coverage

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
