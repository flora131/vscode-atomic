/**
 * TypeScript interfaces matching the Rust shapes in tauri/vscode-base.
 * Kept in sync with VsUri, Position, Range, FileStat in Rust.
 */

// Mirrors vscode-base::uri::VsUri
export interface Uri {
  scheme: string;
  authority: string;
  path: string;
  query: string;
  fragment: string;
}

// Zero-based line and character offsets (same convention as VS Code LSP)
export interface Position {
  line: number;      // 0-based
  character: number; // 0-based
}

// Half-open range [start, end)
export interface Range {
  start: Position;
  end: Position;
}

// Subset of vscode FileStat relevant to the IPC layer
export interface FileStat {
  /** File type bitmap: 0=unknown 1=file 2=dir 64=symlink */
  type: number;
  /** Creation time as Unix ms */
  ctime: number;
  /** Modification time as Unix ms */
  mtime: number;
  /** Size in bytes */
  size: number;
}

// Payload shapes for bridge commands ----------------------------------------

export interface OpenFolderRequest {
  uri: Uri;
}

export interface ReadFileRequest {
  uri: Uri;
}

export interface ReadFileResponse {
  content: string; // UTF-8 text
}

export interface WriteFileRequest {
  uri: Uri;
  content: string; // UTF-8 text
}

export interface WorkbenchReadyPayload {
  version: string;
}
