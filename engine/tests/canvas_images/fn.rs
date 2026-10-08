use super::*;

const CANVAS_SPEC: &str =
    r#"[["imageSmoothingEnabled","imageSmoothingEnabled","set"],["drawImage","drawImage","call"]]"#;

fn reflective(target: &JsValue, key: &str, value: &JsValue) {
    let installer: js_sys::Function = js_sys::Function::new_with_args(
        "target, key, value",
        "Object.defineProperty(target, key, { value: value, writable: true, configurable: true });",
    );
    let outcome: Result<JsValue, JsValue> =
        installer.call3(&JsValue::NULL, target, &JsValue::from_str(key), value);
    assert!(outcome.is_ok(), "the stand-in must accept {key}");
}

fn recording_context() -> (CanvasRenderingContext2d, Object) {
    let log: Array = Array::new();
    let context: Object = Object::new();
    reflective(context.as_ref(), "__log", log.as_ref());
    let arm: js_sys::Function = js_sys::Function::new_with_args(
        "target, log, spec",
        "for (const [key, op, mode] of JSON.parse(spec)) { if (mode === 'set') { Object.defineProperty(target, key, { set(value) { log.push([op, [value]]); }, configurable: true }); } else { Object.defineProperty(target, key, { value: function() { log.push([op, Array.from(arguments)]); }, configurable: true }); } }",
    );
    let armed: Result<JsValue, JsValue> = arm.call3(
        &JsValue::NULL,
        context.as_ref(),
        log.as_ref(),
        &JsValue::from_str(CANVAS_SPEC),
    );
    assert!(armed.is_ok(), "the canvas recorder must install cleanly");
    (context.clone().unchecked_into(), context)
}

fn ops(context: &Object) -> Vec<String> {
    let log: Array = Reflect::get(context, &JsValue::from_str("__log"))
        .expect("the log must be readable")
        .unchecked_into();
    (0..log.length())
        .map(|index: u32| {
            let entry: Array = log.get(index).unchecked_into();
            entry.get(0).as_string().unwrap_or_default()
        })
        .collect()
}

fn args_of(context: &Object, op: &str) -> Array {
    let log: Array = Reflect::get(context, &JsValue::from_str("__log"))
        .expect("the log must be readable")
        .unchecked_into();
    for index in 0..log.length() {
        let entry: Array = log.get(index).unchecked_into();
        if entry.get(0).as_string().as_deref() == Some(op) {
            return entry.get(1).unchecked_into();
        }
    }
    panic!("{op} was never called; recorded {:?}", ops(context));
}

fn image(natural_width: i32, natural_height: i32) -> HtmlImageElement {
    let element: Object = Object::new();
    reflective(
        element.as_ref(),
        "naturalWidth",
        &JsValue::from_f64(f64::from(natural_width)),
    );
    reflective(
        element.as_ref(),
        "naturalHeight",
        &JsValue::from_f64(f64::from(natural_height)),
    );
    element.unchecked_into()
}

fn renderer() -> (CanvasRenderer, Object) {
    let (typed, raw): (CanvasRenderingContext2d, Object) = recording_context();
    let built: CanvasRenderer = CanvasRenderer::new(
        typed,
        Camera2D::create(800.0, 600.0),
        RenderQuality::default(),
    );
    (built, raw)
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a js_sys stand-in for a 2D context, which only exists under wasm"
)]
fn an_image_is_drawn_at_the_four_coordinates_it_was_given() {
    let (subject, log): (CanvasRenderer, Object) = renderer();

    subject.draw_image(&image(16, 16), Vector2D::new(5.0, 7.0), 32.0, 48.0);

    let args: Array = args_of(&log, "drawImage");
    assert_eq!(args.get(1).as_f64(), Some(5.0), "x");
    assert_eq!(args.get(2).as_f64(), Some(7.0), "then y");
    assert_eq!(
        args.get(3).as_f64(),
        Some(32.0),
        "then the destination width, which is this call's argument and not the image's own width"
    );
    assert_eq!(
        args.get(4).as_f64(),
        Some(48.0),
        "and the destination height"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a js_sys stand-in for a 2D context, which only exists under wasm"
)]
fn a_sub_region_draw_forwards_the_source_rectangle_before_the_destination() {
    let (subject, log): (CanvasRenderer, Object) = renderer();

    subject.draw_image_rect(
        &image(64, 64),
        Rect::new(1.0, 2.0, 8.0, 16.0),
        Vector2D::new(100.0, 200.0),
        32.0,
        32.0,
    );

    let args: Array = args_of(&log, "drawImage");
    assert_eq!(
        args.length(),
        9,
        "the nine-argument overload is the only one that carries a source rectangle"
    );
    assert_eq!(
        args.get(1).as_f64(),
        Some(1.0),
        "source x comes first; the destination is the fourth pair, not the first"
    );
    assert_eq!(args.get(2).as_f64(), Some(2.0), "source y");
    assert_eq!(args.get(3).as_f64(), Some(8.0), "source width");
    assert_eq!(args.get(4).as_f64(), Some(16.0), "source height");
    assert_eq!(args.get(5).as_f64(), Some(100.0), "destination x");
    assert_eq!(args.get(6).as_f64(), Some(200.0), "destination y");
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a js_sys stand-in for a 2D context, which only exists under wasm"
)]
fn turning_smoothing_on_writes_the_flag_rather_than_leaving_the_driver_default() {
    let (typed, log): (CanvasRenderingContext2d, Object) = recording_context();

    CanvasRenderer::enable_smoothing_on(&typed);

    assert_eq!(
        ops(&log),
        vec![String::from("imageSmoothingEnabled")],
        "the high quality preset is exactly this one property, and it has to be written \
         explicitly because the canvas default is smoothing off"
    );
    let args: Array = args_of(&log, "imageSmoothingEnabled");
    assert_eq!(
        args.get(0).as_bool(),
        Some(true),
        "on means true; writing false here would read as \"turn smoothing off\""
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a js_sys stand-in for a 2D context, which only exists under wasm"
)]
fn a_recorded_sprite_is_queued_rather_than_drawn_immediately() {
    let mut list: DrawList = DrawList::create();

    list.draw_sprite(
        &image(16, 16),
        Rect::new(0.0, 0.0, 8.0, 8.0),
        Transform2D::identity(),
    );

    assert_eq!(list.len(), 1, "one sprite, queued");
    assert!(
        matches!(
            list.get_commands().first(),
            Some(DrawCommand::DrawSprite { .. })
        ),
        "and it is the sprite variant, carrying the source rect and transform rather than a \
         flattened copy"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a js_sys stand-in for a 2D context, which only exists under wasm"
)]
fn a_recorded_image_rect_keeps_the_size_it_was_given() {
    let mut list: DrawList = DrawList::create();

    list.draw_image_rect(
        &image(64, 64),
        Rect::new(2.0, 3.0, 16.0, 16.0),
        Vector2D::new(20.0, 30.0),
        40.0,
        50.0,
    );

    assert_eq!(list.len(), 1, "one queued draw");
    let recorded: &DrawCommand = list
        .get_commands()
        .first()
        .expect("the list is not empty just checked");
    assert!(
        matches!(
            recorded,
            DrawCommand::DrawImageRect {
                dest_width, dest_height, ..
            } if *dest_width == 40.0 && *dest_height == 50.0
        ),
        "the destination size survives into the queued command; recomputing it at flush time \
         would change what gets drawn if the camera had moved"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a js_sys stand-in for a 2D context, which only exists under wasm"
)]
fn the_backend_trait_reaches_the_same_calls_as_the_inherent_draws() {
    fn draw_through_the_trait<B: RenderBackend>(backend: &B, image: &HtmlImageElement) {
        backend.draw_image(image, Vector2D::new(1.0, 2.0), 3.0, 4.0);
    }
    let (subject, log): (CanvasRenderer, Object) = renderer();

    draw_through_the_trait(&subject, &image(8, 8));

    assert_eq!(
        ops(&log),
        vec![String::from("drawImage")],
        "the trait's default method has to land on the same context call as the inherent one, \
         or a caller that only holds a trait object draws nothing"
    );
    let args: Array = args_of(&log, "drawImage");
    assert_eq!(args.get(1).as_f64(), Some(1.0), "and the same coordinates");
}
