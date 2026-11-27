use super::*;

/// Construction and compact 64-bit packing helpers for `GlyphKey`.
impl GlyphKey {
    /// Losslessly packs all `GlyphKey` fields into the low 40 bits of a `u64`, preserving lexicographical `(codepoint, size_tenths, bold, monospace, italic)` order.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Quantized glyph lookup key.
    ///
    /// # Returns
    ///
    /// - `u64` - Bijective 40-bit packed key representation (`0..2^40`).
    #[inline(always)]
    #[must_use]
    pub const fn pack_u64(&self) -> u64 {
        let Self {
            codepoint,
            size_tenths,
            bold,
            monospace,
            italic,
        } = *self;
        ((codepoint as u64) << 19)
            | ((size_tenths as u64) << 3)
            | ((bold as u64) << 2)
            | ((monospace as u64) << 1)
            | (italic as u64)
    }

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

    /// Builds a quantized `GlyphKey` capturing size, weight, monospace family, and italic slant from `style`.
    ///
    /// # Arguments
    ///
    /// - `char` - Character to rasterize.
    /// - `f32` - Effective physical font size in pixels.
    /// - `TextStyle` - Active typography style.
    ///
    /// # Returns
    ///
    /// - `Self` - Quantized cache key distinguishing sans, monospace, bold, and italic faces.
    #[must_use]
    pub fn from_text_style(codepoint: char, physical_size: f32, style: TextStyle) -> Self {
        let clamped: f32 = physical_size.clamp(6.0, 256.0);
        let size_tenths: u16 = (clamped * 10.0 + 0.5) as u16;
        Self {
            codepoint,
            size_tenths,
            bold: style.get_weight().is_bold(),
            monospace: style.get_family() == FontFamily::Monospace,
            italic: style.get_italic(),
        }
    }
}

/// Single-register 64-bit equality comparison for `GlyphKey`.
impl PartialEq for GlyphKey {
    /// Compares two glyph keys via their bijective 64-bit packed integer representation.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Left glyph key.
    /// - `&Self` - Right glyph key.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when all glyph key fields match.
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.pack_u64() == other.pack_u64()
    }
}

/// Single-register 64-bit partial ordering for `GlyphKey`.
impl PartialOrd for GlyphKey {
    /// Compares two glyph keys via their order-preserving packed 64-bit integers.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Left glyph key.
    /// - `&Self` - Right glyph key.
    ///
    /// # Returns
    ///
    /// - `Option<core::cmp::Ordering>` - Total lexicographical ordering of the two keys.
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Single-register 64-bit total ordering for `GlyphKey`.
impl Ord for GlyphKey {
    /// Compares two glyph keys via their order-preserving packed 64-bit integers.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Left glyph key.
    /// - `&Self` - Right glyph key.
    ///
    /// # Returns
    ///
    /// - `core::cmp::Ordering` - Lexicographical ordering across `(codepoint, size_tenths, bold, monospace, italic)`.
    #[inline(always)]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.pack_u64().cmp(&other.pack_u64())
    }
}

/// Single-word 64-bit hashing for `GlyphKey`.
impl Hash for GlyphKey {
    /// Feeds the single bijective 64-bit packed glyph key directly to `state`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Quantized glyph lookup key.
    /// - `&mut H` - Target hasher state.
    #[inline(always)]
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        state.write_u64(self.pack_u64());
    }
}
