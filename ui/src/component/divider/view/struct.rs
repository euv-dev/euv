use super::*;

/// Props for the [`euv_divider`] component.
///
/// A non-empty `label` switches the plain rule to the labelled form: the
/// text sits between two rule segments.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvDividerProps {
    /// The direction the rule runs in.
    pub orientation: EuvDividerOrientation,
    /// The optional label centred between two rule segments (skipped when
    /// empty).
    pub label: &'static str,
}
