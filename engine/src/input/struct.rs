use super::*;

/// A zero-sized namespace struct providing static input event extraction methods.
///
/// This struct follows the same pattern as `App`, serving as a namespace for
/// free-standing input utility functions that extract data from DOM events.
#[derive(Clone, Copy, Data, Debug, Default, Eq, Hash, New, Ord, PartialEq, PartialOrd)]
pub struct Input;

/// Tracks the current state of all input devices (keyboard, mouse, touch)
/// for a single game frame. The state should be updated by event handlers
/// and cleared of per-frame data at the end of each frame.
#[derive(Clone, Data, Debug, New, PartialEq)]
pub struct InputState {
    /// Key codes that were pressed during this frame.
    #[get_mut(pub(crate))]
    #[new(skip)]
    #[set(pub(crate))]
    pub(crate) keys_pressed: KeyStateSet,
    /// Key codes that are currently held down.
    #[get_mut(pub(crate))]
    #[new(skip)]
    #[set(pub(crate))]
    pub(crate) keys_held: KeyStateSet,
    /// Key codes that were released during this frame.
    #[get_mut(pub(crate))]
    #[new(skip)]
    #[set(pub(crate))]
    pub(crate) keys_released: KeyStateSet,
    /// Mouse buttons that were pressed during this frame.
    #[get_mut(pub(crate))]
    #[new(skip)]
    #[set(pub(crate))]
    pub(crate) mouse_buttons_pressed: HashSet<MouseButton>,
    /// Mouse buttons that are currently held down.
    #[get_mut(pub(crate))]
    #[new(skip)]
    #[set(pub(crate))]
    pub(crate) mouse_buttons_held: HashSet<MouseButton>,
    /// Mouse buttons that were released during this frame.
    #[get_mut(pub(crate))]
    #[new(skip)]
    #[set(pub(crate))]
    pub(crate) mouse_buttons_released: HashSet<MouseButton>,
    /// The current mouse position in screen coordinates.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[new(skip)]
    #[set(pub(crate))]
    pub(crate) mouse_position: Vector2D,
    /// The mouse position delta (movement) since the last frame.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[new(skip)]
    #[set(pub(crate))]
    pub(crate) mouse_delta: Vector2D,
    /// Whether the mouse has moved during this frame.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[new(skip)]
    #[set(pub(crate))]
    pub(crate) mouse_moved: bool,
    /// Active touch points mapped by identifier to screen position.
    #[get_mut(pub(crate))]
    #[new(skip)]
    #[set(pub(crate))]
    pub(crate) touch_points: TouchPointMap,
    /// Touch point identifiers that started during this frame.
    #[get_mut(pub(crate))]
    #[new(skip)]
    #[set(pub(crate))]
    pub(crate) touch_started: HashSet<i32>,
    /// Touch point identifiers that ended during this frame.
    #[get_mut(pub(crate))]
    #[new(skip)]
    #[set(pub(crate))]
    pub(crate) touch_ended: HashSet<i32>,
}

/// Per-frame and steady-state data for a single gamepad index.
///
/// The edge sets follow the same contract as [`InputState`]:
/// `buttons_pressed` records a button that went down on the frame it
/// went down and is cleared by [`GamepadState::end_frame`],
/// `buttons_held` records every button currently down and survives
/// `end_frame`, and `buttons_released` records a button that was held
/// and came up on this frame.
#[derive(Clone, Data, Debug, Default, PartialEq)]
pub struct GamepadState {
    /// The browser-reported identifier of the device, e.g. a mapping
    /// string such as `Xbox 360 Controller (XInput STANDARD GAMEPAD)`.
    #[get(type(clone))]
    #[get_mut(pub(crate))]
    pub(crate) id: String,
    /// The current reading of every analog axis, indexed by axis number.
    /// Values outside the deadzone are stored as reported; the deadzone
    /// is applied on read by [`GamepadState::axis`].
    #[get_mut(pub(crate))]
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) axes: Vec<f64>,
    /// The current pressure of every button, indexed by button number.
    /// Analog triggers report a continuous value here, digital buttons
    /// report `1.0` while down and `0.0` while up.
    #[get_mut(pub(crate))]
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) button_values: Vec<f64>,
    /// Whether the device is currently connected. A disconnected pad
    /// keeps its last `id` so callers can still identify it.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) connected: bool,
    /// Button indices that were pressed during this frame.
    #[get_mut(pub(crate))]
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) buttons_pressed: GamepadButtonSet,
    /// Button indices that are currently held down.
    #[get_mut(pub(crate))]
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) buttons_held: GamepadButtonSet,
    /// Button indices that were released during this frame.
    #[get_mut(pub(crate))]
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) buttons_released: GamepadButtonSet,
}

/// Owns the state of every gamepad the page has seen and drives the
/// per-frame poll.
///
/// A poll re-reads `navigator.get_gamepads()` and folds the fresh raw
/// readings into the stored state, computing this frame's edge sets
/// from the previous frame's held set. The DOM connect / disconnect
/// events are honoured too: a device announced between two polls is
/// already present as a state when the next poll runs, so polling
/// alone never loses a connect or a disconnect.
#[derive(Clone, Data, Debug, Default, PartialEq)]
pub struct GamepadManager {
    /// Every gamepad state ever observed, keyed by gamepad index.
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) states: GamepadStateMap,
    /// Gamepad indices that were first seen during this frame.
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) connected: GamepadIndexSet,
    /// Gamepad indices that were last seen leaving during this frame.
    #[get_mut(pub(crate))]
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) disconnected: GamepadIndexSet,
}
