use super::*;

/// The direction the rows of a field group are laid out in.
///
/// The reference publishes three: the rows stack one under the other,
/// the rows sit side by side, and the rows stack until the group is
/// wide enough to hold them side by side.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum EuvFieldOrientation {
    /// The rows stack one under the other.
    #[default]
    Vertical,
    /// The rows sit side by side.
    Horizontal,
    /// The rows stack until the group is wide enough.
    Responsive,
}

/// The size a field legend is written at.
///
/// The reference publishes two: the legend of a set of fields, and the
/// caption of a single control.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum EuvFieldLegendVariant {
    /// The legend of a set of fields.
    #[default]
    Legend,
    /// The caption of a single control.
    Label,
}
