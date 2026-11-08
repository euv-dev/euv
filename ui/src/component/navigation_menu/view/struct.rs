use super::*;

/// Props for the [`euv_navigation_menu`] component.
///
/// The menu carries the state of the open section, so the trigger and
/// the content inside it read the same state.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvNavigationMenuProps {
    /// Whether a section of the menu is open.
    pub open: Signal<bool>,
}

/// Props for the [`euv_navigation_menu_list`] component.
///
/// The list takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvNavigationMenuListProps {}

/// Props for the [`euv_navigation_menu_item`] component.
///
/// The item takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvNavigationMenuItemProps {}

/// Props for the [`euv_navigation_menu_trigger`] component.
///
/// The trigger carries the state of the section it opens.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvNavigationMenuTriggerProps {
    /// Whether the section is open.
    pub open: Signal<bool>,
}

/// Props for the [`euv_navigation_menu_content`] component.
///
/// The content carries the state of the section it belongs to.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvNavigationMenuContentProps {
    /// Whether the section is open.
    pub open: Signal<bool>,
}

/// Props for the [`euv_navigation_menu_link`] component.
///
/// The link takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvNavigationMenuLinkProps {}

/// Props for the [`euv_navigation_menu_viewport`] component.
///
/// The viewport takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvNavigationMenuViewportProps {}

/// Props for the [`euv_navigation_menu_indicator`] component.
///
/// The indicator takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvNavigationMenuIndicatorProps {}
