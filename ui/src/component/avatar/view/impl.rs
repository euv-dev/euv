use super::*;

impl EuvAvatarSize {
    /// Returns the square edge length in pixels for this size.
    ///
    /// # Returns
    ///
    /// - `i32` - The edge length in pixels: 24 for `Small`, 32 for `Medium`,
    ///   40 for `Large`.
    pub fn px(&self) -> i32 {
        match self {
            EuvAvatarSize::Small => 24,
            EuvAvatarSize::Medium => 32,
            EuvAvatarSize::Large => 40,
        }
    }
}
