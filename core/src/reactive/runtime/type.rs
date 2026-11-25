use super::*;

/// Type-erased memo recomputation closure returning `(new_value, changed)`.
pub(crate) type MemoComputeFn = Rc<RefCell<dyn FnMut(Option<&dyn Any>) -> (Box<dyn Any>, bool)>>;

/// Staged memo closure and prior cached value extracted outside the runtime borrow.
pub(crate) type StagedMemo = (MemoComputeFn, Option<Box<dyn Any>>);

/// Drained resource handles collected while disposing an ownership scope.
pub(crate) type ScopeDrain = (
    Vec<ScopeId>,
    Vec<EffectId>,
    Vec<MemoId>,
    Vec<SignalId>,
    Vec<Box<dyn FnOnce()>>,
);
