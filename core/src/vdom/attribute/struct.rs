use super::*;

/// Represents a single attribute on a virtual DOM node.
///
/// Combines an attribute name with its corresponding value.
///
/// OPT 2: attribute names are `Cow<'static, str>`. The `html!` macro
/// emits `Cow::Borrowed("class")` for the common literal case so
/// attribute keys share a single static slice across the whole rendered
/// DOM, removing the per-attribute `String` heap allocation. The
/// `AttributeValue::Text(String)` payload keeps its owned allocation
/// (text values are user-supplied strings that almost always come
/// from string interpolation or runtime formatting).
#[derive(Clone, CustomDebug, Data, New)]
pub struct AttributeEntry {
    /// The name of the attribute.
    ///
    /// `get_name` is public: `ui/tests/view_factories` collects attribute
    /// names from outside this crate. `get_mut` / `set` stay crate-internal —
    /// an attribute entry is built once by `New` and never rewritten.
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub name: Cow<'static, str>,
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
    ///
    /// Public: the `class!` macro expands `get_name()` into consumer
    /// crates (`ui`, `example`), and the component views in `ui` read it
    /// directly, so the getter cannot be crate-private. The field is
    /// public to match that getter — a public getter over a restricted
    /// field is the leak `public-accessor-leaks-restricted-field`
    /// reports. Mutation stays crate-internal.
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub name: String,
    /// The CSS style declarations (e.g., "max-width: 800px; margin: 0 auto;").
    ///
    /// Public for the same reason as [`Css::name`]: the `class!` macro
    /// expands `get_style()` into consumer crates when a class extends
    /// another class. Mutation stays crate-internal.
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub style: String,
    /// The pseudo-class and pseudo-element rules for this class
    /// (e.g., ":hover", ":focus", ":active", "::before", etc.).
    ///
    /// Public for the same reason as [`Css::name`]: the `class!` macro
    /// expands `get_pseudo_rules()` into consumer crates when a class
    /// extends another class. Mutation stays crate-internal.
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub pseudo_rules: Vec<PseudoRule>,
    /// The media query rules for this class.
    ///
    /// Public for the same reason as [`Css::name`]: the `class!` macro
    /// expands `get_media_rules()` into consumer crates when a class
    /// extends another class. Mutation stays crate-internal.
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub media_rules: Vec<MediaRule>,
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
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub inner: T,
}

/// Adapts an event with a specific event name into an `AttributeValue`.
///
/// This type wraps an event value and its event name, enabling
/// `Into<AttributeValue>` trait implementation for events.
/// Used by the `html!` macro for event attributes like `onclick`.
#[derive(Data, Debug, New)]
pub struct EventNamedAdapter<T> {
    /// The wrapped event value to be adapted.
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub inner: T,
    /// The event name (e.g., "click", "mouseover").
    #[get(type(copy), pub(crate))]
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
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub inner: T,
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
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub inner: T,
}

/// Adapts a callback with a custom name into an `AttributeValue`.
///
/// This type wraps a callback and its custom attribute name, enabling
/// `Into<AttributeValue>` trait implementation for named callbacks.
/// Used by the `html!` macro for component callback props.
#[derive(Data, Debug, New)]
pub struct CallbackNamedAdapter<T> {
    /// The wrapped callback to be adapted.
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub inner: T,
    /// The custom attribute name (e.g., "on-increment", "on-change").
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub name: &'static str,
}

/// Namespace for the injected-class registry accessors.
///
/// The registry itself lives in `INJECTED_CLASSES` (an `RwLock<HashSet>`
/// behind a `LazyLock`). This type only carries the associated functions so
/// the accessors keep a `Type::method()` shape.
///
/// It previously wrapped a `UnsafeCell<HashSet<String>>` and was marked
/// `unsafe impl Sync` so it could sit in a `static mut`. That was undefined
/// behaviour as soon as two threads touched it, which is exactly what
/// parallel test runs do.
#[derive(Debug, Default)]
pub(crate) struct InjectedClassesCell;
