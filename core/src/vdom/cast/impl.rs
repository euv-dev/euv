use super::*;

/// Converts a `Vec<VirtualNode>` into a `VirtualNode::Fragment`.
///
/// This enables using a `Vec<VirtualNode>` directly in the `html!` macro
/// without manually wrapping it in `VirtualNode::Fragment(...)`.
///
/// # Returns
///
/// - `VirtualNode` - A `VirtualNode::Fragment` containing the nodes, or
///   `VirtualNode::Empty` if the vector is empty.
impl From<Vec<VirtualNode>> for VirtualNode {
    fn from(nodes: Vec<VirtualNode>) -> Self {
        if nodes.is_empty() {
            VirtualNode::Empty
        } else {
            VirtualNode::Fragment(nodes)
        }
    }
}

/// Converts an `Option<VirtualNode>` into a `VirtualNode`.
///
/// `Some(node)` returns the inner node, `None` returns `VirtualNode::Empty`.
///
/// # Returns
///
/// - `VirtualNode` - The inner node if `Some`, otherwise `VirtualNode::Empty`.
impl From<Option<VirtualNode>> for VirtualNode {
    fn from(node: Option<VirtualNode>) -> Self {
        match node {
            Some(node) => node,
            None => VirtualNode::Empty,
        }
    }
}

/// Converts an `Option<Vec<VirtualNode>>` into a `VirtualNode`.
///
/// `Some(vec)` converts the vector into a `VirtualNode::Fragment` (or `Empty`
/// if the vector is empty), `None` returns `VirtualNode::Empty`.
///
/// # Returns
///
/// - `VirtualNode` - A `VirtualNode::Fragment` if `Some` with nodes,
///   `VirtualNode::Empty` if `None` or the vector is empty.
impl From<Option<Vec<VirtualNode>>> for VirtualNode {
    fn from(nodes: Option<Vec<VirtualNode>>) -> Self {
        match nodes {
            Some(nodes) => nodes.into(),
            None => VirtualNode::Empty,
        }
    }
}

/// Wraps a `FnMut(&mut HookContext) -> VirtualNode` closure into a `DynamicNode`.
///
/// This enables writing `{move |_: &mut HookContext| html! { ... }}` directly in HTML markup
/// without explicit `DynamicNode` construction.
impl<F> From<F> for VirtualNode
where
    F: FnMut(&mut HookContext) -> VirtualNode + 'static,
{
    /// Wraps this closure into a `VirtualNode::Dynamic` with a fresh hook context.
    ///
    /// # Returns
    ///
    /// - `VirtualNode` - A dynamic virtual node wrapping this closure.
    fn from(render_fn: F) -> Self {
        VirtualNode::create_dynamic(render_fn)
    }
}

/// Converts a `String` into a text virtual node.
impl From<String> for VirtualNode {
    /// Converts this string into a text virtual node.
    ///
    /// # Returns
    ///
    /// - `VirtualNode` - A text virtual node.
    fn from(text: String) -> Self {
        VirtualNode::Text(TextNode::new(text, None))
    }
}

/// Converts a `&str` into a text virtual node.
impl From<&str> for VirtualNode {
    /// Converts this string slice into a text virtual node.
    ///
    /// # Returns
    ///
    /// - `VirtualNode` - A text virtual node.
    fn from(text: &str) -> Self {
        VirtualNode::Text(TextNode::new(text.to_string(), None))
    }
}

/// Converts an `i32` into a text virtual node.
impl From<i32> for VirtualNode {
    /// Converts this integer into a text virtual node.
    ///
    /// # Returns
    ///
    /// - `VirtualNode` - A text virtual node.
    fn from(value: i32) -> Self {
        VirtualNode::Text(TextNode::new(value.to_string(), None))
    }
}

/// Converts a `usize` into a text virtual node.
impl From<usize> for VirtualNode {
    /// Converts this unsigned integer into a text virtual node.
    ///
    /// # Returns
    ///
    /// - `VirtualNode` - A text virtual node.
    fn from(value: usize) -> Self {
        VirtualNode::Text(TextNode::new(value.to_string(), None))
    }
}

/// Converts a `bool` into a text virtual node.
impl From<bool> for VirtualNode {
    /// Converts this boolean into a text virtual node.
    ///
    /// # Returns
    ///
    /// - `VirtualNode` - A text virtual node.
    fn from(value: bool) -> Self {
        VirtualNode::Text(TextNode::new(value.to_string(), None))
    }
}

/// Converts a signal into a reactive text virtual node.
impl<T> From<Signal<T>> for VirtualNode
where
    T: Clone + PartialEq + Display + 'static,
{
    /// Converts this signal into a reactive text virtual node.
    ///
    /// # Returns
    ///
    /// - `VirtualNode` - A reactive text virtual node.
    fn from(signal: Signal<T>) -> Self {
        signal.as_reactive_text()
    }
}

/// Converts a signal into a reactive text node with listener wiring.
impl<T> AsReactiveText for Signal<T>
where
    T: Clone + PartialEq + Display + 'static,
{
    /// Creates a reactive text node that auto-updates when the signal changes.
    ///
    /// Internally creates a bridge `Signal<String>` that subscribes to the
    /// source signal and updates the text content on every change.
    ///
    /// # Returns
    ///
    /// - `VirtualNode` - A text virtual node with reactive signal binding.
    fn as_reactive_text(&self) -> VirtualNode {
        let source: Signal<T> = *self;
        let string_signal: Signal<String> = Signal::create(source.get().to_string());
        let string_signal_clone: Signal<String> = string_signal;
        source.subscribe(move || {
            string_signal_clone.set(source.get().to_string());
        });
        // The closure above captures `string_signal_clone` (which aliases
        // `string_signal`), so `source` now transitively keeps the bridge
        // alive. Register that dependency so the bridge's heap allocation
        // can be reclaimed once `source` is deactivated.
        BridgeRefsCell::track(string_signal.get_inner(), source.get_inner());
        VirtualNode::Text(TextNode::new(string_signal.get(), Some(string_signal)))
    }
}

/// Constructs an `EventAdapter` that wraps any event-compatible value.
impl<T> EventAdapter<T> {
    /// Returns the inner wrapped value, consuming the adapter.
    ///
    /// # Returns
    ///
    /// - `T` - The inner value.
    pub(crate) fn into_inner(self) -> T {
        self.inner
    }
}

/// Adapts a `FnMut(Event)` closure into an `AttributeValue::Event`.
///
/// Wraps the closure into a `NativeEventHandler` and returns it as an
/// event attribute value. This replaces the `__EventWrapper<F>` type
/// that was previously generated inline by the `html!` macro.
impl<F> EventAdapter<F>
where
    F: FnMut(Event) + 'static,
{
    /// Converts the wrapped closure into an event `AttributeValue`.
    ///
    /// # Arguments
    ///
    /// - `&'static str` - The event name string to associate with the handler.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - An `AttributeValue::Event` wrapping the handler.
    pub fn into_attribute(self, event_name: &'static str) -> AttributeValue {
        AttributeValue::Event(NativeEventHandler::create(event_name, self.into_inner()))
    }
}

/// Converts an event with a specific event name into an `AttributeValue`.
impl<F> From<EventNamedAdapter<F>> for AttributeValue
where
    F: FnMut(Event) + 'static,
{
    /// Converts the wrapped closure with event name into an event `AttributeValue`.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - An `AttributeValue::Event` wrapping the handler.
    fn from(adapter: EventNamedAdapter<F>) -> Self {
        AttributeValue::Event(NativeEventHandler::create(
            adapter.get_event_name(),
            adapter.inner,
        ))
    }
}

/// Converts an event named adapter with `NativeEventHandler` into an `AttributeValue`.
impl From<EventNamedAdapter<NativeEventHandler>> for AttributeValue {
    /// Converts the wrapped handler with event name into an event `AttributeValue`.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - An `AttributeValue::Event` wrapping the handler.
    fn from(mut adapter: EventNamedAdapter<NativeEventHandler>) -> Self {
        let event_name: &'static str = adapter.get_event_name();
        adapter.get_mut_inner().set_event_name(event_name);
        AttributeValue::Event(adapter.inner)
    }
}

/// Converts an event named adapter with optional shared closure into an `AttributeValue`.
///
/// `Some(callback)` becomes `AttributeValue::Event` by wrapping the shared closure
/// into a `NativeEventHandler` with the adapter's event name, and `None` becomes
/// `AttributeValue::Text(String::new())`.
impl From<EventNamedAdapter<Option<Rc<dyn Fn(Event)>>>> for AttributeValue {
    /// Converts the wrapped optional shared closure with event name into an event `AttributeValue`.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - An event attribute if `Some`, otherwise an empty text attribute.
    fn from(adapter: EventNamedAdapter<Option<Rc<dyn Fn(Event)>>>) -> Self {
        let event_name: &'static str = adapter.get_event_name();
        match adapter.inner {
            Some(callback) => AttributeValue::Event(NativeEventHandler::create(
                event_name,
                move |event: Event| {
                    callback(event);
                },
            )),
            None => AttributeValue::Text(String::new()),
        }
    }
}

/// Adapts an owned `NativeEventHandler` into an `AttributeValue::Event` directly.
///
/// When the user already provides a `NativeEventHandler`, the handler is
/// re-wrapped with the given `event_name` to ensure the DOM event listener
/// is bound to the correct event type (e.g., "click" rather than "onclick").
impl EventAdapter<NativeEventHandler> {
    /// Converts the wrapped handler into an event `AttributeValue`.
    ///
    /// Re-wraps the handler with the provided `event_name` so that the
    /// DOM event listener uses the correct event type string.
    ///
    /// # Arguments
    ///
    /// - `&'static str` - The event name to bind the handler to.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - An `AttributeValue::Event` containing the re-wrapped handler.
    pub fn into_attribute(self, event_name: &'static str) -> AttributeValue {
        let mut handler: NativeEventHandler = self.into_inner();
        handler.set_event_name(event_name);
        AttributeValue::Event(handler)
    }
}

/// Adapts an `Option<NativeEventHandler>` into an `AttributeValue`.
///
/// `Some(handler)` becomes `AttributeValue::Event(handler)` re-wrapped with the
/// given event name, and `None` becomes `AttributeValue::Text(String::new())`.
impl EventAdapter<Option<NativeEventHandler>> {
    /// Converts the wrapped optional handler into an attribute value.
    ///
    /// Re-wraps a `Some` handler with the provided `event_name` so that the
    /// DOM event listener uses the correct event type string.
    ///
    /// # Arguments
    ///
    /// - `&'static str` - The event name to bind the handler to.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - An event attribute if `Some`, otherwise an empty text attribute.
    pub fn into_attribute(self, event_name: &'static str) -> AttributeValue {
        match self.into_inner() {
            Some(handler) => EventNamedAdapter::new(handler, event_name).into(),
            None => AttributeValue::Text(String::new()),
        }
    }
}

/// Adapts an `Option<Rc<dyn Fn(Event)>>` into an `AttributeValue`.
///
/// `Some(callback)` becomes `AttributeValue::Event` by wrapping the shared closure
/// into a `NativeEventHandler`, and `None` becomes `AttributeValue::Text(String::new())`.
/// This supports component Props that use `Option<Rc<dyn Fn(Event)>>` for event callbacks.
impl EventAdapter<Option<Rc<dyn Fn(Event)>>> {
    /// Converts the wrapped optional shared closure into an attribute value.
    ///
    /// # Arguments
    ///
    /// - `&'static str` - The event name to bind the handler to.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - An event attribute if `Some`, otherwise an empty text attribute.
    pub fn into_attribute(self, event_name: &'static str) -> AttributeValue {
        match self.into_inner() {
            Some(callback) => AttributeValue::Event(NativeEventHandler::create(
                event_name,
                move |event: Event| {
                    callback(event);
                },
            )),
            None => AttributeValue::Text(String::new()),
        }
    }
}

/// Constructs an `AttrValueAdapter` that wraps any attribute-compatible value.
impl<T> AttrValueAdapter<T> {
    /// Returns the inner wrapped value, consuming the adapter.
    ///
    /// # Returns
    ///
    /// - `T` - The inner value.
    pub(crate) fn into_inner(self) -> T {
        self.inner
    }
}

/// Constructs an `InnerHtmlAdapter` that wraps an `inner_html:` payload.
impl<T> InnerHtmlAdapter<T> {
    /// Returns the inner wrapped value, consuming the adapter.
    ///
    /// Mirrors [`AttrValueAdapter::into_inner`] so the html! macro can
    /// use the same "wrap-then-into-inner" pattern for both adapter
    /// kinds without diverging call sites.
    ///
    /// # Returns
    ///
    /// - `T` - The inner value.
    pub(crate) fn into_inner(self) -> T {
        self.inner
    }
}

/// Adapts a `FnMut(Event)` closure into a callback `AttributeValue`.
///
/// This handles the case where a closure is used as a component callback prop.
/// The closure is converted via `IntoCallbackAttribute::into_callback_attribute()`.
impl<F> AttrValueAdapter<F>
where
    F: FnMut(Event) + 'static,
{
    /// Converts the wrapped closure into a callback `AttributeValue`.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - An event attribute value wrapping the adapted closure.
    pub fn into_callback(self) -> AttributeValue {
        self.into_inner().into()
    }

    /// Converts the wrapped closure into a callback `AttributeValue` with a
    /// custom event name for component props.
    ///
    /// # Arguments
    ///
    /// - `&'static str` - The custom attribute name (e.g., "on-increment", "on-change").
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - An event attribute value with the custom name.
    pub fn into_callback_named(self, name: &'static str) -> AttributeValue {
        AttributeValue::Event(NativeEventHandler::create(name, self.into_inner()))
    }
}

/// Converts a named callback adapter into an `AttributeValue`.
impl<F> From<CallbackNamedAdapter<F>> for AttributeValue
where
    F: FnMut(Event) + 'static,
{
    /// Converts the wrapped closure with custom name into a callback `AttributeValue`.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - An event attribute value with the custom name.
    fn from(adapter: CallbackNamedAdapter<F>) -> Self {
        AttributeValue::Event(NativeEventHandler::create(
            adapter.get_name(),
            adapter.inner,
        ))
    }
}

impl AttrValueAdapter<NativeEventHandler> {
    /// Converts the wrapped handler into a callback `AttributeValue` with a
    /// custom event name for component props.
    ///
    /// # Arguments
    ///
    /// - `&'static str` - The custom attribute name.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - An event attribute value with the custom name.
    pub fn into_callback_named(self, name: &'static str) -> AttributeValue {
        let mut handler: NativeEventHandler = self.into_inner();
        handler.set_event_name(name);
        AttributeValue::Event(handler)
    }
}

/// Adapts an `Option<NativeEventHandler>` into an `AttributeValue`.
impl AttrValueAdapter<Option<NativeEventHandler>> {
    /// Converts the wrapped optional handler into an attribute value.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - An event attribute if `Some`, otherwise an empty text attribute.
    pub fn into_callback(self) -> AttributeValue {
        match self.into_inner() {
            Some(handler) => AttrValueAdapter::new(handler).into(),
            None => AttributeValue::Text(String::new()),
        }
    }

    /// Converts this optional handler into a callback `AttributeValue` with a
    /// custom event name for component props.
    ///
    /// # Arguments
    ///
    /// - `&'static str` - The custom attribute name.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - An event attribute with the custom name if `Some`,
    ///   otherwise an empty text attribute.
    pub fn into_callback_named(self, name: &'static str) -> AttributeValue {
        match self.into_inner() {
            Some(handler) => AttrValueAdapter::new(handler).into_callback_named(name),
            None => AttributeValue::Text(String::new()),
        }
    }
}

/// Adapts any type that implements `Into<AttributeValue>` into an `AttributeValue`.
///
/// This is the fallback path for non-closure attribute values (strings, signals,
/// CSS classes, etc.).
impl<T> From<AttrValueAdapter<T>> for AttributeValue
where
    T: Into<AttributeValue>,
{
    /// Converts the wrapped value into an `AttributeValue`.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - The reactive attribute value.
    fn from(adapter: AttrValueAdapter<T>) -> Self {
        adapter.into_inner().into()
    }
}

/// Adapts an `inner_html:` payload into the matching `AttributeValue`
/// variant, routing through `set_inner_html` instead of the generic
/// `Text` attribute path.
///
/// The two blanket impls below cover the user-visible call sites:
///
/// - `inner_html: "<b>hi</b>"` produces
///   `AttributeValue::InnerHtml(String::from("<b>hi</b>"))`.
/// - `inner_html: html_signal` produces
///   `AttributeValue::InnerHtmlSignal(html_signal)`.
///
/// Each impl only requires its specific source type, so the compiler
/// picks the right one based on the inferred `T` at the call site
/// (no manual `.into()` annotation needed).
impl From<InnerHtmlAdapter<String>> for AttributeValue {
    /// Wraps the static `String` payload in an
    /// `AttributeValue::InnerHtml` variant so the renderer can call
    /// `Element::set_inner_html` on it.
    fn from(adapter: InnerHtmlAdapter<String>) -> Self {
        AttributeValue::InnerHtml(adapter.into_inner())
    }
}

impl From<InnerHtmlAdapter<&str>> for AttributeValue {
    /// Wraps the static `&str` payload by allocating a new `String`
    /// so the renderer owns the data independently of the caller's
    /// borrow lifetime.
    fn from(adapter: InnerHtmlAdapter<&str>) -> Self {
        AttributeValue::InnerHtml(adapter.into_inner().to_owned())
    }
}

impl From<InnerHtmlAdapter<Signal<String>>> for AttributeValue {
    /// Wraps the reactive payload in an `AttributeValue::InnerHtmlSignal`
    /// so the renderer subscribes to the signal and re-applies
    /// `set_inner_html` on every change.
    fn from(adapter: InnerHtmlAdapter<Signal<String>>) -> Self {
        AttributeValue::InnerHtmlSignal(adapter.into_inner())
    }
}
