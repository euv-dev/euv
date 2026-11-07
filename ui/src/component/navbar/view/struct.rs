use super::*;

/// One link entry of the [`euv_navbar`] top navigation bar.
///
/// Internal links (hash routes) highlight when their path prefix matches the
/// current route; links starting with `http` open in a new tab.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvNavbarItem {
    /// The display text.
    #[get(type(copy))]
    pub text: &'static str,
    /// The link target (hash route path or external URL).
    #[get(type(copy))]
    pub link: &'static str,
}

/// Props for the [`euv_navbar`] component.
///
/// The trailing actions area (theme toggle, language menu, …) is supplied as
/// children so callers compose arbitrary controls.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvNavbarProps {
    /// The current route signal (drives the active link highlight).
    pub route_signal: Signal<String>,
    /// The brand logo text rendered inside the accent square.
    pub brand_logo: &'static str,
    /// The brand title rendered next to the logo.
    pub brand_title: &'static str,
    /// The brand home route.
    pub brand_href: &'static str,
    /// The navigation links.
    pub items: &'static [EuvNavbarItem],
    /// The mobile drawer signal; when `Some` a hamburger button is rendered
    /// on small viewports and toggles it.
    pub drawer_open: Option<Signal<bool>>,
}

/// Props of the [`euv_navbar_link`] component.
#[derive(Clone, Copy, CustomDebug, Default)]
pub struct EuvNavbarLinkProps {
    /// The current route signal.
    pub route_signal: Signal<String>,
    /// The navbar item to render.
    pub item: EuvNavbarItem,
}

/// Props of the [`euv_menubar`] component.
///
/// The bar holds the menus; the open state of each menu belongs to the
/// caller, so a menu can be closed from the outside.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvMenubarProps {
    /// The accessible name of the bar.
    #[get(type(copy))]
    pub label: &'static str,
}

/// Props of the [`euv_menubar_menu`] component.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvMenubarMenuProps {
    /// The open state of this menu.
    #[get(type(copy))]
    pub open: Signal<bool>,
}

/// Props of the [`euv_menubar_trigger`] component.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvMenubarTriggerProps {
    /// The label shown inside the trigger.
    #[get(type(copy))]
    pub label: &'static str,
    /// The open state of the menu this trigger belongs to.
    #[get(type(copy))]
    pub open: Signal<bool>,
}

/// Props of the [`euv_menubar_content`] component.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvMenubarContentProps {
    /// The open state of this menu.
    #[get(type(copy))]
    pub open: Signal<bool>,
}

/// Props of the [`euv_menubar_item`] component.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvMenubarItemProps {
    /// The label shown inside the row.
    #[get(type(copy))]
    pub label: &'static str,
    /// The value handed to the select callback.
    #[get(type(copy))]
    pub value: &'static str,
    /// Optional select callback receiving the chosen value.
    #[debug(skip)]
    pub on_select: Option<Rc<dyn Fn(&'static str)>>,
}
