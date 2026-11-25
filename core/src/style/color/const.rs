use super::*;

/// Fully transparent RGBA color.
pub const COLOR_TRANSPARENT: Color = Color {
    r: 0,
    g: 0,
    b: 0,
    a: 0,
};

/// Opaque pure black RGBA color.
pub const COLOR_BLACK: Color = Color {
    r: 0,
    g: 0,
    b: 0,
    a: 255,
};

/// Opaque pure white RGBA color.
pub const COLOR_WHITE: Color = Color {
    r: 255,
    g: 255,
    b: 255,
    a: 255,
};

/// Maximum 8-bit channel value as `f32`.
pub(crate) const CHANNEL_MAX_F32: f32 = 255.0;
