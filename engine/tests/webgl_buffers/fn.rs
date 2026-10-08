use super::*;

const GL_NAMES: &str =
    r#"["createBuffer","bindBuffer","bufferData","bufferSubData","deleteBuffer"]"#;

fn recording_context() -> (WebGl2RenderingContext, Object) {
    let context: Object = Object::new();
    let arm: js_sys::Function = js_sys::Function::new_with_args(
        "target, names",
        "target.__log = []; for (const name of JSON.parse(names)) { target[name] = function() { target.__log.push([name, Array.from(arguments)]); return name.startsWith('create') ? { __gl: name } : null; }; }",
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

fn count_of(context: &Object, op: &str) -> usize {
    ops(context)
        .iter()
        .filter(|recorded: &&String| recorded.as_str() == op)
        .count()
}

const TARGET_ARRAY_BUFFER: u32 = 0x8892;
const TARGET_ELEMENT_ARRAY_BUFFER: u32 = 0x8893;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn a_fresh_buffer_reports_the_capacity_it_was_allocated_with() {
    let (context, _log): (WebGl2RenderingContext, Object) = recording_context();
    let buffer: GlBuffer =
        GlBuffer::create_for(&context, BufferUsage::Vertex, 64).expect("driver handle came back");
    assert_eq!(
        buffer.get_capacity(),
        64,
        "the allocation records the size it was asked for, so a later fits has something to compare"
    );
    assert!(
        buffer.fits(64),
        "a payload of exactly the allocated size fits"
    );
    assert!(
        !buffer.fits(65),
        "one byte past the allocation does not fit, which is what triggers a grow"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn a_small_upload_writes_in_place_without_reallocating() {
    let (context, log): (WebGl2RenderingContext, Object) = recording_context();
    let mut buffer: GlBuffer =
        GlBuffer::create_for(&context, BufferUsage::Vertex, 64).expect("created");
    let before: usize = count_of(&log, "bufferData");

    buffer.upload(&context, TARGET_ARRAY_BUFFER, &[0; 32]);

    assert_eq!(
        count_of(&log, "bufferData"),
        before,
        "a payload inside the existing capacity must not reallocate"
    );
    assert_eq!(
        count_of(&log, "bufferSubData"),
        1,
        "the bytes reach the driver through bufferSubData"
    );
    assert_eq!(
        buffer.get_capacity(),
        64,
        "writing into spare capacity must not shrink the allocation"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn a_payload_larger_than_the_allocation_reallocates_and_grows_the_capacity() {
    let (context, log): (WebGl2RenderingContext, Object) = recording_context();
    let mut buffer: GlBuffer =
        GlBuffer::create_for(&context, BufferUsage::Vertex, 16).expect("created");
    let before: usize = count_of(&log, "bufferData");

    buffer.upload(&context, TARGET_ARRAY_BUFFER, &[0; 128]);

    assert_eq!(
        count_of(&log, "bufferData"),
        before + 1,
        "growing past the capacity is exactly one extra allocation"
    );
    assert!(
        buffer.get_capacity() >= 128,
        "the recorded capacity has to cover the payload, got {}",
        buffer.get_capacity()
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn a_sub_range_write_past_the_capacity_is_refused_rather_than_overrunning() {
    let (context, _log): (WebGl2RenderingContext, Object) = recording_context();
    let mut buffer: GlBuffer =
        GlBuffer::create_for(&context, BufferUsage::Vertex, 64).expect("created");

    let inside: bool = buffer.update(&context, TARGET_ARRAY_BUFFER, 32, &[0; 32]);
    assert!(
        inside,
        "a range ending exactly at the capacity is inside it"
    );

    let outside: bool = buffer.update(&context, TARGET_ARRAY_BUFFER, 32, &[0; 64]);
    assert!(
        !outside,
        "a range ending past the capacity must be refused so the driver is never overrun"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn deleting_a_buffer_zeroes_it_so_a_later_upload_reallocates() {
    let (context, log): (WebGl2RenderingContext, Object) = recording_context();
    let mut buffer: GlBuffer =
        GlBuffer::create_for(&context, BufferUsage::Vertex, 64).expect("created");
    buffer.delete(&context, TARGET_ARRAY_BUFFER);

    assert_eq!(
        buffer.get_capacity(),
        0,
        "a deleted buffer keeps no stale capacity, or a later upload writes into reclaimed storage"
    );
    assert!(
        ops(&log).contains(&String::from("deleteBuffer")),
        "the driver handle must actually be released"
    );

    let before: usize = count_of(&log, "bufferData");
    buffer.upload(&context, TARGET_ARRAY_BUFFER, &[0; 8]);
    assert_eq!(
        count_of(&log, "bufferData"),
        before + 1,
        "after a delete every upload must reallocate rather than write into freed memory"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn binding_a_buffer_issues_exactly_one_call_and_leaves_capacity_alone() {
    let (context, log): (WebGl2RenderingContext, Object) = recording_context();
    let buffer: GlBuffer =
        GlBuffer::create_for(&context, BufferUsage::Vertex, 16).expect("created");
    let before: usize = count_of(&log, "bindBuffer");

    buffer.bind(&context, TARGET_ELEMENT_ARRAY_BUFFER);

    assert_eq!(
        count_of(&log, "bindBuffer"),
        before + 1,
        "bind is exactly one bindBuffer call, not a bind plus a re-upload"
    );
    assert_eq!(
        buffer.get_capacity(),
        16,
        "binding is unrelated to capacity, so the allocation must be unchanged"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn each_buffer_usage_asks_the_driver_for_its_own_buffer() {
    let (context, log): (WebGl2RenderingContext, Object) = recording_context();
    let _vertices: GlBuffer =
        GlBuffer::create_for(&context, BufferUsage::Vertex, 4).expect("created");
    let _indices: GlBuffer =
        GlBuffer::create_for(&context, BufferUsage::Index, 4).expect("created");

    assert_eq!(
        count_of(&log, "createBuffer"),
        2,
        "each usage asked for its own driver buffer"
    );
}
