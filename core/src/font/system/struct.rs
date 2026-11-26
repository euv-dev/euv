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

#[cfg(target_os = "windows")]
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub(crate) struct WinTextMetricW {
    pub(crate) tm_height: i32,
    pub(crate) tm_ascent: i32,
    pub(crate) tm_descent: i32,
    pub(crate) tm_internal_leading: i32,
    pub(crate) tm_external_leading: i32,
    pub(crate) tm_ave_char_width: i32,
    pub(crate) tm_max_char_width: i32,
    pub(crate) tm_weight: i32,
    pub(crate) tm_overhang: i32,
    pub(crate) tm_digitized_aspect_x: i32,
    pub(crate) tm_digitized_aspect_y: i32,
    pub(crate) tm_first_char: u16,
    pub(crate) tm_last_char: u16,
    pub(crate) tm_default_char: u16,
    pub(crate) tm_break_char: u16,
    pub(crate) tm_italic: u8,
    pub(crate) tm_underlined: u8,
    pub(crate) tm_struck_out: u8,
    pub(crate) tm_pitch_and_family: u8,
    pub(crate) tm_char_set: u8,
}

#[cfg(any(target_os = "linux", target_os = "android"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct FtBitmap {
    pub(crate) rows: u32,
    pub(crate) width: u32,
    pub(crate) pitch: i32,
    pub(crate) buffer: *const u8,
    pub(crate) num_grays: u16,
    pub(crate) pixel_mode: u8,
    pub(crate) palette_mode: u8,
    pub(crate) palette: *mut c_void,
}

#[cfg(any(target_os = "linux", target_os = "android"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct FtVector {
    pub(crate) x: isize,
    pub(crate) y: isize,
}

#[cfg(any(target_os = "linux", target_os = "android"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct FtGeneric {
    pub(crate) data: *mut c_void,
    pub(crate) finalizer: *mut c_void,
}

#[cfg(any(target_os = "linux", target_os = "android"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct FtBBox {
    pub(crate) x_min: isize,
    pub(crate) y_min: isize,
    pub(crate) x_max: isize,
    pub(crate) y_max: isize,
}

#[cfg(any(target_os = "linux", target_os = "android"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct FtGlyphMetrics {
    pub(crate) width: isize,
    pub(crate) height: isize,
    pub(crate) hori_bearing_x: isize,
    pub(crate) hori_bearing_y: isize,
    pub(crate) hori_advance: isize,
    pub(crate) vert_bearing_x: isize,
    pub(crate) vert_bearing_y: isize,
    pub(crate) vert_advance: isize,
}

#[cfg(any(target_os = "linux", target_os = "android"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct FtSizeMetrics {
    pub(crate) x_ppem: u16,
    pub(crate) y_ppem: u16,
    pub(crate) x_scale: isize,
    pub(crate) y_scale: isize,
    pub(crate) ascender: isize,
    pub(crate) descender: isize,
    pub(crate) height: isize,
    pub(crate) max_advance: isize,
}

#[cfg(any(target_os = "linux", target_os = "android"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct FtSizeRec {
    pub(crate) face: *mut c_void,
    pub(crate) generic: FtGeneric,
    pub(crate) metrics: FtSizeMetrics,
    pub(crate) internal: *mut c_void,
}

#[cfg(any(target_os = "linux", target_os = "android"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct FtGlyphSlotRec {
    pub(crate) library: *mut c_void,
    pub(crate) face: *mut c_void,
    pub(crate) next: *mut c_void,
    pub(crate) glyph_index: u32,
    pub(crate) generic: FtGeneric,
    pub(crate) metrics: FtGlyphMetrics,
    pub(crate) linear_hori_advance: isize,
    pub(crate) linear_vert_advance: isize,
    pub(crate) advance: FtVector,
    pub(crate) format: u32,
    pub(crate) bitmap: FtBitmap,
    pub(crate) bitmap_left: i32,
    pub(crate) bitmap_top: i32,
}

#[cfg(any(target_os = "linux", target_os = "android"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct FtFaceRec {
    pub(crate) num_faces: isize,
    pub(crate) face_index: isize,
    pub(crate) face_flags: isize,
    pub(crate) style_flags: isize,
    pub(crate) num_glyphs: isize,
    pub(crate) family_name: *const c_char,
    pub(crate) style_name: *const c_char,
    pub(crate) num_fixed_sizes: i32,
    pub(crate) available_sizes: *mut c_void,
    pub(crate) num_charmaps: i32,
    pub(crate) charmaps: *mut c_void,
    pub(crate) generic: FtGeneric,
    pub(crate) bbox: FtBBox,
    pub(crate) units_per_em: u16,
    pub(crate) ascender: i16,
    pub(crate) descender: i16,
    pub(crate) height: i16,
    pub(crate) max_advance_width: i16,
    pub(crate) max_advance_height: i16,
    pub(crate) underline_position: i16,
    pub(crate) underline_thickness: i16,
    pub(crate) glyph: *const FtGlyphSlotRec,
    pub(crate) size: *const FtSizeRec,
}

#[cfg(any(target_os = "linux", target_os = "android"))]
pub(crate) struct FreeTypeRuntime {
    pub(crate) regular_face: *mut FtFaceRec,
    pub(crate) bold_face: *mut FtFaceRec,
    pub(crate) cjk_face: *mut FtFaceRec,
    pub(crate) set_pixel_sizes: FtSetPixelSizesFn,
    pub(crate) get_char_index: FtGetCharIndexFn,
    pub(crate) load_glyph: FtLoadGlyphFn,
}
