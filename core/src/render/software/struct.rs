use super::*;

/// High-performance CPU tile and analytical-SDF software rasterizer with zero GPU dependencies.
#[derive(Clone, Data, Debug, Default)]
pub struct SoftwareRenderer {
    /// Persistent RGBA8 framebuffer reused across frames.
    pub(crate) framebuffer: Framebuffer,
    /// Shared glyph rasterization atlas.
    pub(crate) glyph_atlas: GlyphAtlas,
    /// Native or deterministic font rasterization provider.
    pub(crate) font_provider: NativeSystemFont,
    /// Deterministic text measurer matching `LayoutEngine` metrics.
    #[get(type(copy))]
    pub(crate) measurer: DeterministicTextMeasurer,
    /// Active physical clip-rectangle stack.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) clip_stack: Vec<Rect>,
    /// Active anti-aliasing policy.
    #[get(type(copy))]
    pub(crate) antialiasing: AntialiasingMode,
    /// Tile edge length in physical pixels for damage tile accounting.
    #[get(type(copy))]
    pub(crate) tile_size: u32,
}
