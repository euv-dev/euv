use super::*;

fn camera() -> Camera3D {
    Camera3D::create(Vector3D::new(0.0, 0.0, 6.0), Vector3D::zero(), 64.0, 64.0)
}

fn mixed_scene() -> RayTraceScene {
    let occluders: Vec<Occluder> = vec![
        Occluder::sphere(
            Vector3D::new(-1.2, 0.0, 0.0),
            0.9,
            Material::lambert(Vector3D::new(0.8, 0.2, 0.2)),
        ),
        Occluder::aabb(
            Vector3D::new(0.4, -0.6, -0.6),
            Vector3D::new(1.8, 0.6, 0.6),
            Material::phong(Vector3D::new(0.2, 0.8, 0.3), 0.6, 32.0),
        ),
        Occluder::triangle(
            Vector3D::new(-0.2, -0.9, 1.0),
            Vector3D::new(0.6, -0.9, 1.0),
            Vector3D::new(0.2, 0.9, 1.0),
            Material::pbr(Vector3D::new(0.2, 0.3, 0.9)),
        ),
    ];
    RayTraceScene::new(occluders)
}

fn lit_scene() -> LightingUniforms {
    let mut lights: LightingUniforms = LightingUniforms::with_eye(Vector3D::new(0.0, 0.0, 6.0));
    lights.set_ambient(Vector3D::new(0.1, 0.12, 0.15));
    lights.add_light(Light::new_directional(
        Vector3D::new(0.4, 0.8, 1.0),
        Vector3D::new(1.0, 0.95, 0.9),
    ));
    lights.add_light(Light::new_point(
        Vector3D::new(2.0, 2.0, 2.0),
        Vector3D::new(0.6, 0.7, 1.0),
        1.2,
    ));
    lights.add_light(Light::new_spot(
        Vector3D::new(-2.0, 2.5, 1.0),
        Vector3D::new(0.3, -1.0, -0.4),
        Vector3D::new(1.0, 0.8, 0.5),
        2.0,
        0.6,
    ));
    lights
}

#[test]
fn settings_defaults_validate() {
    let settings: RayTraceSettings = RayTraceSettings::new_default();
    assert_eq!(settings.get_samples_per_axis(), 1);
    assert_eq!(settings.get_max_bounces(), 4);
    assert!(settings.validate().is_ok());
    assert_eq!(settings.rays_per_pixel(), 1);
    assert_eq!(
        RayTraceSettings::with_quality(3, 2).rays_per_pixel(),
        9,
        "rays_per_pixel must be samples_per_axis squared"
    );
}

#[test]
fn settings_validate_rejects_out_of_range() {
    for samples in [0u32, 5, 1000] {
        let settings: RayTraceSettings = RayTraceSettings::with_quality(samples, 1);
        assert!(
            settings.validate().is_err(),
            "samples_per_axis {samples} must be rejected"
        );
    }
    for bounces in [17u32, 1000] {
        let settings: RayTraceSettings = RayTraceSettings::with_quality(1, bounces);
        assert!(
            settings.validate().is_err(),
            "max_bounces {bounces} must be rejected"
        );
    }
    assert!(
        RayTraceSettings::with_quality(4, 16).validate().is_ok(),
        "both inclusive upper bounds must be accepted"
    );
    assert!(
        RayTraceSettings::with_quality(1, 0).validate().is_ok(),
        "zero bounces must be accepted"
    );
}

#[test]
fn encode_gpu_empty_scene_packs_only_a_header() {
    let scene: RayTraceScene = RayTraceScene::new(Vec::new());
    let packed: Vec<f32> = match scene.encode_gpu() {
        Ok(data) => data,
        Err(reason) => panic!("empty scene must pack: {reason}"),
    };
    assert_eq!(packed.len(), 4, "an empty scene is exactly one vec4 header");
    assert_eq!(packed[0], 0.0, "node_count");
    assert_eq!(packed[1], 0.0, "primitive_count");
    assert_eq!(packed[2], 1.0, "primitive_offset is header + nodes");
}

#[test]
fn encode_gpu_header_shapes_and_indices() {
    let scene: RayTraceScene = mixed_scene();
    let packed: Vec<f32> = match scene.encode_gpu() {
        Ok(data) => data,
        Err(reason) => panic!("mixed scene must pack: {reason}"),
    };
    let node_count: f32 = packed[0];
    let primitive_count: f32 = packed[1];
    let primitive_offset: f32 = packed[2];
    assert_eq!(primitive_count, 3.0, "one record per occluder");
    assert_eq!(primitive_offset, 1.0 + 3.0 * node_count);
    assert_eq!(
        packed.len() % 4,
        0,
        "the packed buffer is a whole number of vec4 slots"
    );
    let total_slots: usize = packed.len() / 4;
    let expected_slots: usize = 1 + (node_count as usize) * 3 + (primitive_count as usize) * 8;
    assert_eq!(total_slots, expected_slots, "no padding slots are appended");
    for (record, expected_shape) in [(0usize, 0.0f32), (1, 1.0), (2, 2.0)] {
        let shape_slot: usize = primitive_offset as usize + record * 8;
        assert_eq!(packed[shape_slot * 4 + 3], expected_shape);
    }
    let original_indices: Vec<f32> = (0..3)
        .map(|record: usize| {
            let vertex0: usize = primitive_offset as usize + record * 8 + 2;
            packed[vertex0 * 4 + 3]
        })
        .collect();
    for (expected, original) in original_indices.iter().enumerate().take(3) {
        assert_eq!(
            *original, expected as f32,
            "each record carries its original primitive index"
        );
    }
}

#[test]
fn encode_gpu_rejects_non_finite_and_overflow() {
    let broken: Vec<Occluder> = vec![Occluder::sphere(
        Vector3D::new(f64::NAN, 0.0, 0.0),
        1.0,
        Material::lambert(Vector3D::zero()),
    )];
    assert!(
        RayTraceScene::new(broken).encode_gpu().is_err(),
        "a NaN centre must be rejected rather than packed"
    );
    let huge: Vec<Occluder> = vec![Occluder::sphere(
        Vector3D::new(f64::MAX, 0.0, 0.0),
        1.0,
        Material::lambert(Vector3D::zero()),
    )];
    assert!(
        RayTraceScene::new(huge).encode_gpu().is_err(),
        "a centre beyond the f32 range must be rejected rather than truncated"
    );
}

#[test]
fn encode_gpu_bounds_round_outward() {
    let awkward: f64 = 0.1_f64;
    let scene: RayTraceScene = RayTraceScene::new(vec![Occluder::aabb(
        Vector3D::new(-awkward, -awkward, -awkward),
        Vector3D::new(awkward, awkward, awkward),
        Material::lambert(Vector3D::zero()),
    )]);
    let packed: Vec<f32> = match scene.encode_gpu() {
        Ok(data) => data,
        Err(reason) => panic!("scene must pack: {reason}"),
    };
    let node_count: usize = packed[0] as usize;
    let offset: usize = 1;
    for node in 0..node_count {
        let lower: &[f32] = &packed[(offset + node * 3) * 4..(offset + node * 3) * 4 + 3];
        let upper: &[f32] = &packed[(offset + node * 3 + 1) * 4..(offset + node * 3 + 1) * 4 + 3];
        for axis in 0..3 {
            assert!(
                (lower[axis] as f64) <= -awkward,
                "packed lower bound {} must not exceed the source {}",
                lower[axis],
                -awkward
            );
            assert!(
                (upper[axis] as f64) >= awkward,
                "packed upper bound {} must not fall below the source {}",
                upper[axis],
                awkward
            );
        }
    }
}

#[test]
fn encode_gpu_sphere_bounds_still_enclose_the_source() {
    let center: Vector3D = Vector3D::new(0.1, -0.3, 0.7);
    let radius: f64 = 0.45;
    let scene: RayTraceScene = RayTraceScene::new(vec![Occluder::sphere(
        center,
        radius,
        Material::lambert(Vector3D::zero()),
    )]);
    let packed: Vec<f32> = match scene.encode_gpu() {
        Ok(data) => data,
        Err(reason) => panic!("scene must pack: {reason}"),
    };
    let offset: usize = packed[2] as usize;
    let cx: f64 = packed[offset * 4] as f64;
    let cy: f64 = packed[offset * 4 + 1] as f64;
    let cz: f64 = packed[offset * 4 + 2] as f64;
    let packed_radius: f64 = packed[(offset + 1) * 4] as f64;
    assert!(
        cx <= center.get_x() && cy <= center.get_y() && cz <= center.get_z(),
        "the packed centre must round toward -inf on every axis, got ({cx}, {cy}, {cz}) vs ({}, {}, {})",
        center.get_x(),
        center.get_y(),
        center.get_z()
    );
    let centre_drift: f64 = ((cx - center.get_x()).powi(2)
        + (cy - center.get_y()).powi(2)
        + (cz - center.get_z()).powi(2))
    .sqrt();
    assert!(
        packed_radius as f64 >= radius + centre_drift,
        "the widened radius must cover the centre's drift: packed {packed_radius}, source {radius}, drift {centre_drift}"
    );
    let slack: f64 = packed_radius as f64 - radius;
    assert!(
        slack >= 0.0 && slack <= radius * 1e-3 + 1e-6,
        "the radius widening must stay a rounding-scale correction, got {slack}"
    );
    let drift: f64 = ((cx - center.get_x()).powi(2)
        + (cy - center.get_y()).powi(2)
        + (cz - center.get_z()).powi(2))
    .sqrt();
    assert!(
        packed_radius >= radius + drift,
        "the packed radius {} must cover the source radius {} plus the centre drift {}",
        packed_radius,
        radius,
        drift
    );
}

#[test]
fn frame_header_layout_and_light_records() {
    let camera: Camera3D = camera();
    let lights: LightingUniforms = lit_scene();
    let settings: RayTraceSettings = RayTraceSettings::with_quality(2, 3);
    let mut packed: Vec<f32> = Vec::new();
    if let Err(reason) = pack_raytrace_frame(&mut packed, &camera, &lights, &settings, 32, 16) {
        panic!("frame must pack: {reason}");
    }
    assert_eq!(
        packed.len() % 4,
        0,
        "the packed frame is a whole number of vec4 slots"
    );
    let light_count: usize = packed[19] as usize;
    assert_eq!(
        packed.len() / 4,
        8 + light_count * 4,
        "header plus one record per light"
    );
    assert!(
        light_count > 0,
        "the fixture must declare at least one light"
    );
    assert!((packed[0] as f64 - camera.get_position().get_x()).abs() < 1e-6);
    assert!((packed[1] as f64 - camera.get_position().get_y()).abs() < 1e-6);
    assert!((packed[2] as f64 - camera.get_position().get_z()).abs() < 1e-6);
    assert!(
        (packed[3] as f64 - (camera.get_fov() * 0.5).tan()).abs() < 1e-6,
        "slot 0.w must carry tan(fov / 2), not the raw fov"
    );
    assert!(
        (packed[7] as f64 - (32.0 / 16.0)).abs() < 1e-6,
        "slot 1.w must carry width / height aspect"
    );
    assert_eq!(packed[11], 32.0, "slot 2.w carries width");
    assert_eq!(packed[15], 16.0, "slot 3.w carries height");
    assert_eq!(
        packed[19], light_count as f32,
        "slot 4.w carries light_count"
    );
    assert_eq!(
        packed[23],
        settings.get_max_bounces() as f32,
        "slot 5.w carries max_bounces"
    );
    assert!(
        (packed[24] as f64 - camera.get_near()).abs() < 1e-6,
        "slot 6.x carries near"
    );
    assert!(
        (packed[25] as f64 - camera.get_far()).abs() < 1e-6,
        "slot 6.y carries far"
    );
    assert_eq!(
        packed[26],
        settings.get_samples_per_axis() as f32,
        "slot 6.z carries samples_per_axis"
    );
    for light in 0..light_count {
        let base_word: usize = 32 + light * 16;
        assert_eq!(
            packed[base_word + 3],
            light as f32,
            "a light's kind code must match its LightType discriminant order"
        );
        assert!(
            packed[base_word + 13] == 0.0 && packed[base_word + 14] == 0.0,
            "only spot_cos occupies the final light slot's y and z"
        );
    }
}

#[test]
fn frame_rejects_invalid_camera_and_dimensions() {
    let lights: LightingUniforms = lit_scene();
    let settings: RayTraceSettings = RayTraceSettings::new_default();
    let mut packed: Vec<f32> = Vec::new();
    assert!(
        pack_raytrace_frame(&mut packed, &camera(), &lights, &settings, 0, 16).is_err(),
        "a zero width must be rejected"
    );
    assert!(
        pack_raytrace_frame(&mut packed, &camera(), &lights, &settings, 16, 0).is_err(),
        "a zero height must be rejected"
    );
    let mut degenerate: Camera3D = camera();
    degenerate.set_target(degenerate.get_position());
    assert!(
        pack_raytrace_frame(&mut packed, &degenerate, &lights, &settings, 16, 16).is_err(),
        "a zero forward vector must be rejected"
    );
    let mut parallel: Camera3D = camera();
    let forward: Vector3D = parallel.forward();
    parallel.set_up(forward);
    assert!(
        pack_raytrace_frame(&mut packed, &parallel, &lights, &settings, 16, 16).is_err(),
        "an up vector parallel to forward must be rejected"
    );
    let mut wide: Camera3D = camera();
    wide.set_fov(4.0);
    assert!(
        pack_raytrace_frame(&mut packed, &wide, &lights, &settings, 16, 16).is_err(),
        "an fov at or beyond PI must be rejected"
    );
    let mut inverted: Camera3D = camera();
    inverted.set_near(inverted.get_far());
    assert!(
        pack_raytrace_frame(&mut packed, &inverted, &lights, &settings, 16, 16).is_err(),
        "a near plane at or beyond the far plane must be rejected"
    );
    let mut negative: Camera3D = camera();
    negative.set_near(-1.0);
    assert!(
        pack_raytrace_frame(&mut packed, &negative, &lights, &settings, 16, 16).is_err(),
        "a negative near plane must be rejected"
    );
    let mut unbounded: Camera3D = camera();
    unbounded.set_far(f64::MAX);
    assert!(
        pack_raytrace_frame(&mut packed, &unbounded, &lights, &settings, 16, 16).is_err(),
        "a far plane beyond the f32 range must be rejected, not written as infinity"
    );
    let mut beyond: Camera3D = camera();
    beyond.set_position(Vector3D::new(f64::NAN, 0.0, 0.0));
    assert!(
        pack_raytrace_frame(&mut packed, &beyond, &lights, &settings, 16, 16).is_err(),
        "a non-finite eye must be rejected"
    );
}

#[test]
fn frame_reuses_its_buffer() {
    let lights: LightingUniforms = lit_scene();
    let settings: RayTraceSettings = RayTraceSettings::new_default();
    let mut packed: Vec<f32> = Vec::new();
    if let Err(reason) = pack_raytrace_frame(&mut packed, &camera(), &lights, &settings, 8, 8) {
        panic!("frame must pack: {reason}");
    }
    let capacity: usize = packed.capacity();
    if let Err(reason) = pack_raytrace_frame(&mut packed, &camera(), &lights, &settings, 8, 8) {
        panic!("frame must pack: {reason}");
    }
    assert_eq!(
        packed.capacity(),
        capacity,
        "repacking an unchanged frame must not reallocate"
    );
}

#[test]
fn render_rejects_invalid_dimensions_and_settings() {
    let scene: RayTraceScene = mixed_scene();
    let lights: LightingUniforms = lit_scene();
    let mut out: Vec<u8> = Vec::new();
    assert!(
        render_raytrace_rgba(
            &scene,
            &camera(),
            &lights,
            &RayTraceSettings::new_default(),
            0,
            8,
            &mut out
        )
        .is_err(),
        "a zero width must be rejected"
    );
    assert!(
        render_raytrace_rgba(
            &scene,
            &camera(),
            &lights,
            &RayTraceSettings::with_quality(9, 1),
            8,
            8,
            &mut out
        )
        .is_err(),
        "an invalid sample grid must be rejected before any pixel is traced"
    );
}

#[test]
fn render_empty_scene_is_ambient_everywhere() {
    let scene: RayTraceScene = RayTraceScene::new(Vec::new());
    let camera: Camera3D = camera();
    let mut lights: LightingUniforms = LightingUniforms::with_eye(camera.get_position());
    lights.set_ambient(Vector3D::new(0.2, 0.4, 0.6));
    let mut out: Vec<u8> = Vec::new();
    if let Err(reason) = render_raytrace_rgba(
        &scene,
        &camera,
        &lights,
        &RayTraceSettings::new_default(),
        4,
        3,
        &mut out,
    ) {
        panic!("empty scene must render: {reason}");
    }
    assert_eq!(out.len(), 4 * 3 * 4, "RGBA, top-down, one row per scanline");
    let expected: u8 = ((0.2f64).powf(1.0 / 2.2) * 255.0).round() as u8;
    let expected_green: u8 = ((0.4f64).powf(1.0 / 2.2) * 255.0).round() as u8;
    for pixel in 0..12 {
        assert_eq!(out[pixel * 4], expected, "red channel of pixel {pixel}");
        assert_eq!(out[pixel * 4 + 1], expected_green);
        assert_eq!(out[pixel * 4 + 3], 255, "alpha is always opaque");
    }
}

#[test]
fn render_reuses_its_output_buffer() {
    let scene: RayTraceScene = mixed_scene();
    let lights: LightingUniforms = lit_scene();
    let camera: Camera3D = camera();
    let settings: RayTraceSettings = RayTraceSettings::new_default();
    let mut out: Vec<u8> = Vec::new();
    for _ in 0..2 {
        if let Err(reason) =
            render_raytrace_rgba(&scene, &camera, &lights, &settings, 6, 5, &mut out)
        {
            panic!("frame must render: {reason}");
        }
    }
    assert_eq!(out.len(), 6 * 5 * 4);
    let capacity: usize = out.capacity();
    if let Err(reason) = render_raytrace_rgba(&scene, &camera, &lights, &settings, 6, 5, &mut out) {
        panic!("frame must render: {reason}");
    }
    assert_eq!(
        out.capacity(),
        capacity,
        "a steady-state frame must not reallocate the output buffer"
    );
}

#[test]
fn render_is_deterministic_across_repeated_frames() {
    let scene: RayTraceScene = mixed_scene();
    let lights: LightingUniforms = lit_scene();
    let camera: Camera3D = camera();
    let settings: RayTraceSettings = RayTraceSettings::with_quality(2, 2);
    let mut first: Vec<u8> = Vec::new();
    let mut second: Vec<u8> = Vec::new();
    if let Err(reason) = render_raytrace_rgba(&scene, &camera, &lights, &settings, 8, 6, &mut first)
    {
        panic!("frame must render: {reason}");
    }
    if let Err(reason) =
        render_raytrace_rgba(&scene, &camera, &lights, &settings, 8, 6, &mut second)
    {
        panic!("frame must render: {reason}");
    }
    assert_eq!(first, second, "the same inputs must produce the same frame");
}

#[test]
fn render_produces_distinct_pixels_for_a_mixed_scene() {
    let scene: RayTraceScene = mixed_scene();
    let lights: LightingUniforms = lit_scene();
    let mut out: Vec<u8> = Vec::new();
    if let Err(reason) = render_raytrace_rgba(
        &scene,
        &camera(),
        &lights,
        &RayTraceSettings::new_default(),
        24,
        18,
        &mut out,
    ) {
        panic!("frame must render: {reason}");
    }
    let mut distinct: HashSet<(u8, u8, u8)> = HashSet::new();
    for pixel in 0..24 * 18 {
        distinct.insert((out[pixel * 4], out[pixel * 4 + 1], out[pixel * 4 + 2]));
    }
    assert!(
        distinct.len() > 4,
        "a mixed sphere/box/triangle scene must not render flat, got {} distinct colours",
        distinct.len()
    );
    let lit: usize = out
        .as_chunks::<4>()
        .0
        .iter()
        .filter(|pixel: &&[u8; 4]| {
            u32::from(pixel[0]) + u32::from(pixel[1]) + u32::from(pixel[2]) > 60
        })
        .count();
    assert!(
        lit > 4,
        "geometry must be visibly lit, got {lit} lit pixels"
    );
}

#[test]
fn render_matches_per_pixel_scene_trace() {
    let scene: RayTraceScene = mixed_scene();
    let camera: Camera3D = camera();
    let lights: LightingUniforms = lit_scene();
    let settings: RayTraceSettings = RayTraceSettings::new_default();
    let width: u32 = 16;
    let height: u32 = 12;
    let mut out: Vec<u8> = Vec::new();
    if let Err(reason) =
        render_raytrace_rgba(&scene, &camera, &lights, &settings, width, height, &mut out)
    {
        panic!("frame must render: {reason}");
    }
    let mut packed: Vec<f32> = Vec::new();
    if let Err(reason) =
        pack_raytrace_frame(&mut packed, &camera, &lights, &settings, width, height)
    {
        panic!("frame must pack: {reason}");
    }
    let eye: &[f32] = &packed[0..4];
    let projection: &[f32] = &packed[4..8];
    let right: &[f32] = &packed[8..12];
    let up: &[f32] = &packed[12..16];
    let tan_half: f64 = eye[3] as f64;
    let aspect: f64 = projection[3] as f64;
    let width_f64: f64 = right[3] as f64;
    let height_f64: f64 = up[3] as f64;
    let eye_vector: Vector3D = Vector3D::new(eye[0] as f64, eye[1] as f64, eye[2] as f64);
    let forward: Vector3D = Vector3D::new(
        projection[0] as f64,
        projection[1] as f64,
        projection[2] as f64,
    );
    let right_vector: Vector3D = Vector3D::new(right[0] as f64, right[1] as f64, right[2] as f64);
    let up_vector: Vector3D = Vector3D::new(up[0] as f64, up[1] as f64, up[2] as f64);
    for y in 0..height {
        for x in 0..width {
            let ndc_x: f64 = ((x as f64 + 0.5) / width_f64) * 2.0 - 1.0;
            let ndc_y: f64 = 1.0 - ((y as f64 + 0.5) / height_f64) * 2.0;
            let direction: Vector3D = (forward
                + right_vector.scaled(ndc_x * aspect * tan_half)
                + up_vector.scaled(ndc_y * tan_half))
            .normalized();
            let ray: Ray =
                Ray::new(eye_vector, direction).with_range(camera.get_near(), camera.get_far());
            let linear: Vector3D = scene.trace_with_bounces(ray, &lights, 4);
            let expected: (u8, u8, u8) = (
                ((linear.get_x().clamp(0.0, 1.0)).powf(1.0 / 2.2) * 255.0).round() as u8,
                ((linear.get_y().clamp(0.0, 1.0)).powf(1.0 / 2.2) * 255.0).round() as u8,
                ((linear.get_z().clamp(0.0, 1.0)).powf(1.0 / 2.2) * 255.0).round() as u8,
            );
            let offset: usize = ((y as usize) * (width as usize) + x as usize) * 4;
            let actual: (u8, u8, u8) = (out[offset], out[offset + 1], out[offset + 2]);
            assert_eq!(
                actual, expected,
                "pixel ({x}, {y}) must equal the scene trace it delegates to"
            );
        }
    }
}

#[test]
fn supersampling_refines_the_image() {
    let scene: RayTraceScene = mixed_scene();
    let lights: LightingUniforms = lit_scene();
    let camera: Camera3D = camera();
    let mut coarse: Vec<u8> = Vec::new();
    let mut fine: Vec<u8> = Vec::new();
    if let Err(reason) = render_raytrace_rgba(
        &scene,
        &camera,
        &lights,
        &RayTraceSettings::with_quality(1, 2),
        20,
        16,
        &mut coarse,
    ) {
        panic!("frame must render: {reason}");
    }
    if let Err(reason) = render_raytrace_rgba(
        &scene,
        &camera,
        &lights,
        &RayTraceSettings::with_quality(4, 2),
        20,
        16,
        &mut fine,
    ) {
        panic!("frame must render: {reason}");
    }
    let mut changed: usize = 0;
    for pixel in 0..coarse.len() / 4 {
        if coarse[pixel * 4] != fine[pixel * 4] {
            changed += 1;
        }
    }
    assert!(
        changed > 5,
        "a 4x sample grid must refine edges, changed only {changed} pixels"
    );
}

#[test]
fn zero_bounces_still_shades_the_primary_hit() {
    let scene: RayTraceScene = mixed_scene();
    let camera: Camera3D = camera();
    let mut lights: LightingUniforms = LightingUniforms::with_eye(camera.get_position());
    lights.set_ambient(Vector3D::zero());
    let emissive: RayTraceScene = RayTraceScene::new(vec![Occluder::sphere(
        Vector3D::zero(),
        1.2,
        Material::emissive(Vector3D::new(1.0, 0.5, 0.25)),
    )]);
    let mut out: Vec<u8> = Vec::new();
    if let Err(reason) = render_raytrace_rgba(
        &emissive,
        &camera,
        &lights,
        &RayTraceSettings::with_quality(1, 0),
        16,
        16,
        &mut out,
    ) {
        panic!("frame must render: {reason}");
    }
    let center: usize = (8 * 16 + 8) * 4;
    assert!(
        out[center] > 200 && out[center + 1] > 150 && out[center + 3] == 255,
        "zero bounces still shades the primary hit, got {:?}",
        (out[center], out[center + 1], out[center + 2])
    );
    let mut lit_scene_frame: Vec<u8> = Vec::new();
    if let Err(reason) = render_raytrace_rgba(
        &scene,
        &camera,
        &lights,
        &RayTraceSettings::with_quality(1, 0),
        16,
        16,
        &mut lit_scene_frame,
    ) {
        panic!("frame must render: {reason}");
    }
    assert!(
        lit_scene_frame.iter().any(|byte: &u8| *byte > 0),
        "a mixed scene must not render black at zero bounces"
    );
}

#[test]
fn shader_sources_are_data_driven() {
    for source in [raytrace_glsl_fragment(), raytrace_wgsl()] {
        for banned in [
            "GROUND_MIN",
            "MIRROR_CENTER",
            "EMISSIVE_CENTER",
            "SUN_RADIUS",
            "SHADE_EYE",
        ] {
            assert!(
                !source.contains(banned),
                "shader must not bake scene geometry: found {banned}"
            );
        }
        assert!(
            source.contains("samples"),
            "the sample grid must arrive through the frame data"
        );
        assert!(
            source.contains("max_bounces"),
            "the bounce limit must arrive through the frame data"
        );
    }
    assert!(raytrace_glsl_vertex().contains("gl_VertexID"));
    assert!(raytrace_glsl_fragment().contains("texelFetch"));
    assert!(raytrace_wgsl().contains("@fragment"));
    assert!(raytrace_wgsl().contains("@vertex"));
}

#[test]
fn shader_sources_apply_the_camera_fov_and_phong_reflection() {
    for source in [raytrace_glsl_fragment(), raytrace_wgsl()] {
        assert!(
            source.contains("tan_half"),
            "the camera ray must multiply by tan(fov / 2)"
        );
        assert!(
            source.contains("normal * (2.0 * dot(normal, light_dir)) - light_dir"),
            "the Phong lobe must compute reflect(-L, N) = 2*dot(N,L)*N - L"
        );
        assert!(
            !source.contains("light_dir - normal * (2.0 * dot"),
            "reflect(-L, N) must not be spelled with the operands swapped"
        );
        assert!(
            !source.contains("2.0 * dot(light_dir, normal)"),
            "the dot must be taken as dot(N, L), not dot(L, N)"
        );
        assert!(
            !source.contains("normalize(light_dir + view_dir"),
            "the Blinn half-vector form must not appear in the Phong lobe"
        );
        assert!(
            !source.contains("normalize(light_dir + view)"),
            "the Blinn half-vector form must not appear in the Phong lobe"
        );
        assert!(
            source.contains("shadow_distance = distance"),
            "a point or spot shadow ray must reach the actual lamp distance"
        );
    }
}

#[test]
fn shader_row_order_matches_the_cpu_reference() {
    let glsl: &str = raytrace_glsl_fragment();
    let wgsl: &str = raytrace_wgsl();
    let height: f64 = 64.0;
    let ndc_y_top_down: f64 = 1.0 - (8.0 / height) * 2.0;
    let ndc_y_bottom_up: f64 = (8.0 / height) * 2.0 - 1.0;
    assert!(
        (ndc_y_top_down - 0.75).abs() < 1e-12,
        "the CPU reference maps the top row to +1"
    );
    assert!(
        (ndc_y_bottom_up + 0.75).abs() < 1e-12,
        "gl_FragCoord is bottom-up, so row 8 of 64 maps to -0.75"
    );
    assert!(
        (ndc_y_top_down - ndc_y_bottom_up).abs() > 1.0,
        "the two row conventions must disagree in sign, or the flip is untestable"
    );
    assert!(
        glsl.contains("(py / resolution.y) * 2.0 - 1.0"),
        "the GLSL fragment shader must map its bottom-up gl_FragCoord onto the same top-down ndc_y the CPU reference uses"
    );
    assert!(
        !glsl.contains("1.0 - (py / resolution.y) * 2.0"),
        "negating the bottom-up gl_FragCoord flips the image vertically"
    );
    assert!(
        wgsl.contains("1.0 - (py / resolution.y) * 2.0"),
        "the WGSL fragment position is already top-down and must not be inverted again"
    );
}

#[test]
fn shader_sources_share_the_packed_layout() {
    for source in [raytrace_glsl_fragment(), raytrace_wgsl()] {
        assert!(source.contains("SCENE_SLOTS"));
        assert!(source.contains("NODE_SLOTS"));
        assert!(source.contains("PRIMITIVE_SLOTS"));
        assert!(source.contains("FRAME_HEADER_SLOTS"));
        assert!(source.contains("LIGHT_SLOTS"));
        assert!(
            source.contains("surface_offset") || source.contains("SURFACE_OFFSET"),
            "the shadow ray must leave the shaded surface along its normal"
        );
        assert!(
            source.contains("arrayLength") || source.contains("textureSize"),
            "buffer and texture reads must be bounds-aware"
        );
    }
}
