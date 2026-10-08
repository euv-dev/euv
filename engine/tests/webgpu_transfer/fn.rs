use super::*;

fn write_request(texture: Object) -> TextureWriteDescriptor {
    TextureWriteDescriptor::new(
        vec![1u8, 2, 3, 4, 5, 6, 7, 8],
        256,
        1,
        0,
        texture.into(),
        None,
        false,
    )
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records js_sys stand-ins for a WebGPU device, which only exist under wasm"
)]
fn an_offline_target_comes_back_as_a_texture_and_its_default_view() {
    let subject: WebGpuRenderer = renderer(false);
    let before: u32 = mark();

    let (texture, view): (JsValue, JsValue) =
        subject.create_offline_render_target(256, 128, "rgba8unorm");

    assert!(
        !texture.is_undefined() && !view.is_undefined(),
        "both halves of the pair have to exist, or the caller gets a tuple it cannot destructure \
         into anything usable"
    );
    let args: Array = args_on("device", "createTexture", before);
    let descriptor: JsValue = args.get(0);
    let size: Array = field(&descriptor, &["size"]).unchecked_into();
    assert_eq!(
        size.length(),
        3,
        "an extent is always a three-number triple"
    );
    assert_eq!(size.get(0).as_f64(), Some(256.0), "width");
    assert_eq!(size.get(1).as_f64(), Some(128.0), "height");
    assert_eq!(
        size.get(2).as_f64(),
        Some(1.0),
        "and a single layer, since this is the 2D offscreen target preset"
    );
    assert_eq!(
        field(&descriptor, &["usage"]).as_f64(),
        Some(21.0),
        "usage crosses the wire as the numeric GPUTextureUsage bitmask - COPY_SRC 0x01 | \
         TEXTURE_BINDING 0x04 | RENDER_ATTACHMENT 0x10 - because a texture descriptor's usage \
         is a number, and a spelled-out mask string is what a driver rejects outright"
    );
    assert_eq!(
        count_on("device.createTexture()", "createView", before),
        1,
        "the default view is opened on the texture handle the device just handed back"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records js_sys stand-ins for a WebGPU device, which only exist under wasm"
)]
fn a_texture_view_is_opened_on_the_texture_that_was_named() {
    let subject: WebGpuRenderer = renderer(false);
    let before: u32 = mark();
    let texture: Object = node("named-texture");

    let view: JsValue = subject.create_texture_view(&texture.clone().into());

    assert!(!view.is_undefined(), "the driver handed back a view handle");
    assert_eq!(
        count_on("named-texture", "createView", before),
        1,
        "createView belongs to the texture, and the whole point of passing the texture in is \
         that this call lands on it"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records js_sys stand-ins for a WebGPU device, which only exist under wasm"
)]
fn a_storage_texture_carries_the_four_uses_its_documents() {
    let subject: WebGpuRenderer = renderer(false);
    let before: u32 = mark();

    subject.create_storage_texture(32, 32, "r32float");

    let args: Array = args_on("device", "createTexture", before);
    let descriptor: JsValue = args.get(0);
    assert_eq!(
        field(&descriptor, &["size", "width"]).as_f64(),
        Some(32.0),
        "the width this call was given"
    );
    assert_eq!(
        field(&descriptor, &["size", "depthOrArrayLayers"]).as_f64(),
        Some(1.0),
        "a storage texture is one layer deep; this call site builds its extent by hand, so it \
         has to spell the key the spec uses rather than inherit it from somewhere else"
    );
    assert_eq!(
        field(&descriptor, &["format"]).as_string(),
        Some(String::from("r32float")),
        "the format string crosses verbatim rather than going through an enum name table"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records js_sys stand-ins for a WebGPU device, which only exist under wasm"
)]
fn a_texture_readback_needs_an_open_encoder_and_is_skipped_without_one() {
    let subject: WebGpuRenderer = renderer(false);
    let before: u32 = mark();

    subject.copy_texture_to_buffer(&node("texture").into(), &node("buffer").into(), 256, 64, 64);

    assert_eq!(
        ops_from(before),
        Vec::<String>::new(),
        "a freshly built renderer has no command encoder, and the copy has to be skipped \
         rather than encoded into nothing"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records js_sys stand-ins for a WebGPU device, which only exist under wasm"
)]
fn a_texture_upload_goes_to_the_queue_off_the_device() {
    let subject: WebGpuRenderer = renderer(false);
    let before: u32 = mark();
    let texture: Object = node("texture");

    subject.write_texture(&write_request(texture.clone()));

    assert_eq!(
        count_on("device.queue", "writeTexture", before),
        1,
        "writeTexture lives on the queue, which hangs off the device rather than off the \
         renderer's own queue field"
    );
    assert_eq!(
        count_on("queue", "writeTexture", before),
        0,
        "and it must not be confused with the renderer's cached queue, which has no writeTexture"
    );
    let args: Array = args_on("device.queue", "writeTexture", before);
    assert_eq!(
        field(&args.get(0), &["mipLevel"]).as_f64(),
        Some(0.0),
        "the destination names the mip level it writes into"
    );
    assert_eq!(
        field(&args.get(2), &["bytesPerRow"]).as_f64(),
        Some(256.0),
        "the 256-byte row alignment travels in the data layout, not in the destination"
    );
    assert_eq!(
        field(&args.get(2), &["offset"]).as_f64(),
        Some(0.0),
        "and this call site writes from the start of the source data, under the spec's own \
         `offset` spelling for GPUImageDataLayout"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records js_sys stand-ins for a WebGPU device, which only exist under wasm"
)]
fn a_uniform_buffer_upload_sends_three_arguments_and_no_length() {
    let subject: WebGpuRenderer = renderer(false);
    let before: u32 = mark();

    subject.update_uniform_buffer(&node("buffer").into(), &[1.0, 2.0, 3.0, 4.0]);

    let args: Array = args_on("queue", "writeBuffer", before);
    assert_eq!(
        args.length(),
        3,
        "the uniform path omits the byte count and lets the driver read it off the typed array; \
         sending a fourth argument here would be a different call shape from the one the raw \
         write_buffer path uses"
    );
    assert_eq!(
        args.get(1).as_f64(),
        Some(0.0),
        "uniforms are rewritten from the head of the buffer every time"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records js_sys stand-ins for a WebGPU device, which only exist under wasm"
)]
fn a_uniform_buffer_is_rounded_up_to_the_binding_alignment() {
    let subject: WebGpuRenderer = renderer(false);
    let before: u32 = mark();

    subject.create_uniform_buffer(&[1.0, 2.0]);

    let args: Array = args_on("device", "createBuffer", before);
    assert_eq!(
        field(&args.get(0), &["size"]).as_f64(),
        Some(16.0),
        "a bare vec2 uniform is 8 bytes on the wire, but WebGPU requires a uniform binding to \
         be 16-byte aligned in size, so the allocation is rounded up rather than truncated"
    );
}
