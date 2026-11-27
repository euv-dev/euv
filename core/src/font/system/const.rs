/// Display family name for the built-in deterministic bitmap font.
pub const FONT_FAMILY_DETERMINISTIC: &str = "euv-deterministic";
/// Display family name for Apple CoreText system UI font.
pub const FONT_FAMILY_APPLE_SYSTEM: &str = "SF Pro / PingFang SC";
/// Display family name for Windows GDI system UI font.
pub const FONT_FAMILY_WINDOWS_SYSTEM: &str = "Segoe UI / Microsoft YaHei";
/// Display family name for Linux FreeType system UI font.
pub const FONT_FAMILY_LINUX_SYSTEM: &str = "DejaVu Sans / Noto Sans";
/// Display family name for Android system UI font.
pub const FONT_FAMILY_ANDROID_SYSTEM: &str = "Roboto / Noto Sans CJK";
/// Display family name for Web browser Canvas system font.
pub const FONT_FAMILY_WEB_SYSTEM: &str =
    "Inter, -apple-system, BlinkMacSystemFont, \"Segoe UI\", Roboto, sans-serif";
/// Display family name for Web browser Canvas monospace code font.
pub const FONT_FAMILY_WEB_MONO: &str =
    "\"JetBrains Mono\", \"SF Mono\", Menlo, Monaco, Consolas, ui-monospace, monospace";

pub(crate) const CJK_UNIFIED_START: u32 = 0x4E00;
pub(crate) const CJK_UNIFIED_END: u32 = 0x9FFF;
pub(crate) const CJK_EXT_A_START: u32 = 0x3400;
pub(crate) const CJK_EXT_A_END: u32 = 0x4DBF;
pub(crate) const CJK_PUNCT_START: u32 = 0x3000;
pub(crate) const CJK_PUNCT_END: u32 = 0x30FF;
pub(crate) const FULLWIDTH_START: u32 = 0xFF00;
pub(crate) const FULLWIDTH_END: u32 = 0xFFEF;
pub(crate) const HANGUL_START: u32 = 0xAC00;
pub(crate) const HANGUL_END: u32 = 0xD7AF;
pub(crate) const ATLAS_DEFAULT_WIDTH: u32 = 512;

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub(crate) const CT_FONT_UI_FIXED_PITCH: u32 = 1;
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub(crate) const CT_FONT_UI_SYSTEM: u32 = 2;
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub(crate) const CT_FONT_UI_EMPHASIZED_SYSTEM: u32 = 3;
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub(crate) const CT_FONT_TRAIT_ITALIC: u32 = 1 << 0;
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub(crate) const CT_FONT_TRAIT_BOLD: u32 = 1 << 1;
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub(crate) const CT_FONT_ORIENTATION_DEFAULT: u32 = 0;
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub(crate) const CORETEXT_FONT_CACHE_MAX: usize = 64;
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub(crate) const CORETEXT_FALLBACK_SLOTS: usize = 4;
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub(crate) const CORETEXT_SCRATCH_INIT_DIM: usize = 128;

#[cfg(target_os = "windows")]
pub(crate) const WIN32_FONT_CACHE_MAX: usize = 64;
#[cfg(target_os = "windows")]
pub(crate) const WIN_GGO_GRAY8_BITMAP: u32 = 6;
#[cfg(target_os = "windows")]
pub(crate) const WIN_FW_NORMAL: i32 = 400;
#[cfg(target_os = "windows")]
pub(crate) const WIN_FW_BOLD: i32 = 700;
#[cfg(target_os = "windows")]
pub(crate) const WIN_DEFAULT_CHARSET: u32 = 1;
#[cfg(target_os = "windows")]
pub(crate) const WIN_FONT_NAME_UTF16: [u16; 9] = [
    b'S' as u16,
    b'e' as u16,
    b'g' as u16,
    b'o' as u16,
    b'e' as u16,
    b' ' as u16,
    b'U' as u16,
    b'I' as u16,
    0,
];
#[cfg(target_os = "windows")]
pub(crate) const WIN_FONT_MONO_UTF16: [u16; 9] = [
    b'C' as u16,
    b'o' as u16,
    b'n' as u16,
    b's' as u16,
    b'o' as u16,
    b'l' as u16,
    b'a' as u16,
    b's' as u16,
    0,
];
#[cfg(target_os = "windows")]
pub(crate) const WIN_FONT_CJK_UTF16: [u16; 16] = [
    b'M' as u16,
    b'i' as u16,
    b'c' as u16,
    b'r' as u16,
    b'o' as u16,
    b's' as u16,
    b'o' as u16,
    b'f' as u16,
    b't' as u16,
    b' ' as u16,
    b'Y' as u16,
    b'a' as u16,
    b'H' as u16,
    b'e' as u16,
    b'i' as u16,
    0,
];

#[cfg(target_os = "linux")]
pub(crate) const LIB_FREETYPE_SONAME: &str = "libfreetype.so.6";
#[cfg(target_os = "android")]
pub(crate) const LIB_FREETYPE_SONAME: &str = "libft2.so";
#[cfg(any(target_os = "linux", target_os = "android"))]
pub(crate) const SYM_FT_INIT: &str = "FT_Init_FreeType";
#[cfg(any(target_os = "linux", target_os = "android"))]
pub(crate) const SYM_FT_DONE: &str = "FT_Done_FreeType";
#[cfg(any(target_os = "linux", target_os = "android"))]
pub(crate) const SYM_FT_NEW_FACE: &str = "FT_New_Face";
#[cfg(any(target_os = "linux", target_os = "android"))]
pub(crate) const SYM_FT_SET_PIXEL_SIZES: &str = "FT_Set_Pixel_Sizes";
#[cfg(any(target_os = "linux", target_os = "android"))]
pub(crate) const SYM_FT_GET_CHAR_INDEX: &str = "FT_Get_Char_Index";
#[cfg(any(target_os = "linux", target_os = "android"))]
pub(crate) const SYM_FT_LOAD_GLYPH: &str = "FT_Load_Glyph";
#[cfg(any(target_os = "linux", target_os = "android"))]
pub(crate) const FT_LOAD_RENDER: i32 = 4;
#[cfg(any(target_os = "linux", target_os = "android"))]
pub(crate) const FT_PIXEL_MODE_MONO: u8 = 1;
#[cfg(any(target_os = "linux", target_os = "android"))]
pub(crate) const FT_PIXEL_MODE_GRAY: u8 = 2;
#[cfg(any(target_os = "linux", target_os = "android"))]
pub(crate) const RTLD_LAZY_FONT: i32 = 1;
#[cfg(target_os = "linux")]
pub(crate) const LINUX_FONT_CANDIDATES: [&str; 6] = [
    "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
    "/usr/share/fonts/TTF/DejaVuSans.ttf",
    "/usr/share/fonts/noto/NotoSans-Regular.ttf",
    "/usr/share/fonts/truetype/noto/NotoSans-Regular.ttf",
    "/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf",
    "/usr/share/fonts/truetype/liberation2/LiberationSans-Regular.ttf",
];
#[cfg(target_os = "linux")]
pub(crate) const LINUX_BOLD_FONT_CANDIDATES: [&str; 6] = [
    "/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf",
    "/usr/share/fonts/TTF/DejaVuSans-Bold.ttf",
    "/usr/share/fonts/noto/NotoSans-Bold.ttf",
    "/usr/share/fonts/truetype/noto/NotoSans-Bold.ttf",
    "/usr/share/fonts/truetype/liberation/LiberationSans-Bold.ttf",
    "/usr/share/fonts/truetype/liberation2/LiberationSans-Bold.ttf",
];
#[cfg(target_os = "linux")]
pub(crate) const LINUX_MONO_FONT_CANDIDATES: [&str; 6] = [
    "/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf",
    "/usr/share/fonts/TTF/DejaVuSansMono.ttf",
    "/usr/share/fonts/truetype/liberation/LiberationMono-Regular.ttf",
    "/usr/share/fonts/truetype/liberation2/LiberationMono-Regular.ttf",
    "/usr/share/fonts/noto/NotoSansMono-Regular.ttf",
    "/usr/share/fonts/truetype/noto/NotoSansMono-Regular.ttf",
];
#[cfg(target_os = "linux")]
pub(crate) const LINUX_MONO_BOLD_FONT_CANDIDATES: [&str; 6] = [
    "/usr/share/fonts/truetype/dejavu/DejaVuSansMono-Bold.ttf",
    "/usr/share/fonts/TTF/DejaVuSansMono-Bold.ttf",
    "/usr/share/fonts/truetype/liberation/LiberationMono-Bold.ttf",
    "/usr/share/fonts/truetype/liberation2/LiberationMono-Bold.ttf",
    "/usr/share/fonts/noto/NotoSansMono-Bold.ttf",
    "/usr/share/fonts/truetype/noto/NotoSansMono-Bold.ttf",
];
#[cfg(target_os = "linux")]
pub(crate) const LINUX_CJK_FONT_CANDIDATES: [&str; 4] = [
    "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
    "/usr/share/fonts/truetype/noto/NotoSansCJK-Regular.ttc",
    "/usr/share/fonts/truetype/wqy/wqy-zenhei.ttc",
    "/usr/share/fonts/truetype/droid/DroidSansFallbackFull.ttf",
];
#[cfg(target_os = "android")]
pub(crate) const ANDROID_FONT_CANDIDATES: [&str; 4] = [
    "/system/fonts/Roboto-Regular.ttf",
    "/system/fonts/RobotoStatic-Regular.ttf",
    "/system/fonts/NotoSans-Regular.ttf",
    "/system/fonts/DroidSans.ttf",
];
#[cfg(target_os = "android")]
pub(crate) const ANDROID_BOLD_FONT_CANDIDATES: [&str; 3] = [
    "/system/fonts/Roboto-Bold.ttf",
    "/system/fonts/RobotoStatic-Bold.ttf",
    "/system/fonts/Roboto-Medium.ttf",
];
#[cfg(target_os = "android")]
pub(crate) const ANDROID_MONO_FONT_CANDIDATES: [&str; 2] = [
    "/system/fonts/DroidSansMono.ttf",
    "/system/fonts/CutiveMono.ttf",
];
#[cfg(target_os = "android")]
pub(crate) const ANDROID_MONO_BOLD_FONT_CANDIDATES: [&str; 2] = [
    "/system/fonts/DroidSansMono.ttf",
    "/system/fonts/Roboto-Bold.ttf",
];
#[cfg(target_os = "android")]
pub(crate) const ANDROID_CJK_FONT_CANDIDATES: [&str; 3] = [
    "/system/fonts/NotoSansCJK-Regular.ttc",
    "/system/fonts/NotoSansSC-Regular.otf",
    "/system/fonts/DroidSansFallback.ttf",
];

#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_FONT_CANVAS_TAG: &str = "canvas";
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_FONT_CTX_2D: &str = "2d";
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_FONT_WHITE: &str = "#ffffff";
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_FONT_SCRATCH_INIT_DIM: u32 = 128;
