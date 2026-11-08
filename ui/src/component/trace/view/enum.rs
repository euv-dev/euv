use super::*;

/// How far along a trace line is.
///
/// The reference publishes three: the line that is still running, the
/// line that failed, and the line that is still waiting.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum EuvTraceStatus {
    /// The line has not started yet.
    #[default]
    Idle,
    /// The line is running.
    Running,
    /// The line failed.
    Error,
    /// The line is still waiting.
    Pending,
}
