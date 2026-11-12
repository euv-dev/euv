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
    /// The stable ID paired with the controlled content panel.
    #[get(type(copy))]
    pub id: &'static str,
    /// The trigger's position in the menubar, used for arrow-key navigation.
    #[get(type(copy))]
    pub index: usize,
    /// Whether the menu trigger is unavailable.
    #[get(type(copy))]
    pub disabled: bool,
}

/// Props of the [`euv_menubar_content`] component.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvMenubarContentProps {
    /// The open state of this menu.
    #[get(type(copy))]
    pub open: Signal<bool>,
    /// The ID of the trigger that labels and controls this panel.
    #[get(type(copy))]
    pub trigger_id: &'static str,
    /// The optional accessible label when no trigger ID is supplied.
    #[get(type(copy))]
    pub label: &'static str,
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
    /// Whether the item is unavailable.
    #[get(type(copy))]
    pub disabled: bool,
    /// Optional select callback receiving the chosen value.
    #[debug(skip)]
    pub on_select: Option<Rc<dyn Fn(&'static str)>>,
}

/// Props of the [`euv_menubar_group`] component.
///
/// The group names a set of rows that belong together.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvMenubarGroupProps {
    /// The accessible name of the group.
    #[get(type(copy))]
    pub label: &'static str,
}

/// Props of the [`euv_menubar_separator`] component.
///
/// The separator is a plain rule; it carries no settings of its own.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvMenubarSeparatorProps {
    /// The accessible name of the separator, if any.
    #[get(type(copy))]
    pub label: &'static str,
}

/// Props of the [`euv_menubar_label`] component.
///
/// The label is the heading of a group of rows.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvMenubarLabelProps {
    /// The text of the heading.
    #[get(type(copy))]
    pub text: &'static str,
    /// Whether the heading is indented, so it lines up with the rows
    /// that carry an indicator.
    #[get(type(copy))]
    pub inset: bool,
}

/// Props of the [`euv_menubar_shortcut`] component.
///
/// The shortcut shows the keyboard hint at the end of a row.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvMenubarShortcutProps {
    /// The text of the hint.
    #[get(type(copy))]
    pub text: &'static str,
}

/// Props of the [`euv_menubar_checkbox_item`] component.
///
/// The row carries a box the reader ticks.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvMenubarCheckboxItemProps {
    /// The label shown inside the row.
    #[get(type(copy))]
    pub label: &'static str,
    /// Whether the box is ticked.
    #[get(type(copy))]
    pub checked: Signal<bool>,
    /// Whether the row is unavailable.
    #[get(type(copy))]
    pub disabled: bool,
    /// Optional callback fired when the row is chosen; it receives the
    /// new ticked state.
    #[debug(skip)]
    pub on_select: Option<Rc<dyn Fn(bool)>>,
}

/// Props of the [`euv_menubar_radio_item`] component.
///
/// The row carries a dot the reader moves.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvMenubarRadioItemProps {
    /// The label shown inside the row.
    #[get(type(copy))]
    pub label: &'static str,
    /// The value the row stands for.
    #[get(type(copy))]
    pub value: &'static str,
    /// Whether the dot is on this row.
    #[get(type(copy))]
    pub checked: Signal<bool>,
    /// Whether the row is unavailable.
    #[get(type(copy))]
    pub disabled: bool,
    /// Optional callback fired when the row is chosen; it receives the
    /// value of the row.
    #[debug(skip)]
    pub on_select: Option<Rc<dyn Fn(&'static str)>>,
}

/// Props of the [`euv_menubar_radio_group`] component.
///
/// The group holds the radio rows that share one choice.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvMenubarRadioGroupProps {
    /// The accessible name of the group.
    pub label: &'static str,
    /// The value of the chosen row.
    pub value: Signal<String>,
}

/// Props of the [`euv_menubar_sub`] component.
///
/// The sub menu is the container of a nested menu.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvMenubarSubProps {
    /// The accessible name of the nested menu.
    #[get(type(copy))]
    pub label: &'static str,
}

/// Props of the [`euv_menubar_sub_trigger`] component.
///
/// The trigger opens the nested menu.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvMenubarSubTriggerProps {
    /// The label shown inside the trigger.
    #[get(type(copy))]
    pub label: &'static str,
    /// The stable ID paired with the nested content panel.
    #[get(type(copy))]
    pub id: &'static str,
    /// The open state of the nested menu.
    #[get(type(copy))]
    pub open: Signal<bool>,
    /// Whether the nested menu is unavailable.
    #[get(type(copy))]
    pub disabled: bool,
    /// Whether the trigger is indented, so it lines up with the rows
    /// that carry an indicator.
    #[get(type(copy))]
    pub inset: bool,
}

/// Props of the [`euv_menubar_sub_content`] component.
///
/// The content holds the rows of the nested menu.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvMenubarSubContentProps {
    /// The open state of the nested menu.
    #[get(type(copy))]
    pub open: Signal<bool>,
    /// The ID of the trigger that labels and controls this panel.
    #[get(type(copy))]
    pub trigger_id: &'static str,
    /// The optional accessible label of the panel.
    #[get(type(copy))]
    pub label: &'static str,
}
