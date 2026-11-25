/// Primitive kind discriminant encoded inside each `GpuQuadInstance`.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GpuPrimitiveKind {
    /// Solid or bordered rounded rectangle.
    #[default]
    RoundedRect = 0,
    /// Two-stop linear gradient quad.
    Gradient = 1,
    /// Analytical SDF box shadow quad.
    Shadow = 2,
    /// Textured glyph atlas quad.
    Glyph = 3,
    /// Textured RGBA image or hybrid tile quad.
    TexturedQuad = 4,
    /// Filled SDF circle quad.
    Circle = 5,
}
