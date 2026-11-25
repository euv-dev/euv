use super::*;

/// Caching lookup operations for `GlyphAtlas`.
impl GlyphAtlas {
    /// Returns the cached `RasterizedGlyph` for `key`, rasterizing and inserting it on a miss.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable reference to the glyph atlas.
    /// - `GlyphKey` - Quantized glyph lookup key.
    ///
    /// # Returns
    ///
    /// - `&RasterizedGlyph` - Cached coverage bitmap and pixel metrics.
    pub fn get_or_rasterize(&mut self, key: GlyphKey) -> &RasterizedGlyph {
        if !self.get_glyphs().contains_key(&key) {
            let glyph: RasterizedGlyph = rasterize_glyph(key);
            self.get_mut_glyphs().insert(key, glyph);
            let next_rev: u64 = self.get_revision().wrapping_add(1);
            self.set_revision(next_rev);
        }
        &self.get_glyphs()[&key]
    }

    /// Returns the total number of cached glyph entries.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Glyph atlas reference.
    ///
    /// # Returns
    ///
    /// - `usize` - Number of cached glyphs.
    #[must_use]
    pub fn len(&self) -> usize {
        self.get_glyphs().len()
    }

    /// Returns true when the atlas contains no cached glyphs.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Glyph atlas reference.
    ///
    /// # Returns
    ///
    /// - `bool` - True if empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.get_glyphs().is_empty()
    }

    /// Clears all cached glyphs and increments the atlas revision.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable reference to the glyph atlas.
    pub fn clear(&mut self) {
        self.get_mut_glyphs().clear();
        let next_rev: u64 = self.get_revision().wrapping_add(1);
        self.set_revision(next_rev);
    }
}
