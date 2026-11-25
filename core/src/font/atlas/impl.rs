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

    /// Returns the cached `RasterizedGlyph` for `key`, rasterizing through `provider` on a miss.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable reference to the glyph atlas.
    /// - `GlyphKey` - Quantized glyph lookup key.
    /// - `&dyn SystemFontProvider` - Native or deterministic font rasterization provider.
    ///
    /// # Returns
    ///
    /// - `&RasterizedGlyph` - Cached coverage bitmap and pixel metrics.
    pub fn get_or_rasterize_with(
        &mut self,
        key: GlyphKey,
        provider: &dyn SystemFontProvider,
    ) -> &RasterizedGlyph {
        if !self.get_glyphs().contains_key(&key) {
            let glyph: RasterizedGlyph = provider
                .rasterize(key)
                .unwrap_or_else(|| rasterize_glyph(key));
            self.get_mut_glyphs().insert(key, glyph);
            let next_rev: u64 = self.get_revision().wrapping_add(1);
            self.set_revision(next_rev);
        }
        &self.get_glyphs()[&key]
    }

    /// Shelf-packs all cached glyphs into a single GPU-uploadable RGBA8 texture atlas.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Glyph atlas reference.
    ///
    /// # Returns
    ///
    /// - `PackedGlyphAtlas` - Packed RGBA8 texture, per-glyph UV rects, and revision tag.
    #[must_use]
    pub fn pack_rgba_atlas(&self) -> PackedGlyphAtlas {
        let atlas_w: u32 = ATLAS_DEFAULT_WIDTH;
        let mut cursor_x: u32 = 1;
        let mut cursor_y: u32 = 1;
        let mut shelf_h: u32 = 0;
        let mut regions: HashMap<GlyphKey, (u32, u32, u32, u32)> = HashMap::new();
        for (key, glyph) in self.get_glyphs().iter() {
            let gw: u32 = glyph.get_width();
            let gh: u32 = glyph.get_height();
            if cursor_x + gw + 1 > atlas_w {
                cursor_x = 1;
                cursor_y += shelf_h + 1;
                shelf_h = 0;
            }
            regions.insert(*key, (cursor_x, cursor_y, gw, gh));
            cursor_x += gw + 1;
            shelf_h = shelf_h.max(gh);
        }
        let atlas_h: u32 = (cursor_y + shelf_h + 1).max(16);
        let mut pixels: Vec<u8> = vec![0; (atlas_w as usize) * (atlas_h as usize) * 4];
        for (key, (rx, ry, gw, gh)) in regions.iter() {
            if let Some(glyph) = self.get_glyphs().get(key) {
                let cov: &[u8] = glyph.get_coverage();
                for py in 0..(*gh as usize) {
                    for px in 0..(*gw as usize) {
                        let alpha: u8 = cov[py * (*gw as usize) + px];
                        let dst_idx: usize = (((*ry as usize) + py) * (atlas_w as usize)
                            + ((*rx as usize) + px))
                            * 4;
                        pixels[dst_idx] = 255;
                        pixels[dst_idx + 1] = 255;
                        pixels[dst_idx + 2] = 255;
                        pixels[dst_idx + 3] = alpha;
                    }
                }
            }
        }
        PackedGlyphAtlas::new(atlas_w, atlas_h, pixels, regions, self.get_revision())
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
