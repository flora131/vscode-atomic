//! Core git process layer.  Mirrors the `Git` class from extensions/git/src/git.ts.

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use tokio::process::Command;
use tokio_util::codec::{FramedRead, LinesCodec};
use futures::StreamExt;
use tokio::task::JoinHandle;

use crate::scm::{errors::GitError, git_finder::GitInstall};

/// Maximum byte length for a single CLI invocation (path-list chunking).
pub const MAX_CLI_LENGTH: usize = 30_000;

/// Result of a completed git process invocation.
#[derive(Debug, Default)]
pub struct ExecResult {
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub exit_code: i32,
}

/// Wrapper around the git binary with baseline environment isolation.
#[derive(Debug, Clone)]
pub struct Git {
    pub path: PathBuf,
    pub env: HashMap<String, String>,
    pub version: String,
}

impl Git {
    /// Construct from a resolved `GitInstall`.  Sets baseline env vars that
    /// match VS Code's spawn options:
    ///   VSCODE_GIT_COMMAND, LANGUAGE, LC_ALL, GIT_PAGER.
    pub fn new(install: GitInstall) -> Self {
        let mut env = HashMap::new();
        env.insert("VSCODE_GIT_COMMAND".into(), "git".into());
        env.insert("LANGUAGE".into(), "C".into());
        env.insert("LC_ALL".into(), "C".into());
        env.insert("GIT_PAGER".into(), "cat".into());

        Git {
            path: install.path,
            env,
            version: install.version,
        }
    }

    /// Spawn git with the given `args` in `cwd`, optionally writing `stdin_bytes`
    /// to stdin.  Waits for the process to finish and returns `ExecResult`.
    pub async fn exec(
        &self,
        cwd: &Path,
        args: &[&str],
        stdin_bytes: Option<Vec<u8>>,
    ) -> Result<ExecResult, GitError> {
        use tokio::io::AsyncWriteExt;

        let mut cmd = Command::new(&self.path);
        cmd.args(args)
            .current_dir(cwd)
            .envs(&self.env)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());

        if stdin_bytes.is_some() {
            cmd.stdin(std::process::Stdio::piped());
        } else {
            cmd.stdin(std::process::Stdio::null());
        }

        let mut child = cmd.spawn().map_err(|e| GitError::GenericError(e.to_string()))?;

        if let Some(bytes) = stdin_bytes {
            if let Some(mut stdin_handle) = child.stdin.take() {
                stdin_handle
                    .write_all(&bytes)
                    .await
                    .map_err(|e| GitError::GenericError(e.to_string()))?;
            }
        }

        let out = child
            .wait_with_output()
            .await
            .map_err(|e| GitError::GenericError(e.to_string()))?;

        let stderr_str = String::from_utf8_lossy(&out.stderr).to_string();
        if let Some(err) = crate::scm::errors::detect_error(&stderr_str) {
            return Err(err);
        }

        Ok(ExecResult {
            stdout: out.stdout,
            stderr: out.stderr,
            exit_code: out.status.code().unwrap_or(-1),
        })
    }

    /// Spawn git and stream stdout lines via a `FramedRead`/`LinesCodec`.
    /// Returns a `(Stream, JoinHandle)` pair; the JoinHandle lets callers
    /// await process exit.
    pub fn stream(
        &self,
        cwd: &Path,
        args: &[&str],
    ) -> Result<
        (
            impl futures::Stream<Item = Result<String, GitError>>,
            JoinHandle<i32>,
        ),
        GitError,
    > {
        let mut cmd = Command::new(&self.path);
        cmd.args(args)
            .current_dir(cwd)
            .envs(&self.env)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::null())
            .stdin(std::process::Stdio::null());

        let mut child = cmd.spawn().map_err(|e| GitError::GenericError(e.to_string()))?;
        let stdout = child.stdout.take().expect("stdout piped");

        let framed = FramedRead::new(stdout, LinesCodec::new());
        let line_stream = framed.map(|r| r.map_err(|e| GitError::GenericError(e.to_string())));

        let handle = tokio::spawn(async move {
            child.wait().await.map(|s| s.code().unwrap_or(-1)).unwrap_or(-1)
        });

        Ok((line_stream, handle))
    }

    /// Split `args` into chunks where each chunk's joined byte length does not
    /// exceed `MAX_CLI_LENGTH`.  Prefix args (flags etc.) are passed through
    /// unchanged to every chunk.
    pub fn chunk_args(prefix_args: &[&str], path_args: &[&str]) -> Vec<Vec<String>> {
        if path_args.is_empty() {
            return vec![prefix_args.iter().map(|s| s.to_string()).collect()];
        }

        let mut chunks: Vec<Vec<String>> = Vec::new();
        let mut current: Vec<String> = prefix_args.iter().map(|s| s.to_string()).collect();
        let prefix_len: usize = prefix_args.iter().map(|s| s.len() + 1).sum();
        let mut current_len = prefix_len;

        for path in path_args {
            let path_len = path.len() + 1;
            if current_len + path_len > MAX_CLI_LENGTH && current.len() > prefix_args.len() {
                chunks.push(current.clone());
                current = prefix_args.iter().map(|s| s.to_string()).collect();
                current_len = prefix_len;
            }
            current.push(path.to_string());
            current_len += path_len;
        }

        if current.len() > prefix_args.len() || chunks.is_empty() {
            chunks.push(current);
        }

        chunks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── chunk_args tests (no git needed) ──────────────────────────────────

    #[test]
    fn chunk_args_empty_paths_returns_single_prefix_chunk() {
        let chunks = Git::chunk_args(&["diff", "--name-only"], &[]);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0], vec!["diff", "--name-only"]);
    }

    #[test]
    fn chunk_args_small_list_fits_in_one_chunk() {
        let paths: Vec<&str> = vec!["a.rs", "b.rs", "c.rs"];
        let chunks = Git::chunk_args(&["add"], &paths);
        assert_eq!(chunks.len(), 1);
        assert!(chunks[0].contains(&"a.rs".to_string()));
    }

    #[test]
    fn chunk_args_splits_large_path_list() {
        // Generate enough paths to exceed MAX_CLI_LENGTH (30_000 bytes).
        // Each path is ~100 chars; 400 paths = ~40000 bytes total.
        let long_paths: Vec<String> = (0..400)
            .map(|i| format!("workspace/projects/some/really/long/deeply/nested/path/to/a/source/file_number_{:04}.rs", i))
            .collect();
        let path_refs: Vec<&str> = long_paths.iter().map(|s| s.as_str()).collect();

        let chunks = Git::chunk_args(&["add"], &path_refs);
        assert!(chunks.len() > 1, "expected multiple chunks for large path list");

        // Every chunk must start with the prefix.
        for chunk in &chunks {
            assert_eq!(chunk[0], "add");
        }

        // All paths must appear exactly once across all chunks.
        let all_paths: Vec<String> = chunks
            .iter()
            .flat_map(|c| c[1..].to_vec())
            .collect();
        assert_eq!(all_paths.len(), long_paths.len());
    }

    // ── baseline env test ─────────────────────────────────────────────────

    #[test]
    fn new_sets_baseline_env_vars() {
        let install = GitInstall {
            path: PathBuf::from("/usr/bin/git"),
            version: "git version 2.39.0".into(),
        };
        let git = Git::new(install);
        assert_eq!(git.env.get("VSCODE_GIT_COMMAND").map(|s| s.as_str()), Some("git"));
        assert_eq!(git.env.get("LANGUAGE").map(|s| s.as_str()), Some("C"));
        assert_eq!(git.env.get("LC_ALL").map(|s| s.as_str()), Some("C"));
        assert_eq!(git.env.get("GIT_PAGER").map(|s| s.as_str()), Some("cat"));
    }
}
