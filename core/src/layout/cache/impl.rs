use super::*;

/// Construction helpers for `AvailableSpace`.
impl AvailableSpace {
    /// Creates root-level `AvailableSpace` matching the window viewport.
    ///
    /// # Arguments
    ///
    /// - `Size` - Logical viewport dimensions.
    ///
    /// # Returns
    ///
    /// - `Self` - Root available space at `(0, 0)`.
    #[must_use]
    #[inline(always)]
    pub fn root(viewport: Size) -> Self {
        Self::new(
            viewport.get_width(),
            viewport.get_height(),
            viewport,
            0.0,
            0.0,
        )
    }

    /// Marks whether this node's auto width or height is stretched to fill the available slot.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source available space.
    /// - `bool` - Whether auto width stretches to `width`.
    /// - `bool` - Whether auto height stretches to `height`.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated available space with stretch flags applied.
    #[must_use]
    #[inline(always)]
    pub fn with_stretch(mut self, stretch_width: bool, stretch_height: bool) -> Self {
        self.set_stretch_width(stretch_width);
        self.set_stretch_height(stretch_height);
        self
    }

    /// Checks whether this cached layout constraint matches `incoming` size, viewport, and stretch flags.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Previously cached layout constraint.
    /// - `Self` - Incoming layout constraint for the current pass.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when cached node dimensions can be reused without recomputing layout.
    #[must_use]
    #[inline(always)]
    pub(crate) fn can_reuse(&self, incoming: Self) -> bool {
        self.get_stretch_width() == incoming.get_stretch_width()
            && self.get_stretch_height() == incoming.get_stretch_height()
            && (self.get_width() - incoming.get_width()).abs() <= 0.01
            && (self.get_height() - incoming.get_height()).abs() <= 0.01
            && (self.get_viewport().get_width() - incoming.get_viewport().get_width()).abs() <= 0.01
            && (self.get_viewport().get_height() - incoming.get_viewport().get_height()).abs()
                <= 0.01
    }

    /// Checks whether this cached layout constraint shares the same logical origin as `incoming`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Previously cached layout constraint.
    /// - `Self` - Incoming layout constraint for the current pass.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when subtree translation can be skipped completely.
    #[must_use]
    #[inline(always)]
    pub(crate) fn same_origin(&self, incoming: Self) -> bool {
        (self.get_origin_x() - incoming.get_origin_x()).abs() <= 0.001
            && (self.get_origin_y() - incoming.get_origin_y()).abs() <= 0.001
    }
}

/// Bounded 2-way set-associative text measurement cache operations.
impl TextSizeCache {
    /// Returns the cached bounding box for `(text, style, max_width)` or computes and caches it via `measure_fn`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable text-size cache.
    /// - `&str` - UTF-8 text string to measure.
    /// - `TextStyle` - Typography style parameters affecting measurement.
    /// - `Option<f32>` - Optional maximum wrap width in logical pixels.
    /// - `F` - Closure invoked only on a cache miss to measure the exact bounding `Size`.
    ///
    /// # Returns
    ///
    /// - `Size` - Exact measured bounding box.
    pub(crate) fn get_or_measure<F>(
        &mut self,
        text: &str,
        style: TextStyle,
        max_width: Option<f32>,
        measure_fn: F,
    ) -> Size
    where
        F: FnOnce(&str, TextStyle, Option<f32>) -> Size,
    {
        let Ok(text_len) = u32::try_from(text.len()) else {
            return measure_fn(text, style, max_width);
        };
        let (hash_a, hash_b): (u64, u64) = Self::hash_text_128(text.as_bytes());
        let font_size_bits: u32 = Self::normalize_f32_bits(style.get_font_size());
        let line_height_bits: u32 = Self::normalize_f32_bits(style.get_line_height());
        let letter_spacing_bits: u32 = Self::normalize_f32_bits(style.get_letter_spacing());
        let max_width_bits: u32 = match max_width {
            Some(w) if w.is_nan() => 0,
            Some(w) => Self::normalize_f32_bits(w),
            None => TEXT_SIZE_MAX_WIDTH_NONE_BITS,
        };
        let style_flags: u16 = Self::pack_style_flags(style);
        let style_mix: u64 = ((font_size_bits as u64) << 32)
            ^ (max_width_bits as u64)
            ^ ((line_height_bits as u64).rotate_left(17))
            ^ ((letter_spacing_bits as u64).rotate_left(41))
            ^ ((style_flags as u64).wrapping_mul(TEXT_HASH_PRIME_B));
        let set_index: usize = (((hash_a ^ style_mix).wrapping_mul(TEXT_HASH_FINAL_A) >> 16)
            as usize)
            & TEXT_SIZE_CACHE_SET_MASK;
        let base: usize = set_index * TEXT_SIZE_CACHE_WAYS;
        let probe: TextSizeEntry = TextSizeEntry {
            hash_a,
            hash_b,
            text_len,
            font_size_bits,
            line_height_bits,
            letter_spacing_bits,
            max_width_bits,
            width: 0.0,
            height: 0.0,
            style_flags,
        };
        let Self { entries } = self;
        if entries.is_empty() {
            entries.resize(TEXT_SIZE_CACHE_SLOTS, TextSizeEntry::default());
        }
        let way0: TextSizeEntry = entries[base];
        if Self::entry_matches(way0, probe) {
            return Size::new(way0.width, way0.height);
        }
        let way1: TextSizeEntry = entries[base + 1];
        if Self::entry_matches(way1, probe) {
            entries[base] = way1;
            entries[base + 1] = way0;
            return Size::new(way1.width, way1.height);
        }
        let measured: Size = measure_fn(text, style, max_width);
        entries[base + 1] = way0;
        entries[base] = TextSizeEntry {
            width: measured.get_width(),
            height: measured.get_height(),
            ..probe
        };
        measured
    }

    /// Checks whether a cached `TextSizeEntry` matches all text and typography key fields of `probe`.
    ///
    /// # Arguments
    ///
    /// - `TextSizeEntry` - Candidate cache entry.
    /// - `TextSizeEntry` - Probe entry carrying all key fields to match.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the entry is populated and matches every key component.
    #[inline(always)]
    fn entry_matches(entry: TextSizeEntry, probe: TextSizeEntry) -> bool {
        entry.style_flags == probe.style_flags
            && entry.text_len == probe.text_len
            && entry.hash_a == probe.hash_a
            && entry.hash_b == probe.hash_b
            && entry.max_width_bits == probe.max_width_bits
            && entry.font_size_bits == probe.font_size_bits
            && entry.line_height_bits == probe.line_height_bits
            && entry.letter_spacing_bits == probe.letter_spacing_bits
    }

    /// Computes two independent 64-bit hashes over `bytes` in a single 8-byte-chunked pass.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - UTF-8 input bytes.
    ///
    /// # Returns
    ///
    /// - `(u64, u64)` - Pair of independent 64-bit hashes with avalanche mixing.
    #[inline(always)]
    fn hash_text_128(bytes: &[u8]) -> (u64, u64) {
        let len: usize = bytes.len();
        let len_u64: u64 = len as u64;
        let mut h1: u64 = TEXT_HASH_SEED_A ^ len_u64;
        let mut h2: u64 = TEXT_HASH_SEED_B ^ len_u64.wrapping_mul(TEXT_HASH_PRIME_B);
        let mut offset: usize = 0;
        while offset + 8 <= len {
            let word: u64 = u64::from_le_bytes([
                bytes[offset],
                bytes[offset + 1],
                bytes[offset + 2],
                bytes[offset + 3],
                bytes[offset + 4],
                bytes[offset + 5],
                bytes[offset + 6],
                bytes[offset + 7],
            ]);
            h1 = (h1 ^ word).wrapping_mul(TEXT_HASH_PRIME_A);
            h2 = (h2.rotate_left(27) ^ word).wrapping_mul(TEXT_HASH_PRIME_B);
            offset += 8;
        }
        let mut tail: u64 = 0;
        let mut shift: u32 = 0;
        while offset < len {
            tail |= (bytes[offset] as u64) << shift;
            shift += 8;
            offset += 1;
        }
        h1 = (h1 ^ tail).wrapping_mul(TEXT_HASH_PRIME_A);
        h2 = (h2.rotate_left(27) ^ tail).wrapping_mul(TEXT_HASH_PRIME_B);
        h1 ^= h1 >> 33;
        h1 = h1.wrapping_mul(TEXT_HASH_FINAL_A);
        h1 ^= h1 >> 33;
        h2 ^= h2 >> 31;
        h2 = h2.wrapping_mul(TEXT_HASH_FINAL_B);
        h2 ^= h2 >> 31;
        (h1, h2)
    }

    /// Normalizes floating-point bits so `-0.0` and `+0.0` map identically.
    ///
    /// # Arguments
    ///
    /// - `f32` - Input scalar value.
    ///
    /// # Returns
    ///
    /// - `u32` - Normalized IEEE-754 bit pattern.
    #[inline(always)]
    fn normalize_f32_bits(value: f32) -> u32 {
        if value == 0.0 { 0 } else { value.to_bits() }
    }

    /// Packs layout-relevant discrete typography flags plus the occupied marker bit into `u16`.
    ///
    /// # Arguments
    ///
    /// - `TextStyle` - Source typography style.
    ///
    /// # Returns
    ///
    /// - `u16` - Packed flag bitfield with `TEXT_SIZE_ENTRY_OCCUPIED` set.
    #[inline(always)]
    fn pack_style_flags(style: TextStyle) -> u16 {
        let family_bits: u16 = match style.get_family() {
            FontFamily::Sans => 0,
            FontFamily::Monospace => 1,
            FontFamily::Serif => 2,
        };
        let weight_bits: u16 = match style.get_weight() {
            FontWeight::Light => 0,
            FontWeight::Normal => 1,
            FontWeight::Medium => 2,
            FontWeight::SemiBold => 3,
            FontWeight::Bold => 4,
        };
        let overflow_bits: u16 = match style.get_overflow() {
            TextOverflow::Wrap => 0,
            TextOverflow::Clip => 1,
            TextOverflow::Ellipsis => 2,
        };
        let italic_bit: u16 = if style.get_italic() { 1 } else { 0 };
        TEXT_SIZE_ENTRY_OCCUPIED
            | family_bits
            | (weight_bits << 2)
            | (overflow_bits << 5)
            | (italic_bit << 7)
    }
}
