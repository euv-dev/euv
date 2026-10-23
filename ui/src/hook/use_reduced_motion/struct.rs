use super::*;

/// Reactive state for the user's motion preference.
///
/// The handle carries the current answer and the teardown that removes
/// the `change` listener from the browser's `MediaQueryList`.
///
/// A `true` answer means the user asked the operating system to
/// minimise animation, and the UI should replace motion with a
/// cross-fade. The global stylesheet already honours the preference for
/// its own transitions; this handle exists for the component-level
/// decisions a stylesheet cannot make.
#[derive(Clone, CustomDebug, Data)]
pub struct UseReducedMotion {
    /// Whether the user asked for reduced motion.
    #[get(type(copy))]
    pub reduced: Signal<bool>,
    /// The media query text this handle watches.
    #[get(type(copy))]
    pub query: &'static str,
    /// The teardown that removes the `change` listener.
    #[get(pub(crate))]
    #[debug(skip)]
    pub(crate) teardown: Teardown,
}
