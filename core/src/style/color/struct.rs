use super::*;

/// Compact 32-bit sRGB color with 8-bit red, green, blue, and alpha channels.
#[derive(Clone, Copy, Data, Debug, Default, Eq, Hash, New, Ord, PartialEq, PartialOrd)]
pub struct Color {
    /// Red channel in `[0, 255]`.
    #[get(type(copy))]
    pub(crate) r: u8,
    /// Green channel in `[0, 255]`.
    #[get(type(copy))]
    pub(crate) g: u8,
    /// Blue channel in `[0, 255]`.
    #[get(type(copy))]
    pub(crate) b: u8,
    /// Alpha channel in `[0, 255]`, where `255` is fully opaque.
    #[get(type(copy))]
    pub(crate) a: u8,
}
