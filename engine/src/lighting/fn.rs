use super::*;

/// Computes the Lambertian (diffuse) contribution of a single light at a
/// shaded surface point.
///
/// The returned color is computed as:
///
/// ```text
/// color * intensity * max(0, dot(normal, light_dir)) * albedo
/// ```
///
/// For directional lights, `light_dir` is the unit direction toward the
/// light. For point and spot lights, `light_dir` is the unit direction from
/// the surface position to the light position. The light's color and
/// intensity are merged multiplicatively with the Lambertian cosine term
/// and the material's albedo. The caller is expected to have already
/// applied any falloff factor.
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
    let light_dir: Vector3D = light.get_direction();
    let cos: f64 = normal.dot(light_dir).max(0.0);
    let intensity: f64 = light.get_intensity();
    let color: Vector3D = light.get_color();
    let albedo: Vector3D = material.get_albedo();
    let k: f64 = intensity * cos;
    Vector3D::new(
        color.get_x() * albedo.get_x() * k,
        color.get_y() * albedo.get_y() * k,
        color.get_z() * albedo.get_z() * k,
    )
}

/// Computes the Blinn-Phong specular contribution of a single light.
///
/// Returns:
///
/// ```text
/// light_color * intensity * pow(max(0, dot(reflect(-l, n), v)), shininess) * specular
/// ```
///
/// where `reflect(-l, n)` is the mirror reflection of the light direction
/// about the surface normal and `v` is the unit view direction from the
/// shaded point toward the eye.
///
/// # Arguments
///
/// - `&Light` - The light source.
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
    let light_dir: Vector3D = light.get_direction();
    let reflect: Vector3D = (light_dir - normal.scaled(2.0 * light_dir.dot(normal))).normalized();
    let spec_factor: f64 = reflect
        .dot(view_dir)
        .max(0.0)
        .powf(material.get_shininess());
    let specular: f64 = material.get_specular();
    let intensity: f64 = light.get_intensity();
    let color: Vector3D = light.get_color();
    let k: f64 = intensity * spec_factor * specular;
    Vector3D::new(color.get_x() * k, color.get_y() * k, color.get_z() * k)
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

/// Intersects a ray with a sphere centered at `center` with radius `radius`.
///
/// Uses the standard quadratic-form ray-sphere test. Returns `Some((t, n))`
/// where `t` is the nearest positive intersection distance along the ray
/// and `n` is the outward unit normal at the hit point. Returns `None` if
/// the ray misses.
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

/// Computes a soft-shadow visibility factor in the range 0.0..=1.0.
///
/// Casts a single ray from `origin` toward `light_pos` and checks whether
/// any sphere in `occluders` blocks the path. Returns 1.0 when no occluder
/// is intersected, otherwise returns 0.0 (binary shadow). A future
/// refinement could sample multiple rays to approximate penumbra.
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
        if let Some((t, _)) = ray_sphere_intersect(origin, dir, center, radius)
            && t > EPSILON
            && t < dist - EPSILON
        {
            return 0.0;
        }
    }
    1.0
}
