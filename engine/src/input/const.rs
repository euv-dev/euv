/// The DOM event name for key press events, bound to `window`.
pub(crate) const INPUT_EVENT_KEYDOWN: &str = "keydown";

/// The DOM event name for key release events, bound to `window`.
pub(crate) const INPUT_EVENT_KEYUP: &str = "keyup";

/// The DOM event name for mouse button press events, bound to the pointer target.
pub(crate) const INPUT_EVENT_MOUSEDOWN: &str = "mousedown";

/// The DOM event name for mouse button release events, bound to the pointer target.
pub(crate) const INPUT_EVENT_MOUSEUP: &str = "mouseup";

/// The DOM event name for mouse move events, bound to the pointer target.
pub(crate) const INPUT_EVENT_MOUSEMOVE: &str = "mousemove";

/// The DOM event name for the pointer leaving the pointer target.
pub(crate) const INPUT_EVENT_MOUSELEAVE: &str = "mouseleave";

/// The DOM event name for touch start events, bound to the pointer target.
pub(crate) const INPUT_EVENT_TOUCHSTART: &str = "touchstart";

/// The DOM event name for touch move events, bound to the pointer target.
pub(crate) const INPUT_EVENT_TOUCHMOVE: &str = "touchmove";

/// The DOM event name for touch end events, bound to the pointer target.
pub(crate) const INPUT_EVENT_TOUCHEND: &str = "touchend";

/// The DOM event name for the context menu, suppressed on the pointer target
/// so right-click reaches the engine instead of opening the browser menu.
pub(crate) const INPUT_EVENT_CONTEXTMENU: &str = "contextmenu";

/// The DOM event name for gamepad connect events, bound to `window`.
pub(crate) const INPUT_EVENT_GAMEPADCONNECTED: &str = "gamepadconnected";

/// The DOM event name for gamepad disconnect events, bound to `window`.
pub(crate) const INPUT_EVENT_GAMEPADDISCONNECTED: &str = "gamepaddisconnected";

/// The magnitude below which a raw analog axis reading is reported as
/// centered. Analog sticks rest a few percent off zero, so readings
/// strictly inside `[-DEADZONE, +DEADZONE]` would otherwise leak a
/// constant drift into gameplay.
pub(crate) const INPUT_GAMEPAD_AXIS_DEADZONE: f64 = 0.15;

/// The button pressure at or above which a button counts as down. Half
/// keeps a digital button reporting `1.0` and an analog trigger
/// crossing its midpoint agreeing on the same edge.
pub(crate) const GAMEPAD_BUTTON_PRESS_THRESHOLD: f64 = 0.5;
