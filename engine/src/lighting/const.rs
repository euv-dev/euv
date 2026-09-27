use super::*;

/// The default ambient light contribution used when no explicit ambient
/// vector is supplied. Shader-friendly: stored as a [`Vector3D`].
pub(crate) const LIGHTING_DEFAULT_AMBIENT: Vector3D = Vector3D {
    x: 0.1,
    y: 0.1,
    z: 0.1,
};

/// The default Phong specular exponent. Higher values produce tighter,
/// more focused highlights.
pub(crate) const LIGHTING_DEFAULT_SHININESS: f64 = 32.0;

/// Minimum point-light distance used when evaluating inverse-square falloff.
/// Prevents a divide-by-zero when the shaded point is exactly at the light
/// position.
pub(crate) const LIGHTING_POINT_LIGHT_MIN_DISTANCE: f64 = 0.001;

/// The baseline reflectance weight applied to the [`MaterialKind::Pbr`]
/// diffuse term before the Schlick fresnel factor is applied.
///
/// Schlick's approximation is `F = F0 + (1 - F0) * (1 - cos)^5`, whose
/// value at normal incidence is `F0` itself. Keeping that floor non-zero
/// stops the diffuse term from vanishing where the half-vector aligns with
/// the normal, which a raw fresnel multiply would do.
pub const LIGHTING_PBR_ENERGY_CONSERVATION: f64 = 0.04;
