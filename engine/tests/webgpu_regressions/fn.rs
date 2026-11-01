use super::*;

fn recording_renderer() -> WebGpuRenderer {
    let build: Function = Function::new_no_args(
        "return { queue: { calls: [], writeTexture: function() { this.calls.push(Array.from(arguments)); } }, createBindGroup: function(descriptor) { this.bindGroupDescriptor = descriptor; return {}; }, pushErrorScope: function() {}, popErrorScope: function() { return null; } };",
    );
    let device: JsValue = build.call0(&JsValue::NULL).expect("a recording device");
    WebGpuRenderer::new(WebGpuRendererInit {
        queue: field(&device, &["queue"]),
        device,
        context: Object::new().into(),
        canvas: Object::new().unchecked_into(),
        format: String::from("rgba8unorm"),
        width: 1,
        height: 1,
        antialias: false,
    })
}

fn set_field(object: &JsValue, name: &str, value: &JsValue) {
    assert!(Reflect::set(object, &JsValue::from_str(name), value).expect("a writable mock field"));
}

fn texture_with_extent(width: u32, height: u32, depth: u32, dimension: &str) -> JsValue {
    let texture: JsValue = Object::new().into();
    let maximum_dimension: u32 = if dimension == "3d" {
        width.max(height).max(depth)
    } else {
        width.max(height)
    };
    let mip_level_count: u32 = (u32::BITS - maximum_dimension.leading_zeros()).max(1);
    for (name, value) in [
        ("width", width),
        ("height", height),
        ("depthOrArrayLayers", depth),
        ("mipLevelCount", mip_level_count),
    ] {
        set_field(&texture, name, &JsValue::from_f64(f64::from(value)));
    }
    set_field(&texture, "dimension", &JsValue::from_str(dimension));
    texture
}

fn recorded_uploads(subject: &WebGpuRenderer) -> Array {
    field(subject.get_device(), &["queue", "calls"]).unchecked_into()
}

fn assert_extent(arguments: &Array, expected: [u32; 3]) {
    for (index, name) in ["width", "height", "depthOrArrayLayers"].iter().enumerate() {
        assert_eq!(
            field(&arguments.get(3), &[name]).as_f64(),
            Some(f64::from(expected[index])),
            "the {name} is a texel extent, not a source-layout stride"
        );
    }
}

fn assert_direct_bind_resources(explicit_layout: bool) {
    let subject: WebGpuRenderer = recording_renderer();
    let texture: JsValue = Object::new().into();
    let storage: JsValue = Object::new().into();
    let read_only_storage: JsValue = Object::new().into();
    let sampler: JsValue = Object::new().into();
    let buffer: JsValue = Object::new().into();
    let layout: JsValue = Object::new().into();
    let pipeline: JsValue = Object::new().into();
    let get_layout: Function =
        Function::new_with_args("index", "this.requestedIndex = index; return this.layout;");
    set_field(&pipeline, "getBindGroupLayout", get_layout.as_ref());
    set_field(&pipeline, "layout", &layout);
    let entries: [BindGroupEntry; 6] = [
        BindGroupEntry::Texture {
            binding: 2,
            view: texture.clone(),
        },
        BindGroupEntry::StorageTexture {
            binding: 4,
            view: storage.clone(),
            read_only: false,
        },
        BindGroupEntry::StorageTexture {
            binding: 6,
            view: read_only_storage.clone(),
            read_only: true,
        },
        BindGroupEntry::Sampler {
            binding: 8,
            sampler: sampler.clone(),
        },
        BindGroupEntry::Buffer {
            binding: 10,
            buffer: buffer.clone(),
            offset: 32,
            size: Some(64),
        },
        BindGroupEntry::Buffer {
            binding: 12,
            buffer: buffer.clone(),
            offset: 96,
            size: None,
        },
    ];
    let created: JsValue = if explicit_layout {
        subject.create_bind_group_for_layout(&layout, &entries)
    } else {
        subject.create_bind_group(&pipeline, 3, &entries)
    };
    assert!(created.is_object(), "the mock returned a bind group");
    let descriptor: JsValue = field(subject.get_device(), &["bindGroupDescriptor"]);
    assert!(Object::is(&field(&descriptor, &["layout"]), &layout));
    let wire_entries: Array = field(&descriptor, &["entries"]).unchecked_into();
    assert_eq!(wire_entries.length(), 6);
    for (index, expected) in [texture, storage, read_only_storage, sampler]
        .iter()
        .enumerate()
    {
        let entry: JsValue = wire_entries.get(index as u32);
        let resource: JsValue = field(&entry, &["resource"]);
        assert!(
            Object::is(&resource, expected),
            "texture views and samplers are the resource itself, not wrapper dictionaries"
        );
        assert!(field(&resource, &["textureView"]).is_undefined());
        assert!(field(&resource, &["sampler"]).is_undefined());
        assert_eq!(
            field(&entry, &["binding"]).as_f64(),
            Some(((index + 1) * 2) as f64)
        );
    }
    let sized: JsValue = field(&wire_entries.get(4), &["resource"]);
    assert!(
        !Object::is(&sized, &buffer),
        "buffers retain their range dictionary"
    );
    assert!(Object::is(&field(&sized, &["buffer"]), &buffer));
    assert_eq!(field(&sized, &["offset"]).as_f64(), Some(32.0));
    assert_eq!(field(&sized, &["size"]).as_f64(), Some(64.0));
    let unsized_buffer: JsValue = field(&wire_entries.get(5), &["resource"]);
    assert!(Object::is(&field(&unsized_buffer, &["buffer"]), &buffer));
    assert_eq!(field(&unsized_buffer, &["offset"]).as_f64(), Some(96.0));
    assert!(field(&unsized_buffer, &["size"]).is_undefined());
    assert_eq!(
        field(&pipeline, &["requestedIndex"]).as_f64(),
        if explicit_layout { None } else { Some(3.0) }
    );
}

#[wasm_bindgen_test::wasm_bindgen_test]
fn webgpu_regression_pipeline_bindings_preserve_direct_resource_identity() {
    assert_direct_bind_resources(false);
}

#[wasm_bindgen_test::wasm_bindgen_test]
fn webgpu_regression_explicit_layout_bindings_preserve_direct_resource_identity() {
    assert_direct_bind_resources(true);
}

#[wasm_bindgen_test::wasm_bindgen_test]
fn webgpu_regression_rgba_upload_uses_texture_texels_not_byte_stride() {
    let subject: WebGpuRenderer = recording_renderer();
    let texture: JsValue = texture_with_extent(524, 384, 1, "2d");
    let descriptor: TextureWriteDescriptor =
        TextureWriteDescriptor::for_2d(vec![42; 2096 * 384], 2096, texture.clone());
    subject.write_texture(&descriptor);
    let uploads: Array = recorded_uploads(&subject);
    assert_eq!(uploads.length(), 1);
    let arguments: Array = uploads.get(0).unchecked_into();
    assert_extent(&arguments, [524, 384, 1]);
    assert!(Object::is(
        &field(&arguments.get(0), &["texture"]),
        &texture
    ));
    assert_eq!(field(&arguments.get(0), &["mipLevel"]).as_f64(), Some(0.0));
    assert!(field(&arguments.get(0), &["origin"]).is_undefined());
    assert_eq!(
        field(&arguments.get(2), &["bytesPerRow"]).as_f64(),
        Some(2096.0)
    );
    assert!(field(&arguments.get(2), &["rowsPerImage"]).is_undefined());
    assert_eq!(field(&arguments.get(2), &["offset"]).as_f64(), Some(0.0));
    let bytes: Uint8Array = arguments.get(1).unchecked_into();
    assert_eq!(bytes.length(), 2096 * 384);
    assert_eq!(bytes.get_index(0), 42);
    assert_eq!(bytes.get_index(bytes.length() - 1), 42);
}

#[wasm_bindgen_test::wasm_bindgen_test]
fn webgpu_regression_upload_extent_does_not_include_source_padding() {
    let subject: WebGpuRenderer = recording_renderer();
    let descriptor: TextureWriteDescriptor = TextureWriteDescriptor::new(
        vec![17; 32 * 5],
        32,
        5,
        0,
        texture_with_extent(3, 2, 1, "2d"),
        None,
        false,
    );
    subject.write_texture(&descriptor);
    let uploads: Array = recorded_uploads(&subject);
    assert_eq!(uploads.length(), 1);
    let arguments: Array = uploads.get(0).unchecked_into();
    assert_extent(&arguments, [3, 2, 1]);
    assert_eq!(
        field(&arguments.get(2), &["bytesPerRow"]).as_f64(),
        Some(32.0)
    );
    assert_eq!(
        field(&arguments.get(2), &["rowsPerImage"]).as_f64(),
        Some(5.0)
    );
}

#[wasm_bindgen_test::wasm_bindgen_test]
fn webgpu_regression_volume_upload_shrinks_depth_at_the_selected_mip() {
    let subject: WebGpuRenderer = recording_renderer();
    let origin: JsValue = Object::new().into();
    for (name, value) in [("x", 3), ("y", 2), ("z", 1)] {
        set_field(&origin, name, &JsValue::from_f64(f64::from(value)));
    }
    let descriptor: TextureWriteDescriptor = TextureWriteDescriptor::new(
        vec![0; 64 * 9 * 3],
        64,
        9,
        1,
        texture_with_extent(32, 16, 8, "3d"),
        Some(origin.clone()),
        false,
    );
    subject.write_texture(&descriptor);
    let uploads: Array = recorded_uploads(&subject);
    assert_eq!(uploads.length(), 1);
    let arguments: Array = uploads.get(0).unchecked_into();
    assert_extent(&arguments, [13, 6, 3]);
    assert!(Object::is(&field(&arguments.get(0), &["origin"]), &origin));
    assert_eq!(field(&arguments.get(0), &["mipLevel"]).as_f64(), Some(1.0));
}

#[wasm_bindgen_test::wasm_bindgen_test]
fn webgpu_regression_array_layers_do_not_shrink_with_mips() {
    let subject: WebGpuRenderer = recording_renderer();
    let origin: Array = Array::new();
    for value in [1, 1, 2] {
        origin.push(&JsValue::from_f64(f64::from(value)));
    }
    let descriptor: TextureWriteDescriptor = TextureWriteDescriptor::new(
        vec![0; 32 * 3 * 6],
        32,
        3,
        2,
        texture_with_extent(32, 16, 8, "2d"),
        Some(origin.clone().into()),
        false,
    );
    subject.write_texture(&descriptor);
    let uploads: Array = recorded_uploads(&subject);
    assert_eq!(uploads.length(), 1);
    let arguments: Array = uploads.get(0).unchecked_into();
    assert_extent(&arguments, [7, 3, 6]);
    assert!(Object::is(
        &field(&arguments.get(0), &["origin"]),
        origin.as_ref()
    ));
}

#[wasm_bindgen_test::wasm_bindgen_test]
fn webgpu_regression_small_mips_keep_a_single_texel_extent() {
    let subject: WebGpuRenderer = recording_renderer();
    let descriptor: TextureWriteDescriptor = TextureWriteDescriptor::new(
        vec![0; 4],
        0,
        0,
        2,
        texture_with_extent(7, 1, 1, "1d"),
        None,
        false,
    );
    subject.write_texture(&descriptor);
    let uploads: Array = recorded_uploads(&subject);
    assert_eq!(uploads.length(), 1);
    let arguments: Array = uploads.get(0).unchecked_into();
    assert_extent(&arguments, [1, 1, 1]);
    assert!(field(&arguments.get(2), &["bytesPerRow"]).is_undefined());
    assert!(field(&arguments.get(2), &["rowsPerImage"]).is_undefined());
}

#[wasm_bindgen_test::wasm_bindgen_test]
fn webgpu_regression_invalid_upload_bounds_never_reach_the_queue() {
    let subject: WebGpuRenderer = recording_renderer();
    for (width, height, depth, mip, origin_x, origin_y, origin_z) in [
        (0, 2, 1, 0, 0.0, 0.0, 0.0),
        (3, 0, 1, 0, 0.0, 0.0, 0.0),
        (3, 2, 0, 0, 0.0, 0.0, 0.0),
        (3, 2, 1, 8, 0.0, 0.0, 0.0),
        (3, 2, 1, u32::MAX, 0.0, 0.0, 0.0),
        (3, 2, 1, 0, 4.0, 0.0, 0.0),
        (3, 2, 1, 0, 0.0, 3.0, 0.0),
        (3, 2, 1, 0, 0.0, 0.0, 2.0),
        (3, 2, 1, 0, -1.0, 0.0, 0.0),
        (3, 2, 1, 0, 0.5, 0.0, 0.0),
    ] {
        let origin: JsValue = Object::new().into();
        for (name, value) in [("x", origin_x), ("y", origin_y), ("z", origin_z)] {
            set_field(&origin, name, &JsValue::from_f64(value));
        }
        subject.write_texture(&TextureWriteDescriptor::new(
            vec![0; 32],
            16,
            0,
            mip,
            texture_with_extent(width, height, depth, "2d"),
            Some(origin),
            false,
        ));
    }
    subject.write_texture(&TextureWriteDescriptor::for_2d(
        vec![0; 4],
        4,
        Object::new().into(),
    ));
    assert_eq!(recorded_uploads(&subject).length(), 0);
}
