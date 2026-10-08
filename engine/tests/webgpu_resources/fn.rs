use super::*;

fn sized_2d() -> Texture2DDescriptor {
    Texture2DDescriptor::new(64, 32, GpuTextureFormat::Rgba8Unorm)
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records js_sys stand-ins for a WebGPU device, which only exist under wasm"
)]
fn a_two_d_texture_reaches_the_device_with_its_own_extent() {
    let subject: WebGpuRenderer = renderer(false);
    let before: u32 = mark();

    let created: JsValue = subject.create_texture_2d(&sized_2d());

    assert!(
        !created.is_undefined(),
        "a driver handle means the texture was allocated"
    );
    assert_eq!(
        ops_from(before),
        vec![String::from("device.createTexture")],
        "creating a 2D texture is one call and nothing else"
    );
    let args: Array = args_on("device", "createTexture", before);
    let descriptor: JsValue = args.get(0);
    assert_eq!(
        field(&descriptor, &["size", "width"]).as_f64(),
        Some(64.0),
        "the descriptor's own width reaches the extent, not a default"
    );
    assert_eq!(
        field(&descriptor, &["size", "height"]).as_f64(),
        Some(32.0),
        "and so does the height"
    );
    assert_eq!(
        field(&descriptor, &["size", "depthOrArrayLayers"]).as_f64(),
        Some(1.0),
        "a plain 2D texture is one layer deep even when the descriptor never said so"
    );
    assert_eq!(
        field(&descriptor, &["dimension"]).as_string(),
        Some(String::from("2d")),
        "and the narrow 2D preset always asks for the 2d dimension"
    );
    assert_eq!(
        field(&descriptor, &["format"]).as_string(),
        Some(String::from("rgba8unorm")),
        "the format enum becomes the spec's format string"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records js_sys stand-ins for a WebGPU device, which only exist under wasm"
)]
fn a_texture_with_no_width_never_reaches_the_driver() {
    let subject: WebGpuRenderer = renderer(false);
    let before: u32 = mark();

    let created: JsValue = subject.create_texture_2d(&Texture2DDescriptor::new(
        0,
        32,
        GpuTextureFormat::Rgba8Unorm,
    ));

    assert!(
        created.is_undefined(),
        "a zero extent has no meaning to the driver, so the caller gets the failure marker"
    );
    assert_eq!(
        count_on("device", "createTexture", before),
        0,
        "and the zero-extent descriptor must not be forwarded, or the driver raises a \
         validation error nobody in this engine reads"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records js_sys stand-ins for a WebGPU device, which only exist under wasm"
)]
fn a_general_texture_forwards_the_dimensionality_it_was_given() {
    let subject: WebGpuRenderer = renderer(false);
    let before: u32 = mark();
    let mut descriptor: TextureDescriptor =
        TextureDescriptor::new(8, 8, "3d", GpuTextureFormat::Rgba8Unorm, 0);
    descriptor.set_depth_or_layers(4);

    subject.create_texture(&descriptor);

    let args: Array = args_on("device", "createTexture", before);
    let wire: JsValue = args.get(0);
    assert_eq!(
        field(&wire, &["dimension"]).as_string(),
        Some(String::from("3d")),
        "the general constructor is the one that can ask for something other than 2d"
    );
    assert_eq!(
        field(&wire, &["size", "depthOrArrayLayers"]).as_f64(),
        Some(4.0),
        "and the depth it was set to travels in the extent rather than being flattened to 1"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records js_sys stand-ins for a WebGPU device, which only exist under wasm"
)]
fn a_sampler_carries_its_filter_mipmap_and_wrap_decisions_across() {
    let subject: WebGpuRenderer = renderer(false);
    let before: u32 = mark();
    let mut descriptor: SamplerDescriptor = SamplerDescriptor::new();
    descriptor.set_filter(FilterMode::Nearest);
    descriptor.set_mipmap_filter(MipmapFilter::Nearest);
    descriptor.set_address_mode_u(AddressMode::MirrorRepeat);

    subject.create_sampler(&descriptor);

    let args: Array = args_on("device", "createSampler", before);
    let wire: JsValue = args.get(0);
    assert_eq!(
        field(&wire, &["magFilter"]).as_string(),
        Some(String::from("nearest")),
        "the magnification filter is the one the descriptor named"
    );
    assert_eq!(
        field(&wire, &["minFilter"]).as_string(),
        Some(String::from("nearest")),
        "the minification filter travels with it, since the descriptor has one field for both"
    );
    assert_eq!(
        field(&wire, &["mipmapFilter"]).as_string(),
        Some(String::from("nearest")),
        "and mip blending is a separate decision, so it must not inherit the filter"
    );
    assert_eq!(
        field(&wire, &["addressModeU"]).as_string(),
        Some(String::from("mirror-repeat")),
        "wrapping on U is its own field rather than a shared default for all three axes"
    );
    assert!(
        field(&wire, &["compare"]).is_undefined(),
        "a sampler with no depth comparison must omit the compare key entirely, because WebGPU \
         reads a present-but-null compare as a request to enable one"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records js_sys stand-ins for a WebGPU device, which only exist under wasm"
)]
fn an_index_buffer_is_allocated_at_the_payload_size_then_filled_from_zero() {
    let subject: WebGpuRenderer = renderer(false);
    let before: u32 = mark();

    let created: JsValue = subject.create_index_buffer(&[1, 2, 3, 4, 5, 6]);

    assert!(!created.is_undefined(), "the driver handed back a handle");
    assert_eq!(
        ops_from(before),
        vec![
            String::from("device.createBuffer"),
            String::from("queue.writeBuffer")
        ],
        "allocate then fill, in that order, so the caller never sees an unwritten index buffer"
    );
    let allocated: Array = args_on("device", "createBuffer", before);
    assert_eq!(
        field(&allocated.get(0), &["size"]).as_f64(),
        Some(6.0),
        "the buffer is exactly the payload length, not a rounded-up guess"
    );
    let written: Array = args_on("queue", "writeBuffer", before);
    assert_eq!(
        written.get(1).as_f64(),
        Some(0.0),
        "the payload lands at the start of the buffer"
    );
    assert_eq!(
        written.get(3).as_f64(),
        Some(6.0),
        "and its byte length travels as the fourth argument, not as a view length the driver \
         has to re-derive"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records js_sys stand-ins for a WebGPU device, which only exist under wasm"
)]
fn writing_no_bytes_is_skipped_entirely_rather_than_uploading_an_empty_view() {
    let subject: WebGpuRenderer = renderer(false);
    let before: u32 = mark();

    subject.write_buffer(&node("buffer").into(), 0, &[]);

    assert_eq!(
        count_on("queue", "writeBuffer", before),
        0,
        "an empty upload would still take the driver's lock and hand it a zero-length view"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records js_sys stand-ins for a WebGPU device, which only exist under wasm"
)]
fn a_byte_upload_honours_the_offset_the_caller_named() {
    let subject: WebGpuRenderer = renderer(false);
    let before: u32 = mark();

    subject.write_buffer(&node("buffer").into(), 64, &[1, 2, 3, 4]);

    let written: Array = args_on("queue", "writeBuffer", before);
    assert_eq!(
        written.get(1).as_f64(),
        Some(64.0),
        "writing at a non-zero offset must not be silently rewritten to the buffer head"
    );
    assert_eq!(
        written.get(3).as_f64(),
        Some(4.0),
        "the fourth argument is this call's length, not the buffer's"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records js_sys stand-ins for a WebGPU device, which only exist under wasm"
)]
fn a_uniform_bind_group_takes_group_zero_of_the_pipeline_it_was_given() {
    let subject: WebGpuRenderer = renderer(false);
    let before: u32 = mark();
    let pipeline: Object = node("pipeline");

    subject.create_uniform_bind_group(&pipeline.clone().into(), &node("buffer").into());

    let layout: Array = args_on("pipeline", "getBindGroupLayout", before);
    assert_eq!(
        layout.get(0).as_f64(),
        Some(0.0),
        "the uniform convenience path is documented as group 0, and it must actually ask for 0"
    );
    let created: Array = args_on("device", "createBindGroup", before);
    let descriptor: JsValue = created.get(0);
    let entries: Array = field(&descriptor, &["entries"]).unchecked_into();
    assert_eq!(
        entries.length(),
        1,
        "one binding is enough for the uniform-only path"
    );
    assert_eq!(
        field(&entries.get(0), &["binding"]).as_f64(),
        Some(0.0),
        "and it fills binding 0"
    );
    assert_eq!(
        field(&entries.get(0), &["resource", "buffer"]).as_string(),
        Some(String::from("<gpu>")),
        "the buffer handle really did travel into the resource dict; the recorder renders a \
         stand-in handle as a marker rather than recursing into the proxy"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records js_sys stand-ins for a WebGPU device, which only exist under wasm"
)]
fn a_buffer_binding_with_no_size_omits_the_size_key() {
    let subject: WebGpuRenderer = renderer(false);
    let before: u32 = mark();

    subject.create_bind_group(
        &node("pipeline").into(),
        1,
        &[BindGroupEntry::Buffer {
            binding: 3,
            buffer: node("buffer").into(),
            offset: 32,
            size: None,
        }],
    );

    let created: Array = args_on("device", "createBindGroup", before);
    let entries: Array = field(&created.get(0), &["entries"]).unchecked_into();
    assert_eq!(
        field(&entries.get(0), &["binding"]).as_f64(),
        Some(3.0),
        "the caller's binding slot is forwarded verbatim, not renumbered from zero"
    );
    assert_eq!(
        field(&entries.get(0), &["resource", "offset"]).as_f64(),
        Some(32.0),
        "and the buffer offset inside the resource dict is the one the caller gave"
    );
    assert!(
        field(&entries.get(0), &["resource", "size"]).is_undefined(),
        "a bound size of None must leave the key out; sending size: 0 would bind a zero-length \
         range and the driver would reject it"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records js_sys stand-ins for a WebGPU device, which only exist under wasm"
)]
fn a_bind_group_is_created_inside_a_validation_error_scope() {
    let subject: WebGpuRenderer = renderer(false);
    let before: u32 = mark();

    subject.create_bind_group(&node("pipeline").into(), 0, &[]);

    let pushed: Array = args_on("device", "pushErrorScope", before);
    assert_eq!(
        pushed.get(0).as_string(),
        Some(String::from("validation")),
        "a bad bind group is a validation error, so the scope has to be the validation filter"
    );
    let order: Vec<String> = ops_from(before);
    let push_at: usize = order
        .iter()
        .position(|name: &String| name == "device.pushErrorScope")
        .unwrap_or(usize::MAX);
    let create_at: usize = order
        .iter()
        .position(|name: &String| name == "device.createBindGroup")
        .unwrap_or(0);
    assert!(
        push_at < create_at,
        "the scope has to open before the call it is meant to catch, not after"
    );
    assert_eq!(
        count_on("device", "popErrorScope", before),
        1,
        "and it has to be popped, or every later scope in the frame stacks onto this one"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records js_sys stand-ins for a WebGPU device, which only exist under wasm"
)]
fn a_bind_group_layout_turns_each_entry_into_its_spec_slot() {
    let subject: WebGpuRenderer = renderer(false);
    let before: u32 = mark();

    subject.create_bind_group_layout(&[
        BindGroupLayoutEntry::uniform(0, ShaderStage::Vertex | ShaderStage::Fragment),
        BindGroupLayoutEntry::storage(1, ShaderStage::Compute, true),
    ]);

    let args: Array = args_on("device", "createBindGroupLayout", before);
    let entries: Array = field(&args.get(0), &["entries"]).unchecked_into();
    assert_eq!(entries.length(), 2, "one spec slot per layout entry");
    assert_eq!(
        field(&entries.get(0), &["binding"]).as_f64(),
        Some(0.0),
        "the first entry keeps its own binding index"
    );
    assert_eq!(
        field(&entries.get(0), &["visibility"]).as_f64(),
        Some(3.0),
        "vertex and fragment together are 0x1 | 0x2, not one of them alone"
    );
    assert_eq!(
        field(&entries.get(0), &["buffer", "type"]).as_string(),
        Some(String::from("uniform")),
        "the entry kind becomes the spec's buffer binding type string"
    );
    assert_eq!(
        field(&entries.get(1), &["buffer", "type"]).as_string(),
        Some(String::from("read-only-storage")),
        "a read-only storage buffer is its own binding type, not a uniform in disguise"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records js_sys stand-ins for a WebGPU device, which only exist under wasm"
)]
fn an_explicit_layout_skips_the_pipeline_derivation_entirely() {
    let subject: WebGpuRenderer = renderer(false);
    let before: u32 = mark();

    subject.create_bind_group_for_layout(&node("layout").into(), &[]);

    assert_eq!(
        count_on("pipeline", "getBindGroupLayout", before),
        0,
        "this entry point exists precisely so a caller can bind a layout it built itself, with \
         no pipeline in hand to derive one from"
    );
    assert_eq!(
        count_on("device", "createBindGroup", before),
        1,
        "and it still creates the bind group"
    );
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test::wasm_bindgen_test]
async fn reading_a_value_that_is_not_a_buffer_yields_nothing_rather_than_a_zero_length_read() {
    let subject: WebGpuRenderer = renderer(false);

    let bytes: Option<Vec<u8>> = subject.read_buffer(&JsValue::UNDEFINED, 0, 16).await;

    assert!(
        bytes.is_none(),
        "a value with no mapAsync is not a GPUBuffer; returning an empty Vec would read as \
         the buffer having contained zero bytes, and a caller checking length would accept a \
         failed read as a successful empty one"
    );
}
