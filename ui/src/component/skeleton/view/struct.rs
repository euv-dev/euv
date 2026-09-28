use super::*;

/// Props for the [`euv_skeleton`] component.
///
/// Defines the strongly-typed interface for a loading placeholder. The
/// block is measured in whole lines rather than free-form children, so a
/// caller can mirror the shape of the content it replaces without
/// restyling it when the design tokens change.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvSkeletonProps {
    /// The number of placeholder lines to draw, clamped to
    /// [`skeleton_line_count`]'s `1..=8` range.
    pub lines: usize,
    /// The inline width applied to every line, e.g. `"60%"` (falls back
    /// to `"100%"` when empty or not a percentage).
    pub width: &'static str,
    /// The inline height applied to every line, e.g. `"24px"` (falls back
    /// to the stylesheet default when empty).
    pub height: &'static str,
    /// Whether the lines are drawn with rounded corners instead of the
    /// design system's square `border-radius: 0px` default.
    #[get(type(copy))]
    pub rounded: Signal<bool>,
}
