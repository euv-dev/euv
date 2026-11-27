use super::*;

/// Caching lookup, bounded LRU eviction, and telemetry operations for `GlyphAtlas`.
impl GlyphAtlas {
    /// Losslessly packs all fields of `GlyphKey` into the low 40 bits of a `u64`.
    ///
    /// # Arguments
    ///
    /// - `GlyphKey` - Quantized glyph lookup key.
    ///
    /// # Returns
    ///
    /// - `u64` - Bijective 40-bit key representation (`0..2^40`).
    #[inline(always)]
    #[must_use]
    pub(crate) fn pack_glyph_key(key: GlyphKey) -> u64 {
        key.pack_u64()
    }

    /// Computes a fast single-multiply avalanche hash for a packed 40-bit `GlyphKey`.
    ///
    /// # Arguments
    ///
    /// - `u64` - Lossless 40-bit packed glyph key.
    ///
    /// # Returns
    ///
    /// - `usize` - Mixed hash value suitable for power-of-two bitmasking.
    #[inline(always)]
    #[must_use]
    fn hash_packed_key(packed: u64) -> usize {
        let mixed: u64 = packed.wrapping_mul(GLYPH_INDEX_HASH_MULT);
        (mixed ^ (mixed >> 32)) as usize
    }

    /// Estimates the resident memory footprint in bytes of one rasterized glyph entry.
    ///
    /// # Arguments
    ///
    /// - `&RasterizedGlyph` - Cached coverage bitmap entry.
    ///
    /// # Returns
    ///
    /// - `usize` - Tracked byte size including coverage bytes and metadata overhead.
    #[inline(always)]
    #[must_use]
    fn entry_bytes(glyph: &RasterizedGlyph) -> usize {
        glyph
            .get_coverage()
            .len()
            .saturating_add(GLYPH_ENTRY_OVERHEAD_BYTES)
    }

    /// Resolves the active resident memory budget in bytes (`GLYPH_ATLAS_DEFAULT_BUDGET_BYTES` when unset).
    ///
    /// # Arguments
    ///
    /// - `&Self` - Glyph atlas reference.
    ///
    /// # Returns
    ///
    /// - `usize` - Effective byte budget threshold.
    #[inline(always)]
    #[must_use]
    pub fn effective_budget_bytes(&self) -> usize {
        let raw: usize = self.get_budget_bytes();
        if raw == 0 {
            GLYPH_ATLAS_DEFAULT_BUDGET_BYTES
        } else {
            raw
        }
    }

    /// Configures a bounded resident memory budget in bytes and immediately evicts cold glyphs if currently over budget.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable glyph atlas reference.
    /// - `usize` - Maximum resident byte budget (clamped to `>= 1024`).
    pub fn set_memory_budget(&mut self, max_bytes: usize) {
        let clamped: usize = max_bytes.max(GLYPH_ATLAS_MIN_BUDGET_BYTES);
        self.set_budget_bytes(clamped);
        self.evict_if_over_budget();
    }

    /// Returns the cumulative cache hit ratio in `[0.0, 1.0]` (`1.0` before any lookup).
    ///
    /// # Arguments
    ///
    /// - `&Self` - Glyph atlas reference.
    ///
    /// # Returns
    ///
    /// - `f32` - Cache hit rate fraction.
    #[must_use]
    pub fn hit_rate(&self) -> f32 {
        let hits: u64 = self.get_cache_hits();
        let total: u64 = hits.saturating_add(self.get_cache_misses());
        if total == 0 {
            1.0
        } else {
            (hits as f32) / (total as f32)
        }
    }

    /// Evicts least-recently-used glyphs when `resident_bytes` exceeds the active budget and slots are unpinned.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable glyph atlas reference.
    pub fn evict_if_over_budget(&mut self) {
        let budget: usize = self.effective_budget_bytes();
        let Self {
            resident_bytes,
            slots_pinned,
            entries,
            ..
        } = self;
        if *slots_pinned || *resident_bytes <= budget || entries.len() <= 1 {
            return;
        }
        let target: usize = (budget.saturating_mul(3) / 4).max(GLYPH_ENTRY_OVERHEAD_BYTES);
        self.evict_lru_to_target(target);
    }

    /// Compacts the glyph arena by evicting the coldest LRU entries until `resident_bytes <= target_bytes`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable glyph atlas reference.
    /// - `usize` - Target post-eviction resident byte ceiling.
    fn evict_lru_to_target(&mut self, target_bytes: usize) {
        let Self {
            glyph_index,
            entries,
            entry_last_used,
            ascii_tables,
            revision,
            resident_bytes,
            evictions,
            ..
        } = self;
        let count: usize = entries.len();
        if count == 0 || *resident_bytes <= target_bytes {
            return;
        }
        let mut order: Vec<usize> = (0..count).collect();
        order.sort_unstable_by_key(|idx: &usize| entry_last_used.get(*idx).copied().unwrap_or(0));
        let mut keep: Vec<bool> = vec![true; count];
        let mut remaining_bytes: usize = *resident_bytes;
        let mut evicted_count: usize = 0;
        for idx in order.into_iter() {
            if remaining_bytes <= target_bytes {
                break;
            }
            let bytes: usize = Self::entry_bytes(&entries[idx].1);
            remaining_bytes = remaining_bytes.saturating_sub(bytes);
            keep[idx] = false;
            evicted_count += 1;
        }
        if evicted_count == 0 {
            return;
        }
        let mut write_idx: usize = 0;
        let mut next_resident: usize = 0;
        for (read_idx, &kept) in keep.iter().enumerate() {
            if kept {
                if write_idx != read_idx {
                    entries.swap(write_idx, read_idx);
                    if read_idx < entry_last_used.len() && write_idx < entry_last_used.len() {
                        entry_last_used.swap(write_idx, read_idx);
                    }
                }
                next_resident =
                    next_resident.saturating_add(Self::entry_bytes(&entries[write_idx].1));
                write_idx += 1;
            }
        }
        entries.truncate(write_idx);
        entry_last_used.truncate(write_idx);
        *resident_bytes = next_resident;
        *evictions = evictions.saturating_add(evicted_count as u64);
        glyph_index.fill(GLYPH_INDEX_EMPTY);
        for (slot_idx, (key, _)) in entries.iter().enumerate() {
            let packed_key: u64 = Self::pack_glyph_key(*key);
            Self::insert_index(glyph_index, slot_idx + 1, packed_key, slot_idx as u32);
        }
        for table in ascii_tables.iter_mut() {
            table.slots.fill(u32::MAX);
        }
        *revision = revision.wrapping_add(1);
    }

    /// Probes the open-addressed `glyph_index` slice for `packed_key`.
    ///
    /// # Arguments
    ///
    /// - `&[u64]` - Power-of-two open-addressed index table.
    /// - `u64` - Lossless 40-bit packed glyph key.
    ///
    /// # Returns
    ///
    /// - `Option<u32>` - Cached arena slot index when present.
    #[inline(always)]
    #[must_use]
    fn lookup_index(glyph_index: &[u64], packed_key: u64) -> Option<u32> {
        let cap: usize = glyph_index.len();
        if cap == 0 {
            return None;
        }
        let mask: usize = cap - 1;
        let mut idx: usize = Self::hash_packed_key(packed_key) & mask;
        loop {
            let entry: u64 = glyph_index[idx];
            if entry == GLYPH_INDEX_EMPTY {
                return None;
            }
            if (entry & GLYPH_INDEX_KEY_MASK) == packed_key {
                return Some((entry >> GLYPH_INDEX_SLOT_SHIFT) as u32);
            }
            idx = (idx + 1) & mask;
        }
    }

    /// Inserts `(packed_key, slot)` into the open-addressed `glyph_index` table, doubling capacity at 75% load.
    ///
    /// # Arguments
    ///
    /// - `&mut Vec<u64>` - Open-addressed index table storage.
    /// - `usize` - Current total entry count including the newly inserted glyph.
    /// - `u64` - Lossless 40-bit packed glyph key.
    /// - `u32` - Arena slot index into `entries`.
    fn insert_index(glyph_index: &mut Vec<u64>, entry_count: usize, packed_key: u64, slot: u32) {
        let cap: usize = glyph_index.len();
        if cap == 0 {
            glyph_index.resize(GLYPH_INDEX_INIT_CAP, GLYPH_INDEX_EMPTY);
        } else if entry_count * 4 > cap * 3 {
            let new_cap: usize = cap * 2;
            let new_mask: usize = new_cap - 1;
            let mut grown: Vec<u64> = vec![GLYPH_INDEX_EMPTY; new_cap];
            for old_entry in glyph_index.iter().copied() {
                if old_entry == GLYPH_INDEX_EMPTY {
                    continue;
                }
                let old_key: u64 = old_entry & GLYPH_INDEX_KEY_MASK;
                let mut idx: usize = Self::hash_packed_key(old_key) & new_mask;
                while grown[idx] != GLYPH_INDEX_EMPTY {
                    idx = (idx + 1) & new_mask;
                }
                grown[idx] = old_entry;
            }
            *glyph_index = grown;
        }
        let mask: usize = glyph_index.len() - 1;
        let packed_entry: u64 = packed_key | ((slot as u64) << GLYPH_INDEX_SLOT_SHIFT);
        let mut idx: usize = Self::hash_packed_key(packed_key) & mask;
        loop {
            let existing: u64 = glyph_index[idx];
            if existing == GLYPH_INDEX_EMPTY || (existing & GLYPH_INDEX_KEY_MASK) == packed_key {
                glyph_index[idx] = packed_entry;
                return;
            }
            idx = (idx + 1) & mask;
        }
    }

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

    /// Resolves an ASCII byte directly through `ascii_tables[table_idx]` with zero hash probing on hits.
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
        let Self {
            ascii_tables,
            entry_last_used,
            lru_clock,
            cache_hits,
            ..
        } = self;
        let cached: u32 = ascii_tables[table_idx].slots[byte as usize];
        if cached != u32::MAX {
            *cache_hits = cache_hits.wrapping_add(1);
            *lru_clock = lru_clock.wrapping_add(1);
            if let Some(tick) = entry_last_used.get_mut(cached as usize) {
                *tick = *lru_clock;
            }
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

    /// Inserts a newly rasterized glyph into the arena, enforcing the bounded memory budget when unpinned.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable glyph atlas reference.
    /// - `GlyphKey` - Quantized glyph lookup key.
    /// - `u64` - Pre-packed 40-bit glyph key.
    /// - `u32` - Pre-packed style signature for ASCII table updates.
    /// - `RasterizedGlyph` - Newly rasterized coverage bitmap.
    /// - `bool` - Whether a missing ASCII style table should be created automatically.
    ///
    /// # Returns
    ///
    /// - `u32` - Newly assigned arena slot index.
    fn insert_rasterized_glyph(
        &mut self,
        key: GlyphKey,
        packed_key: u64,
        style_sig: u32,
        glyph: RasterizedGlyph,
        create_ascii_table: bool,
    ) -> u32 {
        let entry_sz: usize = Self::entry_bytes(&glyph);
        let budget: usize = self.effective_budget_bytes();
        let needs_evict: bool = {
            let Self {
                resident_bytes,
                slots_pinned,
                entries,
                ..
            } = self;
            !*slots_pinned
                && !entries.is_empty()
                && resident_bytes.saturating_add(entry_sz) > budget
        };
        if needs_evict {
            let target: usize = budget
                .saturating_mul(3)
                .saturating_div(4)
                .saturating_sub(entry_sz);
            self.evict_lru_to_target(target);
        }
        let code: u32 = key.get_codepoint() as u32;
        let Self {
            glyph_index,
            entries,
            entry_last_used,
            ascii_tables,
            revision,
            resident_bytes,
            lru_clock,
            cache_misses,
            ..
        } = self;
        if entries.is_empty() {
            entries.reserve(GLYPH_ENTRIES_INIT_CAP);
            entry_last_used.reserve(GLYPH_ENTRIES_INIT_CAP);
        }
        *cache_misses = cache_misses.wrapping_add(1);
        *lru_clock = lru_clock.wrapping_add(1);
        let slot: u32 = entries.len() as u32;
        entries.push((key, glyph));
        entry_last_used.push(*lru_clock);
        *resident_bytes = resident_bytes.saturating_add(entry_sz);
        Self::insert_index(glyph_index, entries.len(), packed_key, slot);
        if code < 128 {
            let mut updated: bool = false;
            for table in ascii_tables.iter_mut() {
                if table.style_sig == style_sig {
                    table.slots[code as usize] = slot;
                    updated = true;
                    break;
                }
            }
            if create_ascii_table && !updated {
                let mut slots: [u32; 128] = [u32::MAX; 128];
                slots[code as usize] = slot;
                ascii_tables.push(AsciiGlyphStyleTable { style_sig, slots });
            }
        }
        *revision = revision.wrapping_add(1);
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
    #[inline]
    pub(crate) fn get_or_rasterize_slot(&mut self, key: GlyphKey) -> u32 {
        let code: u32 = key.get_codepoint() as u32;
        let style_sig: u32 = Self::pack_style_sig(
            key.get_size_tenths(),
            key.get_bold(),
            key.get_monospace(),
            key.get_italic(),
        );
        let packed_key: u64 = Self::pack_glyph_key(key);
        {
            let Self {
                glyph_index,
                entry_last_used,
                ascii_tables,
                lru_clock,
                cache_hits,
                ..
            } = self;
            if code < 128 {
                for table in ascii_tables.iter() {
                    if table.style_sig == style_sig {
                        let cached: u32 = table.slots[code as usize];
                        if cached != u32::MAX {
                            *cache_hits = cache_hits.wrapping_add(1);
                            *lru_clock = lru_clock.wrapping_add(1);
                            if let Some(tick) = entry_last_used.get_mut(cached as usize) {
                                *tick = *lru_clock;
                            }
                            return cached;
                        }
                        break;
                    }
                }
            }
            if let Some(slot) = Self::lookup_index(glyph_index, packed_key) {
                *cache_hits = cache_hits.wrapping_add(1);
                *lru_clock = lru_clock.wrapping_add(1);
                if let Some(tick) = entry_last_used.get_mut(slot as usize) {
                    *tick = *lru_clock;
                }
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
        }
        let glyph: RasterizedGlyph = rasterize_glyph(key);
        self.insert_rasterized_glyph(key, packed_key, style_sig, glyph, true)
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
    #[inline]
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
        let packed_key: u64 = Self::pack_glyph_key(key);
        {
            let Self {
                glyph_index,
                entry_last_used,
                ascii_tables,
                lru_clock,
                cache_hits,
                ..
            } = self;
            if code < 128 && !ascii_tables.is_empty() {
                for table in ascii_tables.iter() {
                    if table.style_sig == style_sig {
                        let cached: u32 = table.slots[code as usize];
                        if cached != u32::MAX {
                            *cache_hits = cache_hits.wrapping_add(1);
                            *lru_clock = lru_clock.wrapping_add(1);
                            if let Some(tick) = entry_last_used.get_mut(cached as usize) {
                                *tick = *lru_clock;
                            }
                            return cached;
                        }
                        break;
                    }
                }
            }
            if let Some(slot) = Self::lookup_index(glyph_index, packed_key) {
                *cache_hits = cache_hits.wrapping_add(1);
                *lru_clock = lru_clock.wrapping_add(1);
                if let Some(tick) = entry_last_used.get_mut(slot as usize) {
                    *tick = *lru_clock;
                }
                return slot;
            }
        }
        let glyph: RasterizedGlyph = provider
            .rasterize(key)
            .unwrap_or_else(|| rasterize_glyph(key));
        self.insert_rasterized_glyph(key, packed_key, style_sig, glyph, false)
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
    #[inline]
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
    #[inline]
    pub fn get_or_rasterize_with(
        &mut self,
        key: GlyphKey,
        provider: &dyn SystemFontProvider,
    ) -> &RasterizedGlyph {
        let slot: u32 = self.get_or_rasterize_slot_with(key, provider);
        self.glyph_by_slot(slot)
    }

    /// Blits one glyph's 8-bit coverage bitmap into a white-RGB RGBA8 atlas slice at `(rx, ry)`.
    ///
    /// # Arguments
    ///
    /// - `&RasterizedGlyph` - Source coverage bitmap.
    /// - `&mut [u8]` - Destination RGBA8 atlas pixel buffer.
    /// - `usize` - Atlas width in pixels.
    /// - `u32` - Destination X pixel offset.
    /// - `u32` - Destination Y pixel offset.
    #[inline]
    fn blit_glyph_to_rgba(
        glyph: &RasterizedGlyph,
        pixels: &mut [u8],
        atlas_w: usize,
        rx: u32,
        ry: u32,
    ) {
        let gw: usize = glyph.get_width() as usize;
        let gh: usize = glyph.get_height() as usize;
        let cov: &[u8] = glyph.get_coverage().as_slice();
        let row_stride: usize = atlas_w * 4;
        let mut src_off: usize = 0;
        let mut dst_off: usize = ((ry as usize) * atlas_w + (rx as usize)) * 4;
        for _ in 0..gh {
            let src_row: &[u8] = &cov[src_off..src_off + gw];
            let dst_row: &mut [u8] = &mut pixels[dst_off..dst_off + gw * 4];
            let (src_quads, src_rem): (&[[u8; 4]], &[u8]) = src_row.as_chunks::<4>();
            let (dst_quads, dst_rem): (&mut [[u8; 16]], &mut [u8]) = dst_row.as_chunks_mut::<16>();
            for (a4, dst16) in src_quads.iter().zip(dst_quads.iter_mut()) {
                *dst16 = [
                    255, 255, 255, a4[0], 255, 255, 255, a4[1], 255, 255, 255, a4[2], 255, 255,
                    255, a4[3],
                ];
            }
            for (alpha, dst4) in src_rem
                .iter()
                .copied()
                .zip(dst_rem.as_chunks_mut::<4>().0.iter_mut())
            {
                *dst4 = [255, 255, 255, alpha];
            }
            src_off += gw;
            dst_off += row_stride;
        }
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
        let entries: &[(GlyphKey, RasterizedGlyph)] = self.get_entries().as_slice();
        let glyph_count: usize = entries.len();
        let mut total_row_w: u32 = 1;
        let mut max_glyph_w: u32 = 0;
        for (_, glyph) in entries.iter() {
            let gw: u32 = glyph.get_width();
            total_row_w = total_row_w.saturating_add(gw.saturating_add(1));
            if gw > max_glyph_w {
                max_glyph_w = gw;
            }
        }
        let min_needed_w: u32 = max_glyph_w.saturating_add(2).max(ATLAS_MIN_DIM);
        let atlas_w: u32 = if total_row_w <= ATLAS_DEFAULT_WIDTH {
            total_row_w
                .max(ATLAS_MIN_DIM)
                .next_power_of_two()
                .min(ATLAS_DEFAULT_WIDTH)
        } else {
            ATLAS_DEFAULT_WIDTH.max(min_needed_w.next_power_of_two())
        };
        let mut cursor_x: u32 = 1;
        let mut cursor_y: u32 = 1;
        let mut shelf_h: u32 = 0;
        let mut regions: HashMap<GlyphKey, (u32, u32, u32, u32)> =
            HashMap::with_capacity(glyph_count);
        let use_stack: bool = glyph_count <= ATLAS_STACK_PLACEMENTS;
        let mut stack_placements: [(usize, u32, u32); ATLAS_STACK_PLACEMENTS] =
            [(0, 0, 0); ATLAS_STACK_PLACEMENTS];
        let mut heap_placements: Vec<(usize, u32, u32)> = if use_stack {
            Vec::new()
        } else {
            Vec::with_capacity(glyph_count)
        };
        for (idx, (key, glyph)) in entries.iter().enumerate() {
            let gw: u32 = glyph.get_width();
            let gh: u32 = glyph.get_height();
            if cursor_x + gw + 1 > atlas_w {
                cursor_x = 1;
                cursor_y += shelf_h + 1;
                shelf_h = 0;
            }
            let _: Option<(u32, u32, u32, u32)> =
                regions.insert(*key, (cursor_x, cursor_y, gw, gh));
            if use_stack {
                stack_placements[idx] = (idx, cursor_x, cursor_y);
            } else {
                heap_placements.push((idx, cursor_x, cursor_y));
            }
            cursor_x += gw + 1;
            shelf_h = shelf_h.max(gh);
        }
        let atlas_h: u32 = (cursor_y + shelf_h + 1).max(ATLAS_MIN_DIM);
        let atlas_w_usize: usize = atlas_w as usize;
        let mut pixels: Vec<u8> = vec![0; atlas_w_usize * (atlas_h as usize) * 4];
        let placements: &[(usize, u32, u32)] = if use_stack {
            &stack_placements[..glyph_count]
        } else {
            heap_placements.as_slice()
        };
        for &(entry_idx, rx, ry) in placements.iter() {
            Self::blit_glyph_to_rgba(&entries[entry_idx].1, &mut pixels, atlas_w_usize, rx, ry);
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
            glyph_index,
            entries,
            entry_last_used,
            ascii_tables,
            revision,
            resident_bytes,
            ..
        } = self;
        glyph_index.fill(GLYPH_INDEX_EMPTY);
        entries.clear();
        entry_last_used.clear();
        ascii_tables.clear();
        *resident_bytes = 0;
        *revision = revision.wrapping_add(1);
    }
}
