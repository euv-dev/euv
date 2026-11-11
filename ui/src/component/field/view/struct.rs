use super::*;

/// Props for the `euv_field` component.
///
/// Defines the strongly-typed interface for a form field with label, input, and error display.
#[derive(Clone, CustomDebug, Data, Default)]
pub struct EuvFieldProps {
    /// The unique identifier for the input element.
    #[get(type(copy))]
    pub id: &'static str,
    /// The HTML name attribute for the input element.
    #[get(type(copy))]
    pub name: &'static str,
    /// The label text displayed above the input.
    #[get(type(copy))]
    pub label: &'static str,
    /// The HTML input type (e.g. "text", "email", "password", "number").
    #[get(type(copy))]
    pub input_type: &'static str,
    /// The placeholder text shown when the input is empty.
    #[get(type(copy))]
    pub placeholder: &'static str,
    /// The autocomplete hint for the browser.
    #[get(type(copy))]
    pub autocomplete: &'static str,
    /// The current input value.
    #[get(type(copy))]
    pub value: Signal<String>,
    /// The error message signal; when non-empty, the input shows error styling. When None, no error display.
    pub error: Option<Signal<String>>,
    /// Optional input event handler; defaults to `on_input_value(value)` if None.
    #[debug(skip)]
    pub oninput: Option<Rc<dyn Fn(Event)>>,
}

/// Props for the [`euv_field_set`] component.
///
/// The set takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvFieldSetProps {}

/// Props for the [`euv_field_legend`] component.
///
/// The legend says which size it is written at: the size of the legend
/// of a set of fields, or the size of the caption of one control.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvFieldLegendProps {
    /// The size the legend is written at.
    pub variant: EuvFieldLegendVariant,
}

/// Props for the [`euv_field_group`] component.
///
/// The group says how its rows are laid out and whether the fields it
/// holds are in the invalid state.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvFieldGroupProps {
    /// The direction the rows of the group are laid out in.
    pub orientation: EuvFieldOrientation,
    /// Whether the fields of the group are in the invalid state.
    pub invalid: bool,
}

/// Props for the [`euv_field_content`] component.
///
/// The content takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvFieldContentProps {}

/// Props for the [`euv_field_label`] component.
///
/// The label names the control it belongs to.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvFieldLabelProps {
    /// The id of the control the label names.
    pub html_for: &'static str,
}

/// Props for the [`euv_field_title`] component.
///
/// The title takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvFieldTitleProps {}

/// Props for the [`euv_field_description`] component.
///
/// The description takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvFieldDescriptionProps {}

/// Props for the [`euv_field_error`] component.
///
/// The error carries the message that tells the reader what is wrong.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvFieldErrorProps {
    /// The message that tells the reader what is wrong.
    pub message: &'static str,
}

/// Props for the [`euv_field_separator`] component.
///
/// The separator takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvFieldSeparatorProps {}
