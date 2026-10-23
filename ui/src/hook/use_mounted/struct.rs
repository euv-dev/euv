use super::*;

/// Reactive state for the mount flag.
///
/// The flag is `false` while the first render is still being built and
/// `true` once the DOM has been committed.
///
/// It exists for the decisions that must not be taken during a render:
/// focusing an input, measuring a box, or starting an animation that
/// should begin from the committed layout rather than from a layout
/// that does not exist yet.
#[derive(Clone, CustomDebug, Data)]
pub struct UseMounted {
    /// Whether the component has been mounted.
    #[get(type(copy))]
    pub mounted: Signal<bool>,
}
