use super::*;

/// Reactive state for the window's size.
///
/// The handle carries the viewport's width and height and the teardown
/// that removes the window listener.
///
/// The size is a `Signal<i32>`: a render closure that reads it
/// re-renders when the window is resized.
#[derive(Clone, CustomDebug, Data)]
pub struct UseWindowSize {
    /// The viewport width in CSS pixels.
    #[get(type(copy))]
    pub width: Signal<i32>,
    /// The viewport height in CSS pixels.
    #[get(type(copy))]
    pub height: Signal<i32>,
    /// The teardown that removes the window listener.
    #[get(pub(crate))]
    #[debug(skip)]
    pub(crate) teardown: Teardown,
}
