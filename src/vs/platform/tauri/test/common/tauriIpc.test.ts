/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

import assert from 'assert';
import { CancellationTokenSource } from '../../../../base/common/cancellation.js';
import { IDisposable, toDisposable } from '../../../../base/common/lifecycle.js';
import { ensureNoDisposablesAreLeakedInTestSuite } from '../../../../base/test/common/utils.js';
import { ChannelCallRequest, ChannelEventMessage, ChannelResponse, ExtensionHostSidecarRpcEvent, ExtensionSidecarEvent, ExtensionSidecarState, ExtHostRpcDirection, FileStatDto, FileTypeDto, RedactedSecret, TauriBridgeCommands, createExtHostRpcCancelEnvelope, createExtHostRpcErrorEnvelope, createExtHostRpcRequestEnvelope, createExtHostRpcResponseEnvelope } from '../../common/tauriBridge.js';
import { createTauriIpcRuntime, ITauriIpcRuntime, TauriChannelClient, TauriChannelEvent } from '../../common/tauriIpc.js';

suite('TauriChannelClient', () => {

	const store = ensureNoDisposablesAreLeakedInTestSuite();

	test('maps call to channel_call request', async () => {
		const runtime = new TestTauriIpcRuntime();
		const client = store.add(new TauriChannelClient(runtime, { idPrefix: 'test' }));
		const channel = client.getChannel('workbench');

		const result = await channel.call('resolve', { value: 1 });

		assert.deepStrictEqual(result, { ok: true });
		assert.strictEqual(runtime.invocations.length, 1);
		assert.deepStrictEqual(runtime.invocations[0], {
			command: 'channel_call',
			request: {
				requestId: 'test:req:0',
				channel: 'workbench',
				command: 'resolve',
				args: [{ value: 1 }],
				cancellationId: 'test:req:0:cancel'
			}
		});
	});

	test('maps cancellation to cancel_request', async () => {
		const runtime = new TestTauriIpcRuntime();
		runtime.deferNextResponse();

		const client = store.add(new TauriChannelClient(runtime, { idPrefix: 'test' }));
		const cts = store.add(new CancellationTokenSource());
		const promise = client.getChannel('workbench').call('slow', undefined, cts.token);

		cts.cancel();

		await assert.rejects(promise, (error: unknown) => error instanceof Error && error.name === 'Canceled');
		assert.strictEqual(runtime.invocations[1].command, 'cancel_request');
		assert.deepStrictEqual(runtime.invocations[1].request, {
			cancellationId: 'test:req:0:cancel',
			requestId: 'test:req:0'
		});

		runtime.resolveDeferred({ requestId: 'test:req:0', result: 'late' });
	});

	test('maps canceled channel error to CancellationError', async () => {
		const runtime = new TestTauriIpcRuntime({ requestId: 'test:req:0', error: { code: 'Canceled', message: 'Canceled' } });
		const client = store.add(new TauriChannelClient(runtime, { idPrefix: 'test' }));

		await assert.rejects(
			client.getChannel('workbench').call('slow'),
			(error: unknown) => error instanceof Error && error.name === 'Canceled' && error.message === 'Canceled'
		);
	});

	test('propagates generated requestId into supported call args', async () => {
		const runtime = new TestTauriIpcRuntime();
		const client = store.add(new TauriChannelClient(runtime, { idPrefix: 'test' }));

		await client.getChannel('workbench').call('resolve', { requestId: undefined, value: 1 });

		assert.deepStrictEqual((runtime.invocations[0].request as ChannelCallRequest).args, [{ requestId: 'test:req:0', value: 1 }]);
	});

	test('maps listen and dispose to channel_listen and channel_dispose', async () => {
		const runtime = new TestTauriIpcRuntime();
		const client = store.add(new TauriChannelClient(runtime, { idPrefix: 'test' }));
		const channel = client.getChannel('workbench');
		const received: unknown[] = [];

		const disposable = channel.listen('changed', 'arg')(event => received.push(event));
		await runtime.whenIdle();

		assert.deepStrictEqual(runtime.invocations[0], {
			command: 'channel_listen',
			request: {
				requestId: 'test:req:0',
				channel: 'workbench',
				event: 'changed',
				args: ['arg'],
				subscriptionId: 'test:sub:0'
			}
		});

		runtime.fire(TauriChannelEvent, {
			subscriptionId: 'test:sub:0',
			channel: 'workbench',
			event: 'changed',
			payload: { value: 2 }
		});

		assert.deepStrictEqual(received, [{ value: 2 }]);

		disposable.dispose();
		await runtime.whenIdle();

		assert.strictEqual(runtime.invocations[1].command, 'channel_dispose');
		assert.deepStrictEqual(runtime.invocations[1].request, { subscriptionId: 'test:sub:0' });
	});

	test('starts local channel_event listener before channel_listen', async () => {
		const runtime = new TestTauriIpcRuntime();
		const client = store.add(new TauriChannelClient(runtime, { idPrefix: 'test' }));

		const disposable = client.getChannel('workbench').listen('changed')(() => assert.fail());
		await runtime.whenIdle();

		assert.strictEqual(runtime.invocations.length, 1);
		assert.strictEqual(runtime.invocations[0].command, 'channel_listen');
		assert.strictEqual(runtime.listenerCountsAtInvoke[0], 1);

		disposable.dispose();
	});

	test('disposed listener removes local handler and ignores late channel_event payloads', async () => {
		const runtime = new TestTauriIpcRuntime();
		const client = store.add(new TauriChannelClient(runtime, { idPrefix: 'test' }));
		const received: unknown[] = [];

		const disposable = client.getChannel('workbench').listen('changed')(event => received.push(event));
		await runtime.whenIdle();

		const handlers = runtime.getHandlers(TauriChannelEvent);
		assert.strictEqual(handlers.length, 1);

		disposable.dispose();
		await runtime.whenIdle();

		assert.strictEqual(runtime.getHandlers(TauriChannelEvent).length, 0);
		handlers[0]({
			subscriptionId: 'test:sub:0',
			channel: 'workbench',
			event: 'changed',
			payload: { value: 5 }
		});

		assert.deepStrictEqual(received, []);
		assert.strictEqual(runtime.invocations[1].command, 'channel_dispose');
		assert.deepStrictEqual(runtime.invocations[1].request, { subscriptionId: 'test:sub:0' });
	});

	test('maps channel response error to thrown error', async () => {
		const runtime = new TestTauriIpcRuntime({ requestId: 'test:req:0', error: { code: 'ENOENT', message: 'Missing', details: { path: '/tmp/missing' } } });
		const client = store.add(new TauriChannelClient(runtime, { idPrefix: 'test' }));

		await assert.rejects(
			client.getChannel('workbench').call('missing'),
			(error: unknown) => error instanceof Error && error.name === 'ENOENT' && error.message === 'Missing' && Object.prototype.hasOwnProperty.call(error, 'details')
		);
	});

	test('wraps raw Tauri invoke and event payload shape', async () => {
		let listenHandler: ((event: { payload: ChannelEventMessage }) => void) | undefined;
		const runtime = createTauriIpcRuntime(
			async <R>(command: string, args?: unknown): Promise<R> => {
				return { command, args } as R;
			},
			async <T>(_event: string, handler: (event: { payload: T }) => void) => {
				listenHandler = event => {
					handler(event as { payload: T });
				};
				return () => undefined;
			}
		);

		const response = await runtime.invoke('channel_call', {
			requestId: 'test:req:0',
			channel: 'workbench',
			command: 'resolve',
			args: []
		});
		assert.deepStrictEqual(response, {
			command: 'channel_call',
			args: {
				request: {
					requestId: 'test:req:0',
					channel: 'workbench',
					command: 'resolve',
					args: []
				}
			}
		});

		const received: ChannelEventMessage[] = [];
		const disposable = await runtime.listen<ChannelEventMessage>(TauriChannelEvent, event => received.push(event));
		listenHandler?.({
			payload: {
				subscriptionId: 'test:sub:0',
				channel: 'workbench',
				event: 'changed'
			}
		});

		assert.deepStrictEqual(received, [{ subscriptionId: 'test:sub:0', channel: 'workbench', event: 'changed' }]);
		disposable.dispose();
	});

	test('disposed client rejects calls and disables listeners', async () => {
		const runtime = new TestTauriIpcRuntime();
		const client = new TauriChannelClient(runtime, { idPrefix: 'test' });
		const channel = client.getChannel('workbench');

		client.dispose();

		await assert.rejects(channel.call('resolve'), (error: unknown) => error instanceof Error && error.name === 'Canceled');
		channel.listen('changed')(() => assert.fail());
		await runtime.whenIdle();

		assert.strictEqual(runtime.invocations.length, 0);
	});

	test('filters channel events by subscription id, channel, and event', async () => {
		const runtime = new TestTauriIpcRuntime();
		const client = store.add(new TauriChannelClient(runtime, { idPrefix: 'test' }));
		const received: unknown[] = [];

		const disposable = client.getChannel('workbench').listen('changed')(event => received.push(event));
		await runtime.whenIdle();

		runtime.fire(TauriChannelEvent, {
			subscriptionId: 'test:sub:other',
			channel: 'workbench',
			event: 'changed',
			payload: { value: 3 }
		});
		runtime.fire(TauriChannelEvent, {
			subscriptionId: 'test:sub:0',
			channel: 'files',
			event: 'changed',
			payload: { value: 4 }
		});
		runtime.fire(TauriChannelEvent, {
			subscriptionId: 'test:sub:0',
			channel: 'workbench',
			event: 'renamed',
			payload: { value: 5 }
		});
		runtime.fire(TauriChannelEvent, {
			subscriptionId: 'test:sub:0',
			channel: 'workbench',
			event: 'changed',
			payload: { value: 6 }
		});

		assert.deepStrictEqual(received, [{ value: 6 }]);
		disposable.dispose();
	});

	test('bridge DTOs preserve ext host protocol camelCase fields', () => {
		const call: ChannelCallRequest = {
			requestId: 'req-1',
			channel: 'files',
			command: 'stat',
			args: [{ resource: { scheme: 'file', path: '/tmp/a.txt' } }],
			cancellationId: 'cancel-1'
		};
		assert.deepStrictEqual(JSON.parse(JSON.stringify(call)), {
			requestId: 'req-1',
			channel: 'files',
			command: 'stat',
			args: [{ resource: { scheme: 'file', path: '/tmp/a.txt' } }],
			cancellationId: 'cancel-1'
		});

		const stat: FileStatDto = {
			resource: { scheme: 'file', path: '/workspace/src/main.rs' },
			type: FileTypeDto.File,
			ctime: 10,
			mtime: 20,
			size: 30,
			readonly: false
		};
		assert.deepStrictEqual(JSON.parse(JSON.stringify(stat)), {
			resource: { scheme: 'file', path: '/workspace/src/main.rs' },
			type: 'file',
			ctime: 10,
			mtime: 20,
			size: 30,
			readonly: false
		});
	});

	test('ext host rpc envelopes preserve protocol, cancellation, error, and trace ids', () => {
		const request = createExtHostRpcRequestEnvelope('rpc-1', 'trace-rpc-1', ExtHostRpcDirection.ExtHostToMain, {
			actor: 'MainThreadCommands',
			method: '$executeCommand',
			args: ['workbench.action.files.save'],
			cancellationId: 'cancel-rpc-1'
		});
		assert.deepStrictEqual(JSON.parse(JSON.stringify(request)), {
			protocol: 'extHost.protocol',
			requestId: 'rpc-1',
			traceId: 'trace-rpc-1',
			direction: 'extHostToMain',
			type: 'request',
			request: {
				actor: 'MainThreadCommands',
				method: '$executeCommand',
				args: ['workbench.action.files.save'],
				cancellationId: 'cancel-rpc-1'
			}
		});

		assert.deepStrictEqual(JSON.parse(JSON.stringify(createExtHostRpcResponseEnvelope('rpc-1', 'trace-rpc-1', ExtHostRpcDirection.MainToExtHost, { ok: true }))), {
			protocol: 'extHost.protocol', requestId: 'rpc-1', traceId: 'trace-rpc-1', direction: 'mainToExtHost', type: 'response', result: { ok: true }
		});
		assert.deepStrictEqual(JSON.parse(JSON.stringify(createExtHostRpcErrorEnvelope('rpc-2', 'trace-rpc-2', ExtHostRpcDirection.MainToExtHost, { code: 'ENOENT', message: 'Missing', details: { path: '/missing' } }))), {
			protocol: 'extHost.protocol', requestId: 'rpc-2', traceId: 'trace-rpc-2', direction: 'mainToExtHost', type: 'error', error: { code: 'ENOENT', message: 'Missing', details: { path: '/missing' } }
		});
		assert.deepStrictEqual(JSON.parse(JSON.stringify(createExtHostRpcCancelEnvelope('rpc-3', 'trace-rpc-3', ExtHostRpcDirection.ExtHostToMain, 'cancel-rpc-3'))), {
			protocol: 'extHost.protocol', requestId: 'rpc-3', traceId: 'trace-rpc-3', direction: 'extHostToMain', type: 'cancel', cancellationId: 'cancel-rpc-3'
		});
	});

	test('extension sidecar events expose redacted handshake and rpc envelope DTOs', () => {
		const handshake: ExtensionSidecarEvent = {
			event: 'handshake',
			state: ExtensionSidecarState.Ready,
			handshake: {
				token: RedactedSecret,
				transport: { kind: 'tcp', host: '127.0.0.1', port: 0 }
			},
			processId: 42
		};
		assert.deepStrictEqual(JSON.parse(JSON.stringify(handshake)), {
			event: 'handshake',
			state: 'Ready',
			handshake: {
				token: '<redacted>',
				transport: { kind: 'tcp', host: '127.0.0.1', port: 0 }
			},
			processId: 42
		});

		const rpc: ExtensionSidecarEvent = {
			event: ExtensionHostSidecarRpcEvent,
			envelope: createExtHostRpcRequestEnvelope('rpc-4', 'trace-rpc-4', ExtHostRpcDirection.MainToExtHost, {
				actor: 'ExtHostCommands',
				method: '$executeContributedCommand',
				args: ['vscode-api-tests.run']
			})
		};
		assert.strictEqual(rpc.envelope.protocol, 'extHost.protocol');
		assert.strictEqual(rpc.envelope.type, 'request');
		assert.strictEqual(rpc.envelope.request.method, '$executeContributedCommand');
	});
});

class TestTauriIpcRuntime implements ITauriIpcRuntime {

	readonly invocations: Array<{ command: keyof TauriBridgeCommands; request: TauriBridgeCommands[keyof TauriBridgeCommands] }> = [];
	readonly listenerCountsAtInvoke: number[] = [];
	private readonly listeners = new Map<string, Array<(payload: unknown) => void>>();
	private deferred: { resolve: (response: ChannelResponse) => void } | undefined;

	constructor(private readonly response: ChannelResponse = { requestId: 'test:req:0', result: { ok: true } }) { }

	async invoke<K extends keyof TauriBridgeCommands, R = unknown>(command: K, request: TauriBridgeCommands[K]): Promise<R> {
		this.invocations.push({ command, request });
		this.listenerCountsAtInvoke.push(this.listenerCount(TauriChannelEvent));

		if (command === 'channel_call' && this.deferred) {
			return new Promise<R>(resolve => {
				this.deferred!.resolve = response => resolve(response as R);
			});
		}

		return (command === 'channel_call' ? this.response : { requestId: 'test:req:dispose' }) as R;
	}

	async listen<T>(event: string, handler: (payload: T) => void): Promise<IDisposable> {
		const handlers = this.listeners.get(event) ?? [];
		handlers.push(handler as (payload: unknown) => void);
		this.listeners.set(event, handlers);

		return toDisposable(() => {
			const current = this.listeners.get(event) ?? [];
			this.listeners.set(event, current.filter(candidate => candidate !== handler));
		});
	}

	deferNextResponse(): void {
		this.deferred = { resolve: () => undefined };
	}

	resolveDeferred(response: ChannelResponse): void {
		this.deferred?.resolve(response);
		this.deferred = undefined;
	}

	fire(event: string, payload: ChannelEventMessage): void {
		for (const handler of this.listeners.get(event) ?? []) {
			handler(payload);
		}
	}

	getHandlers(event: string): Array<(payload: unknown) => void> {
		return [...(this.listeners.get(event) ?? [])];
	}

	private listenerCount(event: string): number {
		return this.listeners.get(event)?.length ?? 0;
	}

	async whenIdle(): Promise<void> {
		await Promise.resolve();
		await Promise.resolve();
	}
}
