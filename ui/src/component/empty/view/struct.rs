use super::*;

/// Props for the [`euv_empty`] component.
///
/// Defines the strongly-typed interface for an empty state: the glyph
/// that marks the emptiness, the headline, the explanation, and an
/// optional action that offers the reader a way out of it.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvEmptyProps {
    /// The optional glyph shown above the headline (skipped when empty).
    pub icon: &'static str,
    /// The headline naming what is missing.
    pub title: &'static str,
    /// The optional explanation under the headline (skipped when empty).
    pub desc: &'static str,
    /// The optional action label (skipped when empty).
    pub action_label: &'static str,
    /// The callback fired when the action is taken.
    #[debug(skip)]
    pub on_action: Option<Rc<dyn Fn(Event)>>,
}
