use super::*;

const CTX_METHODS: &str = r#"["drawImage","createLinearGradient","createRadialGradient"]"#;

const CTX_PROPERTIES: &str = r#"[
    ["shadowColor", "setShadowColor"],
    ["shadowBlur", "setShadowBlur"],
    ["shadowOffsetX", "setShadowOffsetX"],
    ["shadowOffsetY", "setShadowOffsetY"],
    ["fillStyle", "setFillStyle"],
    ["strokeStyle", "setStrokeStyle"]
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

fn recording_context() -> (CanvasRenderingContext2d, Object) {
    let context: Object = Object::new();
    reflective(context.as_ref(), "__log", Array::new().as_ref());
    let arm: js_sys::Function = js_sys::Function::new_with_args(
        "target, methods, properties",
        "for (const name of JSON.parse(methods)) { target[name] = function() { target.__log.push([name, Array.from(arguments)]); if (name === 'createLinearGradient' || name === 'createRadialGradient') { return { addColorStop: function(position, color) { target.__log.push(['addColorStop', [position, color]]); } }; } return null; }; } for (const [prop, name] of JSON.parse(properties)) { Object.defineProperty(target, prop, { set(value) { target.__log.push([name, [value]]); }, configurable: true }); }",
    );
    let armed: Result<JsValue, JsValue> = arm.call3(
        &JsValue::NULL,
        context.as_ref(),
        &JsValue::from_str(CTX_METHODS),
        &JsValue::from_str(CTX_PROPERTIES),
    );
    assert!(armed.is_ok(), "the canvas recorder must install cleanly");
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

fn last_of(context: &Object, op: &str) -> Array {
    let log: Array = Reflect::get(context, &JsValue::from_str("__log"))
        .expect("log readable")
        .unchecked_into();
    for index in (0..log.length()).rev() {
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

fn last_string(context: &Object, op: &str) -> String {
    last_of(context, op).get(0).as_string().unwrap_or_default()
}

fn renderer(context: &CanvasRenderingContext2d) -> CanvasRenderer {
    CanvasRenderer::new(
        context.clone(),
        Camera2D::create(800.0, 600.0),
        RenderQuality::default(),
    )
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a CanvasRenderingContext2d stand-in, which only exists under wasm"
)]
fn a_linear_gradient_forwards_both_endpoints_and_every_stop_in_order() {
    let (context, log): (CanvasRenderingContext2d, Object) = recording_context();
    let stops: Vec<(f64, String)> = vec![
        (0.0, String::from("#000000")),
        (1.0, String::from("#ffffff")),
    ];
    let gradient: LinearGradient = LinearGradient::create(
        Vector2D::new(0.0, 0.0),
        Vector2D::new(100.0, 0.0),
        stops.clone(),
    );

    let built: Option<CanvasGradient> = gradient.to_gradient(&context);

    assert!(
        built.is_some(),
        "a linear gradient always builds against a live context"
    );
    let created: Array = first_of(&log, "createLinearGradient");
    assert_eq!(
        created.get(0).as_f64(),
        Some(0.0),
        "the start x reaches the context"
    );
    assert_eq!(
        created.get(1).as_f64(),
        Some(0.0),
        "the start y reaches the context"
    );
    assert_eq!(
        created.get(2).as_f64(),
        Some(100.0),
        "the end x reaches the context"
    );
    assert_eq!(
        created.get(3).as_f64(),
        Some(0.0),
        "the end y reaches the context"
    );
    assert_eq!(
        ops(&log)
            .iter()
            .filter(|op: &&String| op.as_str() == "addColorStop")
            .count(),
        2,
        "every stop is added, in the order it was declared"
    );
    let first_stop: Array = first_of(&log, "addColorStop");
    assert_eq!(
        first_stop.get(0).as_f64(),
        Some(0.0),
        "the first stop sits at offset zero"
    );
    assert_eq!(
        first_stop.get(1).as_string(),
        Some(String::from("#000000")),
        "the stop colour is forwarded verbatim"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a CanvasRenderingContext2d stand-in, which only exists under wasm"
)]
fn a_linear_gradient_with_no_stops_still_builds_a_gradient() {
    let (context, log): (CanvasRenderingContext2d, Object) = recording_context();
    let gradient: LinearGradient =
        LinearGradient::create(Vector2D::zero(), Vector2D::new(10.0, 10.0), Vec::new());

    let built: Option<CanvasGradient> = gradient.to_gradient(&context);

    assert!(
        built.is_some(),
        "an empty stop list is still a valid gradient"
    );
    assert!(
        !ops(&log).contains(&String::from("addColorStop")),
        "no stops were declared, so none may be added"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a CanvasRenderingContext2d stand-in, which only exists under wasm"
)]
fn a_radial_gradient_forwards_both_circles_not_just_the_inner_one() {
    let (context, log): (CanvasRenderingContext2d, Object) = recording_context();
    let gradient: RadialGradient = RadialGradient::new(
        Vector2D::new(10.0, 10.0),
        5.0,
        Vector2D::new(20.0, 20.0),
        25.0,
        vec![(0.5, String::from("#ff0000"))],
    );

    let built: Option<CanvasGradient> = gradient.to_gradient(&context);

    assert!(
        built.is_some(),
        "a radial gradient always builds against a live context"
    );
    let created: Array = first_of(&log, "createRadialGradient");
    assert_eq!(
        created.length(),
        6,
        "a radial gradient is two circles, so six arguments"
    );
    assert_eq!(
        created.get(0).as_f64(),
        Some(10.0),
        "the inner centre x reaches the context"
    );
    assert_eq!(
        created.get(1).as_f64(),
        Some(10.0),
        "the inner centre y reaches the context"
    );
    assert_eq!(
        created.get(2).as_f64(),
        Some(5.0),
        "the inner radius reaches the context"
    );
    assert_eq!(
        created.get(3).as_f64(),
        Some(20.0),
        "the outer centre x reaches the context"
    );
    assert_eq!(
        created.get(4).as_f64(),
        Some(20.0),
        "the outer centre y reaches the context"
    );
    assert_eq!(
        created.get(5).as_f64(),
        Some(25.0),
        "the outer radius reaches the context"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a CanvasRenderingContext2d stand-in, which only exists under wasm"
)]
fn a_gradient_fill_reaches_fill_style_and_a_gradient_stroke_reaches_stroke_style() {
    let (context, log): (CanvasRenderingContext2d, Object) = recording_context();
    let subject: CanvasRenderer = renderer(&context);
    let linear: LinearGradient =
        LinearGradient::create(Vector2D::zero(), Vector2D::new(1.0, 1.0), Vec::new());
    let radial: RadialGradient = RadialGradient::new(
        Vector2D::zero(),
        1.0,
        Vector2D::new(2.0, 2.0),
        3.0,
        Vec::new(),
    );

    subject.set_linear_gradient_fill(&linear);
    subject.set_radial_gradient_fill(&radial);
    subject.set_linear_gradient_stroke(&linear);
    subject.set_radial_gradient_stroke(&radial);

    let recorded: Vec<String> = ops(&log);
    assert_eq!(
        recorded
            .iter()
            .filter(|op: &&String| op.as_str() == "setFillStyle")
            .count(),
        2,
        "both fill styles are applied, and neither leaks into the stroke style"
    );
    assert_eq!(
        recorded
            .iter()
            .filter(|op: &&String| op.as_str() == "setStrokeStyle")
            .count(),
        2,
        "both stroke styles are applied, and neither leaks into the fill style"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a CanvasRenderingContext2d stand-in, which only exists under wasm"
)]
fn a_shadow_forwards_all_four_fields_and_clearing_zeroes_them() {
    let (context, log): (CanvasRenderingContext2d, Object) = recording_context();
    let subject: CanvasRenderer = renderer(&context);
    let config: ShadowConfig =
        ShadowConfig::new(String::from("rgba(0, 0, 0, 0.5)"), 4.0, 2.0, -2.0);

    subject.set_shadow(&config);

    assert_eq!(
        last_string(&log, "setShadowColor"),
        String::from("rgba(0, 0, 0, 0.5)"),
        "the shadow colour is forwarded verbatim"
    );
    assert_eq!(
        last_of(&log, "setShadowBlur").get(0).as_f64(),
        Some(4.0),
        "the blur reaches the context"
    );
    assert_eq!(
        last_of(&log, "setShadowOffsetX").get(0).as_f64(),
        Some(2.0),
        "a positive x offset survives the trip"
    );
    assert_eq!(
        last_of(&log, "setShadowOffsetY").get(0).as_f64(),
        Some(-2.0),
        "a negative y offset survives the trip rather than being clamped"
    );

    subject.clear_shadow();

    assert_eq!(
        last_of(&log, "setShadowBlur").get(0).as_f64(),
        Some(0.0),
        "clearing the shadow zeroes the blur"
    );
    assert_eq!(
        last_of(&log, "setShadowOffsetX").get(0).as_f64(),
        Some(0.0),
        "clearing the shadow zeroes the x offset"
    );
    assert_eq!(
        last_of(&log, "setShadowOffsetY").get(0).as_f64(),
        Some(0.0),
        "clearing the shadow zeroes the y offset"
    );
}
