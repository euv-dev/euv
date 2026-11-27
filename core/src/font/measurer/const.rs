/// Sub-pixel float-rounding tolerance applied to text wrap width limits so 1-ULP chrome or trailing-space round-trips never trigger false wraps.
pub(crate) const WRAP_WIDTH_EPSILON: f32 = 0.05;
