use super::*;

/// Lifecycle-owned pending touch gesture and its cancellable browser timer.
#[derive(Data, Default)]
pub(crate) struct ContextMenuLongPress {
    /// Original viewport coordinates and the gesture start time.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) origin: Option<(i32, i32, f64)>,
    /// Browser timeout identifier, absent on native hosts.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) timer: Option<i32>,
    /// Owned callback; never forgotten or captured strongly by itself.
    #[get(skip)]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) callback: Option<Closure<dyn FnMut()>>,
}

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
    /// Whether pointer, keyboard, and touch opening are disabled.
    pub disabled: bool,
    /// The label shown inside the trigger.
    pub label: &'static str,
}

/// Props of the [`euv_context_menu_content`] component.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvContextMenuContentProps {
    /// Whether the menu is open.
    pub open: Signal<bool>,
}

/// Props of the [`euv_context_menu_group`] component.
///
/// The group holds the rows that belong to one heading.
#[derive(Clone, Copy, CustomDebug, Default)]
pub struct EuvContextMenuGroupProps {
    /// The accessible name of the group.
    pub label: &'static str,
}

/// Props of the [`euv_context_menu_radio_group`] component.
///
/// The group holds the rows that share one radio selection.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvContextMenuRadioGroupProps {
    /// The accessible name of the group.
    pub label: &'static str,
    /// Shared selection handle, also passed to every item in this group.
    pub selection: EuvContextMenuRadioGroupState,
}

/// Explicit group context shared by the container and its radio items.
///
/// Create this handle with [`use_context_menu_radio_group`] and clone it for
/// each part. The framework has no inherited component-context API.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvContextMenuRadioGroupState {
    /// The single source of truth for all checked states in this group.
    pub value: Signal<String>,
    /// Whether selection is controlled by the caller's change callback.
    pub controlled: bool,
    /// Called only when activation requests a different value.
    #[debug(skip)]
    pub on_value_change: Option<Rc<dyn Fn(String)>>,
}

/// Props of the [`euv_context_menu_sub`] component.
///
/// The sub-menu holds the rows of a nested menu.
#[derive(Clone, Copy, CustomDebug, Default)]
pub struct EuvContextMenuSubProps {
    /// Whether the sub-menu is open.
    pub open: Signal<bool>,
    /// Immediate owning popup's open signal: root for first-level submenus,
    /// containing submenu for deeper levels. Must differ from `open`.
    /// Closing this owner resets the entire descendant branch, not selection.
    pub parent_open: Signal<bool>,
}

/// Hook-owned guard independent of the app cleanup registry's strong slot handle.
#[derive(Data)]
pub(crate) struct ContextMenuSubBindingScope {
    #[get(pub(crate))]
    pub(crate) slot: Rc<RefCell<Option<ContextMenuSubBinding>>>,
}

/// Owned subscriptions connecting one submenu to its immediate parent.
#[derive(Data)]
pub(crate) struct ContextMenuSubBinding {
    /// Parent popup state, independent of radio selection.
    #[get(pub(crate), type(copy))]
    pub(crate) parent_open: Signal<bool>,
    /// This submenu's actual controlled state.
    #[get(pub(crate), type(copy))]
    pub(crate) open: Signal<bool>,
    /// Listener detached when this binding is replaced or unmounted.
    #[get(pub(crate), type(copy))]
    pub(crate) parent_subscription: usize,
    /// Guard preventing a hidden branch from being opened programmatically.
    #[get(pub(crate), type(copy))]
    pub(crate) open_subscription: usize,
    /// Browser ownership identity and weak controller for synchronous teardown.
    #[cfg(target_arch = "wasm32")]
    #[get(pub(crate))]
    pub(crate) hover_owner: Rc<ContextMenuHoverOwner>,
}

/// Props of the [`euv_context_menu_sub_trigger`] component.
#[derive(Clone, Copy, CustomDebug)]
pub struct EuvContextMenuSubTriggerProps {
    /// The label shown inside the row.
    pub label: &'static str,
    /// Whether the sub-menu is open.
    pub open: Signal<bool>,
    /// Whether activation and navigation skip this trigger.
    pub disabled: bool,
    /// Whether hover-capable mouse or pen movement opens this submenu.
    pub open_on_hover: bool,
    /// Hover entry delay in milliseconds; defaults to 100.
    pub delay_ms: u32,
    /// Hover dismissal delay in milliseconds; defaults to zero.
    pub close_delay_ms: u32,
}

/// Props of the [`euv_context_menu_sub_content`] component.
#[derive(Clone, Copy, CustomDebug, Default)]
pub struct EuvContextMenuSubContentProps {
    /// Whether the sub-menu is open.
    pub open: Signal<bool>,
}

/// Props of the [`euv_context_menu_item`] component.
#[derive(Clone, CustomDebug)]
pub struct EuvContextMenuItemProps {
    /// The shared open-state signal for the context menu.
    pub open: Signal<bool>,
    /// The value the row stands for.
    pub value: &'static str,
    /// The label shown inside the row.
    pub label: &'static str,
    /// Whether the row is disabled.
    pub disabled: bool,
    /// Whether choosing the row closes the menu. Defaults to true.
    pub close_on_click: bool,
    /// Optional callback receiving the chosen value.
    #[debug(skip)]
    pub on_select: Option<Rc<dyn Fn(&'static str)>>,
}

/// Props of the [`euv_context_menu_checkbox_item`] component.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvContextMenuCheckboxItemProps {
    /// The shared open-state signal for the context menu.
    pub open: Signal<bool>,
    /// Whether choosing the row closes the menu. Defaults to false.
    pub close_on_click: bool,
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
    /// The unique value this row represents within its group.
    pub value: &'static str,
    /// The same selection handle supplied to the containing radio group.
    pub selection: EuvContextMenuRadioGroupState,
    /// Whether activation and keyboard navigation skip this row.
    pub disabled: bool,
    /// The root menu's shared open state, including for submenu items.
    pub open: Signal<bool>,
    /// Whether activation closes the menu, including an already-selected row.
    /// Defaults to false.
    pub close_on_click: bool,
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

/// Lifecycle-owned submenu hover controller. All asynchronous callbacks hold Weak handles.
#[cfg(target_arch = "wasm32")]
#[derive(Data)]
pub(crate) struct ContextMenuSubHover {
    #[get(pub(super))]
    #[set(pub(super))]
    #[get_mut(pub(super))]
    pub(super) open: Signal<bool>,
    #[get(pub(super))]
    #[set(pub(super))]
    #[get_mut(pub(super))]
    pub(super) parent: Rc<ContextMenuHoverOwner>,
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
    /// Listener and subscription teardown runs before callback storage drops.
    #[get(pub(super))]
    pub(super) cleanup: RefCell<Option<Box<dyn FnOnce()>>>,
}

/// Binding-owned identity; the controller never strongly owns its binding.
#[cfg(target_arch = "wasm32")]
#[derive(Data)]
pub(crate) struct ContextMenuHoverOwner {
    #[get(pub(crate), type(copy))]
    pub(crate) parent: Signal<bool>,
    #[get(pub(crate))]
    pub(crate) controller: RefCell<Option<Weak<ContextMenuSubHover>>>,
}
