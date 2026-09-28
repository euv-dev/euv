use super::*;

/// The spacing step of the `euv_space` component.
///
/// Each step maps to a design token rather than a hardcoded pixel value, so
/// the gap follows the theme's spacing scale.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EuvSpaceSize {
    /// The tightest step, for separating an icon from its label.
    Xs,
    /// A small step, for list items and inline groups.
    Sm,
    /// The default step, for sibling blocks inside a section.
    #[default]
    Md,
    /// A large step, for separating section groups.
    Lg,
    /// The largest step, for separating top-level page regions.
    Xl,
}
