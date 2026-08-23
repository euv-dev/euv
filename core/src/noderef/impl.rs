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
    /// # Example
    ///
    /// ```ignore
    /// let input_ref: NodeRef<HtmlInputElement> = App::use_node_ref();
    /// html! { input { ref: input_ref.clone() } }
    /// if let Some(el) = input_ref.get_cloned() {
    ///     el.focus();
    /// }
    /// ```
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
    pub fn set(&self, value: JsValue) {
        // SAFETY: `set` replaces the inner value wholesale via `replace`
        // (which uses `mem::swap` under the hood), so we never hold an
        // overlapping reference. The previous `JsValue` is dropped before
        // the new one is stored.
        let cell: *mut Option<JsValue> = self.inner.get();
        unsafe {
            let _ = (*cell).replace(value);
        }
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
            let _ = (*cell).take();
        }
    }

    /// Returns `true` if an element is currently attached to this handle.
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
