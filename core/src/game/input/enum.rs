use super::*;

/// High-level semantic gameplay actions mapped to keyboard and pointer bindings via `ActionMap`.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GameAction {
    /// Move avatar left.
    MoveLeft,
    /// Move avatar right.
    MoveRight,
    /// Move avatar up.
    MoveUp,
    /// Move avatar down.
    MoveDown,
    /// Primary fire or attack action.
    Fire,
    /// Quick dash / dodge boost.
    Dash,
    /// Jump or thrust impulse.
    Jump,
    /// Pause or resume simulation.
    Pause,
    /// Spawn or reset wave action.
    ActionAlt,
}
