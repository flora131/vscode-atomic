//! IFileService — port of `src/vs/platform/files/common/files.ts`.

use async_trait::async_trait;
use bitflags::bitflags;
use tokio::sync::mpsc;
use vscode_base::VsUri;

// ─── FileType ────────────────────────────────────────────────────────────────

bitflags! {
    /// Mirror of VS Code `FileType` enum (bit-field in TS).
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct FileType: u32 {
        const FILE            = 1;
        const DIRECTORY       = 2;
        const SYMBOLIC_LINK   = 64;
    }
}

// ─── FileStat ─────────────────────────────────────────────────────────────────

/// Stat metadata — mirrors `IFileStatWithMetadata`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileStat {
    pub ctime:     u64,   // ms since epoch
    pub mtime:     u64,
    pub size:      u64,
    pub file_type: FileType,
}

// ─── FileSystemProviderCapabilities ──────────────────────────────────────────

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct FileSystemProviderCapabilities: u32 {
        const FILE_READ_WRITE           = 1 << 1;
        const FILE_OPEN_READ_WRITE_CLOSE= 1 << 2;
        const FILE_FOLDER_COPY          = 1 << 3;
        const PATH_CASE_SENSITIVE       = 1 << 10;
        const READONLY                  = 1 << 11;
        const TRASH                     = 1 << 12;
        const FILE_WRITE_UNLOCK         = 1 << 13;
        const FILE_ATOMIC_READ          = 1 << 15;
        const FILE_ATOMIC_WRITE         = 1 << 16;
        const FILE_ATOMIC_DELETE        = 1 << 17;
    }
}

// ─── Write / Delete options ───────────────────────────────────────────────────

#[derive(Debug, Clone, Default)]
pub struct WriteOptions {
    pub create:    bool,
    pub overwrite: bool,
    pub atomic:    bool,
}

#[derive(Debug, Clone, Default)]
pub struct DeleteOptions {
    pub recursive:  bool,
    pub use_trash:  bool,
}

// ─── FileChangeEvent ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileChangeType {
    Updated,
    Added,
    Deleted,
}

#[derive(Debug, Clone)]
pub struct FileChangeEvent {
    pub resource:    VsUri,
    pub change_type: FileChangeType,
}

// ─── Error ───────────────────────────────────────────────────────────────────

#[derive(Debug, thiserror::Error)]
pub enum FileSystemError {
    #[error("file not found: {0}")]
    FileNotFound(String),
    #[error("permission denied: {0}")]
    PermissionDenied(String),
    #[error("file already exists: {0}")]
    FileAlreadyExists(String),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("other: {0}")]
    Other(String),
}

pub type FileSystemResult<T> = Result<T, FileSystemError>;

// ─── IFileService trait ───────────────────────────────────────────────────────

#[async_trait]
pub trait IFileService: Send + Sync {
    /// Capabilities this provider supports.
    fn capabilities(&self) -> FileSystemProviderCapabilities;

    /// Stat metadata for a resource.
    async fn stat(&self, resource: &VsUri) -> FileSystemResult<FileStat>;

    /// Read file contents.
    async fn read_file(&self, resource: &VsUri) -> FileSystemResult<Vec<u8>>;

    /// Write file contents.
    async fn write_file(
        &self,
        resource: &VsUri,
        content: &[u8],
        opts: WriteOptions,
    ) -> FileSystemResult<()>;

    /// List entries in a directory: (name, FileType) pairs.
    async fn read_directory(
        &self,
        resource: &VsUri,
    ) -> FileSystemResult<Vec<(String, FileType)>>;

    /// Create a directory (and parents if needed).
    async fn create_directory(&self, resource: &VsUri) -> FileSystemResult<()>;

    /// Delete a file or directory.
    async fn delete(&self, resource: &VsUri, opts: DeleteOptions) -> FileSystemResult<()>;

    /// Rename / move a resource.
    async fn rename(
        &self,
        source: &VsUri,
        target: &VsUri,
        overwrite: bool,
    ) -> FileSystemResult<()>;

    /// Copy a resource.
    async fn copy(
        &self,
        source: &VsUri,
        target: &VsUri,
        overwrite: bool,
    ) -> FileSystemResult<()>;

    /// Check whether a resource exists.
    async fn exists(&self, resource: &VsUri) -> bool;

    /// Subscribe to file-system change events.
    /// Returns an mpsc receiver; watch implementation may be a placeholder.
    async fn watch(&self, resource: &VsUri) -> mpsc::Receiver<Vec<FileChangeEvent>>;
}
