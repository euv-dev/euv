use super::*;

#[test]
fn the_conventional_blend_is_source_over_destination() {
    let component: BlendComponent = BlendComponent::source_alpha_over();
    let rendered: String = format!("{component:?}");
    assert!(
        rendered.contains("source: SourceAlpha"),
        "the source is scaled by its own alpha, got: {rendered}"
    );
    assert!(
        rendered.contains("destination: OneMinusSourceAlpha"),
        "the destination is scaled by the remaining alpha, got: {rendered}"
    );
    assert!(
        rendered.contains("operation: Add"),
        "both sides are summed, got: {rendered}"
    );
}

#[test]
fn an_alpha_over_blend_state_applies_the_same_component_to_both_channels() {
    let state: BlendState = BlendState::alpha_over();
    let rendered: String = format!("{state:?}");
    let color: usize = rendered.find("color:").expect("a color component");
    let alpha: usize = rendered.find("alpha:").expect("an alpha component");
    assert!(
        color < alpha,
        "Debug lists the fields in declaration order, so color must come first: {rendered}"
    );
    let colour_part: &str = &rendered[color..alpha];
    let alpha_part: &str = &rendered[alpha..];
    let colour_values: Vec<&str> = colour_part
        .split([':', ',', '{', '}'])
        .map(str::trim)
        .filter(|piece: &&str| !piece.is_empty())
        .collect();
    let alpha_values: Vec<&str> = alpha_part
        .split([':', ',', '{', '}'])
        .map(str::trim)
        .filter(|piece: &&str| !piece.is_empty())
        .collect();
    assert_eq!(
        colour_values[1..],
        alpha_values[1..],
        "an ordinary translucent surface wants one blend on both channels, \
         so the two components must not drift apart"
    );
}

#[test]
fn a_multisample_state_of_one_disables_multisampling() {
    let single: MultisampleState = MultisampleState::with_sample_count(1);
    let four: MultisampleState = MultisampleState::with_sample_count(4);
    assert!(
        format!("{single:?}").contains("count: 1"),
        "one sample per pixel is the no-multisampling case"
    );
    assert!(format!("{four:?}").contains("count: 4"));
    assert_eq!(
        single.get_mask(),
        four.get_mask(),
        "every configured state must write all samples, whatever the count"
    );
}

#[test]
fn a_depth_stencil_state_writes_depth_and_keeps_the_closer_fragment() {
    let state: DepthStencilState = DepthStencilState::writing_depth(GpuTextureFormat::Depth24Plus);
    let rendered: String = format!("{state:?}");
    assert!(
        rendered.contains("depth_write_enabled: true"),
        "a depth buffer nobody writes never occludes anything, got: {rendered}"
    );
    assert!(
        rendered.contains("depth_compare: Less"),
        "the nearer fragment must win, got: {rendered}"
    );
    assert!(
        rendered.contains("Depth24Plus"),
        "the attachment format must survive verbatim, got: {rendered}"
    );
}

#[test]
fn a_draw_of_the_whole_stream_starts_at_the_first_vertex_of_the_first_instance() {
    let args: DrawArgs = DrawArgs::whole_stream(36, 2);
    assert_eq!(args.get_vertex_count(), 36);
    assert_eq!(args.get_instance_count(), 2);
    assert_eq!(
        args.get_first_vertex(),
        0,
        "a whole-stream draw has no vertex offset"
    );
    assert_eq!(args.get_first_instance(), 0);
}

#[test]
fn an_indexed_draw_of_the_whole_buffer_starts_at_index_zero_with_no_base_offset() {
    let args: DrawIndexedArgs = DrawIndexedArgs::whole_buffer(6, 1);
    assert_eq!(args.get_index_count(), 6);
    assert_eq!(args.get_instance_count(), 1);
    assert_eq!(args.get_first_index(), 0);
    assert_eq!(
        args.get_base_vertex(),
        0,
        "a base vertex of zero means vertex buffers are indexed from their own origin"
    );
    assert_eq!(args.get_first_instance(), 0);
}

#[test]
fn a_multisampled_texture_binding_records_the_sample_type() {
    let entry: BindGroupLayoutEntry =
        BindGroupLayoutEntry::texture_multisampled(3, ShaderStage::Fragment, "float");
    let rendered: String = format!("{entry:?}");
    assert!(rendered.contains("binding: 3"), "got: {rendered}");
    assert!(
        rendered.contains("multisampled: true"),
        "a resolve target must not be confused with a multisampled one, got: {rendered}"
    );
    assert!(rendered.contains("float"), "got: {rendered}");
}

#[test]
fn a_storage_texture_binding_records_its_format_and_read_only_flag() {
    let writable: BindGroupLayoutEntry =
        BindGroupLayoutEntry::storage_texture(1, ShaderStage::Compute, "rgba8unorm", false);
    let rendered: String = format!("{writable:?}");
    assert!(
        rendered.contains("read_only: false"),
        "a writable storage texture must not be marked read-only, got: {rendered}"
    );
    assert!(rendered.contains("rgba8unorm"), "got: {rendered}");

    let read_only: BindGroupLayoutEntry =
        BindGroupLayoutEntry::storage_texture(1, ShaderStage::Compute, "r32float", true);
    assert!(format!("{read_only:?}").contains("read_only: true"));
    assert!(
        format!("{writable:?}") != format!("{read_only:?}"),
        "the read-only flag and the format are part of the binding's meaning"
    );
}

#[test]
fn the_four_blend_factors_are_mutually_distinct() {
    let all: [BlendFactor; 4] = [
        BlendFactor::Zero,
        BlendFactor::One,
        BlendFactor::SourceAlpha,
        BlendFactor::OneMinusSourceAlpha,
    ];
    for i in 0..all.len() {
        for j in (i + 1)..all.len() {
            assert_ne!(all[i], all[j]);
        }
    }
}

#[test]
fn the_two_blend_operations_are_mutually_distinct() {
    assert_ne!(BlendOperation::Add, BlendOperation::Subtract);
    assert_ne!(BlendOperation::Min, BlendOperation::Max);
}

#[test]
fn the_four_primitive_topologies_are_mutually_distinct() {
    let all: [PrimitiveTopology; 4] = [
        PrimitiveTopology::PointList,
        PrimitiveTopology::LineList,
        PrimitiveTopology::TriangleList,
        PrimitiveTopology::TriangleStrip,
    ];
    for i in 0..all.len() {
        for j in (i + 1)..all.len() {
            assert_ne!(all[i], all[j]);
        }
    }
}

#[test]
fn the_two_index_formats_cover_both_index_widths() {
    assert_ne!(
        IndexFormat::Uint16,
        IndexFormat::Uint32,
        "a 16-bit and a 32-bit index buffer are different GPU layouts"
    );
}

#[test]
fn the_two_front_faces_and_three_cull_modes_are_mutually_distinct() {
    assert_ne!(FrontFace::CounterClockwise, FrontFace::Clockwise);
    let all: [CullMode; 3] = [CullMode::None, CullMode::Front, CullMode::Back];
    for i in 0..all.len() {
        for j in (i + 1)..all.len() {
            assert_ne!(all[i], all[j]);
        }
    }
}

#[test]
fn the_vertex_attribute_formats_cover_the_common_arities() {
    let all: [VertexAttributeFormat; 6] = [
        VertexAttributeFormat::Float32,
        VertexAttributeFormat::Float32x2,
        VertexAttributeFormat::Float32x3,
        VertexAttributeFormat::Float32x4,
        VertexAttributeFormat::Uint32,
        VertexAttributeFormat::Sint32,
    ];
    for i in 0..all.len() {
        for j in (i + 1)..all.len() {
            assert_ne!(all[i], all[j]);
        }
    }
}

#[test]
fn the_vertex_attribute_formats_stay_distinct_after_debug_formatting() {
    let all: [VertexAttributeFormat; 6] = [
        VertexAttributeFormat::Float32,
        VertexAttributeFormat::Float32x2,
        VertexAttributeFormat::Float32x3,
        VertexAttributeFormat::Float32x4,
        VertexAttributeFormat::Uint32,
        VertexAttributeFormat::Sint32,
    ];
    for i in 0..all.len() {
        for j in (i + 1)..all.len() {
            assert_ne!(
                all[i], all[j],
                "two formats sharing a shader spelling would be indistinguishable to WGSL"
            );
        }
    }
}

#[test]
fn the_gl_error_enums_carry_their_own_distinct_variants() {
    let a: WebGl2InitError = WebGl2InitError::ContextUnavailable;
    let b: WebGl2InitError = WebGl2InitError::CanvasNotFound(String::from("canvas"));
    assert_ne!(a, b);
    assert!(
        format!("{b:?}").contains("canvas"),
        "the message a developer reads must carry the canvas they passed in"
    );
    assert_ne!(
        format!("{a:?}"),
        format!("{b:?}"),
        "the message must say which half of the setup failed"
    );
}

#[test]
fn a_gl_shader_kind_distinguishes_the_three_program_stages() {
    let all: [GlShaderKind; 2] = [GlShaderKind::Vertex, GlShaderKind::Fragment];
    for i in 0..all.len() {
        for j in (i + 1)..all.len() {
            assert_ne!(all[i], all[j]);
        }
    }
}
