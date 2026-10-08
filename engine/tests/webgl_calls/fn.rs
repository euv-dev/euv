use super::*;

const GL_NAMES: &str = r#"["createTexture","bindTexture","texParameteri","texImage2D","deleteTexture","generateMipmap","createBuffer","bindBuffer","bufferData","deleteBuffer","createProgram","createShader","deleteProgram","uniform1f","drawElements","viewport","texSubImage2D","activeTexture","pixelStorei","scissor","enable","blendFunc","clearColor","clear","deleteFramebuffer","bindFramebuffer","drawArraysInstanced"]"#;

fn recording_context() -> (WebGl2RenderingContext, Object) {
    let context: Object = Object::new();
    let arm: js_sys::Function = js_sys::Function::new_with_args(
        "target, names",
        "target.__log = []; for (const name of JSON.parse(names)) { target[name] = function() { target.__log.push([name, Array.from(arguments)]); return this.__handles[name] || null; }; } target.__handles = {}; for (const name of ['createTexture','createBuffer','createProgram','createShader','createFramebuffer']) { target.__handles[name] = { __gl: name }; }",
    );
    let armed: Result<JsValue, JsValue> = arm.call2(
        &JsValue::NULL,
        context.as_ref(),
        &JsValue::from_str(GL_NAMES),
    );
    assert!(armed.is_ok(), "the gl recorder must install cleanly");
    let typed: WebGl2RenderingContext = context.clone().unchecked_into();
    (typed, context)
}

fn ops(context: &Object) -> Vec<String> {
    let log: Array = Reflect::get(context, &JsValue::from_str("__log"))
        .expect("log readable")
        .unchecked_into();
    (0..log.length())
        .map(|index: u32| {
            let entry: Array = log.get(index).unchecked_into();
            entry.get(0).as_string().unwrap_or_default()
        })
        .collect()
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn creating_a_texture_uploads_the_texels_and_applies_default_sampling() {
    let (context, log): (WebGl2RenderingContext, Object) = recording_context();
    let created: Option<GlTexture> =
        GlTexture::create(&context, 4, 2, GpuTextureFormat::Rgba8Unorm, &[0; 32]);
    assert!(
        created.is_some(),
        "the driver handle came back, so the texture exists"
    );
    let texture: &GlTexture = created.as_ref().expect("created");
    assert_eq!(
        texture.get_width(),
        4,
        "the allocation records the width it was asked for"
    );
    assert_eq!(
        texture.get_height(),
        2,
        "the allocation records the height it was asked for"
    );
    assert_eq!(
        texture.get_levels(),
        1,
        "a fresh allocation starts with only its base mip level"
    );
    assert!(
        !texture.get_mipmapped(),
        "generateMipmap has not been called, so sampling across levels is still illegal"
    );
    let recorded: Vec<String> = ops(&log);
    assert!(
        recorded.contains(&String::from("texImage2D")),
        "the pixels have to reach the driver through texImage2D"
    );
    assert_eq!(
        recorded
            .iter()
            .filter(|op: &&String| *op == "texParameteri")
            .count(),
        4,
        "min filter, mag filter and both wrap axes are four separate calls"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn a_sub_rectangle_write_inside_the_texture_is_issued_and_one_outside_is_refused() {
    let (context, log): (WebGl2RenderingContext, Object) = recording_context();
    let texture: GlTexture =
        GlTexture::create(&context, 8, 8, GpuTextureFormat::Rgba8Unorm, &[]).expect("created");
    let before: usize = ops(&log).len();

    let inside: bool = texture.update(
        &context,
        &GlScissor::new(0, 0, 8, 8),
        GpuTextureFormat::Rgba8Unorm,
        &[0; 256],
    );
    assert!(
        inside,
        "a rectangle covering the whole allocation is inside it"
    );
    assert!(
        ops(&log)[before..].contains(&String::from("texSubImage2D")),
        "an accepted write must reach texSubImage2D"
    );

    let outside: bool = texture.update(
        &context,
        &GlScissor::new(4, 4, 8, 8),
        GpuTextureFormat::Rgba8Unorm,
        &[0; 256],
    );
    assert!(
        !outside,
        "a rectangle running past the texture must be refused rather than overrunning the driver"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn a_negative_origin_is_clamped_to_zero_before_the_bounds_check() {
    let (context, _log): (WebGl2RenderingContext, Object) = recording_context();
    let texture: GlTexture =
        GlTexture::create(&context, 8, 8, GpuTextureFormat::Rgba8Unorm, &[]).expect("created");
    let inside: bool = texture.update(
        &context,
        &GlScissor::new(-4, -4, 8, 8),
        GpuTextureFormat::Rgba8Unorm,
        &[0; 256],
    );
    assert!(
        inside,
        "clamping a negative origin to zero is what keeps the rectangle inside the allocation"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn deleting_a_texture_zeroes_it_so_a_stale_size_cannot_be_reused() {
    let (context, log): (WebGl2RenderingContext, Object) = recording_context();
    let mut texture: GlTexture =
        GlTexture::create(&context, 4, 4, GpuTextureFormat::Rgba8Unorm, &[]).expect("created");
    texture.delete(&context);
    assert_eq!(
        texture.get_width(),
        0,
        "a deleted texture keeps no stale width"
    );
    assert_eq!(
        texture.get_height(),
        0,
        "a deleted texture keeps no stale height"
    );
    assert!(
        ops(&log).contains(&String::from("deleteTexture")),
        "the driver handle must actually be released"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn generating_the_mip_chain_marks_the_texture_as_mipmapped() {
    let (context, log): (WebGl2RenderingContext, Object) = recording_context();
    let mut texture: GlTexture =
        GlTexture::create(&context, 4, 4, GpuTextureFormat::Rgba8Unorm, &[]).expect("created");
    texture.generate_mipmap(
        &context,
        FilterMode::Linear,
        MipmapFilter::Linear,
        AddressMode::ClampToEdge,
    );
    assert!(
        texture.get_mipmapped(),
        "after generateMipmap the chain exists, so a mip-aware filter becomes legal"
    );
    assert!(
        ops(&log).contains(&String::from("generateMipmap")),
        "the chain is built by the driver, not by the wrapper"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn setting_parameters_issues_exactly_four_calls() {
    let (context, log): (WebGl2RenderingContext, Object) = recording_context();
    let texture: GlTexture =
        GlTexture::create(&context, 4, 4, GpuTextureFormat::Rgba8Unorm, &[]).expect("created");
    let before: usize = ops(&log).len();
    texture.set_parameters(
        &context,
        FilterMode::Linear,
        MipmapFilter::Nearest,
        AddressMode::ClampToEdge,
    );
    let issued: usize = ops(&log)[before..]
        .iter()
        .filter(|op: &&String| *op == "texParameteri")
        .count();
    assert_eq!(
        issued, 4,
        "min filter, mag filter and both wrap axes are four calls, not one"
    );
}
