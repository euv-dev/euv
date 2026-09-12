use super::*;

impl<T: ?Sized> NodeRef<T> {
    /// Creates a new empty `NodeRef`.
    ///
    /// This constructor is `pub` so that `Default::default()` and
    /// `App::use_node_ref()` can both produce handles. Callers should not
    /// normally need to invoke this directly — use [`App::use_node_ref`]
    /// inside a component so the handle participates in the hook order.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a clone of the raw `JsValue` if an element is currently
    /// attached, otherwise `None`.
    ///
    /// Use this when you only need the underlying DOM element without
    /// caring about its concrete type (e.g., passing it to a third-party
    /// JS interop function). For type-safe access, use [`get_cloned`].
    ///
    /// [`get_cloned`]: NodeRef::get_cloned
    ///
    /// # Returns
    ///
    /// - `Option<JsValue>` - The current value (or a snapshot thereof).
    pub fn get(&self) -> Option<JsValue> {
        // SAFETY: we never hand out `&mut Option<JsValue>`; the only mutating
        // access goes through `set` / `clear`, both of which `take` the
        // existing value first, so there is no aliasing on the inner `JsValue`.
        let cell: *mut Option<JsValue> = self.inner.get();
        unsafe { (*cell).as_ref().cloned() }
    }

    /// Returns a clone of the attached element cast to `T`, or `None` if
    /// no element is attached or the cast fails.
    ///
    /// The cast uses [`JsCast::dyn_into`] and discards the `Err` arm — a
    /// failed cast is reported as `None` rather than panicking, which
    /// matches React/Yew behaviour and avoids crashing the renderer on
    /// ref misuse.
    ///
    /// # Returns
    ///
    /// - `Option<T>` - A cloned copy of the inner value, if present.
    pub fn get_cloned(&self) -> Option<T>
    where
        T: JsCast,
    {
        let value: JsValue = self.get()?;
        value.dyn_into::<T>().ok()
    }

    /// Stores the given element as the current value of the handle.
    ///
    /// This is called by the renderer after a `ref:` attribute fires;
    /// users should not normally need to call it directly. Setting the
    /// value clears any previous element first — multiple mounts of the
    /// same `NodeRef` therefore always reflect the most recent element.
    ///
    /// # Arguments
    ///
    /// - `JsValue` - A `JsValue` parameter.
    pub fn set(&self, value: JsValue) {
        // SAFETY: `set` replaces the inner value wholesale via `replace`
        // (which uses `mem::swap` under the hood), so we never hold an
        // overlapping reference. The previous `JsValue` is dropped before
        // the new one is stored.
        let cell: *mut Option<JsValue> = self.inner.get();
        unsafe {
            let _: Option<JsValue> = (*cell).replace(value);
        }
    }

    /// Returns a shared clone of the interior cell for registry wiring.
    ///
    /// The renderer calls this when a `ref:` attribute fires so the
    /// `NodeRef`'s cell can be registered under the element's `euv_id`
    /// and cleared by `cleanup_subtree` when the element is unmounted.
    ///
    /// # Returns
    ///
    /// - `NodeRefEntry` - A clone of the shared interior cell.
    pub(crate) fn share_cell(&self) -> NodeRefEntry {
        self.inner.clone()
    }

    /// Clears the currently attached element, if any.
    ///
    /// Called by the renderer when a node is unmounted. After `clear`,
    /// [`get`] and [`get_cloned`] both return `None` until the next
    /// `set` call.
    ///
    /// [`get`]: NodeRef::get
    pub fn clear(&self) {
        let cell: *mut Option<JsValue> = self.inner.get();
        unsafe {
            let _: Option<JsValue> = (*cell).take();
        }
    }

    /// Returns `true` if an element is currently attached to this handle.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the value has been initialised.
    pub fn is_set(&self) -> bool {
        let cell: *const Option<JsValue> = self.inner.get();
        // SAFETY: only `is_some()` is called — no `&mut`, no mutation.
        unsafe { (*cell).is_some() }
    }
}

// Blanket impl over the unsized `web_sys::Node` is what most users want,
// but the macro passes a `JsValue` and the user chooses `T` per use site,
// so we don't constrain `T` here — `get_cloned`'s `JsCast` bound is the
// single point where the type check happens.
//
// `NodeRef<dyn Any>` (or any unsized type) is accepted by the type system
// because `T: ?Sized`. The internal `JsValue` storage is independent of
// `T` so there is no soundness concern.

/// Manual `Clone` impl: `T` is `?Sized` so the `derive` macro (which
/// requires `T: Clone`) cannot be used. `Rc` clone is cheap and shares
/// the underlying cell with all clones.
impl<T: ?Sized> Clone for NodeRef<T> {
    /// Clones the [`NodeRef`] by reusing shared, cheap-to-clone state where possible.
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

impl<T: ?Sized> Debug for NodeRef<T> {
    /// Formats the [`NodeRef`] via the supplied formatter.
    ///
    /// # Arguments
    ///
    /// - `&mut Formatter<'_>` - The formatter receiving the formatted output.
    ///
    /// # Returns
    ///
    /// - `fmt::Result` - Result of the formatting operation.
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("NodeRef")
            .field("is_set", &self.is_set())
            .finish()
    }
}
