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

/// Cached Apple CoreText `CTFontRef` handle paired with pre-queried vertical metrics and lazy cascaded fallbacks.
#[cfg(any(target_os = "macos", target_os = "ios"))]
#[derive(Clone, Copy)]
pub(crate) struct CoreTextFontCacheEntry {
    /// Packed `(size_tenths | bold<<16 | monospace<<17 | italic<<18)` style signature.
    pub(crate) style_sig: u32,
    /// Retained CoreText `CTFontRef` handle released on eviction or thread exit.
    pub(crate) font: *mut c_void,
    /// Pre-queried clamped typographic ascent in points.
    pub(crate) ascent: f64,
    /// Pre-queried clamped typographic descent in points.
    pub(crate) descent: f64,
    /// Lazily populated cascaded fallback `CTFontRef` slots (e.g. CJK / symbol fonts), null until a glyph misses `font`.
    pub(crate) fallback_fonts: [*mut c_void; CORETEXT_FALLBACK_SLOTS],
}

/// Thread-local cache of Apple CoreText `CTFontRef` handles, singleton grayscale `CGColorSpaceRef`, and reusable bitmap context.
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub(crate) struct CoreTextFontCache {
    /// Singleton device-gray `CGColorSpaceRef` reused across all glyph rasterizations on this thread.
    pub(crate) gray_space: *mut c_void,
    /// Persistent scratch `CGBitmapContextRef` backed by `scratch_buf`, eliminating per-glyph context creation FFI calls.
    pub(crate) scratch_ctx: *mut c_void,
    /// Backing alpha8 pixel storage for `scratch_ctx`.
    pub(crate) scratch_buf: Vec<u8>,
    /// Current pixel width of `scratch_ctx`.
    pub(crate) scratch_w: usize,
    /// Current pixel height of `scratch_ctx`.
    pub(crate) scratch_h: usize,
    /// Bounded table of cached base `CTFontRef` handles (`<= CORETEXT_FONT_CACHE_MAX`).
    pub(crate) fonts: Vec<CoreTextFontCacheEntry>,
}

/// Cached Windows Win32 `HFONT` handle paired with pre-queried font metrics.
#[cfg(target_os = "windows")]
#[derive(Clone, Copy)]
pub(crate) struct Win32FontCacheEntry {
    /// Packed `(size_px | bold<<16 | italic<<17 | face_kind<<18)` style signature.
    pub(crate) style_sig: u32,
    /// Retained Win32 `HFONT` handle deleted on eviction or thread exit.
    pub(crate) hfont: *mut c_void,
    /// Pre-queried clamped typographic ascent in pixels.
    pub(crate) ascent: i32,
    /// Pre-queried clamped typographic descent in pixels.
    pub(crate) descent: i32,
}

/// Thread-local cache of Windows Win32 memory `HDC`, `HFONT` handles, and outline scratch buffer.
#[cfg(target_os = "windows")]
pub(crate) struct Win32FontCache {
    /// Persistent compatible memory device context reused across glyph misses.
    pub(crate) hdc: *mut c_void,
    /// Bounded table of cached `HFONT` handles (`<= WIN32_FONT_CACHE_MAX`).
    pub(crate) fonts: Vec<Win32FontCacheEntry>,
    /// Reusable `GetGlyphOutlineW` byte scratch buffer eliminating per-glyph allocations.
    pub(crate) raw_buf: Vec<u8>,
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

/// Lazily loaded FreeType face slot tracking probe state and most recently applied pixel size.
#[cfg(any(target_os = "linux", target_os = "android"))]
#[derive(Clone, Copy)]
pub(crate) struct FtLazyFace {
    /// Whether candidate font path probing has already been attempted for this slot.
    pub(crate) probed: bool,
    /// Opened FreeType face pointer, or null when unprobed or unavailable.
    pub(crate) face: *mut FtFaceRec,
    /// Most recently applied pixel size on `face` to skip redundant `FT_Set_Pixel_Sizes` calls.
    pub(crate) last_size_px: u32,
}

/// Dynamically loaded FreeType 2 runtime holding the primary face and lazily resolved style/CJK fallback faces.
#[cfg(any(target_os = "linux", target_os = "android"))]
pub(crate) struct FreeTypeRuntime {
    /// Initialized `FT_Library` handle retained for on-demand `FT_New_Face` calls.
    pub(crate) library: *mut c_void,
    /// Resolved `FT_New_Face` function pointer for lazy style and CJK fallback face opening.
    pub(crate) new_face: FtNewFaceFn,
    /// Primary regular Latin face opened on cold initialization.
    pub(crate) regular: FtLazyFace,
    /// Lazily opened bold sans face (opened only on first bold glyph).
    pub(crate) bold: FtLazyFace,
    /// Lazily opened monospace face (opened only on first monospace glyph).
    pub(crate) mono: FtLazyFace,
    /// Lazily opened bold monospace face (opened only on first bold monospace glyph).
    pub(crate) mono_bold: FtLazyFace,
    /// Lazily opened CJK / Unicode fallback face (opened only when primary face misses a codepoint).
    pub(crate) cjk: FtLazyFace,
    /// Resolved `FT_Set_Pixel_Sizes` function pointer.
    pub(crate) set_pixel_sizes: FtSetPixelSizesFn,
    /// Resolved `FT_Get_Char_Index` function pointer.
    pub(crate) get_char_index: FtGetCharIndexFn,
    /// Resolved `FT_Load_Glyph` function pointer.
    pub(crate) load_glyph: FtLoadGlyphFn,
}

/// Thread-local reusable offscreen `<canvas>` and 2D context for Web glyph rasterization.
#[cfg(target_arch = "wasm32")]
pub(crate) struct WebFontScratch {
    /// Lazily created offscreen canvas element.
    pub(crate) canvas: Option<HtmlCanvasElement>,
    /// Cached 2D rendering context bound to `canvas`.
    pub(crate) ctx2d: Option<CanvasRenderingContext2d>,
    /// Current allocated canvas pixel width.
    pub(crate) width: u32,
    /// Current allocated canvas pixel height.
    pub(crate) height: u32,
    /// Last CSS `font` string applied to `ctx2d`.
    pub(crate) last_font: String,
}
