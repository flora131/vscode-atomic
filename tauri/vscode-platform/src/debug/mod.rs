//! DAP debug adapter manager.
//!
//! Provides:
//! - `protocol` — DAP message types and common body structs.
//! - `framing`  — Content-Length wire framing (read/write).
//! - `adapter`  — `DebugAdapter` trait + `ExecutableDebugAdapter`.
//! - `session`  — `RawDebugSession` with seq numbering, request correlation,
//!   cancellation, and `DebugAdapterTracker` hooks.

pub mod protocol;
pub mod framing;
pub mod adapter;
pub mod session;

pub use protocol::{
    ProtocolMessage, MessageKind,
    Capabilities, InitializeArguments,
    StoppedEventBody, OutputEventBody, BreakpointEventBody,
    RunInTerminalRequestArguments, RunInTerminalResponseBody,
};
pub use framing::{read_message, write_message};
pub use adapter::{DebugAdapter, ExecutableDebugAdapter};
pub use session::{RawDebugSession, DebugAdapterTracker, NoopTracker};
