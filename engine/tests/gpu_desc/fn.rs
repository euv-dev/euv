use super::*;

fn rendered<T: Debug>(value: &T) -> String {
    format!("{value:?}")
}

#[test]
fn a_non_filtering_sampler_differs_from_a_filtering_one() {
    let filtering: BindGroupLayoutEntry = BindGroupLayoutEntry::sampler(0, ShaderStage::Fragment);
    let non_filtering: BindGroupLayoutEntry =
        BindGroupLayoutEntry::sampler_non_filtering(0, ShaderStage::Fragment);
    assert_ne!(
        rendered(&filtering),
        rendered(&non_filtering),
        "nearest and linear sampling are different looks; collapsing them would \
         make `non_filtering` a lie"
    );
}

#[test]
fn a_comparison_sampler_differs_from_a_plain_one() {
    let plain: BindGroupLayoutEntry = BindGroupLayoutEntry::sampler(1, ShaderStage::Compute);
    let comparison: BindGroupLayoutEntry =
        BindGroupLayoutEntry::sampler_comparison(1, ShaderStage::Compute);
    assert_ne!(
        rendered(&plain),
        rendered(&comparison),
        "a comparison sampler feeds shadow maps; a plain one must not stand in for it"
    );
}

#[test]
fn every_bind_group_entry_records_the_binding_it_was_given() {
    let entries: [BindGroupLayoutEntry; 4] = [
        BindGroupLayoutEntry::sampler(0, ShaderStage::Fragment),
        BindGroupLayoutEntry::sampler_non_filtering(1, ShaderStage::Fragment),
        BindGroupLayoutEntry::sampler_comparison(2, ShaderStage::Fragment),
        BindGroupLayoutEntry::sampler(3, ShaderStage::Vertex),
    ];
    for (index, entry) in entries.iter().enumerate() {
        let text: String = rendered(entry);
        assert!(
            text.contains(&format!("binding: {index}")),
            "the binding index is how the shader finds the slot, got: {text}"
        );
    }
}

#[test]
fn the_two_bind_group_entry_type_families_stay_distinct() {
    let a: BindGroupLayoutEntry = BindGroupLayoutEntry::sampler(0, ShaderStage::Fragment);
    let b: BindGroupLayoutEntry =
        BindGroupLayoutEntry::texture_multisampled(0, ShaderStage::Fragment, "float");
    assert_ne!(rendered(&a), rendered(&b));
    let _sampler_kind: BindGroupEntryType = BindGroupEntryType::Sampler {
        comparison: false,
        filtering: true,
    };
    let _texture_kind: BindGroupEntryType = BindGroupEntryType::SampledTexture {
        sample_type: String::from("float"),
        multisampled: false,
    };
}

#[test]
fn a_color_target_keeps_the_format_it_was_built_for() {
    let target: ColorTargetState = ColorTargetState::for_format(GpuTextureFormat::Rgba8Unorm);
    let text: String = rendered(&target);
    assert!(
        text.contains("Rgba8Unorm"),
        "a colour attachment's format is the whole point of the call, got: {text}"
    );
    let other: ColorTargetState = ColorTargetState::for_format(GpuTextureFormat::Bgra8Unorm);
    assert_ne!(
        text,
        rendered(&other),
        "two formats must not produce the same attachment"
    );
}

#[test]
fn a_multisample_state_of_one_carries_the_full_sample_mask() {
    let single: MultisampleState = MultisampleState::with_sample_count(1);
    let four: MultisampleState = MultisampleState::with_sample_count(4);
    let single_text: String = rendered(&single);
    assert!(single_text.contains("count: 1"), "got: {single_text}");
    assert!(
        single_text.replace("count: 1", "") == rendered(&four).replace("count: 4", ""),
        "only the count may differ between sample counts; the mask must not: \
         {single_text} vs {}",
        rendered(&four)
    );
}

#[test]
fn a_default_texture_descriptor_has_a_single_mip_and_no_multisampling() {
    let texture: Texture2DDescriptor =
        Texture2DDescriptor::default_for(1920, 1080, GpuTextureFormat::Rgba8Unorm);
    let text: String = rendered(&texture);
    assert!(text.contains("width: 1920"), "got: {text}");
    assert!(text.contains("height: 1080"), "got: {text}");
    assert!(
        text.contains("mip_level_count: 1") && text.contains("sample_count: 1"),
        "a default texture is neither mipmapped nor multisampled, got: {text}"
    );
}

#[test]
fn a_full_texture_view_selects_everything_from_level_zero() {
    let view: TextureViewDescriptor = TextureViewDescriptor::full();
    let text: String = rendered(&view);
    assert!(
        text.contains("base_mip_level: 0"),
        "a full view starts at the base level, got: {text}"
    );
    for field in [
        "mip_level_count: 0",
        "base_array_layer: 0",
        "array_layer_count: 0",
    ] {
        assert!(
            text.contains(field),
            "a full view leaves {field} to the implementation, got: {text}"
        );
    }
}

#[test]
fn a_single_mip_view_starts_at_the_requested_level() {
    let view: TextureViewDescriptor = TextureViewDescriptor::mip(3);
    let text: String = rendered(&view);
    assert!(
        text.contains("base_mip_level: 3"),
        "asking for mip 3 must actually look at mip 3, got: {text}"
    );
}

#[test]
fn a_depth_only_view_starts_at_the_base_level() {
    let view: TextureViewDescriptor = TextureViewDescriptor::depth_only();
    let text: String = rendered(&view);
    assert!(
        text.contains("base_mip_level: 0"),
        "a depth attachment has no mip chain to pick from, got: {text}"
    );
    assert_ne!(
        text,
        rendered(&TextureViewDescriptor::full()),
        "a depth-only view is a different view, not a synonym for the full one"
    );
}

#[test]
fn a_uniform_binding_differs_from_a_storage_binding() {
    let uniform: BindGroupLayoutEntry = BindGroupLayoutEntry::uniform(0, ShaderStage::Vertex);
    let storage: BindGroupLayoutEntry =
        BindGroupLayoutEntry::storage(0, ShaderStage::Vertex, false);
    assert_ne!(
        rendered(&uniform),
        rendered(&storage),
        "read-write storage is a different shader contract than a uniform buffer"
    );
}

#[test]
fn a_read_only_storage_binding_differs_from_a_writable_one() {
    let read_only: BindGroupLayoutEntry =
        BindGroupLayoutEntry::storage(0, ShaderStage::Compute, true);
    let writable: BindGroupLayoutEntry =
        BindGroupLayoutEntry::storage(0, ShaderStage::Compute, false);
    assert_ne!(
        rendered(&read_only),
        rendered(&writable),
        "the read-only flag is the whole difference and must survive into the layout"
    );
}

#[test]
fn a_single_sampled_texture_differs_from_a_multisampled_one() {
    let single: BindGroupLayoutEntry =
        BindGroupLayoutEntry::texture(0, ShaderStage::Fragment, "float");
    let multi: BindGroupLayoutEntry =
        BindGroupLayoutEntry::texture_multisampled(0, ShaderStage::Fragment, "float");
    assert_ne!(
        rendered(&single),
        rendered(&multi),
        "a resolve target and a multisample target are different layouts"
    );
}

#[test]
fn a_write_only_storage_texture_differs_from_a_read_write_one() {
    let read_only: BindGroupLayoutEntry =
        BindGroupLayoutEntry::storage_texture(0, ShaderStage::Compute, "r32float", true);
    let writable: BindGroupLayoutEntry =
        BindGroupLayoutEntry::storage_texture(0, ShaderStage::Compute, "r32float", false);
    assert_ne!(rendered(&read_only), rendered(&writable));
}

#[test]
fn the_nearest_clamp_sampler_filters_nothing_and_clamps_every_edge() {
    let sampler: SamplerDescriptor = SamplerDescriptor::nearest_clamp();
    let text: String = rendered(&sampler);
    assert!(
        text.contains("Nearest"),
        "the cheapest sampler must not blur, got: {text}"
    );
    assert!(
        text.contains("ClampToEdge"),
        "clamp-to-edge is what stops a border texel from wrapping, got: {text}"
    );
}

#[test]
fn a_whole_stream_draw_covers_every_vertex_of_every_instance() {
    let args: DrawArgs = DrawArgs::whole_stream(36, 2);
    let text: String = rendered(&args);
    assert!(text.contains("vertex_count: 36"), "got: {text}");
    assert!(text.contains("instance_count: 2"), "got: {text}");
    assert!(text.contains("first_vertex: 0"), "got: {text}");
    assert!(text.contains("first_instance: 0"), "got: {text}");
}

#[test]
fn a_whole_indexed_draw_covers_every_index_with_no_base_vertex() {
    let args: DrawIndexedArgs = DrawIndexedArgs::whole_buffer(6, 1);
    let text: String = rendered(&args);
    assert!(text.contains("index_count: 6"), "got: {text}");
    assert!(text.contains("first_index: 0"), "got: {text}");
    assert!(
        text.contains("base_vertex: 0"),
        "a non-zero base vertex would silently re-index the whole buffer, got: {text}"
    );
}

#[test]
fn a_dispatch_covers_the_whole_three_dimensional_grid() {
    let args: DispatchArgs = DispatchArgs::grid(8, 4, 2);
    let text: String = rendered(&args);
    for field in ["x: 8", "y: 4", "z: 2"] {
        assert!(
            text.contains(field),
            "the workgroup {field} must survive, got: {text}"
        );
    }
}

#[test]
fn a_sampler_entry_keeps_its_filtering_and_comparison_flags_apart() {
    let plain: BindGroupLayoutEntry = BindGroupLayoutEntry::sampler(0, ShaderStage::Fragment);
    let non_filtering: BindGroupLayoutEntry =
        BindGroupLayoutEntry::sampler_non_filtering(0, ShaderStage::Fragment);
    let comparison: BindGroupLayoutEntry =
        BindGroupLayoutEntry::sampler_comparison(0, ShaderStage::Fragment);
    let shapes: Vec<String> = vec![
        rendered(&plain),
        rendered(&non_filtering),
        rendered(&comparison),
    ];
    for i in 0..shapes.len() {
        for j in (i + 1)..shapes.len() {
            assert_ne!(
                shapes[i], shapes[j],
                "three sampler flavours collapsed into one layout"
            );
        }
    }
}
