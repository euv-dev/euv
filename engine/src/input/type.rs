use super::*;

/// A set of keyboard key code strings currently in a given state.
pub type KeyStateSet = HashSet<String>;

/// A map from touch identifier to touch position.
pub type TouchPointMap = HashMap<i32, Vector2D>;

/// A shared, single-threaded cell holding the engine's [`InputState`].
///
/// Registered DOM event listeners clone this `Rc` to keep the state alive
/// for the lifetime of the document, while game code reads the same cell
/// through `EngineHandle::try_get_input_cell`.
pub type InputStateCell = Rc<EngineCell<InputState>>;

/// A set of gamepad button indices in a single frame edge state.
pub type GamepadButtonSet = HashSet<u32>;

/// A set of gamepad indices in a single frame edge state.
pub type GamepadIndexSet = HashSet<u32>;

/// The states of every gamepad the manager has ever observed, keyed by the
/// browser-assigned gamepad index.
pub type GamepadStateMap = HashMap<u32, GamepadState>;

/// A shared, single-threaded cell holding a [`GamepadManager`].
///
/// `gamepadconnected` / `gamepaddisconnected` listeners own a clone of this
/// `Rc` for the lifetime of the document, so a device that appears between
/// two polls is recorded without waiting for the next frame.
pub type GamepadManagerCell = Rc<EngineCell<GamepadManager>>;
