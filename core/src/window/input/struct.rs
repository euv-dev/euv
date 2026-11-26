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

/// Real-time cross-platform gesture payload emitted reactively through `BuildContext::get_gesture()`.
#[derive(Clone, Copy, Data, Debug, Default, PartialEq)]
pub struct GestureEvent {
    /// Monotonic gesture sequence counter ensuring back-to-back identical gestures trigger reactivity.
    #[get(type(copy))]
    pub(crate) sequence: u64,
    /// Classified gesture kind (`Tap`, `DoubleTap`, `LongPress`, `Swipe*`, `Pan`, `Pinch`).
    #[get(type(copy))]
    pub(crate) kind: GestureEventKind,
    /// Starting logical coordinates of the gesture.
    #[get(type(copy))]
    pub(crate) origin: Point,
    /// Ending or current logical coordinates of the gesture.
    #[get(type(copy))]
    pub(crate) position: Point,
    /// Net displacement `(dx, dy)` from `origin` to `position`.
    #[get(type(copy))]
    pub(crate) delta: Point,
    /// Relative pinch scale factor (`1.0` for non-pinch gestures, `> 1.0` zoom in, `< 1.0` zoom out).
    #[get(type(copy))]
    pub(crate) scale: f32,
    /// Number of active pointers/fingers involved in the gesture.
    #[get(type(copy))]
    pub(crate) pointer_count: u8,
    /// Originating pointer device class (`Mouse`, `Touch`, `Stylus`).
    #[get(type(copy))]
    pub(crate) device: PointerDevice,
}
