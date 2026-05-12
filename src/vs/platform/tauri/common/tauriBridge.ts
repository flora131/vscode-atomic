/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

export type JsonPrimitive = string | number | boolean | null;
export type JsonValue = JsonPrimitive | JsonValue[] | { readonly [key: string]: JsonValue };

export type RequestId = string;
export type SubscriptionId = string;
export type CancellationId = string;
export type TraceId = string;

export interface ChannelCallRequest {
	readonly requestId: RequestId;
	readonly channel: string;
	readonly command: string;
	readonly args: readonly JsonValue[];
	readonly cancellationId?: CancellationId;
}

export interface ChannelListenRequest {
	readonly requestId: RequestId;
	readonly channel: string;
	readonly event: string;
	readonly args: readonly JsonValue[];
	readonly subscriptionId?: SubscriptionId;
}

export interface ChannelDisposeRequest {
	readonly subscriptionId: SubscriptionId;
}

export interface CancelRequest {
	readonly cancellationId: CancellationId;
	readonly requestId?: RequestId;
}

export interface ChannelResponse {
	readonly requestId: RequestId;
	readonly result?: JsonValue;
	readonly error?: ChannelError;
}

export interface ChannelEventMessage {
	readonly subscriptionId: SubscriptionId;
	readonly channel: string;
	readonly event: string;
	readonly payload?: JsonValue;
}

export interface ChannelError {
	readonly code: string;
	readonly message: string;
	readonly details?: JsonValue;
}

export const ExtHostProtocolName = 'extHost.protocol';

export const enum ExtHostRpcDirection {
	MainToExtHost = 'mainToExtHost',
	ExtHostToMain = 'extHostToMain'
}

export type ExtHostRpcActor = string;

export interface ExtHostRpcRequestDto {
	readonly actor: ExtHostRpcActor;
	readonly method: string;
	readonly args: readonly JsonValue[];
	readonly cancellationId?: CancellationId;
}

export type ExtHostRpcEnvelope = Readonly<{
	protocol: typeof ExtHostProtocolName;
	requestId: RequestId;
	traceId: TraceId;
	direction: ExtHostRpcDirection;
} & (
		| { type: 'request'; request: ExtHostRpcRequestDto }
		| { type: 'response'; result?: JsonValue }
		| { type: 'error'; error: ChannelError }
		| { type: 'cancel'; cancellationId?: CancellationId }
	)>;

export function createExtHostRpcRequestEnvelope(requestId: RequestId, traceId: TraceId, direction: ExtHostRpcDirection, request: ExtHostRpcRequestDto): ExtHostRpcEnvelope {
	return { protocol: ExtHostProtocolName, requestId, traceId, direction, type: 'request', request };
}

export function createExtHostRpcResponseEnvelope(requestId: RequestId, traceId: TraceId, direction: ExtHostRpcDirection, result?: JsonValue): ExtHostRpcEnvelope {
	return { protocol: ExtHostProtocolName, requestId, traceId, direction, type: 'response', result };
}

export function createExtHostRpcErrorEnvelope(requestId: RequestId, traceId: TraceId, direction: ExtHostRpcDirection, error: ChannelError): ExtHostRpcEnvelope {
	return { protocol: ExtHostProtocolName, requestId, traceId, direction, type: 'error', error };
}

export function createExtHostRpcCancelEnvelope(requestId: RequestId, traceId: TraceId, direction: ExtHostRpcDirection, cancellationId?: CancellationId): ExtHostRpcEnvelope {
	return { protocol: ExtHostProtocolName, requestId, traceId, direction, type: 'cancel', cancellationId };
}

export const enum ExtensionSidecarState {
	NotStarted = 'NotStarted',
	Starting = 'Starting',
	Ready = 'Ready',
	Crashed = 'Crashed',
	Restarting = 'Restarting',
	Stopped = 'Stopped'
}

export type SidecarTransportEndpoint = Readonly<
	| { kind: 'unixSocket'; path: string }
	| { kind: 'namedPipe'; name: string }
	| { kind: 'tcp'; host: string; port: number }
>;

export interface SidecarHandshake {
	readonly token: typeof RedactedSecret;
	readonly transport: SidecarTransportEndpoint;
}

export interface ExtensionSidecarSnapshot {
	readonly state: ExtensionSidecarState;
	readonly handshake?: SidecarHandshake | null;
	readonly processId?: number | null;
}

export type ExtensionSidecarEvent = Readonly<
	| { event: 'handshake'; state: ExtensionSidecarState; handshake: SidecarHandshake; processId?: number | null }
	| { event: 'lifecycle'; state: ExtensionSidecarState; processId?: number | null }
	| { event: 'callback'; payload?: JsonValue }
	| { event: 'rpc'; envelope: ExtHostRpcEnvelope }
>;

export const RedactedSecret = '<redacted>';
export const ExtensionHostServiceChannel = 'extensionHostService';
export const ExtensionHostSidecarRpcEvent = 'rpc';
export const ExtensionHostSidecarCallbackEvent = 'callback';

export interface UriDto {
	readonly scheme: string;
	readonly authority?: string;
	readonly path: string;
	readonly query?: string;
	readonly fragment?: string;
}

export const enum FileTypeDto {
	Unknown = 'unknown',
	File = 'file',
	Directory = 'directory',
	SymbolicLink = 'symbolicLink'
}

export interface FileStatDto {
	readonly resource: UriDto;
	readonly type: FileTypeDto;
	readonly ctime: number;
	readonly mtime: number;
	readonly size: number;
	readonly readonly?: boolean;
}

export interface FileReadRequest {
	readonly resource: UriDto;
}

export interface FileReadResponse {
	readonly dataBase64: string;
}

export interface FileWriteRequest {
	readonly resource: UriDto;
	readonly dataBase64: string;
	readonly create: boolean;
	readonly overwrite: boolean;
}

export interface FileDeleteRequest {
	readonly resource: UriDto;
	readonly recursive: boolean;
	readonly useTrash: boolean;
}

export interface FileStatRequest {
	readonly resource: UriDto;
}

export interface FileMkdirRequest {
	readonly resource: UriDto;
}

export interface FileReaddirRequest {
	readonly resource: UriDto;
}

export interface FileWatchRequest {
	readonly resource: UriDto;
	readonly recursive?: boolean;
}

export interface FileWatchResponse {
	readonly watchId: string;
}

export interface FileDirEntryDto {
	readonly name: string;
	readonly type: FileTypeDto;
}

export interface TauriBridgeCommands {
	readonly channel_call: ChannelCallRequest;
	readonly channel_listen: ChannelListenRequest;
	readonly channel_dispose: ChannelDisposeRequest;
	readonly cancel_request: CancelRequest;
	readonly fs_stat: FileStatRequest;
	readonly fs_read_file: FileReadRequest;
	readonly fs_write_file: FileWriteRequest;
	readonly fs_delete: FileDeleteRequest;
	readonly fs_mkdir: FileMkdirRequest;
	readonly fs_readdir: FileReaddirRequest;
	readonly fs_watch: FileWatchRequest;
}
