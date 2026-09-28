use super::*;

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
