/// Native OS font rasterization backend active at runtime.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum FontRenderBackend {
    /// Built-in deterministic bitmap/stroke rasterizer guaranteeing bit-identical output everywhere.
    #[default]
    DeterministicBitmap,
    /// Apple CoreText + CoreGraphics native font engine (macOS, iOS, iPadOS).
    CoreText,
    /// Windows Win32 GDI `GetGlyphOutlineW` native font engine.
    Win32Gdi,
    /// FreeType runtime-loaded system font engine (Linux, Android).
    FreeType,
    /// Browser offscreen Canvas 2D system font rasterizer (Web).
    WebCanvas,
}
