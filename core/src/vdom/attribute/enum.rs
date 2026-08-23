use super::*;

/// Represents the value of an HTML attribute.
///
/// Attributes can be static text, reactive signals, event handlers, dynamic expressions,
/// CSS class references, or raw HTML fragments assigned via `inner_html:`.
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
}
