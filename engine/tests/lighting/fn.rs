use super::*;

const EPSILON: f64 = 1e-9;
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
fn phong_specular_peak() {
    let normal: Vector3D = Vector3D::new(0.0, 1.0, 0.0);
    let light_dir: Vector3D = Vector3D::new(0.0, -1.0, 0.0);
    let view_dir: Vector3D = Vector3D::new(0.0, 1.0, 0.0);
    let light: Light = Light::new(
        LightType::Directional,
        Vector3D::zero(),
        light_dir,
        Vector3D::new(1.0, 1.0, 1.0),
        1.0,
        0.0,
        0.0,
    );
    let material: Material = Material::phong(Vector3D::new(1.0, 1.0, 1.0), 1.0, 32.0);
    let result: Vector3D = compute_phong(&light, normal, view_dir, &material);
    assert!(
        (result.get_x() - 1.0).abs() < EPSILON,
        "expected specular peak ~1.0, got {}",
        result.get_x(),
    );
    assert!(
        (result.get_y() - 1.0).abs() < EPSILON,
        "expected specular peak ~1.0, got {}",
        result.get_y(),
    );
    assert!(
        (result.get_z() - 1.0).abs() < EPSILON,
        "expected specular peak ~1.0, got {}",
        result.get_z(),
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
    let uniforms: LightingUniforms = lighting_setup();
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
