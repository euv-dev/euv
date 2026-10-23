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
