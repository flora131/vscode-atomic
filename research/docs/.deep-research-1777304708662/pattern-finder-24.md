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
