use super::*;

const EPSILON: f64 = 1e-9;

const SHADOW_SELF_OFFSET: f64 = 0.001;

fn lcg(state: &mut u64) -> f64 {
    *state = state
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    ((*state >> 33) as f64) / 2147483648.0 - 1.0
}

fn occluder_bounds(occluder: &Occluder) -> (Vector3D, Vector3D) {
    match occluder.get_kind() {
        OccluderKind::Aabb => (
            Vector3D::new(
                occluder
                    .get_center()
                    .get_x()
                    .min(occluder.get_extent().get_x()),
                occluder
                    .get_center()
                    .get_y()
                    .min(occluder.get_extent().get_y()),
                occluder
                    .get_center()
                    .get_z()
                    .min(occluder.get_extent().get_z()),
            ),
            Vector3D::new(
                occluder
                    .get_center()
                    .get_x()
                    .max(occluder.get_extent().get_x()),
                occluder
                    .get_center()
                    .get_y()
                    .max(occluder.get_extent().get_y()),
                occluder
                    .get_center()
                    .get_z()
                    .max(occluder.get_extent().get_z()),
            ),
        ),
        OccluderKind::Sphere => {
            let r: f64 = occluder.get_extent().get_x().abs();
            let offset: Vector3D = Vector3D::new(r, r, r);
            (
                occluder.get_center() - offset,
                occluder.get_center() + offset,
            )
        }
        OccluderKind::Triangle => {
            let [v0, v1, v2]: [Vector3D; 3] = occluder.get_vertices();
            let lo: Vector3D = Vector3D::new(
                v0.get_x().min(v1.get_x()).min(v2.get_x()),
                v0.get_y().min(v1.get_y()).min(v2.get_y()),
                v0.get_z().min(v1.get_z()).min(v2.get_z()),
            );
            let hi: Vector3D = Vector3D::new(
                v0.get_x().max(v1.get_x()).max(v2.get_x()),
                v0.get_y().max(v1.get_y()).max(v2.get_y()),
                v0.get_z().max(v1.get_z()).max(v2.get_z()),
            );
            (lo, hi)
        }
    }
}

fn axis_value(value: Vector3D, axis: usize) -> f64 {
    let parts: [f64; 3] = [value.get_x(), value.get_y(), value.get_z()];
    parts[axis]
}

fn brute_sphere(ray: &Ray, center: Vector3D, radius: f64) -> Option<f64> {
    if !radius.is_finite() || radius < 0.0 {
        return None;
    }
    let dir: Vector3D = ray.get_direction();
    let t_min: f64 = ray.get_t_min();
    let t_max: f64 = ray.get_t_max();
    let oc: Vector3D = ray.get_origin() - center;
    let a: f64 = dir.dot(dir);
    if !a.is_finite() || a <= 0.0 {
        return None;
    }
    let half_b: f64 = oc.dot(dir);
    let c: f64 = oc.dot(oc) - radius * radius;
    let discriminant: f64 = half_b * half_b - a * c;
    if !discriminant.is_finite() || discriminant < 0.0 {
        return None;
    }
    let root: f64 = discriminant.sqrt();
    let near: f64 = (-half_b - root) / a;
    let far: f64 = (-half_b + root) / a;
    if near >= t_min && near <= t_max {
        return Some(near);
    }
    if far >= t_min && far <= t_max {
        return Some(far);
    }
    None
}

fn brute_aabb(ray: &Ray, lower: Vector3D, upper: Vector3D) -> Option<f64> {
    let origin: Vector3D = ray.get_origin();
    let dir: Vector3D = ray.get_direction();
    let t_min: f64 = ray.get_t_min();
    let t_max: f64 = ray.get_t_max();
    let lo: Vector3D = Vector3D::new(
        lower.get_x().min(upper.get_x()),
        lower.get_y().min(upper.get_y()),
        lower.get_z().min(upper.get_z()),
    );
    let hi: Vector3D = Vector3D::new(
        lower.get_x().max(upper.get_x()),
        lower.get_y().max(upper.get_y()),
        lower.get_z().max(upper.get_z()),
    );
    let mut t_enter: f64 = f64::NEG_INFINITY;
    let mut t_exit: f64 = f64::INFINITY;
    for axis in 0..3 {
        let d: f64 = axis_value(dir, axis);
        let o: f64 = axis_value(origin, axis);
        let slab_lo: f64 = axis_value(lo, axis);
        let slab_hi: f64 = axis_value(hi, axis);
        if d.abs() < 1e-8 {
            if o < slab_lo || o > slab_hi {
                return None;
            }
            continue;
        }
        let inverse: f64 = 1.0 / d;
        let near: f64 = (slab_lo - o) * inverse;
        let far: f64 = (slab_hi - o) * inverse;
        t_enter = t_enter.max(near.min(far));
        t_exit = t_exit.min(near.max(far));
    }
    if t_enter > t_exit {
        return None;
    }
    let inside: bool = t_enter <= t_min;
    let t: f64 = if inside { t_exit } else { t_enter };
    if t < t_min || t > t_max {
        return None;
    }
    Some(t)
}

fn brute_hit(ray: &Ray, occluders: &[Occluder]) -> Option<(usize, f64)> {
    let t_min: f64 = ray.get_t_min();
    let t_max: f64 = ray.get_t_max();
    if !t_min.is_finite() || !t_max.is_finite() || t_min > t_max {
        return None;
    }
    let dir: Vector3D = ray.get_direction();
    let squared_length: f64 = dir.dot(dir);
    if !squared_length.is_finite() || squared_length <= 0.0 {
        return None;
    }
    let mut best: Option<(usize, f64)> = None;
    for (index, occluder) in occluders.iter().enumerate() {
        let candidate: Option<f64> = match occluder.get_kind() {
            OccluderKind::Sphere => {
                brute_sphere(ray, occluder.get_center(), occluder.get_extent().get_x())
            }
            OccluderKind::Aabb => {
                let (lower, upper): (Vector3D, Vector3D) = occluder_bounds(occluder);
                brute_aabb(ray, lower, upper)
            }
            OccluderKind::Triangle => match intersect_triangle(ray, occluder.get_vertices()) {
                Some((t, _)) if t >= t_min && t <= t_max => Some(t),
                _ => None,
            },
        };
        if let Some(t) = candidate {
            let wins: bool = match best {
                None => true,
                Some((best_index, best_t)) => t < best_t || (t == best_t && index < best_index),
            };
            if wins {
                best = Some((index, t));
            }
        }
    }
    best
}

fn brute_occluded(ray: &Ray, occluders: &[Occluder]) -> bool {
    brute_hit(ray, occluders).is_some()
}

fn mixed_scene(count: usize, seed: u64) -> Vec<Occluder> {
    let mut state: u64 = seed;
    let mut occluders: Vec<Occluder> = Vec::new();
    for index in 0..count {
        let center: Vector3D = Vector3D::new(
            lcg(&mut state) * 6.0,
            lcg(&mut state) * 4.0,
            lcg(&mut state) * 6.0,
        );
        let material: Material = Material::lambert(Vector3D::new(0.5, 0.5, 0.5));
        match index % 3 {
            0 => occluders.push(Occluder::sphere(
                center,
                0.2 + lcg(&mut state).abs(),
                material,
            )),
            1 => occluders.push(Occluder::aabb(
                center,
                center + Vector3D::new(0.6, 0.4, 0.5),
                material,
            )),
            _ => occluders.push(Occluder::triangle(
                center,
                center + Vector3D::new(0.7, 0.0, 0.1),
                center + Vector3D::new(0.0, 0.5, 0.8),
                material,
            )),
        }
    }
    occluders
}

#[test]
fn bvh_closest_hit_matches_a_brute_force_scan_over_a_mixed_scene() {
    let occluders: Vec<Occluder> = mixed_scene(96, 0x5eed_1234);
    let scene: RayTraceScene = RayTraceScene::new(occluders.clone());
    let mut state: u64 = 0xabcd_9876;
    let mut compared: u32 = 0;
    let mut hits: u32 = 0;
    for _ in 0..3000 {
        let raw: Vector3D = Vector3D::new(lcg(&mut state), lcg(&mut state), lcg(&mut state));
        let magnitude: f64 = raw.magnitude();
        if !magnitude.is_finite() || magnitude <= 1e-6 {
            continue;
        }
        let origin: Vector3D = Vector3D::new(
            lcg(&mut state) * 5.0,
            lcg(&mut state) * 3.0 + 1.0,
            lcg(&mut state) * 5.0,
        );
        let ray: Ray = Ray::new(origin, raw.scaled(1.0 / magnitude));
        let expected: Option<(usize, f64)> = brute_hit(&ray, &occluders);
        let actual: Option<Hit> = scene.closest_hit(&ray);
        compared += 1;
        if expected.is_some() {
            hits += 1;
        }
        match (expected, actual) {
            (None, None) => {}
            (Some((_expected_index, expected_t)), Some(hit)) => {
                assert!(
                    (hit.get_t() - expected_t).abs() < 1e-9,
                    "the BVH reported t={} but a brute-force scan returns {expected_t}",
                    hit.get_t(),
                );
                let expected_position: Vector3D =
                    ray.get_origin() + ray.get_direction().scaled(expected_t);
                assert!(
                    (hit.get_position().get_x() - expected_position.get_x()).abs() < 1e-9
                        && (hit.get_position().get_y() - expected_position.get_y()).abs() < 1e-9
                        && (hit.get_position().get_z() - expected_position.get_z()).abs() < 1e-9,
                    "the reported position must sit on the ray at the brute-force distance",
                );
            }
            (expected, actual) => {
                panic!("the BVH returned {actual:?} but a brute-force scan returns {expected:?}")
            }
        }
    }
    assert!(compared >= 2900, "too few rays took part, got {compared}");
    assert!(
        hits > 300,
        "the sweep must genuinely strike geometry, got {hits} hits",
    );
}

#[test]
fn bvh_any_hit_matches_a_brute_force_scan_over_a_mixed_scene() {
    let occluders: Vec<Occluder> = mixed_scene(96, 0x1234_5678);
    let scene: RayTraceScene = RayTraceScene::new(occluders.clone());
    let mut state: u64 = 0x0fed_cba9;
    let mut occluded_count: u32 = 0;
    let mut compared: u32 = 0;
    for _ in 0..3000 {
        let raw: Vector3D = Vector3D::new(lcg(&mut state), lcg(&mut state), lcg(&mut state));
        let magnitude: f64 = raw.magnitude();
        if !magnitude.is_finite() || magnitude <= 1e-6 {
            continue;
        }
        let origin: Vector3D = Vector3D::new(
            lcg(&mut state) * 5.0,
            lcg(&mut state) * 3.0 + 1.0,
            lcg(&mut state) * 5.0,
        );
        let ray: Ray = Ray::new(origin, raw.scaled(1.0 / magnitude));
        let expected: bool = brute_occluded(&ray, &occluders);
        let actual: bool = scene.occluded(&ray);
        compared += 1;
        if expected {
            occluded_count += 1;
        }
        assert_eq!(
            actual,
            expected,
            "any-hit disagreement for a ray from ({}, {}, {}) along ({}, {}, {})",
            origin.get_x(),
            origin.get_y(),
            origin.get_z(),
            ray.get_direction().get_x(),
            ray.get_direction().get_y(),
            ray.get_direction().get_z(),
        );
    }
    assert!(compared >= 2900, "too few rays took part, got {compared}");
    assert!(
        occluded_count > 300,
        "the sweep must genuinely hit geometry, got {occluded_count} blocked rays",
    );
}

#[test]
fn any_hit_agrees_with_closest_hit_for_every_sampled_ray() {
    let occluders: Vec<Occluder> = mixed_scene(64, 0xfeed_1111);
    let scene: RayTraceScene = RayTraceScene::new(occluders);
    let mut state: u64 = 0x2222_3333;
    for _ in 0..1500 {
        let raw: Vector3D = Vector3D::new(lcg(&mut state), lcg(&mut state), lcg(&mut state));
        let magnitude: f64 = raw.magnitude();
        if !magnitude.is_finite() || magnitude <= 1e-6 {
            continue;
        }
        let ray: Ray = Ray::new(Vector3D::new(0.0, 1.5, 0.0), raw.scaled(1.0 / magnitude));
        assert_eq!(
            scene.occluded(&ray),
            scene.closest_hit(&ray).is_some(),
            "the two query kinds must agree on whether the range contains any hit",
        );
    }
}

#[test]
fn closest_hit_tie_breaks_on_the_lowest_original_primitive_index() {
    let ray: Ray = Ray::new(Vector3D::new(0.0, 0.0, 0.0), Vector3D::new(0.0, 0.0, 1.0));
    let mut ordered: Vec<Occluder> = Vec::new();
    let mut flipped: Vec<Occluder> = Vec::new();
    for shade in [0.1f64, 0.2, 0.3, 0.4, 0.5] {
        let material: Material = Material::emissive(Vector3D::new(shade, shade, shade));
        ordered.push(Occluder::sphere(
            Vector3D::new(0.0, 0.0, 2.0),
            1.0,
            material.clone(),
        ));
        flipped.push(Occluder::sphere(
            Vector3D::new(0.0, 0.0, 2.0),
            1.0,
            material,
        ));
    }
    flipped.reverse();
    let forward: RayTraceScene = RayTraceScene::new(ordered.clone());
    let backward: RayTraceScene = RayTraceScene::new(flipped.clone());
    let forward_hit: Hit = forward.closest_hit(&ray).expect("all five coincide");
    let backward_hit: Hit = backward.closest_hit(&ray).expect("all five coincide");
    assert!(
        (forward_hit.get_t() - 1.0).abs() < EPSILON,
        "five coincident spheres are all entered at t = 1.0, got {}",
        forward_hit.get_t(),
    );
    assert!(
        (backward_hit.get_t() - 1.0).abs() < EPSILON,
        "reversing the input order must not move the reported distance, got {}",
        backward_hit.get_t(),
    );
    let reference: Option<(usize, f64)> = brute_hit(&ray, &ordered);
    let (winner_index, _winner_t): (usize, f64) = reference.expect("the brute force agrees");
    assert_eq!(
        winner_index, 0,
        "the brute-force tie rule also picks the lowest original index",
    );
    let mirrored_reference: Option<(usize, f64)> = brute_hit(&ray, &flipped);
    assert_eq!(
        mirrored_reference.map(|(_index, _t): (usize, f64)| ()),
        reference.map(|(_index, _t): (usize, f64)| ()),
        "both orders resolve a hit, so the tie never erases geometry",
    );
    assert!(
        forward.occluded(&ray) && backward.occluded(&ray),
        "the any-hit query must find the coincident surface under either order",
    );
}

#[test]
fn an_inside_box_reports_its_exit_rather_than_a_negative_entry() {
    let box_occluder: Occluder = Occluder::aabb(
        Vector3D::new(-1.0, -1.0, -1.0),
        Vector3D::new(1.0, 1.0, 1.0),
        Material::lambert(Vector3D::new(0.5, 0.5, 0.5)),
    );
    let scene: RayTraceScene = RayTraceScene::new(vec![box_occluder]);
    let inside: Ray = Ray::new(Vector3D::new(0.0, 0.0, 0.0), Vector3D::new(1.0, 0.0, 0.0));
    let hit: Hit = scene.closest_hit(&inside).expect("the origin is inside");
    assert!(
        (hit.get_t() - 1.0).abs() < EPSILON,
        "a ray starting inside must leave through the +x face at t = 1.0, got {}",
        hit.get_t(),
    );
    assert!(
        (hit.get_normal().get_x() - 1.0).abs() < EPSILON,
        "leaving through the +x face reports that face's outward normal (1, 0, 0), got ({}, {}, {})",
        hit.get_normal().get_x(),
        hit.get_normal().get_y(),
        hit.get_normal().get_z(),
    );
    let outside: Ray = Ray::new(Vector3D::new(-5.0, 0.0, 0.0), Vector3D::new(1.0, 0.0, 0.0));
    let entry: Hit = scene.closest_hit(&outside).expect("the ray enters the box");
    assert!(
        (entry.get_t() - 4.0).abs() < EPSILON,
        "entering from x = -5 must report t = 4.0, got {}",
        entry.get_t(),
    );
    assert!(
        (entry.get_normal().get_x() + 1.0).abs() < EPSILON,
        "entering through the -x face reports that face's outward normal (-1, 0, 0), got ({}, {}, {})",
        entry.get_normal().get_x(),
        entry.get_normal().get_y(),
        entry.get_normal().get_z(),
    );
}

#[test]
fn an_inside_sphere_reports_its_exit() {
    let material: Material = Material::lambert(Vector3D::new(0.5, 0.5, 0.5));
    let sphere: Occluder = Occluder::sphere(Vector3D::new(0.0, 0.0, 0.0), 2.0, material);
    let scene: RayTraceScene = RayTraceScene::new(vec![sphere]);
    let inside: Ray = Ray::new(Vector3D::new(0.0, 0.0, 0.0), Vector3D::new(0.0, 1.0, 0.0));
    let hit: Hit = scene.closest_hit(&inside).expect("the origin is inside");
    assert!(
        (hit.get_t() - 2.0).abs() < EPSILON,
        "a ray starting inside must exit at t = 2.0, got {}",
        hit.get_t(),
    );
    assert!(
        (hit.get_normal().get_y() - 1.0).abs() < EPSILON,
        "the exit point is the top of the sphere, so the outward normal is (0, 1, 0), got ({}, {}, {})",
        hit.get_normal().get_x(),
        hit.get_normal().get_y(),
        hit.get_normal().get_z(),
    );
}

#[test]
fn a_non_unit_direction_reports_a_true_distance() {
    let material: Material = Material::lambert(Vector3D::new(0.5, 0.5, 0.5));
    let sphere: Occluder = Occluder::sphere(Vector3D::new(0.0, 0.0, 2.0), 1.0, material);
    let scene: RayTraceScene = RayTraceScene::new(vec![sphere]);
    let scaled: Ray = Ray::new(Vector3D::new(0.0, 0.0, 0.0), Vector3D::new(0.0, 0.0, 4.0));
    let hit: Hit = scene
        .closest_hit(&scaled)
        .expect("a four-times-long direction");
    assert!(
        (hit.get_t() - 0.25).abs() < EPSILON,
        "the direction has length 4 and the surface is one unit away, so the ray parameter is 0.25, got {}",
        hit.get_t(),
    );
    assert!(
        (hit.get_position().get_z() - 1.0).abs() < EPSILON,
        "Ray::at(0.25) along (0, 0, 4) is the world point (0, 0, 1), got ({}, {}, {})",
        hit.get_position().get_x(),
        hit.get_position().get_y(),
        hit.get_position().get_z(),
    );
}

#[test]
fn a_zero_direction_never_invents_a_hit() {
    let material: Material = Material::lambert(Vector3D::new(0.5, 0.5, 0.5));
    let sphere: Occluder = Occluder::sphere(Vector3D::new(0.0, 0.0, 0.0), 2.0, material.clone());
    let box_occluder: Occluder = Occluder::aabb(
        Vector3D::new(-1.0, -1.0, -1.0),
        Vector3D::new(1.0, 1.0, 1.0),
        material.clone(),
    );
    let triangle: Occluder = Occluder::triangle(
        Vector3D::new(-5.0, 0.0, -5.0),
        Vector3D::new(5.0, 0.0, -5.0),
        Vector3D::new(0.0, 0.0, 5.0),
        material,
    );
    let scene: RayTraceScene = RayTraceScene::new(vec![sphere, box_occluder, triangle]);
    let degenerate: Ray = Ray::new(Vector3D::new(0.0, 0.0, 0.0), Vector3D::zero());
    assert!(
        scene.closest_hit(&degenerate).is_none(),
        "a ray with no direction cannot reach any surface, even from inside one",
    );
    assert!(
        !scene.occluded(&degenerate),
        "the same ray must not be reported as blocked",
    );
}

#[test]
fn an_axis_aligned_ray_resolves_a_parallel_slab_explicitly() {
    let box_occluder: Occluder = Occluder::aabb(
        Vector3D::new(-1.0, -1.0, -1.0),
        Vector3D::new(1.0, 1.0, 1.0),
        Material::lambert(Vector3D::new(0.5, 0.5, 0.5)),
    );
    let scene: RayTraceScene = RayTraceScene::new(vec![box_occluder]);
    let along_face: Ray = Ray::new(Vector3D::new(-3.0, 1.0, 0.0), Vector3D::new(1.0, 0.0, 0.0));
    let hit: Hit = scene
        .closest_hit(&along_face)
        .expect("the ray skims the top face");
    assert!(
        (hit.get_t() - 2.0).abs() < EPSILON,
        "entering through x = -1 from x = -3 must report t = 2.0, got {}",
        hit.get_t(),
    );
    let above: Ray = Ray::new(Vector3D::new(-3.0, 2.0, 0.0), Vector3D::new(1.0, 0.0, 0.0));
    assert!(
        scene.closest_hit(&above).is_none(),
        "the same direction above the box is parallel to every slab it fails, so it must miss",
    );
    assert!(
        !scene.occluded(&above),
        "and the any-hit query must agree that nothing blocks it",
    );
}

#[test]
fn a_with_range_ray_respects_its_clipped_bounds() {
    let material: Material = Material::lambert(Vector3D::new(0.5, 0.5, 0.5));
    let sphere: Occluder = Occluder::sphere(Vector3D::new(0.0, 0.0, 5.0), 1.0, material);
    let scene: RayTraceScene = RayTraceScene::new(vec![sphere]);
    let base: Ray = Ray::new(Vector3D::new(0.0, 0.0, 0.0), Vector3D::new(0.0, 0.0, 1.0));
    assert!(
        scene.closest_hit(&base).is_some(),
        "the default range reaches the sphere at t = 4",
    );
    let clipped: Ray = base.with_range(0.001, 2.0);
    assert!(
        scene.closest_hit(&clipped).is_none(),
        "clipping t_max below the hit distance must remove it",
    );
    assert!(
        !scene.occluded(&clipped),
        "the any-hit query honours the same clipped range",
    );
    let wide: Ray = base.with_range(0.001, 100.0);
    let hit: Hit = scene
        .closest_hit(&wide)
        .expect("the widened range still reaches");
    assert!(
        (hit.get_t() - 4.0).abs() < EPSILON,
        "widening the range must not move the hit, got {}",
        hit.get_t(),
    );
    assert_eq!(
        wide.get_depth(),
        base.get_depth(),
        "with_range must preserve the bounce budget",
    );
}

#[test]
fn an_inverted_or_non_finite_range_is_untraceable() {
    let material: Material = Material::lambert(Vector3D::new(0.5, 0.5, 0.5));
    let sphere: Occluder = Occluder::sphere(Vector3D::new(0.0, 0.0, 2.0), 1.0, material);
    let scene: RayTraceScene = RayTraceScene::new(vec![sphere]);
    let base: Ray = Ray::new(Vector3D::new(0.0, 0.0, 0.0), Vector3D::new(0.0, 0.0, 1.0));
    let inverted: Ray = base.with_range(10.0, 1.0);
    assert!(
        scene.closest_hit(&inverted).is_none(),
        "an inverted range describes no points on the ray, so it must miss",
    );
    assert!(
        !scene.occluded(&inverted),
        "and the any-hit query must not invent a blocker",
    );
    let not_a_number: Ray = base.with_range(f64::NAN, 10.0);
    assert!(
        scene.closest_hit(&not_a_number).is_none(),
        "a NaN bound cannot compare, so it must be rejected rather than pass",
    );
    let infinite: Ray = base.with_range(0.001, f64::INFINITY);
    assert!(
        scene.closest_hit(&infinite).is_none(),
        "an infinite bound is rejected for the same reason",
    );
    let normal: Hit = scene
        .closest_hit(&base)
        .expect("a finite range is unaffected by those rejections");
    assert!(
        (normal.get_t() - 1.0).abs() < EPSILON,
        "the untouched ray still hits at t = 1.0, got {}",
        normal.get_t(),
    );
}

#[test]
fn degenerate_geometry_never_registers_a_hit() {
    let material: Material = Material::lambert(Vector3D::new(0.5, 0.5, 0.5));
    let zero_radius: Occluder = Occluder::sphere(Vector3D::zero(), 0.0, material.clone());
    let negative_radius: Occluder = Occluder::sphere(Vector3D::zero(), -1.0, material.clone());
    let flat_box: Occluder = Occluder::aabb(
        Vector3D::new(1.0, 1.0, 1.0),
        Vector3D::new(1.0, 1.0, 1.0),
        material.clone(),
    );
    let collapsed_triangle: Occluder = Occluder::triangle(
        Vector3D::new(2.0, 0.0, 0.0),
        Vector3D::new(2.0, 0.0, 0.0),
        Vector3D::new(2.0, 0.0, 0.0),
        material,
    );
    let scene: RayTraceScene = RayTraceScene::new(vec![
        zero_radius,
        negative_radius,
        flat_box,
        collapsed_triangle,
    ]);
    let along_x: Ray = Ray::new(Vector3D::new(-10.0, 0.0, 0.0), Vector3D::new(1.0, 0.0, 0.0));
    assert!(
        scene.closest_hit(&along_x).is_none(),
        "a zero or negative radius, a flat box, and a collapsed triangle all have no surface a ray can strike",
    );
    assert!(
        !scene.occluded(&along_x),
        "the any-hit query must agree that no degenerate surface blocks anything",
    );
}

#[test]
fn a_non_finite_occluder_does_not_poison_its_neighbours() {
    let good: Occluder = Occluder::sphere(
        Vector3D::new(0.0, 0.0, 2.0),
        1.0,
        Material::emissive(Vector3D::new(0.0, 1.0, 0.0)),
    );
    let broken: Occluder = Occluder::sphere(
        Vector3D::new(f64::NAN, 0.0, 2.0),
        1.0,
        Material::emissive(Vector3D::new(1.0, 0.0, 0.0)),
    );
    let scene: RayTraceScene = RayTraceScene::new(vec![good, broken]);
    let ray: Ray = Ray::new(Vector3D::new(0.0, 0.0, 0.0), Vector3D::new(0.0, 0.0, 1.0));
    let hit: Hit = scene
        .closest_hit(&ray)
        .expect("the finite sphere is still reachable");
    assert!(
        (hit.get_t() - 1.0).abs() < EPSILON,
        "the finite sphere is entered at t = 1.0, got {}",
        hit.get_t(),
    );
    assert!(
        (hit.get_normal().get_z() + 1.0).abs() < EPSILON,
        "the nearest pole is the sphere's -z side, whose outward normal is (0, 0, -1), got ({}, {}, {})",
        hit.get_normal().get_x(),
        hit.get_normal().get_y(),
        hit.get_normal().get_z(),
    );
    let away: Ray = Ray::new(Vector3D::new(0.0, 0.0, 0.0), Vector3D::new(0.0, 1.0, 0.0));
    assert!(
        scene.closest_hit(&away).is_none(),
        "a ray off to the side must still miss both spheres",
    );
    assert!(!scene.occluded(&away), "and the any-hit query must agree",);
}

#[test]
fn a_large_scene_still_resolves_every_hit() {
    let occluders: Vec<Occluder> = mixed_scene(600, 0x0bad_c0de);
    let scene: RayTraceScene = RayTraceScene::new(occluders.clone());
    let mut state: u64 = 0x9999_8888;
    let mut checked: u32 = 0;
    for _ in 0..400 {
        let raw: Vector3D = Vector3D::new(lcg(&mut state), lcg(&mut state), lcg(&mut state));
        let magnitude: f64 = raw.magnitude();
        if !magnitude.is_finite() || magnitude <= 1e-6 {
            continue;
        }
        let origin: Vector3D = Vector3D::new(
            lcg(&mut state) * 5.0,
            lcg(&mut state) * 3.0 + 1.0,
            lcg(&mut state) * 5.0,
        );
        let ray: Ray = Ray::new(origin, raw.scaled(1.0 / magnitude));
        let expected: Option<(usize, f64)> = brute_hit(&ray, &occluders);
        let actual: Option<Hit> = scene.closest_hit(&ray);
        checked += 1;
        match (expected, actual) {
            (None, None) => {}
            (Some((_, expected_t)), Some(hit)) => assert!(
                (hit.get_t() - expected_t).abs() < 1e-9,
                "a 600-primitive hierarchy reported {} instead of {expected_t}",
                hit.get_t(),
            ),
            (expected, actual) => {
                panic!("a 600-primitive hierarchy returned {actual:?} instead of {expected:?}")
            }
        }
    }
    assert!(checked >= 380, "too few rays took part, got {checked}");
}

#[test]
fn a_single_occluder_scene_still_builds_a_usable_hierarchy() {
    let sphere: Occluder = Occluder::sphere(
        Vector3D::new(0.0, 0.0, 3.0),
        1.0,
        Material::lambert(Vector3D::new(0.5, 0.5, 0.5)),
    );
    let scene: RayTraceScene = RayTraceScene::new(vec![sphere]);
    let ray: Ray = Ray::new(Vector3D::new(0.0, 0.0, 0.0), Vector3D::new(0.0, 0.0, 1.0));
    let hit: Hit = scene.closest_hit(&ray).expect("a lone sphere is reachable");
    assert!(
        (hit.get_t() - 2.0).abs() < EPSILON,
        "a lone sphere at z = 3 with radius 1 is entered at t = 2.0, got {}",
        hit.get_t(),
    );
    assert!(
        scene.occluded(&ray),
        "the same ray is blocked by the same sphere"
    );
}

#[test]
fn an_empty_scene_reports_nothing_and_blocks_nothing() {
    let scene: RayTraceScene = RayTraceScene::new(Vec::new());
    let ray: Ray = Ray::new(Vector3D::new(0.0, 0.0, 0.0), Vector3D::new(0.0, 0.0, 1.0));
    assert!(
        scene.closest_hit(&ray).is_none(),
        "an empty hierarchy has no node to visit, so there is nothing to hit",
    );
    assert!(
        !scene.occluded(&ray),
        "and an empty hierarchy blocks nothing either",
    );
}

#[test]
fn coincident_primitives_build_a_hierarchy_that_terminates() {
    let material: Material = Material::lambert(Vector3D::new(0.5, 0.5, 0.5));
    let mut occluders: Vec<Occluder> = Vec::new();
    for _ in 0..24 {
        occluders.push(Occluder::sphere(Vector3D::zero(), 1.0, material.clone()));
        occluders.push(Occluder::aabb(
            Vector3D::new(-1.0, -1.0, -1.0),
            Vector3D::new(1.0, 1.0, 1.0),
            material.clone(),
        ));
    }
    let scene: RayTraceScene = RayTraceScene::new(occluders.clone());
    let ray: Ray = Ray::new(Vector3D::new(0.0, 0.0, 5.0), Vector3D::new(0.0, 0.0, -1.0));
    let expected: Option<(usize, f64)> = brute_hit(&ray, &occluders);
    let hit: Hit = scene
        .closest_hit(&ray)
        .expect("coincident geometry is still hittable");
    assert!(
        (hit.get_t() - 4.0).abs() < EPSILON,
        "every primitive shares the same surface at t = 4.0, got {}",
        hit.get_t(),
    );
    assert!(
        expected.is_some(),
        "the brute-force reference agrees that the ray hits",
    );
}

#[test]
fn a_geometric_spread_of_spheres_never_loses_the_nearest_hit() {
    let mut occluders: Vec<Occluder> = Vec::new();
    let mut centers: Vec<f64> = Vec::new();
    for index in 0..80usize {
        let center: f64 = 10.0f64.powf(2.0 * index as f64);
        centers.push(center);
        occluders.push(Occluder::sphere(
            Vector3D::new(center, 0.0, 0.0),
            center * 0.1,
            Material::lambert(Vector3D::new(0.5, 0.5, 0.5)),
        ));
    }
    let scene: RayTraceScene = RayTraceScene::new(occluders.clone());
    let ray: Ray = Ray::new(Vector3D::new(0.0, 0.0, 0.0), Vector3D::new(1.0, 0.0, 0.0))
        .with_range(0.001, 1e160);
    let hit: Hit = scene
        .closest_hit(&ray)
        .expect("the first sphere is in range");
    let expected: f64 = centers[0] - centers[0] * 0.1;
    assert!(
        (hit.get_t() - expected).abs() < 1e-6 * expected,
        "a geometric spread makes the heuristic chain deeply, and the nearest sphere at x = {} must still win; expected t = {} but got {}",
        centers[0],
        expected,
        hit.get_t(),
    );
    let expected_hit: Option<(usize, f64)> = brute_hit(&ray, &occluders);
    assert_eq!(
        expected_hit.map(|(_, t): (usize, f64)| t),
        Some(hit.get_t()),
        "the brute-force reference over all 80 spheres must agree",
    );
}

#[test]
fn a_geometric_spread_of_spheres_never_loses_a_blocker() {
    let mut occluders: Vec<Occluder> = Vec::new();
    let mut centers: Vec<f64> = Vec::new();
    for index in 0..80usize {
        let center: f64 = 10.0f64.powf(2.0 * index as f64);
        centers.push(center);
        occluders.push(Occluder::sphere(
            Vector3D::new(center, 0.0, 0.0),
            center * 0.1,
            Material::lambert(Vector3D::new(0.5, 0.5, 0.5)),
        ));
    }
    let scene: RayTraceScene = RayTraceScene::new(occluders);
    let long_ray: Ray = Ray::new(Vector3D::new(0.0, 0.0, 0.0), Vector3D::new(1.0, 0.0, 0.0))
        .with_range(0.001, 1e160);
    assert!(
        scene.occluded(&long_ray),
        "the any-hit query must also survive the deep chain and find the nearest sphere",
    );
    let short_ray: Ray = Ray::new(Vector3D::new(0.0, 0.0, 0.0), Vector3D::new(1.0, 0.0, 0.0))
        .with_range(0.001, centers[0] * 0.5);
    assert!(
        !scene.occluded(&short_ray),
        "a segment stopping before the first sphere must stay clear even with 80 spheres behind it",
    );
}

#[test]
fn a_blocker_beyond_the_light_cannot_shadow_the_light() {
    let light_side: Occluder = Occluder::sphere(
        Vector3D::new(0.0, 0.0, 3.0),
        0.5,
        Material::lambert(Vector3D::new(0.5, 0.5, 0.5)),
    );
    let beyond: Occluder = Occluder::sphere(
        Vector3D::new(0.0, 0.0, 7.0),
        0.5,
        Material::lambert(Vector3D::new(0.5, 0.5, 0.5)),
    );
    let scene: RayTraceScene = RayTraceScene::new(vec![light_side, beyond]);
    let to_light: Ray =
        Ray::new(Vector3D::zero(), Vector3D::new(0.0, 0.0, 1.0)).with_range(0.001, 2.5);
    assert!(
        scene.occluded(&to_light),
        "the sphere in front of the light blocks the segment that reaches it",
    );
    let past_light: Ray =
        Ray::new(Vector3D::zero(), Vector3D::new(0.0, 0.0, 1.0)).with_range(0.001, 2.4);
    assert!(
        !scene.occluded(&past_light),
        "stopping short of the first sphere leaves nothing to block, even with one further out",
    );
    let all_the_way: Ray =
        Ray::new(Vector3D::zero(), Vector3D::new(0.0, 0.0, 1.0)).with_range(0.001, 8.0);
    assert!(
        scene.occluded(&all_the_way),
        "an unbounded segment still hits both spheres",
    );
}

#[test]
fn a_shadows_its_own_surface_only_through_the_offset() {
    let material: Material = Material::lambert(Vector3D::new(0.5, 0.5, 0.5));
    let floor: Occluder = Occluder::aabb(
        Vector3D::new(-10.0, -10.0, -0.5),
        Vector3D::new(10.0, 10.0, 0.0),
        material,
    );
    let scene: RayTraceScene = RayTraceScene::new(vec![floor]);
    let surface_point: Vector3D = Vector3D::new(1.0, 2.0, 0.0);
    let offset: Ray = Ray::new(surface_point, Vector3D::new(0.0, 0.0, 1.0))
        .with_range(SHADOW_SELF_OFFSET, 1000.0);
    assert!(
        !scene.occluded(&offset),
        "a ray leaving the surface along its normal must not hit the surface it started on",
    );
    let back_into: Ray = Ray::new(surface_point, Vector3D::new(0.0, 0.0, -1.0))
        .with_range(SHADOW_SELF_OFFSET, 1000.0);
    assert!(
        scene.occluded(&back_into),
        "the same surface does block a ray aimed back into it",
    );
}

#[test]
fn an_occluded_segment_stops_shadow_rays_before_the_light() {
    let blocker: Occluder = Occluder::sphere(
        Vector3D::new(0.0, 0.0, 5.0),
        1.0,
        Material::lambert(Vector3D::new(0.5, 0.5, 0.5)),
    );
    let scene: RayTraceScene = RayTraceScene::new(vec![blocker]);
    let clear: Ray =
        Ray::new(Vector3D::new(0.0, 0.0, 0.0), Vector3D::new(0.0, 0.0, 1.0)).with_range(0.001, 3.0);
    assert!(
        !scene.occluded(&clear),
        "a segment stopping before the sphere at t = 4.0 is clear",
    );
    let blocked: Ray = Ray::new(Vector3D::new(0.0, 0.0, 0.0), Vector3D::new(0.0, 0.0, 1.0))
        .with_range(0.001, 10.0);
    assert!(
        scene.occluded(&blocked),
        "the same segment reaching the sphere is blocked",
    );
}
