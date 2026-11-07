use super::*;

/// Props for the `euv_spinner` component.
///
/// The spinner is a busy indicator, so it always carries an accessible
/// name: the `label` is written into `aria-label` whether or not it is
/// also shown next to the glyph.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvSpinnerProps {
    /// The accessible name of the busy indicator.
    #[get(type(copy))]
    pub label: &'static str,
    /// Whether the label is also shown next to the glyph.
    #[get(type(copy))]
    pub show_label: Signal<bool>,
}
