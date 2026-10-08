use super::*;

const DRAW_NAMES: &str = r#"[
    "drawArraysInstanced","drawElementsInstanced","drawRangeElements","drawElements"
]"#;

fn recording_context() -> (WebGl2RenderingContext, Array) {
    let log: Array = Array::new();
    let context: Object = Object::new();
    let arm: js_sys::Function = js_sys::Function::new_with_args(
        "target, log, names",
        "for (const name of JSON.parse(names)) { target[name] = function() { log.push([name, Array.from(arguments)]); }; }",
    );
    let armed: Result<JsValue, JsValue> = arm.call3(
        &JsValue::NULL,
        context.as_ref(),
        log.as_ref(),
        &JsValue::from_str(DRAW_NAMES),
    );
    assert!(armed.is_ok(), "the draw recorder must install cleanly");
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

fn count_after(log: &Array, op: &str, from: u32) -> usize {
    calls_after(log, from)
        .iter()
        .filter(|recorded: &&String| recorded.as_str() == op)
        .count()
}

fn args_after(log: &Array, op: &str, from: u32) -> Array {
    for index in from..log.length() {
        let entry: Array = log.get(index).unchecked_into();
        if entry.get(0).as_string().as_deref() == Some(op) {
            return entry.get(1).unchecked_into();
        }
    }
    panic!(
        "{op} was never called after that point; recorded {:?}",
        calls_after(log, from)
    );
}

fn indexed(base_vertex: i32) -> DrawIndexedArgs {
    let mut args: DrawIndexedArgs = DrawIndexedArgs::new();
    args.set_index_count(36);
    args.set_instance_count(4);
    args.set_first_index(6);
    args.set_base_vertex(base_vertex);
    args.set_first_instance(0);
    args
}

fn stream(vertex_count: u32, instances: u32) -> DrawArgs {
    let mut args: DrawArgs = DrawArgs::new();
    args.set_first_vertex(3);
    args.set_vertex_count(vertex_count);
    args.set_instance_count(instances);
    args.set_first_instance(0);
    args
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a js_sys stand-in for a WebGL2 context, which only exists under wasm"
)]
fn an_inverted_element_range_never_reaches_the_driver() {
    let (context, log): (WebGl2RenderingContext, Array) = recording_context();
    let subject: WebGl2Backend = WebGl2Backend::from_init(WebGl2BackendInit {
        canvas: Object::new().unchecked_into(),
        context: context.clone(),
        width: 800,
        height: 600,
        clear_color: Color::new(0.0, 0.0, 0.0, 1.0),
    });
    let from: u32 = log.length();

    let drew: bool = subject.draw_element_range(
        &context,
        PrimitiveTopology::TriangleList,
        10,
        4,
        IndexFormat::Uint16,
    );

    assert!(!drew, "an end below the start has no vertices to draw");
    assert_eq!(
        calls_after(&log, from),
        Vec::<String>::new(),
        "and computing end - start + 1 on an inverted range wraps around to a huge count, \
         which the driver would happily try to draw"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a js_sys stand-in for a WebGL2 context, which only exists under wasm"
)]
fn an_element_range_counts_both_of_its_ends() {
    let (context, log): (WebGl2RenderingContext, Array) = recording_context();
    let subject: WebGl2Backend = WebGl2Backend::from_init(WebGl2BackendInit {
        canvas: Object::new().unchecked_into(),
        context: context.clone(),
        width: 800,
        height: 600,
        clear_color: Color::new(0.0, 0.0, 0.0, 1.0),
    });
    let from: u32 = log.length();

    let drew: bool = subject.draw_element_range(
        &context,
        PrimitiveTopology::TriangleList,
        4,
        10,
        IndexFormat::Uint16,
    );

    assert!(drew, "a well-ordered range is drawn");
    let args: Array = args_after(&log, "drawRangeElements", from);
    assert_eq!(args.get(1).as_f64(), Some(4.0), "the start index");
    assert_eq!(args.get(2).as_f64(), Some(10.0), "then the end index");
    assert_eq!(
        args.get(3).as_f64(),
        Some(7.0),
        "the count is inclusive of both ends - 10 - 4 + 1 - because drawRangeElements treats \
         end as the last index, not the one past it"
    );
    assert_eq!(
        args.get(5).as_f64(),
        Some(0.0),
        "and a byte offset of zero, since a range is addressed by index rather than by offset"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a js_sys stand-in for a WebGL2 context, which only exists under wasm"
)]
fn a_single_instanced_array_draw_still_uses_the_instanced_entry_point() {
    let (context, log): (WebGl2RenderingContext, Array) = recording_context();
    let subject: WebGl2Backend = WebGl2Backend::from_init(WebGl2BackendInit {
        canvas: Object::new().unchecked_into(),
        context: context.clone(),
        width: 800,
        height: 600,
        clear_color: Color::new(0.0, 0.0, 0.0, 1.0),
    });
    let from: u32 = log.length();

    subject.draw_arrays_instanced(&context, PrimitiveTopology::TriangleList, &stream(6, 1));

    assert_eq!(
        count_after(&log, "drawArraysInstanced", from),
        1,
        "this entry point exists for GPU-generated geometry, so even one instance goes through \
         the instanced call rather than falling back to drawArrays"
    );
    let args: Array = args_after(&log, "drawArraysInstanced", from);
    assert_eq!(args.get(1).as_f64(), Some(3.0), "the first vertex offset");
    assert_eq!(args.get(2).as_f64(), Some(6.0), "then the vertex count");
    assert_eq!(
        args.get(3).as_f64(),
        Some(1.0),
        "and an instance count of one, never zero: a zero here draws nothing at all"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a js_sys stand-in for a WebGL2 context, which only exists under wasm"
)]
fn an_instanced_indexed_draw_uses_the_instanced_call_and_converts_the_index_offset() {
    let (context, log): (WebGl2RenderingContext, Array) = recording_context();
    let subject: WebGl2Backend = WebGl2Backend::from_init(WebGl2BackendInit {
        canvas: Object::new().unchecked_into(),
        context: context.clone(),
        width: 800,
        height: 600,
        clear_color: Color::new(0.0, 0.0, 0.0, 1.0),
    });
    let from: u32 = log.length();

    let drew: bool = subject.draw_elements_instanced(
        &context,
        PrimitiveTopology::TriangleList,
        &indexed(0),
        IndexFormat::Uint32,
    );

    assert!(drew, "a flat base vertex is drawable");
    let args: Array = args_after(&log, "drawElementsInstanced", from);
    assert_eq!(
        args.get(1).as_f64(),
        Some(36.0),
        "the index count comes first"
    );
    assert_eq!(
        args.get(3).as_f64(),
        Some(24.0),
        "the first index is converted to a byte offset - index 6 times the four-byte stride of \
         a uint32 index buffer - because this WebGL entry point takes no index format"
    );
    assert_eq!(
        args.get(4).as_f64(),
        Some(4.0),
        "and the instance count follows the offset"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a js_sys stand-in for a WebGL2 context, which only exists under wasm"
)]
fn an_instanced_indexed_draw_with_a_base_vertex_is_refused_like_its_single_instanced_twin() {
    let (context, log): (WebGl2RenderingContext, Array) = recording_context();
    let subject: WebGl2Backend = WebGl2Backend::from_init(WebGl2BackendInit {
        canvas: Object::new().unchecked_into(),
        context: context.clone(),
        width: 800,
        height: 600,
        clear_color: Color::new(0.0, 0.0, 0.0, 1.0),
    });
    let from: u32 = log.length();

    let drew: bool = subject.draw_elements_instanced(
        &context,
        PrimitiveTopology::TriangleList,
        &indexed(4),
        IndexFormat::Uint32,
    );

    assert!(
        !drew,
        "WebGL has no base-vertex argument, so a non-zero one cannot be honored"
    );
    assert_eq!(
        calls_after(&log, from),
        Vec::<String>::new(),
        "and nothing may be issued at all: silently dropping the base vertex would draw the \
         wrong geometry instead of reporting that it could not be drawn"
    );
}
