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
