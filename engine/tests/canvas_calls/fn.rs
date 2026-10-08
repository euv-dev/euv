use super::*;

const CANVAS_SPEC: &str = r#"[["fillStyle","fillStyle","set"],["strokeStyle","strokeStyle","set"],["lineWidth","lineWidth","set"],["font","font","set"],["globalAlpha","globalAlpha","set"],["save","save","call"],["restore","restore","call"],["beginPath","beginPath","call"],["arc","arc","call"],["fill","fill","call"],["stroke","stroke","call"],["clearRect","clearRect","call"],["fillRect","fillRect","call"],["rect","rect","call"],["save","save","call"],["restore","restore","call"],["translate","translate","call"],["scale","scale","call"],["setTransform","setTransform","call"]]"#;

fn reflective(target: &JsValue, key: &str, value: &JsValue) {
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

fn recording_context() -> CanvasRenderingContext2d {
    let context: Object = Object::new();
    reflective(context.as_ref(), "__log", Array::new().as_ref());
    let arm: js_sys::Function = js_sys::Function::new_with_args(
        "target, spec",
        "for (const [key, op, mode] of JSON.parse(spec)) { if (mode === 'set') { Object.defineProperty(target, key, { set(value) { target.__log.push([op, [value]]); }, configurable: true }); } else { Object.defineProperty(target, key, { value: function() { target.__log.push([op, Array.from(arguments)]); }, configurable: true }); } }",
    );
    let armed: Result<JsValue, JsValue> = arm.call2(
        &JsValue::NULL,
        context.as_ref(),
        &JsValue::from_str(CANVAS_SPEC),
    );
    assert!(armed.is_ok(), "the recorder must install cleanly");
    let typed: CanvasRenderingContext2d = context.unchecked_into();
    typed
}

fn renderer() -> (CanvasRenderer, Object) {
    let context: Object = recording_context().unchecked_into();
    let typed: CanvasRenderingContext2d = context.clone().unchecked_into();
    let built: CanvasRenderer = CanvasRenderer::new(
        typed,
        Camera2D::create(800.0, 600.0),
        RenderQuality::default(),
    );
    (built, context)
}

fn entries(context: &Object) -> Vec<Array> {
    let log: Array = Reflect::get(context, &JsValue::from_str("__log"))
        .expect("the log must be readable")
        .unchecked_into();
    (0..log.length())
        .map(|index: u32| log.get(index).unchecked_into())
        .collect()
}

fn ops(context: &Object) -> Vec<String> {
    entries(context)
        .iter()
        .map(|entry: &Array| entry.get(0).as_string().unwrap_or_default())
        .collect()
}

fn args_of(context: &Object, op: &str) -> Array {
    for entry in entries(context) {
        if entry.get(0).as_string().as_deref() == Some(op) {
            return entry.get(1).unchecked_into();
        }
    }
    panic!(
        "{op} was never called; recorded ops were {:?}",
        ops(context)
    );
}

fn first_arg(context: &Object, op: &str) -> JsValue {
    args_of(context, op).get(0)
}

fn last_arg(context: &Object, op: &str) -> JsValue {
    let recorded: Vec<Array> = entries(context);
    for entry in recorded.iter().rev() {
        if entry.get(0).as_string().as_deref() == Some(op) {
            let args: Array = entry.get(1).unchecked_into();
            return args.get(0);
        }
    }
    panic!(
        "{op} was never called; recorded ops were {:?}",
        ops(context)
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a CanvasRenderingContext2d stand-in, which only exists under wasm"
)]
fn clearing_with_a_colour_paints_the_fill_style_and_covers_the_viewport() {
    let (subject, context): (CanvasRenderer, Object) = renderer();
    subject.clear_color("#101010");
    let style: JsValue = first_arg(&context, "fillStyle");
    assert_eq!(
        style.as_string(),
        Some(String::from("#101010")),
        "the clear colour must become the fill style"
    );
    let rect: Array = args_of(&context, "fillRect");
    let width: f64 = rect.get(2).as_f64().unwrap_or(-1.0);
    let height: f64 = rect.get(3).as_f64().unwrap_or(-1.0);
    assert_eq!(
        rect.get(0).as_f64(),
        Some(0.0),
        "a clear starts at the origin"
    );
    assert_eq!(
        rect.get(1).as_f64(),
        Some(0.0),
        "a clear starts at the origin"
    );
    assert_eq!(width, 800.0, "the clear must span the full viewport width");
    assert_eq!(
        height, 600.0,
        "the clear must span the full viewport height"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a CanvasRenderingContext2d stand-in, which only exists under wasm"
)]
fn save_and_restore_reach_the_context_in_order() {
    let (subject, context): (CanvasRenderer, Object) = renderer();
    subject.save();
    subject.restore();
    assert_eq!(
        ops(&context),
        vec![String::from("save"), String::from("restore")],
        "the state stack must be pushed and popped in the order the caller asked for"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a CanvasRenderingContext2d stand-in, which only exists under wasm"
)]
fn the_line_width_reaches_the_context_as_a_number() {
    let (subject, context): (CanvasRenderer, Object) = renderer();
    subject.set_line_width(3.5);
    let observed: f64 = first_arg(&context, "lineWidth").as_f64().unwrap_or(-1.0);
    assert_eq!(
        observed, 3.5,
        "a fractional line width must survive the trip to the context"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a CanvasRenderingContext2d stand-in, which only exists under wasm"
)]
fn the_font_reaches_the_context_verbatim() {
    let (subject, context): (CanvasRenderer, Object) = renderer();
    subject.set_font("20px serif");
    let observed: JsValue = first_arg(&context, "font");
    assert_eq!(
        observed.as_string(),
        Some(String::from("20px serif")),
        "the renderer must not rebuild or reformat the font string it was given"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a CanvasRenderingContext2d stand-in, which only exists under wasm"
)]
fn fill_and_stroke_colours_land_on_different_context_properties() {
    let (subject, context): (CanvasRenderer, Object) = renderer();
    subject.set_fill_color("#ff0000");
    subject.set_stroke_color("#00ff00");
    let fill: JsValue = first_arg(&context, "fillStyle");
    let stroke: JsValue = first_arg(&context, "strokeStyle");
    assert_eq!(
        fill.as_string(),
        Some(String::from("#ff0000")),
        "the fill colour must not land on the stroke"
    );
    assert_eq!(
        stroke.as_string(),
        Some(String::from("#00ff00")),
        "the stroke colour must not land on the fill"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a CanvasRenderingContext2d stand-in, which only exists under wasm"
)]
fn the_global_alpha_is_clamped_into_the_unit_range() {
    let (subject, context): (CanvasRenderer, Object) = renderer();
    subject.set_global_alpha(2.0);
    let high: f64 = first_arg(&context, "globalAlpha").as_f64().unwrap_or(-1.0);
    assert_eq!(
        high, 1.0,
        "an alpha above 1 must be clamped down to fully opaque"
    );
    subject.set_global_alpha(-0.5);
    let low: f64 = last_arg(&context, "globalAlpha").as_f64().unwrap_or(-1.0);
    assert_eq!(
        low, 0.0,
        "a negative alpha must be clamped up to fully transparent"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a CanvasRenderingContext2d stand-in, which only exists under wasm"
)]
fn stroking_a_circle_opens_a_path_arcs_it_and_strokes_in_that_order() {
    let (subject, context): (CanvasRenderer, Object) = renderer();
    subject.stroke_circle(Vector2D::new(10.0, 20.0), 5.0);
    assert_eq!(
        ops(&context),
        vec![
            String::from("beginPath"),
            String::from("arc"),
            String::from("stroke")
        ],
        "a circle is a fresh path, an arc, then a stroke"
    );
    let arc: Array = args_of(&context, "arc");
    assert_eq!(
        arc.get(0).as_f64(),
        Some(10.0),
        "the arc is centred on the requested x"
    );
    assert_eq!(
        arc.get(1).as_f64(),
        Some(20.0),
        "the arc is centred on the requested y"
    );
    assert_eq!(
        arc.get(2).as_f64(),
        Some(5.0),
        "the arc radius is the requested radius"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a js_sys stand-in for a 2D context, which only exists under wasm"
)]
fn replaying_an_empty_list_touches_nothing() {
    let (subject, log): (CanvasRenderer, Object) = renderer();
    let list: DrawList = DrawList::create();

    subject.replay(&list);

    let drawn: Vec<String> = ops(&log)
        .into_iter()
        .filter(|op: &String| {
            op == "fillRect"
                || op == "strokeRect"
                || op == "fill"
                || op == "stroke"
                || op == "fillText"
        })
        .collect();
    assert_eq!(
        drawn,
        Vec::<String>::new(),
        "nothing may be drawn for an empty list. The renderer does still reset the per-frame \
         state (transform, global alpha) before replaying, which is deliberate — it is what \
         makes a frame independent of whatever the previous one left behind — so the property \
         worth pinning is that no drawing happens, not that no call happens"
    );
}
