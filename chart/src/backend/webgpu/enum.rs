/// A batch is one contiguous run of triangles that share a pipeline and a
/// bind group, and therefore cost exactly one draw call. The backend groups
/// the display list into batches of these two kinds, so a steady-state
/// frame issues a handful of draw calls no matter how many shapes the chart
/// holds.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum WebGpuBatchKind {
    /// Flat-coloured triangles: fills, strokes, area strips, and label
    /// backgrounds.
    ///
    /// These all live in one vertex buffer and are drawn with the solid
    /// pipeline, which needs no texture bindings.
    #[default]
    Solid,
    /// Textured triangles: glyph quads read the glyph atlas, and image
    /// quads read the image itself.
    ///
    /// These live in one vertex buffer and are drawn with the textured
    /// pipeline, which samples whatever texture is bound to group one.
    Textured,
}
