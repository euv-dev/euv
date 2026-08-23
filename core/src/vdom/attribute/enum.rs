use super::*;

/// Type-erased clone-able handle to a DOM element, used as the payload of
/// `AttributeValue::Ref` so the renderer can populate the ref without
/// committing to a concrete element type at attribute-set time.
///
/// Internally this is just `Rc<UnsafeCell<Option<JsValue>>>`, the same
/// shape as [`NodeRef<T>`]. We don't carry the phantom type here because
/// `AttributeValue` is `Clone` and we want one type-erased cell.
pub type NodeRefDyn = NodeRef<JsValue>;

/// Represents the value of an HTML attribute.
///
/// Attributes can be static text, reactive signals, event handlers, dynamic expressions,
/// or CSS class references.
#[derive(Clone, CustomDebug)]
pub enum AttributeValue {
    /// A static string value.
    Text(String),
    /// A dynamic signal-backed value.
    #[debug(skip)]
    Signal(Signal<String>),
    /// An event handler callback.
    #[debug(skip)]
    Event(NativeEventHandler),
    /// A dynamic expression value of any type (for component props).
    Dynamic(String),
    /// A CSS class reference created by the `class!` macro.
    Css(Css),
    /// A reactive handle to the element being created, populated by the
    /// renderer after the corresponding `ref:` attribute fires.
    ///
    /// The renderer does **not** write a `ref="..."` attribute into the
    /// DOM — it intercepts this variant, calls [`NodeRefDyn::set`] with
    /// the freshly-created element, then `clear()`s it on unmount.
    #[debug(skip)]
    Ref(NodeRefDyn),
}
