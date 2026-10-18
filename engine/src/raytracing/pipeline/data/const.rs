/// The inclusive lower bound accepted by
/// [`RayTraceSettings::validate`] for `samples_per_axis`.
pub(crate) const RAYTRACE_SAMPLES_PER_AXIS_MIN: u32 = 1;

/// The inclusive upper bound accepted by
/// [`RayTraceSettings::validate`] for `samples_per_axis`.
pub(crate) const RAYTRACE_SAMPLES_PER_AXIS_MAX: u32 = 4;

/// The inclusive lower bound accepted by
/// [`RayTraceSettings::validate`] for `max_bounces`.
pub(crate) const RAYTRACE_MAX_BOUNCES_MIN: u32 = 0;

/// The inclusive upper bound accepted by
/// [`RayTraceSettings::validate`] for `max_bounces`.
pub(crate) const RAYTRACE_MAX_BOUNCES_MAX: u32 = 16;

/// The default `samples_per_axis` used by [`RayTraceSettings::new_default`].
pub(crate) const RAYTRACE_DEFAULT_SAMPLES_PER_AXIS: u32 = 1;

/// The default `max_bounces`, mirroring
/// [`RAYTRACE_DEFAULT_MAX_BOUNCES`](super::super::RAYTRACE_DEFAULT_MAX_BOUNCES).
pub(crate) const RAYTRACE_DEFAULT_MAX_BOUNCES_LIMIT: u32 = 4;

/// The number of `vec4` slots one packed scene node occupies.
pub(crate) const RAYTRACE_PACKED_NODE_VEC4S: usize = 3;

/// The number of `vec4` slots one packed primitive record occupies.
pub(crate) const RAYTRACE_PACKED_PRIMITIVE_VEC4S: usize = 8;

/// The number of `vec4` slots in the packed scene header.
pub(crate) const RAYTRACE_PACKED_SCENE_HEADER_VEC4S: usize = 1;

/// The number of `vec4` slots in the packed frame header.
pub(crate) const RAYTRACE_PACKED_FRAME_HEADER_VEC4S: usize = 8;

/// The number of `vec4` slots one packed light record occupies.
pub(crate) const RAYTRACE_PACKED_LIGHT_VEC4S: usize = 4;

/// The gamma exponent applied to the clamped linear color, applied
/// identically by the CPU renderer and by both shader backends.
pub(crate) const RAYTRACE_GAMMA: f64 = 2.2;

/// The largest finite `f32` magnitude, as an `f64` so the comparison needs
/// no narrowing. A component beyond this has no `f32` representation at all.
pub(crate) const RAYTRACE_MAX_F32_MAGNITUDE: f64 = f32::MAX as f64;

/// The largest integer a `f32` represents exactly. Packed indices above
/// this bound would be silently truncated by the shaders.
pub(crate) const RAYTRACE_MAX_EXACT_F32_INDEX: usize = 1 << 24;

/// The largest number of packed `f32` values a single GPU buffer or data
/// texture may address, chosen as the floor of the WebGPU
/// `maxStorageBufferBindingSize` default and of the WebGL2
/// `MAX_TEXTURE_SIZE` bound on the packed-data texture.
pub(crate) const RAYTRACE_MAX_PACKED_FLOATS: usize = 134_217_728;

/// Error message returned when `samples_per_axis` is out of range.
pub(crate) const RAYTRACE_ERROR_SAMPLES_PER_AXIS: &str =
    "RayTraceSettings::validate: samples_per_axis must be within 1..=4";

/// Error message returned when `max_bounces` is out of range.
pub(crate) const RAYTRACE_ERROR_MAX_BOUNCES: &str =
    "RayTraceSettings::validate: max_bounces must be within 0..=16";

/// Error message returned when a render dimension is zero.
pub(crate) const RAYTRACE_ERROR_ZERO_DIMENSION: &str =
    "render_raytrace_rgba: width and height must both be greater than zero";

/// Error message returned when a render dimension overflows the packed
/// buffer limit.
pub(crate) const RAYTRACE_ERROR_DIMENSION_OVERFLOW: &str =
    "render_raytrace_rgba: width * height exceeds the packed buffer limit";

/// Error message returned when the packed frame would exceed the GPU data
/// size limit.
pub(crate) const RAYTRACE_ERROR_FRAME_OVERFLOW: &str =
    "pack_raytrace_frame: packed frame exceeds the GPU data limit";

/// Error message returned when the packed scene would exceed the GPU data
/// size limit.
pub(crate) const RAYTRACE_ERROR_SCENE_OVERFLOW: &str =
    "RayTraceScene::encode_gpu: packed scene exceeds the GPU data limit";

/// Error message returned when a geometry component is not finite.
pub(crate) const RAYTRACE_ERROR_NON_FINITE: &str =
    "RayTraceScene::encode_gpu: geometry contains a non-finite component";

/// Error message returned when a geometry component overflows `f32`.
pub(crate) const RAYTRACE_ERROR_F32_RANGE: &str =
    "RayTraceScene::encode_gpu: geometry component is not representable as f32";

/// Error message returned when a primitive index does not fit in `f32`.
pub(crate) const RAYTRACE_ERROR_INDEX_RANGE: &str =
    "RayTraceScene::encode_gpu: primitive index is not representable as f32";

/// Error message returned when a camera component is not finite.
pub(crate) const RAYTRACE_ERROR_CAMERA_NON_FINITE: &str =
    "pack_raytrace_frame: camera contains a non-finite component";

/// Error message returned when the camera basis cannot be built.
pub(crate) const RAYTRACE_ERROR_CAMERA_BASIS: &str =
    "pack_raytrace_frame: camera basis is degenerate";

/// Error message returned when a light or ambient component is not finite.
pub(crate) const RAYTRACE_ERROR_LIGHT_NON_FINITE: &str =
    "pack_raytrace_frame: lights contain a non-finite component";

/// Error message returned when a light component overflows `f32`.
pub(crate) const RAYTRACE_ERROR_LIGHT_F32_RANGE: &str =
    "pack_raytrace_frame: light component is not representable as f32";

/// Error message returned when the camera field of view is outside the
/// open interval `(0, PI)`.
pub(crate) const RAYTRACE_ERROR_CAMERA_FOV: &str =
    "pack_raytrace_frame: fov must lie strictly within 0 < fov < PI";

/// Error message returned when the camera depth range is invalid.
pub(crate) const RAYTRACE_ERROR_CAMERA_RANGE: &str =
    "pack_raytrace_frame: depth range must satisfy 0 <= near < far";

/// Error message returned when a camera range component overflows `f32`.
pub(crate) const RAYTRACE_ERROR_CAMERA_F32_RANGE: &str =
    "pack_raytrace_frame: camera range is not representable as f32";
