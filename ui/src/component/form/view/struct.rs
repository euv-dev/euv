use super::*;

/// Props for the [`euv_form`] component.
///
/// The component only owns the form shell: the fields themselves are passed
/// in as children, so the caller decides which controls the form holds.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvFormProps {
    /// The label of the submit button.
    ///
    /// An empty string renders the form without a submit button, which is
    /// the right choice for a form that submits itself.
    pub submit_label: &'static str,
    /// The callback invoked when the form is submitted.
    #[debug(skip)]
    pub on_submit: Option<Rc<dyn Fn()>>,
}

/// Props for the [`euv_form_field`] component.
///
/// The field wraps one control and tells the parts inside it which name the
/// control carries, so a label, a description and a message can point at the
/// same control without the caller repeating the name in three places.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvFormFieldProps {
    /// The name the control inside the field carries.
    pub name: &'static str,
    /// The id the control inside the field carries.
    pub id: &'static str,
    /// Whether the control inside the field is in the invalid state.
    pub invalid: bool,
}

/// Props for the [`euv_form_item`] component.
///
/// The item is the box around one field. It owns the id that the label, the
/// description and the message of the field point at.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvFormItemProps {
    /// The id the parts of the field point at.
    pub id: &'static str,
}
