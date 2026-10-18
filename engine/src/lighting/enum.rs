use super::*;

/// Describes the kind of light source being represented.
///
/// ## Direction convention
///
/// Every direction a [`Light`] carries is measured **toward the light
/// source**, with one deliberate exception:
///
/// - [`LightType::Directional`]: `direction` points from the surface
///   toward the light (sourceward), so a surface is lit when
///   `dot(normal, direction) > 0`.
/// - [`LightType::Point`]: the direction is derived from
///   `position - surface_point` at shading time and is also sourceward;
///   the light stores no direction of its own.
/// - [`LightType::Spot`]: `direction` points **outward along the cone
///   axis**, away from the lamp, i.e. the way the light travels. The
///   surface→light vector used for shading is its negation. This is the
///   opposite sense from the other two kinds, so a spot light's cone axis
///   and a spot light's shading direction are *not* the same vector.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum LightType {
    /// An infinitely distant directional light (e.g. sunlight).
    ///
    /// The `direction` field carries the unit vector pointing from the
    /// shaded surface toward the light source, matching the convention of
    /// [`LightType::Point`].
    #[default]
    Directional,
    /// A positional point light with inverse-square falloff.
    ///
    /// The light radiates in every direction; the surface→light vector is
    /// computed from `position` at shading time.
    Point,
    /// A positional spotlight with a cone defined by a unit direction and a
    /// half-angle whose cosine is stored in the light's `spot_cos` field.
    ///
    /// The `direction` field is the cone **axis pointing outward** from the
    /// lamp, in the direction the beam travels — the opposite sense to the
    /// shading direction derived from `position`.
    Spot,
}

/// Describes the shading model applied to a [`Material`] during lighting and
/// ray-tracing evaluation.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum MaterialKind {
    /// Pure Lambertian diffuse, no specular highlight.
    ///
    /// [`LightingUniforms::shade`] evaluates the diffuse term only.
    #[default]
    Lambert,
    /// Lambertian diffuse plus a Phong specular highlight.
    ///
    /// [`LightingUniforms::shade`] evaluates the diffuse term plus a specular
    /// lobe driven by the material's `specular` and `shininess` fields.
    Phong,
    /// Diffuse with a Schlick fresnel rim, and no separate specular lobe.
    ///
    /// ## This is not a full PBR model
    ///
    /// A physically based renderer would evaluate a microfacet BRDF with
    /// roughness-driven geometry, a metallic / dielectric split, multiple
    /// scattering, and per-channel Fresnel. None of that is implemented here.
    /// What this variant actually does is scale the Lambertian diffuse term
    /// by [`apply_schlick_fresnel`], which brightens grazing angles and adds
    /// a rim. That is a *deliberate, documented approximation* rather than
    /// the physically based model the variant name suggests.
    ///
    /// The variant is kept rather than removed so callers can branch on
    /// material model without a breaking change when a real BRDF lands. Note
    /// that `specular` and `shininess` are ignored on this path: the diffuse
    /// fresnel blend replaces them entirely.
    Pbr,
}
