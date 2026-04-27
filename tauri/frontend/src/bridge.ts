/**
 * bridge.ts — typed wrappers around @tauri-apps/api invoke() and event.listen().
 *
 * Provides a Channel<TReq, TRes> abstraction for request/response IPC,
 * plus typed stubs for the commands the Rust back-end will eventually expose.
 */

import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';

import type {
  OpenFolderRequest,
  ReadFileRequest,
  ReadFileResponse,
  WriteFileRequest,
  WorkbenchReadyPayload,
} from './types.js';

// ─────────────────────────────────────────────────────────────────────────────
// Channel abstraction
// ─────────────────────────────────────────────────────────────────────────────

/**
 * Typed channel wrapping a Tauri command name.
 *
 * Usage:
 *   const ch = new Channel<ReadFileRequest, ReadFileResponse>('read_file');
 *   const result = await ch.call({ uri });
 */
export class Channel<TReq, TRes> {
  constructor(private readonly command: string) {}

  /**
   * Invoke the command and return the typed response.
   * Throws if Tauri returns an error string.
   */
  async call(payload: TReq): Promise<TRes> {
    return invoke<TRes>(this.command, payload as unknown as Record<string, unknown>);
  }
}

// ─────────────────────────────────────────────────────────────────────────────
// Event listener helper
// ─────────────────────────────────────────────────────────────────────────────

/**
 * Subscribe to a Tauri back-end event with typed payload.
 * Returns an unlisten function — call it to unsubscribe.
 */
export async function listenEvent<T>(
  event: string,
  handler: (payload: T) => void,
): Promise<UnlistenFn> {
  return listen<T>(event, (ev) => handler(ev.payload));
}

// ─────────────────────────────────────────────────────────────────────────────
// Command stubs
// ─────────────────────────────────────────────────────────────────────────────

/** Signal the Rust side that the workbench DOM is ready. */
export async function workbenchReady(payload: WorkbenchReadyPayload): Promise<void> {
  return invoke<void>('workbench_ready', payload as unknown as Record<string, unknown>);
}

/** Ask the Rust back-end to open a folder and return its URI. */
export const openFolderChannel = new Channel<OpenFolderRequest, void>('open_folder');

/** Read file contents by URI. */
export const readFileChannel = new Channel<ReadFileRequest, ReadFileResponse>('read_file');

/** Write file contents by URI. */
export const writeFileChannel = new Channel<WriteFileRequest, void>('write_file');
