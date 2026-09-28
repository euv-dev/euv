use super::*;

impl EuvIconSize {
    /// Returns the glyph edge length in pixels for this size.
    ///
    /// # Returns
    ///
    /// - `i32` - The edge length in pixels: 12 for `Xs`, 16 for `Sm`, 20 for
    ///   `Md`, 24 for `Lg`, 32 for `Xl`.
    pub fn px(&self) -> i32 {
        match self {
            EuvIconSize::Xs => 12,
            EuvIconSize::Sm => 16,
            EuvIconSize::Md => 20,
            EuvIconSize::Lg => 24,
            EuvIconSize::Xl => 32,
        }
    }
}
