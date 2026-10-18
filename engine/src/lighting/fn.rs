use super::*;

/// Computes the Lambertian (diffuse) contribution of a single light at a
/// shaded surface point.
///
/// ```text
/// color * intensity * albedo * max(0, dot(normal, light_dir))
/// ```
///
/// `light_dir` is the unit direction from the surface toward the light;
/// any falloff must already be folded into a scaled intensity by the
/// caller.
///
/// A thin wrapper over [`lambert_term`], which takes the same quantities
/// unpacked instead of through a [`Light`], so a direct call and the same
/// light routed through [`LightingUniforms::shade_with_visibility`] share
/// one implementation.
///
/// # Arguments
///
/// - `&Light` - The light source being evaluated.
/// - `Vector3D` - The surface normal (expected to be unit length).
/// - `&Material` - The material at the shaded point.
///
/// # Returns
///
/// - `Vector3D` - The diffuse contribution of this light.
pub fn compute_lambert(light: &Light, normal: Vector3D, material: &Material) -> Vector3D {
    lambert_term(
        light.get_color(),
        light.get_intensity(),
        light.get_direction(),
        normal,
        material.get_albedo(),
    )
}

/// The Lambertian term shared by [`compute_lambert`] and
/// [`LightingUniforms::shade_with_visibility`].
///
/// Unpacking the light into its colour, intensity, and direction is what
/// lets the shading loop evaluate a light once — one distance, one
/// direction, one falloff — instead of building a mutated copy of the
/// [`Light`] per term.
///
/// # Arguments
///
/// - `Vector3D` - The light's RGB color.
/// - `f64` - The light's intensity, already scaled by any falloff.
/// - `Vector3D` - The unit direction from the surface toward the light.
/// - `Vector3D` - The surface normal (unit length).
/// - `Vector3D` - The material's diffuse albedo.
///
/// # Returns
///
/// - `Vector3D` - `color * albedo * intensity * max(0, dot(normal, light_dir))`.
pub(crate) fn lambert_term(
    light_color: Vector3D,
    light_intensity: f64,
    light_dir: Vector3D,
    normal: Vector3D,
    albedo: Vector3D,
) -> Vector3D {
    let cos: f64 = normal.dot(light_dir).max(0.0);
    let k: f64 = light_intensity * cos;
    Vector3D::new(
        light_color.get_x() * albedo.get_x() * k,
        light_color.get_y() * albedo.get_y() * k,
        light_color.get_z() * albedo.get_z() * k,
    )
}

/// Computes the Phong specular contribution of a single light.
///
/// ```text
/// color * intensity * specular * pow(max(0, dot(reflect(-l, n), v)), shininess)
/// ```
///
/// where `l` is the unit direction from the surface toward the light, `n`
/// the surface normal, and `v` the unit view direction from the shaded
/// point toward the eye. A thin wrapper over [`phong_specular_term`], so
/// a direct call and the same light routed through
/// [`LightingUniforms::shade_with_visibility`] share one implementation.
///
/// # Arguments
///
/// - `&Light` - The light source. Its `direction` must point from the
///   surface toward the light.
/// - `Vector3D` - The surface normal (unit length).
/// - `Vector3D` - The unit view direction (from the surface toward the eye).
/// - `&Material` - The material at the shaded point.
///
/// # Returns
///
/// - `Vector3D` - The specular contribution of this light.
pub fn compute_phong(
    light: &Light,
    normal: Vector3D,
    view_dir: Vector3D,
    material: &Material,
) -> Vector3D {
    phong_specular_term(
        light.get_color(),
        light.get_intensity(),
        light.get_direction(),
        normal,
        view_dir,
        material.get_specular(),
        material.get_shininess(),
    )
}

/// The Phong specular term shared by [`compute_phong`] and
/// [`LightingUniforms::shade_with_visibility`]. Returns the zero vector
/// unless the surface faces the light, so the lobe cannot survive on the
/// back side of a surface.
///
/// # Arguments
///
/// - `Vector3D` - The light's RGB color.
/// - `f64` - The light's intensity, already scaled by any falloff.
/// - `Vector3D` - The unit direction from the surface toward the light.
/// - `Vector3D` - The surface normal (unit length).
/// - `Vector3D` - The unit view direction (from the surface toward the eye).
/// - `f64` - The material's specular intensity.
/// - `f64` - The Phong specular exponent.
///
/// # Returns
///
/// - `Vector3D` - `color * intensity * specular * spec_factor`.
pub(crate) fn phong_specular_term(
    light_color: Vector3D,
    light_intensity: f64,
    light_dir: Vector3D,
    normal: Vector3D,
    view_dir: Vector3D,
    material_specular: f64,
    shininess: f64,
) -> Vector3D {
    let spec_factor: f64 = phong_specular_factor(light_dir, normal, view_dir, shininess);
    let k: f64 = light_intensity * spec_factor * material_specular;
    Vector3D::new(
        light_color.get_x() * k,
        light_color.get_y() * k,
        light_color.get_z() * k,
    )
}

/// Computes the Phong specular factor in the range 0.0..=1.0.
///
/// The factor is `pow(max(0, dot(reflect(-l, n), v)), shininess)`.
///
/// ## Why `-l` is reflected, not `l`
///
/// `reflect(i, n) = i - 2 * dot(i, n) * n` mirrors an **incident** ray `i`
/// about the normal. The incident ray travels from the light into the
/// surface, so it is `-l`, and the expansion is
///
/// ```text
/// reflect(-l, n) = -l + 2 * dot(n, l) * n
/// ```
///
/// Reflecting `l` instead yields a vector pointing back at the lamp, so
/// `dot(reflect, v)` peaks only where the eye is *behind* the surface and
/// the highlight lands on the unlit side.
///
/// With `n` and `l` both unit length the reflection is already unit
/// length — `|2 * n * d - l|² = 4d² - 4d² + 1 = 1` for `d = dot(n, l)` —
/// so this needs no normalization, no division, and has no degenerate
/// case when `l` opposes `n`. The lobe peaks at `v == 2 * n * dot(n, l) -
/// l`, i.e. off-axis unless the light is exactly head-on.
///
/// Returns `0.0` — never `NaN` — when `dot(n, l) <= 0.0`: a surface lit
/// from behind must not flash a highlight.
///
/// # Arguments
///
/// - `Vector3D` - The unit direction from the surface toward the light.
/// - `Vector3D` - The surface normal (unit length).
/// - `Vector3D` - The unit view direction (from the surface toward the eye).
/// - `f64` - The Phong specular exponent.
///
/// # Returns
///
/// - `f64` - The specular factor.
pub(crate) fn phong_specular_factor(
    light_dir: Vector3D,
    normal: Vector3D,
    view_dir: Vector3D,
    shininess: f64,
) -> f64 {
    if normal.dot(light_dir) <= 0.0 {
        return 0.0;
    }
    let reflected: Vector3D = normal.scaled(2.0 * normal.dot(light_dir)) - light_dir;
    reflected.dot(view_dir).max(0.0).powf(shininess)
}

/// Applies the inverse-square falloff formula
/// `1.0 / (1.0 + falloff * d²)`, clamped to a non-negative result.
///
/// # Arguments
///
/// - `f64` - The distance from the light source. Negative values are
///   treated as zero.
/// - `f64` - The falloff coefficient (0.0 disables falloff).
///
/// # Returns
///
/// - `f64` - A non-negative scalar in the range 0.0..=1.0.
pub fn apply_falloff(distance: f64, falloff: f64) -> f64 {
    let d: f64 = distance.abs();
    let denom: f64 = 1.0 + falloff * d * d;
    (1.0 / denom).max(0.0)
}

/// Computes a Schlick fresnel factor for the [`MaterialKind::Pbr`] branch of
/// [`LightingUniforms::shade`].
///
/// Schlick's approximation is `F = F0 + (1 - F0) * (1 - cos)^5` where
/// `cos` is the cosine between the surface normal and the half-vector
/// between the light and the eye, and `F0` is the normal-incidence
/// reflectance. Returns a scalar in the range
/// [`LIGHTING_PBR_ENERGY_CONSERVATION`]..=1.0: it is minimal when the
/// half-vector aligns with the normal and grows as the half-vector turns
/// away from it, so a lit silhouette picks up a rim.
///
/// This is a single-lobe approximation, not a microfacet BRDF: it ignores
/// roughness, multiple scattering, and per-channel Fresnel, so it should be
/// read as "diffuse plus a fresnel rim" rather than as a physically based
/// reflectance model.
///
/// # Arguments
///
/// - `Vector3D` - The unit direction from the surface toward the light.
/// - `Vector3D` - The unit view direction (from the surface toward the eye).
/// - `Vector3D` - The surface normal (unit length).
///
/// # Returns
///
/// - `f64` - The fresnel reflectance scalar.
pub fn apply_schlick_fresnel(light_dir: Vector3D, view_dir: Vector3D, normal: Vector3D) -> f64 {
    let half: Vector3D = light_dir + view_dir;
    let half_len: f64 = half.magnitude();
    let cos: f64 = if half_len > EPSILON {
        normal.dot(half.scaled(1.0 / half_len)).abs().min(1.0)
    } else {
        1.0
    };
    let base: f64 = LIGHTING_PBR_ENERGY_CONSERVATION;
    base + (1.0 - base) * (1.0 - cos).powi(5)
}

/// Returns the unit-length form of `vector`, or the zero vector when it is
/// too short to have a meaningful direction.
///
/// The threshold is [`EPSILON`], so a degenerate vector normalises to
/// zero rather than to `NaN` from a division by a vanishing magnitude.
/// A zero direction is safe downstream: it fails both the facing test and
/// the cone test.
///
/// # Arguments
///
/// - `Vector3D` - The vector to normalize.
///
/// # Returns
///
/// - `Vector3D` - The unit vector, or the zero vector at or below [`EPSILON`].
pub(crate) fn unit_vector(vector: Vector3D) -> Vector3D {
    unit_vector_and_length(vector).0
}

/// Returns `(unit(vector), length)` while taking the magnitude **once**.
///
/// Shading needs both the unit direction and the original length of the
/// same vector; measuring it twice would duplicate the square root on the
/// hot path. A vector at or below [`EPSILON`] yields
/// `(zero, 0.0)`.
///
/// # Arguments
///
/// - `Vector3D` - The vector to normalize.
///
/// # Returns
///
/// - `(Vector3D, f64)` - The unit vector and the vector's own length.
pub(crate) fn unit_vector_and_length(vector: Vector3D) -> (Vector3D, f64) {
    let length: f64 = vector.magnitude();
    if length <= EPSILON {
        return (Vector3D::zero(), 0.0);
    }
    (vector.scaled(1.0 / length), length)
}

/// Returns whether `light` can reach a surface point at all, before any
/// visibility query: the spot cone test plus a zero-intensity reject.
///
/// These are the only two rejects a caller can make without geometry, so
/// [`LightingUniforms::shade_with_visibility`] applies them before
/// invoking the visibility callback.
///
/// ## Why the cone test compares against `-spot_cos`
///
/// A spot light stores `direction` as the cone axis pointing **outward**
/// along the beam, i.e. the way the light travels. `light_dir` points the
/// other way, from the surface **into** the lamp, so for a surface on the
/// axis `dot(light_dir, axis)` is `-1.0`, not `+1.0`. Being inside the cone
/// is therefore
///
/// ```text
/// dot(light_dir, axis) <= -cos(half_angle) == -spot_cos
/// ```
///
/// Testing `dot(light_dir, axis) >= spot_cos` instead would invert the
/// cone: every point on the beam is rejected and every point behind the
/// lamp is lit.
///
/// When the surface sits exactly on the lamp `light_dir` is zero, whose dot
/// product with any axis is `0.0`; for a cone of half-angle 90° or narrower
/// that would read as out of cone. The degenerate case is accepted
/// explicitly — a surface coincident with a lamp sits at the cone's apex
/// and is inside it by definition — and the zero `light_dir` makes the
/// facing test reject it regardless.
///
/// # Arguments
///
/// - `&Light` - The light being tested.
/// - `Vector3D` - The normalized direction from the surface toward the light.
///
/// # Returns
///
/// - `bool` - `false` when the light is out of cone or carries no intensity.
pub(crate) fn light_reaches_position(light: &Light, light_dir: Vector3D) -> bool {
    if light.get_intensity() <= 0.0 {
        return false;
    }
    if light.get_kind() != LightType::Spot {
        return true;
    }
    if light_dir == Vector3D::zero() {
        return true;
    }
    light_dir.dot(light.get_direction()) <= -light.get_spot_cos()
}

/// Intersects a ray with a sphere centered at `center` with radius `radius`.
///
/// Uses the standard quadratic-form ray-sphere test.
///
/// # Arguments
///
/// - `Vector3D` - The ray origin.
/// - `Vector3D` - The ray direction (expected to be unit length).
/// - `Vector3D` - The sphere center.
/// - `f64` - The sphere radius (must be positive).
///
/// # Returns
///
/// - `Option<(f64, Vector3D)>` - The hit distance and surface normal, or
///   `None` on miss.
pub fn ray_sphere_intersect(
    origin: Vector3D,
    dir: Vector3D,
    center: Vector3D,
    radius: f64,
) -> Option<(f64, Vector3D)> {
    let oc: Vector3D = origin - center;
    let b: f64 = oc.dot(dir);
    let c: f64 = oc.dot(oc) - radius * radius;
    let disc: f64 = b * b - c;
    if disc < 0.0 {
        return None;
    }
    let sq: f64 = disc.sqrt();
    let t1: f64 = -b - sq;
    let t2: f64 = -b + sq;
    let t: f64 = if t1 >= 0.0 {
        t1
    } else if t2 >= 0.0 {
        t2
    } else {
        return None;
    };
    let hit: Vector3D = origin + dir.scaled(t);
    let normal: Vector3D = (hit - center).normalized();
    Some((t, normal))
}

/// Intersects a ray with an axis-aligned bounding box using the slab method.
///
/// # Arguments
///
/// - `Vector3D` - The ray origin.
/// - `Vector3D` - The ray direction (expected to be unit length).
/// - `Vector3D` - The AABB minimum corner.
/// - `Vector3D` - The AABB maximum corner.
///
/// # Returns
///
/// - `Option<(f64, f64, Vector3D)>` - `(t_near, t_far, normal)` on hit, or
///   `None` if the ray misses or is parallel to the slab.
pub fn ray_aabb_intersect(
    origin: Vector3D,
    dir: Vector3D,
    aabb_min: Vector3D,
    aabb_max: Vector3D,
) -> Option<(f64, f64, Vector3D)> {
    let inv_dir: Vector3D = Vector3D::new(1.0 / dir.get_x(), 1.0 / dir.get_y(), 1.0 / dir.get_z());
    let t1x: f64 = (aabb_min.get_x() - origin.get_x()) * inv_dir.get_x();
    let t2x: f64 = (aabb_max.get_x() - origin.get_x()) * inv_dir.get_x();
    let t1y: f64 = (aabb_min.get_y() - origin.get_y()) * inv_dir.get_y();
    let t2y: f64 = (aabb_max.get_y() - origin.get_y()) * inv_dir.get_y();
    let t1z: f64 = (aabb_min.get_z() - origin.get_z()) * inv_dir.get_z();
    let t2z: f64 = (aabb_max.get_z() - origin.get_z()) * inv_dir.get_z();
    let tmin_x: f64 = t1x.min(t2x);
    let tmax_x: f64 = t1x.max(t2x);
    let tmin_y: f64 = t1y.min(t2y);
    let tmax_y: f64 = t1y.max(t2y);
    let tmin_z: f64 = t1z.min(t2z);
    let tmax_z: f64 = t1z.max(t2z);
    let t_near: f64 = tmin_x.max(tmin_y).max(tmin_z);
    let t_far: f64 = tmax_x.min(tmax_y).min(tmax_z);
    if t_near > t_far || t_far < 0.0 {
        return None;
    }
    let hit: Vector3D = origin + dir.scaled(t_near);
    let cx: f64 = (aabb_min.get_x() + aabb_max.get_x()) * 0.5;
    let cy: f64 = (aabb_min.get_y() + aabb_max.get_y()) * 0.5;
    let cz: f64 = (aabb_min.get_z() + aabb_max.get_z()) * 0.5;
    let dx: f64 = hit.get_x() - cx;
    let dy: f64 = hit.get_y() - cy;
    let dz: f64 = hit.get_z() - cz;
    let ex: f64 = (aabb_max.get_x() - aabb_min.get_x()) * 0.5;
    let ey: f64 = (aabb_max.get_y() - aabb_min.get_y()) * 0.5;
    let ez: f64 = (aabb_max.get_z() - aabb_min.get_z()) * 0.5;
    let ax: f64 = dx.abs() / ex.max(EPSILON);
    let ay: f64 = dy.abs() / ey.max(EPSILON);
    let az: f64 = dz.abs() / ez.max(EPSILON);
    let normal: Vector3D = if ax >= ay && ax >= az {
        Vector3D::new(dx.signum(), 0.0, 0.0)
    } else if ay >= az {
        Vector3D::new(0.0, dy.signum(), 0.0)
    } else {
        Vector3D::new(0.0, 0.0, dz.signum())
    };
    Some((t_near, t_far, normal))
}

/// Computes a visibility factor in the range 0.0..=1.0 for the segment from
/// `origin` to `light_pos`.
///
/// Despite the historical name this is **binary**, not soft: `1.0` when
/// nothing blocks the segment, `0.0` when anything does. No penumbra is
/// sampled. Callers that hold real geometry should use
/// [`LightingUniforms::shade_with_visibility`] with a callback instead;
/// this is the bounded fallback for callers holding only
/// `(center, radius)` bounding spheres, which necessarily over-shadow a
/// ray passing through empty space beside an occluder.
///
/// # Arguments
///
/// - `Vector3D` - The surface point casting the shadow ray.
/// - `Vector3D` - The light position to test against.
/// - `&[(Vector3D, f64)]` - A slice of `(center, radius)` occluder spheres.
///
/// # Returns
///
/// - `f64` - 1.0 if the light is visible, 0.0 if fully occluded.
pub fn soft_shadow_factor(
    origin: Vector3D,
    light_pos: Vector3D,
    occluders: &[(Vector3D, f64)],
) -> f64 {
    let to_light: Vector3D = light_pos - origin;
    let dist: f64 = to_light.magnitude();
    if dist < EPSILON {
        return 1.0;
    }
    let dir: Vector3D = to_light.scaled(1.0 / dist);
    for &(center, radius) in occluders.iter() {
        if segment_crosses_sphere(origin, dir, dist, center, radius) {
            return 0.0;
        }
    }
    1.0
}

/// Returns whether either ray-sphere root lands strictly inside the open
/// segment `(EPSILON, dist - EPSILON)`.
///
/// **Both roots are tested, not just the near one.** A shadow ray is very
/// often cast from the surface point of one of the occluders themselves:
/// for that ray the near root sits at `t ≈ 0`, inside the [`EPSILON`]
/// self-hit threshold, while the far root sits at `t ≈ 2r`. Testing only
/// the nearest root would report the light as visible through the very
/// sphere the ray starts inside.
///
/// # Arguments
///
/// - `Vector3D` - The segment start (the shaded point).
/// - `Vector3D` - The unit direction toward the light.
/// - `f64` - The distance from the segment start to the light.
/// - `Vector3D` - The sphere center.
/// - `f64` - The sphere radius.
///
/// # Returns
///
/// - `bool` - `true` when the segment passes through the sphere's interior.
pub(crate) fn segment_crosses_sphere(
    origin: Vector3D,
    dir: Vector3D,
    dist: f64,
    center: Vector3D,
    radius: f64,
) -> bool {
    let oc: Vector3D = origin - center;
    let b: f64 = oc.dot(dir);
    let c: f64 = oc.dot(oc) - radius * radius;
    let disc: f64 = b * b - c;
    if disc < 0.0 {
        return false;
    }
    let sq: f64 = disc.sqrt();
    let near: f64 = -b - sq;
    let far: f64 = -b + sq;
    let inside: f64 = EPSILON;
    let outside: f64 = dist - EPSILON;
    (near > inside && near < outside) || (far > inside && far < outside)
}
