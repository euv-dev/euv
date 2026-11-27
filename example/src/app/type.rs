use super::*;

/// Browser `requestAnimationFrame` callback closure stored across foreground/background transitions.
pub(crate) type WebRafCallback = Closure<dyn FnMut(f64)>;
