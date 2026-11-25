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
