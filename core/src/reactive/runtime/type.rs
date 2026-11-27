use super::*;

/// Type-erased memo recomputation closure mutating the cached value in place and returning `changed`.
pub(crate) type MemoComputeFn = Rc<RefCell<dyn FnMut(&mut Option<Box<dyn Any>>) -> bool>>;

/// Staged memo closure and prior cached value extracted outside the runtime borrow.
pub(crate) type StagedMemo = (MemoComputeFn, Option<Box<dyn Any>>);
