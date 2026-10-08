use super::*;

const CTX_METHODS: &str = r#"[
    "createVertexArray","bindVertexArray","deleteVertexArray",
    "enableVertexAttribArray","vertexAttribPointer","vertexAttribDivisor",
    "createBuffer","bindBuffer","bufferData","deleteBuffer"
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
        "target, methods",
        "for (const name of JSON.parse(methods)) { target[name] = function() { target.__log.push([name, Array.from(arguments)]); return name.indexOf('create') === 0 ? { __gl: name } : null; }; }",
    );
    let armed: Result<JsValue, JsValue> = arm.call2(
        &JsValue::NULL,
        context.as_ref(),
        &JsValue::from_str(CTX_METHODS),
    );
    assert!(armed.is_ok(), "the vao recorder must install cleanly");
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

fn count_of(context: &Object, op: &str) -> usize {
    ops(context)
        .iter()
        .filter(|recorded: &&String| recorded.as_str() == op)
        .count()
}

fn first_of(context: &Object, op: &str) -> Array {
    let log: Array = Reflect::get(context, &JsValue::from_str("__log"))
        .expect("log readable")
        .unchecked_into();
    for index in 0..log.length() {
        let entry: Array = log.get(index).unchecked_into();
        if entry.get(0).as_string().as_deref() == Some(op) {
            return entry.get(1).unchecked_into();
        }
    }
    panic!(
        "{op} was never called; recorded ops were {:?}",
        ops(context)
    );
}

fn empty_layout() -> VertexBufferLayout {
    VertexBufferLayout::new(12, VertexStepMode::Vertex, Vec::new())
}

fn layout_with(attribute: VertexAttribute) -> VertexBufferLayout {
    VertexBufferLayout::new(12, VertexStepMode::Vertex, vec![attribute])
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn creating_an_array_object_binds_it_and_then_releases_it() {
    let (context, log): (WebGl2RenderingContext, Object) = recording_context();

    let vao: Option<GlVertexArray> = GlVertexArray::create(&context);

    assert!(vao.is_some(), "the driver handle came back");
    assert_eq!(
        count_of(&log, "createVertexArray"),
        1,
        "one array object is created"
    );
    assert_eq!(
        count_of(&log, "bindVertexArray"),
        0,
        "creation alone must not bind, or it would silently overwrite whatever array object the caller had current"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn each_declared_attribute_is_enabled_pointed_and_given_a_divisor() {
    let (context, log): (WebGl2RenderingContext, Object) = recording_context();
    let vao: GlVertexArray = GlVertexArray::create(&context).expect("created");
    let buffer: GlBuffer =
        GlBuffer::create_for(&context, BufferUsage::Vertex, 64).expect("created");
    let layout: VertexBufferLayout =
        layout_with(VertexAttribute::new(2, 8, VertexAttributeFormat::Float32x4));

    vao.set_layout(&context, &buffer, &layout);

    assert_eq!(
        count_of(&log, "enableVertexAttribArray"),
        1,
        "the one declared attribute is enabled"
    );
    assert_eq!(
        count_of(&log, "vertexAttribPointer"),
        1,
        "and receives a pointer describing how to read its bytes"
    );
    assert_eq!(
        count_of(&log, "vertexAttribDivisor"),
        1,
        "a per-vertex step mode records divisor zero"
    );
    let divisor: Array = first_of(&log, "vertexAttribDivisor");
    assert_eq!(
        divisor.get(1).as_f64(),
        Some(0.0),
        "Vertex step mode means one step per vertex, so the divisor is zero"
    );
    let pointer: Array = first_of(&log, "vertexAttribPointer");
    assert_eq!(
        pointer.get(0).as_f64(),
        Some(2.0),
        "the pointer is recorded at the attribute's own shader location"
    );
    assert_eq!(
        pointer.get(4).as_f64(),
        Some(12.0),
        "the stride comes from the layout, so the shader and the buffer agree on vertex size"
    );
    assert_eq!(
        pointer.get(5).as_f64(),
        Some(8.0),
        "the byte offset comes from the attribute, not a running total"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn an_instance_step_mode_records_divisor_one() {
    let (context, log): (WebGl2RenderingContext, Object) = recording_context();
    let vao: GlVertexArray = GlVertexArray::create(&context).expect("created");
    let buffer: GlBuffer =
        GlBuffer::create_for(&context, BufferUsage::Vertex, 64).expect("created");
    let layout: VertexBufferLayout = VertexBufferLayout::new(
        16,
        VertexStepMode::Instance,
        vec![VertexAttribute::new(1, 0, VertexAttributeFormat::Float32x4)],
    );

    vao.set_instance_layout(&context, &buffer, &layout);

    let divisor: Array = first_of(&log, "vertexAttribDivisor");
    assert_eq!(
        divisor.get(1).as_f64(),
        Some(1.0),
        "Instance step mode advances the buffer once per instance, so the divisor is one"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn unbinding_passes_none_so_the_context_returns_to_its_default_state() {
    let (context, log): (WebGl2RenderingContext, Object) = recording_context();

    GlVertexArray::unbind(&context);

    let bound: Array = first_of(&log, "bindVertexArray");
    assert!(
        bound.get(0).is_undefined(),
        "unbind passes an explicit None, which is what restores the default vertex state"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn a_layout_with_no_attributes_still_binds_the_array_and_the_buffer() {
    let (context, log): (WebGl2RenderingContext, Object) = recording_context();
    let vao: GlVertexArray = GlVertexArray::create(&context).expect("created");
    let buffer: GlBuffer =
        GlBuffer::create_for(&context, BufferUsage::Vertex, 64).expect("created");
    let before: usize = count_of(&log, "enableVertexAttribArray");

    vao.set_layout(&context, &buffer, &empty_layout());

    assert_eq!(
        count_of(&log, "enableVertexAttribArray"),
        before,
        "an empty attribute list must not enable anything"
    );
    assert_eq!(
        count_of(&log, "vertexAttribPointer"),
        before,
        "and must not record any pointer either"
    );
    assert!(
        count_of(&log, "bindBuffer") >= 1,
        "the source buffer is still bound, or the later pointers would read the wrong buffer"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn setting_an_explicit_divisor_records_exactly_that_value() {
    let (context, log): (WebGl2RenderingContext, Object) = recording_context();
    let vao: GlVertexArray = GlVertexArray::create(&context).expect("created");

    vao.set_divisor(&context, 3, 1);

    let recorded: Array = first_of(&log, "vertexAttribDivisor");
    assert_eq!(
        recorded.get(0).as_f64(),
        Some(3.0),
        "the divisor call names the attribute index it was given"
    );
    assert_eq!(
        recorded.get(1).as_f64(),
        Some(1.0),
        "an explicit divisor of one is recorded as asked, not overridden by the layout"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn deleting_an_array_object_releases_the_handle() {
    let (context, log): (WebGl2RenderingContext, Object) = recording_context();
    let vao: GlVertexArray = GlVertexArray::create(&context).expect("created");
    let before: usize = count_of(&log, "deleteVertexArray");

    vao.delete(&context);

    assert_eq!(
        count_of(&log, "deleteVertexArray"),
        before + 1,
        "the driver handle is released exactly once"
    );
}
