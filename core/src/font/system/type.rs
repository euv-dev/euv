use super::*;

/// Function pointer signature for platform-host supplied anti-aliased glyph rasterizers (e.g. Android JNI `Paint`/`Canvas`).
pub type ExternalGlyphRasterizer = fn(GlyphKey) -> Option<RasterizedGlyph>;

/// FreeType `FT_New_Face` C ABI function pointer type.
#[cfg(any(target_os = "linux", target_os = "android"))]
pub(crate) type FtNewFaceFn =
    unsafe extern "C" fn(*mut c_void, *const c_char, isize, *mut *mut FtFaceRec) -> i32;

/// FreeType `FT_Set_Pixel_Sizes` C ABI function pointer type.
#[cfg(any(target_os = "linux", target_os = "android"))]
pub(crate) type FtSetPixelSizesFn = unsafe extern "C" fn(*mut FtFaceRec, u32, u32) -> i32;

/// FreeType `FT_Get_Char_Index` C ABI function pointer type.
#[cfg(any(target_os = "linux", target_os = "android"))]
pub(crate) type FtGetCharIndexFn = unsafe extern "C" fn(*mut FtFaceRec, usize) -> u32;

/// FreeType `FT_Load_Glyph` C ABI function pointer type.
#[cfg(any(target_os = "linux", target_os = "android"))]
pub(crate) type FtLoadGlyphFn = unsafe extern "C" fn(*mut FtFaceRec, u32, i32) -> i32;
