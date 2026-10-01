use euv_engine::*;

#[test]
fn trace_miss_returns_ambient() {
    let eye: Vector3D = Vector3D::new(0.0, 0.0, 0.0);
    let mut lights: LightingUniforms = LightingUniforms::with_eye(eye);
    lights.set_ambient(Vector3D::new(0.2, 0.4, 0.6));
    let ray: Ray = Ray::new(Vector3D::new(0.0, 0.0, 0.0), Vector3D::new(1.0, 0.0, 0.0));
    let occluders: Vec<Occluder> = Vec::new();
    let scene: RayTraceScene = RayTraceScene::new(occluders);
    let color: Vector3D = scene.trace(ray, &lights);
    assert!(
        (color.get_x() - 0.2).abs() < EPSILON,
        "expected ambient red 0.2, got {}",
        color.get_x(),
    );
    assert!(
        (color.get_y() - 0.4).abs() < EPSILON,
        "expected ambient green 0.4, got {}",
        color.get_y(),
    );
    assert!(
        (color.get_z() - 0.6).abs() < EPSILON,
        "expected ambient blue 0.6, got {}",
        color.get_z(),
    );
}

#[test]
fn trace_emissive_sphere() {
    let eye: Vector3D = Vector3D::new(0.0, 0.0, 5.0);
    let mut lights: LightingUniforms = LightingUniforms::with_eye(eye);
    lights.set_ambient(Vector3D::zero());
    let sphere_material: Material = Material::emissive(Vector3D::new(1.0, 0.0, 0.0));
    let sphere: Occluder = Occluder::sphere(Vector3D::zero(), 1.0, sphere_material);
    let occluders: Vec<Occluder> = vec![sphere];
    let scene: RayTraceScene = RayTraceScene::new(occluders);
    let ray: Ray = Ray::new(Vector3D::new(0.0, 0.0, 5.0), Vector3D::new(0.0, 0.0, -1.0));
    let color: Vector3D = scene.trace(ray, &lights);
    assert!(
        (color.get_x() - 1.0).abs() < EPSILON,
        "expected emissive red 1.0, got {}",
        color.get_x(),
    );
    assert!(
        color.get_y().abs() < EPSILON,
        "expected emissive green 0.0, got {}",
        color.get_y(),
    );
    assert!(
        color.get_z().abs() < EPSILON,
        "expected emissive blue 0.0, got {}",
        color.get_z(),
    );
}

#[test]
fn trace_reflection_single_bounce() {
    let eye: Vector3D = Vector3D::new(0.0, 0.0, 10.0);
    let mut lights: LightingUniforms = LightingUniforms::with_eye(eye);
    lights.set_ambient(Vector3D::zero());
    let mirror_material: Material = Material::phong(Vector3D::zero(), 1.0, 32.0);
    let mirror: Occluder = Occluder::sphere(Vector3D::zero(), 1.0, mirror_material);
    let emissive_material: Material = Material::emissive(Vector3D::new(0.0, 1.0, 0.0));
    let emissive: Occluder =
        Occluder::sphere(Vector3D::new(0.0, 0.0, 15.0), 1.0, emissive_material);
    let occluders: Vec<Occluder> = vec![mirror, emissive];
    let scene: RayTraceScene = RayTraceScene::new(occluders);
    let ray: Ray = Ray::new(Vector3D::new(0.0, 0.0, 10.0), Vector3D::new(0.0, 0.0, -1.0));
    let color: Vector3D = scene.trace(ray, &lights);
    assert!(
        color.get_y() > 0.0,
        "expected bounce to bring back some green, got {}",
        color.get_y(),
    );
    assert!(
        color.get_x().abs() < EPSILON,
        "expected red ~0 (no red light), got {}",
        color.get_x(),
    );
    assert!(
        color.get_z().abs() < EPSILON,
        "expected blue ~0 (no blue light), got {}",
        color.get_z(),
    );
}

fn demo_scene() -> (Vec<Occluder>, LightingUniforms) {
    let ground: Occluder = Occluder::aabb(
        Vector3D::new(-5.0, -0.6, -5.0),
        Vector3D::new(5.0, -0.5, 5.0),
        Material::phong(Vector3D::new(0.30, 0.32, 0.36), 0.30, 24.0),
    );
    let mirror: Occluder = Occluder::sphere(
        Vector3D::new(0.0, 0.4, 0.0),
        0.9,
        Material::phong(Vector3D::new(0.05, 0.05, 0.06), 1.0, 64.0),
    );
    let emissive: Occluder = Occluder::sphere(
        Vector3D::new(1.6, 0.6, -1.4),
        0.45,
        Material::emissive(Vector3D::new(1.0, 0.45, 0.10)),
    );
    let occluders: Vec<Occluder> = vec![ground, mirror, emissive];
    let eye: Vector3D = Vector3D::new(0.0, 0.8, 3.5);
    let yaw: f64 = 0.7;
    let light_dir: Vector3D = Vector3D::new(-yaw.cos(), -0.5, -yaw.sin()).normalized();
    let sun: Light = Light::new_directional(light_dir, Vector3D::new(1.0, 0.95, 0.85));
    let mut lights: LightingUniforms = LightingUniforms::with_eye(eye);
    lights.set_ambient(Vector3D::new(0.10, 0.10, 0.14));
    lights.add_light(sun);
    (occluders, lights)
}

#[test]
fn closest_hit_returns_analytic_t() {
    let (occluders, _lights): (Vec<Occluder>, LightingUniforms) = demo_scene();
    let scene: RayTraceScene = RayTraceScene::new(occluders);
    let dead_center: Ray = Ray::new(Vector3D::new(0.0, 0.4, 5.0), Vector3D::new(0.0, 0.0, -1.0));
    let expected_t: f64 = 5.0 - 0.9;
    let hit: Option<Hit> = scene.closest_hit(&dead_center);
    assert!(hit.is_some(), "expected dead-center ray to hit the mirror");
    assert!(
        (hit.expect("checked above").get_t() - expected_t).abs() < 1e-9,
        "expected analytic t {expected_t}",
    );
    let away: Ray = Ray::new(Vector3D::new(0.0, 0.4, 5.0), Vector3D::new(0.0, 0.0, 1.0));
    assert!(
        scene.closest_hit(&away).is_none(),
        "expected ray pointing away from the scene to miss",
    );
}

fn ground_triangle() -> Occluder {
    Occluder::triangle(
        Vector3D::new(0.0, 0.0, 0.0),
        Vector3D::new(1.0, 0.0, 0.0),
        Vector3D::new(0.0, 0.0, 1.0),
        Material::lambert(Vector3D::new(0.5, 0.5, 0.5)),
    )
}

fn triangle_at_y(plane_y: f64) -> Occluder {
    Occluder::triangle(
        Vector3D::new(0.0, plane_y, 0.0),
        Vector3D::new(1.0, plane_y, 0.0),
        Vector3D::new(0.0, plane_y, 1.0),
        Material::lambert(Vector3D::new(0.5, 0.5, 0.5)),
    )
}

fn close_to(expected: f64, actual: f64) -> bool {
    (expected - actual).abs() < 1e-9
}

fn vector_is(expected: Vector3D, actual: Vector3D) -> bool {
    close_to(expected.get_x(), actual.get_x())
        && close_to(expected.get_y(), actual.get_y())
        && close_to(expected.get_z(), actual.get_z())
}

#[test]
fn intersect_triangle_hits_from_above_at_unit_distance_with_upward_normal() {
    let v0: Vector3D = Vector3D::new(0.0, 0.0, 0.0);
    let v1: Vector3D = Vector3D::new(1.0, 0.0, 0.0);
    let v2: Vector3D = Vector3D::new(0.0, 0.0, 1.0);
    let origin: Vector3D = Vector3D::new(0.25, 1.0, 0.25);
    let dir: Vector3D = Vector3D::new(0.0, -1.0, 0.0);
    let hit: Option<(f64, Vector3D)> = intersect_triangle(origin, dir, v0, v1, v2);
    assert!(
        hit.is_some(),
        "ray (0.25,1,0.25) -> (0,-1,0) passes through the interior of the y=0 triangle",
    );
    let (t, normal): (f64, Vector3D) = hit.expect("checked above");
    assert!(
        close_to(1.0, t),
        "origin sits exactly 1.0 above the y=0 plane and the direction is unit length, so t must be 1.0, got {t}",
    );
    assert!(
        vector_is(Vector3D::new(0.0, 1.0, 0.0), normal),
        "edge1=(1,0,0) cross edge2=(0,0,1) is the raw normal (0,-1,0); a downward ray needs the flipped (0,1,0), got ({}, {}, {})",
        normal.get_x(),
        normal.get_y(),
        normal.get_z(),
    );
    let position: Vector3D = origin + dir.scaled(t);
    assert!(
        vector_is(Vector3D::new(0.25, 0.0, 0.25), position),
        "expected hit point (0.25, 0, 0.25), got ({}, {}, {})",
        position.get_x(),
        position.get_y(),
        position.get_z(),
    );
}

#[test]
fn intersect_triangle_hits_from_below_at_unit_distance_with_downward_normal() {
    let v0: Vector3D = Vector3D::new(0.0, 0.0, 0.0);
    let v1: Vector3D = Vector3D::new(1.0, 0.0, 0.0);
    let v2: Vector3D = Vector3D::new(0.0, 0.0, 1.0);
    let origin: Vector3D = Vector3D::new(0.25, -1.0, 0.25);
    let dir: Vector3D = Vector3D::new(0.0, 1.0, 0.0);
    let hit: Option<(f64, Vector3D)> = intersect_triangle(origin, dir, v0, v1, v2);
    assert!(
        hit.is_some(),
        "the test is two-sided, so the back face of the y=0 triangle must also register a hit",
    );
    let (t, normal): (f64, Vector3D) = hit.expect("checked above");
    assert!(
        close_to(1.0, t),
        "origin sits exactly 1.0 below the y=0 plane and the direction is unit length, so t must be 1.0, got {t}",
    );
    assert!(
        vector_is(Vector3D::new(0.0, -1.0, 0.0), normal),
        "the raw normal (0,-1,0) already opposes an upward ray, so it is returned unflipped, got ({}, {}, {})",
        normal.get_x(),
        normal.get_y(),
        normal.get_z(),
    );
    assert!(
        normal.dot(dir) < 0.0,
        "the shading normal must always face against the ray, got dot {}",
        normal.dot(dir),
    );
}

#[test]
fn intersect_triangle_misses_beyond_the_hypotenuse() {
    let v0: Vector3D = Vector3D::new(0.0, 0.0, 0.0);
    let v1: Vector3D = Vector3D::new(1.0, 0.0, 0.0);
    let v2: Vector3D = Vector3D::new(0.0, 0.0, 1.0);
    let hit: Option<(f64, Vector3D)> = intersect_triangle(
        Vector3D::new(2.0, 1.0, 2.0),
        Vector3D::new(0.0, -1.0, 0.0),
        v0,
        v1,
        v2,
    );
    assert!(
        hit.is_none(),
        "the landing point (2, 0, 2) has x + z = 4 > 1, which is outside the triangle",
    );
}

#[test]
fn intersect_triangle_returns_none_for_ray_parallel_to_the_plane() {
    let v0: Vector3D = Vector3D::new(0.0, 0.0, 0.0);
    let v1: Vector3D = Vector3D::new(1.0, 0.0, 0.0);
    let v2: Vector3D = Vector3D::new(0.0, 0.0, 1.0);
    let parallel_in_plane: Option<(f64, Vector3D)> = intersect_triangle(
        Vector3D::new(0.25, 1.0, 0.0),
        Vector3D::new(1.0, 0.0, 0.0),
        v0,
        v1,
        v2,
    );
    assert!(
        parallel_in_plane.is_none(),
        "a direction of (1,0,0) is parallel to the y=0 triangle plane, so the determinant is zero and there is no intersection",
    );
    let parallel_above: Option<(f64, Vector3D)> = intersect_triangle(
        Vector3D::new(0.25, 1.0, 0.25),
        Vector3D::new(1.0, 0.0, 0.0),
        v0,
        v1,
        v2,
    );
    assert!(
        parallel_above.is_none(),
        "even a ray skimming above the plane stays parallel, so it must miss too",
    );
}

#[test]
fn ray_intersect_triangle_rejects_hits_beyond_the_ray_range() {
    let v0: Vector3D = Vector3D::new(0.0, 0.0, 0.0);
    let v1: Vector3D = Vector3D::new(1.0, 0.0, 0.0);
    let v2: Vector3D = Vector3D::new(0.0, 0.0, 1.0);
    let far: Ray = Ray::new(
        Vector3D::new(0.25, 2000.0, 0.25),
        Vector3D::new(0.0, -1.0, 0.0),
    );
    assert!(
        far.intersect_triangle(v0, v1, v2).is_none(),
        "t = 2000 exceeds the default t_max of 1000, so the ray method must reject the hit",
    );
    let unbounded: Option<(f64, Vector3D)> = intersect_triangle(
        Vector3D::new(0.25, 2000.0, 0.25),
        Vector3D::new(0.0, -1.0, 0.0),
        v0,
        v1,
        v2,
    );
    assert!(
        close_to(
            2000.0,
            unbounded.expect("the free function does not clamp t").0
        ),
        "the free function must still report the raw distance of 2000",
    );
    let grazing: Ray = Ray::new(
        Vector3D::new(0.25, 0.0, 0.25),
        Vector3D::new(0.0, -1.0, 0.0),
    );
    assert!(
        grazing.intersect_triangle(v0, v1, v2).is_none(),
        "t = 0 is below the default t_min of 0.001, so the ray method must reject the self-hit",
    );
}

#[test]
fn occluder_triangle_points_cover_the_vertex_bounds() {
    let triangle: Occluder = ground_triangle();
    assert!(
        triangle.get_kind() == OccluderKind::Triangle,
        "Occluder::triangle must record the Triangle kind",
    );
    let points: Vec<(Vector3D, f64)> = triangle.occluder_points();
    assert!(
        points.len() == 1,
        "one occluder must produce exactly one shadow bounding sphere, got {}",
        points.len(),
    );
    let (center, radius): (Vector3D, f64) = points[0];
    assert!(
        vector_is(Vector3D::new(0.5, 0.0, 0.5), center),
        "the vertex AABB spans x in [0,1] and z in [0,1], so its centre is (0.5, 0, 0.5), got ({}, {}, {})",
        center.get_x(),
        center.get_y(),
        center.get_z(),
    );
    let expected_radius: f64 = 0.5f64.sqrt();
    assert!(
        close_to(expected_radius, radius),
        "the half extents are (0.5, 0, 0.5), so the bounding radius is sqrt(0.25 + 0.25) = 0.7071..., got {radius}",
    );
}

#[test]
fn closest_hit_resolves_a_lone_triangle_occluder() {
    let scene: RayTraceScene = RayTraceScene::new(vec![ground_triangle()]);
    let ray: Ray = Ray::new(
        Vector3D::new(0.25, 1.0, 0.25),
        Vector3D::new(0.0, -1.0, 0.0),
    );
    let hit: Option<Hit> = scene.closest_hit(&ray);
    assert!(
        hit.is_some(),
        "the lone triangle must be hit by the test ray"
    );
    let resolved: Hit = hit.expect("checked above");
    assert!(
        close_to(1.0, resolved.get_t()),
        "expected t = 1.0 through the public scene path, got {}",
        resolved.get_t(),
    );
    assert!(
        vector_is(Vector3D::new(0.25, 0.0, 0.25), resolved.get_position()),
        "expected position (0.25, 0, 0.25), got ({}, {}, {})",
        resolved.get_position().get_x(),
        resolved.get_position().get_y(),
        resolved.get_position().get_z(),
    );
    assert!(
        vector_is(Vector3D::new(0.0, 1.0, 0.0), resolved.get_normal()),
        "expected the anti-ray normal (0, 1, 0), got ({}, {}, {})",
        resolved.get_normal().get_x(),
        resolved.get_normal().get_y(),
        resolved.get_normal().get_z(),
    );
}

#[test]
fn closest_hit_prefers_the_nearer_sphere_over_a_farther_triangle() {
    let triangle: Occluder = triangle_at_y(-2.0);
    let sphere: Occluder = Occluder::sphere(
        Vector3D::new(0.25, -0.5, 0.25),
        0.25,
        Material::lambert(Vector3D::new(1.0, 1.0, 1.0)),
    );
    let scene: RayTraceScene = RayTraceScene::new(vec![triangle, sphere]);
    let ray: Ray = Ray::new(
        Vector3D::new(0.25, 1.0, 0.25),
        Vector3D::new(0.0, -1.0, 0.0),
    );
    let hit: Option<Hit> = scene.closest_hit(&ray);
    assert!(hit.is_some(), "both occluders are on the ray path");
    let resolved: Hit = hit.expect("checked above");
    assert!(
        close_to(1.25, resolved.get_t()),
        "the sphere at y = -0.5 with radius 0.25 is entered at t = 1 - (-0.25) = 1.25, closer than the triangle at t = 3.0, got {}",
        resolved.get_t(),
    );
    assert!(
        close_to(1.0, resolved.get_normal().get_y()),
        "the winning surface is the sphere, whose normal at the top pole is (0, 1, 0), got ({}, {}, {})",
        resolved.get_normal().get_x(),
        resolved.get_normal().get_y(),
        resolved.get_normal().get_z(),
    );
}

#[test]
fn closest_hit_prefers_the_nearer_triangle_over_a_farther_aabb() {
    let box_occluder: Occluder = Occluder::aabb(
        Vector3D::new(-1.0, -3.0, -1.0),
        Vector3D::new(1.0, -2.0, 1.0),
        Material::lambert(Vector3D::new(1.0, 1.0, 1.0)),
    );
    let scene: RayTraceScene = RayTraceScene::new(vec![box_occluder, ground_triangle()]);
    let ray: Ray = Ray::new(
        Vector3D::new(0.25, 1.0, 0.25),
        Vector3D::new(0.0, -1.0, 0.0),
    );
    let resolved: Hit = scene
        .closest_hit(&ray)
        .expect("both occluders are on the ray path");
    assert!(
        close_to(1.0, resolved.get_t()),
        "the triangle at y = 0 is nearer than the AABB front face at y = -2, so t = 1.0, got {}",
        resolved.get_t(),
    );
    assert!(
        vector_is(Vector3D::new(0.0, 1.0, 0.0), resolved.get_normal()),
        "the winning surface is the triangle, whose anti-ray normal is (0, 1, 0), got ({}, {}, {})",
        resolved.get_normal().get_x(),
        resolved.get_normal().get_y(),
        resolved.get_normal().get_z(),
    );
}

#[test]
fn closest_hit_ignores_a_triangle_beyond_the_ray_range() {
    let scene: RayTraceScene = RayTraceScene::new(vec![triangle_at_y(-2000.0)]);
    let ray: Ray = Ray::new(
        Vector3D::new(0.25, 1.0, 0.25),
        Vector3D::new(0.0, -1.0, 0.0),
    );
    assert!(
        scene.closest_hit(&ray).is_none(),
        "the only triangle is entered at t = 2001, past the default t_max of 1000, so the broad-phase reject and the exact test agree on a miss",
    );
}

#[test]
fn closest_hit_keeps_the_nearer_sphere_when_a_far_triangle_is_pruned() {
    let triangle: Occluder = triangle_at_y(-2000.0);
    let sphere: Occluder = Occluder::sphere(
        Vector3D::new(0.25, -0.5, 0.25),
        0.25,
        Material::lambert(Vector3D::new(1.0, 1.0, 1.0)),
    );
    let scene: RayTraceScene = RayTraceScene::new(vec![triangle, sphere]);
    let ray: Ray = Ray::new(
        Vector3D::new(0.25, 1.0, 0.25),
        Vector3D::new(0.0, -1.0, 0.0),
    );
    let resolved: Hit = scene
        .closest_hit(&ray)
        .expect("the sphere is inside the ray range");
    assert!(
        close_to(1.25, resolved.get_t()),
        "the far triangle must be skipped without displacing the sphere hit at t = 1.25, got {}",
        resolved.get_t(),
    );
}

#[test]
fn closest_hit_rejects_a_ray_that_only_crosses_the_triangle_bounds() {
    let scene: RayTraceScene = RayTraceScene::new(vec![ground_triangle()]);
    let ray: Ray = Ray::new(Vector3D::new(0.6, 1.0, 0.6), Vector3D::new(0.0, -1.0, 0.0));
    assert!(
        scene.closest_hit(&ray).is_none(),
        "the landing point (0.6, 0, 0.6) lies inside the vertex AABB but has x + z = 1.2 > 1, so the exact test must reject what the broad phase admits",
    );
}

#[test]
fn trace_bounces_shades_a_triangle_occluder() {
    let eye: Vector3D = Vector3D::new(0.25, 1.0, 0.25);
    let mut lights: LightingUniforms = LightingUniforms::with_eye(eye);
    lights.set_ambient(Vector3D::zero());
    let emissive_material: Material = Material::emissive(Vector3D::new(0.0, 0.0, 1.0));
    let triangle: Occluder = Occluder::triangle(
        Vector3D::new(0.0, 0.0, 0.0),
        Vector3D::new(1.0, 0.0, 0.0),
        Vector3D::new(0.0, 0.0, 1.0),
        emissive_material,
    );
    let scene: RayTraceScene = RayTraceScene::new(vec![triangle]);
    let ray: Ray = Ray::new(eye, Vector3D::new(0.0, -1.0, 0.0));
    let color: Vector3D = scene.trace(ray, &lights);
    assert!(
        close_to(1.0, color.get_z()),
        "an emissive blue triangle must shade the ray blue with a value of 1.0, got {}",
        color.get_z(),
    );
    assert!(
        color.get_x().abs() < EPSILON,
        "no red contribution is expected, got {}",
        color.get_x(),
    );
}

fn unbounded_closest_hit(ray: &Ray, occluders: &[Occluder]) -> Option<f64> {
    let origin: Vector3D = ray.get_origin();
    let dir: Vector3D = ray.get_direction();
    let t_min: f64 = ray.get_t_min();
    let t_max: f64 = ray.get_t_max();
    let mut best: Option<f64> = None;
    for occ in occluders.iter() {
        let candidate: Option<f64> = match occ.get_kind() {
            OccluderKind::Sphere => {
                let radius: f64 = occ.get_extent().get_x();
                match ray_sphere_intersect(origin, dir, occ.get_center(), radius) {
                    Some((t, _)) if t >= t_min && t <= t_max => Some(t),
                    _ => None,
                }
            }
            OccluderKind::Aabb => {
                match ray_aabb_intersect(origin, dir, occ.get_center(), occ.get_extent()) {
                    Some((t_near, _t_far, _)) if t_near >= t_min && t_near <= t_max => Some(t_near),
                    _ => None,
                }
            }
            OccluderKind::Triangle => {
                let vertices: [Vector3D; 3] = occ.get_vertices();
                match intersect_triangle(origin, dir, vertices[0], vertices[1], vertices[2]) {
                    Some((t, _)) if t >= t_min && t <= t_max => Some(t),
                    _ => None,
                }
            }
        };
        if let Some(t) = candidate {
            let keep_previous: bool = matches!(&best, Some(previous) if *previous <= t);
            if !keep_previous {
                best = Some(t);
            }
        }
    }
    best
}

fn next_random(state: &mut u64) -> f64 {
    *state = state
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    ((*state >> 33) as f64) / 2147483648.0 - 1.0
}

#[test]
fn broad_phase_matches_an_unpruned_scan_over_a_dense_scene() {
    let (base, _lights): (Vec<Occluder>, LightingUniforms) = demo_scene();
    let mut state: u64 = 12345;
    let mut occluders: Vec<Occluder> = base;
    for _ in 0..40 {
        let sphere_center: Vector3D = Vector3D::new(
            next_random(&mut state) * 4.0,
            next_random(&mut state) * 3.0,
            next_random(&mut state) * 4.0,
        );
        let radius: f64 = next_random(&mut state).abs() * 0.5 + 0.05;
        occluders.push(Occluder::sphere(
            sphere_center,
            radius,
            Material::lambert(Vector3D::new(0.4, 0.4, 0.4)),
        ));
        let base_vertex: Vector3D = Vector3D::new(
            next_random(&mut state) * 3.0,
            next_random(&mut state) * 2.0,
            next_random(&mut state) * 3.0,
        );
        occluders.push(Occluder::triangle(
            base_vertex,
            base_vertex + Vector3D::new(0.5, 0.0, 0.2),
            base_vertex + Vector3D::new(0.0, 0.0, 0.6),
            Material::lambert(Vector3D::new(0.4, 0.4, 0.4)),
        ));
    }
    let scene: RayTraceScene = RayTraceScene::new(occluders.clone());
    let mut hits: u32 = 0;
    let mut compared: u32 = 0;
    for _ in 0..2000 {
        let dir: Vector3D = Vector3D::new(
            next_random(&mut state),
            next_random(&mut state),
            next_random(&mut state),
        );
        let magnitude: f64 = dir.magnitude();
        if magnitude < 1e-6 {
            continue;
        }
        let origin: Vector3D = Vector3D::new(
            next_random(&mut state) * 3.0,
            next_random(&mut state) * 3.0 + 2.0,
            next_random(&mut state) * 3.0,
        );
        let ray: Ray = Ray::new(origin, dir.scaled(1.0 / magnitude));
        let expected: Option<f64> = unbounded_closest_hit(&ray, &occluders);
        let actual: Option<f64> = scene.closest_hit(&ray).map(|hit: Hit| hit.get_t());
        compared += 1;
        if expected.is_some() {
            hits += 1;
        }
        match (expected, actual) {
            (None, None) => {}
            (Some(e), Some(a)) => assert!(
                close_to(e, a),
                "the broad phase returned t={a} but an unpruned scan of every occluder returns {e}",
            ),
            (e, a) => panic!("the broad phase returned {a:?} but an unpruned scan returns {e:?}"),
        }
    }
    assert!(
        compared >= 1900,
        "nearly every sampled ray must take part in the comparison, got {compared}",
    );
    assert!(
        hits > 100,
        "the sweep must genuinely strike geometry for the comparison to mean anything, got {hits} hits",
    );
}

#[test]
fn a_ray_with_depth_keeps_its_geometry_and_only_changes_the_budget() {
    let ray: Ray = Ray::new(Vector3D::new(0.0, 0.0, 0.0), Vector3D::new(0.0, 0.0, -1.0));
    let deeper: Ray = ray.with_depth(3);
    assert_eq!(
        deeper.get_origin(),
        ray.get_origin(),
        "the origin is carried over"
    );
    assert_eq!(
        deeper.get_direction(),
        ray.get_direction(),
        "and so is the direction"
    );
    assert_eq!(deeper.get_t_min(), ray.get_t_min(), "and the near bound");
    assert_eq!(deeper.get_t_max(), ray.get_t_max(), "and the far bound");
    assert_eq!(deeper.get_depth(), 3, "only the bounce budget changes");
    assert_eq!(
        ray.get_depth(),
        0,
        "and the receiver is untouched, since with_depth takes &self and clones"
    );
}

#[test]
fn trace_follows_the_reflection_path_while_a_zero_bounce_budget_does_not() {
    let eye: Vector3D = Vector3D::new(0.0, 0.0, 10.0);
    let mut lights: LightingUniforms = LightingUniforms::with_eye(eye);
    lights.set_ambient(Vector3D::zero());
    let mirror_material: Material = Material::phong(Vector3D::zero(), 1.0, 32.0);
    let mirror: Occluder = Occluder::sphere(Vector3D::zero(), 1.0, mirror_material);
    let emissive_material: Material = Material::emissive(Vector3D::new(0.0, 1.0, 0.0));
    let emissive: Occluder =
        Occluder::sphere(Vector3D::new(0.0, 0.0, 15.0), 1.0, emissive_material);
    let scene: RayTraceScene = RayTraceScene::new(vec![mirror, emissive]);
    let ray: Ray = Ray::new(Vector3D::new(0.0, 0.0, 10.0), Vector3D::new(0.0, 0.0, -1.0));
    let full: Vector3D = scene.trace(ray.clone(), &lights);
    let no_bounce: Vector3D = scene.trace_with_bounces(ray, &lights, 0);
    assert!(
        full.get_y() > no_bounce.get_y(),
        "trace walks the reflection path on its own, so a zero bounce budget must come back dimmer: \
         trace {} vs trace_with_bounces(.., 0) {}",
        full.get_y(),
        no_bounce.get_y()
    );
}
