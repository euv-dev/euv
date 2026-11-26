use super::*;

/// Shared event handler callback stored on a scene node.
pub type UiEventHandler = Rc<RefCell<dyn FnMut(&UiEvent)>>;

/// Custom 2D canvas draw callback invoked during the paint pass.
pub type CanvasDrawCallback = Rc<RefCell<dyn FnMut(Rect, &mut DisplayList)>>;
