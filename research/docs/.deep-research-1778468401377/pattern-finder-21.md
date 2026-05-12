# Authentication Provider Contract Patterns
## Microsoft Authentication Extension (extensions/microsoft-authentication/)

This document catalogs the core patterns for implementing and registering authentication providers in VS Code, extracted from the microsoft-authentication extension.

---

#### Pattern: Authentication Provider Registration
**Where:** `extensions/microsoft-authentication/src/extension.ts:118-130`
**What:** Registers an AuthenticationProvider implementation with VS Code using the authentication API with multi-account and challenge-response support.
```typescript
context.subscriptions.push(authentication.registerAuthenticationProvider(
	'microsoft',
	'Microsoft',
	authProvider,
	{
		supportsMultipleAccounts: true,
		supportsChallenges: true,
		supportedAuthorizationServers: [
			Uri.parse('https://login.microsoftonline.com/*'),
			Uri.parse('https://login.microsoftonline.com/*/v2.0')
		]
	}
));
```
**Variations / call-sites:** 
- Sovereign cloud provider registration at `extensions/microsoft-authentication/src/extension.ts:62-68` with custom environment parameters
- Both patterns use `context.subscriptions.push()` to manage lifecycle

---

#### Pattern: AuthenticationProvider Interface Implementation
**Where:** `extensions/microsoft-authentication/src/node/authProvider.ts:39-566`
**What:** Full implementation of VS Code's AuthenticationProvider interface with methods for session management (get, create, remove) and challenge-response authentication.
```typescript
export class MsalAuthProvider implements AuthenticationProvider {
	private readonly _onDidChangeSessionsEmitter = new EventEmitter<AuthenticationProviderAuthenticationSessionsChangeEvent>();
	
	onDidChangeSessions = this._onDidChangeSessionsEmitter.event;

	async getSessions(scopes: string[] | undefined, options: AuthenticationGetSessionOptions = {}): Promise<AuthenticationSession[]> {
		const scopeData = new ScopeData(scopes, undefined, options?.authorizationServer, options?.clientId);
		// ... implementation fetches sessions from public client manager
	}

	async createSession(scopes: readonly string[], options: AuthenticationProviderSessionOptions): Promise<AuthenticationSession> {
		const scopeData = new ScopeData(scopes, undefined, options.authorizationServer, options.clientId);
		const cachedPca = await this._publicClientManager.getOrCreate(scopeData.clientId);
		const flows = getMsalFlows({...});
		// ... iterates through auth flows with error handling
	}

	async removeSession(sessionId: string): Promise<void> {
		// ... removes account from all matching public client applications
	}
}
```
**Variations / call-sites:**
- Challenge-based variants: `createSessionFromChallenges()` at line 328, `getSessionsFromChallenges()` at line 306

---

#### Pattern: Session Creation with Multi-Flow Fallback
**Where:** `extensions/microsoft-authentication/src/node/authProvider.ts:202-276`
**What:** Implements fallback authentication flows: attempts default loopback, then protocol handler, then device code flow with user prompts between attempts.
```typescript
async createSession(scopes: readonly string[], options: AuthenticationProviderSessionOptions): Promise<AuthenticationSession> {
	const flows = getMsalFlows({
		extensionHost: this._context.extension.extensionKind === ExtensionKind.UI ? ExtensionHost.Local : ExtensionHost.Remote,
		supportedClient: isSupportedClient(callbackUri),
		isBrokerSupported: cachedPca.isBrokerAvailable,
		isPortableMode: env.isAppPortable
	});

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
				cachedPca, authority, scopes: scopeData.scopesToSend,
				loginHint: options.account?.label,
				windowHandle: window.nativeHandle ? Buffer.from(window.nativeHandle) : undefined,
				logger: this._logger,
				uriHandler: this._uriHandler,
				callbackUri
			});
			return this.sessionFromAuthenticationResult(result, scopeData.originalScopes);
		} catch (e) {
			lastError = e;
			if (e instanceof ServerError || (e as ClientAuthError)?.errorCode === ClientAuthErrorCodes.userCanceled) {
				throw e;
			}
		}
	}
	throw lastError ?? new Error('No auth flow succeeded');
}
```
**Variations / call-sites:** 
- Challenge response variant at `extensions/microsoft-authentication/src/node/authProvider.ts:328-416`

---

#### Pattern: URI Handler for Callback Registration
**Where:** `extensions/microsoft-authentication/src/UriEventHandler.ts:8-19`
**What:** Custom URI handler implementing both EventEmitter and UriHandler interface to capture OAuth callback URIs from browser redirects.
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
- Instantiated at `extensions/microsoft-authentication/src/extension.ts:110`
- Passed to `MsalAuthProvider.create()` at line 112
- Used in URL handler flow at `extensions/microsoft-authentication/src/node/flows.ts:85`

---

#### Pattern: Credential Storage with Secret Management
**Where:** `extensions/microsoft-authentication/src/betterSecretStorage.ts:15-248`
**What:** Generic wrapper over VS Code's SecretStorage API with async operation queuing, cross-window synchronization, and predicate-based filtering.
```typescript
export class BetterTokenStorage<T> {
	private _operationInProgress = false;
	private _tokensPromise: Promise<Map<string, T>> = Promise.resolve(new Map());
	private readonly _secretStorage: SecretStorage;
	private _didChangeInOtherWindow = new EventEmitter<IDidChangeInOtherWindowEvent<T>>();

	constructor(private keylistKey: string, context: ExtensionContext) {
		this._secretStorage = context.secrets;
		context.subscriptions.push(context.secrets.onDidChange((e) => this.handleSecretChange(e)));
		this.initialize();
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
	}

	async getAll(predicate?: (item: T) => boolean): Promise<T[]> {
		const tokens = await this.getTokens();
		const values = new Array<T>();
		for (const [_, value] of tokens) {
			if (!predicate || predicate(value)) {
				values.push(value);
			}
		}
		return values;
	}
}
```
**Variations / call-sites:**
- Used for session migration at `extensions/microsoft-authentication/src/node/authProvider.ts:118`
- OnDidChangeInOtherWindow event handler at line 194-247

---

#### Pattern: Scope and Tenant Resolution
**Where:** `extensions/microsoft-authentication/src/common/scopeData.ts:14-117`
**What:** Parses scope strings to extract implicit client IDs and tenant identifiers using VSCODE_CLIENT_ID and VSCODE_TENANT prefixes, filters internal scopes for token endpoint, ensures OIDC compliance.
```typescript
export class ScopeData {
	readonly allScopes: string[];
	readonly scopeStr: string;
	readonly scopesToSend: string[];
	readonly clientId: string;
	readonly tenant: string;
	readonly tenantId: string | undefined;
	readonly claims?: string;

	constructor(readonly originalScopes: readonly string[] = [], claims?: string, authorizationServer?: Uri, clientId?: string) {
		const modifiedScopes = [...originalScopes];
		modifiedScopes.sort();
		this.allScopes = modifiedScopes;
		this.scopeStr = modifiedScopes.join(' ');
		this.claims = claims;
		this.scopesToSend = this.getScopesToSend(modifiedScopes);
		this.clientId = clientId?.trim() || this.getClientId(this.allScopes);
		this.tenant = this.getTenant(this.allScopes, authorizationServer);
		this.tenantId = this.getTenantId(this.tenant);
	}

	private getScopesToSend(scopes: string[]): string[] {
		const scopesToSend = scopes.filter(s => !s.startsWith('VSCODE_'));
		const set = new Set(scopesToSend);
		for (const scope of OIDC_SCOPES) {
			set.delete(scope);
		}
		// If we only had OIDC scopes, we need to add a tack-on scope
		if (!set.size) {
			scopesToSend.push(GRAPH_TACK_ON_SCOPE);
		}
		return scopesToSend;
	}
}
```
**Variations / call-sites:**
- Used in getSessions() at `extensions/microsoft-authentication/src/node/authProvider.ts:173`
- Used in createSession() at line 203
- Used in challenge-based flows at lines 318, 341

---

#### Pattern: Event Buffering for Batch Processing
**Where:** `extensions/microsoft-authentication/src/common/event.ts:27-107`
**What:** Delays event firing during critical code sections, supports event reduction/merging to consolidate rapid account changes into single events.
```typescript
export class EventBufferer {
	private data: { buffers: Function[] }[] = [];

	wrapEvent<T>(event: Event<T>, reduce?: (last: T | O | undefined, event: T) => T | O, initial?: O): Event<O | T> {
		return (listener, thisArgs?, disposables?) => {
			return event(i => {
				const data = this.data[this.data.length - 1];
				if (!reduce) {
					if (data) {
						data.buffers.push(() => listener.call(thisArgs, i));
					} else {
						listener.call(thisArgs, i);
					}
					return;
				}
				// ... reduce logic for event merging
			}, undefined, disposables);
		};
	}

	async bufferEventsAsync<R = void>(fn: () => Promise<R>): Promise<R> {
		const data = { buffers: new Array<Function>() };
		this.data.push(data);
		try {
			const r = await fn();
			return r;
		} finally {
			this.data.pop();
			data.buffers.forEach(flush => flush());
		}
	}
}
```
**Variations / call-sites:**
- Account change event wrapping at `extensions/microsoft-authentication/src/node/authProvider.ts:71-92`
- Used in `getAllSessionsForPca()` at line 485

---

#### Pattern: Telemetry Reporter with GDPR Compliance
**Where:** `extensions/microsoft-authentication/src/common/telemetryReporter.ts:16-143`
**What:** Custom telemetry reporter implementing IExperimentationTelemetry, includes GDPR markers, sanitizes PII, tracks authentication flow failures and account events.
```typescript
export class MicrosoftAuthenticationTelemetryReporter implements IExperimentationTelemetry {
	private sharedProperties: Record<string, string> = {};
	protected _telemetryReporter: TelemetryReporter;

	constructor(aiKey: string) {
		this._telemetryReporter = new TelemetryReporter(aiKey);
	}

	sendLoginEvent(scopes: readonly string[]): void {
		/* __GDPR__
			"login" : {
				"owner": "TylerLeonhardt",
				"comment": "Used to determine the usage of the Microsoft Auth Provider.",
				"scopes": { "classification": "PublicNonPersonalData", "purpose": "FeatureInsight" }
			}
		*/
		this._telemetryReporter.sendTelemetryEvent('login', {
			scopes: JSON.stringify(this._scrubGuids(scopes)),
		});
	}

	sendTelemetryErrorEvent(error: Error | string): void {
		let errorMessage: string | undefined;
		if (typeof error === 'string') {
			errorMessage = error;
		} else {
			const authError: AuthError = error as AuthError;
			// don't set error message or stack because it contains PII
		}
		this._telemetryReporter.sendTelemetryErrorEvent('msalError', {
			errorMessage, errorName, errorCode, errorCorrelationId,
		});
	}
}
```
**Variations / call-sites:**
- MicrosoftSovereignCloudAuthenticationTelemetryReporter subclass at `extensions/microsoft-authentication/src/extension.ts:11`
- sendAccountEvent() at `extensions/microsoft-authentication/src/node/authProvider.ts:151`

---

## Summary

The microsoft-authentication extension demonstrates six core patterns for porting VS Code's authentication system:

1. **Provider Registration**: Uses `authentication.registerAuthenticationProvider()` API with declarative capabilities (multiAccount, challenges, authorization servers)
2. **Provider Implementation**: Full contract with `getSessions()`, `createSession()`, `removeSession()` plus challenge-response variants
3. **Multi-Flow Fallback**: Graceful degradation through loopback → protocol handler → device code with user prompts
4. **URI Callback Handling**: Custom EventEmitter+UriHandler implementation for OAuth redirect capture
5. **Secure Token Storage**: Generic async-safe wrapper over SecretStorage with cross-window sync and filtering
6. **Scope/Tenant Parsing**: Implicit client ID and tenant extraction from scope strings using VS Code conventions
7. **Event Batching**: EventBufferer for consolidating rapid account changes to avoid redundant UI updates
8. **Telemetry**: GDPR-compliant reporter with sanitization and error categorization

All patterns rely on VS Code's core APIs: `vscode.authentication`, `vscode.window.registerUriHandler()`, `context.secrets` (SecretStorage), and `EventEmitter`. A Tauri/Rust port would need to replicate these extension APIs with Rust equivalents and ensure cross-platform clipboard/URI handling parity.
