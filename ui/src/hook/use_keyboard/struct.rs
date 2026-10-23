use super::*;

/// Reactive state for the keyboard.
///
/// The handle carries the key that is currently held down and the
/// teardown that removes the window listeners.
///
/// The key is a `Signal<String>`: a render closure that reads it
/// re-renders when a key goes down or comes up, and nothing else in
/// the tree is disturbed.
#[derive(Clone, CustomDebug, Data)]
pub struct UseKeyboard {
    /// The key currently held down, empty when no key is down.
    #[get(type(copy))]
    pub pressed: Signal<String>,
    /// The teardown that removes the window listeners.
    #[get(pub(crate))]
    #[debug(skip)]
    pub(crate) teardown: Teardown,
}
