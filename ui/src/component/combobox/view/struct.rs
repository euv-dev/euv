use super::*;

/// Props of the [`euv_combobox`] component.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvComboboxProps {
    /// Whether the list of options is open.
    pub open: Signal<bool>,
    /// The value the field is showing.
    pub value: Signal<String>,
    /// The accessible name of the combobox.
    pub label: &'static str,
}

/// Props of the [`euv_combobox_trigger`] component.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvComboboxTriggerProps {
    /// Whether the list of options is open.
    pub open: Signal<bool>,
    /// The label shown inside the trigger.
    pub label: &'static str,
    /// Whether the trigger is disabled.
    pub disabled: bool,
}

/// Props of the [`euv_combobox_input`] component.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvComboboxInputProps {
    /// Whether the list of options is open.
    pub open: Signal<bool>,
    /// The value the field is showing.
    pub value: Signal<String>,
    /// The text shown while the field is empty.
    pub placeholder: &'static str,
    /// Whether the field is disabled.
    pub disabled: bool,
}

/// Props of the [`euv_combobox_content`] component.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvComboboxContentProps {
    /// Whether the list of options is open.
    pub open: Signal<bool>,
}

/// Props of the [`euv_combobox_list`] component.
#[derive(Clone, Copy, CustomDebug, Default)]
pub struct EuvComboboxListProps {
    /// The accessible name of the list.
    pub label: &'static str,
}

/// Props of the [`euv_combobox_item`] component.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvComboboxItemProps {
    /// The open-state signal for the list of options.
    pub open: Signal<bool>,
    /// The value the row stands for.
    pub value: &'static str,
    /// The label shown inside the row.
    pub label: &'static str,
    /// Whether the row is the selected one.
    pub selected: bool,
    /// Whether the row is disabled.
    pub disabled: bool,
    /// Optional callback receiving the chosen value.
    #[debug(skip)]
    pub on_select: Option<Rc<dyn Fn(&'static str)>>,
}

/// Props of the [`euv_combobox_group`] component.
#[derive(Clone, Copy, CustomDebug, Default)]
pub struct EuvComboboxGroupProps {
    /// The accessible name of the group.
    pub label: &'static str,
}

/// Props of the [`euv_combobox_label`] component.
#[derive(Clone, Copy, CustomDebug, Default)]
pub struct EuvComboboxLabelProps {
    /// The text of the label.
    pub text: &'static str,
}

/// Props of the [`euv_combobox_empty`] component.
#[derive(Clone, Copy, CustomDebug, Default)]
pub struct EuvComboboxEmptyProps {
    /// The text shown when nothing matched the query.
    pub text: &'static str,
}

/// Props of the [`euv_combobox_separator`] component.
#[derive(Clone, Copy, CustomDebug, Default)]
pub struct EuvComboboxSeparatorProps {
    /// The label reported to assistive technology, if any.
    pub label: &'static str,
}

/// Props of the [`euv_combobox_clear`] component.
#[derive(Clone, Copy, CustomDebug, Default)]
pub struct EuvComboboxClearProps {
    /// The accessible name of the button.
    pub label: &'static str,
}

/// Props of the [`euv_combobox_chips`] component.
///
/// The chip row is the group that holds the chosen values and the text
/// input the reader types into.
#[derive(Clone, Copy, CustomDebug, Default)]
pub struct EuvComboboxChipsProps {
    /// The accessible name of the chip row.
    pub label: &'static str,
}

/// Props of the [`euv_combobox_chip_list`] component.
///
/// The chip list holds the chip rows themselves; it is a wrapper that
/// groups them for assistive technology.
#[derive(Clone, Copy, CustomDebug, Default)]
pub struct EuvComboboxChipListProps {
    /// The accessible name of the chip list.
    pub label: &'static str,
}

/// Props of the [`euv_combobox_chip`] component.
///
/// A chip is one chosen value. The chip carries its own remove button,
/// so the caller decides what removing a value means.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvComboboxChipProps {
    /// The text shown inside the chip.
    pub label: &'static str,
    /// The value the chip stands for.
    pub value: &'static str,
    /// Whether the chip is disabled.
    pub disabled: bool,
    /// Optional callback fired when the chip is removed.
    #[debug(skip)]
    pub on_remove: Option<Rc<dyn Fn(&'static str)>>,
}

/// Props of the [`euv_combobox_chips_input`] component.
///
/// The input lives inside the chip row: the reader types the query there
/// while the chosen values sit next to it as chips.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvComboboxChipsInputProps {
    /// The value the field is showing.
    pub value: Signal<String>,
    /// The text shown while the field is empty.
    pub placeholder: &'static str,
    /// Whether the field is disabled.
    pub disabled: bool,
}

/// Props of the [`euv_combobox_value`] component.
///
/// The value shows one chosen entry of the list.
#[derive(Clone, Copy, CustomDebug, Default)]
pub struct EuvComboboxValueProps {
    /// The text of the chosen value.
    pub label: &'static str,
}
