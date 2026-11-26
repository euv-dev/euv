use super::*;

/// Parsed `on_event!(EventType => handler)` invocation.
pub(crate) struct OnEventInput {
    /// Target custom event type.
    pub(crate) event_type: Type,
    /// Callback closure handling `&EventType`.
    pub(crate) handler: Expr,
}
