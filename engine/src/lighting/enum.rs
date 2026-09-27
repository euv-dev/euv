use super::*;

/// Describes the kind of light source being represented.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum LightType {
    /// An infinitely distant directional light (e.g. sunlight). The `direction`
    /// field carries the unit vector pointing away from the light source.
    #[default]
    Directional,
    /// A positional point light with inverse-square falloff.
    Point,
    /// A positional spotlight with a cone defined by a unit direction and a
    /// half-angle whose cosine is stored in the light's `spot_cos` field.
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
    /// Lambertian diffuse plus Blinn-Phong style specular highlight.
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
