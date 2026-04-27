//! Terminal PTY backend.
//!
//! Provides [`IPty`] trait and [`LocalPty`] backed by [`portable_pty`].
//! Mirror of src/vs/workbench/contrib/terminal/electron-browser/localPty.ts.

pub mod pty;

pub use pty::{spawn, ExitCode, IPty, LocalPty, PtyOutputEvent, PtySpawnOptions};
