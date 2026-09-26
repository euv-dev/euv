use super::*;

/// Implements `Display` for `KeepAliveTab` to provide human-readable tab labels.
impl Display for KeepAliveTab {
    /// Formats the tab variant as its display label string.
    ///
    /// # Arguments
    ///
    /// - `f` - The formatter to write into.
    ///
    /// # Returns
    ///
    /// - `Result` - Whether the formatting succeeded.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let display_text: &str = match self {
            KeepAliveTab::Counter => "Counter",
            KeepAliveTab::Form => "Form",
            KeepAliveTab::Timer => "Timer",
        };
        f.write_str(display_text)
    }
}
