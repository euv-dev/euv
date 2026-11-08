use super::*;

/// Props for the [`euv_separator`] component.
///
/// A separator that is decorative carries no role at all: it is a rule
/// that divides, not a control, so the screen reader has nothing to
/// announce about it.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvSeparatorProps {
    /// The direction the rule runs in.
    pub orientation: EuvSeparatorOrientation,
    /// Whether the rule is decorative.
    pub decorative: bool,
}
