use super::*;

/// A contour: one flattened sub-path of a [`Path`].
///
/// A contour is the unit of both filling and stroking. Filling
/// triangulates the set of contours together (so that holes are punched
/// correctly); stroking expands each contour on its own.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Contour {
    /// The flattened vertices.
    ///
    /// A closed contour repeats neither its first nor its last point; the
    /// closing edge is implicit.
    pub points: Vec<Point>,
    /// Whether the contour is explicitly closed.
    pub closed: bool,
}

/// A path flattened into contours.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FlatPath {
    /// The contours, in the order they appear in the source path.
    pub contours: Vec<Contour>,
}

/// Turns paths into GPU-ready triangle meshes.
///
/// The tessellator owns the scratch buffers it needs, so filling a chart
/// with many series does not allocate once per shape: the buffers are
/// reused across calls and only grow when a larger shape shows up.
///
/// A tessellator is not thread-safe; each renderer owns one.
#[derive(Clone, Debug, lombok_macros::Data)]
pub struct Tessellator {
    /// The tolerance used when flattening curves.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) tolerance: f64,
    /// Scratch storage for the flattened path.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) flat: FlatPath,
    /// Scratch storage for the working polygon during triangulation.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) working: Vec<Point>,
    /// Scratch storage for the output triangle indices.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) indices: Vec<usize>,
}

/// A vertex of a solid-colour mesh.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SolidVertex {
    /// The horizontal position, in canvas pixels.
    pub x: f32,
    /// The vertical position, in canvas pixels.
    pub y: f32,
    /// The packed colour, as `0xRRGGBBAA`.
    pub color: u32,
}

/// A solid-colour triangle mesh.
///
/// The mesh is uploaded to the GPU verbatim: the vertex buffer is the
/// `vertices` array reinterpreted as `SolidVertex`, and the index buffer
/// is the `indices` array. Keeping the two in a single struct lets the
/// backends share one upload path and one pipeline layout.
#[derive(Clone, Debug, Default, lombok_macros::Data, PartialEq)]
pub struct SolidMesh {
    /// The vertices.
    pub vertices: Vec<SolidVertex>,
    /// The triangle indices.
    pub indices: Vec<u32>,
}

/// A textured vertex, used for glyph-atlas text.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TextVertex {
    /// The horizontal position of the glyph quad, in canvas pixels.
    pub x: f32,
    /// The vertical position of the glyph quad, in canvas pixels.
    pub y: f32,
    /// The horizontal texture coordinate of the glyph's top-left corner.
    pub u: f32,
    /// The vertical texture coordinate of the glyph's top-left corner.
    pub v: f32,
    /// The packed colour, as `0xRRGGBBAA`.
    pub color: u32,
}

/// A textured triangle mesh.
#[derive(Clone, Debug, Default, lombok_macros::Data, PartialEq)]
pub struct TextMesh {
    /// The vertices.
    pub vertices: Vec<TextVertex>,
    /// The triangle indices.
    pub indices: Vec<u32>,
}
