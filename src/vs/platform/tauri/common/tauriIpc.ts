/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

import { CancellationToken } from '../../../base/common/cancellation.js';
import { CancellationError } from '../../../base/common/errors.js';
import { Emitter, Event } from '../../../base/common/event.js';
import { DisposableStore, IDisposable, toDisposable } from '../../../base/common/lifecycle.js';
import { IChannel, IChannelClient } from '../../../base/parts/ipc/common/ipc.js';
import { CancelRequest, ChannelCallRequest, ChannelDisposeRequest, ChannelError, ChannelEventMessage, ChannelListenRequest, ChannelResponse, JsonValue, TauriBridgeCommands } from './tauriBridge.js';

export const TauriChannelEvent = 'channel_event';

export interface ITauriIpcRuntime {
	invoke<K extends keyof TauriBridgeCommands, R = unknown>(command: K, request: TauriBridgeCommands[K]): Promise<R>;
	listen<T>(event: string, handler: (payload: T) => void): Promise<IDisposable>;
}

export interface ITauriEvent<T> {
	readonly payload: T;
}

export interface ITauriInvoke {
	<R = unknown>(command: string, args?: unknown): Promise<R>;
}

export interface ITauriListen {
	<T>(event: string, handler: (event: ITauriEvent<T>) => void): Promise<IDisposable | (() => void)>;
}

export function createTauriIpcRuntime(invoke: ITauriInvoke, listen: ITauriListen): ITauriIpcRuntime {
	return {
		invoke: (command, request) => invoke(command, { request }),
		listen: async <T>(event: string, handler: (payload: T) => void) => {
			const disposable = await listen<T>(event, tauriEvent => handler(tauriEvent.payload));
			return typeof disposable === 'function' ? toDisposable(disposable) : disposable;
		}
	};
}

export interface ITauriChannelClientOptions {
	readonly eventName?: string;
	readonly idPrefix?: string;
}

export class TauriChannelClient implements IChannelClient, IDisposable {

	private readonly disposables = new DisposableStore();
	private readonly eventName: string;
	private readonly idPrefix: string;
	private lastRequestId = 0;
	private lastSubscriptionId = 0;
	private isDisposed = false;

	constructor(private readonly runtime: ITauriIpcRuntime, options: ITauriChannelClientOptions = {}) {
		this.eventName = options.eventName ?? TauriChannelEvent;
		this.idPrefix = options.idPrefix ?? 'tauri-ipc';
	}

	getChannel<T extends IChannel>(channelName: string): T {
		// eslint-disable-next-line local/code-no-dangerous-type-assertions
		return {
			call: (command: string, arg?: unknown, cancellationToken?: CancellationToken) => this.call(channelName, command, arg, cancellationToken),
			listen: (event: string, arg?: unknown) => this.listen(channelName, event, arg)
		} as T;
	}

	dispose(): void {
		this.isDisposed = true;
		this.disposables.dispose();
	}

	private async call<T>(channel: string, command: string, arg?: unknown, cancellationToken = CancellationToken.None): Promise<T> {
		if (this.isDisposed || cancellationToken.isCancellationRequested) {
			throw new CancellationError();
		}

		const requestId = this.nextRequestId();
		const cancellationId = `${requestId}:cancel`;
		const request: ChannelCallRequest = {
			requestId,
			channel,
			command,
			args: toArgs(arg, requestId),
			cancellationId
		};

		let didCancel = false;
		let cancellationListener: IDisposable | undefined;
		const cancellationPromise = new Promise<never>((_, reject) => {
			cancellationListener = cancellationToken.onCancellationRequested(() => {
				didCancel = true;
				void this.cancel({ cancellationId, requestId });
				reject(new CancellationError());
			});
		});

		const responsePromise = (async () => {
			const response = await this.runtime.invoke<'channel_call', ChannelResponse>('channel_call', request);
			throwIfChannelError(response);

			if (didCancel || cancellationToken.isCancellationRequested) {
				throw new CancellationError();
			}

			return response.result as T;
		})();

		try {
			return await Promise.race([responsePromise, cancellationPromise]);
		} finally {
			cancellationListener?.dispose();
		}
	}

	private listen<T>(channel: string, event: string, arg?: unknown): Event<T> {
		if (this.isDisposed) {
			return Event.None;
		}

		const subscriptionId = this.nextSubscriptionId();
		const store = new DisposableStore();
		let didStart = false;
		let didDispose = false;
		const client = this;

		async function start(): Promise<void> {
			if (didStart || didDispose || client.isDisposed) {
				return;
			}

			didStart = true;

			const eventListener = await client.runtime.listen<ChannelEventMessage>(client.eventName, message => {
				if (!didDispose && !client.isDisposed && message.subscriptionId === subscriptionId && message.channel === channel && message.event === event) {
					emitter.fire(message.payload as T);
				}
			});

			if (didDispose || client.isDisposed) {
				eventListener.dispose();
				return;
			}

			store.add(eventListener);

			const requestId = client.nextRequestId();
			const request: ChannelListenRequest = {
				requestId,
				channel,
				event,
				args: toArgs(arg, requestId),
				subscriptionId
			};

			throwIfChannelError(await client.runtime.invoke<'channel_listen', ChannelResponse>('channel_listen', request));
		}

		function disposeEvent(): void {
			if (didDispose) {
				return;
			}

			didDispose = true;
			store.dispose();
			void client.disposeSubscription(subscriptionId);
		}

		const emitter = new Emitter<T>({
			onWillAddFirstListener: () => void start(),
			onDidRemoveLastListener: () => disposeEvent()
		});

		this.disposables.add(toDisposable(() => {
			disposeEvent();
			emitter.dispose();
		}));

		return emitter.event;
	}

	private async cancel(request: CancelRequest): Promise<void> {
		try {
			await this.runtime.invoke('cancel_request', request);
		} catch {
			// Ignore cancellation transport errors. Original call completion owns final state.
		}
	}

	private async disposeSubscription(subscriptionId: string): Promise<void> {
		const request: ChannelDisposeRequest = { subscriptionId };
		try {
			await this.runtime.invoke('channel_dispose', request);
		} catch {
			// Ignore dispose transport errors: listener is already locally disposed.
		}
	}

	private nextRequestId(): string {
		return `${this.idPrefix}:req:${this.lastRequestId++}`;
	}

	private nextSubscriptionId(): string {
		return `${this.idPrefix}:sub:${this.lastSubscriptionId++}`;
	}
}

function toArgs(arg: unknown, requestId: string): readonly JsonValue[] {
	return typeof arg === 'undefined' ? [] : [withRequestId(arg, requestId) as JsonValue];
}

function withRequestId(arg: unknown, requestId: string): unknown {
	if (typeof arg === 'object' && arg !== null && Object.prototype.hasOwnProperty.call(arg, 'requestId')) {
		const request = arg as { readonly requestId?: unknown };
		if (typeof request.requestId === 'undefined') {
			return { ...request, requestId };
		}
	}

	return arg;
}

function throwIfChannelError(response: ChannelResponse): void {
	if (response.error) {
		throw toError(response.error);
	}
}

function toError(error: ChannelError): Error {
	if (error.code === 'Canceled' && error.message === 'Canceled') {
		return new CancellationError();
	}

	const result = new Error(error.message);
	result.name = error.code;
	if (typeof error.details !== 'undefined') {
		Object.assign(result, { details: error.details });
	}
	return result;
}
