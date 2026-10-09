use super::*;

/// Type-erased clone-able handle to a DOM element, used as the payload of
/// `AttributeValue::Ref` so the renderer can populate the ref without
/// committing to a concrete element type at attribute-set time.
///
/// Internally this is just `Rc<UnsafeCell<Option<JsValue>>>`, the same
/// shape as [`NodeRef<T>`]. We don't carry the phantom type here because
/// `AttributeValue` is `Clone` and we want one type-erased cell.
pub type NodeRefDyn = NodeRef<JsValue>;

/// The write guard over [`INJECTED_CLASSES`] held while recording a class.
/// Spelled out once because the full
/// `Result<Guard, PoisonError<Guard>>` type is too long to read inline.
pub(crate) type InjectedClassesWriteGuard<'a> = RwLockWriteGuard<'a, HashSet<String>>;
