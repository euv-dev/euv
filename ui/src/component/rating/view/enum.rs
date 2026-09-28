use super::*;

/// The star glyph size of the `euv_rating` component.
///
/// The steps follow the shared monochrome type scale, one step below the
/// `euv_icon` `Xl` glyph so a five-star row stays compact.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EuvRatingSize {
    /// A 14px star for dense table cells.
    Sm,
    /// An 18px star, the default size for list rows.
    #[default]
    Md,
    /// A 24px star for standalone ratings.
    Lg,
}
