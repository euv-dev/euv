use super::*;

/// A light source contributing illumination to a shaded point.
///
/// The fields have meanings that depend on [`LightType`]. Directions point
/// **toward the light source** except for the spot cone axis, which points
/// outward along the beam; see [`LightType`] for the full convention.
///
/// - For [`LightType::Directional`], `position` is unused and `direction` is
///   the unit vector from the surface toward the light.
/// - For [`LightType::Point`], `position` is the world position of the light
///   and `direction` is unused.
/// - For [`LightType::Spot`], `position` is the lamp position, `direction`
///   is the cone axis pointing outward along the beam, and `spot_cos`
///   carries `cos(half_angle)` of the cone.
///
/// A light is never mutated during shading:
/// [`LightingUniforms::shade_with_visibility`] reads it and derives
/// per-point quantities into locals.
#[derive(Clone, Data, Debug, New, PartialEq)]
pub struct Light {
    /// The kind of light source (directional, point, or spot).
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub kind: LightType,
    /// The world position of the light.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub position: Vector3D,
    /// Unit direction. Toward the light for [`LightType::Directional`];
    /// the outward cone axis for [`LightType::Spot`]; unused for
    /// [`LightType::Point`].
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub direction: Vector3D,
    /// RGB intensity multiplier applied to the contribution.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) color: Vector3D,
    /// Overall intensity scalar in the range 0.0..=infinity.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub intensity: f64,
    /// Inverse-square falloff factor for point and spot lights.
    ///
    /// `pub` rather than `pub(crate)`: the getter is an out-of-crate
    /// consumer's entry point (`tests/`, and the ray tracer's data stage),
    /// and a narrowed field behind a public getter would silently
    /// re-expose it.
    #[get(type(copy))]
    pub falloff: f64,
    /// `cos(half_angle)` for spot lights; zero for other kinds.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub spot_cos: f64,
}

/// A material describing how a surface responds to illumination.
#[derive(Clone, Data, Debug, New, PartialEq)]
pub struct Material {
    /// The shading model applied during evaluation.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) kind: MaterialKind,
    /// The base reflectance color in the range 0.0..=1.0 per channel.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) albedo: Vector3D,
    /// Specular intensity multiplier in the range 0.0..=1.0.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) specular: f64,
    /// Phong specular exponent; larger values produce tighter highlights.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) shininess: f64,
    /// Self-illumination term used when a surface is also a light source
    /// (e.g. an emissive sphere visible through ray tracing).
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) emissive: Vector3D,
}

/// One light's contribution at a single shaded point, resolved once and
/// then shared by every term that light feeds.
///
/// A [`Light`] stores the **authored** values: the intensity before any
/// falloff, and — for a spot light — the outward cone axis rather than the
/// direction toward the lamp. Shading needs different numbers: the
/// surface-to-light direction and the intensity after falloff. Deriving
/// them into locals keeps the [`Light`] itself un-cloned, but it leaves
/// each term reading a bare pair of `Vector3D` / `f64` parameters whose
/// roles no longer match the field names they came from.
///
/// A `LightSample` names the derived pair next to the colour they apply
/// to, so [`lambert_term`] and [`phong_specular_term`] take one borrowed
/// argument instead of three loose ones and neither rebuilds what the
/// shading loop already measured. The caller still decides the values:
/// [`compute_lambert`] samples the authored direction and intensity
/// directly, while [`LightingUniforms::shade_with_visibility`] samples the
/// derived direction and the falloff-attenuated intensity.
///
/// Every field is a `Copy` scalar or vector, so a sample is built on the
/// stack and never allocates.
#[derive(Clone, Copy, Debug, Getter)]
pub(crate) struct LightSample {
    /// The light's RGB color.
    #[get(pub(crate), type(copy))]
    pub(crate) color: Vector3D,
    /// The intensity, already scaled by any falloff the caller applied.
    #[get(pub(crate), type(copy))]
    pub(crate) intensity: f64,
    /// The unit direction from the shaded surface toward the light.
    #[get(pub(crate), type(copy))]
    pub(crate) direction: Vector3D,
}

/// The bounded segment a shadow ray travels: a start, a unit direction, and
/// the length of the leg it covers.
///
/// [`soft_shadow_factor`] measures all three once per light and then tests
/// every occluder sphere against the same segment, which is what keeps the
/// open-interval bounds identical across the loop — [`EPSILON`] at the near
/// end and `distance - EPSILON` at the far end, so a root landing exactly
/// on an endpoint still counts as a miss.
///
/// The triple is declared here rather than borrowed from the ray tracer. A
/// ray-tracing `Ray` carries `t_min` / `t_max` / `depth` for scene
/// traversal, while a shadow leg here is defined by the two endpoints the
/// light and the shaded point supply; naming that pair as one value keeps
/// `lighting` free of a dependency on `raytracing` and of the cycle a
/// shared `Ray` would risk forming.
#[derive(Clone, Copy, Debug, Getter)]
pub(crate) struct ShadowSegment {
    /// The shaded point the segment starts from.
    #[get(pub(crate), type(copy))]
    pub(crate) origin: Vector3D,
    /// The unit direction from `origin` toward the light.
    #[get(pub(crate), type(copy))]
    pub(crate) direction: Vector3D,
    /// The distance from `origin` to the light.
    #[get(pub(crate), type(copy))]
    pub(crate) distance: f64,
}

/// Bundles of uniforms supplied to lighting and shading routines.
#[derive(Clone, Data, Debug, New, PartialEq)]
pub struct LightingUniforms {
    /// All lights contributing to the scene.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) lights: Vec<Light>,
    /// Ambient light contribution applied to every shaded point.
    ///
    /// The raytracing integration suites set the ambient term from outside
    /// the crate, so the setter stays public; the getter and mut-getter have
    /// no external consumer and stay crate-internal.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    pub ambient: Vector3D,
    /// View position used for specular term calculation.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) eye: Vector3D,
}
