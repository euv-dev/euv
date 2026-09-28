use super::*;

/// The direction a [`euv_divider`] rule runs in.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EuvDividerOrientation {
    /// A full-width horizontal rule separating stacked blocks.
    #[default]
    Horizontal,
    /// A self-stretching vertical rule separating side-by-side blocks.
    Vertical,
}
