use super::*;

/// Props for the `desktop_layout` component.
///
/// Defines the strongly-typed interface for the desktop application shell layout.
#[derive(Clone, Debug, Default)]
pub(crate) struct DesktopLayoutProps {
    /// The reactive signal holding the current route.
    pub(crate) route_signal: Signal<String>,
    /// The reactive signal holding the current theme (kept for API consistency).
    pub(crate) theme_signal: Signal<String>,
    /// The reactive signal holding the root class name.
    pub(crate) root_class_signal: Signal<String>,
    /// The reactive signal controlling vconsole panel visibility.
    pub(crate) panel_open: Signal<bool>,
}

/// Props for the `mobile_layout` component.
///
/// Defines the strongly-typed interface for the mobile application shell layout.
#[derive(Clone, Debug, Default)]
pub(crate) struct MobileLayoutProps {
    /// The reactive signal holding the current route.
    pub(crate) route_signal: Signal<String>,
    /// The reactive signal holding the current theme.
    pub(crate) theme_signal: Signal<String>,
    /// The reactive signal holding the root class name.
    pub(crate) root_class_signal: Signal<String>,
    /// The reactive signal controlling vconsole panel visibility.
    pub(crate) panel_open: Signal<bool>,
    /// The reactive signal controlling the mobile nav drawer visibility.
    pub(crate) drawer_open: Signal<bool>,
}

/// The JSON response from docs.rs crate status endpoint.
///
/// Example JSON: `{"doc_status": true, "version": "0.6.23"}`
#[derive(Clone, Data, Default, Deserialize, New)]
pub(crate) struct DocsStatus {
    /// Whether the documentation build succeeded.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) doc_status: bool,
    /// The latest version string of the crate.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) version: String,
}
