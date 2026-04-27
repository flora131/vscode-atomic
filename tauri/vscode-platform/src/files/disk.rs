//! DiskFileSystemProvider — local-disk impl using tokio::fs.
//!
//! `watch()` is a placeholder returning an empty stream (full impl: task #17).

use async_trait::async_trait;
use tokio::sync::mpsc;
use vscode_base::VsUri;

use super::service::{
    DeleteOptions, FileChangeEvent, FileStat, FileSystemError, FileSystemProviderCapabilities,
    FileSystemResult, FileType, IFileService, WriteOptions,
};

// ─── DiskFileSystemProvider ──────────────────────────────────────────────────

pub struct DiskFileSystemProvider;

impl DiskFileSystemProvider {
    pub fn new() -> Self {
        Self
    }

    fn to_path(resource: &VsUri) -> std::path::PathBuf {
        std::path::PathBuf::from(resource.fs_path())
    }
}

impl Default for DiskFileSystemProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl IFileService for DiskFileSystemProvider {
    fn capabilities(&self) -> FileSystemProviderCapabilities {
        FileSystemProviderCapabilities::FILE_READ_WRITE
            | FileSystemProviderCapabilities::FILE_FOLDER_COPY
            | FileSystemProviderCapabilities::PATH_CASE_SENSITIVE
    }

    async fn stat(&self, resource: &VsUri) -> FileSystemResult<FileStat> {
        let path = Self::to_path(resource);
        let meta = tokio::fs::metadata(&path).await.map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                FileSystemError::FileNotFound(path.display().to_string())
            } else {
                FileSystemError::Io(e)
            }
        })?;

        let file_type = if meta.is_symlink() {
            FileType::SYMBOLIC_LINK
        } else if meta.is_dir() {
            FileType::DIRECTORY
        } else {
            FileType::FILE
        };

        let mtime = meta
            .modified()
            .ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);

        let ctime = meta
            .created()
            .ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);

        Ok(FileStat {
            ctime,
            mtime,
            size: meta.len(),
            file_type,
        })
    }

    async fn read_file(&self, resource: &VsUri) -> FileSystemResult<Vec<u8>> {
        let path = Self::to_path(resource);
        tokio::fs::read(&path).await.map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                FileSystemError::FileNotFound(path.display().to_string())
            } else {
                FileSystemError::Io(e)
            }
        })
    }

    async fn write_file(
        &self,
        resource: &VsUri,
        content: &[u8],
        opts: WriteOptions,
    ) -> FileSystemResult<()> {
        let path = Self::to_path(resource);

        if !opts.create && !path.exists() {
            return Err(FileSystemError::FileNotFound(path.display().to_string()));
        }

        if path.exists() && !opts.overwrite {
            return Err(FileSystemError::FileAlreadyExists(path.display().to_string()));
        }

        // Ensure parent directory exists.
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        if opts.atomic {
            // Write to temp file, then rename for atomicity.
            let tmp = path.with_extension("tmp~");
            tokio::fs::write(&tmp, content).await?;
            tokio::fs::rename(&tmp, &path).await?;
        } else {
            tokio::fs::write(&path, content).await?;
        }

        Ok(())
    }

    async fn read_directory(
        &self,
        resource: &VsUri,
    ) -> FileSystemResult<Vec<(String, FileType)>> {
        let path = Self::to_path(resource);
        let mut entries = tokio::fs::read_dir(&path).await.map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                FileSystemError::FileNotFound(path.display().to_string())
            } else {
                FileSystemError::Io(e)
            }
        })?;

        let mut result = Vec::new();
        while let Some(entry) = entries.next_entry().await? {
            let ft = entry.file_type().await?;
            let kind = if ft.is_symlink() {
                FileType::SYMBOLIC_LINK
            } else if ft.is_dir() {
                FileType::DIRECTORY
            } else {
                FileType::FILE
            };
            let name = entry.file_name().to_string_lossy().into_owned();
            result.push((name, kind));
        }

        Ok(result)
    }

    async fn create_directory(&self, resource: &VsUri) -> FileSystemResult<()> {
        let path = Self::to_path(resource);
        tokio::fs::create_dir_all(&path).await?;
        Ok(())
    }

    async fn delete(&self, resource: &VsUri, opts: DeleteOptions) -> FileSystemResult<()> {
        let path = Self::to_path(resource);

        if !path.exists() {
            return Err(FileSystemError::FileNotFound(path.display().to_string()));
        }

        if path.is_dir() {
            if opts.recursive {
                tokio::fs::remove_dir_all(&path).await?;
            } else {
                tokio::fs::remove_dir(&path).await?;
            }
        } else {
            tokio::fs::remove_file(&path).await?;
        }

        Ok(())
    }

    async fn rename(
        &self,
        source: &VsUri,
        target: &VsUri,
        overwrite: bool,
    ) -> FileSystemResult<()> {
        let src = Self::to_path(source);
        let dst = Self::to_path(target);

        if !src.exists() {
            return Err(FileSystemError::FileNotFound(src.display().to_string()));
        }

        if dst.exists() && !overwrite {
            return Err(FileSystemError::FileAlreadyExists(dst.display().to_string()));
        }

        // Ensure parent of dst exists.
        if let Some(parent) = dst.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        tokio::fs::rename(&src, &dst).await?;
        Ok(())
    }

    async fn copy(
        &self,
        source: &VsUri,
        target: &VsUri,
        overwrite: bool,
    ) -> FileSystemResult<()> {
        let src = Self::to_path(source);
        let dst = Self::to_path(target);

        if !src.exists() {
            return Err(FileSystemError::FileNotFound(src.display().to_string()));
        }

        if dst.exists() && !overwrite {
            return Err(FileSystemError::FileAlreadyExists(dst.display().to_string()));
        }

        // Ensure parent of dst exists.
        if let Some(parent) = dst.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        tokio::fs::copy(&src, &dst).await?;
        Ok(())
    }

    async fn exists(&self, resource: &VsUri) -> bool {
        Self::to_path(resource).exists()
    }

    async fn watch(&self, _resource: &VsUri) -> mpsc::Receiver<Vec<FileChangeEvent>> {
        // Placeholder — full impl deferred to task #17.
        let (_tx, rx) = mpsc::channel(1);
        rx
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn tmp_uri(dir: &TempDir, rel: &str) -> VsUri {
        let p = dir.path().join(rel);
        VsUri::from_file_path(p.to_str().unwrap())
    }

    fn provider() -> DiskFileSystemProvider {
        DiskFileSystemProvider::new()
    }

    // ── write → read round-trip ──────────────────────────────────────────────
    #[tokio::test]
    async fn write_read_roundtrip() {
        let dir = TempDir::new().unwrap();
        let uri = tmp_uri(&dir, "hello.txt");
        let svc = provider();

        svc.write_file(
            &uri,
            b"hello world",
            WriteOptions { create: true, overwrite: false, atomic: false },
        )
        .await
        .unwrap();

        let bytes = svc.read_file(&uri).await.unwrap();
        assert_eq!(bytes, b"hello world");
    }

    // ── atomic write → read ──────────────────────────────────────────────────
    #[tokio::test]
    async fn atomic_write_read_roundtrip() {
        let dir = TempDir::new().unwrap();
        let uri = tmp_uri(&dir, "atomic.txt");
        let svc = provider();

        svc.write_file(
            &uri,
            b"atomic content",
            WriteOptions { create: true, overwrite: false, atomic: true },
        )
        .await
        .unwrap();

        let bytes = svc.read_file(&uri).await.unwrap();
        assert_eq!(bytes, b"atomic content");
    }

    // ── write fails when file exists and overwrite=false ────────────────────
    #[tokio::test]
    async fn write_no_overwrite_fails() {
        let dir = TempDir::new().unwrap();
        let uri = tmp_uri(&dir, "existing.txt");
        let svc = provider();

        svc.write_file(
            &uri,
            b"first",
            WriteOptions { create: true, overwrite: false, atomic: false },
        )
        .await
        .unwrap();

        let result = svc
            .write_file(
                &uri,
                b"second",
                WriteOptions { create: true, overwrite: false, atomic: false },
            )
            .await;

        assert!(matches!(result, Err(FileSystemError::FileAlreadyExists(_))));
    }

    // ── overwrite succeeds ───────────────────────────────────────────────────
    #[tokio::test]
    async fn write_overwrite_succeeds() {
        let dir = TempDir::new().unwrap();
        let uri = tmp_uri(&dir, "overwrite.txt");
        let svc = provider();

        svc.write_file(
            &uri,
            b"v1",
            WriteOptions { create: true, overwrite: false, atomic: false },
        )
        .await
        .unwrap();

        svc.write_file(
            &uri,
            b"v2",
            WriteOptions { create: false, overwrite: true, atomic: false },
        )
        .await
        .unwrap();

        let bytes = svc.read_file(&uri).await.unwrap();
        assert_eq!(bytes, b"v2");
    }

    // ── mkdir + readdir lists entries ────────────────────────────────────────
    #[tokio::test]
    async fn mkdir_and_readdir() {
        let dir = TempDir::new().unwrap();
        let sub = tmp_uri(&dir, "subdir");
        let svc = provider();

        svc.create_directory(&sub).await.unwrap();

        // Write a file inside
        let file_uri = tmp_uri(&dir, "subdir/a.txt");
        svc.write_file(
            &file_uri,
            b"data",
            WriteOptions { create: true, overwrite: false, atomic: false },
        )
        .await
        .unwrap();

        let entries = svc.read_directory(&sub).await.unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].0, "a.txt");
        assert_eq!(entries[0].1, FileType::FILE);
    }

    // ── delete file ──────────────────────────────────────────────────────────
    #[tokio::test]
    async fn delete_file() {
        let dir = TempDir::new().unwrap();
        let uri = tmp_uri(&dir, "todelete.txt");
        let svc = provider();

        svc.write_file(
            &uri,
            b"bye",
            WriteOptions { create: true, overwrite: false, atomic: false },
        )
        .await
        .unwrap();

        svc.delete(&uri, DeleteOptions { recursive: false, use_trash: false })
            .await
            .unwrap();

        assert!(!svc.exists(&uri).await);
    }

    // ── delete recursive ────────────────────────────────────────────────────
    #[tokio::test]
    async fn delete_recursive() {
        let dir = TempDir::new().unwrap();
        let sub = tmp_uri(&dir, "recurse");
        let svc = provider();

        svc.create_directory(&sub).await.unwrap();

        let f = tmp_uri(&dir, "recurse/f.txt");
        svc.write_file(
            &f,
            b"x",
            WriteOptions { create: true, overwrite: false, atomic: false },
        )
        .await
        .unwrap();

        svc.delete(&sub, DeleteOptions { recursive: true, use_trash: false })
            .await
            .unwrap();

        assert!(!svc.exists(&sub).await);
    }

    // ── rename ───────────────────────────────────────────────────────────────
    #[tokio::test]
    async fn rename_file() {
        let dir = TempDir::new().unwrap();
        let src = tmp_uri(&dir, "src.txt");
        let dst = tmp_uri(&dir, "dst.txt");
        let svc = provider();

        svc.write_file(
            &src,
            b"content",
            WriteOptions { create: true, overwrite: false, atomic: false },
        )
        .await
        .unwrap();

        svc.rename(&src, &dst, false).await.unwrap();

        assert!(!svc.exists(&src).await);
        assert!(svc.exists(&dst).await);
        assert_eq!(svc.read_file(&dst).await.unwrap(), b"content");
    }

    // ── copy ─────────────────────────────────────────────────────────────────
    #[tokio::test]
    async fn copy_file() {
        let dir = TempDir::new().unwrap();
        let src = tmp_uri(&dir, "orig.txt");
        let dst = tmp_uri(&dir, "copy.txt");
        let svc = provider();

        svc.write_file(
            &src,
            b"data",
            WriteOptions { create: true, overwrite: false, atomic: false },
        )
        .await
        .unwrap();

        svc.copy(&src, &dst, false).await.unwrap();

        assert!(svc.exists(&src).await);
        assert!(svc.exists(&dst).await);
        assert_eq!(svc.read_file(&dst).await.unwrap(), b"data");
    }

    // ── stat reports correct size and file_type ──────────────────────────────
    #[tokio::test]
    async fn stat_file() {
        let dir = TempDir::new().unwrap();
        let uri = tmp_uri(&dir, "stat.txt");
        let svc = provider();

        let content = b"hello stat";
        svc.write_file(
            &uri,
            content,
            WriteOptions { create: true, overwrite: false, atomic: false },
        )
        .await
        .unwrap();

        let stat = svc.stat(&uri).await.unwrap();
        assert_eq!(stat.size, content.len() as u64);
        assert_eq!(stat.file_type, FileType::FILE);
    }

    // ── stat directory ───────────────────────────────────────────────────────
    #[tokio::test]
    async fn stat_directory() {
        let dir = TempDir::new().unwrap();
        let sub = tmp_uri(&dir, "statdir");
        let svc = provider();

        svc.create_directory(&sub).await.unwrap();

        let stat = svc.stat(&sub).await.unwrap();
        assert_eq!(stat.file_type, FileType::DIRECTORY);
    }

    // ── stat missing → FileNotFound ──────────────────────────────────────────
    #[tokio::test]
    async fn stat_missing_returns_not_found() {
        let dir = TempDir::new().unwrap();
        let uri = tmp_uri(&dir, "missing.txt");
        let svc = provider();

        let result = svc.stat(&uri).await;
        assert!(matches!(result, Err(FileSystemError::FileNotFound(_))));
    }

    // ── exists returns false for missing ────────────────────────────────────
    #[tokio::test]
    async fn exists_false_for_missing() {
        let dir = TempDir::new().unwrap();
        let uri = tmp_uri(&dir, "nope.txt");
        let svc = provider();

        assert!(!svc.exists(&uri).await);
    }

    // ── watch returns empty receiver (placeholder) ───────────────────────────
    #[tokio::test]
    async fn watch_placeholder_returns_empty_receiver() {
        let dir = TempDir::new().unwrap();
        let uri = tmp_uri(&dir, "");
        let svc = provider();

        let mut rx = svc.watch(&uri).await;
        // Channel was dropped on sender side immediately — recv returns None.
        let result = rx.recv().await;
        assert!(result.is_none());
    }

    // ── read_file missing → FileNotFound ────────────────────────────────────
    #[tokio::test]
    async fn read_missing_file_returns_not_found() {
        let dir = TempDir::new().unwrap();
        let uri = tmp_uri(&dir, "absent.txt");
        let svc = provider();

        let result = svc.read_file(&uri).await;
        assert!(matches!(result, Err(FileSystemError::FileNotFound(_))));
    }

    // ── delete missing → FileNotFound ───────────────────────────────────────
    #[tokio::test]
    async fn delete_missing_returns_not_found() {
        let dir = TempDir::new().unwrap();
        let uri = tmp_uri(&dir, "ghost.txt");
        let svc = provider();

        let result = svc
            .delete(&uri, DeleteOptions { recursive: false, use_trash: false })
            .await;
        assert!(matches!(result, Err(FileSystemError::FileNotFound(_))));
    }

    // ── rename fails when dst exists and overwrite=false ────────────────────
    #[tokio::test]
    async fn rename_no_overwrite_fails() {
        let dir = TempDir::new().unwrap();
        let src = tmp_uri(&dir, "s.txt");
        let dst = tmp_uri(&dir, "d.txt");
        let svc = provider();

        svc.write_file(
            &src,
            b"s",
            WriteOptions { create: true, overwrite: false, atomic: false },
        )
        .await
        .unwrap();
        svc.write_file(
            &dst,
            b"d",
            WriteOptions { create: true, overwrite: false, atomic: false },
        )
        .await
        .unwrap();

        let result = svc.rename(&src, &dst, false).await;
        assert!(matches!(result, Err(FileSystemError::FileAlreadyExists(_))));
    }

    // ── copy fails when dst exists and overwrite=false ───────────────────────
    #[tokio::test]
    async fn copy_no_overwrite_fails() {
        let dir = TempDir::new().unwrap();
        let src = tmp_uri(&dir, "cs.txt");
        let dst = tmp_uri(&dir, "cd.txt");
        let svc = provider();

        svc.write_file(
            &src,
            b"s",
            WriteOptions { create: true, overwrite: false, atomic: false },
        )
        .await
        .unwrap();
        svc.write_file(
            &dst,
            b"d",
            WriteOptions { create: true, overwrite: false, atomic: false },
        )
        .await
        .unwrap();

        let result = svc.copy(&src, &dst, false).await;
        assert!(matches!(result, Err(FileSystemError::FileAlreadyExists(_))));
    }

    // ── readdir on missing dir → FileNotFound ───────────────────────────────
    #[tokio::test]
    async fn readdir_missing_returns_not_found() {
        let dir = TempDir::new().unwrap();
        let uri = tmp_uri(&dir, "nodir");
        let svc = provider();

        let result = svc.read_directory(&uri).await;
        assert!(matches!(result, Err(FileSystemError::FileNotFound(_))));
    }
}
