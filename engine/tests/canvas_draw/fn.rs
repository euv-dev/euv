use super::*;

const CTX_METHODS: &str = r#"[
    "translate","scale","rotate","setTransform","save","restore","clearRect",
    "drawImage"
]"#;

const CTX_PROPERTIES: &str = r#"[["globalAlpha","setGlobalAlpha"],["fillStyle","setFillStyle"]]"#;

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

fn recording_context() -> (CanvasRenderingContext2d, Object) {
    let context: Object = Object::new();
    reflective(context.as_ref(), "__log", Array::new().as_ref());
    let arm: js_sys::Function = js_sys::Function::new_with_args(
        "target, methods, properties",
        "for (const name of JSON.parse(methods)) { target[name] = function() { target.__log.push([name, Array.from(arguments)]); return null; }; } for (const [prop, name] of JSON.parse(properties)) { Object.defineProperty(target, prop, { set(value) { target.__log.push([name, [value]]); }, configurable: true }); }",
    );
    let armed: Result<JsValue, JsValue> = arm.call3(
        &JsValue::NULL,
        context.as_ref(),
        &JsValue::from_str(CTX_METHODS),
        &JsValue::from_str(CTX_PROPERTIES),
    );
    assert!(armed.is_ok(), "the draw recorder must install cleanly");
    let typed: CanvasRenderingContext2d = context.clone().unchecked_into();
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

fn renderer_with(camera: Camera2D) -> (CanvasRenderer, CanvasRenderingContext2d, Object) {
    let (context, log): (CanvasRenderingContext2d, Object) = recording_context();
    let built: CanvasRenderer =
        CanvasRenderer::new(context.clone(), camera, RenderQuality::default());
    (built, context, log)
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a CanvasRenderingContext2d stand-in, which only exists under wasm"
)]
fn the_camera_transform_moves_to_the_centre_scales_rotates_then_offsets_back() {
    let (subject, _context, log): (CanvasRenderer, CanvasRenderingContext2d, Object) =
        renderer_with(Camera2D::create(800.0, 600.0));

    subject.apply_camera();

    let recorded: Vec<String> = ops(&log);
    assert_eq!(
        recorded,
        vec![
            String::from("translate"),
            String::from("scale"),
            String::from("rotate"),
            String::from("translate"),
        ],
        "the order is the whole point: centre, scale, rotate, then undo the centre"
    );
    let first: Array = nth_of(&log, "translate", 0);
    assert_eq!(
        first.get(0).as_f64(),
        Some(400.0),
        "the first translate is half the viewport width"
    );
    assert_eq!(
        first.get(1).as_f64(),
        Some(300.0),
        "and half the viewport height"
    );
    let scaled: Array = nth_of(&log, "scale", 0);
    assert_eq!(
        scaled.get(0).as_f64(),
        scaled.get(1).as_f64(),
        "zoom is uniform, so both axes get the same factor"
    );
    assert_eq!(
        scaled.get(0).as_f64(),
        Some(1.0),
        "the default camera is unzoomed, so this pins the factor to a known value rather than \
         only checking that the two axes agree"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a CanvasRenderingContext2d stand-in, which only exists under wasm"
)]
fn a_camera_with_a_nonzero_zoom_scales_by_that_factor() {
    let mut camera: Camera2D = Camera2D::create(800.0, 600.0);
    camera.set_zoom(2.5);
    let (subject, _context, log): (CanvasRenderer, CanvasRenderingContext2d, Object) =
        renderer_with(camera);

    subject.apply_camera();

    let scaled: Array = nth_of(&log, "scale", 0);
    assert_eq!(
        scaled.get(0).as_f64(),
        Some(2.5),
        "the zoom the caller set is the factor both axes are scaled by"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a CanvasRenderingContext2d stand-in, which only exists under wasm"
)]
fn the_camera_offset_undoes_the_centre_translate() {
    let mut camera: Camera2D = Camera2D::create(800.0, 600.0);
    camera.set_position(Vector2D::new(100.0, 50.0));
    let (subject, _context, log): (CanvasRenderer, CanvasRenderingContext2d, Object) =
        renderer_with(camera);

    subject.apply_camera();

    let undo: Array = nth_of(&log, "translate", 1);
    assert_eq!(
        undo.get(0).as_f64(),
        Some(-100.0),
        "the second translate is the negated camera position in x"
    );
    assert_eq!(
        undo.get(1).as_f64(),
        Some(-50.0),
        "and the negated camera position in y"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a CanvasRenderingContext2d stand-in, which only exists under wasm"
)]
fn the_render_backend_trait_reaches_the_same_calls_as_the_inherent_methods() {
    let (subject, _context, log): (CanvasRenderer, CanvasRenderingContext2d, Object) =
        renderer_with(Camera2D::create(800.0, 600.0));

    let backend: &CanvasRenderer = &subject;
    backend.save();
    backend.set_fill_color("#123456");
    backend.restore();

    let recorded: Vec<String> = ops(&log);
    assert!(
        recorded.contains(&String::from("save")) && recorded.contains(&String::from("restore")),
        "the trait is a thin forward, so it emits exactly the underlying calls: {recorded:?}"
    );
    assert_eq!(count_of(&log, "save"), 1, "one save reaches the context");
    assert_eq!(
        count_of(&log, "restore"),
        1,
        "one restore reaches the context"
    );
    assert!(
        recorded.contains(&String::from("setFillStyle")),
        "the fill colour goes through the same context property as the inherent setter"
    );
}
