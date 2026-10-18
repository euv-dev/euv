use super::*;

fn epsilon(actual: f64, expected: f64) -> bool {
    (actual - expected).abs() < 1e-9
}

const EPSILON: f64 = 1e-9;
type CountingVisibility = Box<dyn FnMut(&Light, Vector3D, f64) -> f64>;

#[test]
fn lambert_diffuse_face_normal() {
    let light: Light =
        Light::new_directional(Vector3D::new(0.0, 1.0, 0.0), Vector3D::new(1.0, 0.0, 0.0));
    let material: Material = Material::lambert(Vector3D::new(0.5, 0.5, 0.5));
    let normal: Vector3D = Vector3D::new(0.0, 1.0, 0.0);
    let result: Vector3D = compute_lambert(&light, normal, &material);
    let expected: f64 = 1.0 * 1.0 * 1.0 * 0.5;
    assert!(
        (result.get_x() - expected).abs() < EPSILON,
        "expected red channel {expected}, got {}",
        result.get_x(),
    );
    assert!(
        result.get_y().abs() < EPSILON,
        "expected green channel 0.0, got {}",
        result.get_y(),
    );
    assert!(
        result.get_z().abs() < EPSILON,
        "expected blue channel 0.0, got {}",
        result.get_z(),
    );
}

#[test]
fn phong_specular_is_one_when_the_light_the_normal_and_the_eye_all_point_along_the_normal() {
    let normal: Vector3D = Vector3D::new(0.0, 1.0, 0.0);
    let color: Vector3D = Vector3D::new(1.0, 1.0, 1.0);
    let light: Light = Light::new_directional(normal, color);
    let material: Material = Material::phong(color, 1.0, 32.0);
    let result: Vector3D = compute_phong(&light, normal, normal, &material);
    assert!(
        (result.get_x() - 1.0).abs() < EPSILON
            && (result.get_y() - 1.0).abs() < EPSILON
            && (result.get_z() - 1.0).abs() < EPSILON,
        "with l, n, and v all on +y the reflection reflect(-l, n) equals n, so dot(reflect, v) is 1 and the lobe is full, got {result:?}"
    );
}

#[test]
fn phong_specular_is_zero_for_a_back_facing_light_even_when_the_eye_sits_on_the_reflection() {
    let normal: Vector3D = Vector3D::new(0.0, 1.0, 0.0);
    let opposite_the_normal: Vector3D = Vector3D::new(0.0, -1.0, 0.0);
    let color: Vector3D = Vector3D::new(1.0, 1.0, 1.0);
    let material: Material = Material::phong(color, 1.0, 32.0);
    let light: Light = Light::new_directional(opposite_the_normal, color);
    let eye_on_the_reflection: Vector3D =
        compute_phong(&light, normal, opposite_the_normal, &material);
    assert!(
        eye_on_the_reflection.get_x().abs() < EPSILON
            && eye_on_the_reflection.get_y().abs() < EPSILON
            && eye_on_the_reflection.get_z().abs() < EPSILON,
        "dot(n, l) is -1 yet reflect(-l, n) equals -n and the eye sits on -n, so dot(reflect, v) would be 1 without the NdotL > 0 gate; the gate alone must force zero, got {eye_on_the_reflection:?}"
    );
    let eye_in_front: Vector3D = compute_phong(&light, normal, normal, &material);
    assert!(
        eye_in_front.get_x().abs() < EPSILON,
        "the same back-facing light seen by an eye in front is zero by the dot(reflect, v) clamp instead of the facing gate, got {}",
        eye_in_front.get_x(),
    );
}

#[test]
fn phong_highlight_peaks_at_the_reflection_of_the_light_and_not_at_the_half_vector() {
    let normal: Vector3D = Vector3D::new(0.0, 1.0, 0.0);
    let color: Vector3D = Vector3D::new(1.0, 1.0, 1.0);
    let light_dir: Vector3D = Vector3D::new(0.6, 0.8, 0.0);
    let light: Light = Light::new_directional(light_dir, color);
    let shininess: f64 = 2.0;
    let material: Material = Material::phong(color, 1.0, shininess);
    let reflected: Vector3D = Vector3D::new(-0.6, 0.8, 0.0);
    assert!(
        (reflected.get_x().powf(2.0) + reflected.get_y().powf(2.0) - 1.0).abs() < EPSILON,
        "2 * n * dot(n, l) - l is already unit length for unit n and l, so the lobe needs no normalization, yet this fixture vector squared to {}",
        reflected.get_x().powf(2.0) + reflected.get_y().powf(2.0),
    );
    let on_the_peak: Vector3D = compute_phong(&light, normal, reflected, &material);
    assert!(
        (on_the_peak.get_x() - 1.0).abs() < EPSILON,
        "the lobe peaks at exactly 1.0 where the eye sits on reflect(-l, n) == (-0.6, 0.8, 0.0), got {}",
        on_the_peak.get_x(),
    );
    let seen_from_the_normal: Vector3D = compute_phong(&light, normal, normal, &material);
    assert!(
        (seen_from_the_normal.get_x() - 0.64).abs() < EPSILON,
        "seen from the normal the angle to the reflection is 0.8, so the reflective Phong factor is 0.8^2 == 0.64, got {}",
        seen_from_the_normal.get_x(),
    );
    assert!(
        (seen_from_the_normal.get_x() - 0.9).abs() > 1e-6,
        "the reflective Phong lobe must not read as the Blinn half-vector value pow(0.9, 2) == 0.9, got {}",
        seen_from_the_normal.get_x(),
    );
}

#[test]
fn point_light_falloff_distance() {
    let falloff: f64 = 1.0;
    let f0: f64 = apply_falloff(0.0, falloff);
    let f1: f64 = apply_falloff(1.0, falloff);
    let f2: f64 = apply_falloff(2.0, falloff);
    assert!((f0 - 1.0).abs() < EPSILON, "d=0 should yield 1.0, got {f0}");
    assert!(
        (f1 - 1.0 / (1.0 + 1.0)).abs() < EPSILON,
        "d=1 should yield 0.5, got {f1}",
    );
    assert!(
        (f2 - 1.0 / (1.0 + 4.0)).abs() < EPSILON,
        "d=2 should yield 0.2, got {f2}",
    );
}

#[test]
fn ray_sphere_intersect_hit_miss_inside() {
    let origin: Vector3D = Vector3D::new(0.0, 0.0, 5.0);
    let dir: Vector3D = Vector3D::new(0.0, 0.0, -1.0);
    let center: Vector3D = Vector3D::zero();
    let radius: f64 = 1.0;
    let hit: Option<(f64, Vector3D)> = ray_sphere_intersect(origin, dir, center, radius);
    assert!(hit.is_some(), "ray from outside should hit sphere");
    let (t, normal): (f64, Vector3D) = hit.unwrap();
    assert!((t - 4.0).abs() < EPSILON, "expected t=4, got {t}");
    assert!(
        (normal.get_z() - 1.0).abs() < EPSILON,
        "expected normal (0,0,1), got (0,0,{})",
        normal.get_z(),
    );
    let origin_miss: Vector3D = Vector3D::new(10.0, 0.0, 5.0);
    let dir_miss: Vector3D = Vector3D::new(0.0, 0.0, -1.0);
    let miss: Option<(f64, Vector3D)> = ray_sphere_intersect(origin_miss, dir_miss, center, radius);
    assert!(miss.is_none(), "ray far from sphere should miss");
    let origin_in: Vector3D = Vector3D::zero();
    let dir_in: Vector3D = Vector3D::new(1.0, 0.0, 0.0);
    let inside: Option<(f64, Vector3D)> = ray_sphere_intersect(origin_in, dir_in, center, radius);
    assert!(
        inside.is_some(),
        "ray from inside should still hit exit point"
    );
    let (t_in, normal_in): (f64, Vector3D) = inside.unwrap();
    assert!(
        (t_in - 1.0).abs() < EPSILON,
        "expected t=1 (exit through +x), got {t_in}",
    );
    assert!(
        (normal_in.get_x() - 1.0).abs() < EPSILON,
        "expected exit normal (1,0,0), got ({},0,0)",
        normal_in.get_x(),
    );
}

#[test]
fn soft_shadow_no_occluder_returns_one() {
    let origin: Vector3D = Vector3D::zero();
    let light_pos: Vector3D = Vector3D::new(0.0, 0.0, 10.0);
    let occluders: [(Vector3D, f64); 0] = [];
    let v: f64 = soft_shadow_factor(origin, light_pos, &occluders);
    assert!(
        (v - 1.0).abs() < EPSILON,
        "empty occluders should yield 1.0, got {v}"
    );
}

fn lighting_setup() -> LightingUniforms {
    let mut uniforms: LightingUniforms = LightingUniforms::with_eye(Vector3D::new(0.0, 0.0, 10.0));
    uniforms.add_light(Light::new_directional(
        Vector3D::new(0.0, 0.0, -1.0),
        Vector3D::new(1.0, 1.0, 1.0),
    ));
    uniforms
}

#[test]
fn shade_lambert_has_no_specular_lobe() {
    let mut uniforms: LightingUniforms = LightingUniforms::with_eye(Vector3D::new(-6.0, 0.0, 8.0));
    uniforms.add_light(Light::new_directional(
        Vector3D::new(0.6, 0.0, 0.8),
        Vector3D::new(1.0, 1.0, 1.0),
    ));
    let normal: Vector3D = Vector3D::new(0.0, 0.0, 1.0);
    let pos: Vector3D = Vector3D::zero();
    let lambert: Material = Material::new(
        MaterialKind::Lambert,
        Vector3D::new(0.5, 0.5, 0.5),
        1.0,
        32.0,
        Vector3D::zero(),
    );
    let shaded: Vector3D = uniforms.shade(pos, normal, &lambert, &[]);
    let phong: Material = Material::new(
        MaterialKind::Phong,
        Vector3D::new(0.5, 0.5, 0.5),
        1.0,
        32.0,
        Vector3D::zero(),
    );
    let shaded_phong: Vector3D = uniforms.shade(pos, normal, &phong, &[]);
    assert!(
        (shaded.get_z() - 0.5).abs() < EPSILON,
        "ambient 0.1 plus diffuse 0.5 * 0.8 with no specular lobe at all, got {}",
        shaded.get_z(),
    );
    assert!(
        (shaded_phong.get_z() - 1.5).abs() < EPSILON,
        "ambient 0.1 plus diffuse 0.4 plus a full specular lobe aimed at this eye, got {}",
        shaded_phong.get_z(),
    );
    assert!(
        shaded_phong.get_z() > shaded.get_z() + EPSILON,
        "a Phong material must pick up a specular lobe the Lambert one lacks: phong={} lambert={}",
        shaded_phong.get_z(),
        shaded.get_z()
    );
}

#[test]
fn shade_pbr_ignores_specular_fields() {
    let uniforms: LightingUniforms = lighting_setup();
    let normal: Vector3D = Vector3D::new(0.0, 0.0, 1.0);
    let pos: Vector3D = Vector3D::zero();
    let no_spec: Material = Material::new(
        MaterialKind::Pbr,
        Vector3D::new(0.5, 0.5, 0.5),
        0.0,
        32.0,
        Vector3D::zero(),
    );
    let max_spec: Material = Material::new(
        MaterialKind::Pbr,
        Vector3D::new(0.5, 0.5, 0.5),
        1.0,
        32.0,
        Vector3D::zero(),
    );
    let a: Vector3D = uniforms.shade(pos, normal, &no_spec, &[]);
    let b: Vector3D = uniforms.shade(pos, normal, &max_spec, &[]);
    assert!(
        (a.get_x() - b.get_x()).abs() < EPSILON
            && (a.get_y() - b.get_y()).abs() < EPSILON
            && (a.get_z() - b.get_z()).abs() < EPSILON,
        "Pbr must not read the specular field: {a:?} vs {b:?}"
    );
}

#[test]
fn shade_pbr_is_brighter_than_lambert_at_same_albedo() {
    let uniforms: LightingUniforms = lighting_setup();
    let normal: Vector3D = Vector3D::new(0.0, 0.0, 1.0);
    let pos: Vector3D = Vector3D::zero();
    let pbr: Material = Material::pbr(Vector3D::new(0.5, 0.5, 0.5));
    let lambert: Material = Material::lambert(Vector3D::new(0.5, 0.5, 0.5));
    let pbr_shade: Vector3D = uniforms.shade(pos, normal, &pbr, &[]);
    let lambert_shade: Vector3D = uniforms.shade(pos, normal, &lambert, &[]);
    assert!(
        pbr_shade.get_z() >= lambert_shade.get_z() - EPSILON,
        "the fresnel blend must not darken diffuse: pbr={} lambert={}",
        pbr_shade.get_z(),
        lambert_shade.get_z()
    );
}

#[test]
fn schlick_fresnel_head_on_returns_floor() {
    let normal: Vector3D = Vector3D::new(0.0, 0.0, 1.0);
    let light: Vector3D = Vector3D::new(0.0, 0.0, 1.0);
    let view: Vector3D = Vector3D::new(0.0, 0.0, 1.0);
    let f: f64 = apply_schlick_fresnel(light, view, normal);
    assert!(
        (f - 0.04).abs() < EPSILON,
        "head-on fresnel must equal the F0 floor, got {f}"
    );
}

#[test]
fn schlick_fresnel_rises_away_from_head_on() {
    let normal: Vector3D = Vector3D::new(0.0, 0.0, 1.0);
    let head_on: f64 = apply_schlick_fresnel(
        Vector3D::new(0.0, 0.0, 1.0),
        Vector3D::new(0.0, 0.0, 1.0),
        normal,
    );
    let grazing: f64 = apply_schlick_fresnel(
        Vector3D::new(1.0, 0.0, 0.0),
        Vector3D::new(1.0, 0.0, -1.0).normalized(),
        normal,
    );
    assert!(
        grazing > head_on + 0.05,
        "a grazing half-vector must raise the fresnel term: head_on={head_on} grazing={grazing}"
    );
}

#[test]
fn schlick_fresnel_stays_in_unit_range() {
    let normal: Vector3D = Vector3D::new(0.0, 0.0, 1.0);
    for (lx, ly, lz) in [(0.0, 0.0, 1.0), (1.0, 0.0, 0.0), (0.0, 1.0, 0.0)] {
        let f: f64 = apply_schlick_fresnel(
            Vector3D::new(lx, ly, lz),
            Vector3D::new(0.0, 0.0, 1.0),
            normal,
        );
        assert!(
            (0.04 - EPSILON..=1.0 + EPSILON).contains(&f),
            "fresnel out of range: {f}"
        );
    }
}

#[test]
fn a_point_light_has_no_direction() {
    let light: Light = Light::new_point(
        Vector3D::new(1.0, 2.0, 3.0),
        Vector3D::new(0.5, 0.5, 0.5),
        0.75,
    );
    assert_eq!(light.get_kind(), LightType::Point, "the type is Point");
    assert!(
        epsilon(light.get_position().get_y(), 2.0),
        "the position round-trips"
    );
    assert!(
        epsilon(light.get_intensity(), 0.75),
        "the intensity round-trips"
    );
    assert_eq!(
        light.get_direction(),
        Vector3D::zero(),
        "a point light radiates in every direction, so its direction is the zero vector"
    );
    assert!(
        epsilon(light.get_spot_cos(), 0.0),
        "a point light stores a zero cone cosine, so the shading cone test never culls it, got {}",
        light.get_spot_cos()
    );
    assert!(
        epsilon(light.get_falloff(), 1.0),
        "and a falloff of one, got {}",
        light.get_falloff()
    );
}

#[test]
fn a_spot_light_carries_its_direction_and_half_angle() {
    let direction: Vector3D = Vector3D::new(0.0, -1.0, 0.0);
    let light: Light = Light::new_spot(
        Vector3D::zero(),
        direction,
        Vector3D::new(1.0, 1.0, 1.0),
        2.0,
        0.5,
    );
    assert_eq!(light.get_kind(), LightType::Spot, "the type is Spot");
    assert_eq!(
        light.get_direction(),
        direction,
        "the direction round-trips"
    );
    assert!(
        epsilon(light.get_intensity(), 2.0),
        "the intensity round-trips"
    );
    assert!(
        epsilon(light.get_spot_cos(), 0.5_f64.cos()),
        "the half angle is stored as its cosine, got {}",
        light.get_spot_cos()
    );
}

#[test]
fn a_directional_light_points_and_is_unbounded() {
    let light: Light =
        Light::new_directional(Vector3D::new(0.0, 3.0, 0.0), Vector3D::new(1.0, 1.0, 1.0));
    assert_eq!(light.get_kind(), LightType::Directional, "the type sticks");
    assert_eq!(
        light.get_direction(),
        Vector3D::new(0.0, 1.0, 0.0),
        "a directional light is defined by its direction alone"
    );
    assert!(
        epsilon(light.get_falloff(), 0.0),
        "and no falloff term, since a directional light never attenuates, got {}",
        light.get_falloff()
    );
    assert!(
        light.get_position() == Vector3D::zero(),
        "a directional light is not positioned anywhere, it comes from infinity"
    );
    assert!(
        epsilon(light.get_spot_cos(), 0.0),
        "and it has no cone either, got {}",
        light.get_spot_cos()
    );
}

#[test]
fn the_light_type_enum_is_matchable() {
    let point: LightType = LightType::Point;
    let spot: LightType = LightType::Spot;
    let label: &str = match point {
        LightType::Point => "point",
        LightType::Spot => "spot",
        LightType::Directional => "directional",
    };
    assert_eq!(label, "point", "a point light labels as point");
    assert_ne!(spot, point, "and spot is a distinct type");
}

#[test]
fn the_collider_shape_enums_default_to_the_box_form() {
    assert_eq!(
        ColliderShape::default(),
        ColliderShape::Aabb,
        "the 2D default is Aabb"
    );
    assert_eq!(
        ColliderShape3D::default(),
        ColliderShape3D::Aabb,
        "the 3D default is Aabb"
    );
    assert_ne!(
        ColliderShape::Circle,
        ColliderShape::Aabb,
        "the variants are distinct"
    );
    assert_ne!(ColliderShape3D::Sphere, ColliderShape3D::Aabb, "in 3D too");
}

#[test]
fn a_spot_lights_a_surface_on_its_beam_and_leaves_one_off_axis_dark() {
    let half_angle: f64 = 0.5;
    let drop: f64 = 10.0;
    let lamp: Vector3D = Vector3D::new(0.0, drop, 0.0);
    let axis: Vector3D = Vector3D::new(0.0, -1.0, 0.0);
    let eye: Vector3D = Vector3D::new(0.0, 0.0, 30.0);
    let color: Vector3D = Vector3D::new(1.0, 1.0, 1.0);
    let mut uniforms: LightingUniforms = LightingUniforms::with_eye(eye);
    uniforms.add_light(Light::new_spot(lamp, axis, color, 1.0, half_angle));
    let material: Material = Material::lambert(color);
    let facing_up: Vector3D = Vector3D::new(0.0, 1.0, 0.0);
    let on_beam: Vector3D = uniforms.shade(Vector3D::zero(), facing_up, &material, &[]);
    let on_beam_expected: f64 = 0.1 + 1.0 / (1.0 + drop * drop);
    assert!(
        (on_beam.get_x() - on_beam_expected).abs() < 1e-12,
        "10 units below a lamp on the beam has cosine 1.0 and falloff 1/(1+100) = 0.0099, so ambient 0.1 + that gives {on_beam_expected}, got {}",
        on_beam.get_x(),
    );
    let ninety_degrees_off_axis: Vector3D =
        uniforms.shade(Vector3D::new(10.0, 0.0, 0.0), facing_up, &material, &[]);
    assert!(
        (ninety_degrees_off_axis.get_x() - 0.1).abs() < EPSILON,
        "a surface 90 degrees off a 0.5 rad cone must fall back to ambient 0.1 even though its normal faces the lamp, got {}",
        ninety_degrees_off_axis.get_x(),
    );
    let slant: f64 = drop * 0.4f64.tan();
    let inside: Vector3D =
        uniforms.shade(Vector3D::new(slant, 0.0, 0.0), facing_up, &material, &[]);
    let true_distance: f64 = drop / 0.4f64.cos();
    let cosine: f64 = drop / true_distance;
    let falloff: f64 = 1.0 / (1.0 + true_distance * true_distance);
    let expected: f64 = 0.1 + cosine * falloff;
    assert!(
        (inside.get_x() - expected).abs() < 1e-12,
        "a surface exactly 0.4 rad off axis is inside the cone, giving cosine {cosine} and falloff {falloff} over ambient 0.1, expected {expected}, got {}",
        inside.get_x(),
    );
}

#[test]
fn a_surface_exactly_on_the_spot_cone_boundary_is_lit_and_one_step_past_it_is_not() {
    let half_angle: f64 = 0.5;
    let drop: f64 = 10.0;
    let boundary_slant: f64 = drop * half_angle.tan();
    let true_distance: f64 = drop / half_angle.cos();
    let eye: Vector3D = Vector3D::new(0.0, 0.0, 30.0);
    let color: Vector3D = Vector3D::new(1.0, 1.0, 1.0);
    let mut uniforms: LightingUniforms = LightingUniforms::with_eye(eye);
    uniforms.add_light(Light::new_spot(
        Vector3D::new(0.0, drop, 0.0),
        Vector3D::new(0.0, -1.0, 0.0),
        color,
        1.0,
        half_angle,
    ));
    let material: Material = Material::lambert(color);
    let surface_normal: Vector3D = Vector3D::new(boundary_slant, drop, 0.0).normalized();
    let on_boundary: Vector3D = uniforms.shade(
        Vector3D::new(boundary_slant, 0.0, 0.0),
        surface_normal,
        &material,
        &[],
    );
    let surface_cosine: f64 =
        (drop * drop - boundary_slant * boundary_slant) / (true_distance * true_distance);
    let falloff: f64 = 1.0 / (1.0 + true_distance * true_distance);
    let expected: f64 = 0.1 + surface_cosine * falloff;
    assert!(
        (on_boundary.get_x() - expected).abs() < 1e-12,
        "the cone test is dot(light_dir, axis) <= -cos(half_angle), so the surface exactly on the boundary is lit; its normal tilts toward the lamp as much as the lamp tilts away, making the diffuse cosine {surface_cosine} with falloff {falloff} over ambient 0.1, expected {expected}, got {}",
        on_boundary.get_x(),
    );
    let just_past: Vector3D = uniforms.shade(
        Vector3D::new(boundary_slant * 1.01, 0.0, 0.0),
        surface_normal,
        &material,
        &[],
    );
    assert!(
        (just_past.get_x() - 0.1).abs() < EPSILON,
        "1 percent past the cone boundary must fall back to ambient 0.1, got {}",
        just_past.get_x(),
    );
}

#[test]
fn the_visibility_callback_is_called_once_per_light_with_the_true_distance_and_unit_direction() {
    let eye: Vector3D = Vector3D::new(0.0, 0.0, 10.0);
    let color: Vector3D = Vector3D::new(1.0, 1.0, 1.0);
    let lamp: Vector3D = Vector3D::new(0.0, 3.0, 4.0);
    let mut uniforms: LightingUniforms = LightingUniforms::with_eye(eye);
    uniforms.add_light(Light::new_point(lamp, color, 1.0));
    let material: Material = Material::lambert(color);
    let facing_up: Vector3D = Vector3D::new(0.0, 1.0, 0.0);
    let mut calls: Vec<(LightType, Vector3D, f64)> = Vec::new();
    let shaded: Vector3D = uniforms.shade_with_visibility(
        Vector3D::zero(),
        facing_up,
        &material,
        |light: &Light, dir: Vector3D, max: f64| {
            calls.push((light.get_kind(), dir, max));
            1.0
        },
    );
    assert_eq!(
        calls.len(),
        1,
        "one contributing light must cost exactly one callback, got {calls:?}"
    );
    let recorded: (LightType, Vector3D, f64) = calls[0];
    let kind: LightType = recorded.0;
    let dir: Vector3D = recorded.1;
    let max_distance: f64 = recorded.2;
    assert_eq!(
        kind,
        LightType::Point,
        "the callback must receive the light itself so it can read its position"
    );
    assert!(
        (dir.get_y() - 0.6).abs() < EPSILON && (dir.get_z() - 0.8).abs() < EPSILON,
        "the callback direction must be the unit surface-to-light vector (0, 0.6, 0.8) of the 3-4-5 triangle, got {dir:?}"
    );
    assert!(
        (max_distance - 5.0).abs() < EPSILON,
        "max_distance must be the true surface-to-light distance 5.0 of the 3-4-5 triangle, never the 0.001 falloff floor, got {max_distance}"
    );
    let expected: f64 = 0.1 + 0.6 / 26.0;
    assert!(
        (shaded.get_x() - expected).abs() < 1e-9,
        "cosine 0.6 with falloff 1/(1+25) over ambient 0.1 gives {expected}, got {}",
        shaded.get_x(),
    );
}

#[test]
fn the_visibility_factor_scales_the_whole_contribution_and_zero_leaves_only_ambient() {
    let eye: Vector3D = Vector3D::new(0.0, 0.0, 10.0);
    let red_only: Vector3D = Vector3D::new(1.0, 0.0, 0.0);
    let facing_camera: Vector3D = Vector3D::new(0.0, 0.0, 1.0);
    let mut uniforms: LightingUniforms = LightingUniforms::with_eye(eye);
    uniforms.add_light(Light::new_directional(facing_camera, red_only));
    let material: Material = Material::lambert(Vector3D::new(1.0, 1.0, 1.0));
    let half_visible: Vector3D = uniforms.shade_with_visibility(
        Vector3D::zero(),
        facing_camera,
        &material,
        |_light: &Light, _dir: Vector3D, _max: f64| 0.5,
    );
    let fully_occluded: Vector3D = uniforms.shade_with_visibility(
        Vector3D::zero(),
        facing_camera,
        &material,
        |_light: &Light, _dir: Vector3D, _max: f64| 0.0,
    );
    assert!(
        (half_visible.get_x() - 0.6).abs() < EPSILON,
        "a cosine of 1.0 at 0.5 visibility over ambient 0.1 gives 0.6, got {}",
        half_visible.get_x(),
    );
    assert!(
        (fully_occluded.get_x() - 0.1).abs() < EPSILON,
        "a fully occluded light must leave only the unshadowed ambient 0.1, got {}",
        fully_occluded.get_x(),
    );
}

#[test]
fn back_facing_zero_intensity_and_out_of_cone_lights_cost_no_callback() {
    let eye: Vector3D = Vector3D::new(0.0, 0.0, 10.0);
    let color: Vector3D = Vector3D::new(1.0, 1.0, 1.0);
    let facing_camera: Vector3D = Vector3D::new(0.0, 0.0, 1.0);
    let away_from_camera: Vector3D = Vector3D::new(0.0, 0.0, -1.0);
    let calls: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let counter: Rc<Cell<u32>> = Rc::clone(&calls);
    let mut count: CountingVisibility =
        Box::new(move |_light: &Light, _dir: Vector3D, _max: f64| -> f64 {
            counter.set(counter.get() + 1);
            1.0
        });
    let material: Material = Material::lambert(color);
    let mut sun_behind_the_surface: LightingUniforms = LightingUniforms::with_eye(eye);
    sun_behind_the_surface.add_light(Light::new_directional(away_from_camera, color));
    let _: Vector3D = sun_behind_the_surface.shade_with_visibility(
        Vector3D::zero(),
        facing_camera,
        &material,
        &mut count,
    );
    assert_eq!(
        calls.get(),
        0,
        "a directional light's direction points toward the lamp, so a surface whose normal opposes it has dot(n, l) < 0 and must be rejected before any shadow ray"
    );
    let mut unlit: LightingUniforms = LightingUniforms::with_eye(eye);
    unlit.add_light(Light::new(
        LightType::Directional,
        Vector3D::zero(),
        facing_camera,
        color,
        0.0,
        0.0,
        0.0,
    ));
    let _: Vector3D =
        unlit.shade_with_visibility(Vector3D::zero(), facing_camera, &material, &mut count);
    assert_eq!(
        calls.get(),
        0,
        "a zero-intensity light must be rejected before any shadow ray"
    );
    let mut spot: LightingUniforms = LightingUniforms::with_eye(eye);
    spot.add_light(Light::new_spot(
        Vector3D::zero(),
        Vector3D::new(0.0, -1.0, 0.0),
        color,
        1.0,
        0.2,
    ));
    let _: Vector3D = spot.shade_with_visibility(
        Vector3D::new(10.0, 0.0, 0.0),
        Vector3D::new(0.0, 1.0, 0.0),
        &material,
        &mut count,
    );
    assert_eq!(
        calls.get(),
        0,
        "a surface outside a 0.2 rad cone must be rejected before any shadow ray"
    );
    let facing: Vector3D = sun_behind_the_surface.shade_with_visibility(
        Vector3D::zero(),
        away_from_camera,
        &material,
        &mut count,
    );
    assert_eq!(
        calls.get(),
        1,
        "the same light facing the surface must reach the callback exactly once, shaded {facing:?}"
    );
}

#[test]
fn a_directional_light_reports_the_ray_tracer_far_bound_instead_of_a_distance() {
    let eye: Vector3D = Vector3D::new(0.0, 0.0, 10.0);
    let color: Vector3D = Vector3D::new(1.0, 1.0, 1.0);
    let facing_camera: Vector3D = Vector3D::new(0.0, 0.0, 1.0);
    let mut uniforms: LightingUniforms = LightingUniforms::with_eye(eye);
    uniforms.add_light(Light::new_directional(facing_camera, color));
    let material: Material = Material::lambert(color);
    let mut seen: f64 = -1.0;
    let _: Vector3D = uniforms.shade_with_visibility(
        Vector3D::zero(),
        facing_camera,
        &material,
        |_light: &Light, _dir: Vector3D, max: f64| -> f64 {
            seen = max;
            1.0
        },
    );
    assert!(
        (seen - 1000.0).abs() < EPSILON,
        "a directional light has no distance of its own, so its shadow ray runs to the 1000.0 far bound, got {seen}"
    );
}

#[test]
fn a_point_light_dimmonotonically_as_it_recedes_and_reports_true_distances() {
    let eye: Vector3D = Vector3D::new(0.0, 0.0, 10.0);
    let color: Vector3D = Vector3D::new(1.0, 1.0, 1.0);
    let facing_up: Vector3D = Vector3D::new(0.0, 1.0, 0.0);
    let material: Material = Material::lambert(color);
    let mut seen: Vec<f64> = Vec::new();
    let mut previous: f64 = f64::INFINITY;
    for distance in [0.5, 1.0, 2.0, 4.0, 8.0] {
        let mut uniforms: LightingUniforms = LightingUniforms::with_eye(eye);
        uniforms.add_light(Light::new_point(
            Vector3D::new(0.0, distance, 0.0),
            color,
            1.0,
        ));
        let shaded: Vector3D = uniforms.shade_with_visibility(
            Vector3D::zero(),
            facing_up,
            &material,
            |_light: &Light, _dir: Vector3D, max: f64| -> f64 {
                seen.push(max);
                1.0
            },
        );
        let expected: f64 = 0.1 + 1.0 / (1.0 + distance * distance);
        assert!(
            (shaded.get_x() - expected).abs() < 1e-9,
            "face-on at d={distance} falloff is 1/(1+d^2) over ambient 0.1, expected {expected}, got {}",
            shaded.get_x(),
        );
        assert!(
            shaded.get_x() < previous,
            "a point light must dim as it recedes, and the 0.5 sample sits strictly between the 0.001 falloff floor and any real distance so a clamped max_distance would show up here, {shaded:?} is not dimmer than {previous}"
        );
        previous = shaded.get_x();
    }
    assert_eq!(
        seen,
        vec![0.5, 1.0, 2.0, 4.0, 8.0],
        "every callback must see the true surface-to-light distance regardless of the falloff applied, got {seen:?}"
    );
}

#[test]
fn a_surface_inside_the_min_distance_still_reports_the_true_distance_to_its_lamp() {
    let eye: Vector3D = Vector3D::new(0.0, 0.0, 10.0);
    let color: Vector3D = Vector3D::new(1.0, 1.0, 1.0);
    let closer_than_the_falloff_floor: f64 = 1e-5;
    let lamp: Vector3D = Vector3D::new(0.0, closer_than_the_falloff_floor, 0.0);
    let mut uniforms: LightingUniforms = LightingUniforms::with_eye(eye);
    uniforms.add_light(Light::new_point(lamp, color, 1.0));
    let material: Material = Material::lambert(color);
    let facing_up: Vector3D = Vector3D::new(0.0, 1.0, 0.0);
    let mut seen: f64 = -1.0;
    let shaded: Vector3D = uniforms.shade_with_visibility(
        Vector3D::zero(),
        facing_up,
        &material,
        |_light: &Light, _dir: Vector3D, max: f64| -> f64 {
            seen = max;
            1.0
        },
    );
    assert!(
        shaded.get_x().is_finite(),
        "a surface 1e-5 from a lamp must not shade to NaN or infinity, got {}",
        shaded.get_x(),
    );
    let floored_falloff: f64 = 1.0 / (1.0 + 0.001f64 * 0.001);
    assert!(
        (shaded.get_x() - (0.1 + floored_falloff)).abs() < 1e-9,
        "the falloff distance alone is floored at 0.001, so a 1e-5 lamp distance must attenuate by 1/(1+0.001^2) == {floored_falloff} over ambient 0.1, got {}",
        shaded.get_x(),
    );
    assert!(
        (seen - closer_than_the_falloff_floor).abs() < 1e-12,
        "max_distance must stay the true 1e-5 distance, never the 0.001 falloff floor, got {seen}"
    );
}

#[test]
fn a_surface_exactly_on_a_point_light_is_finite_and_costs_no_callback() {
    let eye: Vector3D = Vector3D::new(0.0, 0.0, 10.0);
    let color: Vector3D = Vector3D::new(1.0, 1.0, 1.0);
    let mut uniforms: LightingUniforms = LightingUniforms::with_eye(eye);
    uniforms.add_light(Light::new_point(Vector3D::zero(), color, 1.0));
    let material: Material = Material::lambert(color);
    let facing_up: Vector3D = Vector3D::new(0.0, 1.0, 0.0);
    let calls: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let counter: Rc<Cell<u32>> = Rc::clone(&calls);
    let mut count: CountingVisibility =
        Box::new(move |_light: &Light, _dir: Vector3D, _max: f64| -> f64 {
            counter.set(counter.get() + 1);
            1.0
        });
    let shaded: Vector3D =
        uniforms.shade_with_visibility(Vector3D::zero(), facing_up, &material, &mut count);
    assert!(
        shaded.get_x().is_finite(),
        "a surface sitting exactly on a lamp has a zero direction vector and must not shade to NaN or infinity, got {}",
        shaded.get_x(),
    );
    assert_eq!(
        calls.get(),
        0,
        "the zero direction fails the dot(n, l) > 0 facing test, so a coincident surface is culled before any shadow ray"
    );
}
