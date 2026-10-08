use super::*;

fn both_stages() -> ShaderStages {
    ShaderStage::Vertex | ShaderStage::Fragment
}

#[test]
fn a_lone_stage_becomes_a_mask_naming_only_itself() {
    let compute: ShaderStages = ShaderStage::Compute.into();

    assert_eq!(
        compute.bits(),
        4,
        "GPUShaderStage.COMPUTE is 0x04, and widening must not shift it"
    );
    assert!(
        !compute.contains(ShaderStage::Vertex),
        "and widening one stage must not sweep in the others"
    );
    assert!(compute.contains(ShaderStage::Compute));
}

#[test]
fn two_stages_or_together_into_the_union_of_their_bits() {
    let both: ShaderStages = both_stages();

    assert_eq!(
        both.bits(),
        3,
        "VERTEX 0x01 | FRAGMENT 0x02; this is the combination a uniform block shared by the \
         two stages needs, and it is exactly what a single ShaderStage could not express"
    );
    assert!(both.contains(ShaderStage::Vertex));
    assert!(both.contains(ShaderStage::Fragment));
    assert!(
        !both.contains(ShaderStage::Compute),
        "a compute pass cannot see a vertex/fragment binding, so its bit must stay clear"
    );
}

#[test]
fn adding_a_stage_to_a_mask_leaves_the_earlier_bits_alone() {
    let vertex_only: ShaderStages = ShaderStage::Vertex.into();

    let all_three: ShaderStages = vertex_only | ShaderStage::Fragment | ShaderStage::Compute;

    assert_eq!(all_three.bits(), 7, "0x01 | 0x02 | 0x04");
    assert!(all_three.contains(ShaderStage::Vertex));
    assert!(all_three.contains(ShaderStage::Compute));
}

#[test]
fn two_masks_union_without_losing_either_side() {
    let render: ShaderStages = both_stages();
    let compute: ShaderStages = ShaderStage::Compute.into();

    assert_eq!(
        (render | compute).bits(),
        7,
        "combining two independently built halves of a layout entry is why the mask itself is \
         combinable, not just a stage with one"
    );
}

#[test]
fn the_default_mask_names_nothing_at_all() {
    let empty: ShaderStages = ShaderStages::default();

    assert!(
        empty.is_empty(),
        "the default has to be the empty mask, not vertex"
    );
    assert_eq!(
        empty.bits(),
        0,
        "and zero bits, which the driver reads as a slot nothing can see"
    );
}

#[test]
fn a_layout_entry_keeps_both_stages_visible_when_given_the_combination() {
    let entry: BindGroupLayoutEntry = BindGroupLayoutEntry::uniform(0, both_stages());

    assert_eq!(
        entry.visibility.bits(),
        3,
        "the entry's own visibility field is the combined mask, so a caller who passes the \\
         documented Vertex | Fragment actually gets a two-stage slot"
    );
    assert_eq!(
        entry.binding, 0,
        "and the binding slot is untouched by the combination"
    );
}

#[test]
fn a_layout_entry_still_accepts_a_bare_stage_unchanged() {
    let entry: BindGroupLayoutEntry = BindGroupLayoutEntry::uniform(2, ShaderStage::Compute);

    assert_eq!(
        entry.visibility.bits(),
        4,
        "the single-stage call sites widen without any change at the call site, which is what \\
         keeps every existing caller compiling"
    );
}
