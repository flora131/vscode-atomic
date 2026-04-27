# Pattern Research: VS Code Authentication Extension (microsoft-authentication)

## Overview
This partition analyzes the microsoft-authentication extension to extract patterns relevant to porting VS Code's IDE functionality from TypeScript/Electron to Tauri/Rust. The extension demonstrates authentication flow management, secrets handling, and cross-platform browser integration patterns that would need to be ported.

---

## Pattern: Authentication Provider Registration

**Where:** `extensions/microsoft-authentication/src/extension.ts:62-67`

**What:** Registers an authentication provider with VS Code's authentication subsystem, exposing capabilities like multi-account support and challenge-response authentication.

```typescript
const disposable = authentication.registerAuthenticationProvider(
    'microsoft-sovereign-cloud',
    authProviderName,
    authProvider,
    { supportsMultipleAccounts: true, supportsChallenges: true }
);
context.subscriptions.push(disposable);
```

**Variations / call-sites:**
- `extensions/microsoft-authentication/src/extension.ts:118-130` - Main Microsoft provider registration with authorization server specifications

---

## Pattern: Secrets Storage Abstraction with Change Events

**Where:** `extensions/microsoft-authentication/src/betterSecretStorage.ts:33-76`

**What:** Wraps VS Code's SecretStorage API to provide key-list tracking and multi-window synchronization, listening for secret changes and propagating events when secrets are modified in other windows.

```typescript
constructor(private keylistKey: string, context: ExtensionContext) {
    this._secretStorage = context.secrets;
    context.subscriptions.push(context.secrets.onDidChange((e) => this.handleSecretChange(e)));
    this.initialize();
}

private initialize(): void {
    this._operationInProgress = true;
    this._tokensPromise = new Promise((resolve, _) => {
        this._secretStorage.get(this.keylistKey).then(
            keyListStr => {
                if (!keyListStr) {
                    resolve(new Map());
                    return;
                }
                const keyList: Array<string> = JSON.parse(keyListStr);
                const promises = keyList.map(key => new Promise<{ key: string; value: string | undefined }>((res, rej) => {
                    this._secretStorage.get(key).then((value) => {
                        res({ key, value });
                    }, rej);
                }));
                Promise.allSettled(promises).then((results => {
                    const tokens = new Map<string, T>();
                    results.forEach(p => {
                        if (p.status === 'fulfilled' && p.value.value) {
                            const secret = this.parseSecret(p.value.value);
                            tokens.set(p.value.key, secret);
                        }
                    });
                    resolve(tokens);
                }));
            }
        );
    });
    this._operationInProgress = false;
}
```

**Variations / call-sites:**
- `extensions/microsoft-authentication/src/betterSecretStorage.ts:94-120` - Token storage with key list updates
- `extensions/microsoft-authentication/src/betterSecretStorage.ts:194-247` - Secret change detection across windows

---

## Pattern: Cache Plugin for Token Storage Persistence

**Where:** `extensions/microsoft-authentication/src/common/cachePlugin.ts:9-55`

**What:** Implements MSAL's cache plugin interface to serialize/deserialize token cache before and after MSAL operations, persisting to SecretStorage only when cache changes.

```typescript
export class SecretStorageCachePlugin implements ICachePlugin, Disposable {
    private readonly _onDidChange: EventEmitter<void> = new EventEmitter<void>();
    readonly onDidChange = this._onDidChange.event;

    private _disposable: Disposable;
    private _value: string | undefined;

    constructor(
        private readonly _secretStorage: SecretStorage,
        private readonly _key: string
    ) {
        this._disposable = Disposable.from(
            this._onDidChange,
            this._registerChangeHandler()
        );
    }

    async beforeCacheAccess(tokenCacheContext: TokenCacheContext): Promise<void> {
        const data = await this._secretStorage.get(this._key);
        this._value = data;
        if (data) {
            tokenCacheContext.tokenCache.deserialize(data);
        }
    }

    async afterCacheAccess(tokenCacheContext: TokenCacheContext): Promise<void> {
        if (tokenCacheContext.cacheHasChanged) {
            const value = tokenCacheContext.tokenCache.serialize();
            if (value !== this._value) {
                await this._secretStorage.store(this._key, value);
            }
        }
    }
}
```

**Variations / call-sites:**
- `extensions/microsoft-authentication/src/node/cachedPublicClientApplication.ts:46-49` - Instantiation with clientId-scoped key

---

## Pattern: URI Event Handler for OAuth Callbacks

**Where:** `extensions/microsoft-authentication/src/UriEventHandler.ts:8-19`

**What:** Extends EventEmitter to implement VS Code's UriHandler interface, capturing OAuth redirect URIs and emitting them as events for listening flows to parse authorization codes.

```typescript
export class UriEventHandler extends vscode.EventEmitter<vscode.Uri> implements vscode.UriHandler {
    private _disposable = vscode.window.registerUriHandler(this);

    handleUri(uri: vscode.Uri) {
        this.fire(uri);
    }

    override dispose(): void {
        super.dispose();
        this._disposable.dispose();
    }
}
```

**Variations / call-sites:**
- `extensions/microsoft-authentication/src/common/loopbackClientAndOpener.ts:23-34` - Consumes URI events and extracts OAuth parameters (code, state, error)

---

## Pattern: Loopback Client Adapter for MSAL Integration

**Where:** `extensions/microsoft-authentication/src/common/loopbackClientAndOpener.ts:15-50`

**What:** Adapts VS Code's URI handler to MSAL's ILoopbackClient interface, translating URI events into authorization code responses and managing browser opening.

```typescript
export class UriHandlerLoopbackClient implements ILoopbackClientAndOpener {
    constructor(
        private readonly _uriHandler: UriEventHandler,
        private readonly _redirectUri: string,
        private readonly _callbackUri: Uri,
        private readonly _logger: LogOutputChannel
    ) { }

    async listenForAuthCode(): Promise<ServerAuthorizationCodeResponse> {
        const url = await toPromise(this._uriHandler.event);
        this._logger.debug(`Received URL event. Authority: ${url.authority}`);
        const result = new URL(url.toString(true));
        return {
            code: result.searchParams.get('code') ?? undefined,
            state: result.searchParams.get('state') ?? undefined,
            error: result.searchParams.get('error') ?? undefined,
            error_description: result.searchParams.get('error_description') ?? undefined,
            error_uri: result.searchParams.get('error_uri') ?? undefined,
        };
    }

    async openBrowser(url: string): Promise<void> {
        const uri = Uri.parse(url + `&state=${encodeURI(this._callbackUri.toString(true))}`);
        await env.openExternal(uri);
    }
}
```

**Variations / call-sites:**
- `extensions/microsoft-authentication/src/node/flows.ts:85-103` - Instantiated by UrlHandlerFlow

---

## Pattern: Pluggable Authentication Flows with Capability Matching

**Where:** `extensions/microsoft-authentication/src/node/flows.ts:46-128`

**What:** Defines multiple authentication flows (DefaultLoopback, UrlHandler, DeviceCode) with capability flags, selecting flows at runtime based on extension host, broker availability, and platform.

```typescript
interface IMsalFlowOptions {
    supportsRemoteExtensionHost: boolean;
    supportsUnsupportedClient: boolean;
    supportsBroker: boolean;
    supportsPortableMode: boolean;
}

class DefaultLoopbackFlow implements IMsalFlow {
    label = 'default';
    options: IMsalFlowOptions = {
        supportsRemoteExtensionHost: false,
        supportsUnsupportedClient: true,
        supportsBroker: true,
        supportsPortableMode: true
    };

    async trigger({ cachedPca, authority, scopes, claims, loginHint, windowHandle, logger }: IMsalFlowTriggerOptions): Promise<AuthenticationResult> {
        logger.info('Trying default msal flow...');
        let redirectUri: string | undefined;
        if (cachedPca.isBrokerAvailable && process.platform === 'darwin') {
            redirectUri = Config.macOSBrokerRedirectUri;
        }
        return await cachedPca.acquireTokenInteractive({
            openBrowser: async (url: string) => { await env.openExternal(Uri.parse(url)); },
            scopes,
            authority,
            successTemplate: loopbackTemplate,
            errorTemplate: loopbackTemplate,
            loginHint,
            prompt: loginHint ? undefined : 'select_account',
            windowHandle,
            claims,
            redirectUri
        });
    }
}

export function getMsalFlows(query: IMsalFlowQuery): IMsalFlow[] {
    const flows = [];
    for (const flow of allFlows) {
        // Filtering logic based on query capabilities
    }
    return flows;
}
```

**Variations / call-sites:**
- `extensions/microsoft-authentication/src/node/flows.ts:76-103` - UrlHandlerFlow for remote extension hosts
- `extensions/microsoft-authentication/src/node/flows.ts:106-123` - DeviceCodeFlow for unsupported clients
- `extensions/microsoft-authentication/src/node/authProvider.ts:228-233` - Flow selection during session creation

---

## Pattern: Cached Public Client Application Manager

**Where:** `extensions/microsoft-authentication/src/node/publicClientCache.ts:47-126`

**What:** Manages multiple PublicClientApplication instances keyed by clientId, handling lazy initialization, secret storage persistence, token migration from legacy flows, and account change propagation.

```typescript
export class CachedPublicClientApplicationManager implements ICachedPublicClientApplicationManager {
    private readonly _pcas = new Map<string, ICachedPublicClientApplication>();
    private readonly _pcaDisposables = new Map<string, Disposable>();

    static async create(
        secretStorage: SecretStorage,
        logger: LogOutputChannel,
        telemetryReporter: MicrosoftAuthenticationTelemetryReporter,
        env: Environment
    ): Promise<CachedPublicClientApplicationManager> {
        const pcasSecretStorage = await PublicClientApplicationsSecretStorage.create(secretStorage, env.name);
        const migrations = await pcasSecretStorage.getOldValue();
        const accountAccess = await ScopedAccountAccess.create(secretStorage, env.name, logger, migrations);
        const manager = new CachedPublicClientApplicationManager(env, pcasSecretStorage, accountAccess, secretStorage, logger, telemetryReporter, [pcasSecretStorage, accountAccess]);
        await manager.initialize();
        return manager;
    }

    async getOrCreate(clientId: string, migrate?: { refreshTokensToMigrate?: string[]; tenant: string }): Promise<ICachedPublicClientApplication> {
        let pca = this._pcas.get(clientId);
        if (pca) {
            this._logger.debug(`[getOrCreate] [${clientId}] PublicClientApplicationManager cache hit`);
        } else {
            this._logger.debug(`[getOrCreate] [${clientId}] PublicClientApplicationManager cache miss, creating new PCA...`);
            pca = await this._doCreatePublicClientApplication(clientId);
            await this._storePublicClientApplications();
        }
        if (migrate?.refreshTokensToMigrate?.length) {
            const authority = new URL(migrate.tenant, this._env.activeDirectoryEndpointUrl).toString();
            for (const refreshToken of migrate.refreshTokensToMigrate) {
                const result = await pca.acquireTokenByRefreshToken({
                    refreshToken,
                    forceCache: true,
                    scopes: [],
                    authority,
                    redirectUri
                });
            }
        }
        return pca;
    }
}
```

**Variations / call-sites:**
- `extensions/microsoft-authentication/src/node/authProvider.ts:106` - Instantiated during auth provider creation
- `extensions/microsoft-authentication/src/node/authProvider.ts:195-207` - Used for token acquisition flows

---

## Pattern: Multi-Flow Session Creation with Error Recovery

**Where:** `extensions/microsoft-authentication/src/node/authProvider.ts:202-276`

**What:** Implements createSession with fallback flows, prompting user for alternative auth methods when flows fail, catching specific MSAL errors (UserCanceled, ServerError) versus recoverable errors.

```typescript
async createSession(scopes: readonly string[], options: AuthenticationProviderSessionOptions): Promise<AuthenticationSession> {
    const scopeData = new ScopeData(scopes, undefined, options.authorizationServer, options.clientId);
    this._logger.info('[createSession]', `[${scopeData.scopeStr}]`, 'starting');
    const cachedPca = await this._publicClientManager.getOrCreate(scopeData.clientId);

    let userCancelled: boolean | undefined;
    const yes = l10n.t('Yes');
    const no = l10n.t('No');
    const promptToContinue = async (mode: string) => {
        if (userCancelled === undefined) {
            return;
        }
        const message = userCancelled
            ? l10n.t('Having trouble logging in? Would you like to try a different way? ({0})', mode)
            : l10n.t('You have not yet finished authorizing...');
        const result = await window.showWarningMessage(message, yes, no);
        if (result !== yes) {
            throw new CancellationError();
        }
    };

    const callbackUri = await env.asExternalUri(Uri.parse(`${env.uriScheme}://vscode.microsoft-authentication`));
    const flows = getMsalFlows({
        extensionHost: this._context.extension.extensionKind === ExtensionKind.UI ? ExtensionHost.Local : ExtensionHost.Remote,
        supportedClient: isSupportedClient(callbackUri),
        isBrokerSupported: cachedPca.isBrokerAvailable,
        isPortableMode: env.isAppPortable
    });

    const authority = new URL(scopeData.tenant, this._env.activeDirectoryEndpointUrl).toString();
    let lastError: Error | undefined;
    for (const flow of flows) {
        if (flow !== flows[0]) {
            try {
                await promptToContinue(flow.label);
            } finally {
                this._telemetryReporter.sendLoginFailedEvent();
            }
        }
        try {
            const result = await flow.trigger({
                cachedPca,
                authority,
                scopes: scopeData.scopesToSend,
                loginHint: options.account?.label,
                windowHandle: window.nativeHandle ? Buffer.from(window.nativeHandle) : undefined,
                logger: this._logger,
                uriHandler: this._uriHandler,
                callbackUri
            });

            const session = this.sessionFromAuthenticationResult(result, scopeData.originalScopes);
            this._telemetryReporter.sendLoginEvent(session.scopes);
            return session;
        } catch (e) {
            lastError = e;
            if (e instanceof ServerError || (e as ClientAuthError)?.errorCode === ClientAuthErrorCodes.userCanceled) {
                this._telemetryReporter.sendLoginFailedEvent();
                throw e;
            }
            if (e instanceof CancellationError) {
                userCancelled = true;
            }
        }
    }

    this._telemetryReporter.sendLoginFailedEvent();
    throw lastError ?? new Error('No auth flow succeeded');
}
```

**Variations / call-sites:**
- `extensions/microsoft-authentication/src/node/authProvider.ts:328-416` - createSessionFromChallenges with claims extraction

---

## Pattern: Operation Sequencing for Concurrent Secret Operations

**Where:** `extensions/microsoft-authentication/src/betterSecretStorage.ts:79-183`

**What:** Uses a Sequencer utility to queue storage operations and maintain operation state flags, ensuring multi-window synchronization and preventing race conditions in secret updates.

```typescript
async get(key: string): Promise<T | undefined> {
    const tokens = await this.getTokens();
    return tokens.get(key);
}

async store(key: string, value: T): Promise<void> {
    const tokens = await this.getTokens();
    const isAddition = !tokens.has(key);
    tokens.set(key, value);
    const valueStr = this.serializeSecret(value);
    this._operationInProgress = true;
    this._tokensPromise = new Promise((resolve, _) => {
        const promises = [this._secretStorage.store(key, valueStr)];
        if (isAddition) {
            promises.push(this.updateKeyList(tokens));
        }
        Promise.allSettled(promises).then(results => {
            results.forEach(r => {
                if (r.status === 'rejected') {
                    Logger.error(r.reason);
                }
            });
            resolve(tokens);
        });
    });
    this._operationInProgress = false;
}

private async getTokens(): Promise<Map<string, T>> {
    let tokens;
    do {
        tokens = await this._tokensPromise;
    } while (this._operationInProgress);
    return tokens;
}
```

**Variations / call-sites:**
- `extensions/microsoft-authentication/src/betterSecretStorage.ts:122-144` - Deletion with key list updates

---

## Summary

The microsoft-authentication extension demonstrates six critical patterns for porting VS Code to Tauri/Rust:

1. **Authentication Provider Registration**: The plugin architecture for registering auth providers with capabilities negotiation
2. **Secrets Storage Abstraction**: Cross-window secret synchronization and event-driven updates
3. **Cache Plugin Architecture**: MSAL integration via pluggable cache serialization
4. **URI Handler Pattern**: OAuth callback handling through VS Code's URI scheme system
5. **Loopback Client Adapter**: Bridging framework-specific URI events to standard MSAL interfaces
6. **Pluggable Authentication Flows**: Runtime flow selection based on platform, host, and client capabilities
7. **Public Client Application Manager**: Multi-tenant token management with lazy loading and migration
8. **Multi-Flow Session Creation**: Fallback authentication with user-facing error recovery
9. **Operation Sequencing**: Race-condition prevention in concurrent secret storage operations

These patterns reveal the need for:
- A Rust-based secrets storage backend (replacing Electron's safeStorage)
- URI scheme handling comparable to VS Code's event system
- Pluggable authentication flow selection logic
- Token cache serialization/deserialization mechanisms
- Cross-window synchronization primitives
- Error recovery and user prompting infrastructure

