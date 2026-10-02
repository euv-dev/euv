use euv_engine::*;

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
    camera.set_rotation(std::f64::consts::FRAC_PI_2);
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
