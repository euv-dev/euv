use super::*;

fn pass() -> Object {
    node("pass")
}

fn indexed() -> DrawIndexedArgs {
    let mut args: DrawIndexedArgs = DrawIndexedArgs::new();
    args.set_index_count(36);
    args.set_instance_count(1);
    args.set_first_index(0);
    args.set_base_vertex(4);
    args.set_first_instance(0);
    args
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records js_sys stand-ins for a WebGPU pass encoder"
)]
fn a_vertex_buffer_is_bound_at_the_slot_the_caller_named() {
    let subject: WebGpuRenderer = renderer(false);
    let before: u32 = mark();

    subject.set_vertex_buffer(&pass().into(), 2, &device().into());

    assert_eq!(count_on("pass", "setVertexBuffer", before), 1);
    let args: Array = args_on("pass", "setVertexBuffer", before);
    assert_eq!(
        args.get(0).as_f64(),
        Some(2.0),
        "the slot the caller named is the binding slot, not a running counter"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records js_sys stand-ins for a WebGPU pass encoder"
)]
fn an_absent_vertex_buffer_is_skipped_rather_than_bound_as_undefined() {
    let subject: WebGpuRenderer = renderer(false);
    let before: u32 = mark();

    subject.set_vertex_buffer(&pass().into(), 0, &JsValue::UNDEFINED);

    assert_eq!(
        count_on("pass", "setVertexBuffer", before),
        0,
        "binding undefined would make the driver read a null buffer with no error"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records js_sys stand-ins for a WebGPU pass encoder"
)]
fn the_index_buffer_format_travels_as_the_spec_string() {
    let subject: WebGpuRenderer = renderer(false);
    let before: u32 = mark();

    subject.set_index_buffer(&pass().into(), &device().into(), IndexFormat::Uint32);

    let args: Array = args_on("pass", "setIndexBuffer", before);
    assert_eq!(
        args.get(1).as_string(),
        Some(String::from("uint32")),
        "the engine enum becomes the spec's format string, so a mismatch with the buffer's \
         element type is visible to the driver rather than silently reinterpreted"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records js_sys stand-ins for a WebGPU pass encoder"
)]
fn an_indexed_draw_forwards_all_five_arguments_in_spec_order() {
    let subject: WebGpuRenderer = renderer(false);
    let before: u32 = mark();

    subject.draw_indexed(&pass().into(), &indexed());

    assert_eq!(count_on("pass", "drawIndexed", before), 1);
    let args: Array = args_on("pass", "drawIndexed", before);
    assert_eq!(
        args.length(),
        5,
        "drawIndexed takes five positional arguments"
    );
    assert_eq!(args.get(0).as_f64(), Some(36.0), "index count comes first");
    assert_eq!(args.get(1).as_f64(), Some(1.0), "then the instance count");
    assert_eq!(args.get(2).as_f64(), Some(0.0), "then the first index");
    assert_eq!(
        args.get(3).as_f64(),
        Some(4.0),
        "then the base vertex, which is not the same as the first index"
    );
    assert_eq!(
        args.get(4).as_f64(),
        Some(0.0),
        "and the first instance last"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records js_sys stand-ins for a WebGPU pass encoder"
)]
fn creating_a_uniform_buffer_reaches_the_device() {
    let subject: WebGpuRenderer = renderer(false);
    let before: u32 = mark();

    let created: JsValue = subject.create_uniform_buffer(&[0.0; 16]);

    assert!(
        !created.is_undefined(),
        "the device handed back a handle, so the buffer exists"
    );
    assert_eq!(
        count_on("device", "createBuffer", before),
        1,
        "uniform buffers are created on the device like any other resource"
    );
}
