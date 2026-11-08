use super::*;

/// Props for the `euv_label` component.
///
/// A label names a control through the `for` attribute, so the text is
/// both the visible caption and the accessible name of the control it
/// points at.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvLabelProps {
    /// The id of the control the label names.
    #[get(type(copy))]
    pub html_for: &'static str,
    /// The visible label text.
    #[get(type(copy))]
    pub text: &'static str,
    /// Whether the labelled control is disabled.
    #[get(type(copy))]
    pub disabled: Signal<bool>,
}
