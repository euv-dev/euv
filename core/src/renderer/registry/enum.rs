use super::*;

/// Per-attribute source-signal bridge entry used by
/// `core/src/renderer/registry/impl.rs::register_attribute_bridge`.
///
/// Each variant carries the DOM target the mount path captured when
/// the attribute signal was installed, plus the static attribute name
/// (`SetAttribute` only — the other variants address whole subtrees).
///
/// Mutation API:
/// - `SetAttribute` — write `attr_name = value` via
///   `Element::set_attribute_or_property`.
/// - `SetInnerHtml` — replace `innerHTML` via
///   `Element::set_inner_html`.
/// - `SetTextContent` — replace the text data on a `Text` node.
pub(crate) enum AttributeBridge {
    /// Writes `attr_name = value` via `Element::set_attribute_or_property`.
    SetAttribute {
        /// The DOM element to mutate on every source-signal set.
        elem: Element,
        /// The attribute name (compile-time static — never allocates).
        attr_name: &'static str,
    },
    /// Replaces `innerHTML` via `Element::set_inner_html`.
    SetInnerHtml {
        /// The DOM element whose `innerHTML` is replaced on every
        /// source-signal set.
        elem: Element,
    },
    /// Replaces text content via `Text::set_text_content`.
    SetTextContent {
        /// The DOM `Text` node whose data is replaced on every
        /// source-signal set.
        text: Text,
    },
}
