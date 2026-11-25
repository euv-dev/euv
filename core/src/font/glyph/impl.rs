use super::*;

/// Construction helpers for `GlyphKey`.
impl GlyphKey {
    /// Builds a quantized `GlyphKey` from a character, pixel size, and font weight.
    ///
    /// # Arguments
    ///
    /// - `char` - Character to rasterize.
    /// - `f32` - Effective physical font size in pixels.
    /// - `FontWeight` - Font weight determining bold synthesis.
    ///
    /// # Returns
    ///
    /// - `Self` - Quantized cache key.
    #[must_use]
    pub fn from_style(codepoint: char, physical_size: f32, weight: FontWeight) -> Self {
        let clamped: f32 = physical_size.clamp(6.0, 256.0);
        let size_tenths: u16 = (clamped * 10.0 + 0.5) as u16;
        Self::new(codepoint, size_tenths, weight.is_bold())
    }
}
