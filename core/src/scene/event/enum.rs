use super::*;

/// Unified UI interaction event dispatched identically across all 7 platforms.
#[derive(Clone, Debug, PartialEq)]
pub enum UiEventKind {
    /// Primary click or tap activation.
    Click(Point),
    /// Pointer contact down (mouse press, finger touch, stylus down).
    PointerDown(Point),
    /// Pointer motion over or dragging the node.
    PointerMove(Point),
    /// Pointer release.
    PointerUp(Point),
    /// Pointer entered node bounds.
    PointerEnter,
    /// Pointer exited node bounds.
    PointerLeave,
    /// Scroll wheel or touch pan delta.
    Scroll(Point),
    /// Key press while focused.
    KeyDown(KeyCode, Modifiers),
    /// Key release while focused.
    KeyUp(KeyCode, Modifiers),
    /// Text committed via keyboard or IME.
    TextInput(String),
    /// Input value mutated.
    Change(String),
    /// Node gained keyboard focus.
    Focus,
    /// Node lost keyboard focus.
    Blur,
}
