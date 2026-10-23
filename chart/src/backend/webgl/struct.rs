use super::*;

/// The WebGL2 rendering backend for the chart.
///
/// This is the GPU fallback for browsers that do not expose WebGPU. It
/// replays the same display list as the WebGPU backend and the Canvas2D
/// backend, so the picture it produces is the same; only the plumbing
/// differs.
///
/// # Performance
///
/// The display list is retained between frames, and the backend takes
/// advantage of that in two ways.
///
/// The first is caching: the backend fingerprints the display list and
/// only re-tessellates and re-uploads when the fingerprint changes. An
/// unchanged chart therefore costs no vertex work at all — the GPU
/// buffers are simply drawn again.
///
/// The second is batching: every shape in the display list ends up in
/// one vertex buffer, in paint order, and is drawn with a handful of
/// draw calls. A steady-state frame of an ordinary chart issues two or
/// three draw calls, and none of them crosses the JavaScript boundary
/// more than a handful of times, because the vertex data is uploaded in
/// one go rather than one vertex at a time.
#[derive(Data)]
pub struct WebGl2ChartBackend {
    /// The canvas the chart draws into.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) canvas: HtmlCanvasElement,
    /// The WebGL2 rendering context every call goes through.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) context: WebGl2RenderingContext,
    /// The linked shader program, or `None` before it is built.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) program: Option<WebGl2Program>,
    /// The vertex array object that records the vertex layout.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) vertex_array: Option<WebGlVertexArrayObject>,
    /// The buffer holding the vertices of the whole frame.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) vertex_buffer: Option<WebGlBuffer>,
    /// The buffer holding the indices of the whole frame.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) index_buffer: Option<WebGlBuffer>,
    /// The texture the glyph atlas is uploaded into.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) atlas_texture: Option<WebGlTexture>,
    /// The texture holding the colour ramp of every gradient.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) ramp_texture: Option<WebGlTexture>,
    /// The texture holding the geometry of every gradient.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) parameter_texture: Option<WebGlTexture>,
    /// The images the display list refers to, in the order they appear.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) images: Vec<WebGl2Image>,
    /// The tessellator, reused across frames so its scratch buffers are
    /// allocated once.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) tessellator: Tessellator,
    /// The vertices of the whole frame, interleaved.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) vertices: Vec<f32>,
    /// The indices of the whole frame.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) indices: Vec<u32>,
    /// The batches to draw, in paint order.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) batches: Vec<WebGl2Batch>,
    /// The gradients the frame uses, one row each in the ramp texture.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) gradients: Vec<WebGl2Gradient>,
    /// The fingerprint of the display list the buffers currently hold.
    ///
    /// The fingerprint is a hash over everything in the list that can
    /// change what is drawn — the commands, the paths, the paints, the
    /// text, and the positions. A chart whose data changed but whose
    /// shape count stayed the same therefore still invalidates the
    /// cache.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) fingerprint: u64,
    /// Whether the GPU buffers currently hold the display list.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) built: bool,
    /// The logical width of the canvas, in CSS pixels.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) width: f64,
    /// The logical height of the canvas, in CSS pixels.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) height: f64,
    /// The device pixel ratio the canvas renders at.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) dpr: f64,
    /// The scissor rectangle currently in effect, in CSS pixels.
    ///
    /// WebGL2 has no path clipping, so a clip path is approximated by
    /// the rectangle that bounds it. The rectangle is tracked so that a
    /// frame only issues a scissor call when the clip actually changes.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) scissor: Option<WebGl2Scissor>,
    /// Whether the scissor rectangle has to be re-applied.
    ///
    /// A resize moves the framebuffer the rectangle is expressed in, so
    /// the rectangle is re-applied after one even when it did not
    /// change.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) scissor_dirty: bool,
    /// Which texture is bound to the primary texture unit.
    ///
    /// The atlas and the images share one unit, so the backend has to
    /// know what is bound to avoid rebinding it for every batch. The
    /// second component names the image, for an image binding.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) primary_texture: Option<(WebGl2BatchKind, usize)>,
}

/// The linked shader program and the uniform locations it exposes.
///
/// The uniform locations are resolved once, when the program is built:
/// resolving one costs a round trip into the driver that walks the
/// program's uniform table, and the names never change afterwards.
#[derive(Clone, Data)]
pub(crate) struct WebGl2Program {
    /// The linked program.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) program: WebGlProgram,
    /// The location of canvas size, in CSS pixels.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) resolution: Option<WebGlUniformLocation>,
    /// The location of the sampler that reads the atlas or the image.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) atlas: Option<WebGlUniformLocation>,
    /// The location of the sampler that reads the gradient ramps.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) ramp: Option<WebGlUniformLocation>,
    /// The location of the sampler that reads the gradient parameters.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) parameters: Option<WebGlUniformLocation>,
    /// The location of the number of gradient rows.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) gradient_count: Option<WebGlUniformLocation>,
}

/// The shading a run of vertices shares.
///
/// The kind, the colour, and the gradient row are the same for every
/// vertex of a shape, so they are carried together instead of being
/// passed around one by one.
#[derive(Clone, Copy, Data, Debug)]
pub(crate) struct WebGl2Style {
    /// The colour of the vertices, as normalised channels.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) color: [f32; 4],
    /// The shading kind of the vertices.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) kind: f32,
    /// The gradient row the vertices belong to.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) gradient: f32,
}

/// The rectangle of a texture a quad covers.
///
/// The rectangle is expressed in texture coordinates: the top-left
/// corner and the size of the area the quad reads.
#[derive(Clone, Copy, Data, Debug)]
pub(crate) struct WebGl2TextureRect {
    /// The horizontal coordinate of the top-left corner.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) u: f32,
    /// The vertical coordinate of the top-left corner.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) v: f32,
    /// The width of the rectangle.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) u_size: f32,
    /// The height of the rectangle.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) v_size: f32,
}

/// One contiguous run of triangles that costs a single draw call.
///
/// The batches are kept in paint order. A batch ends when the texture it
/// samples changes, or when the clip rectangle changes, so an ordinary
/// chart holds one or two of them.
#[derive(Clone, Copy, Data)]
pub(crate) struct WebGl2Batch {
    /// The texture the batch samples.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) kind: WebGl2BatchKind,
    /// The index of the batch's first vertex.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) first_index: u32,
    /// The number of indices in the batch.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) index_count: u32,
    /// The image the batch samples, for an image batch.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) image: usize,
    /// The scissor rectangle the batch is clipped to, in CSS pixels.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) clip: Option<WebGl2Scissor>,
}

/// A scissor rectangle, in CSS pixels, with the origin at the top left.
///
/// The rectangle is stored in the chart's own coordinate system; the
/// backend converts it into the framebuffer's bottom-left origin when
/// it hands it to WebGL.
#[derive(Clone, Copy, Data, Debug, PartialEq)]
pub(crate) struct WebGl2Scissor {
    /// The left edge of the rectangle.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) x: f64,
    /// The top edge of the rectangle.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) y: f64,
    /// The width of the rectangle.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) width: f64,
    /// The height of the rectangle.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) height: f64,
}

/// The GPU-side state of one image.
///
/// The image is uploaded into a texture of its own and drawn with the
/// same program the glyphs use, so an image costs exactly one draw call
/// on top of the frame.
#[derive(Data)]
pub(crate) struct WebGl2Image {
    /// The image element the texture was made from.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) element: HtmlImageElement,
    /// The texture holding the image, or `None` before the first upload.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) texture: Option<WebGlTexture>,
}

/// A small hash used to fingerprint the display list.
///
/// The hash is an FNV-1a variant over machine words. It is not a
/// cryptographic hash and does not need to be: it only has to notice
/// that the display list changed, and it has to do so without allocating
/// or crossing into JavaScript, because it runs on every frame.
#[derive(Data)]
pub(crate) struct DisplayHasher {
    /// The hash accumulated so far.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) hash: u64,
}
