use super::*;

/// A raw HTML string that the renderer must insert
/// without escaping.
///
/// Constructed via `RawHtml::new` (from Lombok `New`) or,
/// preferably, the `unsafe_no_inline!` macro which makes
/// the security warning loud at the call site.
///
/// The `content` field is `pub(crate)` with a public
/// getter (`get_content`), a crate-internal mut-getter,
/// and a crate-internal setter. The public getter lets
/// the renderer read the unescaped content; the
/// setter / mut-getter are kept crate-internal so the
/// content is set at construction (preferably through
/// `unsafe_no_inline!`) and not mutated after.
#[derive(Clone, Data, Debug, Default, Eq, Hash, New, PartialEq)]
pub struct RawHtml {
    /// The unescaped HTML content.
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) content: String,
}
