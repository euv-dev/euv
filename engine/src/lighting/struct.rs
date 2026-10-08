use super::*;

/// A light source contributing illumination to a shaded point.
///
/// The fields have meanings that depend on [`LightType`]:
///
/// - For [`LightType::Directional`], `position` is unused and `direction` is
///   the unit vector pointing from the surface toward the light.
/// - For [`LightType::Point`], `position` is the world position of the light
///   and `direction` is unused.
/// - For [`LightType::Spot`], both `position` and `direction` are meaningful
///   and `spot_cos` carries `cos(half_angle)` of the spotlight cone.
#[derive(Clone, Data, Debug, New, PartialEq)]
pub struct Light {
    /// The kind of light source (directional, point, or spot).
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) kind: LightType,
    /// World-space position for point and spot lights; otherwise unused.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) position: Vector3D,
    /// Unit direction for directional and spot lights; otherwise unused.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    pub(crate) direction: Vector3D,
    /// RGB intensity multiplier applied to the contribution.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) color: Vector3D,
    /// Overall intensity scalar in the range 0.0..=infinity.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) intensity: f64,
    /// Inverse-square falloff factor for point and spot lights.
    #[get(type(copy))]
    pub(crate) falloff: f64,
    /// `cos(half_angle)` for spot lights; zero for other kinds.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) spot_cos: f64,
}

/// A material describing how a surface responds to illumination.
#[derive(Clone, Data, Debug, New, PartialEq)]
pub struct Material {
    /// The shading model applied during evaluation.
    #[get(type(copy))]
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
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    pub(crate) ambient: Vector3D,
    /// View position used for specular term calculation.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) eye: Vector3D,
}
