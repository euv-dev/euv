/// The name of the horizontal resizable direction.
pub(crate) const RESIZABLE_DIRECTION_HORIZONTAL: &str = "horizontal";

/// The name of the vertical resizable direction.
pub(crate) const RESIZABLE_DIRECTION_VERTICAL: &str = "vertical";

/// Keyboard key that moves a horizontal separator to the left.
pub(crate) const RESIZABLE_KEY_ARROW_LEFT: &str = "ArrowLeft";
/// Keyboard key that moves a horizontal separator to the right.
pub(crate) const RESIZABLE_KEY_ARROW_RIGHT: &str = "ArrowRight";
/// Keyboard key that moves a vertical separator up.
pub(crate) const RESIZABLE_KEY_ARROW_UP: &str = "ArrowUp";
/// Keyboard key that moves a vertical separator down.
pub(crate) const RESIZABLE_KEY_ARROW_DOWN: &str = "ArrowDown";
/// Keyboard key that moves a separator to its lower bound.
pub(crate) const RESIZABLE_KEY_HOME: &str = "Home";
/// Keyboard key that moves a separator to its upper bound.
pub(crate) const RESIZABLE_KEY_END: &str = "End";
/// Accessible name shared by resizable separators.
pub(crate) const RESIZABLE_SEPARATOR_LABEL: &str = "Resize panels";
/// Arrow-key adjustment for a standard keyboard press, in percentage points.
pub(crate) const RESIZABLE_KEY_STEP: f64 = 1.0;
/// Arrow-key adjustment when Shift is held, in percentage points.
pub(crate) const RESIZABLE_KEY_MODIFIED_STEP: f64 = 10.0;
