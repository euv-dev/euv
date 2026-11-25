use super::*;

/// Keyboard modifier key states.
#[derive(Clone, Copy, Data, Debug, Default, Eq, Hash, New, Ord, PartialEq, PartialOrd)]
pub struct Modifiers {
    /// Shift modifier pressed.
    #[get(type(copy))]
    pub(crate) shift: bool,
    /// Control modifier pressed.
    #[get(type(copy))]
    pub(crate) ctrl: bool,
    /// Alt / Option modifier pressed.
    #[get(type(copy))]
    pub(crate) alt: bool,
    /// Command / Meta / Super modifier pressed.
    #[get(type(copy))]
    pub(crate) meta: bool,
}

/// Unified pointer event normalizing mouse, touch, and stylus/Apple Pencil input.
#[derive(Clone, Copy, Data, Debug, Default, New, PartialEq)]
pub struct PointerEvent {
    /// Stable pointer contact identifier (multi-touch / stylus tracking).
    #[get(type(copy))]
    pub(crate) pointer_id: u64,
    /// Originating pointer hardware class.
    #[get(type(copy))]
    pub(crate) device: PointerDevice,
    /// Logical coordinates inside the window.
    #[get(type(copy))]
    pub(crate) position: Point,
    /// Activated button.
    #[get(type(copy))]
    pub(crate) button: PointerButton,
    /// Normalized contact pressure in `[0.0, 1.0]`.
    #[get(type(copy))]
    pub(crate) pressure: f32,
    /// Active keyboard modifiers.
    #[get(type(copy))]
    pub(crate) modifiers: Modifiers,
}

/// Unified keyboard event payload.
#[derive(Clone, Copy, Data, Debug, Default, Eq, Hash, New, PartialEq)]
pub struct KeyEvent {
    /// Normalized key code.
    #[get(type(copy))]
    pub(crate) code: KeyCode,
    /// Active keyboard modifiers.
    #[get(type(copy))]
    pub(crate) modifiers: Modifiers,
    /// Whether this event is an OS auto-repeat.
    #[get(type(copy))]
    pub(crate) repeat: bool,
}
