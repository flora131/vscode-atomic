//! Locate the git executable on the host OS.
//!
//! Mirrors the `findGit{Darwin,Windows,Linux}` logic from extensions/git/src/git.ts.

use std::path::PathBuf;
use crate::scm::errors::GitError;

/// Resolved git installation.
#[derive(Debug, Clone)]
pub struct GitInstall {
    pub path: PathBuf,
    pub version: String,
}

/// Find git on the current platform.
pub async fn find_git() -> Result<GitInstall, GitError> {
    #[cfg(target_os = "macos")]
    return find_git_darwin().await;

    #[cfg(target_os = "windows")]
    return find_git_windows().await;

    #[cfg(target_os = "linux")]
    return find_git_linux().await;

    #[allow(unreachable_code)]
    Err(GitError::GitNotFound)
}

/// Query version string from a git binary path.
pub async fn query_version(path: &PathBuf) -> Result<String, GitError> {
    let out = tokio::process::Command::new(path)
        .arg("--version")
        .output()
        .await
        .map_err(|_| GitError::GitNotFound)?;

    if out.status.success() {
        let raw = String::from_utf8_lossy(&out.stdout);
        Ok(raw.trim().to_string())
    } else {
        Err(GitError::GitNotFound)
    }
}

#[cfg(target_os = "macos")]
async fn find_git_darwin() -> Result<GitInstall, GitError> {
    // Try PATH first.
    if let Ok(path) = which_git() {
        let version = query_version(&path).await?;
        return Ok(GitInstall { path, version });
    }

    // Fall back to xcode-select --print-path.
    let xcode_out = tokio::process::Command::new("xcode-select")
        .arg("--print-path")
        .output()
        .await
        .ok();

    if let Some(out) = xcode_out {
        if out.status.success() {
            let base = String::from_utf8_lossy(&out.stdout).trim().to_string();
            let candidate = PathBuf::from(format!("{}/usr/bin/git", base));
            if candidate.exists() {
                let version = query_version(&candidate).await?;
                return Ok(GitInstall { path: candidate, version });
            }
        }
    }

    Err(GitError::GitNotFound)
}

#[cfg(target_os = "windows")]
async fn find_git_windows() -> Result<GitInstall, GitError> {
    // Try PATH first.
    if let Ok(path) = which_git() {
        let version = query_version(&path).await?;
        return Ok(GitInstall { path, version });
    }

    // Probe common install directories (stub – registry lookup deferred).
    let candidates = [
        r"C:\Program Files\Git\bin\git.exe",
        r"C:\Program Files (x86)\Git\bin\git.exe",
    ];
    for c in &candidates {
        let p = PathBuf::from(c);
        if p.exists() {
            let version = query_version(&p).await?;
            return Ok(GitInstall { path: p, version });
        }
    }

    Err(GitError::GitNotFound)
}

#[cfg(target_os = "linux")]
async fn find_git_linux() -> Result<GitInstall, GitError> {
    let path = which_git()?;
    let version = query_version(&path).await?;
    Ok(GitInstall { path, version })
}

/// Resolve `git` via PATH using `which`/`where`.
fn which_git() -> Result<PathBuf, GitError> {
    #[cfg(target_os = "windows")]
    let which_cmd = "where";
    #[cfg(not(target_os = "windows"))]
    let which_cmd = "which";

    let out = std::process::Command::new(which_cmd)
        .arg("git")
        .output()
        .map_err(|_| GitError::GitNotFound)?;

    if out.status.success() {
        let s = String::from_utf8_lossy(&out.stdout);
        // `where` may return multiple lines; take first.
        let first = s.lines().next().unwrap_or("").trim();
        if !first.is_empty() {
            return Ok(PathBuf::from(first));
        }
    }
    Err(GitError::GitNotFound)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn find_git_returns_valid_install_when_git_available() {
        // Skip if git not on PATH — CI may not have it.
        if std::process::Command::new("which").arg("git").status().map(|s| s.success()).unwrap_or(false)
            || std::process::Command::new("where").arg("git").status().map(|s| s.success()).unwrap_or(false)
        {
            let install = find_git().await.expect("find_git should succeed when git is available");
            assert!(install.path.exists(), "git path should exist on disk");
            assert!(install.version.contains("git version"), "version string should contain 'git version'");
        }
    }
}
