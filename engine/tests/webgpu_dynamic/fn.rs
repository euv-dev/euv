use super::*;

fn viewport() -> ViewportDescriptor {
    let mut descriptor: ViewportDescriptor = ViewportDescriptor::new(4.0, 8.0, 320.0, 240.0);
    descriptor.set_min_depth(0.25);
    descriptor.set_max_depth(0.75);
    descriptor
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records js_sys stand-ins for a WebGPU pass, which only exist under wasm"
)]
fn a_viewport_travels_as_six_scalars_rather_than_a_dictionary() {
    let subject: WebGpuRenderer = renderer(false);
    let before: u32 = mark();
    let pass: Object = node("pass");

    subject.set_viewport(&pass.clone().into(), &viewport());

    let args: Array = args_since("setViewport", before);
    assert_eq!(
        args.length(),
        6,
        "setViewport takes six scalars; the old descriptor-dict form reached the driver as \
         NaN on every field and was swallowed as a validation error"
    );
    assert_eq!(args.get(0).as_f64(), Some(4.0), "x");
    assert_eq!(args.get(1).as_f64(), Some(8.0), "y");
    assert_eq!(args.get(2).as_f64(), Some(320.0), "width");
    assert_eq!(args.get(3).as_f64(), Some(240.0), "height");
    assert_eq!(
        args.get(4).as_f64(),
        Some(0.25),
        "minDepth, which clamps NDC -1..1 into the depth range and is not the same as the \
         rectangle's y"
    );
    assert_eq!(args.get(5).as_f64(), Some(0.75), "maxDepth last");
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records js_sys stand-ins for a WebGPU pass, which only exist under wasm"
)]
fn a_scissor_rect_forwards_four_integers_in_spec_order() {
    let subject: WebGpuRenderer = renderer(false);
    let before: u32 = mark();
    let pass: Object = node("pass");

    subject.set_scissor_rect(&pass.clone().into(), 10, 20, 30, 40);

    let args: Array = args_since("setScissorRect", before);
    assert_eq!(
        args.length(),
        4,
        "x, y, width, height — no offsets and no flags"
    );
    assert_eq!(args.get(0).as_f64(), Some(10.0), "x");
    assert_eq!(args.get(1).as_f64(), Some(20.0), "y");
    assert_eq!(args.get(2).as_f64(), Some(30.0), "width");
    assert_eq!(
        args.get(3).as_f64(),
        Some(40.0),
        "height last; swapping width and height here silently crops the wrong axis"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records js_sys stand-ins for a WebGPU pass, which only exist under wasm"
)]
fn a_blend_constant_keeps_its_channel_order() {
    let subject: WebGpuRenderer = renderer(false);
    let before: u32 = mark();
    let pass: Object = node("pass");

    subject.set_blend_constant(&pass.clone().into(), 0.1, 0.2, 0.3, 0.4);

    let args: Array = args_since("setBlendConstant", before);
    assert_eq!(
        args.length(),
        1,
        "setBlendConstant takes a single GPUColorDict, not four scalars — the opposite of \
         setViewport, and getting this backwards reaches the driver as one unrecognised \
         argument and a silent no-op"
    );
    let color: JsValue = args.get(0);
    assert_eq!(
        field(&color, &["r"]).as_f64(),
        Some(f64::from(0.1_f32)),
        "red, widened from the f32 the engine stores"
    );
    assert_eq!(
        field(&color, &["g"]).as_f64(),
        Some(f64::from(0.2_f32)),
        "green"
    );
    assert_eq!(
        field(&color, &["b"]).as_f64(),
        Some(f64::from(0.3_f32)),
        "blue"
    );
    assert_eq!(
        field(&color, &["a"]).as_f64(),
        Some(f64::from(0.4_f32)),
        "alpha, which is the channel a constant-alpha blend multiplies by"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records js_sys stand-ins for a WebGPU pass, which only exist under wasm"
)]
fn a_stencil_reference_reaches_the_pass_under_the_method_name_the_spec_uses() {
    let subject: WebGpuRenderer = renderer(false);
    let before: u32 = mark();
    let pass: Object = node("pass");

    subject.set_stencil_reference(&pass.clone().into(), 7);

    let args: Array = args_since("setStencilReference", before);
    assert_eq!(
        args.get(0).as_f64(),
        Some(7.0),
        "the comparison value itself; stencil state lives on the pipeline, so this call only \
         carries the per-draw reference"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records js_sys stand-ins for a WebGPU pass, which only exist under wasm"
)]
fn an_empty_dynamic_offset_list_still_uses_the_four_argument_overload() {
    let subject: WebGpuRenderer = renderer(false);
    let before: u32 = mark();
    let pass: Object = node("pass");

    subject.set_bind_group_with_dynamic_offsets(
        &pass.clone().into(),
        0,
        &node("group").into(),
        &[],
    );

    let args: Array = args_since("setBindGroup", before);
    assert_eq!(
        args.length(),
        4,
        "an empty offset list is still the dynamic overload, and that is well-defined; \
         collapsing to two arguments when the list happens to be empty would make the same \
         binding mean different things depending on the caller's data"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records js_sys stand-ins for a WebGPU pass, which only exist under wasm"
)]
fn a_compute_bind_group_with_offsets_uses_the_same_overload() {
    let subject: WebGpuRenderer = renderer(false);
    let before: u32 = mark();
    let pass: Object = node("pass");

    subject.set_bind_group_compute_with_dynamic_offsets(
        &pass.clone().into(),
        1,
        &node("group").into(),
        &[32],
    );

    let args: Array = args_since("setBindGroup", before);
    assert_eq!(
        args.length(),
        4,
        "the compute-side entry point has to reach the same four-argument overload; the render \
         and compute setters share a name, so the call shape is the only thing telling them \
         apart in a trace"
    );
    assert_eq!(
        args.get(0).as_f64(),
        Some(1.0),
        "at the index the caller named"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records js_sys stand-ins for a WebGPU device, which only exist under wasm"
)]
fn generating_mipmaps_calls_the_texture_that_was_named() {
    let subject: WebGpuRenderer = renderer(false);
    let before: u32 = mark();
    let texture: Object = node("named-texture");

    subject.generate_mipmaps(&texture.clone().into());

    assert_eq!(
        count_on("named-texture", "generateMipmap", before),
        1,
        "generateMipmap belongs to the texture, and passing the texture in is the whole point"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records js_sys stand-ins for a WebGPU device, which only exist under wasm"
)]
fn an_error_scope_names_the_filter_the_caller_asked_for() {
    let subject: WebGpuRenderer = renderer(false);
    let before: u32 = mark();

    subject.push_error_scope(GpuErrorFilter::OutOfMemory);

    let args: Array = args_since("pushErrorScope", before);
    assert_eq!(
        args.get(0).as_string(),
        Some(String::from("out-of-memory")),
        "the engine enum becomes the spec's filter string, so a validation scope and an \
         out-of-memory scope cannot be confused at the driver"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records js_sys stand-ins for a WebGPU device, which only exist under wasm"
)]
fn the_render_pipeline_preset_asks_for_the_auto_layout() {
    let subject: WebGpuRenderer = renderer(false);
    let before: u32 = mark();

    let created: JsValue = subject.create_render_pipeline("@vertex fn vs() {}");

    assert!(
        !created.is_undefined(),
        "the driver handed back a pipeline handle"
    );
    assert_eq!(
        ops_since(before),
        vec![
            String::from("createShaderModule"),
            String::from("createRenderPipeline"),
        ],
        "module first, then the pipeline that references it"
    );
    let args: Array = args_since("createRenderPipeline", before);
    assert_eq!(
        field(&args.get(0), &["layout"]).as_string(),
        Some(String::from("auto")),
        "this preset is documented as the auto-layout one; the explicit-layout path is a \
         different entry point"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records js_sys stand-ins for a WebGPU device, which only exist under wasm"
)]
fn resizing_rewrites_the_backing_store_and_reconfigures_the_swap_chain() {
    let mut subject: WebGpuRenderer = renderer(false);
    let before: u32 = mark();

    let resized: bool = subject.resize(1024, 768);

    assert!(resized, "a configure that does not throw reports success");
    let writes: Vec<String> = ops_from(before)
        .into_iter()
        .filter(|name: &String| name.starts_with("canvas."))
        .collect();
    assert_eq!(
        writes,
        vec![String::from("canvas.width"), String::from("canvas.height")],
        "the backing store is resized before the swap chain is reconfigured, or the driver \
         reads the old extent"
    );
    let configured: Array = args_on("context", "configure", before);
    assert_eq!(
        field(&configured.get(0), &["format"]).as_string(),
        Some(String::from("bgra8unorm")),
        "and the reconfigure carries the format this renderer negotiated, not a default"
    );
}
