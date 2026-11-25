use super::*;

/// Metadata describing the active native or deterministic font rasterization provider.
#[derive(Clone, Debug, Eq, Getter, New, PartialEq)]
pub struct SystemFontInfo {
    /// Active font backend discriminant.
    #[get(type(copy))]
    pub(crate) backend: FontRenderBackend,
    /// Resolved human-readable system font family name.
    pub(crate) family_name: String,
    /// Whether full Unicode / CJK glyph shaping is supported.
    #[get(type(copy))]
    pub(crate) supports_unicode: bool,
    /// Whether smooth subpixel/alpha anti-aliasing is enabled.
    #[get(type(copy))]
    pub(crate) supports_antialiasing: bool,
}

/// Cross-platform native system font provider backed by direct OS font APIs.
#[derive(Clone, Debug, Getter)]
pub struct NativeSystemFont {
    /// Probed system font capability descriptor.
    pub(crate) info: SystemFontInfo,
    /// When `true`, bypasses OS font engines for bit-identical deterministic output.
    #[get(type(copy))]
    pub(crate) deterministic_only: bool,
}

/// Shelf-packed alpha8/RGBA8 glyph texture atlas ready for GPU texture upload.
#[derive(Clone, Debug, Getter, New)]
pub struct PackedGlyphAtlas {
    /// Texture width in pixels.
    #[get(type(copy))]
    pub(crate) width: u32,
    /// Texture height in pixels.
    #[get(type(copy))]
    pub(crate) height: u32,
    /// Packed RGBA8 pixel data (white RGB with glyph alpha coverage).
    pub(crate) pixels: Vec<u8>,
    /// Atlas UV rectangles `(x, y, w, h)` per cached `GlyphKey`.
    pub(crate) regions: HashMap<GlyphKey, (u32, u32, u32, u32)>,
    /// Revision number matching the source `GlyphAtlas`.
    #[get(type(copy))]
    pub(crate) revision: u64,
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct CtRange {
    pub(crate) location: isize,
    pub(crate) length: isize,
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct CtPoint {
    pub(crate) x: f64,
    pub(crate) y: f64,
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct CtSize {
    pub(crate) width: f64,
    pub(crate) height: f64,
}

#[cfg(target_os = "windows")]
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub(crate) struct WinFixed {
    pub(crate) fract: u16,
    pub(crate) value: i16,
}

#[cfg(target_os = "windows")]
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub(crate) struct WinMat2 {
    pub(crate) em11: WinFixed,
    pub(crate) em12: WinFixed,
    pub(crate) em21: WinFixed,
    pub(crate) em22: WinFixed,
}

#[cfg(target_os = "windows")]
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub(crate) struct WinGlyphMetrics {
    pub(crate) black_box_x: u32,
    pub(crate) black_box_y: u32,
    pub(crate) glyph_origin_x: i32,
    pub(crate) glyph_origin_y: i32,
    pub(crate) cell_inc_x: i16,
    pub(crate) cell_inc_y: i16,
}
