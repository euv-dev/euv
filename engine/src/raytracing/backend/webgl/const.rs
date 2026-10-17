/// The GL stage name used in a WebGL compile diagnostic.
pub(crate) const BACKEND_WEBGL_STAGE_VERTEX: &str = "vertex";

/// The GL stage name used in a WebGL compile diagnostic.
pub(crate) const BACKEND_WEBGL_STAGE_FRAGMENT: &str = "fragment";

/// The sampler uniform name of the packed scene texture.
pub(crate) const RAYTRACE_GLSL_UNIFORM_SCENE: &str = "u_scene";

/// The sampler uniform name of the packed frame texture.
pub(crate) const RAYTRACE_GLSL_UNIFORM_FRAME: &str = "u_frame";

/// The texture-unit index the scene sampler reads.
pub(crate) const WEBGL_SAMPLER_UNIT_SCENE: i32 = 0;

/// The texture-unit index the frame sampler reads.
pub(crate) const WEBGL_SAMPLER_UNIT_FRAME: i32 = 1;

/// The texels one `vec4` slot occupies in a packed data texture.
pub(crate) const WEBGL_PACKED_TEXELS_PER_VEC4: usize = 4;

/// The number of entries `MAX_VIEWPORT_DIMS` reports.
pub(crate) const WEBGL_VIEWPORT_AXIS_COUNT: usize = 2;

/// The largest per-axis viewport dimension accepted from a driver, so a
/// context reporting an absurd pair still yields a conservative limit
/// rather than one no surface could ever satisfy.
pub(crate) const WEBGL_VIEWPORT_DIMENSION_CEILING: u32 = 65535;

/// The most pending `glGetError` codes one drain reads, so a context
/// that reports errors faster than they clear cannot spin forever.
pub(crate) const WEBGL_ERROR_DRAIN_LIMIT: usize = 32;

/// The edge of the smallest texture a context can be asked for, used as
/// the placeholder allocation so a context that never uploads a block
/// still owns a complete, bindable texture.
pub(crate) const WEBGL_PLACEHOLDER_TEXELS: i32 = 1;

/// The number of mip levels `texStorage2D` allocates. One level is
/// exactly what a packed data texture needs: it is sampled with
/// `texelFetch` at level zero and never minified, so a mip chain would
/// be pure memory.
pub(crate) const WEBGL_MIP_LEVELS_ONE: i32 = 1;

/// The `glGetError` code meaning no error is pending.
pub(crate) const WEBGL_ERROR_NO_ERROR: u32 = 0;

/// The `MAX_TEXTURE_SIZE` assumed when the implementation reports no
/// usable value, the smallest bound every WebGL 2 context guarantees.
pub(crate) const WEBGL_TEXTURE_SIZE_LIMIT_FALLBACK: u32 = 2048;

/// The per-axis `MAX_VIEWPORT_DIMS` assumed when the implementation
/// reports no usable pair, the bound every WebGL 2 context guarantees.
pub(crate) const WEBGL_VIEWPORT_DIMENSION_FALLBACK: u32 = 1024;

/// The `glGetError` name for an illegal enum value.
pub(crate) const WEBGL_ERROR_NAME_INVALID_ENUM: &str = "INVALID_ENUM";

/// The `glGetError` name for an out-of-range numeric argument.
pub(crate) const WEBGL_ERROR_NAME_INVALID_VALUE: &str = "INVALID_VALUE";

/// The `glGetError` name for an operation illegal in the current state.
pub(crate) const WEBGL_ERROR_NAME_INVALID_OPERATION: &str = "INVALID_OPERATION";

/// The `glGetError` name for a failed allocation.
pub(crate) const WEBGL_ERROR_NAME_OUT_OF_MEMORY: &str = "OUT_OF_MEMORY";

/// The `glGetError` name for a code this backend has no name for.
pub(crate) const WEBGL_ERROR_NAME_UNKNOWN: &str = "an unrecognised code";

/// The call name used when a drain reports an error raised before the
/// call being attributed, so the diagnostic does not blame the wrong one.
pub(crate) const WEBGL_ERROR_SOURCE_PENDING: &str = "the pending queue";

/// The call name used when a draw's drain reports an error raised by one
/// of the binding calls that preceded it.
pub(crate) const WEBGL_ERROR_SOURCE_BINDING: &str = "the binding state";

/// The error detail written when `createVertexArray` produced nothing.
pub(crate) const ERROR_WEBGL_VAO_REJECTED: &str = "createVertexArray returned null";

/// The error detail written when `createProgram` produced nothing.
pub(crate) const ERROR_WEBGL_PROGRAM_NULL: &str = "createProgram returned null";

/// The error detail written when `createTexture` produced nothing.
pub(crate) const ERROR_WEBGL_TEXTURE_NULL: &str = "createTexture returned null";

/// The error detail written when a packed block exceeds what any shape
/// this context can allocate holds.
pub(crate) const ERROR_WEBGL_BLOCK_TOO_LARGE: &str =
    "the packed block does not fit a texture this WebGL 2 context can allocate";

/// The error reported when the packed texture write was refused.
pub(crate) const ERROR_WEBGL_UPLOAD: &str = "the WebGL 2 packed texture write was refused";

/// The error reported when a packed block carries no slots at all.
pub(crate) const ERROR_WEBGL_EMPTY_BLOCK: &str = "the packed block carries no vec4 slots";

/// The error reported when a requested surface dimension exceeds what
/// this context accepts.
pub(crate) const ERROR_WEBGL_SURFACE_LIMIT: &str =
    "the requested surface dimension exceeds this WebGL 2 context's limit";

/// The error reported when the drawing buffer has no drawable area.
pub(crate) const ERROR_WEBGL_EMPTY_DRAWING_BUFFER: &str =
    "the WebGL 2 drawing buffer has no drawable area";
