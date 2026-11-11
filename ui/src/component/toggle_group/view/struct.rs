use super::*;

/// One entry of the `euv_toggle_group` control.
///
/// Carries the machine value written into the selection signal, the label
/// the user reads, and whether the entry can be picked at all.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct EuvToggleGroupItem {
    /// The machine value written into the selection signal when picked.
    #[get(type(copy))]
    pub value: &'static str,
    /// The display label shown on the entry.
    #[get(type(copy))]
    pub label: &'static str,
    /// Whether the entry is disabled.
    #[get(type(copy))]
    pub disabled: bool,
}

/// Props for the `euv_toggle_group` component.
///
/// The selection lives in a caller-owned `Signal<Vec<String>>` so the
/// control takes part in two-way binding with the rest of the page. A
/// single-selection group keeps one value in it; a multiple-selection
/// group keeps every picked value.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvToggleGroupProps {
    /// The entries rendered in the control, in display order.
    pub items: Vec<EuvToggleGroupItem>,
    /// The signal holding the selected values.
    pub values: Signal<Vec<String>>,
    /// Whether more than one entry can be selected at a time.
    pub multiple: bool,
    /// Whether a single-selection group can be emptied by picking the
    /// selected entry again.
    pub collapsible: bool,
    /// The visual variant of every entry.
    pub variant: EuvToggleVariant,
    /// The control size of every entry.
    pub size: EuvToggleSize,
    /// Optional callback receiving the selected values.
    #[debug(skip)]
    pub on_change: Option<Rc<dyn Fn(Vec<String>)>>,
}

/// The settings one entry of a toggle group is drawn with.
///
/// The entry carries everything the row needs, so the row is drawn by
/// one call and the group keeps the shape of its own data.
pub(crate) struct ToggleGroupEntry {
    /// The entry the row stands for.
    pub(crate) item: EuvToggleGroupItem,
    /// Whether the entry is pressed.
    pub(crate) is_pressed: bool,
    /// The values the group has selected.
    pub(crate) values: Signal<Vec<String>>,
    /// Whether the group selects more than one entry.
    pub(crate) multiple: bool,
    /// Whether selecting an entry deselects the others.
    pub(crate) collapsible: bool,
    /// The visual variant of the row.
    pub(crate) variant: EuvToggleVariant,
    /// The control size of the row.
    pub(crate) size: EuvToggleSize,
    /// The value whose entry currently owns the roving tab stop.
    pub(crate) focus_value: Signal<String>,
    /// The callback fired when the selection changes.
    pub(crate) on_change: Option<Rc<dyn Fn(Vec<String>)>>,
}
