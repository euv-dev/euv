/// The dark theme name string.
pub(crate) const THEME_DARK: &str = "dark";

/// The light theme name string.
pub(crate) const THEME_LIGHT: &str = "light";

/// The media query matching users whose OS requests a dark color scheme.
pub(crate) const THEME_DARK_SCHEME_MEDIA_QUERY: &str = "(prefers-color-scheme: dark)";

/// The event type emitted by `MediaQueryList` when the scheme changes.
pub(crate) const THEME_MEDIA_QUERY_CHANGE_EVENT: &str = "change";
