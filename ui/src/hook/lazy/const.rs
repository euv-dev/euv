/// The failure message recorded when a lazy component's factory unwinds
/// with a payload that carries no text.
///
/// Mirrors the `Failed` phase of `LoadState`, which the renderer shows
/// in place of the component.
pub(crate) const LAZY_FACTORY_PANIC_MESSAGE: &str = "factory panicked";

/// The type name this `Debug` implementation reports for a `LazyComponent`.
pub(crate) const LAZY_DEBUG_TYPE_NAME: &str = "LazyComponent";

/// The field name this `Debug` implementation reports for a
/// `LazyComponent`'s cached load state.
pub(crate) const LAZY_DEBUG_FIELD_STATE: &str = "state";
