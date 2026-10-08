use super::*;

fn blank_canvas() -> Object {
    let canvas: Object = Object::new();
    for key in ["width", "height", "clientWidth", "clientHeight"] {
        let _: Result<bool, JsValue> = Reflect::set(
            canvas.as_ref(),
            &JsValue::from_str(key),
            &JsValue::from_f64(0.0),
        );
    }
    canvas
}

fn renderer_with(canvas: Object) -> WebGpuRenderer {
    WebGpuRenderer::new(WebGpuRendererInit {
        device: device().into(),
        queue: queue().into(),
        context: context().into(),
        canvas: canvas.unchecked_into(),
        format: String::from("bgra8unorm"),
        width: 800,
        height: 600,
        antialias: false,
    })
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records js_sys stand-ins for a WebGPU device, which only exist under wasm"
)]
fn the_pending_error_slot_starts_empty_and_taking_twice_is_not_an_error() {
    let subject: WebGpuRenderer = renderer(false);

    assert!(
        subject.take_last_error().is_none(),
        "nothing has pushed an error scope on a fresh renderer"
    );
    assert!(
        subject.take_last_error().is_none(),
        "and taking again is still nothing, because the first take drained the slot rather \
         than leaving a stale value behind"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records js_sys stand-ins for a WebGPU device, which only exist under wasm"
)]
fn a_device_whose_lost_is_not_a_promise_is_skipped_rather_than_panicking() {
    let mut subject: WebGpuRenderer = renderer(false);
    let callback: js_sys::Function = js_sys::Function::new_no_args("");

    subject.on_device_lost(callback);

    assert!(
        subject.get_device_lost_callback().is_none(),
        "there is no GPUDeviceLostInfo to hand on when lost is not a promise, so no handler is \
         registered and the renderer must not claim one exists"
    );
    assert!(
        !subject.get_device_lost(),
        "and the renderer is certainly not marked lost just because registration was skipped"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records js_sys stand-ins for a WebGPU device, which only exist under wasm"
)]
fn disposing_releases_the_swap_chain_before_the_device() {
    let subject: WebGpuRenderer = renderer(false);
    let before: u32 = mark();

    subject.dispose();

    let order: Vec<String> = ops_from(before);
    let unconfigure_at: usize = order
        .iter()
        .position(|name: &String| name == "context.unconfigure")
        .unwrap_or(usize::MAX);
    let destroy_at: usize = order
        .iter()
        .position(|name: &String| name == "device.destroy")
        .unwrap_or(usize::MAX);
    assert_ne!(
        unconfigure_at,
        usize::MAX,
        "the swap chain has to be released"
    );
    assert_ne!(destroy_at, usize::MAX, "and the device has to be destroyed");
    assert!(
        unconfigure_at < destroy_at,
        "the documented teardown order is unconfigure first: destroying the device first leaves \
         the canvas context holding a dead handle"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records js_sys stand-ins for a WebGPU device, which only exist under wasm"
)]
fn a_hidden_canvas_whose_layout_box_has_collapsed_is_left_alone() {
    let mut subject: WebGpuRenderer = renderer_with(blank_canvas());
    let before: u32 = mark();

    let synced: bool = subject.sync_to_current_canvas();

    assert!(
        !synced,
        "a canvas with no backing store and no layout box has no size to sync to"
    );
    assert_eq!(
        ops_from(before),
        Vec::<String>::new(),
        "and it must not be resized to zero, which would blank the canvas and make the page \
         unrecoverable without a full re-init"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records js_sys stand-ins for a WebGPU device, which only exist under wasm"
)]
fn syncing_a_sized_canvas_reconfigures_the_swap_chain_at_that_size() {
    let mut subject: WebGpuRenderer = renderer(false);
    let before: u32 = mark();

    let synced: bool = subject.sync_to_current_canvas();

    assert!(synced, "a canvas with a real layout box syncs");
    let configured: Array = args_on("context", "configure", before);
    assert_eq!(
        field(&configured.get(0), &["format"]).as_string(),
        Some(String::from("bgra8unorm")),
        "the swap chain is reconfigured with the format this renderer negotiated"
    );
    let written: Vec<String> = ops_from(before)
        .into_iter()
        .filter(|name: &String| name.starts_with("canvas."))
        .collect();
    assert_eq!(
        written,
        vec![String::from("canvas.width"), String::from("canvas.height")],
        "and the backing store is resized to the physical size before the reconfigure"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records js_sys stand-ins for a WebGPU device, which only exist under wasm"
)]
fn a_timestamp_query_set_asks_for_the_timestamp_type_and_the_count_it_was_given() {
    let subject: WebGpuRenderer = renderer(false);
    let before: u32 = mark();

    let created: JsValue = subject.create_timestamp_query_set(2);

    assert!(
        !created.is_undefined(),
        "the driver handed back a query set"
    );
    let args: Array = args_on("device", "createQuerySet", before);
    assert_eq!(
        field(&args.get(0), &["type"]).as_string(),
        Some(String::from("timestamp")),
        "the set is the timestamp kind; occlusivity would need a different query type entirely"
    );
    assert_eq!(
        field(&args.get(0), &["count"]).as_f64(),
        Some(2.0),
        "and it holds exactly the number of slots the caller asked for"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records js_sys stand-ins for a WebGPU device, which only exist under wasm"
)]
fn a_timestamp_write_without_a_query_set_is_skipped() {
    let subject: WebGpuRenderer = renderer(false);
    let before: u32 = mark();

    subject.write_timestamp(&node("pass").into(), &JsValue::UNDEFINED, 0);

    assert_eq!(
        count_on("pass", "writeTimestamp", before),
        0,
        "recording against no query set would fault in the driver and poison the whole command \
         buffer, so the call is dropped instead"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records js_sys stand-ins for a WebGPU device, which only exist under wasm"
)]
fn resolving_a_timestamp_range_forwards_all_six_arguments_in_spec_order() {
    let subject: WebGpuRenderer = renderer(false);
    let before: u32 = mark();
    let encoder: Object = node("encoder");

    subject.resolve_timestamp(
        &encoder.clone().into(),
        &node("query-set").into(),
        0,
        2,
        &node("buffer").into(),
        0,
    );

    let args: Array = args_on("encoder", "resolveQuerySet", before);
    assert_eq!(
        args.length(),
        5,
        "resolveQuerySet takes the set, a first index, a count, a destination and a byte offset"
    );
    assert_eq!(args.get(1).as_f64(), Some(0.0), "first query");
    assert_eq!(args.get(2).as_f64(), Some(2.0), "query count");
    assert_eq!(
        args.get(4).as_f64(),
        Some(0.0),
        "and the destination offset, which is a byte offset rather than an element index"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records js_sys stand-ins for a WebGPU device, which only exist under wasm"
)]
fn resolving_without_a_destination_buffer_is_skipped() {
    let subject: WebGpuRenderer = renderer(false);
    let before: u32 = mark();

    subject.resolve_timestamp(
        &node("encoder").into(),
        &node("query-set").into(),
        0,
        2,
        &JsValue::UNDEFINED,
        0,
    );

    assert_eq!(
        count_on("encoder", "resolveQuerySet", before),
        0,
        "there is nowhere to put the resolved nanoseconds, so nothing is encoded"
    );
}
