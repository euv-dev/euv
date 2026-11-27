use super::*;

/// Re-exported JSON value type used by `AiAppHooks` callbacks without requiring callers to add `serde_json`.
pub type AiJsonValue = Value;

/// Backward-compatible alias for `AiRuntimeSnapshot` used across AI Schema v1/v2 inspection APIs.
pub type UiSnapshot = AiRuntimeSnapshot;

/// Callback returning the active application route string.
pub type AiRouteGetter = Rc<dyn Fn() -> String>;

/// Callback returning the active design scheme label string.
pub type AiSchemeGetter = Rc<dyn Fn() -> String>;

/// Callback returning structured 3D/2D game engine telemetry JSON.
pub type AiGameStatsGetter = Rc<dyn Fn() -> Value>;

/// Callback switching the active application route.
pub type AiNavigateHandler = Rc<RefCell<dyn FnMut(&str) -> Result<(), String>>>;

/// Callback switching the active design scheme and/or appearance mode (`Light` / `Dark`).
pub type AiThemeHandler = Rc<RefCell<dyn FnMut(Option<&str>, Option<&str>) -> Result<(), String>>>;

/// Callback dispatching a game engine command with structured JSON parameters.
pub type AiGameControlHandler = Rc<RefCell<dyn FnMut(&str, &Value) -> Result<Value, String>>>;
