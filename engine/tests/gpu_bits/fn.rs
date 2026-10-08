use super::*;

fn all_buffer_usages() -> [BufferUsage; 10] {
    [
        BufferUsage::MapRead,
        BufferUsage::MapWrite,
        BufferUsage::CopySource,
        BufferUsage::CopyDestination,
        BufferUsage::Index,
        BufferUsage::Vertex,
        BufferUsage::Uniform,
        BufferUsage::Storage,
        BufferUsage::Indirect,
        BufferUsage::QueryResolve,
    ]
}

fn all_texture_usages() -> [TextureUsage; 4] {
    [
        TextureUsage::CopySource,
        TextureUsage::CopyDestination,
        TextureUsage::TextureBinding,
        TextureUsage::RenderAttachment,
    ]
}

fn all_shader_stages() -> [ShaderStage; 3] {
    [
        ShaderStage::Vertex,
        ShaderStage::Fragment,
        ShaderStage::Compute,
    ]
}

#[test]
fn every_buffer_usage_occupies_exactly_one_bit() {
    for usage in all_buffer_usages() {
        let bit: u32 = usage.usage_bit();
        assert_eq!(
            bit.count_ones(),
            1,
            "{usage:?} must map to a single power-of-two bit so uses combine with |, got {bit:#x}"
        );
        assert!(
            bit.is_power_of_two(),
            "{usage:?} must be a power of two, got {bit:#x}"
        );
    }
}

#[test]
fn the_buffer_usage_bits_are_all_distinct() {
    let mut seen: Vec<u32> = Vec::new();
    for usage in all_buffer_usages() {
        let bit: u32 = usage.usage_bit();
        assert!(
            !seen.contains(&bit),
            "{usage:?} collides with an earlier buffer use at {bit:#x}"
        );
        seen.push(bit);
    }
    assert_eq!(seen.len(), 10);
}

#[test]
fn the_buffer_and_texture_families_are_not_interchangeable() {
    assert_ne!(
        BufferUsage::CopySource.usage_bit(),
        TextureUsage::CopySource.usage_bit(),
        "the two families share variant NAMES but not values — CopySource is 4 \
         for a buffer and 1 for a texture, so sharing a table would silently \
         misdeclare one of them"
    );
    assert_ne!(
        BufferUsage::CopyDestination.usage_bit(),
        TextureUsage::CopyDestination.usage_bit()
    );
}

#[test]
fn the_texture_usage_bits_are_all_distinct_single_bits() {
    let mut seen: Vec<u32> = Vec::new();
    for usage in all_texture_usages() {
        let bit: u32 = usage.usage_bit();
        assert_eq!(
            bit.count_ones(),
            1,
            "{usage:?} must be a single bit, got {bit:#x}"
        );
        assert!(!seen.contains(&bit), "{usage:?} collides at {bit:#x}");
        seen.push(bit);
    }
}

#[test]
fn combining_buffer_uses_yields_the_union_of_their_bits() {
    let a: u32 = BufferUsage::Vertex.usage_bit();
    let b: u32 = BufferUsage::CopyDestination.usage_bit();
    let combined: u32 = a | b;
    assert_eq!(
        combined.count_ones(),
        2,
        "two distinct uses occupy two bits"
    );
    assert!(combined & a != 0 && combined & b != 0);
}

#[test]
fn a_repeated_use_does_not_add_a_bit() {
    let once: u32 = BufferUsage::Storage.usage_bit();
    let twice: u32 = once | once;
    assert_eq!(
        once, twice,
        "`|` must be idempotent, or a caller listing a use twice gets a \
         different descriptor for the same intent"
    );
}

#[test]
fn every_shader_stage_occupies_exactly_one_bit() {
    for stage in all_shader_stages() {
        let bit: u32 = stage.stage_bit();
        assert_eq!(
            bit.count_ones(),
            1,
            "{stage:?} must be a single bit so visibility combines with |, got {bit:#x}"
        );
    }
}

#[test]
fn the_shader_stage_bits_are_all_distinct() {
    let mut seen: Vec<u32> = Vec::new();
    for stage in all_shader_stages() {
        let bit: u32 = stage.stage_bit();
        assert!(!seen.contains(&bit), "{stage:?} collides at {bit:#x}");
        seen.push(bit);
    }
}

#[test]
fn combining_two_stages_exposes_exactly_them() {
    let vertex: u32 = ShaderStage::Vertex.stage_bit();
    let fragment: u32 = ShaderStage::Fragment.stage_bit();
    let both: u32 = vertex | fragment;
    assert_eq!(both.count_ones(), 2);
    assert_eq!(
        both & ShaderStage::Compute.stage_bit(),
        0,
        "Compute was not asked for, so a layout built this way must not          expose it — over-declaring visibility is a validation error in WebGPU"
    );
}

#[test]
fn the_vertex_step_mode_strings_are_the_wgsl_spellings() {
    assert_eq!(VertexStepMode::Vertex.as_str(), "vertex");
    assert_eq!(
        VertexStepMode::Instance.as_str(),
        "instance",
        "the string is written into the WebGPU layout, so a typo is rejected \
         only at pipeline creation"
    );
    assert_ne!(
        VertexStepMode::Vertex.as_str(),
        VertexStepMode::Instance.as_str()
    );
}

#[test]
fn the_render_quality_ladder_is_distinct_and_defaults_to_high() {
    let all: [RenderQuality; 3] = [
        RenderQuality::Low,
        RenderQuality::Medium,
        RenderQuality::High,
    ];
    for i in 0..all.len() {
        for j in (i + 1)..all.len() {
            assert_ne!(all[i], all[j], "two quality levels share a value");
        }
    }
    assert_eq!(
        RenderQuality::default(),
        RenderQuality::High,
        "the documented default errs toward visual fidelity over a few \
         milliseconds of GPU time, because users notice aliasing first"
    );
}

#[test]
fn the_render_quality_names_are_distinct_and_ordered() {
    let names: Vec<String> = [
        RenderQuality::Low,
        RenderQuality::Medium,
        RenderQuality::High,
    ]
    .iter()
    .map(|quality: &RenderQuality| format!("{quality:?}"))
    .collect();
    assert_eq!(
        names,
        vec!["Low", "Medium", "High"],
        "the ladder must read low → medium → high in that order"
    );
}
