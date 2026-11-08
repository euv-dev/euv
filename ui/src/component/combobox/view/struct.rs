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
