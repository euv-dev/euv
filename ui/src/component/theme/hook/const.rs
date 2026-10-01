/// The light theme name string.
pub const THEME_LIGHT: &str = "light";

/// The dark theme name string.
pub const THEME_DARK: &str = "dark";

/// The media query matching users whose OS requests a dark color scheme.
pub const THEME_DARK_SCHEME_MEDIA_QUERY: &str = "(prefers-color-scheme: dark)";

/// The event type emitted by `MediaQueryList` when the scheme changes.
pub const THEME_MEDIA_QUERY_CHANGE_EVENT: &str = "change";
