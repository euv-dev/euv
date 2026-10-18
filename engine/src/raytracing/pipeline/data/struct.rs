use super::*;

/// Ray-tracing quality settings shared by the CPU reference renderer and
/// both GPU shader backends.
///
/// The values are packed into the GPU frame header so the CPU path and the
/// shaders sample and bounce identically.
#[derive(Clone, Copy, Data, Debug, New, PartialEq)]
pub struct RayTraceSettings {
    /// Supersampling grid resolution per axis. `1` traces one ray per
    /// pixel; `4` traces sixteen and averages them. Valid range 1..=4.
    #[get(type(copy))]
    pub samples_per_axis: u32,
    /// Maximum number of secondary bounces after the primary hit. `0`
    /// still shades the primary hit. Valid range 0..=16.
    #[get(type(copy))]
    pub max_bounces: u32,
}

/// A validated, immutable description of one frame's camera.
///
/// Building a [`FrameBasis`] performs every check the packed frame header
/// and the CPU pixel loop both need: the settings are in range, the
/// dimensions are usable, the field of view lies in `(0, PI)`, the depth
/// range satisfies `0 <= near < far` and survives the `f32` narrowing, and
/// the camera basis is non-degenerate. Holding the result means neither
/// path has to re-derive the basis or re-run the validation, and neither
/// can accept a frame the other rejects.
#[derive(Clone, Copy, Data, Debug, PartialEq)]
pub(crate) struct FrameBasis {
    /// The world-space eye position.
    #[get(pub(crate), type(copy))]
    pub(crate) eye: Vector3D,
    /// The unit forward direction, from the eye toward the target.
    #[get(pub(crate), type(copy))]
    pub(crate) forward: Vector3D,
    /// The unit right direction, `forward` crossed with the camera up.
    #[get(pub(crate), type(copy))]
    pub(crate) right: Vector3D,
    /// The unit up direction, `right` crossed with `forward`.
    #[get(pub(crate), type(copy))]
    pub(crate) up: Vector3D,
    /// `tan(fov / 2)`, the factor that makes the rendered vertical field
    /// of view exactly `fov`.
    #[get(pub(crate), type(copy))]
    pub(crate) tan_half_fov: f64,
    /// `width / height`.
    #[get(pub(crate), type(copy))]
    pub(crate) aspect: f64,
    /// The frame width in pixels.
    #[get(pub(crate), type(copy))]
    pub(crate) width: u32,
    /// The frame height in pixels.
    #[get(pub(crate), type(copy))]
    pub(crate) height: u32,
    /// The near clipping distance used as the primary ray's `t_min`.
    #[get(pub(crate), type(copy))]
    pub(crate) near_t: f64,
    /// The far clipping distance used as the primary ray's `t_max`.
    #[get(pub(crate), type(copy))]
    pub(crate) far_t: f64,
}
