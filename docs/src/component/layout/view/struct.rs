use super::*;

/// Props of the docs shell components.
#[derive(Clone, Default)]
pub(crate) struct DocsShellProps {
    /// The current route signal.
    pub(crate) route_signal: Signal<String>,
    /// The current theme name signal.
    pub(crate) theme_signal: Signal<String>,
    /// The theme root class signal (`c_app_root c_theme_light`, …).
    pub(crate) root_class_signal: Signal<String>,
    /// Mobile drawer open state.
    pub(crate) drawer_open: Signal<bool>,
    /// Locale dropdown open state.
    pub(crate) locale_menu_open: Signal<bool>,
    /// Collapsed sidebar group keys.
    pub(crate) collapsed: Signal<Vec<String>>,
}
