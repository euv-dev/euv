use super::*;

/// A float data texture holding one packed block.
///
/// The texture is allocated for the block that actually needs it: attach
/// creates a one-texel placeholder and the first upload that outgrows
/// it allocates the real size, so nothing reserves the whole addressable
/// budget up front. A later, larger block allocates one new texture,
/// swaps it in, and only then deletes the texture it replaced, which is
/// what makes a growth step transactional. The filters are `NEAREST` and
/// the wrap mode is `CLAMP_TO_EDGE` precisely because the block is read
/// with `texelFetch`, which must never interpolate or wrap. The texture
/// is only sampled, never rendered into, so no float colour-buffer
/// extension is involved.
#[derive(Clone, Data, Debug)]
pub(crate) struct WebGlRayTraceTexture {
    /// The GL texture object.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) texture: WebGlTexture,
    /// The texture width in texels, the row stride the shader divides by.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub(crate) width: i32,
    /// The texture height in texels.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub(crate) height: i32,
}

/// The WebGL 2 backend: a fullscreen triangle over two float data
/// textures.
///
/// The context, program, vertex array, and both textures are created
/// once. Each frame is one `texSubImage2D` of the frame block, a
/// viewport derived from the drawing buffer, and one `drawArrays`;
/// nothing in the steady-state path allocates a GL object or a Rust
/// vector. The scene block is uploaded only by
/// [`WebGlRayTraceBackend::upload_scene`], never by the draw.
///
/// The viewport is re-applied from the drawing buffer on every draw
/// because setting the canvas width or height resets it, and a resize
/// that leaves the viewport stale draws into a corner of the surface.
#[derive(Data, Debug)]
pub(crate) struct WebGlRayTraceBackend {
    /// The WebGL 2 rendering context every call goes through.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) context: WebGl2RenderingContext,
    /// The linked ray-trace program.
    #[get(pub(crate))]
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub(crate) program: WebGlProgram,
    /// The vertex array object the fullscreen triangle is drawn through.
    #[get(pub(crate))]
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub(crate) vao: WebGlVertexArrayObject,
    /// The packed scene data texture.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) scene_texture: WebGlRayTraceTexture,
    /// The packed frame data texture.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) frame_texture: WebGlRayTraceTexture,
    /// The cached sampler location of `u_scene`.
    #[get(pub(crate))]
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub(crate) scene_uniform: WebGlUniformLocation,
    /// The cached sampler location of `u_frame`.
    #[get(pub(crate))]
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub(crate) frame_uniform: WebGlUniformLocation,
    /// The zero-extended upload staging buffer for the scene texture,
    /// reused so a steady scene costs no allocation.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) scene_scratch: Vec<f32>,
    /// The zero-extended upload staging buffer for the frame texture.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) frame_scratch: Vec<f32>,
    /// The largest surface dimension this context accepts, the smaller
    /// of the texture and viewport bounds.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub(crate) dimension_limit_value: u32,
}
