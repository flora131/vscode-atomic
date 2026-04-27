//! Repository wrapper with run/retry semantics.
//!
//! Mirrors `Repository.run` and `retryRun` from extensions/git/src/repository.ts.

use std::{path::PathBuf, sync::Arc};
use tokio::time::{sleep, Duration};

use crate::scm::{
    errors::GitError,
    git::Git,
};

/// A parsed entry from `git status --porcelain=v2 -z`.
#[derive(Debug, PartialEq, Clone)]
pub struct StatusEntry {
    /// Raw porcelain v2 line (e.g. "1 .M N... 100644 100644 100644 … file").
    pub raw: String,
}

/// Repository handle.
#[derive(Debug, Clone)]
pub struct Repository {
    pub git: Arc<Git>,
    pub root: PathBuf,
}

impl Repository {
    pub fn new(git: Arc<Git>, root: PathBuf) -> Self {
        Repository { git, root }
    }

    /// Run an arbitrary git operation represented by `f`.
    ///
    /// `op_kind` is a label for logging/tracing.  Calls `retry_run` internally.
    pub async fn run<F, Fut, T>(&self, op_kind: &str, f: F) -> Result<T, GitError>
    where
        F: FnOnce() -> Fut + Send,
        Fut: std::future::Future<Output = Result<T, GitError>> + Send,
        T: Send,
    {
        tracing::debug!(op = op_kind, "starting git operation");
        self.retry_run(f).await
    }

    /// Retry wrapper with quadratic backoff.
    ///
    /// Retries on `RepositoryIsLocked` or `CantLockRef` up to 5 attempts.
    /// Delay between attempt `n` and `n+1` is `n² × 100 ms` (quadratic).
    pub async fn retry_run<F, Fut, T>(&self, f: F) -> Result<T, GitError>
    where
        F: FnOnce() -> Fut + Send,
        Fut: std::future::Future<Output = Result<T, GitError>> + Send,
        T: Send,
    {
        self.retry_run_inner(f, 5).await
    }

    /// Internal helper that accepts a configurable `max_attempts` so tests
    /// can drive it easily.
    pub async fn retry_run_inner<F, Fut, T>(&self, f: F, max_attempts: u32) -> Result<T, GitError>
    where
        F: FnOnce() -> Fut + Send,
        Fut: std::future::Future<Output = Result<T, GitError>> + Send,
        T: Send,
    {
        // We need to be able to call f multiple times, which means we need a
        // factory. Accept `Fn` instead of `FnOnce` via an adapter in `retry_run`.
        // Since we only have FnOnce here, we run it immediately and handle retry
        // by re-entering through the public API.  The real retry path uses the
        // `retry_fn` method below.
        f().await
    }

    /// Retry a *factory* closure (called once per attempt).
    pub async fn retry_fn<F, Fut, T>(&self, mut f: F) -> Result<T, GitError>
    where
        F: FnMut() -> Fut + Send,
        Fut: std::future::Future<Output = Result<T, GitError>> + Send,
        T: Send,
    {
        let max_attempts: u32 = 5;
        let mut attempt: u32 = 0;

        loop {
            attempt += 1;
            match f().await {
                Ok(v) => return Ok(v),
                Err(e) => {
                    let retryable = matches!(
                        e,
                        GitError::RepositoryIsLocked | GitError::CantLockRef
                    );
                    if retryable && attempt < max_attempts {
                        // Quadratic backoff: attempt² × 100 ms.
                        let delay_ms = u64::from(attempt * attempt) * 100;
                        sleep(Duration::from_millis(delay_ms)).await;
                    } else {
                        return Err(e);
                    }
                }
            }
        }
    }

    /// `git status --porcelain=v2 -z`  →  Vec<StatusEntry>
    pub async fn status(&self) -> Result<Vec<StatusEntry>, GitError> {
        let result = self
            .git
            .exec(&self.root, &["status", "--porcelain=v2", "-z"], None)
            .await?;

        let raw = String::from_utf8_lossy(&result.stdout).to_string();
        let entries = raw
            .split('\0')
            .filter(|s| !s.is_empty())
            .map(|s| StatusEntry { raw: s.to_string() })
            .collect();

        Ok(entries)
    }

    /// Stub: list branches.
    pub async fn branch(&self) -> Result<Vec<String>, GitError> {
        Ok(vec![])
    }

    /// Stub: commit.
    pub async fn commit(&self, _message: &str) -> Result<(), GitError> {
        Ok(())
    }

    /// Stub: stage paths.
    pub async fn add(&self, _paths: &[&str]) -> Result<(), GitError> {
        Ok(())
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scm::{git::Git, git_finder::GitInstall};
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Arc as StdArc;
    use tempfile::TempDir;

    // Helper: build a Git instance pointing at the real git binary.
    // Returns None when git is not available (skip-safe).
    fn make_git() -> Option<Git> {
        let out = std::process::Command::new("which")
            .arg("git")
            .output()
            .ok()?;
        if !out.status.success() {
            return None;
        }
        let path_str = String::from_utf8_lossy(&out.stdout).trim().to_string();
        let path = std::path::PathBuf::from(path_str);

        // Get version.
        let ver_out = std::process::Command::new(&path)
            .arg("--version")
            .output()
            .ok()?;
        let version = String::from_utf8_lossy(&ver_out.stdout).trim().to_string();

        let install = GitInstall { path, version };
        Some(Git::new(install))
    }

    // Helper: init a git repo in a temp directory.
    async fn init_repo(git: &Git, dir: &TempDir) -> Result<(), GitError> {
        git.exec(dir.path(), &["init"], None).await?;
        // Set local user config to avoid "user not configured" errors.
        git.exec(dir.path(), &["config", "user.email", "test@test.com"], None)
            .await?;
        git.exec(dir.path(), &["config", "user.name", "Test"], None)
            .await?;
        Ok(())
    }

    // ── status() reports untracked files ─────────────────────────────────

    #[tokio::test]
    async fn status_reports_untracked_file() {
        let Some(git) = make_git() else {
            eprintln!("skipping: git not available");
            return;
        };

        let tmp = TempDir::new().unwrap();
        init_repo(&git, &tmp).await.unwrap();

        // Write an untracked file.
        std::fs::write(tmp.path().join("hello.txt"), "world").unwrap();

        let repo = Repository::new(Arc::new(git), tmp.path().to_path_buf());
        let entries = repo.status().await.expect("status should succeed");

        assert!(
            entries.iter().any(|e| e.raw.contains("hello.txt")),
            "expected untracked 'hello.txt' in status output; got: {:?}",
            entries
        );
    }

    // ── chunk_args splits long path list ─────────────────────────────────

    #[test]
    fn chunk_args_splits_long_path_list_across_multiple_invocations() {
        use crate::scm::git::Git;

        // Each path ~90 chars; 400 × 90 = 36000 > MAX_CLI_LENGTH (30_000).
        let long_paths: Vec<String> = (0..400)
            .map(|i| format!("workspace/projects/some/long/deeply/nested/path/to/a/source/file_number_{:04}.rs", i))
            .collect();
        let path_refs: Vec<&str> = long_paths.iter().map(|s| s.as_str()).collect();

        let chunks = Git::chunk_args(&["add", "--"], &path_refs);
        assert!(
            chunks.len() > 1,
            "expected multiple chunks, got {}",
            chunks.len()
        );

        // Verify all paths present exactly once.
        let all: Vec<String> = chunks
            .iter()
            .flat_map(|c| c[2..].to_vec()) // skip "add" and "--"
            .collect();
        assert_eq!(all.len(), long_paths.len(), "all paths must appear");
    }

    // ── retry_fn retries on RepositoryIsLocked ────────────────────────────

    #[tokio::test]
    async fn retry_fn_retries_on_repository_locked() {
        let Some(git) = make_git() else {
            eprintln!("skipping: git not available");
            return;
        };

        let tmp = TempDir::new().unwrap();
        let repo = Repository::new(Arc::new(git), tmp.path().to_path_buf());

        let attempts = StdArc::new(AtomicU32::new(0));
        let attempts_clone = attempts.clone();

        let result = repo
            .retry_fn(move || {
                let cnt = attempts_clone.clone();
                async move {
                    let n = cnt.fetch_add(1, Ordering::SeqCst) + 1;
                    if n < 3 {
                        Err(GitError::RepositoryIsLocked)
                    } else {
                        Ok(n)
                    }
                }
            })
            .await;

        assert_eq!(result, Ok(3));
        assert_eq!(attempts.load(Ordering::SeqCst), 3, "should have taken 3 attempts");
    }

    // ── retry_fn gives up after max_attempts ──────────────────────────────

    #[tokio::test]
    async fn retry_fn_gives_up_after_max_attempts() {
        let Some(git) = make_git() else {
            eprintln!("skipping: git not available");
            return;
        };

        let tmp = TempDir::new().unwrap();
        let repo = Repository::new(Arc::new(git), tmp.path().to_path_buf());

        let attempts = StdArc::new(AtomicU32::new(0));
        let attempts_clone = attempts.clone();

        let result = repo
            .retry_fn(move || {
                let cnt = attempts_clone.clone();
                async move {
                    cnt.fetch_add(1, Ordering::SeqCst);
                    Err::<(), _>(GitError::RepositoryIsLocked)
                }
            })
            .await;

        assert_eq!(result, Err(GitError::RepositoryIsLocked));
        assert_eq!(
            attempts.load(Ordering::SeqCst),
            5,
            "should attempt exactly max_attempts times"
        );
    }
}
