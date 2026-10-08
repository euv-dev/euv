use super::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "builds a js_sys stand-in for the device, which only exists under wasm"
)]
fn submitting_command_buffers_reaches_the_queue_with_the_whole_array() {
    let before: u32 = mark();
    let subject: WebGpuRenderer = renderer(false);

    subject.submit(&[JsValue::from_str("buffer-a"), JsValue::from_str("buffer-b")]);

    assert_eq!(
        ops_from(before),
        vec![String::from("queue.submit")],
        "one submit call, and nothing else on the queue"
    );
    let args: Array = args_on("queue", "submit", before);
    let buffers: Array = args.get(0).unchecked_into();
    assert_eq!(buffers.length(), 2, "both buffers are in that one array");
    assert_eq!(
        buffers.get(0).as_string(),
        Some(String::from("buffer-a")),
        "and they keep the order the caller gave them"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "builds a js_sys stand-in for the device, which only exists under wasm"
)]
fn a_single_command_buffer_still_goes_across_as_an_array() {
    let before: u32 = mark();
    let subject: WebGpuRenderer = renderer(false);

    subject.submit(&[JsValue::from_str("only")]);

    let args: Array = args_on("queue", "submit", before);
    let buffers: Array = args.get(0).unchecked_into();
    assert_eq!(
        buffers.length(),
        1,
        "the one-buffer case must not degrade into a bare buffer, or the driver sees a different \
         argument shape than the many-buffer case"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "builds a js_sys stand-in for the device, which only exists under wasm"
)]
fn creating_a_vertex_buffer_allocates_then_fills_it_before_returning() {
    let before: u32 = mark();
    let subject: WebGpuRenderer = renderer(false);

    let created: JsValue = subject.create_vertex_buffer(&[7; 64]);

    assert!(
        !created.is_undefined(),
        "a driver that hands back a handle means the buffer was created"
    );
    assert_eq!(
        ops_from(before),
        vec![
            String::from("device.createBuffer"),
            String::from("queue.writeBuffer")
        ],
        "a vertex buffer is allocated on the device and then filled through the queue, in that \
         order, before the caller sees the handle back"
    );
    let written: Array = args_on("queue", "writeBuffer", before);
    assert_eq!(
        written.get(1).as_f64(),
        Some(0.0),
        "the payload is written from the start of the buffer"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "builds a js_sys stand-in for the device, which only exists under wasm"
)]
fn the_internal_caches_start_empty_on_a_freshly_built_renderer() {
    let subject: WebGpuRenderer = renderer(false);

    assert!(
        subject.get_multisample_texture().is_none(),
        "no multisample texture until one is created, and a skipped field defaults to None"
    );
    assert!(
        subject.get_command_encoder().is_none(),
        "no command encoder is open on a fresh renderer"
    );
    assert!(
        !subject.get_device_lost(),
        "and the device is not reported lost before anything has happened to it"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "builds a js_sys stand-in for the device, which only exists under wasm"
)]
fn a_fresh_renderer_keeps_the_constructor_arguments_it_was_built_with() {
    let subject: WebGpuRenderer = renderer(false);

    assert_eq!(subject.get_width(), 800, "the width it was built with");
    assert_eq!(subject.get_height(), 600, "and the height");
    assert_eq!(
        subject.get_format(),
        "bgra8unorm",
        "and the swap chain format the caller negotiated"
    );
    assert!(!subject.get_antialias(), "and whether multisampling is on");
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "builds a js_sys stand-in for the device, which only exists under wasm"
)]
fn the_device_and_queue_the_renderer_holds_are_the_ones_it_was_given() {
    let subject: WebGpuRenderer = renderer(false);

    assert!(
        js_sys::Object::is(subject.get_device(), &device().into()),
        "a renderer built around a stand-in device still reports that same stand-in, so a caller \
         can line its own objects up with the renderer's"
    );
    assert!(
        js_sys::Object::is(subject.get_queue(), &queue().into()),
        "and the same holds for the queue"
    );
}
