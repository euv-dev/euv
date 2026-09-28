use super::*;

/// The border treatment of the `euv_panel` component.
///
/// The monochrome design system expresses separation with borders rather than
/// shadows or background fills, so the variant only chooses the border style.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EuvPanelVariant {
    /// No border, for panels already separated by surrounding whitespace.
    #[default]
    Plain,
    /// A solid 1px border, for a strongly grouped section.
    Bordered,
    /// A dashed 1px border, for a loosely grouped section.
    Dashed,
}
