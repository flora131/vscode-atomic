//! SCM / git service layer — Rust port of the git process layer from
//! extensions/git/src/git.ts and extensions/git/src/repository.ts.

pub mod errors;
pub mod git_finder;
pub mod git;
pub mod repository;

pub use errors::GitError;
pub use git::{Git, ExecResult, MAX_CLI_LENGTH};
pub use git_finder::{find_git, GitInstall};
pub use repository::{Repository, StatusEntry};
