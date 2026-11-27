use super::*;

/// Caching lookup operations for `GlyphAtlas`.
impl GlyphAtlas {
    /// Packs quantized glyph style attributes into a compact 32-bit signature for ASCII table matching.
    ///
    /// # Arguments
    ///
    /// - `u16` - Quantized physical font size in tenths of a pixel.
    /// - `bool` - Bold flag.
    /// - `bool` - Monospace family flag.
    /// - `bool` - Italic flag.
    ///
    /// # Returns
    ///
    /// - `u32` - Packed style signature.
    #[inline(always)]
    #[must_use]
    pub(crate) fn pack_style_sig(
        size_tenths: u16,
        bold: bool,
        monospace: bool,
        italic: bool,
    ) -> u32 {
        (size_tenths as u32)
            | (u32::from(bold) << 16)
            | (u32::from(monospace) << 17)
            | (u32::from(italic) << 18)
    }

    /// Returns the index of the `[u32; 128]` ASCII slot table matching `(size_tenths, bold, monospace, italic)`, creating it if absent.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable glyph atlas reference.
    /// - `u16` - Quantized physical font size in tenths of a pixel.
    /// - `bool` - Bold flag.
    /// - `bool` - Monospace family flag.
    /// - `bool` - Italic flag.
    ///
    /// # Returns
    ///
    /// - `usize` - Index inside `ascii_tables` for zero-hash ASCII glyph resolution.
    pub(crate) fn ascii_table_index(
        &mut self,
        size_tenths: u16,
        bold: bool,
        monospace: bool,
        italic: bool,
    ) -> usize {
        let style_sig: u32 = Self::pack_style_sig(size_tenths, bold, monospace, italic);
        let Self { ascii_tables, .. } = self;
        for (idx, table) in ascii_tables.iter().enumerate() {
            if table.style_sig == style_sig {
                return idx;
            }
        }
        let idx: usize = ascii_tables.len();
        ascii_tables.push(AsciiGlyphStyleTable {
            style_sig,
            slots: [u32::MAX; 128],
        });
        idx
    }

    /// Resolves an ASCII byte directly through `ascii_tables[table_idx]` with zero HashMap probing on hits.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable glyph atlas reference.
    /// - `usize` - Style table index returned by `ascii_table_index`.
    /// - `u8` - ASCII codepoint byte in `0..128`.
    /// - `(u16, bool, bool, bool)` - Quantized style metadata `(size_tenths, bold, monospace, italic)`.
    /// - `&dyn SystemFontProvider` - Native or deterministic font rasterization provider.
    ///
    /// # Returns
    ///
    /// - `u32` - Direct slot index into `entries`.
    #[inline(always)]
    pub(crate) fn get_or_rasterize_ascii_slot_with(
        &mut self,
        table_idx: usize,
        byte: u8,
        key_meta: (u16, bool, bool, bool),
        provider: &dyn SystemFontProvider,
    ) -> u32 {
        let Self { ascii_tables, .. } = self;
        let cached: u32 = ascii_tables[table_idx].slots[byte as usize];
        if cached != u32::MAX {
            return cached;
        }
        let (size_tenths, bold, monospace, italic): (u16, bool, bool, bool) = key_meta;
        let key: GlyphKey = GlyphKey {
            codepoint: byte as char,
            size_tenths,
            bold,
            monospace,
            italic,
        };
        let slot: u32 = self.get_or_rasterize_slot_with(key, provider);
        let Self { ascii_tables, .. } = self;
        ascii_tables[table_idx].slots[byte as usize] = slot;
        slot
    }

    /// Returns the arena slot index for `key`, rasterizing through `rasterize_glyph` on a miss.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable glyph atlas reference.
    /// - `GlyphKey` - Quantized glyph lookup key.
    ///
    /// # Returns
    ///
    /// - `u32` - Direct slot index into `entries`.
    pub(crate) fn get_or_rasterize_slot(&mut self, key: GlyphKey) -> u32 {
        let code: u32 = key.get_codepoint() as u32;
        let style_sig: u32 = Self::pack_style_sig(
            key.get_size_tenths(),
            key.get_bold(),
            key.get_monospace(),
            key.get_italic(),
        );
        let Self {
            glyph_indices,
            entries,
            ascii_tables,
            revision,
        } = self;
        if code < 128 {
            for table in ascii_tables.iter() {
                if table.style_sig == style_sig {
                    let cached: u32 = table.slots[code as usize];
                    if cached != u32::MAX {
                        return cached;
                    }
                    break;
                }
            }
        }
        if let Some(&slot) = glyph_indices.get(&key) {
            if code < 128 {
                for table in ascii_tables.iter_mut() {
                    if table.style_sig == style_sig {
                        table.slots[code as usize] = slot;
                        break;
                    }
                }
            }
            return slot;
        }
        let glyph: RasterizedGlyph = rasterize_glyph(key);
        let slot: u32 = entries.len() as u32;
        entries.push((key, glyph));
        glyph_indices.insert(key, slot);
        if code < 128 {
            let mut updated: bool = false;
            for table in ascii_tables.iter_mut() {
                if table.style_sig == style_sig {
                    table.slots[code as usize] = slot;
                    updated = true;
                    break;
                }
            }
            if !updated {
                let mut slots: [u32; 128] = [u32::MAX; 128];
                slots[code as usize] = slot;
                ascii_tables.push(AsciiGlyphStyleTable { style_sig, slots });
            }
        }
        *revision = revision.wrapping_add(1);
        slot
    }

    /// Returns the arena slot index for `key`, rasterizing through `provider` on a miss.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable glyph atlas reference.
    /// - `GlyphKey` - Quantized glyph lookup key.
    /// - `&dyn SystemFontProvider` - Native or deterministic font rasterization provider.
    ///
    /// # Returns
    ///
    /// - `u32` - Direct slot index into `entries`.
    pub(crate) fn get_or_rasterize_slot_with(
        &mut self,
        key: GlyphKey,
        provider: &dyn SystemFontProvider,
    ) -> u32 {
        let code: u32 = key.get_codepoint() as u32;
        let style_sig: u32 = Self::pack_style_sig(
            key.get_size_tenths(),
            key.get_bold(),
            key.get_monospace(),
            key.get_italic(),
        );
        let Self {
            glyph_indices,
            entries,
            ascii_tables,
            revision,
        } = self;
        if let Some(&slot) = glyph_indices.get(&key) {
            return slot;
        }
        let glyph: RasterizedGlyph = provider
            .rasterize(key)
            .unwrap_or_else(|| rasterize_glyph(key));
        let slot: u32 = entries.len() as u32;
        entries.push((key, glyph));
        glyph_indices.insert(key, slot);
        if code < 128 {
            for table in ascii_tables.iter_mut() {
                if table.style_sig == style_sig {
                    table.slots[code as usize] = slot;
                    break;
                }
            }
        }
        *revision = revision.wrapping_add(1);
        slot
    }

    /// Returns a read-only reference to the `RasterizedGlyph` at flat arena index `slot`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Glyph atlas reference.
    /// - `u32` - Valid slot index into `entries`.
    ///
    /// # Returns
    ///
    /// - `&RasterizedGlyph` - Cached coverage bitmap and pixel metrics.
    #[inline(always)]
    #[must_use]
    pub(crate) fn glyph_by_slot(&self, slot: u32) -> &RasterizedGlyph {
        let Self { entries, .. } = self;
        &entries[slot as usize].1
    }

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
        let slot: u32 = self.get_or_rasterize_slot(key);
        self.glyph_by_slot(slot)
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
        self.get_entries().len()
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
        self.get_entries().is_empty()
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
        let slot: u32 = self.get_or_rasterize_slot_with(key, provider);
        self.glyph_by_slot(slot)
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
        let entries: &[(GlyphKey, RasterizedGlyph)] = self.get_entries().as_slice();
        let glyph_count: usize = entries.len();
        let mut cursor_x: u32 = 1;
        let mut cursor_y: u32 = 1;
        let mut shelf_h: u32 = 0;
        let mut regions: HashMap<GlyphKey, (u32, u32, u32, u32)> =
            HashMap::with_capacity(glyph_count);
        let mut placements: Vec<(&RasterizedGlyph, u32, u32)> = Vec::with_capacity(glyph_count);
        for (key, glyph) in entries.iter() {
            let gw: u32 = glyph.get_width();
            let gh: u32 = glyph.get_height();
            if cursor_x + gw + 1 > atlas_w {
                cursor_x = 1;
                cursor_y += shelf_h + 1;
                shelf_h = 0;
            }
            regions.insert(*key, (cursor_x, cursor_y, gw, gh));
            placements.push((glyph, cursor_x, cursor_y));
            cursor_x += gw + 1;
            shelf_h = shelf_h.max(gh);
        }
        let atlas_h: u32 = (cursor_y + shelf_h + 1).max(16);
        let atlas_w_usize: usize = atlas_w as usize;
        let mut pixels: Vec<u8> = vec![0; atlas_w_usize * (atlas_h as usize) * 4];
        for (glyph, rx, ry) in placements.into_iter() {
            let gw: usize = glyph.get_width() as usize;
            let gh: usize = glyph.get_height() as usize;
            let cov: &[u8] = glyph.get_coverage().as_slice();
            for py in 0..gh {
                let src_row: &[u8] = &cov[py * gw..(py + 1) * gw];
                let dst_start: usize = (((ry as usize) + py) * atlas_w_usize + (rx as usize)) * 4;
                let dst_row: &mut [u8] = &mut pixels[dst_start..dst_start + gw * 4];
                for (alpha, dst_px) in src_row
                    .iter()
                    .copied()
                    .zip(dst_row.as_chunks_mut::<4>().0.iter_mut())
                {
                    *dst_px = [255, 255, 255, alpha];
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
        let Self {
            glyph_indices,
            entries,
            ascii_tables,
            revision,
        } = self;
        glyph_indices.clear();
        entries.clear();
        ascii_tables.clear();
        *revision = revision.wrapping_add(1);
    }
}
