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
