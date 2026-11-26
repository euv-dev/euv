use super::*;

/// Contiguous RGBA8 pixel framebuffer handed directly to OS surfaces or GPU upload paths.
#[derive(Clone, Data, Debug, Default, Eq, PartialEq)]
pub struct Framebuffer {
    /// Physical width in pixels.
    #[get(type(copy))]
    pub(crate) width: u32,
    /// Physical height in pixels.
    #[get(type(copy))]
    pub(crate) height: u32,
    /// Packed RGBA8 pixel data (`width * height * 4` bytes).
    pub(crate) pixels: Vec<u8>,
}

/// Zero-copy borrowed horizontal row band of a `Framebuffer` enabling safe multi-threaded parallel rasterization.
#[derive(Debug, Eq, PartialEq)]
pub struct PixelBand<'a> {
    /// Full framebuffer physical width in pixels.
    pub(crate) width: u32,
    /// Starting physical Y row coordinate of this band inside the full framebuffer.
    pub(crate) y_offset: u32,
    /// Number of physical rows owned by this band.
    pub(crate) height: u32,
    /// Contiguous mutable RGBA8 byte slice for rows `[y_offset, y_offset + height)`.
    pub(crate) pixels: &'a mut [u8],
}

/// Dirty bounding region tracking damaged screen areas between frames.
#[derive(Clone, Copy, Data, Debug, Default, PartialEq)]
pub struct DamageRegion {
    /// Whether the entire surface must be redrawn.
    #[get(type(copy))]
    pub(crate) full_damage: bool,
    /// Accumulated dirty bounding box in physical pixels.
    #[get(skip)]
    pub(crate) rect: Option<Rect>,
}
