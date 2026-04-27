//! Git error types with stderr-based detection.

use thiserror::Error;

/// All error variants for git operations.
#[derive(Debug, Error, PartialEq, Clone)]
pub enum GitError {
    #[error("git not installed or not found in PATH")]
    NotInstalled,

    #[error("repository is locked (index.lock exists)")]
    RepositoryIsLocked,

    #[error("cannot lock ref")]
    CantLockRef,

    #[error("bad config file")]
    BadConfigFile,

    #[error("authentication failed")]
    AuthenticationFailed,

    #[error("no user.name configured")]
    NoUserNameConfigured,

    #[error("no user.email configured")]
    NoUserEmailConfigured,

    #[error("remote connection error")]
    RemoteConnectionError,

    #[error("dirty work tree")]
    DirtyWorkTree,

    #[error("cannot open resource")]
    CantOpenResource,

    #[error("git executable not found")]
    GitNotFound,

    #[error("not in a git repository")]
    NotInRepository,

    #[error("{0}")]
    GenericError(String),
}

/// Detect a `GitError` from stderr text using the same regexes as `getGitErrorCode` in git.ts.
pub fn detect_error(stderr: &str) -> Option<GitError> {
    use regex::Regex;
    use std::sync::OnceLock;

    macro_rules! pat {
        ($cell:ident, $re:literal) => {{
            static $cell: OnceLock<Regex> = OnceLock::new();
            $cell.get_or_init(|| Regex::new($re).unwrap())
        }};
    }

    if pat!(RE_LOCK, r"(?i)Another git process seems to be running in this repository|If no other git process is currently running").is_match(stderr) {
        return Some(GitError::RepositoryIsLocked);
    }
    if pat!(RE_LOCK_REF, r"(?i)cannot lock ref|unable to update local ref").is_match(stderr) {
        return Some(GitError::CantLockRef);
    }
    if pat!(RE_AUTH, r"(?i)Authentication failed").is_match(stderr) {
        return Some(GitError::AuthenticationFailed);
    }
    if pat!(RE_NOT_REPO, r"(?i)Not a git repository").is_match(stderr) {
        return Some(GitError::NotInRepository);
    }
    if pat!(RE_BAD_CFG, r"bad config file").is_match(stderr) {
        return Some(GitError::BadConfigFile);
    }
    if pat!(RE_DIRTY, r"(?i)Please,? commit your changes or stash them").is_match(stderr) {
        return Some(GitError::DirtyWorkTree);
    }
    if pat!(RE_REMOTE, r"(?i)unable to access|remote connection").is_match(stderr) {
        return Some(GitError::RemoteConnectionError);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_repository_locked() {
        let err = detect_error("Another git process seems to be running in this repository");
        assert_eq!(err, Some(GitError::RepositoryIsLocked));
    }

    #[test]
    fn detects_cant_lock_ref() {
        let err = detect_error("error: cannot lock ref 'refs/heads/main'");
        assert_eq!(err, Some(GitError::CantLockRef));
    }

    #[test]
    fn detects_auth_failed() {
        let err = detect_error("fatal: Authentication failed for 'https://github.com/foo'");
        assert_eq!(err, Some(GitError::AuthenticationFailed));
    }

    #[test]
    fn detects_not_in_repository() {
        let err = detect_error("fatal: Not a git repository (or any of the parent directories): .git");
        assert_eq!(err, Some(GitError::NotInRepository));
    }

    #[test]
    fn returns_none_for_unknown() {
        let err = detect_error("some random output");
        assert_eq!(err, None);
    }
}
