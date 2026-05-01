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
