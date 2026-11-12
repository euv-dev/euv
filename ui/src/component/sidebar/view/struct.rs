use super::*;

/// One node of the [`euv_sidebar`] navigation tree.
///
/// A node with empty `children` is a leaf link; a node with children is a
/// collapsible group whose `link` (when set) navigates to the group index
/// page without toggling the group.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvSidebarItem {
    /// The display text.
    #[get(type(copy))]
    pub text: &'static str,
    /// The leaf route; groups may link to their index page.
    #[get(type(copy))]
    pub link: Option<&'static str>,
    /// Nested children (non-empty renders a collapsible group).
    #[get(type(copy))]
    pub children: &'static [EuvSidebarItem],
}

/// Props for the [`euv_sidebar`] component.
///
/// The `collapsed` signal is caller-owned so several sidebars (desktop aside
/// and mobile drawer) can share the collapse state; `prefix` is internal and
/// used by the recursion to key groups.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvSidebarProps {
    /// The current route signal (drives the active link highlight).
    pub route_signal: Signal<String>,
    /// The collapsed group keys.
    pub collapsed: Signal<Vec<String>>,
    /// The tree items.
    pub items: &'static [EuvSidebarItem],
    /// The group key prefix (internal recursion state, leave default).
    pub prefix: String,
    /// Optional navigation interceptor receiving the target link; when set it
    /// replaces the default `Router::navigate` (used by drawers to close via
    /// the overlay stack).
    #[debug(skip)]
    pub on_navigate: Option<Rc<dyn Fn(&'static str)>>,
}

/// Props of the [`euv_sidebar_provider`] component.
///
/// The provider holds the state the sidebar and its groups share, so a
/// caller can fold the tree without reaching into the sidebar itself.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvSidebarProviderProps {
    /// The collapsed groups.
    pub collapsed: Signal<Vec<String>>,
}

/// Props of the [`euv_sidebar_item`] component.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvSidebarItemProps {
    /// The current route signal.
    pub route_signal: Signal<String>,
    /// The collapsed group keys.
    pub collapsed: Signal<Vec<String>>,
    /// The item to render.
    pub item: EuvSidebarItem,
    /// The group key prefix of the parent.
    pub prefix: String,
    /// Optional navigation interceptor (see [`EuvSidebarProps::on_navigate`]).
    #[debug(skip)]
    pub on_navigate: Option<Rc<dyn Fn(&'static str)>>,
}

/// Props of the [`euv_sidebar_header`] component.
///
/// The header holds whatever the caller puts in it and takes no settings
/// of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvSidebarHeaderProps {}

/// Props of the [`euv_sidebar_content`] component.
///
/// The content is the scrollable middle of the sidebar and takes no
/// settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvSidebarContentProps {}

/// Props of the [`euv_sidebar_footer`] component.
///
/// The footer holds whatever the caller puts in it and takes no settings
/// of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvSidebarFooterProps {}

/// Props of the [`euv_sidebar_group`] component.
///
/// The group holds a caption and its rows and takes no settings of its
/// own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvSidebarGroupProps {}

/// Props of the [`euv_sidebar_group_label`] component.
///
/// The caption of a group holds its name and takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvSidebarGroupLabelProps {}

/// Props of the [`euv_sidebar_group_action`] component.
///
/// The control at the end of a caption takes no settings of its own: what
/// it does is the caller's business, and it reports through the click
/// handler of the button.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvSidebarGroupActionProps {}

/// Props of the [`euv_sidebar_group_content`] component.
///
/// The rows of a group take no settings of their own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvSidebarGroupContentProps {}

/// Props of the [`euv_sidebar_menu`] component.
///
/// The menu holds the rows and takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvSidebarMenuProps {}

/// Props of the [`euv_sidebar_menu_item`] component.
///
/// A row of the menu holds its button and takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvSidebarMenuItemProps {}

/// Props of the [`euv_sidebar_menu_button`] component.
///
/// The row of a menu item is the clickable part of it. The reference
/// renders a link when the row leads somewhere and a button when it only
/// acts, so the target route decides which of the two it is.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvSidebarMenuButtonProps {
    /// Whether the row is the one the reader is on.
    ///
    /// The state travels in the `data-active` attribute, so the styling of
    /// the row can be driven from the markup alone.
    pub is_active: bool,
    /// The route the row leads to.
    ///
    /// An empty route renders a button instead of a link.
    pub href: &'static str,
}

/// Props of the [`euv_sidebar_menu_action`] component.
///
/// The control at the end of a menu row takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvSidebarMenuActionProps {}

/// Props of the [`euv_sidebar_menu_badge`] component.
///
/// The count at the end of a menu row takes no settings of its own: the
/// count itself is what the caller puts in it.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvSidebarMenuBadgeProps {}

/// Props of the [`euv_sidebar_menu_sub`] component.
///
/// The nested list of rows takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvSidebarMenuSubProps {}

/// Props of the [`euv_sidebar_menu_sub_item`] component.
///
/// A row of a nested menu takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvSidebarMenuSubItemProps {}

/// Props of the [`euv_sidebar_menu_sub_button`] component.
///
/// A nested row behaves like a row of the menu itself: it carries the
/// active state and leads to a route.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvSidebarMenuSubButtonProps {
    /// Whether the row is the one the reader is on.
    pub is_active: bool,
    /// The route the row leads to.
    ///
    /// An empty route renders a button instead of a link.
    pub href: &'static str,
}

/// Props of the [`euv_sidebar_menu_skeleton`] component.
///
/// The placeholder that stands in for a row while it loads can carry an
/// icon in front of its text, the way the reference does.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvSidebarMenuSkeletonProps {
    /// Whether the placeholder carries an icon.
    pub show_icon: bool,
}

/// Props of the [`euv_sidebar_input`] component.
///
/// The search field of the sidebar carries the placeholder that tells the
/// reader what to search for.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvSidebarInputProps {
    /// The placeholder text of the field.
    pub placeholder: &'static str,
}

/// Props of the [`euv_sidebar_inset`] component.
///
/// The main area beside the sidebar takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvSidebarInsetProps {}

/// Props of the [`euv_sidebar_rail`] component.
///
/// The edge that opens and closes the sidebar takes no settings of its
/// own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvSidebarRailProps {}

/// Props of the [`euv_sidebar_separator`] component.
///
/// The rule that divides one band of the sidebar from the next takes no
/// settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvSidebarSeparatorProps {}

/// Props of the [`euv_sidebar_trigger`] component.
///
/// The button that opens and closes the sidebar reports the state it
/// leaves the sidebar in, so the reader can tell an open sidebar from a
/// closed one.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvSidebarTriggerProps {
    /// Whether the sidebar is open.
    pub expanded: bool,
}
