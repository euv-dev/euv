use super::*;

/// Returns the component-wise minimum of two vectors.
///
/// # Arguments
///
/// - `Vector3D` - The first vector.
/// - `Vector3D` - The second vector.
///
/// # Returns
///
/// - `Vector3D` - The component-wise minimum.
fn vector3d_min(a: Vector3D, b: Vector3D) -> Vector3D {
    Vector3D::new(
        a.get_x().min(b.get_x()),
        a.get_y().min(b.get_y()),
        a.get_z().min(b.get_z()),
    )
}

/// Returns the component-wise maximum of two vectors.
///
/// # Arguments
///
/// - `Vector3D` - The first vector.
/// - `Vector3D` - The second vector.
///
/// # Returns
///
/// - `Vector3D` - The component-wise maximum.
fn vector3d_max(a: Vector3D, b: Vector3D) -> Vector3D {
    Vector3D::new(
        a.get_x().max(b.get_x()),
        a.get_y().max(b.get_y()),
        a.get_z().max(b.get_z()),
    )
}

/// Returns the inclusive parameter interval of a single AABB slab along one
/// ray axis, or `None` when the ray runs parallel to the slab outside it.
///
/// A parallel axis is not rejected outright: the interval becomes
/// `(-inf, +inf)` when the origin lies inside the slab and the caller keeps
/// only the two axes that actually constrain the ray.
///
/// # Arguments
///
/// - `f64` - The ray origin component on this axis.
/// - `f64` - The ray direction component on this axis.
/// - `f64` - The slab lower bound.
/// - `f64` - The slab upper bound.
///
/// # Returns
///
/// - `Option<(f64, f64)>` - The `(enter, exit)` parameters, or `None` when
///   the ray is parallel to the slab and outside it.
fn slab_interval(
    origin_component: f64,
    dir_component: f64,
    lower: f64,
    upper: f64,
) -> Option<(f64, f64)> {
    if dir_component.abs() < RAYTRACE_TRIANGLE_EPSILON {
        if origin_component < lower || origin_component > upper {
            return None;
        }
        return Some((f64::NEG_INFINITY, f64::INFINITY));
    }
    let inverse: f64 = 1.0 / dir_component;
    let t0: f64 = (lower - origin_component) * inverse;
    let t1: f64 = (upper - origin_component) * inverse;
    Some((t0.min(t1), t0.max(t1)))
}

/// Reports whether the ray segment `t_min`..=`t_max` can touch the AABB
/// spanning `lower` to `upper`.
///
/// This is a conservative broad-phase reject: it never discards an occluder
/// whose exact test could report a hit inside the requested range, because
/// the AABB fully contains the occluder and every exact hit lies inside the
/// AABB.
///
/// # Arguments
///
/// - `Vector3D` - The ray origin.
/// - `Vector3D` - The ray direction.
/// - `Vector3D` - The AABB minimum corner.
/// - `Vector3D` - The AABB maximum corner.
/// - `f64` - The minimum ray parameter considered a valid hit.
/// - `f64` - The maximum ray parameter considered a valid hit.
///
/// # Returns
///
/// - `bool` - `true` when the segment may touch the AABB.
fn ray_segment_overlaps_aabb(
    origin: Vector3D,
    dir: Vector3D,
    lower: Vector3D,
    upper: Vector3D,
    t_min: f64,
    t_max: f64,
) -> bool {
    let x: Option<(f64, f64)> =
        slab_interval(origin.get_x(), dir.get_x(), lower.get_x(), upper.get_x());
    let y: Option<(f64, f64)> =
        slab_interval(origin.get_y(), dir.get_y(), lower.get_y(), upper.get_y());
    let z: Option<(f64, f64)> =
        slab_interval(origin.get_z(), dir.get_z(), lower.get_z(), upper.get_z());
    match (x, y, z) {
        (Some((x0, x1)), Some((y0, y1)), Some((z0, z1))) => {
            let t_enter: f64 = x0.max(y0).max(z0);
            let t_exit: f64 = x1.min(y1).min(z1);
            t_enter <= t_exit && t_exit >= t_min && t_enter <= t_max
        }
        _ => false,
    }
}

/// Intersects a ray with the triangle `v0`, `v1`, `v2` using the
/// Moller-Trumbore algorithm.
///
/// The test is two-sided: a hit on either face counts, and the returned
/// normal is flipped when needed so that it always faces against the ray
/// direction, making back-face hits usable for shading. A ray parallel to
/// the triangle plane yields `None`. No `t_min`/`t_max` filtering is
/// applied here; use [`Ray::intersect_triangle`] for the range-clamped
/// variant.
///
/// # Arguments
///
/// - `Vector3D` - The ray origin.
/// - `Vector3D` - The ray direction (expected to be unit length).
/// - `Vector3D` - The first triangle vertex.
/// - `Vector3D` - The second triangle vertex.
/// - `Vector3D` - The third triangle vertex.
///
/// # Returns
///
/// - `Option<(f64, Vector3D)>` - The hit distance along the ray and the
///   unit surface normal oriented against the ray direction, or `None` on
///   miss.
pub fn intersect_triangle(
    origin: Vector3D,
    dir: Vector3D,
    v0: Vector3D,
    v1: Vector3D,
    v2: Vector3D,
) -> Option<(f64, Vector3D)> {
    let edge1: Vector3D = v1 - v0;
    let edge2: Vector3D = v2 - v0;
    let pvec: Vector3D = dir.cross(edge2);
    let det: f64 = edge1.dot(pvec);
    if det.abs() < RAYTRACE_TRIANGLE_EPSILON {
        return None;
    }
    let inverse_det: f64 = 1.0 / det;
    let tvec: Vector3D = origin - v0;
    let u: f64 = tvec.dot(pvec) * inverse_det;
    if u < 0.0 || u > 1.0 {
        return None;
    }
    let qvec: Vector3D = tvec.cross(edge1);
    let v: f64 = dir.dot(qvec) * inverse_det;
    if v < 0.0 || u + v > 1.0 {
        return None;
    }
    let t: f64 = edge2.dot(qvec) * inverse_det;
    let geometric: Vector3D = edge1.cross(edge2).normalized();
    let normal: Vector3D = if geometric.dot(dir) > 0.0 {
        -geometric
    } else {
        geometric
    };
    Some((t, normal))
}

/// Returns the AABB extents `(min, max)` of an [`Occluder`].
///
/// For AABB occluders this is `(center, extent)`. For sphere occluders
/// the bounding box is computed from the center and the `.x` component of
/// `extent` (the sphere radius). For triangle occluders the bounding box
/// is the component-wise span of the three vertices.
///
/// # Arguments
///
/// - `&Occluder` - The occluder to bound.
///
/// # Returns
///
/// - `(Vector3D, Vector3D)` - The `(min, max)` corners of the AABB.
fn occluder_aabb_extents(occluder: &Occluder) -> (Vector3D, Vector3D) {
    let (mn, mx): (Vector3D, Vector3D) = match occluder.get_kind() {
        OccluderKind::Aabb => (occluder.get_center(), occluder.get_extent()),
        OccluderKind::Sphere => {
            let center: Vector3D = occluder.get_center();
            let radius: f64 = occluder.get_extent().get_x();
            let r: Vector3D = Vector3D::new(radius, radius, radius);
            (center - r, center + r)
        }
        OccluderKind::Triangle => {
            let [v0, v1, v2]: [Vector3D; 3] = occluder.get_vertices();
            let lo: Vector3D = vector3d_min(v0, vector3d_min(v1, v2));
            let hi: Vector3D = vector3d_max(v0, vector3d_max(v1, v2));
            (lo, hi)
        }
    };
    (vector3d_min(mn, mx), vector3d_max(mn, mx))
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
/// Every occluder is first run through a broad-phase AABB slab test
/// against the ray's `t_min`..=`t_max` segment; occluders whose bounds the
/// segment provably cannot touch are skipped without any exact math. The
/// test is conservative, so it only removes candidates that could not have
/// produced a hit, leaving the returned hit identical to a full scan.
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
        let (bound_min, bound_max): (Vector3D, Vector3D) = occluder_aabb_extents(occ);
        if !ray_segment_overlaps_aabb(origin, dir, bound_min, bound_max, t_min, t_max) {
            continue;
        }
        let candidate: Option<(f64, Vector3D)> = match occ.get_kind() {
            OccluderKind::Sphere => {
                let center: Vector3D = occ.get_center();
                let radius: f64 = occ.get_extent().get_x();
                match ray_sphere_intersect(origin, dir, center, radius) {
                    Some((t, n)) if t >= t_min && t <= t_max => Some((t, n)),
                    _ => None,
                }
            }
            OccluderKind::Aabb => match ray_aabb_intersect(origin, dir, bound_min, bound_max) {
                Some((t_near, _t_far, n)) if t_near >= t_min && t_near <= t_max => {
                    Some((t_near, n))
                }
                _ => None,
            },
            OccluderKind::Triangle => {
                let [v0, v1, v2]: [Vector3D; 3] = occ.get_vertices();
                ray.intersect_triangle(v0, v1, v2)
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
