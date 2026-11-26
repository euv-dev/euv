use super::*;

/// Dismiss callback stored for each active overlay entry in `EuvOverlayStack`.
pub type OverlayDismissCallback = Rc<RefCell<dyn FnMut()>>;
