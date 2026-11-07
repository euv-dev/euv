use super::*;

/// Props of the [`euv_context_menu`] component.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvContextMenuProps {
    /// Whether the menu is open.
    pub open: Signal<bool>,
}

/// Props of the [`euv_context_menu_trigger`] component.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvContextMenuTriggerProps {
    /// Whether the menu is open.
    pub open: Signal<bool>,
    /// The label shown inside the trigger.
    pub label: &'static str,
}

/// Props of the [`euv_context_menu_content`] component.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvContextMenuContentProps {
    /// Whether the menu is open.
    pub open: Signal<bool>,
}

/// Props of the [`euv_context_menu_item`] component.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvContextMenuItemProps {
    /// The value the row stands for.
    pub value: &'static str,
    /// The label shown inside the row.
    pub label: &'static str,
    /// Whether the row is disabled.
    pub disabled: bool,
    /// Optional callback receiving the chosen value.
    #[debug(skip)]
    pub on_select: Option<Rc<dyn Fn(&'static str)>>,
}

/// Props of the [`euv_context_menu_checkbox_item`] component.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvContextMenuCheckboxItemProps {
    /// The label shown inside the row.
    pub label: &'static str,
    /// Whether the box is checked.
    pub checked: Signal<bool>,
    /// Optional callback fired when the row is chosen.
    #[debug(skip)]
    pub on_select: Option<Rc<dyn Fn(bool)>>,
}

/// Props of the [`euv_context_menu_radio_item`] component.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvContextMenuRadioItemProps {
    /// The label shown inside the row.
    pub label: &'static str,
    /// Whether the row is the chosen one.
    pub checked: Signal<bool>,
}

/// Props of the [`euv_context_menu_label`] component.
#[derive(Clone, Copy, CustomDebug, Default)]
pub struct EuvContextMenuLabelProps {
    /// The text of the heading.
    pub text: &'static str,
}

/// Props of the [`euv_context_menu_separator`] component.
#[derive(Clone, Copy, CustomDebug, Default)]
pub struct EuvContextMenuSeparatorProps {
    /// The label reported to assistive technology, if any.
    pub label: &'static str,
}

/// Props of the [`euv_context_menu_shortcut`] component.
#[derive(Clone, Copy, CustomDebug, Default)]
pub struct EuvContextMenuShortcutProps {
    /// The key hint text.
    pub text: &'static str,
}
