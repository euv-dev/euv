use super::*;

/// Precomputed 256-entry linear-light RGBA channel blend lookup tables for translucent interior fills.
pub(crate) type RgbaLutQuad = ([u8; 256], [u8; 256], [u8; 256], [u8; 256]);

/// Read-only auxiliary frame slices passed to band rasterizers (glyphs, caret rects, rounded clips, and cached corner masks).
pub(crate) type BandAuxSlices<'a> = (
    &'a [PositionedGlyph],
    &'a [Rect],
    &'a [RoundedClipMask],
    &'a [CachedCornerMask],
);
