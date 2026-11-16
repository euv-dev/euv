use super::*;

/// Geometry supplied by the selected menu class, independent of the DOM.
#[cfg(any(target_arch = "wasm32", test))]
#[derive(Clone, Copy, CustomDebug, Data)]
pub(crate) struct DropdownGeometryPolicy {
    /// Align the popup's leading edge rather than its trailing edge.
    #[get(pub(crate), type(copy))]
    pub(crate) align_start: bool,
    /// Use the trigger instead of the encompassing root for horizontal placement.
    #[get(pub(crate), type(copy))]
    pub(crate) anchor_trigger: bool,
    /// Size to the content instead of matching the horizontal anchor.
    #[get(pub(crate), type(copy))]
    pub(crate) content_width: bool,
    /// Distance from the trigger in viewport pixels.
    #[get(pub(crate), type(copy))]
    pub(crate) gap: f64,
}

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
    /// Whether this row is unavailable for focus and selection.
    #[get(type(copy))]
    pub disabled: bool,
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

/// Props for a composable dropdown root. Use the same signal for its trigger and content.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvDropdownMenuProps {
    /// Whether the owning menu is open.
    pub open: Signal<bool>,
    /// Optional owning surface state; closing it resets this menu and its descendants.
    /// This lifecycle parent is not a menu level and is never dismissed by menu actions.
    pub parent_open: Option<Signal<bool>>,
}

/// Props for the always-mounted composable dropdown popup.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvDropdownMenuContentProps {
    /// The open state shared with the root and trigger.
    pub open: Signal<bool>,
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
    /// Whether the row ignores focus and activation.
    pub disabled: bool,
    /// The owning menu's open state when close-on-click is requested.
    pub open: Signal<bool>,
    /// Whether activation closes the owning menu. Defaults to false.
    pub close_on_click: bool,
    /// Called once with the new checked value after activation.
    #[debug(skip)]
    pub on_checked_change: Option<Rc<dyn Fn(bool)>>,
}

/// Props of the [`euv_dropdown_menu_radio_group`] component.
///
/// The group holds the rows that share one radio selection.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvDropdownMenuRadioGroupProps {
    /// The accessible name of the group.
    pub label: &'static str,
    /// Shared selection handle, also passed explicitly to every radio item.
    pub selection: EuvDropdownMenuRadioGroupState,
}

/// Explicit shared selection created by [`use_dropdown_menu_radio_group`].
///
/// Clone this handle into the group and each item; no inherited context is used.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvDropdownMenuRadioGroupState {
    /// The single source of truth for this group's selected value.
    pub value: Signal<String>,
    /// Whether only the caller may update the value signal.
    pub controlled: bool,
    /// Requests a different value, after updating an uncontrolled signal.
    #[debug(skip)]
    pub on_value_change: Option<Rc<dyn Fn(String)>>,
}

/// Props of the [`euv_dropdown_menu_radio_item`] component.
///
/// The row carries a dot the reader moves.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvDropdownMenuRadioItemProps {
    /// The label shown inside the row.
    pub label: &'static str,
    /// Unique value within the containing group.
    pub value: &'static str,
    /// The same handle passed to the containing radio group.
    pub selection: EuvDropdownMenuRadioGroupState,
    /// Whether activation and keyboard navigation skip this item.
    pub disabled: bool,
    /// The owning dropdown's open signal.
    pub open: Signal<bool>,
    /// Whether activation closes the menu, even for the selected value. Defaults to false.
    pub close_on_click: bool,
}

/// Props of the [`euv_dropdown_menu_sub`] component.
///
/// The sub-menu holds the rows of a nested menu.
#[derive(Clone, Copy, CustomDebug, Default)]
pub struct EuvDropdownMenuSubProps {
    /// Optional descriptive label retained for compatibility.
    pub label: &'static str,
    /// This submenu's distinct controlled open state.
    pub open: Signal<bool>,
    /// Immediate parent popup state; closing it resets this entire branch.
    pub parent_open: Signal<bool>,
}

/// Props of the [`euv_dropdown_menu_sub_trigger`] component.
#[derive(Clone, Copy, CustomDebug)]
pub struct EuvDropdownMenuSubTriggerProps {
    /// The label shown inside the row.
    pub label: &'static str,
    /// Whether the sub-menu is open.
    pub open: Signal<bool>,
    /// Whether native activation and menu navigation skip this trigger.
    pub disabled: bool,
    /// Whether a hover-capable pointer opens this submenu. Defaults to true.
    pub open_on_hover: bool,
    /// Hover entry delay in milliseconds. Defaults to 100.
    pub delay_ms: u32,
    /// Hover dismissal delay in milliseconds. Defaults to zero.
    pub close_delay_ms: u32,
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
#[derive(Clone, CustomDebug, Data, New)]
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
    /// The owning root menu's open state, including for recursive submenu actions.
    /// Share this root signal with all action rows; submenu signals control panels only.
    pub open: Signal<bool>,
    /// Whether activation closes the owning menu. Defaults to true.
    #[get(type(copy))]
    pub close_on_click: bool,
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

/// Lifecycle-owned submenu hover controller. All asynchronous callbacks hold Weak handles.
#[cfg(target_arch = "wasm32")]
#[derive(Data)]
pub(super) struct DropdownSubHover {
    #[get(pub(super))]
    #[set(pub(super))]
    #[get_mut(pub(super))]
    pub(super) open: Signal<bool>,
    #[get(pub(super))]
    #[set(pub(super))]
    #[get_mut(pub(super))]
    pub(super) parent: Rc<Cell<Signal<bool>>>,
    #[get(pub(super))]
    #[set(pub(super))]
    #[get_mut(pub(super))]
    pub(super) target: NodeRef<Element>,
    #[get(pub(super))]
    #[set(pub(super))]
    #[get_mut(pub(super))]
    pub(super) options: Cell<(bool, bool, u32, u32)>,
    #[get(pub(super))]
    #[set(pub(super))]
    #[get_mut(pub(super))]
    pub(super) mounted: Cell<bool>,
    #[get(pub(super))]
    #[set(pub(super))]
    #[get_mut(pub(super))]
    pub(super) timer: Cell<Option<i32>>,
    #[get(pub(super))]
    #[set(pub(super))]
    #[get_mut(pub(super))]
    pub(super) intent: Cell<Option<bool>>,
    #[get(pub(super))]
    #[set(pub(super))]
    #[get_mut(pub(super))]
    pub(super) anchor: Cell<Option<(f64, f64)>>,
    #[get(pub(super))]
    #[set(pub(super))]
    #[get_mut(pub(super))]
    pub(super) callback: RefCell<Option<Closure<dyn FnMut()>>>,
}
