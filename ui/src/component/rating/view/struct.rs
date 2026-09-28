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

impl EuvRatingSize {
    /// Returns the star glyph edge length in pixels for this size.
    ///
    /// # Returns
    ///
    /// - `i32` - The edge length in pixels: 14 for `Sm`, 18 for `Md`, 24 for
    ///   `Lg`.
    pub fn px(&self) -> i32 {
        match self {
            EuvRatingSize::Sm => 14,
            EuvRatingSize::Md => 18,
            EuvRatingSize::Lg => 24,
        }
    }
}

/// Props for the `euv_rating` component.
///
/// Defines the strongly-typed interface for the star rating. The value and
/// scale live in caller-owned signals so a parent can drive the rating and
/// receive updates when `readonly` is false.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvRatingProps {
    /// The current rating value, which may be fractional for half stars.
    pub value: Signal<f64>,
    /// The maximum rating value, the number of stars rendered.
    pub max: Signal<f64>,
    /// Whether the rating is display-only, suppressing the fill overlay.
    pub readonly: Signal<bool>,
    /// The star glyph size.
    pub size: EuvRatingSize,
}
