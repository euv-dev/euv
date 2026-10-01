use super::*;

/// Provides static label strings for `DynamicTagType` button display.
impl DynamicTagType {
    /// Returns the static display label for the tag type variant.
    ///
    /// # Returns
    ///
    /// - `&'static str` - The display label string.
    pub(crate) fn label(self) -> &'static str {
        match self {
            DynamicTagType::Div => "div",
            DynamicTagType::Span => DYNAMIC_TAG_SPAN,
            DynamicTagType::EuvCard => "euv card",
            DynamicTagType::Badge => DYNAMIC_TAG_BADGE,
        }
    }
}

/// Implements `Display` for `DynamicTagType` to provide the tag name string used
/// by the html! macro's dynamic tag syntax.
impl Display for DynamicTagType {
    /// Formats the tag type variant as its tag name string.
    ///
    /// # Arguments
    ///
    /// - `&mut Formatter<'_>` - The formatter to write into.
    ///
    /// # Returns
    ///
    /// - `fmt::Result` - Whether the formatting succeeded.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let tag_name: &str = match self {
            DynamicTagType::Div => "div",
            DynamicTagType::Span => DYNAMIC_TAG_SPAN,
            DynamicTagType::EuvCard => "euv_card",
            DynamicTagType::Badge => DYNAMIC_TAG_BADGE,
        };
        f.write_str(tag_name)
    }
}
