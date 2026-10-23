use super::*;

/// Returns the component-wise minimum of two vectors.
///
/// # Arguments
///
/// - `Vector3D` - The first vector.
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

/// Reports whether every component of `value` is a finite number.
///
/// Degenerate geometry and non-finite ray data must never register a hit,
/// and comparison operators silently accept `NaN` on both sides, so every
/// exact test filters its inputs and its resulting `t` through this
/// predicate instead of relying on `<` and `>` alone.
///
/// # Arguments
///
/// - `Vector3D` - The vector to inspect.
///
/// # Returns
///
/// - `bool` - `true` when no component is `NaN` or infinite.
fn is_finite_vector(value: Vector3D) -> bool {
    value.get_x().is_finite() && value.get_y().is_finite() && value.get_z().is_finite()
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

/// Returns the clipped entry parameter of a ray segment against an AABB.
///
/// The result is `None` when the segment provably cannot touch the box.
/// The interval is clipped to the ray's own `t_min`..=`t_max`, so a
/// traversal that narrows `t_max` as it discovers nearer hits also prunes
/// the boxes behind those hits for free. Parallel slabs are handled inside
/// [`slab_interval`], so an axis-aligned ray running along a face still
/// reports the other two axes' constraint instead of dividing by zero.
///
/// # Arguments
///
/// - `&Ray` - The ray whose origin, direction and parameter range are used.
/// - `Vector3D` - The AABB minimum corner.
///
/// # Returns
///
/// - `Option<f64>` - The entry parameter clipped into the ray's range, or
///   `None` when the segment misses the box.
fn aabb_entry_parameter(ray: &Ray, lower: Vector3D, upper: Vector3D) -> Option<f64> {
    let origin: Vector3D = ray.get_origin();
    let dir: Vector3D = ray.get_direction();
    if !is_finite_vector(origin)
        || !is_finite_vector(dir)
        || !is_finite_vector(lower)
        || !is_finite_vector(upper)
    {
        return None;
    }
    let x: Option<(f64, f64)> =
        slab_interval(origin.get_x(), dir.get_x(), lower.get_x(), upper.get_x());
    let y: Option<(f64, f64)> =
        slab_interval(origin.get_y(), dir.get_y(), lower.get_y(), upper.get_y());
    let z: Option<(f64, f64)> =
        slab_interval(origin.get_z(), dir.get_z(), lower.get_z(), upper.get_z());
    let (x0, x1): (f64, f64) = x?;
    let (y0, y1): (f64, f64) = y?;
    let (z0, z1): (f64, f64) = z?;
    let t_enter: f64 = x0.max(y0).max(z0);
    let t_exit: f64 = x1.min(y1).min(z1);
    if t_enter > t_exit {
        return None;
    }
    let clipped: f64 = t_enter.max(ray.get_t_min());
    if clipped > t_exit.min(ray.get_t_max()) {
        return None;
    }
    Some(clipped)
}

/// Intersects a ray with a triangle using the Moller-Trumbore algorithm.
///
/// The test is two-sided: a hit on either face counts, and the returned
/// normal is flipped when needed so that it always faces against the ray
/// direction, making back-face hits usable for shading. A ray parallel to
/// the triangle plane yields `None`.
///
/// The ray's own `t_min`/`t_max` are deliberately not consulted: this free
/// function reports the raw distance along the ray however far away the
/// triangle is. [`Ray::intersect_triangle`] is the range-clamped variant and
/// the two must stay distinguishable at the call site.
///
/// # Arguments
///
/// - `&Ray` - The ray whose origin and direction are tested; its parameter
///   range is ignored.
/// - `[Vector3D; 3]` - The three triangle vertices in winding order.
///
/// # Returns
///
/// - `Option<(f64, Vector3D)>` - The hit distance along the ray and the
///   unit surface normal oriented against the ray direction, or `None` on
///   miss.
pub fn intersect_triangle(ray: &Ray, vertices: [Vector3D; 3]) -> Option<(f64, Vector3D)> {
    let [v0, v1, v2]: [Vector3D; 3] = vertices;
    let origin: Vector3D = ray.get_origin();
    let dir: Vector3D = ray.get_direction();
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
    if !(0.0..=1.0).contains(&u) {
        return None;
    }
    let qvec: Vector3D = tvec.cross(edge1);
    let v: f64 = dir.dot(qvec) * inverse_det;
    // `is_finite` is the guard, not a `neg_cmp`: a NaN barycentric coordinate
    // must fall through to the miss branch, and `!(x >= 0.0)` would also let a
    // NaN through on the negation while losing that intent.
    let v_usable: bool = v.is_finite() && (0.0..=1.0).contains(&v);
    let edge_usable: bool = (u + v).is_finite() && (u + v <= 1.0);
    if !v_usable || !edge_usable {
        return None;
    }
    let t: f64 = edge2.dot(qvec) * inverse_det;
    if !t.is_finite() {
        return None;
    }
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
///
/// # Arguments
///
/// - `&[Occluder]` - The occluders to flatten into bounding spheres.
///
/// # Returns
///
/// - `Vec<(Vector3D, f64)>` - The `(center, radius)` sphere of each occluder.
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

/// Returns the surface area of an AABB, the cost metric behind the
/// surface-area-heuristic split search.
///
/// # Arguments
///
/// - `Vector3D` - The AABB minimum corner.
///
/// # Returns
///
/// - `f64` - Twice the surface area, clamped to zero for flat or inverted
///   bounds.
fn aabb_surface_area(lower: Vector3D, upper: Vector3D) -> f64 {
    let dx: f64 = (upper.get_x() - lower.get_x()).max(0.0);
    let dy: f64 = (upper.get_y() - lower.get_y()).max(0.0);
    let dz: f64 = (upper.get_z() - lower.get_z()).max(0.0);
    2.0 * (dx * dy + dy * dz + dz * dx)
}

/// Returns the component of `value` on the axis selected by `axis`.
///
/// # Arguments
///
/// - `Vector3D` - The vector to read.
/// - `usize` - `0` for `x`, `1` for `y`, `2` for `z`.
///
/// # Returns
///
/// - `f64` - The selected component.
fn axis_component(value: Vector3D, axis: usize) -> f64 {
    match axis {
        0 => value.get_x(),
        1 => value.get_y(),
        _ => value.get_z(),
    }
}

/// Intersects a ray with a sphere, honouring the ray's parameter range.
///
/// Unlike the free function in `lighting`, this variant solves the
/// quadratic in general form so a non-unit direction reports true
/// distances, and it accepts the exit root when the origin is already
/// inside the sphere. A ray whose direction is zero, a negative radius, or
/// any `NaN` input never reports a hit.
///
/// The direction is read from the ray and never normalised, so a non-unit
/// direction reports a true distance in the ray's own parameterisation.
///
/// # Arguments
///
/// - `&Ray` - The ray whose origin, direction and parameter range are used.
/// - `Vector3D` - The sphere center.
/// - `f64` - The sphere radius.
///
/// # Returns
///
/// - `Option<(f64, Vector3D)>` - The hit parameter and the outward unit
///   normal, or `None` when the range-constrained ray misses.
fn intersect_sphere_in_range(ray: &Ray, center: Vector3D, radius: f64) -> Option<(f64, Vector3D)> {
    // A zero or negative radius describes no surface at all, and the
    // normalising step below would divide by its length, so reject both
    // before any arithmetic.
    if !radius.is_finite() || radius <= 0.0 {
        return None;
    }
    let origin: Vector3D = ray.get_origin();
    let dir: Vector3D = ray.get_direction();
    if !is_finite_vector(origin) || !is_finite_vector(dir) || !is_finite_vector(center) {
        return None;
    }
    let a: f64 = dir.dot(dir);
    // A zero-length or non-finite direction carries no ray: a NaN `a` would
    // divide every candidate by NaN, so the guard must reject it explicitly
    // rather than relying on a negated comparison.
    if !a.is_finite() || a <= 0.0 {
        return None;
    }
    let t_min: f64 = ray.get_t_min();
    let t_max: f64 = ray.get_t_max();
    let oc: Vector3D = origin - center;
    let half_b: f64 = oc.dot(dir);
    let c: f64 = oc.dot(oc) - radius * radius;
    let discriminant: f64 = half_b * half_b - a * c;
    // NaN and infinity must both miss: a negative discriminant is the miss
    // case, and a non-finite one has no real root to take a square root of.
    let discriminant_usable: bool = discriminant.is_finite() && discriminant >= 0.0;
    if !discriminant_usable {
        return None;
    }
    let root: f64 = discriminant.sqrt();
    let near: f64 = (-half_b - root) / a;
    let far: f64 = (-half_b + root) / a;
    let t: f64 = if near >= t_min && near <= t_max {
        near
    } else if far >= t_min && far <= t_max {
        far
    } else {
        return None;
    };
    let normal: Vector3D = (origin + dir.scaled(t) - center).normalized();
    Some((t, normal))
}

/// Intersects a ray with an AABB, honouring the ray's parameter range.
///
/// The slab test branches explicitly on a parallel axis instead of
/// dividing by the direction component, so a ray aligned with an axis is
/// decided by the remaining two axes rather than by `0 * infinity`
/// producing a `NaN` that comparison operators would silently accept.
/// When the origin is inside the box the exit parameter is reported, matching
/// the sphere behaviour so a shadow ray leaving a solid surface finds its way
/// out instead of being rejected as a self-hit. The reported normal is the
/// box's outward face normal, on entry and on exit alike.
///
/// # Arguments
///
/// - `&Ray` - The ray whose origin, direction and parameter range are used.
/// - `Vector3D` - The AABB minimum corner.
///
/// # Returns
///
/// - `Option<(f64, Vector3D)>` - The hit parameter and the outward unit
///   normal, or `None` when the range-constrained ray misses.
fn intersect_aabb_in_range(ray: &Ray, lower: Vector3D, upper: Vector3D) -> Option<(f64, Vector3D)> {
    let origin: Vector3D = ray.get_origin();
    let dir: Vector3D = ray.get_direction();
    let t_min: f64 = ray.get_t_min();
    let t_max: f64 = ray.get_t_max();
    if !is_finite_vector(origin) || !is_finite_vector(dir) {
        return None;
    }
    if !is_finite_vector(lower) || !is_finite_vector(upper) {
        return None;
    }
    let lo: Vector3D = vector3d_min(lower, upper);
    let hi: Vector3D = vector3d_max(lower, upper);
    // A box collapsed on any axis has no face a ray can strike: every point
    // of its slab interval is a single plane, so reporting it would invent a
    // surface where the caller supplied none.
    if lo.get_x() >= hi.get_x() || lo.get_y() >= hi.get_y() || lo.get_z() >= hi.get_z() {
        return None;
    }
    let origins: [f64; 3] = [origin.get_x(), origin.get_y(), origin.get_z()];
    let dirs: [f64; 3] = [dir.get_x(), dir.get_y(), dir.get_z()];
    let los: [f64; 3] = [lo.get_x(), lo.get_y(), lo.get_z()];
    let his: [f64; 3] = [hi.get_x(), hi.get_y(), hi.get_z()];
    let mut t_enter: f64 = f64::NEG_INFINITY;
    let mut t_exit: f64 = f64::INFINITY;
    let mut enter_axis: usize = 0;
    let mut exit_axis: usize = 0;
    for axis in 0..3 {
        let d: f64 = dirs[axis];
        let o: f64 = origins[axis];
        if d.abs() < RAYTRACE_TRIANGLE_EPSILON {
            if o < los[axis] || o > his[axis] {
                return None;
            }
            continue;
        }
        let inverse: f64 = 1.0 / d;
        let plane_a: f64 = (los[axis] - o) * inverse;
        let plane_b: f64 = (his[axis] - o) * inverse;
        // Dividing by a negative direction component swaps the two planes,
        // so the pair must be ordered before it is compared against the
        // running interval. Without this swap a ray travelling along a
        // negative axis records the far plane as the near one, which both
        // rejects genuine hits and reports the wrong face normal.
        let (near, far): (f64, f64) = if plane_a <= plane_b {
            (plane_a, plane_b)
        } else {
            (plane_b, plane_a)
        };
        if near > t_enter {
            t_enter = near;
            enter_axis = axis;
        }
        if far < t_exit {
            t_exit = far;
            exit_axis = axis;
        }
    }
    if t_enter > t_exit {
        return None;
    }
    // The origin is inside only when the whole slab interval straddles the
    // origin. Comparing `t_enter` to `t_min` alone is wrong for a ray that
    // begins outside the box but before it: there `t_enter` is positive yet
    // the box is still ahead of the origin, and reporting the exit would
    // skip the real surface. A box entirely behind the origin has
    // `t_exit < 0`, so it also fails this test and its entry parameter is
    // rejected by the range check below.
    let inside: bool = t_enter <= 0.0 && t_exit >= 0.0;
    let t: f64 = if inside { t_exit } else { t_enter };
    if !t.is_finite() {
        return None;
    }
    if t < t_min {
        return None;
    }
    if t > t_max {
        return None;
    }
    let axis: usize = if inside { exit_axis } else { enter_axis };
    // The normal is the box's own outward face normal, matching the
    // documented `Hit` contract: a ray entering through the low face meets
    // that face's outward direction, and a ray that started inside and
    // leaves through the high face carries that face's outward direction.
    // So the sign follows the face the ray meets, not the ray itself: entry
    // opposes the direction of travel, exit agrees with it.
    let travelling_positive: bool = dirs[axis] > 0.0;
    let sign: f64 = if inside {
        if travelling_positive { 1.0 } else { -1.0 }
    } else if travelling_positive {
        -1.0
    } else {
        1.0
    };
    let normal: Vector3D = match axis {
        0 => Vector3D::new(sign, 0.0, 0.0),
        1 => Vector3D::new(0.0, sign, 0.0),
        _ => Vector3D::new(0.0, 0.0, sign),
    };
    Some((t, normal))
}

/// Runs the exact intersection test for one occluder inside a ray's
/// parameter range.
///
/// This is the single source of truth for what counts as a hit: sphere,
/// AABB and triangle each dispatch to their own range-aware routine, and a
/// degenerate or non-finite occluder falls through to `None` rather than
/// registering a spurious surface.
///
/// # Arguments
///
/// - `&Occluder` - The occluder to test.
/// - `&Ray` - The ray whose origin, direction and parameter range are used.
///
/// # Returns
///
/// - `Option<(f64, Vector3D)>` - The hit parameter and the outward unit
///   normal, or `None` when the occluder is missed or degenerate.
fn intersect_occluder_in_range(occluder: &Occluder, ray: &Ray) -> Option<(f64, Vector3D)> {
    let t_min: f64 = ray.get_t_min();
    let t_max: f64 = ray.get_t_max();
    match occluder.get_kind() {
        OccluderKind::Sphere => {
            intersect_sphere_in_range(ray, occluder.get_center(), occluder.get_extent().get_x())
        }
        OccluderKind::Aabb => {
            let (lower, upper): (Vector3D, Vector3D) = occluder_aabb_extents(occluder);
            intersect_aabb_in_range(ray, lower, upper)
        }
        OccluderKind::Triangle => match intersect_triangle(ray, occluder.get_vertices()) {
            Some((t, normal)) if t.is_finite() && t >= t_min && t <= t_max => Some((t, normal)),
            _ => None,
        },
    }
}

/// Returns the tight AABB covering the primitives listed in a range.
///
/// # Arguments
///
/// - `&[usize]` - The primitive indexes of the range to measure.
/// - `&BvhBuild` - The per-primitive bounds and centroids.
/// - `BvhRange` - The primitives to measure.
///
/// # Returns
///
/// - `(Vector3D, Vector3D)` - The `(min, max)` corners covering the range.
fn bounds_of_range(indices: &[usize], build: &BvhBuild, range: BvhRange) -> (Vector3D, Vector3D) {
    let mut lower: Vector3D = Vector3D::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
    let mut upper: Vector3D =
        Vector3D::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);
    for slot in 0..range.count {
        let primitive: usize = indices[range.start + slot];
        lower = vector3d_min(lower, build.lower[primitive]);
        upper = vector3d_max(upper, build.upper[primitive]);
    }
    (lower, upper)
}

/// Returns the widest centroid axis of a range together with its midpoint.
///
/// This is the fallback split used when the binned heuristic finds no
/// worthwhile bin boundary. Selecting the axis by extent rather than by
/// sorting keeps a degenerate centroid cloud halvable.
///
/// # Arguments
///
/// - `&[usize]` - The primitive indexes of the range.
/// - `&BvhBuild` - The per-primitive bounds and centroids.
/// - `BvhRange` - The primitives to measure.
///
/// # Returns
///
/// - `SahSplit` - The axis and midpoint cutting the widest centroid spread.
fn longest_centroid_axis(indices: &[usize], build: &BvhBuild, range: BvhRange) -> SahSplit {
    let centroids: &[Vector3D] = &build.centroids;
    let mut best_axis: usize = 0;
    let mut best_span: f64 = -1.0;
    let mut best_mid: f64 = 0.0;
    for axis in 0..3 {
        let mut min: f64 = f64::INFINITY;
        let mut max: f64 = f64::NEG_INFINITY;
        for slot in 0..range.count {
            let value: f64 = axis_component(centroids[indices[range.start + slot]], axis);
            min = min.min(value);
            max = max.max(value);
        }
        let span: f64 = max - min;
        if span > best_span {
            best_span = span;
            best_axis = axis;
            best_mid = (min + max) * 0.5;
        }
    }
    SahSplit {
        axis: best_axis,
        boundary: best_mid,
    }
}

/// Partitions a range around a centroid split and reports where the left
/// partition ends.
///
/// The partition is an in-place two-way scan, so one level of the build
/// costs linear time instead of sorting every axis at every level.
/// Degenerate centroid sets can place every element on one side; the caller
/// detects a `0` or `count` result and falls back to halving the range,
/// which always makes progress.
///
/// # Arguments
///
/// - `&mut [usize]` - The primitive index array, partitioned in place.
/// - `&BvhBuild` - The per-primitive bounds and centroids.
/// - `BvhRange` - The range to partition.
/// - `SahSplit` - The centroid axis and split position.
///
/// # Returns
///
/// - `usize` - The number of entries placed on the left side.
fn partition_range(
    indices: &mut [usize],
    build: &BvhBuild,
    range: BvhRange,
    split: SahSplit,
) -> usize {
    let centroids: &[Vector3D] = &build.centroids;
    let mut left: usize = 0;
    let mut right: usize = range.count;
    while left < right {
        while left < right
            && axis_component(centroids[indices[range.start + left]], split.axis) < split.boundary
        {
            left += 1;
        }
        while right > left
            && axis_component(centroids[indices[range.start + right - 1]], split.axis)
                >= split.boundary
        {
            right -= 1;
        }
        if left >= right {
            break;
        }
        indices.swap(range.start + left, range.start + right - 1);
        left += 1;
        right -= 1;
    }
    left
}

/// Searches all three axes for the binned surface-area-heuristic split.
///
/// Each axis is cut into [`RAYTRACE_BVH_BINS`] bins by centroid, swept left
/// to right with a running cost, then swept right to left, so a level
/// costs linear time in the primitive count with no sorting at all. A
/// split is rejected when it cannot beat leaving the range as one leaf,
/// which keeps clustered or coincident geometry from being chopped into
/// ever thinner slices.
///
/// # Arguments
///
/// - `&[usize]` - The primitive indexes of the range to split.
/// - `&BvhBuild` - The per-primitive bounds and centroids.
/// - `BvhRange` - The range to split.
/// - `f64` - The surface area of the range's parent bounds.
///
/// # Returns
///
/// - `Option<SahSplit>` - The chosen axis and split position, or `None`
///   when no bin boundary improves on a leaf.
fn choose_sah_split(
    indices: &[usize],
    build: &BvhBuild,
    range: BvhRange,
    parent_area: f64,
) -> Option<SahSplit> {
    let count: usize = range.count;
    if count < 2 || !parent_area.is_finite() || parent_area <= 0.0 {
        return None;
    }
    let centroids: &[Vector3D] = &build.centroids;
    let mut best: Option<(usize, f64, f64)> = None;
    for axis in 0..3 {
        let mut centroid_min: f64 = f64::INFINITY;
        let mut centroid_max: f64 = f64::NEG_INFINITY;
        for slot in 0..count {
            let value: f64 = axis_component(centroids[indices[range.start + slot]], axis);
            centroid_min = centroid_min.min(value);
            centroid_max = centroid_max.max(value);
        }
        let span: f64 = centroid_max - centroid_min;
        if !span.is_finite() || span <= RAYTRACE_BVH_AXIS_EPSILON {
            continue;
        }
        let scale: f64 = RAYTRACE_BVH_BINS as f64 / span;
        let mut bin_lower: [Vector3D; RAYTRACE_BVH_BINS] = [Vector3D::zero(); RAYTRACE_BVH_BINS];
        let mut bin_upper: [Vector3D; RAYTRACE_BVH_BINS] = [Vector3D::zero(); RAYTRACE_BVH_BINS];
        let mut bin_count: [usize; RAYTRACE_BVH_BINS] = [0; RAYTRACE_BVH_BINS];
        for slot in 0..count {
            let primitive: usize = indices[range.start + slot];
            let scaled: f64 = (axis_component(centroids[primitive], axis) - centroid_min) * scale;
            let mut bin: usize = scaled as usize;
            if bin >= RAYTRACE_BVH_BINS {
                bin = RAYTRACE_BVH_BINS - 1;
            }
            if bin_count[bin] == 0 {
                bin_lower[bin] = build.lower[primitive];
                bin_upper[bin] = build.upper[primitive];
            } else {
                bin_lower[bin] = vector3d_min(bin_lower[bin], build.lower[primitive]);
                bin_upper[bin] = vector3d_max(bin_upper[bin], build.upper[primitive]);
            }
            bin_count[bin] += 1;
        }
        let mut left_area: [f64; RAYTRACE_BVH_BINS] = [0.0; RAYTRACE_BVH_BINS];
        let mut left_count: [usize; RAYTRACE_BVH_BINS] = [0; RAYTRACE_BVH_BINS];
        let mut running_lower: Vector3D =
            Vector3D::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut running_upper: Vector3D =
            Vector3D::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);
        let mut running_count: usize = 0;
        for bin in 0..RAYTRACE_BVH_BINS {
            if bin_count[bin] > 0 {
                running_lower = vector3d_min(running_lower, bin_lower[bin]);
                running_upper = vector3d_max(running_upper, bin_upper[bin]);
                running_count += bin_count[bin];
            }
            left_area[bin] = aabb_surface_area(running_lower, running_upper);
            left_count[bin] = running_count;
        }
        let mut right_lower: Vector3D = Vector3D::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut right_upper: Vector3D =
            Vector3D::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);
        let mut right_count: usize = 0;
        for bin in (1..RAYTRACE_BVH_BINS).rev() {
            if bin_count[bin] > 0 {
                right_lower = vector3d_min(right_lower, bin_lower[bin]);
                right_upper = vector3d_max(right_upper, bin_upper[bin]);
                right_count += bin_count[bin];
            }
            let left_total: usize = left_count[bin - 1];
            if left_total == 0 || right_count == 0 {
                continue;
            }
            let cost: f64 = left_area[bin - 1] * left_total as f64
                + aabb_surface_area(right_lower, right_upper) * right_count as f64;
            if matches!(best, Some((_, _, incumbent)) if cost >= incumbent) {
                continue;
            }
            best = Some((axis, centroid_min + bin as f64 / scale, cost));
        }
    }
    let (axis, boundary, cost): (usize, f64, f64) = best?;
    let incumbent: f64 = count as f64 * parent_area;
    if !cost.is_finite() || cost >= incumbent {
        return None;
    }
    Some(SahSplit { axis, boundary })
}

/// Builds one BVH subtree in preorder, appending its root and descendants.
///
/// The root is pushed before its children so the array stays a dense
/// preorder sequence: the left child lands immediately after the root, and
/// the escape is back-patched once both subtrees have been emitted. A
/// range that already fits in a leaf, or that has reached
/// [`RAYTRACE_BVH_MAX_DEPTH`], is never split: the depth cap is what keeps
/// the finished hierarchy inside the fixed traversal stack, because the
/// heuristic can otherwise chain linearly on a geometric spread.
///
/// The node array and the index array it permutes live in `context`, and
/// `subtree` carries only this call's own node, so recursion allocates
/// nothing beyond the frames themselves.
///
/// # Arguments
///
/// - `&mut BvhBuildContext` - The node array, index array and per-primitive
///   build data shared by every step of this build.
/// - `BvhSubtree` - The bounds, primitive range and nesting level of the
///   node to emit.
fn build_bvh_subtree(context: &mut BvhBuildContext, subtree: BvhSubtree) {
    let root: usize = context.nodes.len();
    let bounds: (Vector3D, Vector3D) = subtree.bounds;
    let range: BvhRange = subtree.range;
    let too_small_to_split: bool = range.count <= RAYTRACE_BVH_LEAF_SIZE || range.count < 2;
    if too_small_to_split || subtree.depth >= RAYTRACE_BVH_MAX_DEPTH {
        context.nodes.push(RayTraceNode {
            lower: bounds.0,
            upper: bounds.1,
            start: range.start,
            count: range.count,
            escape: root + 1,
        });
        return;
    }
    let build: &BvhBuild = context.build;
    let split: SahSplit = match choose_sah_split(
        context.indices,
        build,
        range,
        aabb_surface_area(bounds.0, bounds.1),
    ) {
        Some(chosen) => chosen,
        None => longest_centroid_axis(context.indices, build, range),
    };
    let partitioned: usize = partition_range(context.indices, build, range, split);
    let left_count: usize = if partitioned == 0 || partitioned == range.count {
        range.count / 2
    } else {
        partitioned
    };
    context.nodes.push(RayTraceNode {
        lower: bounds.0,
        upper: bounds.1,
        start: range.start,
        count: 0,
        escape: root + 1,
    });
    let left_range: BvhRange = BvhRange {
        start: range.start,
        count: left_count,
    };
    let left_bounds: (Vector3D, Vector3D) = bounds_of_range(context.indices, build, left_range);
    build_bvh_subtree(
        context,
        BvhSubtree {
            bounds: left_bounds,
            range: left_range,
            depth: subtree.depth + 1,
        },
    );
    let right_range: BvhRange = BvhRange {
        start: range.start + left_count,
        count: range.count - left_count,
    };
    let right_bounds: (Vector3D, Vector3D) = bounds_of_range(context.indices, build, right_range);
    build_bvh_subtree(
        context,
        BvhSubtree {
            bounds: right_bounds,
            range: right_range,
            depth: subtree.depth + 1,
        },
    );
    context.nodes[root].escape = context.nodes.len();
}

/// Builds the flat BVH for a list of occluders.
///
/// The result is the pair of arrays [`RayTraceScene`] stores: preorder
/// nodes and the permuted primitive index list the leaves address. The
/// hierarchy is built exactly once at construction, so no query pays for
/// it.
///
/// # Arguments
///
/// - `&[Occluder]` - The occluders to bound.
///
/// # Returns
///
/// - `(Vec<RayTraceNode>, Vec<usize>)` - The preorder node array and the
///   reordered primitive indexes; both empty for an empty scene.
pub(crate) fn build_bvh(occluders: &[Occluder]) -> (Vec<RayTraceNode>, Vec<usize>) {
    let count: usize = occluders.len();
    let mut nodes: Vec<RayTraceNode> = Vec::new();
    let mut indices: Vec<usize> = Vec::with_capacity(count);
    if count == 0 {
        return (nodes, indices);
    }
    let mut lower: Vec<Vector3D> = Vec::with_capacity(count);
    let mut upper: Vec<Vector3D> = Vec::with_capacity(count);
    let mut centroids: Vec<Vector3D> = Vec::with_capacity(count);
    for (index, occluder) in occluders.iter().enumerate() {
        let (lo, hi): (Vector3D, Vector3D) = occluder_aabb_extents(occluder);
        let midpoint: Vector3D = (lo + hi).scaled(0.5);
        let centroid: Vector3D = if is_finite_vector(midpoint) {
            midpoint
        } else {
            Vector3D::zero()
        };
        lower.push(lo);
        upper.push(hi);
        centroids.push(centroid);
        indices.push(index);
    }
    let build: BvhBuild = BvhBuild {
        lower,
        upper,
        centroids,
    };
    let root_range: BvhRange = BvhRange { start: 0, count };
    let root_bounds: (Vector3D, Vector3D) = bounds_of_range(&indices, &build, root_range);
    nodes.reserve(count.saturating_mul(2));
    let mut context: BvhBuildContext = BvhBuildContext {
        nodes: &mut nodes,
        indices: &mut indices,
        build: &build,
    };
    build_bvh_subtree(
        &mut context,
        BvhSubtree {
            bounds: root_bounds,
            range: root_range,
            depth: 0,
        },
    );
    (nodes, indices)
}

/// Reports whether a ray's range and geometry are usable for a query.
///
/// An inverted or non-finite range, or a non-finite origin or direction,
/// cannot produce a meaningful intersection, so both query entry points
/// reject such rays instead of letting `NaN` comparisons decide.
///
/// # Arguments
///
/// - `&Ray` - The ray to validate.
///
/// # Returns
///
/// - `bool` - `true` when the ray may be traced.
fn ray_is_traceable(ray: &Ray) -> bool {
    let t_min: f64 = ray.get_t_min();
    let t_max: f64 = ray.get_t_max();
    if !t_min.is_finite() || !t_max.is_finite() || t_min > t_max {
        return false;
    }
    is_finite_vector(ray.get_origin()) && is_finite_vector(ray.get_direction())
}

/// Returns the direction a ray may be traced with.
///
/// The direction is deliberately passed through unchanged. `t` is consumed
/// by `Ray::at(t)`, which advances along the raw direction, so any
/// rescaling here would make the reported parameter disagree with the point
/// the caller reconstructs from it. Each shape therefore solves for `t` in
/// the ray's own parameterisation: the sphere uses the full quadratic with
/// `a = d . d`, and the slab test divides by each direction component.
///
/// # Arguments
///
/// - `&Ray` - The ray to validate.
///
/// # Returns
///
/// - `Option<Vector3D>` - The original direction, or `None` when the ray is
///   degenerate and cannot be traced without a division blow-up.
fn traceable_direction_of(ray: &Ray) -> Option<Vector3D> {
    if !ray_is_traceable(ray) {
        return None;
    }
    let dir: Vector3D = ray.get_direction();
    let squared_length: f64 = dir.dot(dir);
    if !squared_length.is_finite() || squared_length <= 0.0 {
        return None;
    }
    Some(dir)
}

/// Runs the closest-hit query as a straight scan over every primitive.
///
/// Used for scenes too small for the hierarchy to pay for itself. The tie
/// rule is identical to the traversal's: a nearer hit always wins, and an
/// exact tie is resolved towards the lower original index, so the linear
/// and hierarchical paths cannot disagree on the same scene.
///
/// # Arguments
///
/// - `&Ray` - The ray being traced, carrying origin, direction and range.
/// - `&[Occluder]` - Every primitive in the scene.
///
/// # Returns
///
/// - `Option<(usize, f64, Vector3D, Vector3D)>` - The winning primitive
///   index, the hit parameter, the hit point and the surface normal.
fn linear_closest_hit(
    ray: &Ray,
    occluders: &[Occluder],
) -> Option<(usize, f64, Vector3D, Vector3D)> {
    let origin: Vector3D = ray.get_origin();
    let dir: Vector3D = ray.get_direction();
    let mut best: Option<(usize, f64, Vector3D, Vector3D)> = None;
    for (index, occluder) in occluders.iter().enumerate() {
        let hit: Option<(f64, Vector3D)> = intersect_occluder_in_range(occluder, ray);
        let Some((t, normal)) = hit else {
            continue;
        };
        let nearer: bool = match &best {
            None => true,
            Some((best_index, best_t, _, _)) => {
                t < *best_t || (t == *best_t && index < *best_index)
            }
        };
        if nearer {
            best = Some((index, t, origin + dir.scaled(t), normal));
        }
    }
    best
}

/// Runs the any-hit query as a straight scan over every primitive.
///
/// # Arguments
///
/// - `&Ray` - The ray being tested, carrying origin, direction and range.
/// - `&[Occluder]` - Every primitive in the scene.
///
/// # Returns
///
/// - `bool` - Whether any primitive blocks the ray.
fn linear_occluded(ray: &Ray, occluders: &[Occluder]) -> bool {
    occluders
        .iter()
        .any(|occluder: &Occluder| intersect_occluder_in_range(occluder, ray).is_some())
}

/// Finds the closest intersection between a ray and a scene's BVH without
/// touching any [`Material`].
///
/// Traversal walks the preorder node array with a fixed-size stack held on
/// the caller's frame, so a query allocates nothing. Each node is tested
/// against the current best distance, which prunes whole subtrees, and the
/// nearer child is pushed last so it pops first. Ties are broken by the
/// lowest original primitive index rather than by traversal order, so the
/// result is stable no matter how the tree was built.
///
/// # Arguments
///
/// - `&Ray` - The ray to test.
/// - `&RayTraceScene` - The scene holding the hierarchy.
///
/// # Returns
///
/// - `Option<(usize, f64, Vector3D, Vector3D)>` - The occluder index, the
///   hit distance `t`, the hit position, and the surface normal, or `None`
///   if the ray misses.
pub(crate) fn closest_hit_indexed(
    ray: &Ray,
    scene: &RayTraceScene,
) -> Option<(usize, f64, Vector3D, Vector3D)> {
    // The direction is validated once here rather than threaded through the
    // traversal: every node test re-reads it from `ray` unchanged, so no
    // rescaled copy can drift away from the parameterisation `Ray::at`
    // reconstructs points in.
    traceable_direction_of(ray)?;
    let nodes: &[RayTraceNode] = scene.get_nodes();
    if nodes.is_empty() {
        return None;
    }
    let occluders: &[Occluder] = scene.get_occluders();
    if occluders.len() <= RAYTRACE_LINEAR_MAX_PRIMITIVES {
        return linear_closest_hit(ray, occluders);
    }
    let indices: &[usize] = scene.get_indices();
    let origin: Vector3D = ray.get_origin();
    let t_min: f64 = ray.get_t_min();
    let mut best_t: f64 = ray.get_t_max();
    let mut best_index: usize = 0;
    let mut best_normal: Vector3D = Vector3D::zero();
    let mut found: bool = false;
    let mut stack: [usize; RAYTRACE_BVH_STACK_SIZE] = [0usize; RAYTRACE_BVH_STACK_SIZE];
    let root: &RayTraceNode = &nodes[0];
    let dir: Vector3D = ray.get_direction();
    // The root's entry parameter is only a probe: a root the ray misses means
    // the whole subtree is out of range, so `?` on the discarded value is the
    // early return clippy asks for.
    let _root_entry: f64 = aabb_entry_parameter(ray, root.get_lower(), root.get_upper())?;
    stack[0] = 0usize;
    let mut depth: usize = 1;
    while depth > 0 {
        depth -= 1;
        let node_index: usize = stack[depth];
        let node: RayTraceNode = nodes[node_index];
        if node.get_count() == 0 {
            let left_index: usize = node_index + 1;
            let right_index: usize = nodes[left_index].get_escape();
            let left: &RayTraceNode = &nodes[left_index];
            let right: &RayTraceNode = &nodes[right_index];
            let probe: Ray = ray.with_range(t_min, best_t);
            let left_entry: Option<f64> =
                aabb_entry_parameter(&probe, left.get_lower(), left.get_upper());
            let right_entry: Option<f64> =
                aabb_entry_parameter(&probe, right.get_lower(), right.get_upper());
            match (left_entry, right_entry) {
                (None, None) => {}
                (Some(_), None) => {
                    stack[depth] = left_index;
                    depth += 1;
                }
                (None, Some(_)) => {
                    stack[depth] = right_index;
                    depth += 1;
                }
                (Some(left_t), Some(right_t)) => {
                    let near_first: usize = if left_t <= right_t {
                        left_index
                    } else {
                        right_index
                    };
                    stack[depth] = if left_t <= right_t {
                        right_index
                    } else {
                        left_index
                    };
                    depth += 1;
                    stack[depth] = near_first;
                    depth += 1;
                }
            }
            continue;
        }
        let start: usize = node.get_start();
        let mut probe: Ray = ray.with_range(t_min, best_t);
        for slot in 0..node.get_count() {
            let primitive: usize = indices[start + slot];
            let hit: Option<(f64, Vector3D)> =
                intersect_occluder_in_range(&occluders[primitive], &probe);
            if let Some((t, normal)) = hit {
                let nearer: bool = t < best_t;
                let stable_tie: bool = t == best_t && (!found || primitive < best_index);
                if nearer || stable_tie {
                    best_t = t;
                    best_index = primitive;
                    best_normal = normal;
                    found = true;
                    probe = ray.with_range(t_min, best_t);
                }
            }
        }
    }
    if !found {
        return None;
    }
    Some((best_index, best_t, origin + dir.scaled(best_t), best_normal))
}

/// Reports whether any occluder blocks the ray's parameter range.
///
/// This is the any-hit counterpart to [`closest_hit_indexed`]: it returns
/// as soon as one occluder is hit, so a shadow ray stops at the first
/// blocker instead of resolving the whole hierarchy.
///
/// # Arguments
///
/// - `&Ray` - The ray to test.
/// - `&RayTraceScene` - The scene holding the hierarchy.
///
/// # Returns
///
/// - `bool` - `true` when at least one occluder intersects the range.
pub(crate) fn occluded_indexed(ray: &Ray, scene: &RayTraceScene) -> bool {
    if traceable_direction_of(ray).is_none() {
        return false;
    }
    let occluders: &[Occluder] = scene.get_occluders();
    if occluders.len() <= RAYTRACE_LINEAR_MAX_PRIMITIVES {
        return linear_occluded(ray, occluders);
    }
    let nodes: &[RayTraceNode] = scene.get_nodes();
    if nodes.is_empty() {
        return false;
    }
    let indices: &[usize] = scene.get_indices();
    let mut stack: [usize; RAYTRACE_BVH_STACK_SIZE] = [0usize; RAYTRACE_BVH_STACK_SIZE];
    let root: &RayTraceNode = &nodes[0];
    if aabb_entry_parameter(ray, root.get_lower(), root.get_upper()).is_none() {
        return false;
    }
    stack[0] = 0usize;
    let mut depth: usize = 1;
    while depth > 0 {
        depth -= 1;
        let node_index: usize = stack[depth];
        let node: RayTraceNode = nodes[node_index];
        if node.get_count() == 0 {
            let left_index: usize = node_index + 1;
            let right_index: usize = nodes[left_index].get_escape();
            for child_index in [left_index, right_index] {
                let child: &RayTraceNode = &nodes[child_index];
                let entry: Option<f64> =
                    aabb_entry_parameter(ray, child.get_lower(), child.get_upper());
                if entry.is_some() {
                    stack[depth] = child_index;
                    depth += 1;
                }
            }
            continue;
        }
        let start: usize = node.get_start();
        for slot in 0..node.get_count() {
            let primitive: usize = indices[start + slot];
            let blocked: bool = intersect_occluder_in_range(&occluders[primitive], ray).is_some();
            if blocked {
                return true;
            }
        }
    }
    false
}

/// Builds a reflected ray bouncing off a surface point with the given
/// normal.
///
/// # Arguments
///
/// - `&Ray` - The incoming ray.
/// - `Vector3D` - The world-space hit position.
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

/// Builds the shadow visibility callback consumed by
/// `LightingUniforms::shade_with_visibility`.
///
/// The callback turns a light into one exact any-hit query against the
/// scene's BVH. The ray leaves the shading point along the surface normal
/// rather than the surface itself, so a lit surface never shadows itself
/// through its own footprint, and the segment stops short of a finite light
/// position so geometry enclosing the emitter does not stay dark. A
/// surface already facing away from the light is reported as lit without
/// spending a query at all.
///
/// The returned factor is the full `1.0` / `0.0` visibility the shader
/// path also uses, so CPU and GPU agree on what "shadowed" means.
///
/// # Arguments
///
/// - `&'a RayTraceScene` - The scene to query for blockers.
/// - `Vector3D` - The world-space shading position.
///
/// # Returns
/// - `impl FnMut(&Light, Vector3D, f64) -> f64` - A callback returning the
///   light's visibility from the shading point.
pub(crate) fn shadow_visibility<'a>(
    scene: &'a RayTraceScene,
    position: Vector3D,
    normal: Vector3D,
) -> impl FnMut(&Light, Vector3D, f64) -> f64 + 'a {
    let origin: Vector3D = position + normal.scaled(RAYTRACE_SHADOW_SELF_OFFSET);
    move |light: &Light, light_direction: Vector3D, max_distance: f64| {
        let facing: f64 = normal.dot(light_direction);
        if facing <= 0.0 {
            return 1.0;
        }
        let length: f64 = light_direction.magnitude();
        let distance_usable: bool =
            length.is_finite() && length > 0.0 && max_distance.is_finite() && max_distance > 0.0;
        if !distance_usable {
            return 1.0;
        }
        let limit: f64 = match light.get_kind() {
            LightType::Directional => max_distance,
            LightType::Point | LightType::Spot => max_distance - RAYTRACE_SHADOW_LIGHT_EPSILON,
        };
        let limit: f64 = if limit > RAYTRACE_SHADOW_SELF_OFFSET {
            limit
        } else {
            return 1.0;
        };
        let dir: Vector3D = light_direction.scaled(1.0 / length);
        let ray: Ray = Ray::new(origin, dir).with_range(RAYTRACE_SHADOW_SELF_OFFSET, limit);
        let blocked: bool = occluded_indexed(&ray, scene);
        if blocked { 0.0 } else { 1.0 }
    }
}

/// Iteratively traces a ray against a scene using its BVH, performing no
/// heap allocation per bounce.
///
/// Shadows are resolved exactly through the scene's own any-hit traversal
/// rather than through the legacy bounding-sphere list, so a traced hit is
/// shaded with the same visibility the GPU path computes. Color
/// contributions are accumulated with a specular throughput: each bounce
/// multiplies the throughput by the hit material's specular intensity, and
/// a miss adds the ambient color scaled by the current throughput. The
/// bounce loop stops when the ray misses, when `depth` reaches
/// `max_bounces`, or when the hit material's specular intensity is not
/// greater than [`EPSILON`], matching the behavior of the historical
/// recursive formulation.
///
/// # Arguments
///
/// - `Ray` - The ray to trace.
/// - `&RayTraceScene` - The scene to trace through; its BVH resolves every
///   closest-hit and every shadow query.
/// - `&LightingUniforms` - Lighting parameters used during shading.
/// - `u32` - The maximum number of bounces allowed for this ray.
///
/// # Returns
///
/// - `Vector3D` - The final traced color.
pub(crate) fn trace_bounces(
    ray: Ray,
    scene: &RayTraceScene,
    lights: &LightingUniforms,
    max_bounces: u32,
) -> Vector3D {
    let ambient: Vector3D = lights.get_ambient();
    let mut color: Vector3D = Vector3D::zero();
    let mut throughput: f64 = 1.0;
    let mut current: Ray = ray;
    loop {
        let (index, _t, position, normal): (usize, f64, Vector3D, Vector3D) =
            match closest_hit_indexed(&current, scene) {
                None => {
                    color += ambient.scaled(throughput);
                    break;
                }
                Some(hit) => hit,
            };
        let material: &Material = scene.get_occluders()[index].get_material();
        let shaded: Vector3D = lights.shade_with_visibility(
            position,
            normal,
            material,
            shadow_visibility(scene, position, normal),
        );
        color += shaded.scaled(throughput);
        let spec: f64 = material.get_specular();
        if current.get_depth() >= max_bounces || spec <= EPSILON {
            break;
        }
        throughput *= spec;
        current = bounce_ray(&current, position, normal);
    }
    color
}
