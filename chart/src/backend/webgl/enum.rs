use super::*;

/// How the fragment stage shades one vertex.
///
/// The kind travels with the vertex instead of being a uniform, because
/// the chart puts fills, strokes, labels, and images into one buffer and
/// draws them in the order the display list asks for. A per-vertex kind
/// is what makes that possible: the whole frame is one draw call, and
/// the paint order is the order of the triangles.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum WebGl2VertexKind {
    /// A flat-coloured triangle: a fill, a stroke, an area strip, or a
    /// label background.
    ///
    /// The colour is the one the vertex carries.
    #[default]
    Solid,
    /// A glyph quad, read from the glyph atlas.
    ///
    /// The atlas holds white glyphs, so only the coverage the texture
    /// carries is used and the colour comes from the vertex.
    Text,
    /// A triangle painted with a gradient.
    ///
    /// The colour comes from the ramp row the vertex names, and the
    /// vertex's own alpha scales it.
    Gradient,
    /// An image quad, read from the image itself.
    ///
    /// Unlike a glyph, an image carries its own colour, so the texture
    /// is multiplied by the vertex's colour.
    Image,
}

/// Which texture a batch samples.
///
/// The chart draws in batches: a batch is a run of consecutive triangles
/// that share a texture and a clip rectangle, and therefore cost exactly
/// one draw call. Splitting the frame by texture is the only split the
/// backend needs, because the glyph atlas and the images cannot be
/// sampled from one binding.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum WebGl2BatchKind {
    /// The batch samples the glyph atlas.
    #[default]
    Atlas,
    /// The batch samples an image element.
    Image,
}

/// A gradient the frame uses.
///
/// Gradients are rare compared to plain colours, so they are collected
/// into a table instead of being baked into the vertices: every shape
/// that uses the same gradient points at the same row of the ramp
/// texture.
#[derive(Clone)]
pub(crate) enum WebGl2Gradient {
    /// A linear gradient, running from one point to another.
    Linear(LinearGradient),
    /// A radial gradient, running from a centre outwards.
    Radial(RadialGradient),
}
