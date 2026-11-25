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
pub const FONT_FAMILY_WEB_SYSTEM: &str = "system-ui";

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
pub(crate) const CT_FONT_UI_SYSTEM: u32 = 2;
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub(crate) const CT_FONT_ORIENTATION_DEFAULT: u32 = 0;

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

#[cfg(target_os = "linux")]
pub(crate) const LIB_FREETYPE_SONAME: &str = "libfreetype.so.6";
#[cfg(target_os = "android")]
pub(crate) const LIB_FREETYPE_SONAME: &str = "libft2.so";
#[cfg(any(target_os = "linux", target_os = "android"))]
pub(crate) const SYM_FT_INIT: &str = "FT_Init_FreeType";
#[cfg(any(target_os = "linux", target_os = "android"))]
pub(crate) const SYM_FT_DONE: &str = "FT_Done_FreeType";
#[cfg(any(target_os = "linux", target_os = "android"))]
pub(crate) const RTLD_LAZY_FONT: i32 = 1;
#[cfg(target_os = "linux")]
pub(crate) const LINUX_FONT_CANDIDATES: [&str; 4] = [
    "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
    "/usr/share/fonts/TTF/DejaVuSans.ttf",
    "/usr/share/fonts/noto/NotoSans-Regular.ttf",
    "/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf",
];
#[cfg(target_os = "android")]
pub(crate) const ANDROID_FONT_CANDIDATES: [&str; 2] = [
    "/system/fonts/Roboto-Regular.ttf",
    "/system/fonts/NotoSansCJK-Regular.ttc",
];

#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_FONT_CANVAS_TAG: &str = "canvas";
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_FONT_CTX_2D: &str = "2d";
#[cfg(target_arch = "wasm32")]
pub(crate) const WEB_FONT_WHITE: &str = "#ffffff";
