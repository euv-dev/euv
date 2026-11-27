use super::*;

/// Precomputed 256-entry linear-light RGBA channel blend lookup tables for translucent interior fills.
pub(crate) type RgbaLutQuad = ([u8; 256], [u8; 256], [u8; 256], [u8; 256]);

/// Prepared band-ready draw commands, cached text layouts, and active rounded clip masks for one frame range.
pub(crate) type PreparedFrameData = (
    Vec<PreppedCommand>,
    Vec<TextLayoutResult>,
    Vec<RoundedClipMask>,
);
