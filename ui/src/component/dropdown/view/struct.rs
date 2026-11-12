use super::*;

/// One entry of the [`euv_dropdown`] menu.
///
/// Carries the display label and the opaque value handed to `on_select`.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvDropdownItem {
    /// The display label.
    #[get(type(copy))]
    pub label: &'static str,
    /// The opaque value passed to the select callback.
    #[get(type(copy))]
    pub value: &'static str,
    /// Whether this item is the currently selected value.
    ///
    /// Drives the `_active` class: the accent fill and the `text-on-accent`
    /// label colour, matching how `euv_sidebar` marks the current page.
    /// Defaults to `false`, so existing call sites keep the neutral look.
    #[get(type(copy))]
    pub active: bool,
}

/// Props for the [`euv_dropdown`] component.
///
/// The open state is owned by the caller so the trigger (passed as children)
/// can toggle it; selecting an item closes the menu.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvDropdownProps {
    /// The open state signal toggled by the trigger children.
    pub open: Signal<bool>,
    /// The menu items.
    pub items: Vec<EuvDropdownItem>,
    /// Optional select callback receiving the chosen item value.
    #[debug(skip)]
    pub on_select: Option<Rc<dyn Fn(&'static str)>>,
}

/// Props of the [`euv_dropdown_trigger`] component.
///
/// The trigger carries the open state of the menu it opens, so it can
/// report that state to a reader.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvDropdownTriggerProps {
    /// Whether the menu is open.
    pub open: Signal<bool>,
}

/// Props of the [`euv_dropdown_menu_group`] component.
///
/// The group holds the rows that belong to one heading.
#[derive(Clone, Copy, CustomDebug, Default)]
pub struct EuvDropdownMenuGroupProps {
    /// The accessible name of the group.
    pub label: &'static str,
}

/// Props of the [`euv_dropdown_menu_checkbox_item`] component.
///
/// The row carries a box the reader ticks.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvDropdownMenuCheckboxItemProps {
    /// The label shown inside the row.
    pub label: &'static str,
    /// Whether the box is ticked.
    pub checked: Signal<bool>,
}

/// Props of the [`euv_dropdown_menu_radio_group`] component.
///
/// The group holds the rows that share one radio selection.
#[derive(Clone, Copy, CustomDebug, Default)]
pub struct EuvDropdownMenuRadioGroupProps {
    /// The accessible name of the group.
    pub label: &'static str,
}

/// Props of the [`euv_dropdown_menu_radio_item`] component.
///
/// The row carries a dot the reader moves.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvDropdownMenuRadioItemProps {
    /// The label shown inside the row.
    pub label: &'static str,
    /// Whether the dot is the selected one.
    pub checked: Signal<bool>,
}

/// Props of the [`euv_dropdown_menu_sub`] component.
///
/// The sub-menu holds the rows of a nested menu.
#[derive(Clone, Copy, CustomDebug, Default)]
pub struct EuvDropdownMenuSubProps {
    /// The label of the row that opens the sub-menu.
    pub label: &'static str,
    /// Whether the sub-menu is open.
    pub open: Signal<bool>,
}

/// Props of the [`euv_dropdown_menu_sub_trigger`] component.
#[derive(Clone, Copy, CustomDebug, Default)]
pub struct EuvDropdownMenuSubTriggerProps {
    /// The label shown inside the row.
    pub label: &'static str,
    /// Whether the sub-menu is open.
    pub open: Signal<bool>,
}

/// Props of the [`euv_dropdown_menu_sub_content`] component.
#[derive(Clone, Copy, CustomDebug, Default)]
pub struct EuvDropdownMenuSubContentProps {
    /// Whether the sub-menu is open.
    pub open: Signal<bool>,
}

/// Props of the [`euv_dropdown_menu_item`] component.
///
/// A menu item is the leaf of a menu: it carries the label the reader
/// sees, the key hint shown at its trailing edge, and the value handed
/// back when it is chosen.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvDropdownMenuItemProps {
    /// The label shown inside the item.
    #[get(type(copy))]
    pub label: &'static str,
    /// The key hint shown at the trailing edge of the item.
    #[get(type(copy))]
    pub shortcut: &'static str,
    /// The opaque value handed to the select callback.
    #[get(type(copy))]
    pub value: &'static str,
    /// Whether the item is disabled.
    #[get(type(copy))]
    pub disabled: bool,
    /// Optional select callback receiving the chosen value.
    #[debug(skip)]
    pub on_select: Option<Rc<dyn Fn(&'static str)>>,
}

/// Props of the [`euv_dropdown_menu_label`] component.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvDropdownMenuLabelProps {
    /// The text of the group heading.
    #[get(type(copy))]
    pub text: &'static str,
}

/// Props of the [`euv_dropdown_menu_separator`] component.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvDropdownMenuSeparatorProps {
    /// The label reported to assistive technology, if any.
    #[get(type(copy))]
    pub label: &'static str,
}

/// Props of the [`euv_dropdown_menu_shortcut`] component.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvDropdownMenuShortcutProps {
    /// The key hint text.
    #[get(type(copy))]
    pub text: &'static str,
}
