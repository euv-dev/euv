use super::*;

/// Native listener storage owned by one mounted root.
pub(crate) type ScrollAreaListener = (EventTarget, &'static str, Closure<dyn FnMut(Event)>);
/// Observer callback retained until disconnect.
pub(crate) type ScrollAreaObserverCallback = Closure<dyn FnMut(Array)>;
