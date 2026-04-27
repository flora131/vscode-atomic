//! files — port of `src/vs/platform/files/`.
//!
//! Exports the IFileService async trait, supporting types, and the
//! local-disk implementation.

pub mod service;
pub mod disk;

pub use service::{
    DeleteOptions,
    FileChangeEvent,
    FileChangeType,
    FileStat,
    FileSystemError,
    FileSystemProviderCapabilities,
    FileSystemResult,
    FileType,
    IFileService,
    WriteOptions,
};
pub use disk::DiskFileSystemProvider;
