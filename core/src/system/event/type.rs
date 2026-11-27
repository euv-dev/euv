use super::*;

/// Canonical OS/browser appearance theme unified with `ThemeMode`.
pub type SystemTheme = ThemeMode;

/// Shared dynamically dispatched system event listener callback.
pub(crate) type SystemEventCallback = Rc<RefCell<dyn FnMut(&SystemEvent) + 'static>>;
