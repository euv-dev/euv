use super::*;

/// Abstraction for native OS and deterministic glyph rasterization providers.
pub trait SystemFontProvider {
    /// Returns metadata describing this font provider's active backend and capabilities.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Font provider reference.
    ///
    /// # Returns
    ///
    /// - `SystemFontInfo` - Provider capabilities and family metadata.
    fn font_info(&self) -> SystemFontInfo;

    /// Rasterizes a quantized `GlyphKey` into an alpha8 coverage bitmap.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Font provider reference.
    /// - `GlyphKey` - Quantized character, font size, bold flag, and family.
    ///
    /// # Returns
    ///
    /// - `Option<RasterizedGlyph>` - Rasterized glyph bitmap if supported by this provider.
    fn rasterize(&self, key: GlyphKey) -> Option<RasterizedGlyph>;
}
