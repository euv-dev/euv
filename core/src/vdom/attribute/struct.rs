use super::*;

/// Represents a single attribute on a virtual DOM node.
///
/// Combines an attribute name with its corresponding value.
#[derive(Clone, CustomDebug, Data, New)]
pub struct AttributeEntry {
    /// The name of the attribute.
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) name: String,
    /// The value of the attribute.
    #[debug(skip)]
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) value: AttributeValue,
}

/// Represents a CSS pseudo-class or pseudo-element rule attached to a class.
///
/// Each rule has a selector suffix (e.g., ":hover", "::before", ":focus")
/// and a style declaration string. When injected into the DOM, it produces
/// a rule like `.class-name:hover { background: red; }`.
#[derive(Clone, Data, Debug, Default, Eq, Hash, New, PartialEq)]
pub struct PseudoRule {
    /// The CSS pseudo selector suffix appended to the class name
    /// (e.g., ":hover", ":focus", ":active", ":disabled", "::before", "::after",
    /// ":first-child", ":last-child", ":nth-child(2n)", etc.).
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    selector: String,
    /// The CSS style declarations for this pseudo rule
    /// (e.g., "background: rgba(79, 70, 229, 0.04); color: #4f46e5;").
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    style: String,
}

/// Represents a CSS class with a name, its style declarations, and optional pseudo rules.
///
/// Created by the `class!` macro and used in `html!` via the `class:` attribute.
/// When the renderer encounters a `Css`, it injects the styles into the
/// DOM's `<style>` element on first use and applies the class name to the element.
#[derive(Clone, Data, Debug, Default, New)]
pub struct Css {
    /// The CSS class name used in the DOM.
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    name: String,
    /// The CSS style declarations (e.g., "max-width: 800px; margin: 0 auto;").
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    style: String,
    /// The pseudo-class and pseudo-element rules for this class
    /// (e.g., ":hover", ":focus", ":active", "::before", etc.).
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pseudo_rules: Vec<PseudoRule>,
    /// The media query rules for this class.
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    media_rules: Vec<MediaRule>,
}

/// Represents a CSS @media rule attached to a class.
///
/// Each media rule has a query string (e.g., "(max-width: 767px)"),
/// a style declaration string, and optional nested pseudo-element rules.
/// When injected into the DOM, it produces a rule like:
/// `@media (max-width: 767px) { .class-name { font-size: 14px; } .class-name::-webkit-scrollbar { width: 0px; } }`.
#[derive(Clone, Data, Debug, Default, Eq, Hash, New, PartialEq)]
pub struct MediaRule {
    /// The media query condition string (e.g., "(max-width: 767px)").
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    query: String,
    /// The CSS style declarations inside this media rule
    /// (e.g., "font-size: 14px; padding: 8px;").
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    style: String,
    /// The pseudo-element rules nested inside this media rule
    /// (e.g., `::-webkit-scrollbar { width: "0px"; }`).
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pseudo_rules: Vec<PseudoRule>,
}

/// Adapts various event value types into an `AttributeValue` for event attributes.
///
/// The `html!` macro generates `EventAdapter::new(expr).into_attribute(event_name)`
/// instead of inline trait dispatch boilerplate. This eliminates the per-attribute-site
/// generation of `__EventWrapper`, `__IsClosure`, `__ClosurePicker`, `__ValuePicker`,
/// `__FallbackHelper`, and `__dispatch` types, significantly reducing macro output size.
///
/// The adapter pattern handles three cases:
/// - `FnMut(NativeEvent)` closure → `AttributeValue::Event` via `NativeEventHandler`
/// - `NativeEventHandler` directly → `AttributeValue::Event` as-is
/// - `Option<NativeEventHandler>` → `AttributeValue::Event` or `AttributeValue::Text`
#[derive(Data, Debug, New)]
pub struct EventAdapter<T> {
    /// The wrapped value to be adapted into an attribute.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) inner: T,
}

/// Adapts an event with a specific event name into an `AttributeValue`.
///
/// This type wraps an event value and its event name, enabling
/// `Into<AttributeValue>` trait implementation for events.
/// Used by the `html!` macro for event attributes like `onclick`.
#[derive(Data, Debug, New)]
pub struct EventNamedAdapter<T> {
    /// The wrapped event value to be adapted.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) inner: T,
    /// The event name (e.g., "click", "mouseover").
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) event_name: &'static str,
}

/// Adapts an arbitrary attribute value expression into an `AttributeValue`.
///
/// Handles the dispatch between event closures and reactive values without
/// requiring the macro to generate inline trait hierarchies. The macro emits
/// `AttrValueAdapter::new(expr).into_attribute_value()` instead of the
/// `__IsClosure` / `__ClosurePicker` / `__ValuePicker` / `__FallbackHelper`
/// / `__dispatch` boilerplate.
///
/// For event attributes (key starts with "on"), event closures are wrapped
/// into `AttributeValue::Event`. For non-event attributes, values are
/// converted via `IntoReactiveValue`.
#[derive(Data, Debug, New)]
pub struct AttrValueAdapter<T> {
    /// The wrapped value to be adapted into an attribute.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) inner: T,
}

/// Adapts an `inner_html:` payload into the matching `AttributeValue`
/// variant (`InnerHtml(String)` for static strings, `InnerHtmlSignal`
/// for `Signal<String>`).
///
/// This is a sibling to [`AttrValueAdapter`] specialised for the
/// `inner_html:` attribute key. The html! macro emits
/// `InnerHtmlAdapter::new(expr).into()` whenever it sees an
/// `inner_html: ...` binding, so that `inner_html: "raw"` and
/// `inner_html: my_signal` route through `set_inner_html` rather than
/// the generic `set_attribute_or_property` path used for ordinary
/// `Text` attributes.
///
/// The actual `String` ↔ `Signal<String>` dispatch happens in the
/// `From<InnerHtmlAdapter<T>> for AttributeValue` impl below, where
/// the trait bounds on `T` decide which variant is produced.
#[derive(Data, Debug, New)]
pub struct InnerHtmlAdapter<T> {
    /// The wrapped value to be adapted into an `AttributeValue` for
    /// the `inner_html:` attribute.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) inner: T,
}

/// Adapts a callback with a custom name into an `AttributeValue`.
///
/// This type wraps a callback and its custom attribute name, enabling
/// `Into<AttributeValue>` trait implementation for named callbacks.
/// Used by the `html!` macro for component callback props.
#[derive(Data, Debug, New)]
pub struct CallbackNamedAdapter<T> {
    /// The wrapped callback to be adapted.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) inner: T,
    /// The custom attribute name (e.g., "on-increment", "on-change").
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) name: &'static str,
}

/// A `Sync` wrapper for single-threaded global `HashSet` access.
///
/// SAFETY: This type is only safe to use in single-threaded contexts
/// (e.g., WASM). It implements `Sync` to allow usage as a `static`
/// variable, but concurrent access from multiple threads would be
/// undefined behavior.
#[derive(Data, Debug, New)]
pub(crate) struct InjectedClassesCell(
    /// Interior-mutable storage for the set of CSS class names already
    /// injected into the DOM.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) UnsafeCell<HashSet<String>>,
);
