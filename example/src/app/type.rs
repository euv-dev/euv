use super::*;

/// Shared reference-counted slot holding the browser `requestAnimationFrame` callback closure.
pub(crate) type WebRafSlot = Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>>;
