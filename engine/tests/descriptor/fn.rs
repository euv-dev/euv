use super::*;

#[test]
fn a_2d_texture_descriptor_defaults_to_one_mip_and_one_sample() {
    let observed: Texture2DDescriptor =
        Texture2DDescriptor::default_for(800, 600, GpuTextureFormat::Rgba8Unorm);
    let debugged: String = format!("{observed:?}");
    assert!(
        debugged.contains("800"),
        "the width is stored, got {debugged}"
    );
    assert!(
        debugged.contains("600"),
        "the height is stored, got {debugged}"
    );
    assert!(
        debugged.contains("mip_level_count: 1"),
        "a fresh 2D texture is a single mip, got {debugged}"
    );
    assert!(
        debugged.contains("sample_count: 1"),
        "a fresh 2D texture is not multisampled, got {debugged}"
    );
}

#[test]
fn a_full_texture_view_leaves_every_override_at_zero() {
    let observed: TextureViewDescriptor = TextureViewDescriptor::full();
    let debugged: String = format!("{observed:?}");
    assert!(
        debugged.contains("base_mip_level: 0") && debugged.contains("mip_level_count: 0"),
        "a full view covers every mip from the base, got {debugged}"
    );
    assert!(
        debugged.contains("base_array_layer: 0") && debugged.contains("array_layer_count: 0"),
        "and every array layer, got {debugged}"
    );
}

#[test]
fn a_full_texture_view_defers_format_and_dimension_to_the_texture() {
    let observed: TextureViewDescriptor = TextureViewDescriptor::full();
    let debugged: String = format!("{observed:?}");
    assert!(
        debugged.contains("format: None") && debugged.contains("dimension: None"),
        "None means inherit from the parent texture, got {debugged}"
    );
}

#[test]
fn a_single_mip_view_pins_the_base_level_to_that_mip() {
    let observed: TextureViewDescriptor = TextureViewDescriptor::mip(3);
    let debugged: String = format!("{observed:?}");
    assert!(
        debugged.contains("base_mip_level: 3"),
        "the requested mip becomes the base, got {debugged}"
    );
    assert!(
        debugged.contains("mip_level_count: 1"),
        "a single-mip view covers exactly one level, got {debugged}"
    );
}

#[test]
fn a_single_mip_view_still_inherits_its_format() {
    let observed: TextureViewDescriptor = TextureViewDescriptor::mip(0);
    let debugged: String = format!("{observed:?}");
    assert!(
        debugged.contains("format: None"),
        "picking a mip says nothing about the format, got {debugged}"
    );
}

#[test]
fn a_depth_only_view_pins_the_depth_aspect() {
    let observed: TextureViewDescriptor = TextureViewDescriptor::depth_only();
    let debugged: String = format!("{observed:?}");
    assert!(
        debugged.contains("depth-only"),
        "a depth-only view must declare the depth aspect, got {debugged}"
    );
}

#[test]
fn a_depth_only_view_leaves_the_colour_aspect_unset() {
    let observed: TextureViewDescriptor = TextureViewDescriptor::depth_only();
    let debugged: String = format!("{observed:?}");
    assert!(
        debugged.contains("aspect: Some(\"depth-only\")"),
        "the aspect is the depth one, got {debugged}"
    );
}

#[test]
fn a_depth_only_view_covers_every_layer_like_a_full_view() {
    let full: TextureViewDescriptor = TextureViewDescriptor::full();
    let depth: TextureViewDescriptor = TextureViewDescriptor::depth_only();
    let full_debug: String = format!("{full:?}");
    let depth_debug: String = format!("{depth:?}");
    assert!(
        full_debug.contains("array_layer_count: 0") && depth_debug.contains("array_layer_count: 0"),
        "the aspect is the only difference between the two shapes"
    );
}

#[test]
fn a_uniform_layout_entry_records_its_binding_and_visibility() {
    let observed: BindGroupLayoutEntry = BindGroupLayoutEntry::uniform(4, ShaderStage::Vertex);
    assert_eq!(
        observed.binding, 4,
        "the binding index is carried through to the entry"
    );
    assert_eq!(
        observed.visibility.bits(),
        1,
        "a lone vertex stage widens to the mask naming exactly that one stage, rather than the \
         entry keeping a bare ShaderStage that could never be combined"
    );
    let both: BindGroupLayoutEntry =
        BindGroupLayoutEntry::uniform(4, ShaderStage::Vertex | ShaderStage::Fragment);
    assert_eq!(
        both.visibility.bits(),
        3,
        "and the same constructor takes the combination, which is the case a uniform block \
         shared by the two stages needs"
    );
}

#[test]
fn a_storage_layout_entry_records_read_only_when_asked() {
    let readonly: BindGroupLayoutEntry =
        BindGroupLayoutEntry::storage(2, ShaderStage::Compute, true);
    let readwrite: BindGroupLayoutEntry =
        BindGroupLayoutEntry::storage(2, ShaderStage::Compute, false);
    assert_eq!(
        readonly.binding, readwrite.binding,
        "the binding index is unaffected"
    );
    let readonly_debug: String = format!("{readonly:?}");
    let readwrite_debug: String = format!("{readwrite:?}");
    assert!(
        readonly_debug.contains("read_only: true"),
        "a read-only storage buffer says so, got {readonly_debug}"
    );
    assert!(
        readwrite_debug.contains("read_only: false"),
        "a writable storage buffer says so too, got {readwrite_debug}"
    );
}

#[test]
fn a_uniform_entry_is_not_a_storage_entry() {
    let uniform: BindGroupLayoutEntry = BindGroupLayoutEntry::uniform(0, ShaderStage::Fragment);
    let storage: BindGroupLayoutEntry =
        BindGroupLayoutEntry::storage(0, ShaderStage::Fragment, false);
    assert_ne!(
        format!("{uniform:?}"),
        format!("{storage:?}"),
        "a uniform buffer and a storage buffer are different bindings"
    );
}

#[test]
fn a_uniform_entry_keeps_its_binding_verbatim() {
    for index in [0_u32, 1, 7, 255] {
        let observed: BindGroupLayoutEntry =
            BindGroupLayoutEntry::uniform(index, ShaderStage::Vertex);
        assert_eq!(
            observed.binding, index,
            "the binding index is carried through unchanged"
        );
    }
}

#[test]
fn a_storage_entry_keeps_its_binding_verbatim() {
    for index in [0_u32, 3, 64] {
        let observed: BindGroupLayoutEntry =
            BindGroupLayoutEntry::storage(index, ShaderStage::Fragment, true);
        assert_eq!(
            observed.binding, index,
            "the binding index is carried through unchanged"
        );
    }
}

#[test]
fn every_bind_group_entry_reports_the_slot_it_was_declared_at() {
    let buffer: BindGroupEntry = BindGroupEntry::Buffer {
        binding: 3,
        buffer: JsValue::NULL,
        offset: 0,
        size: None,
    };
    let texture: BindGroupEntry = BindGroupEntry::Texture {
        binding: 4,
        view: JsValue::NULL,
    };
    let sampler: BindGroupEntry = BindGroupEntry::Sampler {
        binding: 5,
        sampler: JsValue::NULL,
    };
    let storage: BindGroupEntry = BindGroupEntry::StorageTexture {
        binding: 6,
        view: JsValue::NULL,
        read_only: true,
    };

    assert_eq!(buffer.binding(), 3, "a buffer binding keeps its slot");
    assert_eq!(texture.binding(), 4, "so does a texture binding");
    assert_eq!(sampler.binding(), 5, "and a sampler binding");
    assert_eq!(
        storage.binding(),
        6,
        "and a storage-texture binding; the four variants share one accessor precisely so a \
         caller cannot have to special-case them when laying out a group"
    );
}
