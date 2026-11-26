/// Precomputed 256-entry linear-light RGBA channel blend lookup tables for translucent interior fills.
pub(crate) type RgbaLutQuad = ([u8; 256], [u8; 256], [u8; 256], [u8; 256]);
