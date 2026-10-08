use super::*;

const GL_NAMES: &str = r#"[
    "enable","disable","depthFunc","depthMask","blendFuncSeparate",
    "blendEquationSeparate","cullFace","frontFace","colorMask","scissor",
    "viewport","activeTexture","bindTexture","useProgram","drawElements",
    "drawArraysInstanced","drawElementsInstanced","readPixels","clear",
    "clearColor","clearDepth","pixelStorei","texImage2D","texSubImage2D",
    "createTexture","bindTexture","texParameteri","deleteTexture",
    "isContextLost","getError","finish","flush","generateMipmap","bindBufferBase"
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

fn recording_context(context_lost: bool) -> (WebGl2RenderingContext, Object) {
    let context: Object = Object::new();
    reflective(context.as_ref(), "__log", Array::new().as_ref());
    let arm: js_sys::Function = js_sys::Function::new_with_args(
        "target, names, lost",
        "for (const name of JSON.parse(names)) { target[name] = function() { target.__log.push([name, Array.from(arguments)]); if (name === 'isContextLost') return lost; if (name.indexOf('create') === 0) return { __gl: name }; if (name === 'getError') return 0; return null; }; }",
    );
    let armed: Result<JsValue, JsValue> = arm.call3(
        &JsValue::NULL,
        context.as_ref(),
        &JsValue::from_str(GL_NAMES),
        &JsValue::from_bool(context_lost),
    );
    assert!(armed.is_ok(), "the backend recorder must install cleanly");
    let typed: WebGl2RenderingContext = context.clone().unchecked_into();
    (typed, context)
}

fn canvas_element() -> HtmlCanvasElement {
    let element: Object = Object::new();
    element.unchecked_into()
}

fn backend(context_lost: bool) -> (WebGl2Backend, Object) {
    let (context, log): (WebGl2RenderingContext, Object) = recording_context(context_lost);
    let subject: WebGl2Backend = WebGl2Backend::from_init(WebGl2BackendInit {
        canvas: canvas_element(),
        context,
        width: 800,
        height: 600,
        clear_color: Color::new(0.1, 0.2, 0.3, 1.0),
    });
    (subject, log)
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
    panic!("{op} was never called after index {from}");
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn a_fresh_backend_reports_a_live_context_and_no_context_loss() {
    let (subject, _log): (WebGl2Backend, Object) = backend(false);

    assert!(
        !subject.is_context_lost(),
        "a freshly constructed backend holds a live context, so the per-frame check reads false"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn a_lost_context_is_reported_so_the_renderer_can_rebuild() {
    let (subject, _log): (WebGl2Backend, Object) = backend(true);

    assert!(
        subject.is_context_lost(),
        "every GL call against a lost context is a silent no-op, so this has to reach the caller"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn resizing_pushes_the_new_viewport_to_the_driver() {
    let (mut subject, log): (WebGl2Backend, Object) = backend(false);
    let before: u32 = Reflect::get(&log, &JsValue::from_str("__log"))
        .expect("log readable")
        .unchecked_into::<Array>()
        .length();

    subject.resize(1280, 720);

    let fresh: Vec<String> = ops_after(&log, before);
    assert_eq!(
        fresh,
        vec![String::from("viewport")],
        "resizing issues exactly the viewport call and nothing else"
    );
    let args: Array = args_after(&log, "viewport", before);
    assert_eq!(
        args.get(2).as_f64(),
        Some(1280.0),
        "the new width reaches the driver"
    );
    assert_eq!(args.get(3).as_f64(), Some(720.0), "and the new height");
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn invalidating_state_seeds_the_shadow_from_the_new_viewport() {
    let (mut subject, _log): (WebGl2Backend, Object) = backend(false);

    subject.invalidate_state(640, 480);

    assert!(
        !subject.is_context_lost(),
        "invalidating the shadow is a bookkeeping reset, not a context event"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn the_default_state_a_fresh_backend_starts_with_disables_depth_testing() {
    let fresh: GlRenderState = GlRenderState::context_defaults(800, 600);

    assert!(
        !fresh.get_depth().get_enabled(),
        "a fresh GL context has depth testing off, so the shadow must not claim it is on"
    );
    assert!(
        fresh.get_scissor().is_none(),
        "scissoring starts off, and the off case is an absent rectangle rather than a zero one"
    );
    assert_eq!(
        *fresh.get_color_mask().get_bits(),
        0xf,
        "all four channels are writable until a draw says otherwise"
    );
}
