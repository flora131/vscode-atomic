/**
 * Compile-time shape tests for types.ts.
 *
 * No runtime assertions needed — if tsc accepts this file, all interfaces
 * are structurally sound. Run: npx tsc --noEmit -p tsconfig.json
 */

import type {
  Uri,
  Position,
  Range,
  FileStat,
  OpenFolderRequest,
  ReadFileRequest,
  ReadFileResponse,
  WriteFileRequest,
  WorkbenchReadyPayload,
} from '../types.js';

// ── Uri ───────────────────────────────────────────────────────────────────────

const uri: Uri = {
  scheme: 'file',
  authority: '',
  path: '/home/user/project/file.ts',
  query: '',
  fragment: '',
};

// All fields required
type _UriAllFields = keyof Uri extends 'scheme' | 'authority' | 'path' | 'query' | 'fragment'
  ? true
  : never;
const _uriCheck: _UriAllFields = true; void _uriCheck;

// ── Position ──────────────────────────────────────────────────────────────────

const pos: Position = { line: 0, character: 5 };
type _PosAllFields = keyof Position extends 'line' | 'character' ? true : never;
const _posCheck: _PosAllFields = true; void _posCheck;

// ── Range ─────────────────────────────────────────────────────────────────────

const range: Range = {
  start: { line: 0, character: 0 },
  end:   { line: 0, character: 10 },
};

// ── FileStat ──────────────────────────────────────────────────────────────────

const stat: FileStat = {
  type: 1,
  ctime: Date.now(),
  mtime: Date.now(),
  size: 1024,
};

// ── Request / Response shapes ─────────────────────────────────────────────────

const openReq: OpenFolderRequest = { uri };
const readReq: ReadFileRequest   = { uri };
const readRes: ReadFileResponse  = { content: 'hello' };
const writeReq: WriteFileRequest = { uri, content: 'hello' };
const readyPayload: WorkbenchReadyPayload = { version: '0.0.1' };

// Silence "declared but never read" for pure compile-time checks
void openReq; void readReq; void readRes; void writeReq; void readyPayload;
void pos; void range; void stat;
