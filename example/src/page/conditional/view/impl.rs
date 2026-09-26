use super::*;

/// Provides static label strings for `ConditionalUserType` button display.
impl ConditionalUserType {
    /// Returns the static display label for the user type variant.
    ///
    /// # Returns
    ///
    /// - `&'static str` - The display label string.
    pub(crate) fn label(self) -> &'static str {
        match self {
            ConditionalUserType::Guest => "Guest",
            ConditionalUserType::User => "User",
            ConditionalUserType::Admin => "Admin",
        }
    }
}

/// Implements `Display` for `ConditionalTab` to provide human-readable tab labels.
impl Display for ConditionalTab {
    /// Formats the tab variant as its display label string.
    ///
    /// # Arguments
    ///
    /// - `&mut Formatter<'_>` - The formatter to write the display string into.
    ///
    /// # Returns
    ///
    /// - `Result` - Whether the formatting succeeded.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let display_text: &str = match self {
            ConditionalTab::Info => "Info",
            ConditionalTab::Settings => "Settings",
            ConditionalTab::About => "About",
        };
        f.write_str(display_text)
    }
}

/// Implements `Display` for `ConditionalUserType` to provide human-readable role labels.
impl Display for ConditionalUserType {
    /// Formats the user type variant as its display label string.
    ///
    /// # Arguments
    ///
    /// - `&mut Formatter<'_>` - The formatter to write the display string into.
    ///
    /// # Returns
    ///
    /// - `Result` - Whether the formatting succeeded.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let display_text: &str = match self {
            ConditionalUserType::Guest => "Guest",
            ConditionalUserType::User => "User",
            ConditionalUserType::Admin => "Admin",
        };
        f.write_str(display_text)
    }
}
