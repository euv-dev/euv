use super::*;

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

fn image(width: f64, height: f64) -> HtmlImageElement {
    let element: Object = Object::new();
    reflective(element.as_ref(), "width", &JsValue::from_f64(width));
    reflective(element.as_ref(), "height", &JsValue::from_f64(height));
    element.unchecked_into()
}

fn recording_context() -> (CanvasRenderingContext2d, Object) {
    let context: Object = Object::new();
    reflective(context.as_ref(), "__log", Array::new().as_ref());
    let arm: js_sys::Function = js_sys::Function::new_with_args(
        "target",
        "target.setTransform = function() { target.__log.push(['setTransform', Array.from(arguments)]); }; target.drawImage = function() { target.__log.push(['drawImage', Array.from(arguments)]); };",
    );
    let armed: Result<JsValue, JsValue> = arm.call1(&JsValue::NULL, context.as_ref());
    assert!(
        armed.is_ok(),
        "the sprite draw recorder must install cleanly"
    );
    let typed: CanvasRenderingContext2d = context.clone().unchecked_into();
    (typed, context)
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

fn log_len(log: &Object) -> u32 {
    Reflect::get(log, &JsValue::from_str("__log"))
        .expect("log readable")
        .unchecked_into::<Array>()
        .length()
}

fn sheet() -> SpriteSheet {
    SpriteSheet::from_image(image(64.0, 32.0), 16.0, 16.0)
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a CanvasRenderingContext2d stand-in, which only exists under wasm"
)]
fn a_frame_is_drawn_between_a_transform_and_its_reset() {
    let (context, log): (CanvasRenderingContext2d, Object) = recording_context();
    let subject: SpriteSheet = sheet();
    let before: u32 = log_len(&log);

    subject.draw_frame(&context, 0, &Transform2D::default());

    assert_eq!(
        ops_after(&log, before),
        vec![
            String::from("setTransform"),
            String::from("drawImage"),
            String::from("setTransform"),
        ],
        "the transform is composed, the sprite is drawn, then the context is handed back"
    );
    let reset: Array = args_after(&log, "setTransform", before + 2);
    assert_eq!(
        reset.get(0).as_f64(),
        Some(1.0),
        "the reset is the identity matrix, so a later draw is not scaled by this sprite"
    );
    assert_eq!(reset.get(1).as_f64(), Some(0.0), "with no shear");
    assert_eq!(reset.get(4).as_f64(), Some(0.0), "and no translation");
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a CanvasRenderingContext2d stand-in, which only exists under wasm"
)]
fn the_destination_is_centred_on_the_transform_position() {
    let (context, log): (CanvasRenderingContext2d, Object) = recording_context();
    let subject: SpriteSheet = sheet();
    let before: u32 = log_len(&log);

    subject.draw_frame(&context, 0, &Transform2D::default());

    let draw: Array = args_after(&log, "drawImage", before);
    assert_eq!(
        draw.get(5).as_f64(),
        Some(-8.0),
        "a 16 wide frame is offset by half its width so the sprite straddles its position"
    );
    assert_eq!(
        draw.get(6).as_f64(),
        Some(-8.0),
        "and half its height on the other axis"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a CanvasRenderingContext2d stand-in, which only exists under wasm"
)]
fn the_source_rectangle_follows_the_frame_index() {
    let (context, log): (CanvasRenderingContext2d, Object) = recording_context();
    let subject: SpriteSheet = sheet();

    let first: u32 = log_len(&log);
    subject.draw_frame(&context, 1, &Transform2D::default());
    let first_draw: Array = args_after(&log, "drawImage", first);

    let second: u32 = log_len(&log);
    subject.draw_frame(&context, 2, &Transform2D::default());
    let second_draw: Array = args_after(&log, "drawImage", second);

    assert_eq!(
        first_draw.get(1).as_f64(),
        Some(16.0),
        "frame one of a 16 pixel grid starts one column in"
    );
    assert_eq!(
        second_draw.get(1).as_f64(),
        Some(32.0),
        "and frame two starts two columns in"
    );
    assert_eq!(
        second_draw.get(2).as_f64(),
        Some(0.0),
        "a frame index within the first row does not move down"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a CanvasRenderingContext2d stand-in, which only exists under wasm"
)]
fn the_transform_rotation_reaches_the_matrix() {
    let (context, log): (CanvasRenderingContext2d, Object) = recording_context();
    let subject: SpriteSheet = sheet();
    let before: u32 = log_len(&log);
    let mut transform: Transform2D = Transform2D::default();
    transform.rotate(PI / 2.0);

    subject.draw_frame(&context, 0, &transform);

    let matrix: Array = args_after(&log, "setTransform", before);
    let a: f64 = matrix.get(0).as_f64().unwrap_or(-1.0);
    let b: f64 = matrix.get(1).as_f64().unwrap_or(-1.0);
    assert!(
        a.abs() < 1e-9,
        "a quarter turn maps the x axis onto the y axis, so a is cos(90) = 0, got {a}"
    );
    assert!(
        (b - 1.0).abs() < 1e-9,
        "and the x axis now points along y, so b is sin(90) = 1, got {b}"
    );
}
