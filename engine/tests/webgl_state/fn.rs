use super::*;

const GL_NAMES: &str = r#"[
    "enable","disable","depthFunc","depthMask","blendFuncSeparate",
    "blendEquationSeparate","cullFace","frontFace","colorMask","scissor",
    "viewport","activeTexture","createTexture","bindTexture","texParameteri",
    "deleteTexture","isContextLost","clear","clearDepth"
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
        "for (const name of JSON.parse(names)) { target[name] = function() { target.__log.push([name, Array.from(arguments)]); if (name.indexOf('create') === 0) return { __gl: name }; return null; }; }",
    );
    let armed: Result<JsValue, JsValue> = arm.call2(
        &JsValue::NULL,
        context.as_ref(),
        &JsValue::from_str(GL_NAMES),
    );
    assert!(armed.is_ok(), "the state recorder must install cleanly");
    let typed: WebGl2RenderingContext = context.clone().unchecked_into();
    (typed, context)
}

fn backend() -> (WebGl2Backend, Object) {
    let (context, log): (WebGl2RenderingContext, Object) = recording_context();
    let element: Object = Object::new();
    let subject: WebGl2Backend = WebGl2Backend::from_init(WebGl2BackendInit {
        canvas: element.unchecked_into(),
        context,
        width: 800,
        height: 600,
        clear_color: Color::new(0.0, 0.0, 0.0, 1.0),
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

fn count_after(log: &Object, op: &str, from: u32) -> usize {
    ops_after(log, from)
        .iter()
        .filter(|recorded: &&String| recorded.as_str() == op)
        .count()
}

fn log_len(log: &Object) -> u32 {
    Reflect::get(log, &JsValue::from_str("__log"))
        .expect("log readable")
        .unchecked_into::<Array>()
        .length()
}

fn depth(enabled: bool, write: bool) -> GlDepthState {
    GlDepthState::new(enabled, CompareFunction::LessEqual, write)
}

fn viewport(x: i32, y: i32, width: i32, height: i32) -> GlViewport {
    GlViewport::new(x, y, width, height)
}

fn state_with(depth_state: GlDepthState, scissor: Option<GlScissor>, mask: u32) -> GlRenderState {
    GlRenderState::from_init(GlRenderStateInit {
        depth: depth_state,
        blend: GlBlendState::default(),
        cull: GlCullState::default(),
        color_mask: GlColorMask::new(mask),
        scissor,
        viewport: viewport(0, 0, 800, 600),
    })
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn applying_the_state_a_fresh_backend_already_holds_issues_nothing() {
    let (mut subject, log): (WebGl2Backend, Object) = backend();
    let before: u32 = log_len(&log);

    subject.apply_state(&GlRenderState::context_defaults(800, 600));

    assert_eq!(
        ops_after(&log, before),
        Vec::<String>::new(),
        "the shadow was seeded with the driver's own defaults, so the first apply of that \
         same state must not re-assert a dozen calls"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn turning_depth_testing_on_toggles_the_capability_and_the_compare_function() {
    let (mut subject, log): (WebGl2Backend, Object) = backend();
    let before: u32 = log_len(&log);

    subject.apply_state(&state_with(depth(true, true), None, 0xf));

    assert_eq!(
        count_after(&log, "enable", before),
        1,
        "the capability is enabled exactly once"
    );
    assert_eq!(
        count_after(&log, "depthFunc", before),
        1,
        "and the comparison is pushed with it, because a toggle invalidates it"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn a_write_mask_change_alone_issues_only_the_colour_mask() {
    let (mut subject, log): (WebGl2Backend, Object) = backend();
    let before: u32 = log_len(&log);

    subject.apply_state(&state_with(depth(false, true), None, 0x3));

    assert_eq!(
        count_after(&log, "colorMask", before),
        1,
        "narrowing the write mask is one call"
    );
    assert_eq!(
        count_after(&log, "enable", before),
        0,
        "a mask change does not touch the capability state"
    );
    assert_eq!(
        count_after(&log, "depthMask", before),
        0,
        "nor the depth write state"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn switching_scissor_on_enables_the_capability_and_sets_the_rectangle() {
    let (mut subject, log): (WebGl2Backend, Object) = backend();
    let before: u32 = log_len(&log);

    subject.apply_state(&state_with(
        depth(false, true),
        Some(GlScissor::new(10, 20, 100, 50)),
        0xf,
    ));

    assert_eq!(
        count_after(&log, "enable", before),
        1,
        "the scissor rectangle only takes effect once SCISSOR_TEST is enabled"
    );
    assert_eq!(
        count_after(&log, "scissor", before),
        1,
        "and the rectangle itself is pushed"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn a_zero_sized_scissor_still_enables_the_capability_because_it_clips_everything() {
    let (mut subject, log): (WebGl2Backend, Object) = backend();
    let before: u32 = log_len(&log);

    subject.apply_state(&state_with(
        depth(false, true),
        Some(GlScissor::new(0, 0, 0, 0)),
        0xf,
    ));

    assert_eq!(
        count_after(&log, "enable", before),
        1,
        "a zero sized box is meaningfully different from scissoring off, so the capability is \
         still enabled"
    );
}
