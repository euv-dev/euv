use super::*;

const CTX_METHODS: &str = r#"[
    "createBuffer","bindBuffer","bufferData","bufferSubData","deleteBuffer",
    "bindBufferBase","createShader","shaderSource","compileShader",
    "getShaderParameter","getShaderInfoLog","deleteShader","createProgram",
    "attachShader","linkProgram","getProgramParameter","getProgramInfoLog",
    "deleteProgram","getUniformBlockIndex","uniformBlockBinding",
    "uniformMatrix4fv"
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

fn recording_context(block_index: f64) -> (WebGl2RenderingContext, Object) {
    let context: Object = Object::new();
    reflective(context.as_ref(), "__log", Array::new().as_ref());
    let arm: js_sys::Function = js_sys::Function::new_with_args(
        "target, methods, blockIndex",
        "for (const name of JSON.parse(methods)) { target[name] = function() { target.__log.push([name, Array.from(arguments)]); if (name.indexOf('create') === 0) return { __gl: name }; if (name === 'getShaderParameter' || name === 'getProgramParameter') return true; if (name === 'getUniformBlockIndex') return blockIndex; if (name.indexOf('InfoLog') >= 0) return 'log'; return null; }; }",
    );
    let armed: Result<JsValue, JsValue> = arm.call3(
        &JsValue::NULL,
        context.as_ref(),
        &JsValue::from_str(CTX_METHODS),
        &JsValue::from_f64(block_index),
    );
    assert!(
        armed.is_ok(),
        "the uniform block recorder must install cleanly"
    );
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

fn nth_of(context: &Object, op: &str, nth: usize) -> Array {
    let log: Array = Reflect::get(context, &JsValue::from_str("__log"))
        .expect("log readable")
        .unchecked_into();
    let mut seen: usize = 0;
    for index in 0..log.length() {
        let entry: Array = log.get(index).unchecked_into();
        if entry.get(0).as_string().as_deref() == Some(op) {
            if seen == nth {
                return entry.get(1).unchecked_into();
            }
            seen += 1;
        }
    }
    panic!(
        "{op} call #{nth} was never made; recorded ops were {:?}",
        ops(context)
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn a_block_records_its_capacity_and_binding_and_binds_itself() {
    let (context, log): (WebGl2RenderingContext, Object) = recording_context(0.0);

    let block: Option<GlUniformBlock> = GlUniformBlock::create(&context, 128, 3);

    assert!(block.is_some(), "the driver handle came back");
    let created: &GlUniformBlock = block.as_ref().expect("created");
    assert_eq!(
        created.get_capacity(),
        128,
        "the block records the capacity it reserved"
    );
    assert_eq!(
        created.get_binding(),
        3,
        "the block records the binding point the shader reads from"
    );
    let bound: Array = nth_of(&log, "bindBufferBase", 0);
    assert_eq!(
        bound.get(1).as_f64(),
        Some(3.0),
        "creation binds the buffer to the binding point it was given"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn a_zero_sized_request_still_reserves_one_byte_so_the_buffer_is_valid() {
    let (context, _log): (WebGl2RenderingContext, Object) = recording_context(0.0);

    let block: Option<GlUniformBlock> = GlUniformBlock::create(&context, 0, 0);

    assert!(
        block.is_some(),
        "a zero sized request must not fail; the driver refuses a zero byte buffer"
    );
    assert_eq!(
        block.expect("created").get_capacity(),
        1,
        "the capacity is clamped up to one byte so the buffer allocation is legal"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn a_write_inside_the_block_goes_through_buffer_sub_data_without_reallocating() {
    let (context, log): (WebGl2RenderingContext, Object) = recording_context(0.0);
    let mut block: GlUniformBlock = GlUniformBlock::create(&context, 256, 0).expect("created");
    let before: usize = count_of(&log, "bufferData");

    let issued: bool = block.update(&context, 64, &[0; 32]);

    assert!(issued, "a write inside the reserved capacity is issued");
    assert_eq!(
        count_of(&log, "bufferData"),
        before,
        "the existing allocation covers the write, so no reallocation is needed"
    );
    let write: Array = nth_of(&log, "bufferSubData", 0);
    assert_eq!(
        write.get(1).as_f64(),
        Some(64.0),
        "the byte offset the caller asked for reaches the driver"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn a_write_past_the_block_reallocates_and_the_capacity_grows_to_cover_it() {
    let (context, log): (WebGl2RenderingContext, Object) = recording_context(0.0);
    let mut block: GlUniformBlock = GlUniformBlock::create(&context, 64, 0).expect("created");
    let before: usize = count_of(&log, "bufferData");

    let issued: bool = block.update(&context, 0, &[0; 256]);

    assert!(
        issued,
        "a growing write is still issued, after reallocating"
    );
    assert_eq!(
        count_of(&log, "bufferData"),
        before + 1,
        "outgrowing the block is exactly one reallocation"
    );
    assert_eq!(
        block.get_capacity(),
        256,
        "the recorded capacity covers the new end, not the payload length alone"
    );
    let realloc: Array = nth_of(&log, "bufferData", count_of(&log, "bufferData") - 1);
    assert_eq!(
        realloc.get(1).as_f64(),
        Some(256.0),
        "the driver is told the full new size, so the tail is defined memory"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn a_matrix_write_past_the_block_is_refused_rather_than_truncating() {
    let (context, log): (WebGl2RenderingContext, Object) = recording_context(0.0);
    let mut block: GlUniformBlock = GlUniformBlock::create(&context, 16, 0).expect("created");
    let before: usize = count_of(&log, "bufferSubData");
    let matrix: Matrix4x4 = Matrix4x4::identity();

    let accepted: bool = block.set_mat4(&context, 0, &matrix);

    assert!(
        !accepted,
        "a 64 byte transform cannot fit in 16 bytes, so it must be refused rather than truncated"
    );
    assert_eq!(
        count_of(&log, "bufferSubData"),
        before,
        "a refused write must not reach the driver at all"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn a_matrix_that_fits_is_written_as_sixty_four_bytes() {
    let (context, log): (WebGl2RenderingContext, Object) = recording_context(0.0);
    let mut block: GlUniformBlock = GlUniformBlock::create(&context, 256, 0).expect("created");
    let matrix: Matrix4x4 = Matrix4x4::identity();

    let accepted: bool = block.set_mat4(&context, 16, &matrix);

    assert!(accepted, "a 64 byte transform fits in a 256 byte block");
    let write: Array = nth_of(&log, "bufferSubData", 0);
    assert_eq!(
        write.get(1).as_f64(),
        Some(16.0),
        "the record offset the caller gave is the offset used"
    );
    let payload: JsValue = write.get(2);
    let length: u32 = Array::from(&payload).length();
    assert_eq!(
        length, 64,
        "a mat4 is 16 f32 lanes, so exactly 64 bytes reach the driver"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn deleting_a_block_unbinds_its_point_and_zeroes_the_capacity() {
    let (context, log): (WebGl2RenderingContext, Object) = recording_context(0.0);
    let mut block: GlUniformBlock = GlUniformBlock::create(&context, 128, 5).expect("created");
    let before: usize = count_of(&log, "bindBufferBase");

    block.delete(&context);

    assert_eq!(
        count_of(&log, "bindBufferBase"),
        before + 1,
        "delete releases the binding point rather than leaving the buffer bound"
    );
    let released: Array = nth_of(&log, "bindBufferBase", before);
    assert_eq!(
        released.get(1).as_f64(),
        Some(5.0),
        "the released point is the one the block was created with"
    );
    assert!(
        released.get(2).is_undefined(),
        "the buffer is unbound at that point, which is what a None binding means"
    );
    assert_eq!(
        block.get_capacity(),
        0,
        "a deleted block keeps no stale capacity, or a later write targets freed storage"
    );
    assert_eq!(
        count_of(&log, "deleteBuffer"),
        1,
        "the buffer handle is released exactly once"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn a_uniform_block_index_is_resolved_once_and_then_cached() {
    let (context, log): (WebGl2RenderingContext, Object) = recording_context(7.0);
    let mut program: GlProgram =
        GlProgram::create(&context, "vertex body", "fragment body").expect("linked");
    let after_link: usize = count_of(&log, "getUniformBlockIndex");

    let first: bool = program.bind_uniform_block(&context, "Scene", 2);
    let second: bool = program.bind_uniform_block(&context, "Scene", 2);

    assert!(
        first && second,
        "a block the driver knows about binds both times"
    );
    assert_eq!(
        count_of(&log, "getUniformBlockIndex"),
        after_link + 1,
        "the second bind of the same name is served from the cache"
    );
    assert_eq!(
        count_of(&log, "uniformBlockBinding"),
        2,
        "each bind still reaches the driver, cache or not"
    );
    let binding: Array = nth_of(&log, "uniformBlockBinding", 1);
    assert_eq!(
        binding.get(1).as_f64(),
        Some(7.0),
        "the cached index is what the second bind reports, not a fresh lookup"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a WebGl2RenderingContext stand-in, which only exists under wasm"
)]
fn a_uniform_block_the_driver_optimised_out_reports_false_without_binding() {
    let missing: f64 = f64::from(WebGl2RenderingContext::UNIFORM_BLOCK_INDEX);
    let (context, log): (WebGl2RenderingContext, Object) = recording_context(missing);
    let mut program: GlProgram =
        GlProgram::create(&context, "vertex body", "fragment body").expect("linked");

    let bound: bool = program.bind_uniform_block(&context, "Unused", 4);

    assert!(
        !bound,
        "the sentinel index means the driver has no such block, so the bind did not happen"
    );
    assert_eq!(
        count_of(&log, "uniformBlockBinding"),
        0,
        "a missing block must not be bound, or the shader would read whatever occupies the point"
    );
}
