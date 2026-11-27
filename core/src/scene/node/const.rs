use super::*;

/// Shared read-only empty `EventHandlers` reference returned when a `SceneNode` has no lazily allocated `NodeExtras`.
pub(crate) const EMPTY_EVENT_HANDLERS_REF: &EventHandlers = &EventHandlers::empty();
