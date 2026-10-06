use super::*;

#[test]
fn texture_2d_descriptor_default_for_pins_mip_and_sample_counts_to_one() {
    let descriptor: Texture2DDescriptor =
        Texture2DDescriptor::default_for(64, 32, GpuTextureFormat::Rgba8Unorm);
    assert_eq!(descriptor.get_width(), 64);
    assert_eq!(descriptor.get_height(), 32);
    assert_eq!(descriptor.get_format(), GpuTextureFormat::Rgba8Unorm);
    assert_eq!(descriptor.get_mip_level_count(), 1);
    assert_eq!(descriptor.get_sample_count(), 1);
}

#[test]
fn texture_view_descriptor_mip_selects_exactly_one_level_at_that_index() {
    let descriptor: TextureViewDescriptor = TextureViewDescriptor::mip(3);
    assert_eq!(descriptor.get_base_mip_level(), 3);
    assert_eq!(descriptor.get_mip_level_count(), 1);
    assert_eq!(descriptor.get_base_array_layer(), 0);
    assert_eq!(descriptor.get_array_layer_count(), 0);
    assert_eq!(descriptor.get_aspect(), None);
    assert_eq!(descriptor.get_format(), None);
    assert_eq!(descriptor.get_dimension(), None);
}

#[test]
fn texture_view_descriptor_depth_only_selects_the_depth_aspect() {
    let descriptor: TextureViewDescriptor = TextureViewDescriptor::depth_only();
    assert_eq!(descriptor.get_aspect(), Some("depth-only"));
    assert_eq!(descriptor.get_mip_level_count(), 0);
    assert_eq!(descriptor.get_base_mip_level(), 0);
}

#[test]
fn texture_write_descriptor_for_2d_leaves_the_three_dimensional_fields_unset() {
    let data: Vec<u8> = vec![1, 2, 3, 4];
    let descriptor: TextureWriteDescriptor =
        TextureWriteDescriptor::for_2d(data.clone(), 256, JsValue::NULL);
    assert_eq!(descriptor.get_data(), data);
    assert_eq!(descriptor.get_bytes_per_row(), 256);
    assert_eq!(descriptor.get_rows_per_image(), 0);
    assert_eq!(descriptor.get_mip_level(), 0);
    assert_eq!(descriptor.get_origin(), None);
    assert!(!descriptor.get_flip_y());
}

#[test]
fn bind_group_entry_binding_reads_the_slot_number_of_every_variant() {
    let buffer: BindGroupEntry = BindGroupEntry::Buffer {
        binding: 0,
        buffer: JsValue::NULL,
        offset: 0,
        size: None,
    };
    let texture: BindGroupEntry = BindGroupEntry::Texture {
        binding: 1,
        view: JsValue::NULL,
    };
    let storage: BindGroupEntry = BindGroupEntry::StorageTexture {
        binding: 2,
        view: JsValue::NULL,
        read_only: true,
    };
    let sampler: BindGroupEntry = BindGroupEntry::Sampler {
        binding: 3,
        sampler: JsValue::NULL,
    };
    assert_eq!(buffer.binding(), 0);
    assert_eq!(texture.binding(), 1);
    assert_eq!(storage.binding(), 2);
    assert_eq!(sampler.binding(), 3);
}

#[test]
fn bind_group_layout_entry_uniform_is_a_uniform_buffer_slot() {
    let entry: BindGroupLayoutEntry = BindGroupLayoutEntry::uniform(4, ShaderStage::Vertex);
    assert_eq!(entry.binding, 4);
    assert_eq!(entry.visibility, ShaderStage::Vertex);
    assert!(matches!(entry.ty, BindGroupEntryType::UniformBuffer));
}

#[test]
fn bind_group_layout_entry_read_only_storage_differs_from_writable_storage() {
    let read_only: BindGroupLayoutEntry =
        BindGroupLayoutEntry::storage(1, ShaderStage::Compute, true);
    let writable: BindGroupLayoutEntry =
        BindGroupLayoutEntry::storage(1, ShaderStage::Compute, false);
    assert!(matches!(
        read_only.ty,
        BindGroupEntryType::StorageBuffer { read_only: true }
    ));
    assert!(matches!(
        writable.ty,
        BindGroupEntryType::StorageBuffer { read_only: false }
    ));
}

#[test]
fn bind_group_layout_entry_texture_is_single_sampled_and_multisampled_is_not() {
    let single: BindGroupLayoutEntry =
        BindGroupLayoutEntry::texture(2, ShaderStage::Fragment, "float");
    let multi: BindGroupLayoutEntry =
        BindGroupLayoutEntry::texture_multisampled(2, ShaderStage::Fragment, "float");
    assert!(matches!(
        single.ty,
        BindGroupEntryType::SampledTexture {
            sample_type,
            multisampled: false
        } if sample_type == "float"
    ));
    assert!(matches!(
        multi.ty,
        BindGroupEntryType::SampledTexture {
            multisampled: true,
            ..
        }
    ));
}

#[test]
fn bind_group_layout_entry_storage_texture_carries_its_format_and_access() {
    let entry: BindGroupLayoutEntry =
        BindGroupLayoutEntry::storage_texture(5, ShaderStage::Compute, "r32float", false);
    assert!(matches!(
        entry.ty,
        BindGroupEntryType::StorageTexture {
            read_only: false,
            ref format
        } if format == "r32float"
    ));
}

#[test]
fn the_three_sampler_constructors_differ_only_in_filtering_and_comparison() {
    let filtering: BindGroupLayoutEntry = BindGroupLayoutEntry::sampler(0, ShaderStage::Fragment);
    let point: BindGroupLayoutEntry =
        BindGroupLayoutEntry::sampler_non_filtering(0, ShaderStage::Fragment);
    let comparing: BindGroupLayoutEntry =
        BindGroupLayoutEntry::sampler_comparison(0, ShaderStage::Fragment);
    assert!(matches!(
        filtering.ty,
        BindGroupEntryType::Sampler {
            filtering: true,
            comparison: false
        }
    ));
    assert!(matches!(
        point.ty,
        BindGroupEntryType::Sampler {
            filtering: false,
            comparison: false
        }
    ));
    assert!(matches!(
        comparing.ty,
        BindGroupEntryType::Sampler {
            filtering: false,
            comparison: true
        }
    ));
}

#[test]
fn color_target_state_for_format_writes_every_channel_without_blending() {
    let state: ColorTargetState = ColorTargetState::for_format(GpuTextureFormat::Bgra8Unorm);
    assert_eq!(state.get_format(), GpuTextureFormat::Bgra8Unorm);
    assert!(state.get_blend().is_none(), "an unblended target must not carry a blend state");
    assert_eq!(state.get_write_mask(), 0xf);
}

#[test]
fn multisample_state_with_sample_count_writes_every_sample() {
    let state: MultisampleState = MultisampleState::with_sample_count(4);
    assert_eq!(state.get_count(), 4);
    assert_eq!(state.get_mask(), 0xf);
}

#[test]
fn depth_stencil_state_writing_depth_keeps_the_nearer_fragment() {
    let state: DepthStencilState =
        DepthStencilState::writing_depth(GpuTextureFormat::Depth24Plus);
    assert_eq!(state.get_format(), GpuTextureFormat::Depth24Plus);
    assert!(state.get_depth_write_enabled());
    assert_eq!(state.get_depth_compare(), CompareFunction::Less);
}

#[test]
fn blend_component_source_alpha_over_adds_alpha_scaled_source_and_destination() {
    let component: BlendComponent = BlendComponent::source_alpha_over();
    assert_eq!(component.get_operation(), BlendOperation::Add);
    assert_eq!(component.get_source(), BlendFactor::SourceAlpha);
    assert_eq!(component.get_destination(), BlendFactor::OneMinusSourceAlpha);
}

#[test]
fn blend_state_alpha_over_uses_one_component_for_color_and_alpha() {
    let state: BlendState = BlendState::alpha_over();
    assert_eq!(state.get_color().get_operation(), BlendOperation::Add);
    assert_eq!(
        state.get_alpha().get_source(),
        BlendFactor::SourceAlpha,
        "alpha channel must be scaled by the same source alpha as color"
    );
    assert_eq!(
        state.get_alpha().get_destination(),
        BlendFactor::OneMinusSourceAlpha
    );
}

#[test]
fn sampler_descriptor_nearest_clamp_is_nearest_and_clamped_on_all_three_axes() {
    let sampler: SamplerDescriptor = SamplerDescriptor::nearest_clamp();
    assert_eq!(sampler.get_filter(), FilterMode::Nearest);
    assert_eq!(sampler.get_mipmap_filter(), MipmapFilter::Nearest);
    assert_eq!(sampler.get_address_mode_u(), AddressMode::ClampToEdge);
    assert_eq!(sampler.get_address_mode_v(), AddressMode::ClampToEdge);
    assert_eq!(sampler.get_address_mode_w(), AddressMode::ClampToEdge);
    assert_eq!(sampler.get_compare(), None);
}

#[test]
fn draw_args_whole_stream_starts_at_the_first_vertex_of_the_first_instance() {
    let args: DrawArgs = DrawArgs::whole_stream(36, 2);
    assert_eq!(args.get_vertex_count(), 36);
    assert_eq!(args.get_instance_count(), 2);
    assert_eq!(args.get_first_vertex(), 0);
    assert_eq!(args.get_first_instance(), 0);
}

#[test]
fn draw_indexed_args_whole_buffer_starts_at_index_zero_with_no_base_vertex() {
    let args: DrawIndexedArgs = DrawIndexedArgs::whole_buffer(6, 1);
    assert_eq!(args.get_index_count(), 6);
    assert_eq!(args.get_instance_count(), 1);
    assert_eq!(args.get_first_index(), 0);
    assert_eq!(args.get_base_vertex(), 0);
    assert_eq!(args.get_first_instance(), 0);
}

#[test]
fn dispatch_args_grid_keeps_all_three_workgroup_axes() {
    let args: DispatchArgs = DispatchArgs::grid(8, 4, 2);
    assert_eq!(args.get_x(), 8);
    assert_eq!(args.get_y(), 4);
    assert_eq!(args.get_z(), 2);
}

#[test]
fn blend_mode_to_css_returns_the_canvas_composite_operation_name() {
    let expected: [(BlendMode, &str); 17] = [
        (BlendMode::Normal, "source-over"),
        (BlendMode::Multiply, "multiply"),
        (BlendMode::Screen, "screen"),
        (BlendMode::Lighter, "lighter"),
        (BlendMode::Overlay, "overlay"),
        (BlendMode::Darken, "darken"),
        (BlendMode::Lighten, "lighten"),
        (BlendMode::ColorDodge, "color-dodge"),
        (BlendMode::ColorBurn, "color-burn"),
        (BlendMode::HardLight, "hard-light"),
        (BlendMode::SoftLight, "soft-light"),
        (BlendMode::Difference, "difference"),
        (BlendMode::Exclusion, "exclusion"),
        (BlendMode::Hue, "hue"),
        (BlendMode::Saturation, "saturation"),
        (BlendMode::Color, "color"),
        (BlendMode::Luminosity, "luminosity"),
    ];
    for (mode, css) in expected {
        assert_eq!(mode.to_css(), css, "wrong css name for {mode:?}");
    }
}

#[test]
fn vertex_step_mode_as_str_is_the_webgpu_step_mode_name() {
    assert_eq!(VertexStepMode::Vertex.as_str(), "vertex");
    assert_eq!(VertexStepMode::Instance.as_str(), "instance");
}

#[test]
fn buffer_usage_bit_matches_the_webgpu_buffer_usage_table() {
    let expected: [(BufferUsage, u32); 10] = [
        (BufferUsage::MapRead, 1),
        (BufferUsage::MapWrite, 2),
        (BufferUsage::CopySource, 4),
        (BufferUsage::CopyDestination, 8),
        (BufferUsage::Index, 16),
        (BufferUsage::Vertex, 32),
        (BufferUsage::Uniform, 64),
        (BufferUsage::Storage, 128),
        (BufferUsage::Indirect, 256),
        (BufferUsage::QueryResolve, 512),
    ];
    for (usage, bit) in expected {
        assert_eq!(usage.usage_bit(), bit, "wrong bit for {usage:?}");
    }
}

#[test]
fn texture_usage_bit_matches_the_webgpu_texture_usage_table() {
    let expected: [(TextureUsage, u32); 5] = [
        (TextureUsage::CopySource, 1),
        (TextureUsage::CopyDestination, 2),
        (TextureUsage::TextureBinding, 4),
        (TextureUsage::StorageBinding, 8),
        (TextureUsage::RenderAttachment, 16),
    ];
    for (usage, bit) in expected {
        assert_eq!(usage.usage_bit(), bit, "wrong bit for {usage:?}");
    }
}

#[test]
fn buffer_and_texture_families_share_variant_names_but_not_bit_values() {
    assert_ne!(
        BufferUsage::CopySource.usage_bit(),
        TextureUsage::CopySource.usage_bit(),
        "CopySource is 4 for a buffer and 1 for a texture"
    );
    assert_eq!(BufferUsage::Vertex.usage_bit(), 32);
    assert_eq!(TextureUsage::RenderAttachment.usage_bit(), 16);
}

#[test]
fn shader_stage_bit_matches_the_webgpu_shader_stage_table() {
    let expected: [(ShaderStage, u32); 3] = [
        (ShaderStage::Vertex, 1),
        (ShaderStage::Fragment, 2),
        (ShaderStage::Compute, 4),
    ];
    for (stage, bit) in expected {
        assert_eq!(stage.stage_bit(), bit, "wrong bit for {stage:?}");
    }
}

#[test]
fn bit_or_ors_the_bits_of_both_operands_in_every_bitmask_family() {
    assert_eq!(
        BufferUsage::Vertex | BufferUsage::CopyDestination,
        32 | 8,
        "a buffer that is both copied into and drawn from needs both bits"
    );
    assert_eq!(
        TextureUsage::RenderAttachment | TextureUsage::TextureBinding,
        16 | 4
    );
    assert_eq!(
        ShaderStage::Vertex | ShaderStage::Fragment,
        1 | 2,
        "a bind group visible to both stages needs both bits"
    );
}

#[test]
fn bit_or_yields_a_plain_mask_so_a_third_use_is_added_through_usage_bit() {
    let two: u32 = BufferUsage::Vertex | BufferUsage::Index;
    assert_eq!(two, 48);
    let three: u32 = two | BufferUsage::CopySource.usage_bit();
    assert_eq!(three, 52);
    assert_eq!(three, 32 | 16 | 4);
}

fn assert_vector_close(actual: Vector2D, expected_x: f64, expected_y: f64, what: &str) {
    let epsilon: f64 = 1e-9;
    assert!(
        (actual.get_x() - expected_x).abs() < epsilon
            && (actual.get_y() - expected_y).abs() < epsilon,
        "{what}: expected ({expected_x}, {expected_y}), got ({}, {})",
        actual.get_x(),
        actual.get_y()
    );
}

#[test]
fn camera_2d_create_starts_at_the_origin_with_unit_zoom_and_no_rotation() {
    let camera: Camera2D = Camera2D::create(800.0, 600.0);
    assert_vector_close(camera.get_position(), 0.0, 0.0, "position");
    assert_eq!(camera.get_zoom(), 1.0);
    assert_eq!(camera.get_rotation(), 0.0);
    assert_eq!(camera.get_viewport_width(), 800.0);
    assert_eq!(camera.get_viewport_height(), 600.0);
}

#[test]
fn camera_2d_default_is_an_800_by_600_viewport() {
    let camera: Camera2D = Camera2D::default();
    assert_eq!(camera.get_viewport_width(), 800.0);
    assert_eq!(camera.get_viewport_height(), 600.0);
    assert_eq!(camera.get_zoom(), 1.0);
}

#[test]
fn world_to_screen_puts_the_camera_position_at_the_viewport_center() {
    let camera: Camera2D = Camera2D::create(800.0, 600.0);
    assert_vector_close(camera.world_to_screen(Vector2D::zero()), 400.0, 300.0, "origin");
    assert_vector_close(
        camera.world_to_screen(Vector2D::new(100.0, 50.0)),
        500.0,
        350.0,
        "offset point",
    );
}

#[test]
fn screen_to_world_undoes_world_to_screen() {
    let mut camera: Camera2D = Camera2D::create(800.0, 600.0);
    camera.set_position(Vector2D::new(30.0, -20.0));
    camera.set_zoom(2.5);
    camera.set_rotation(0.4);
    for point in [
        Vector2D::zero(),
        Vector2D::new(100.0, 50.0),
        Vector2D::new(-75.0, 220.0),
    ] {
        let round_trip: Vector2D = camera.screen_to_world(camera.world_to_screen(point));
        assert_vector_close(round_trip, point.get_x(), point.get_y(), "round trip");
    }
}

#[test]
fn translate_moves_the_camera_so_world_space_slides_the_other_way() {
    let mut camera: Camera2D = Camera2D::create(800.0, 600.0);
    camera.translate(Vector2D::new(10.0, 25.0));
    assert_vector_close(camera.get_position(), 10.0, 25.0, "translated position");
    assert_vector_close(
        camera.world_to_screen(Vector2D::zero()),
        390.0,
        275.0,
        "origin seen from a moved camera",
    );
}

#[test]
fn zoom_by_scales_screen_space_about_the_viewport_center() {
    let mut camera: Camera2D = Camera2D::create(800.0, 600.0);
    camera.zoom_by(2.0);
    assert_eq!(camera.get_zoom(), 2.0);
    assert_vector_close(
        camera.world_to_screen(Vector2D::new(100.0, 0.0)),
        600.0,
        300.0,
        "doubled zoom",
    );
}

#[test]
fn zoom_by_clamps_at_epsilon_instead_of_reaching_zero() {
    let mut camera: Camera2D = Camera2D::create(800.0, 600.0);
    camera.zoom_by(0.0);
    assert!(
        camera.get_zoom() > 0.0 && camera.get_zoom().is_finite(),
        "zoom must stay positive and finite, got {}",
        camera.get_zoom()
    );
    let screen: Vector2D = camera.world_to_screen(Vector2D::new(1.0, 1.0));
    assert!(
        screen.get_x().is_finite() && screen.get_y().is_finite(),
        "the inverse in screen_to_world would divide by zero at a zero zoom"
    );
}

#[test]
fn world_to_screen_rotates_by_the_negated_camera_rotation() {
    let mut camera: Camera2D = Camera2D::create(800.0, 600.0);
    camera.set_rotation(consts::FRAC_PI_2);
    assert_vector_close(
        camera.world_to_screen(Vector2D::new(1.0, 0.0)),
        400.0,
        299.0,
        "a quarter turn sends +x to -y on screen",
    );
}

#[test]
fn font_builds_a_css_shorthand_from_size_and_family() {
    assert_eq!(CanvasRenderer::font(16.0, "sans-serif"), "16px sans-serif");
    assert_eq!(
        CanvasRenderer::font(12.5, String::from("monospace")),
        "12.5px monospace",
        "the family is generic over anything that is AsRef<str>"
    );
}

#[test]
fn default_font_is_sixteen_px_sans_serif() {
    assert_eq!(CanvasRenderer::default_font(), "16px sans-serif");
}

#[test]
fn to_css_rounds_channels_to_bytes_and_keeps_alpha_as_a_fraction() {
    assert_eq!(
        Color::to_css(&Color::new(1.0, 0.5, 0.0, 1.0)),
        "rgba(255, 128, 0, 1)"
    );
    assert_eq!(
        Color::to_css(&Color::new(0.0, 0.0, 0.0, 0.5)),
        "rgba(0, 0, 0, 0.5)"
    );
    assert_eq!(
        Color::to_css(&Color::new(0.0, 0.0, 0.0, 1.0)),
        "rgba(0, 0, 0, 1)",
        "opaque black is the documented default"
    );
}

fn three_d_camera() -> Camera3D {
    Camera3D::create(Vector3D::new(0.0, 0.0, 5.0), Vector3D::zero(), 800.0, 600.0)
}

fn eye_distance(camera: &Camera3D) -> f64 {
    (camera.get_position() - camera.get_target()).magnitude()
}

#[test]
fn a_three_dimensional_camera_starts_at_the_default_clip_planes() {
    let camera: Camera3D = three_d_camera();
    assert_eq!(camera.get_position().get_z(), 5.0);
    assert_eq!(camera.get_target().get_z(), 0.0);
    assert_eq!(camera.get_fov(), DEFAULT_CAMERA_FOV);
    assert_eq!(camera.get_near(), DEFAULT_CAMERA_NEAR);
    assert_eq!(camera.get_far(), DEFAULT_CAMERA_FAR);
    assert_eq!(camera.get_up().get_y(), 1.0, "the up vector is world up");
}

#[test]
fn a_camera_aspect_is_width_over_height() {
    assert_eq!(three_d_camera().aspect(), 800.0 / 600.0);
}

#[test]
fn a_camera_with_a_degenerate_height_reports_a_neutral_aspect_instead_of_dividing_by_zero() {
    let camera: Camera3D = Camera3D::create(Vector3D::zero(), Vector3D::zero(), 800.0, 0.0);
    assert_eq!(camera.aspect(), 1.0, "a zero height must not yield an infinity");
    let tiny: Camera3D = Camera3D::create(Vector3D::zero(), Vector3D::zero(), 800.0, 1e-12);
    assert_eq!(tiny.aspect(), 1.0, "the same guard covers a near-zero height");
    assert!(three_d_camera().aspect().is_finite());
}

#[test]
fn a_camera_forward_vector_points_from_the_eye_to_the_target() {
    let camera: Camera3D = three_d_camera();
    let forward: Vector3D = camera.forward();
    assert!(
        (forward.get_z() + 1.0).abs() < 1e-9,
        "the eye sits at +z looking down -z, got {}",
        forward.get_z()
    );
    assert_eq!(forward.magnitude(), 1.0, "the direction is normalized");
}

#[test]
fn a_camera_projects_its_own_target_onto_the_centre_of_the_viewport() {
    let camera: Camera3D = three_d_camera();
    let screen: Vector3D = camera.world_to_screen(camera.get_target());
    assert!(
        (screen.get_x() - 400.0).abs() < 1e-6,
        "the target is on the view axis, so it lands at the horizontal centre, got {}",
        screen.get_x()
    );
    assert!(
        (screen.get_y() - 300.0).abs() < 1e-6,
        "and at the vertical centre, got {}",
        screen.get_y()
    );
}

#[test]
fn a_camera_keeps_its_target_in_frustum_and_drops_a_point_behind_the_eye() {
    let camera: Camera3D = three_d_camera();
    assert!(camera.in_frustum(camera.get_target()), "the target is by definition visible");
    assert!(
        !camera.in_frustum(Vector3D::new(0.0, 0.0, 50.0)),
        "a point far behind the eye is outside"
    );
    assert!(
        !camera.in_frustum(Vector3D::new(0.0, 0.0, -5000.0)),
        "a point far beyond the far plane is outside"
    );
}

#[test]
fn translating_a_three_dimensional_camera_moves_eye_and_target_together() {
    let mut camera: Camera3D = three_d_camera();
    let before: Matrix4x4 = camera.view_matrix();
    camera.translate(Vector3D::new(1.0, 2.0, 0.0));
    assert_eq!(camera.get_position().get_x(), 1.0);
    assert_eq!(camera.get_target().get_x(), 1.0, "the target rides along");
    assert_eq!(camera.get_target().get_y(), 2.0);
    assert_eq!(camera.get_position().get_z(), 5.0, "only the offset axes moved");
    assert_ne!(before, camera.view_matrix(), "a moved camera has a different view");
}

#[test]
fn zooming_a_three_dimensional_camera_pulls_the_eye_towards_a_fixed_target() {
    let mut camera: Camera3D = three_d_camera();
    let before: f64 = eye_distance(&camera);
    camera.zoom(1.0);
    let after: f64 = eye_distance(&camera);
    assert!(
        (before - after - 1.0).abs() < 1e-9,
        "zoom moves one unit along the forward axis, {before} -> {after}"
    );
    assert_eq!(camera.get_target().get_z(), 0.0, "the target never moves");
    camera.zoom(-1.0);
    assert!(
        (eye_distance(&camera) - before).abs() < 1e-9,
        "a negative zoom walks it back out again"
    );
}

#[test]
fn orbiting_a_three_dimensional_camera_keeps_the_distance_to_its_target() {
    let mut camera: Camera3D = three_d_camera();
    let before: f64 = eye_distance(&camera);
    camera.orbit(0.7, 0.3);
    assert!(
        (eye_distance(&camera) - before).abs() < 1e-9,
        "orbit is a rotation about the target, not a zoom"
    );
    assert_eq!(camera.get_target().get_z(), 0.0, "the target is the pivot");
    let eye: Vector3D = camera.get_position();
    assert!(
        (eye.get_x()).abs() > 1e-6,
        "a non-zero yaw must actually move the eye sideways"
    );
}

#[test]
fn a_linear_gradient_keeps_its_endpoints_and_its_stops_in_order() {
    let stops: Vec<(f64, String)> = vec![
        (0.0, String::from("#000000")),
        (0.5, String::from("#808080")),
        (1.0, String::from("#ffffff")),
    ];
    let gradient: LinearGradient =
        LinearGradient::create(Vector2D::zero(), Vector2D::new(0.0, 100.0), stops.clone());
    assert_eq!(gradient.get_start().get_y(), 0.0);
    assert_eq!(gradient.get_end().get_y(), 100.0);
    assert_eq!(gradient.get_stops().len(), 3);
    assert_eq!(gradient.get_stops()[0], stops[0]);
    assert_eq!(gradient.get_stops()[2].0, 1.0);
}

#[test]
fn a_radial_gradient_keeps_both_circles_and_its_stops() {
    let stops: Vec<(f64, String)> = vec![(0.0, String::from("#ff0000")), (1.0, String::from("#0000ff"))];
    let gradient: RadialGradient = RadialGradient::create(
        Vector2D::new(10.0, 10.0),
        0.0,
        Vector2D::new(10.0, 10.0),
        50.0,
        stops,
    );
    assert_eq!(gradient.get_inner_radius(), 0.0);
    assert_eq!(gradient.get_outer_radius(), 50.0);
    assert_eq!(gradient.get_inner_center().get_x(), 10.0);
    assert_eq!(gradient.get_outer_center().get_y(), 10.0);
    assert_eq!(gradient.get_stops().len(), 2);
}

#[test]
fn the_three_render_layers_carry_their_documented_z_indices_and_are_visible() {
    let background: RenderLayer = RenderLayer::background();
    let foreground: RenderLayer = RenderLayer::foreground();
    let ui: RenderLayer = RenderLayer::ui();
    assert_eq!(background.get_z_index(), 0);
    assert_eq!(foreground.get_z_index(), 100);
    assert_eq!(ui.get_z_index(), 1000);
    for layer in [background, foreground, ui] {
        assert!(
            layer.get_visible(),
            "a layer created by one of the named constructors is visible by default"
        );
    }
}

#[test]
fn the_named_render_layers_are_ordered_background_first() {
    let background: i32 = RenderLayer::background().get_z_index();
    let foreground: i32 = RenderLayer::foreground().get_z_index();
    let ui: i32 = RenderLayer::ui().get_z_index();
    assert!(
        background < foreground && foreground < ui,
        "the names only mean something if they stack in that order, got {background}/{foreground}/{ui}"
    );
}

#[test]
fn the_default_gl_state_seeds_its_viewport_from_the_given_dimensions() {
    let state: GlRenderState = GlRenderState::context_defaults(321, 654);
    assert_eq!(
        *state.get_viewport().get_width(),
        321,
        "an unusual width cannot be confused with a baked-in default"
    );
    assert_eq!(*state.get_viewport().get_height(), 654);
    assert_eq!(
        *state.get_viewport().get_x(),
        0,
        "the origin is not part of the arguments"
    );
    assert_eq!(*state.get_viewport().get_y(), 0);
}

#[test]
fn two_different_default_states_do_not_share_a_viewport() {
    let narrow: GlRenderState = GlRenderState::context_defaults(100, 200);
    let wide: GlRenderState = GlRenderState::context_defaults(1600, 900);
    assert_ne!(
        *narrow.get_viewport().get_width(),
        *wide.get_viewport().get_width(),
        "a cached or hardcoded viewport would make these equal"
    );
    assert_eq!(*wide.get_viewport().get_height(), 900);
}

#[test]
fn the_default_gl_state_has_no_scissor_and_says_so_without_panicking() {
    let state: GlRenderState = GlRenderState::context_defaults(800, 600);
    assert_eq!(
        state.get_scissor(),
        None,
        "a fresh context starts with scissoring off, so the getter must yield None \
         rather than unwrapping an empty option"
    );
    assert_eq!(
        state.try_get_scissor(),
        None,
        "the safe accessor agrees with the panicking-proof one"
    );
}
