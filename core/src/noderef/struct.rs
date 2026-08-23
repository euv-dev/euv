use super::*;

/// A reactive handle to a mounted DOM element.
///
/// `NodeRef` is created via [`App::use_node_ref`] (which routes through the
/// current [`HookContext`]) and is populated by the renderer after the
/// corresponding virtual node is mounted into the real DOM. Before
/// the first mount the inner value is `None`; after unmount it is reset
/// to `None` again, so consumers can rely on `get()` returning `None`
/// to detect the unmounted state.
///
/// The type parameter `T` is purely a phantom marker that names the
/// expected element type (e.g. `NodeRef<HtmlInputElement>`). The runtime
/// stores the element as a raw `JsValue`; calling [`get_cloned`] performs
/// the `dyn_into` cast on demand. This avoids pulling in `web_sys` types
/// in the core hot path and keeps the type zero-cost when the consumer
/// only needs the raw `JsValue`.
///
/// `NodeRef` is `Clone` and cheap to copy (it is an `Rc` clone). All clones
/// share the same underlying cell, so setting the value through one clone
/// is visible through every other clone.
///
/// [`get_cloned`]: NodeRef::get_cloned
pub struct NodeRef<T: ?Sized> {
    /// Shared interior mutability cell holding the (optional) raw DOM
    /// element as a `JsValue`.
    pub(crate) inner: Rc<UnsafeCell<Option<JsValue>>>,
    /// Phantom marker for the expected element type. Not used at runtime
    /// — `get_cloned` only inspects the `T: Into<JsValue>` bound.
    pub(crate) _marker: PhantomData<fn() -> T>,
}

/// Manual `Clone` impl: `T` is `?Sized` so the `derive` macro (which
/// requires `T: Clone`) cannot be used. `Rc` clone is cheap and shares
/// the underlying cell with all clones.
impl<T: ?Sized> Clone for NodeRef<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            _marker: PhantomData,
        }
    }
}

impl<T: ?Sized> Default for NodeRef<T> {
    /// Returns an empty `NodeRef` (no element associated).
    ///
    /// The returned handle is independent of any hook context: it is not
    /// registered as a hook and will never be populated by the renderer
    /// unless it is the same instance that was returned by
    /// [`App::use_node_ref`] and then later attached via a `ref:` attribute.
    /// Prefer [`App::use_node_ref`] inside a component for normal usage.
    fn default() -> Self {
        Self {
            inner: Rc::new(UnsafeCell::new(None)),
            _marker: PhantomData,
        }
    }
}

// `Debug` is implemented manually as well: we want to skip the
// non-`Debug` `JsValue` payload but still expose the phantom marker
// type, which is useful for assertions in tests.
impl<T: ?Sized> std::fmt::Debug for NodeRef<T> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("NodeRef")
            .field("is_set", &self.is_set())
            .finish()
    }
}
