use super::*;

const GL_NAMES: &str = r#"[
    "activeTexture","bindTexture","createTexture","texParameteri","deleteTexture",
    "drawArrays","drawArraysInstanced","drawElements","drawElementsInstanced",
    "enable","disable","viewport","isContextLost","clear"
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
    assert!(armed.is_ok(), "the draw recorder must install cleanly");
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

fn texture(_subject: &WebGl2Backend, context: &WebGl2RenderingContext) -> GlTexture {
    GlTexture::create(context, 8, 8, GpuTextureFormat::Rgba8Unorm, &[]).expect("created")
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

fn count_after(log: &Object, op: &str, from: u32) -> usize {
    ops_after(log, from)
        .iter()
        .filter(|recorded: &&String| recorded.as_str() == op)
        .count()
}

fn indexed(
    index_count: u32,
    first_index: u32,
    instances: u32,
    base_vertex: i32,
) -> DrawIndexedArgs {
    let mut args: DrawIndexedArgs = DrawIndexedArgs::new();
    args.set_index_count(index_count);
    args.set_first_index(first_index);
    args.set_instance_count(instances);
    args.set_base_vertex(base_vertex);
    args.set_first_instance(0);
    args
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn a_single_instance_indexed_draw_uses_the_plain_call() {
    let (subject, log): (WebGl2Backend, Object) = backend();
    let before: u32 = log_len(&log);

    let issued: bool = subject.draw_elements(
        &subject.get_context().clone(),
        PrimitiveTopology::TriangleList,
        &indexed(36, 0, 1, 0),
        IndexFormat::Uint16,
    );

    assert!(issued, "a zero base vertex is drawable on WebGL 2");
    assert_eq!(
        count_after(&log, "drawElements", before),
        1,
        "one instance takes the plain draw, not the instanced one"
    );
    assert_eq!(
        count_after(&log, "drawElementsInstanced", before),
        0,
        "the instanced call would cost an extra draw for no reason"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn several_instances_take_the_instanced_call() {
    let (subject, log): (WebGl2Backend, Object) = backend();
    let before: u32 = log_len(&log);

    subject.draw_elements(
        &subject.get_context().clone(),
        PrimitiveTopology::TriangleList,
        &indexed(36, 0, 4, 0),
        IndexFormat::Uint16,
    );

    assert_eq!(
        count_after(&log, "drawElementsInstanced", before),
        1,
        "four instances cannot be expressed as one draw"
    );
    assert_eq!(
        count_after(&log, "drawElements", before),
        0,
        "and the non-instanced call is not issued as well"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn a_non_zero_base_vertex_is_refused_rather_than_silently_drawing_the_wrong_mesh() {
    let (subject, log): (WebGl2Backend, Object) = backend();
    let before: u32 = log_len(&log);

    let issued: bool = subject.draw_elements(
        &subject.get_context().clone(),
        PrimitiveTopology::TriangleList,
        &indexed(36, 0, 1, 4),
        IndexFormat::Uint16,
    );

    assert!(
        !issued,
        "WebGL 2 has no baseVertex, so honouring it is impossible; drawing anyway would fetch \
         the wrong vertices with no diagnostic"
    );
    assert_eq!(
        ops_after(&log, before),
        Vec::<String>::new(),
        "and a refused draw must not reach the driver at all"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn the_first_index_is_scaled_by_the_element_width_before_it_reaches_gl() {
    let (subject, log): (WebGl2Backend, Object) = backend();
    let before: u32 = log_len(&log);

    subject.draw_elements(
        &subject.get_context().clone(),
        PrimitiveTopology::TriangleList,
        &indexed(6, 10, 1, 0),
        IndexFormat::Uint32,
    );

    let args: Array = args_after(&log, "drawElements", before);
    assert_eq!(
        args.get(3).as_f64(),
        Some(40.0),
        "GL takes a byte offset, so index 10 in a 4-byte format is 40 bytes"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn binding_a_texture_switches_the_active_unit_only_when_it_differs() {
    let (mut subject, log): (WebGl2Backend, Object) = backend();
    let sample: GlTexture = texture(&subject, &subject.get_context().clone());

    let first: u32 = log_len(&log);
    subject.bind_texture_unit(&subject.get_context().clone(), 0, &sample);
    assert_eq!(
        count_after(&log, "activeTexture", first),
        1,
        "the first bind has to select the unit"
    );

    let second: u32 = log_len(&log);
    subject.bind_texture_unit(&subject.get_context().clone(), 0, &sample);
    assert_eq!(
        count_after(&log, "activeTexture", second),
        0,
        "rebinding the same unit costs no call, which is the whole point of shadowing it"
    );
    assert_eq!(
        count_after(&log, "bindTexture", second),
        1,
        "but the texture itself is still bound, because it may have changed"
    );

    let third: u32 = log_len(&log);
    subject.bind_texture_unit(&subject.get_context().clone(), 1, &sample);
    assert_eq!(
        count_after(&log, "activeTexture", third),
        1,
        "switching units does have to select the new one"
    );
}
