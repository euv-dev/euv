use super::*;

/// Real-time polled keyboard, pointer, and scroll input state for games and interactive viewports.
#[derive(Clone, Data, Debug, Default, New)]
pub struct InputState {
    /// Set of keys currently held down.
    #[new(skip)]
    pub(crate) keys_down: HashSet<KeyCode>,
    /// Set of keys pressed during the current frame.
    #[new(skip)]
    pub(crate) keys_just_pressed: HashSet<KeyCode>,
    /// Set of keys released during the current frame.
    #[new(skip)]
    pub(crate) keys_just_released: HashSet<KeyCode>,
    /// Set of pointer buttons currently held down.
    #[new(skip)]
    pub(crate) buttons_down: HashSet<PointerButton>,
    /// Latest logical pointer coordinates.
    #[new(skip)]
    #[get(type(copy))]
    pub(crate) pointer_position: Point,
    /// Accumulated logical pointer movement delta during the current frame.
    #[new(skip)]
    #[get(type(copy))]
    pub(crate) pointer_delta: Point,
    /// Accumulated scroll delta during the current frame.
    #[new(skip)]
    #[get(type(copy))]
    pub(crate) scroll_delta: Point,
    /// Virtual analog movement stick `(-1.0..=1.0, -1.0..=1.0)` fed by touch drag-joysticks or gamepads.
    #[new(skip)]
    #[get(type(copy))]
    pub(crate) virtual_axis: Point,
    /// Virtual touch/gamepad primary fire trigger state.
    #[new(skip)]
    #[get(type(copy))]
    pub(crate) virtual_fire: bool,
    /// Virtual touch/gamepad dash/boost trigger state.
    #[new(skip)]
    #[get(type(copy))]
    pub(crate) virtual_dash: bool,
}

/// Declarative multi-key and pointer-button binding table mapping physical inputs to `GameAction`s.
#[derive(Clone, Data, Debug)]
pub struct ActionMap {
    /// Keyboard bindings grouped by semantic `GameAction`.
    pub(crate) key_bindings: HashMap<GameAction, Vec<KeyCode>>,
    /// Pointer button bindings grouped by semantic `GameAction`.
    pub(crate) button_bindings: HashMap<GameAction, Vec<PointerButton>>,
}
