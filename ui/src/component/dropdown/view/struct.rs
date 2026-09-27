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
