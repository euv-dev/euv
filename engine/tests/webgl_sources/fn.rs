use super::*;

const SOURCE_NAMES: &str = r#"[
    "createTexture","bindTexture","texParameteri","deleteTexture",
    "texImage2D","pixelStorei","generateMipmap"
]"#;

fn reflective(target: &Object, key: &str, value: &JsValue) {
    let installer: js_sys::Function = js_sys::Function::new_with_args(
        "target, key, value",
        "Object.defineProperty(target, key, { value: value, writable: true, configurable: true });",
    );
    let outcome: Result<JsValue, JsValue> = installer.call3(
        &JsValue::NULL,
        target.as_ref(),
        &JsValue::from_str(key),
        value,
    );
    assert!(outcome.is_ok(), "the stand-in must accept {key}");
}

fn recording_context() -> (WebGl2RenderingContext, Array) {
    let log: Array = Array::new();
    let context: Object = Object::new();
    let arm: js_sys::Function = js_sys::Function::new_with_args(
        "target, log, names",
        "for (const name of JSON.parse(names)) { target[name] = function() { log.push([name, Array.from(arguments)]); if (name === 'createTexture') return { __gl: name }; }; }",
    );
    let armed: Result<JsValue, JsValue> = arm.call3(
        &JsValue::NULL,
        context.as_ref(),
        log.as_ref(),
        &JsValue::from_str(SOURCE_NAMES),
    );
    assert!(armed.is_ok(), "the source recorder must install cleanly");
    (context.clone().unchecked_into(), log)
}

fn calls_after(log: &Array, from: u32) -> Vec<String> {
    (from..log.length())
        .map(|index: u32| {
            let entry: Array = log.get(index).unchecked_into();
            entry.get(0).as_string().unwrap_or_default()
        })
        .collect()
}

fn flip_values(log: &Array, from: u32) -> Vec<f64> {
    (from..log.length())
        .filter_map(|index: u32| {
            let entry: Array = log.get(index).unchecked_into();
            if entry.get(0).as_string().as_deref() != Some("pixelStorei") {
                return None;
            }
            let args: Array = entry.get(1).unchecked_into();
            args.get(1).as_f64()
        })
        .collect()
}

fn counted_element(keys: &[&str], value: f64) -> HtmlImageElement {
    let element: Object = Object::new();
    for key in keys {
        reflective(element.as_ref(), key, &JsValue::from_f64(value));
    }
    element.unchecked_into()
}

fn sized_canvas() -> HtmlCanvasElement {
    let element: Object = Object::new();
    reflective(element.as_ref(), "width", &JsValue::from_f64(256.0));
    reflective(element.as_ref(), "height", &JsValue::from_f64(128.0));
    element.unchecked_into()
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a js_sys stand-in for a WebGL2 context, which only exists under wasm"
)]
fn an_image_becomes_a_texture_sized_from_its_natural_dimensions() {
    let (context, log): (WebGl2RenderingContext, Array) = recording_context();
    let from: u32 = log.length();

    let created: Option<GlTexture> = GlTexture::create_from_image(
        &context,
        &counted_element(&["naturalWidth", "naturalHeight"], 64.0),
        GpuTextureFormat::Rgba8Unorm,
    );

    assert!(created.is_some(), "the driver handed back a texture");
    assert_eq!(
        calls_after(&log, from),
        vec![
            String::from("createTexture"),
            String::from("pixelStorei"),
            String::from("bindTexture"),
            String::from("texImage2D"),
            String::from("pixelStorei"),
            String::from("texParameteri"),
            String::from("texParameteri"),
            String::from("texParameteri"),
            String::from("texParameteri"),
        ],
        "allocate, set the unpack flip, bind, upload, put the flip back, then set the default \
         parameters - in that order. Restoring UNPACK_FLIP_Y_WEBGL is not optional: it is context \
         state, and leaving it flipped inverts every later upload on this context"
    );
    assert_eq!(
        flip_values(&log, from),
        vec![1.0, 0.0],
        "an <img> has top-to-bottom rows and a texture has bottom-to-top ones, so this source \
         type gets the vertical flip - and the state is handed back exactly as it was found"
    );
    let texture: GlTexture = created.expect("checked just above");
    assert_eq!(texture.get_width(), 64, "the level-0 width");
    assert_eq!(
        texture.get_height(),
        64,
        "and the height, both from the image's natural size rather than its laid-out size"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a js_sys stand-in for a WebGL2 context, which only exists under wasm"
)]
fn an_image_with_no_intrinsic_size_is_clamped_up_to_one_pixel() {
    let (context, log): (WebGl2RenderingContext, Array) = recording_context();
    let from: u32 = log.length();

    let created: Option<GlTexture> = GlTexture::create_from_image(
        &context,
        &counted_element(&["naturalWidth", "naturalHeight"], 0.0),
        GpuTextureFormat::Rgba8Unorm,
    );

    let texture: GlTexture = created.expect("the driver handed back a texture");
    assert_eq!(
        texture.get_width(),
        1,
        "an image that has not loaded yet reports a zero natural size; a zero-extent texture is \
         rejected by the driver, so the width is clamped to one rather than forwarded"
    );
    assert_eq!(texture.get_height(), 1, "and the height with it");
    assert_eq!(
        calls_after(&log, from).first().map(String::as_str),
        Some("createTexture"),
        "the upload still happens either way; only the reported size is clamped"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a js_sys stand-in for a WebGL2 context, which only exists under wasm"
)]
fn a_canvas_source_is_uploaded_without_the_vertical_flip() {
    let (context, log): (WebGl2RenderingContext, Array) = recording_context();
    let from: u32 = log.length();

    let created: Option<GlTexture> =
        GlTexture::create_from_canvas(&context, &sized_canvas(), GpuTextureFormat::Rgba8Unorm);

    assert!(created.is_some(), "the driver handed back a texture");
    assert_eq!(
        calls_after(&log, from),
        vec![
            String::from("createTexture"),
            String::from("bindTexture"),
            String::from("texImage2D"),
            String::from("texParameteri"),
            String::from("texParameteri"),
            String::from("texParameteri"),
            String::from("texParameteri"),
        ],
        "no pixelStorei at all on this path: a 2D canvas is already addressed bottom-left, so \
         flipping it would turn a correct upload upside down"
    );
    let texture: GlTexture = created.expect("checked just above");
    assert_eq!(texture.get_width(), 256, "the source canvas width");
    assert_eq!(texture.get_height(), 128, "and its height");
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a js_sys stand-in for a WebGL2 context, which only exists under wasm"
)]
fn a_bitmap_source_is_uploaded_from_the_raw_array_it_is_handed() {
    let (context, log): (WebGl2RenderingContext, Array) = recording_context();
    let from: u32 = log.length();
    let pixels: Array = Array::of4(
        &JsValue::from_f64(255.0),
        &JsValue::from_f64(0.0),
        &JsValue::from_f64(0.0),
        &JsValue::from_f64(255.0),
    );

    let created: Option<GlTexture> =
        GlTexture::create_from_bitmap(&context, &pixels, 32, 16, GpuTextureFormat::Rgba8Unorm);

    assert!(created.is_some(), "the driver handed back a texture");
    let texture: GlTexture = created.expect("checked just above");
    assert_eq!(
        texture.get_width(),
        32,
        "a bitmap carries no dimensions of its own, so the caller-declared width is the only \
         source of truth for the texture's reported size"
    );
    assert_eq!(texture.get_height(), 16, "and the height with it");
    assert_eq!(
        flip_values(&log, from),
        vec![1.0, 0.0],
        "and a bitmap source does get the vertical flip, same as an image, with the context \
         state handed back afterwards"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a js_sys stand-in for a WebGL2 context, which only exists under wasm"
)]
fn a_driver_that_hands_back_no_texture_reports_the_failure_instead_of_a_handle() {
    let context: Object = Object::new();
    let arm: js_sys::Function = js_sys::Function::new_with_args(
        "target",
        "target.createTexture = function() { return null; };",
    );
    let armed: Result<JsValue, JsValue> = arm.call1(&JsValue::NULL, context.as_ref());
    assert!(armed.is_ok(), "the refusing recorder must install cleanly");
    let typed: WebGl2RenderingContext = context.unchecked_into();

    let created: Option<GlTexture> =
        GlTexture::create_from_bitmap(&typed, &Array::new(), 4, 4, GpuTextureFormat::Rgba8Unorm);

    assert!(
        created.is_none(),
        "a null texture from the driver is the documented failure signal, and it has to reach \
         the caller as None rather than as a half-built texture"
    );
}
