use super::*;

/// The owner document and its shared release-on-final-drop lease.
#[cfg(target_arch = "wasm32")]
pub(crate) type SheetScrollOwner = (Document, Rc<SheetScrollLease>);

/// One root's lifecycle-owned, idempotently synchronized lease slot.
#[cfg(target_arch = "wasm32")]
pub(crate) type SheetScrollSlot = RefCell<Option<SheetScrollOwner>>;

/// Weak per-document entries; the final lease removes its own entry.
#[cfg(target_arch = "wasm32")]
pub(crate) type SheetScrollRegistry = RefCell<Vec<(Document, Weak<SheetScrollLease>)>>;

/// Weak scheduling edge avoids a frame callback retaining its own owner.
#[cfg(target_arch = "wasm32")]
pub(crate) type SheetNextFrame = RefCell<Option<Weak<dyn Fn()>>>;

/// Ordered inline declarations and their original CSS priorities.
#[cfg(target_arch = "wasm32")]
pub(crate) type SheetScrollSnapshot = Vec<(String, String, String)>;

/// A mounted sheet's current parent contract and its exact subscription teardown.
pub(crate) type SheetParentBinding = (Signal<bool>, Option<Signal<bool>>, Box<dyn FnOnce()>);
