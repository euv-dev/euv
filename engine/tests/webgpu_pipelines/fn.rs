use super::*;

const SHADER: &str = "@compute @workgroup_size(8) fn cs_main() {}";

fn grid() -> DispatchArgs {
    let mut args: DispatchArgs = DispatchArgs::new();
    args.set_x(4);
    args.set_y(2);
    args.set_z(1);
    args
}

fn pipeline_descriptor() -> RenderPipelineDescriptor {
    RenderPipelineDescriptor::new(
        VertexState::new(node("module").into(), String::from("vs_main"), Vec::new()),
        PrimitiveState::default(),
        None,
        MultisampleState::default(),
        Some(FragmentState::new(
            node("module").into(),
            String::from("fs_main"),
            vec![ColorTargetState::for_format(GpuTextureFormat::Rgba8Unorm)],
        )),
    )
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records js_sys stand-ins for a WebGPU device, which only exist under wasm"
)]
fn a_compute_pipeline_compiles_its_shader_before_asking_for_a_pipeline() {
    let subject: WebGpuRenderer = renderer(false);
    let before: u32 = mark();

    let created: JsValue = subject.create_compute_pipeline(SHADER, "cs_main");

    assert!(!created.is_undefined(), "the driver handed back a handle");
    assert_eq!(
        ops_from(before),
        vec![
            String::from("device.createShaderModule"),
            String::from("device.createComputePipeline")
        ],
        "the module has to exist before the pipeline that references it"
    );
    let module_args: Array = args_on("device", "createShaderModule", before);
    assert_eq!(
        field(&module_args.get(0), &["code"]).as_string(),
        Some(String::from(SHADER)),
        "the WGSL source crosses the boundary verbatim"
    );
    let pipeline_args: Array = args_on("device", "createComputePipeline", before);
    let descriptor: JsValue = pipeline_args.get(0);
    assert_eq!(
        field(&descriptor, &["layout"]).as_string(),
        Some(String::from("auto")),
        "this entry point is documented as the auto-layout preset"
    );
    assert_eq!(
        field(&descriptor, &["compute", "entryPoint"]).as_string(),
        Some(String::from("cs_main")),
        "and the entry point name the caller asked for, not one derived from the source"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records js_sys stand-ins for a WebGPU device, which only exist under wasm"
)]
fn a_compute_pass_opens_on_the_encoder_it_was_given() {
    let subject: WebGpuRenderer = renderer(false);
    let before: u32 = mark();
    let encoder: Object = node("encoder");

    let opened: JsValue = subject.begin_compute_pass(&encoder.clone().into());

    assert!(
        !opened.is_undefined(),
        "a driver handle means the pass was opened"
    );
    let args: Array = args_on("encoder", "beginComputePass", before);
    assert_eq!(
        args.length(),
        1,
        "beginComputePass still takes a descriptor object, and omitting it entirely is what \
         makes some drivers default the label to the empty string"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records js_sys stand-ins for a WebGPU device, which only exist under wasm"
)]
fn setting_a_compute_pipeline_reaches_the_pass_under_the_render_name() {
    let subject: WebGpuRenderer = renderer(false);
    let before: u32 = mark();
    let pass: Object = node("pass");

    subject.set_compute_pipeline(&pass.clone().into(), &node("pipeline").into());

    assert_eq!(
        count_on("pass", "setPipeline", before),
        1,
        "WebGPU names this setter the same on both pass kinds, and this is the compute one"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records js_sys stand-ins for a WebGPU device, which only exist under wasm"
)]
fn a_dispatch_with_a_bind_group_binds_everything_before_it_dispatches() {
    let subject: WebGpuRenderer = renderer(false);
    let before: u32 = mark();
    let pass: Object = node("pass");

    subject.dispatch_with_bind_group(
        &pass.clone().into(),
        &node("pipeline").into(),
        &node("group").into(),
        &grid(),
    );

    assert_eq!(
        ops_from(before),
        vec![
            String::from("pass.setPipeline"),
            String::from("pass.setBindGroup"),
            String::from("pass.dispatchWorkgroups")
        ],
        "pipeline, then bind group, then work: any other order is a validation error in every \
         spec-compliant driver"
    );
    let bound: Array = args_on("pass", "setBindGroup", before);
    assert_eq!(
        bound.get(0).as_f64(),
        Some(0.0),
        "this helper binds group 0, which is what its signature promises"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records js_sys stand-ins for a WebGPU device, which only exist under wasm"
)]
fn a_dispatch_keeps_the_axis_order_x_then_y_then_z() {
    let subject: WebGpuRenderer = renderer(false);
    let before: u32 = mark();
    let pass: Object = node("pass");

    subject.dispatch(&pass.clone().into(), &grid());

    let args: Array = args_on("pass", "dispatchWorkgroups", before);
    assert_eq!(args.length(), 3, "one count per axis, no more");
    assert_eq!(args.get(0).as_f64(), Some(4.0), "x is the first argument");
    assert_eq!(args.get(1).as_f64(), Some(2.0), "y is the second");
    assert_eq!(
        args.get(2).as_f64(),
        Some(1.0),
        "z is the third; the whole reason DispatchArgs exists as a struct is that these three \
         read the same backwards"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records js_sys stand-ins for a WebGPU device, which only exist under wasm"
)]
fn an_offset_indexed_draw_moves_the_first_index_and_leaves_the_base_vertex_at_zero() {
    let subject: WebGpuRenderer = renderer(false);
    let before: u32 = mark();
    let pass: Object = node("pass");

    subject.draw_indexed_offset(&pass.clone().into(), 12, 36, 2);

    let args: Array = args_on("pass", "drawIndexed", before);
    assert_eq!(
        args.get(0).as_f64(),
        Some(36.0),
        "the index count comes first"
    );
    assert_eq!(args.get(1).as_f64(), Some(2.0), "then the instance count");
    assert_eq!(
        args.get(2).as_f64(),
        Some(12.0),
        "the offset this call is named for is the first index, which is a different slot from \
         the base vertex"
    );
    assert_eq!(
        args.get(3).as_f64(),
        Some(0.0),
        "and the base vertex stays at zero, because this entry point has no argument for it"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records js_sys stand-ins for a WebGPU device, which only exist under wasm"
)]
fn a_frame_with_a_bind_group_encodes_ends_finishes_and_submits() {
    let mut subject: WebGpuRenderer = renderer(false);
    let before: u32 = mark();

    subject.render_frame_with_bind_group(
        &node("pipeline").into(),
        &node("group").into(),
        Color::new(0.0, 0.0, 0.0, 1.0),
        6,
    );

    assert_eq!(
        ops_since(before),
        vec![
            String::from("createCommandEncoder"),
            String::from("getCurrentTexture"),
            String::from("createView"),
            String::from("beginRenderPass"),
            String::from("setPipeline"),
            String::from("setBindGroup"),
            String::from("draw"),
            String::from("end"),
            String::from("finish"),
            String::from("submit"),
        ],
        "the whole frame in order: grab the swap chain texture, open a pass on it, bind, draw, \
         end, finish, submit. This compares method names rather than full paths because the \
         renderer caches its method lookups process-wide, so the receiver recorded here belongs \
         to whichever test first touched that method"
    );
    let drawn: Array = args_since("draw", before);
    assert_eq!(
        drawn.get(0).as_f64(),
        Some(6.0),
        "the vertex count the caller asked for comes first; the topology is baked into the \
         pipeline, so there is no primitive mode argument on this call"
    );
    assert_eq!(
        drawn.get(1).as_f64(),
        Some(1.0),
        "and a single instance, which is what the whole-stream draw this path uses means"
    );
    let submitted: Array = args_since("submit", before);
    assert_eq!(
        submitted.get(0).unchecked_into::<Array>().length(),
        1,
        "one finished command buffer crosses to the queue, always wrapped in an array"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records js_sys stand-ins for a WebGPU device, which only exist under wasm"
)]
fn a_render_pipeline_built_on_an_explicit_layout_carries_that_layout_rather_than_auto() {
    let subject: WebGpuRenderer = renderer(false);
    let before: u32 = mark();

    subject.create_render_pipeline_with_layout(&pipeline_descriptor(), &node("layout").into());

    let args: Array = args_on("device", "createRenderPipeline", before);
    let descriptor: JsValue = args.get(0);
    assert_eq!(
        field(&descriptor, &["layout"]).as_string(),
        Some(String::from("<gpu>")),
        "the caller's pre-built layout has to travel across; substituting the auto-layout \
         string here would silently break the sharing this entry point exists for"
    );
    assert_eq!(
        field(&descriptor, &["vertex", "entryPoint"]).as_string(),
        Some(String::from("vs_main")),
        "the vertex stage is threaded through from the descriptor"
    );
    assert_eq!(
        field(&descriptor, &["fragment", "entryPoint"]).as_string(),
        Some(String::from("fs_main")),
        "and so is the fragment stage, since this descriptor has one"
    );
    assert!(
        field(&descriptor, &["depthStencil"]).is_undefined(),
        "a descriptor with no depth-stencil state must leave the key out; an empty object there \
         is a different pipeline to the driver"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records js_sys stand-ins for a WebGPU device, which only exist under wasm"
)]
fn a_render_pipeline_without_a_fragment_stage_still_reaches_the_device() {
    let subject: WebGpuRenderer = renderer(false);
    let before: u32 = mark();
    let depth_only: RenderPipelineDescriptor = RenderPipelineDescriptor::new(
        VertexState::new(node("module").into(), String::from("vs_main"), Vec::new()),
        PrimitiveState::default(),
        None,
        MultisampleState::default(),
        None,
    );

    subject.create_render_pipeline_with_layout(&depth_only, &node("layout").into());

    let args: Array = args_on("device", "createRenderPipeline", before);
    assert_eq!(
        count_on("device", "createRenderPipeline", before),
        1,
        "a depth-only prepass is still a pipeline and must be created"
    );
    assert!(
        field(&args.get(0), &["fragment"]).is_undefined(),
        "with no fragment key, because the driver infers a depth-only pipeline from its absence"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records js_sys stand-ins for a WebGPU device, which only exist under wasm"
)]
fn a_render_pass_onto_a_named_texture_never_touches_the_swap_chain() {
    let mut subject: WebGpuRenderer = renderer(false);
    let before: u32 = mark();

    let opened: JsValue = subject.begin_render_pass_to_texture(
        &node("encoder").into(),
        &node("colour-view").into(),
        Some(Color::new(0.0, 0.0, 0.0, 1.0)),
        Some(&node("depth-view").into()),
        Some(1.0),
    );

    assert!(
        !opened.is_undefined(),
        "the driver handed back a pass handle"
    );
    assert_eq!(
        count_on("context", "getCurrentTexture", before),
        0,
        "rendering off-screen must not reach for the swap chain texture; acquiring it for a \
         frame nobody renders to is a wasted round trip and presents an untouched canvas \
         alongside the real output"
    );
    assert_eq!(
        count_since("beginRenderPass", before),
        1,
        "but it does open exactly one render pass, on the encoder the caller handed in"
    );
    let descriptor: JsValue = args_since("beginRenderPass", before).get(0);
    assert_eq!(
        field(&descriptor, &["colorAttachments"])
            .unchecked_into::<Array>()
            .length(),
        1,
        "with the single colour attachment the caller named"
    );
    assert_eq!(
        field(&descriptor, &["depthStencilAttachment", "view"]).as_string(),
        Some(String::from("<gpu>")),
        "and the depth view they passed alongside it, which is attached because this call \
         carries one rather than leaving the attachment off"
    );
}
