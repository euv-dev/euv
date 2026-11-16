use super::*;

/// Immutable configuration captured through a scope-owned, copyable signal handle.
#[derive(Clone, Getter)]
pub(crate) struct LayoutSettings {
    /// Shared immutable shell configuration.
    #[get(pub(crate))]
    pub(crate) props: Rc<EuvLayoutProps>,
}

/// Caller-supplied class names. Every field defaults to an empty string.
#[derive(Clone, Debug, Default)]
pub struct EuvLayoutClasses {
    /// Class names for the `root_extra` region or state.
    pub root_extra: String,
    /// Class names for the `header` region or state.
    pub header: String,
    /// Class names for the `header_left` region or state.
    pub header_left: String,
    /// Class names for the `brand` region or state.
    pub brand: String,
    /// Class names for the `header_navigation` region or state.
    pub header_navigation: String,
    /// Class names for the `header_actions` region or state.
    pub header_actions: String,
    /// Class names for the `body` region or state.
    pub body: String,
    /// Class names for the `sidebar` region or state.
    pub sidebar: String,
    /// Class names for the `sidebar_navigation` region or state.
    pub sidebar_navigation: String,
    /// Class names for the `sidebar_footer` region or state.
    pub sidebar_footer: String,
    /// Class names for the `main` region or state.
    pub main: String,
    /// Class names for the `menu_button` region or state.
    pub menu_button: String,
    /// Class names for the `menu_button_active` region or state.
    pub menu_button_active: String,
    /// Class names for the `overlay` region or state.
    pub overlay: String,
    /// Class names for the `overlay_hidden` region or state.
    pub overlay_hidden: String,
    /// Class names for the `drawer` region or state.
    pub drawer: String,
    /// Class names for the `drawer_closed` region or state.
    pub drawer_closed: String,
    /// Class names for the `drawer_header` region or state.
    pub drawer_header: String,
    /// Class names for the `drawer_close` region or state.
    pub drawer_close: String,
}

/// Optional content factories; absent slots render no content.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvLayoutSlots {
    /// Content for the `brand` region.
    #[debug(skip)]
    pub brand: Option<EuvLayoutSlot>,
    /// Content for the `header_navigation` region.
    #[debug(skip)]
    pub header_navigation: Option<EuvLayoutSlot>,
    /// Content for the `selector` region.
    #[debug(skip)]
    pub selector: Option<EuvLayoutSlot>,
    /// Content for the `theme_control` region.
    #[debug(skip)]
    pub theme_control: Option<EuvLayoutSlot>,
    /// Content for the `navigation` region.
    #[debug(skip)]
    pub navigation: Option<EuvLayoutSlot>,
    /// Content for the `navigation_footer` region.
    #[debug(skip)]
    pub navigation_footer: Option<EuvLayoutSlot>,
    /// Content for the `content` region.
    #[debug(skip)]
    pub content: Option<EuvLayoutSlot>,
    /// Content for the `extra` region.
    #[debug(skip)]
    pub extra: Option<EuvLayoutSlot>,
}

/// Controlled, unstyled application shell. Hosts retain resize and overlay-history setup.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvLayoutProps {
    /// Existing breakpoint signal: true selects mobile.
    pub mobile: Signal<bool>,
    /// Existing host-owned drawer state.
    pub drawer_open: Signal<bool>,
    /// Reactive root class names (for example host-owned theme classes).
    pub root_class: Signal<String>,
    /// Static region and state class names; no styles are generated.
    pub classes: EuvLayoutClasses,
    /// Content factories shared by both responsive modes.
    pub slots: EuvLayoutSlots,
    /// Localized accessible name for the menu control and navigation drawer.
    pub menu_label: &'static str,
    /// Localized accessible name for the close control.
    pub close_label: &'static str,
}
