use super::*;

/// Returns the AABB extents `(min, max)` of an [`Occluder`].
///
/// For AABB occluders this is `(center, extent)`. For sphere occluders
/// the bounding box is computed from the center and the `.x` component of
/// `extent` (the sphere radius).
///
/// # Arguments
///
/// - `&Occluder` - The occluder to bound.
///
/// # Returns
///
/// - `(Vector3D, Vector3D)` - The `(min, max)` corners of the AABB.
fn occluder_aabb_extents(occluder: &Occluder) -> (Vector3D, Vector3D) {
    match occluder.get_kind() {
        OccluderKind::Aabb => (occluder.get_center(), occluder.get_extent()),
        OccluderKind::Sphere => {
            let center: Vector3D = occluder.get_center();
            let radius: f64 = occluder.get_extent().get_x();
            let r: Vector3D = Vector3D::new(radius, radius, radius);
            (center - r, center + r)
        }
    }
}

/// Flattens every occluder into `(center, radius)` sphere tuples used by
/// [`soft_shadow_factor`].
///
/// For sphere occluders the tuple is `(center, radius)`. For AABB
/// occluders a conservative bounding sphere is computed from the AABB.
pub(crate) fn collect_occluder_points(occluders: &[Occluder]) -> Vec<(Vector3D, f64)> {
    let mut out: Vec<(Vector3D, f64)> = Vec::new();
    for occ in occluders.iter() {
        let (mn, mx): (Vector3D, Vector3D) = occluder_aabb_extents(occ);
        let cx: f64 = (mn.get_x() + mx.get_x()) * 0.5;
        let cy: f64 = (mn.get_y() + mx.get_y()) * 0.5;
        let cz: f64 = (mn.get_z() + mx.get_z()) * 0.5;
        let ex: f64 = (mx.get_x() - mn.get_x()) * 0.5;
        let ey: f64 = (mx.get_y() - mn.get_y()) * 0.5;
        let ez: f64 = (mx.get_z() - mn.get_z()) * 0.5;
        let r: f64 = (ex * ex + ey * ey + ez * ez).sqrt();
        out.push((Vector3D::new(cx, cy, cz), r));
    }
    out
}

/// Finds the closest intersection between a ray and a list of occluders
/// without touching any [`Material`].
///
/// Returns the winning occluder's index alongside the hit data so callers
/// can borrow the material directly from the occluder list instead of
/// cloning it per candidate. The tie-breaking rule matches the historical
/// behavior: the first occluder achieving the minimum `t` wins.
///
/// # Arguments
///
/// - `&Ray` - The ray to test.
/// - `&[Occluder]` - The occluders to test against.
///
/// # Returns
///
/// - `Option<(usize, f64, Vector3D, Vector3D)>` - The occluder index, the
///   hit distance `t`, the hit position, and the surface normal, or `None`
///   if the ray misses.
pub(crate) fn closest_hit_indexed(
    ray: &Ray,
    occluders: &[Occluder],
) -> Option<(usize, f64, Vector3D, Vector3D)> {
    let origin: Vector3D = ray.get_origin();
    let dir: Vector3D = ray.get_direction();
    let t_min: f64 = ray.get_t_min();
    let t_max: f64 = ray.get_t_max();
    let mut best: Option<(usize, f64, Vector3D, Vector3D)> = None;
    for (index, occ) in occluders.iter().enumerate() {
        let candidate: Option<(f64, Vector3D)> = match occ.get_kind() {
            OccluderKind::Sphere => {
                let center: Vector3D = occ.get_center();
                let radius: f64 = occ.get_extent().get_x();
                match ray_sphere_intersect(origin, dir, center, radius) {
                    Some((t, n)) if t >= t_min && t <= t_max => Some((t, n)),
                    _ => None,
                }
            }
            OccluderKind::Aabb => {
                let aabb_min: Vector3D = occ.get_center();
                let aabb_max: Vector3D = occ.get_extent();
                match ray_aabb_intersect(origin, dir, aabb_min, aabb_max) {
                    Some((t_near, _t_far, n)) if t_near >= t_min && t_near <= t_max => {
                        Some((t_near, n))
                    }
                    _ => None,
                }
            }
        };
        if let Some((t, n)) = candidate {
            let keep_previous: bool = matches!(&best, Some(previous) if previous.1 <= t);
            if !keep_previous {
                let hit_pos: Vector3D = origin + dir.scaled(t);
                best = Some((index, t, hit_pos, n));
            }
        }
    }
    best
}

/// Builds a reflected ray bouncing off a surface point with the given
/// normal.
///
/// # Arguments
///
/// - `&Ray` - The incoming ray.
/// - `Vector3D` - The world-space hit position.
/// - `Vector3D` - The outward unit normal at the hit point.
///
/// # Returns
///
/// - `Ray` - A new ray originating at the hit point with the reflected
///   direction, `t_min` reset to `RAYTRACE_DEFAULT_T_MIN`, `t_max` set to
///   `RAYTRACE_DEFAULT_T_MAX`, and `depth` incremented by one.
fn bounce_ray(ray: &Ray, position: Vector3D, normal: Vector3D) -> Ray {
    let dir: Vector3D = ray.get_direction();
    let dot: f64 = dir.dot(normal);
    let reflected_dir: Vector3D = dir - normal.scaled(2.0 * dot);
    Ray {
        origin: position,
        direction: reflected_dir,
        t_min: RAYTRACE_DEFAULT_T_MIN,
        t_max: RAYTRACE_DEFAULT_T_MAX,
        depth: ray.get_depth() + 1,
    }
}

/// Iteratively traces a ray against `occluders` using precomputed shadow
/// bounding spheres, performing no heap allocation per bounce.
///
/// Color contributions are accumulated with a specular throughput: each
/// bounce multiplies the throughput by the hit material's specular
/// intensity, and a miss adds the ambient color scaled by the current
/// throughput. The bounce loop stops when the ray misses, when `depth`
/// reaches `max_bounces`, or when the hit material's specular intensity is
/// not greater than [`EPSILON`], matching the behavior of the historical
/// recursive formulation.
///
/// # Arguments
///
/// - `Ray` - The ray to trace.
/// - `&[Occluder]` - All occluding surfaces in the scene.
/// - `&[(Vector3D, f64)]` - Precomputed `(center, radius)` shadow bounding
///   spheres, one per occluder.
/// - `&LightingUniforms` - Lighting parameters used during shading.
/// - `u32` - The maximum number of bounces allowed for this ray.
///
/// # Returns
///
/// - `Vector3D` - The final traced color.
pub(crate) fn trace_bounces(
    ray: Ray,
    occluders: &[Occluder],
    shadow_points: &[(Vector3D, f64)],
    lights: &LightingUniforms,
    max_bounces: u32,
) -> Vector3D {
    let ambient: Vector3D = lights.get_ambient();
    let mut color: Vector3D = Vector3D::zero();
    let mut throughput: f64 = 1.0;
    let mut current: Ray = ray;
    loop {
        let (index, _t, position, normal): (usize, f64, Vector3D, Vector3D) =
            match closest_hit_indexed(&current, occluders) {
                None => {
                    color += ambient.scaled(throughput);
                    break;
                }
                Some(hit) => hit,
            };
        let material: &Material = occluders[index].get_material();
        color += lights
            .shade(position, normal, material, shadow_points)
            .scaled(throughput);
        let spec: f64 = material.get_specular();
        if current.get_depth() >= max_bounces || spec <= EPSILON {
            break;
        }
        throughput *= spec;
        current = bounce_ray(&current, position, normal);
    }
    color
}
