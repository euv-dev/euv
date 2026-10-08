use super::*;

const GL_NAMES: &str = r#"[
    "createShader","shaderSource","compileShader","getShaderParameter",
    "getShaderInfoLog","deleteShader","createProgram","attachShader",
    "linkProgram","getProgramParameter","getProgramInfoLog","deleteProgram",
    "useProgram","clear","clearColor","clearDepth","viewport","drawArrays",
    "drawArraysInstanced","drawElements","drawElementsInstanced","activeTexture",
    "bindTexture","readPixels","getError","finish","isContextLost","enable","disable"
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

fn recording_context() -> (WebGl2RenderingContext, Object) {
    let context: Object = Object::new();
    reflective(context.as_ref(), "__log", Array::new().as_ref());
    let arm: js_sys::Function = js_sys::Function::new_with_args(
        "target, names",
        "for (const name of JSON.parse(names)) { target[name] = function() { target.__log.push([name, Array.from(arguments)]); if (name === 'getShaderParameter' || name === 'getProgramParameter') return true; if (name.indexOf('create') === 0) return { __gl: name }; if (name.indexOf('InfoLog') >= 0) return ''; if (name === 'readPixels') return new Uint8Array(4); return null; }; }",
    );
    let armed: Result<JsValue, JsValue> = arm.call2(
        &JsValue::NULL,
        context.as_ref(),
        &JsValue::from_str(GL_NAMES),
    );
    assert!(armed.is_ok(), "the frame recorder must install cleanly");
    let typed: WebGl2RenderingContext = context.clone().unchecked_into();
    (typed, context)
}

fn backend(width: u32, height: u32) -> (WebGl2Backend, Object) {
    let (context, log): (WebGl2RenderingContext, Object) = recording_context();
    let element: Object = Object::new();
    reflective(
        element.as_ref(),
        "width",
        &JsValue::from_f64(f64::from(width)),
    );
    reflective(
        element.as_ref(),
        "height",
        &JsValue::from_f64(f64::from(height)),
    );
    let subject: WebGl2Backend = WebGl2Backend::from_init(WebGl2BackendInit {
        canvas: element.unchecked_into(),
        context,
        width,
        height,
        clear_color: Color::new(0.0, 0.0, 0.0, 1.0),
    });
    (subject, log)
}

fn program(context: &WebGl2RenderingContext) -> GlProgram {
    GlProgram::create(context, "vertex body", "fragment body").expect("linked")
}

fn log_len(log: &Object) -> u32 {
    Reflect::get(log, &JsValue::from_str("__log"))
        .expect("log readable")
        .unchecked_into::<Array>()
        .length()
}

fn ops_after(log: &Object, from: u32) -> Vec<String> {
    let entries: Array = Reflect::get(log, &JsValue::from_str("__log"))
        .expect("log readable")
        .unchecked_into();
    (from..entries.length())
        .map(|index: u32| {
            let entry: Array = entries.get(index).unchecked_into();
            entry.get(0).as_string().unwrap_or_default()
        })
        .collect()
}

fn count_after(log: &Object, op: &str, from: u32) -> usize {
    ops_after(log, from)
        .iter()
        .filter(|recorded: &&String| recorded.as_str() == op)
        .count()
}

fn args_after(log: &Object, op: &str, from: u32) -> Array {
    let entries: Array = Reflect::get(log, &JsValue::from_str("__log"))
        .expect("log readable")
        .unchecked_into();
    for index in from..entries.length() {
        let entry: Array = entries.get(index).unchecked_into();
        if entry.get(0).as_string().as_deref() == Some(op) {
            return entry.get(1).unchecked_into();
        }
    }
    panic!(
        "{op} was never called after index {from}; saw {:?}",
        ops_after(log, from)
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn clearing_the_depth_buffer_alone_masks_out_the_colour_buffer() {
    let (subject, log): (WebGl2Backend, Object) = backend(800, 600);
    let before: u32 = log_len(&log);

    subject.clear_depth(&subject.get_context().clone());

    assert_eq!(
        count_after(&log, "clear", before),
        1,
        "a depth-only clear is one call"
    );
    let bits: Array = args_after(&log, "clear", before);
    let mask: u32 = bits.get(0).as_f64().unwrap_or(-1.0) as u32;
    let depth_bit: u32 = WebGl2RenderingContext::DEPTH_BUFFER_BIT;
    let color_bit: u32 = WebGl2RenderingContext::COLOR_BUFFER_BIT;
    assert_ne!(
        mask & depth_bit,
        0,
        "the depth bit has to be set or the clear does nothing"
    );
    assert_eq!(
        mask & color_bit,
        0,
        "masking the colour buffer too would throw away results a pass is about to read"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn a_frame_clears_both_buffers_to_the_colour_it_was_given() {
    let (mut subject, log): (WebGl2Backend, Object) = backend(800, 600);
    let context: WebGl2RenderingContext = subject.get_context().clone();
    let program: GlProgram = program(&context);
    let before: u32 = log_len(&log);

    subject.render_frame(&context, &program, Color::new(0.25, 0.5, 0.75, 1.0), 3);

    let clear: Array = args_after(&log, "clear", before);
    let mask: u32 = clear.get(0).as_f64().unwrap_or(-1.0) as u32;
    assert_ne!(
        mask & WebGl2RenderingContext::COLOR_BUFFER_BIT,
        0,
        "a frame clear covers the colour buffer"
    );
    assert_ne!(
        mask & WebGl2RenderingContext::DEPTH_BUFFER_BIT,
        0,
        "and the depth buffer too, unlike the depth-only clear"
    );
    let color: Array = args_after(&log, "clearColor", before);
    assert_eq!(
        color.get(0).as_f64(),
        Some(0.25),
        "the clear colour the caller asked for, in channel order"
    );
    assert_eq!(color.get(3).as_f64(), Some(1.0), "including alpha");
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn a_frame_draws_the_vertex_count_the_caller_asked_for() {
    let (mut subject, log): (WebGl2Backend, Object) = backend(800, 600);
    let context: WebGl2RenderingContext = subject.get_context().clone();
    let program: GlProgram = program(&context);
    let before: u32 = log_len(&log);

    subject.render_frame(&context, &program, Color::new(0.0, 0.0, 0.0, 1.0), 3);

    assert_eq!(
        count_after(&log, "drawArrays", before),
        1,
        "one full-screen triangle-list draw, generated in the vertex shader"
    );
    let draw: Array = args_after(&log, "drawArrays", before);
    assert_eq!(
        draw.get(2).as_f64(),
        Some(3.0),
        "the vertex count is the caller's, not a hardcoded triangle"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn a_second_frame_rebinds_nothing_that_has_not_changed() {
    let (mut subject, log): (WebGl2Backend, Object) = backend(800, 600);
    let context: WebGl2RenderingContext = subject.get_context().clone();
    let program: GlProgram = program(&context);
    subject.render_frame(&context, &program, Color::new(0.0, 0.0, 0.0, 1.0), 3);

    let before: u32 = log_len(&log);
    subject.render_frame(&context, &program, Color::new(0.0, 0.0, 0.0, 1.0), 3);

    assert_eq!(
        count_after(&log, "useProgram", before),
        0,
        "the same program is still bound, and the shadow says so"
    );
    assert_eq!(
        count_after(&log, "clearColor", before),
        1,
        "but the clear colour is re-asserted, because it is not diffed"
    );
    assert_eq!(
        count_after(&log, "viewport", before),
        0,
        "and the viewport is unchanged since the last frame"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn reading_pixels_back_returns_the_buffer_the_driver_filled() {
    let (mut subject, log): (WebGl2Backend, Object) = backend(2, 2);
    let before: u32 = log_len(&log);

    let read: Vec<u8> = subject.read_pixels(&subject.get_context().clone(), 0, 0, 2, 2);

    assert_eq!(
        read.len(),
        16,
        "a 2x2 RGBA readback is sixteen bytes, so the buffer was filled and returned"
    );
    assert_eq!(
        count_after(&log, "readPixels", before),
        1,
        "reading back is one call"
    );
}
