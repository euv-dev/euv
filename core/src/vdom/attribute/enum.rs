use super::*;

/// Represents the value of an HTML attribute.
///
/// Attributes can be static text, reactive signals, event handlers, dynamic expressions,
/// CSS class references, or raw HTML fragments assigned via `inner_html:`.
#[derive(Clone, CustomDebug)]
pub enum AttributeValue {
    /// A static string value.
    Text(String),
    /// OPT 10: a `'static` string slice that bypasses the runtime allocation
    /// that `Text(String)` would require.
    ///
    /// Used by the `html!` and `class!` macros when every component of the
    /// value is a string literal (e.g. `style: { color: "red" }` or
    /// `class: "static-class-name"`). Renderer treats this exactly like
    /// `Text(value.to_string())` minus the heap allocation.
    StaticText(&'static str),
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
    /// OPT 11: a borrowed `'static` reference to a `Css` produced by the
    /// `class!` macro, avoiding the deep `Css::clone()` that the owned
    /// [`AttributeValue::Css`] variant performs.
    ///
    /// The reference points to the `OnceLock<Css>` instance that `class!`
    /// returns, so the lifetime is `'static` for the duration of the
    /// program. Renderers inject the style on first sight and then read
    /// the class name through the reference with zero copies.
    CssRef(&'static Css),
    /// A raw HTML fragment assigned via the `inner_html:` attribute.
    ///
    /// Replaces the element's children wholesale via
    /// [`web_sys::Element::set_inner_html`]. Unlike `Text` (which the
    /// browser escapes), this variant trusts the input string and runs
    /// any embedded `<script>` tags — it is the euv equivalent of
    /// React's `dangerouslySetInnerHTML`. Always document the XSS
    /// surface when exposing this attribute to user-supplied data.
    ///
    /// When both `inner_html:` and `class:` / other attributes are set
    /// on the same element, `inner_html` is applied last so it wins on
    /// children. Element children listed inside the same `html!` block
    /// are skipped (mirroring React's behaviour).
    InnerHtml(String),
    /// A reactive `inner_html:` payload that re-renders the element's
    /// children whenever the signal value changes.
    ///
    /// Same XSS semantics as [`AttributeValue::InnerHtml`] — the signal
    /// may carry any HTML, including executable `<script>` tags.
    #[debug(skip)]
    InnerHtmlSignal(Signal<String>),
    /// A reactive boolean attribute value (e.g. `checked`, `disabled`)
    /// driven directly by a `Signal<bool>`.
    ///
    /// The renderer writes the attribute as the string `"true"` / `"false"`
    /// and subscribes the source signal to the element directly — no
    /// intermediate `Signal<String>` mapping signal is allocated (the
    /// previous `bool_to_attr` bridge), and no subscription is created per
    /// re-render.
    #[debug(skip)]
    BoolSignal(Signal<bool>),
    /// A reactive handle to the element being created, populated by the
    /// renderer after the corresponding `ref:` attribute fires.
    ///
    /// The renderer does **not** write a `ref="..."` attribute into the
    /// DOM — it intercepts this variant, calls [`NodeRefDyn::set`] with
    /// the freshly-created element, then `clear()`s it on unmount.
    #[debug(skip)]
    Ref(NodeRefDyn),
}
