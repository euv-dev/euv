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
        "target.drawImage = function() { target.__log.push(Array.from(arguments)); };",
    );
    let armed: Result<JsValue, JsValue> = arm.call1(&JsValue::NULL, context.as_ref());
    assert!(armed.is_ok(), "the sprite recorder must install cleanly");
    let typed: CanvasRenderingContext2d = context.clone().unchecked_into();
    (typed, context)
}

fn draw_calls(log: &Object) -> Vec<Array> {
    let entries: Array = Reflect::get(log, &JsValue::from_str("__log"))
        .expect("log readable")
        .unchecked_into();
    (0..entries.length())
        .map(|index: u32| entries.get(index).unchecked_into())
        .collect()
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "stands in an HtmlImageElement as a plain object"
)]
fn a_sprite_sheet_counts_columns_and_rows_from_the_image_and_the_frame_size() {
    let sheet: SpriteSheet = SpriteSheet::from_image(image(64.0, 32.0), 16.0, 16.0);

    assert_eq!(
        sheet.get_columns(),
        4,
        "64 wide in 16 pixel frames is four columns"
    );
    assert_eq!(
        sheet.get_rows(),
        2,
        "32 tall in 16 pixel frames is two rows"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "stands in an HtmlImageElement as a plain object"
)]
fn a_frame_size_larger_than_the_image_still_yields_one_column_and_one_row() {
    let sheet: SpriteSheet = SpriteSheet::from_image(image(8.0, 8.0), 16.0, 16.0);

    assert_eq!(
        sheet.get_columns(),
        1,
        "a frame wider than the image cannot divide down to zero frames"
    );
    assert_eq!(sheet.get_rows(), 1, "the same holds for the row count");
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "stands in an HtmlImageElement as a plain object"
)]
fn a_nine_slice_splits_its_source_into_nine_patches_in_reading_order() {
    let nine: NineSlice = NineSlice::from_image(image(30.0, 30.0), 10.0);

    let rects: NineSliceRects = nine.source_rects();

    assert_eq!(rects.to_vec().len(), 9, "three by three patches");
    let first: Rect = rects.to_vec()[0];
    assert_eq!(
        (first.get_x(), first.get_y()),
        (0.0, 0.0),
        "reading order starts at the top left corner"
    );
    let middle: Rect = rects.to_vec()[4];
    assert_eq!(
        (middle.get_x(), middle.get_y()),
        (10.0, 10.0),
        "and the middle patch sits at the inset on both axes"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "stands in an HtmlImageElement as a plain object"
)]
fn the_corners_and_edges_keep_their_border_size_while_the_middle_stretches() {
    let nine: NineSlice = NineSlice::from_image(image(30.0, 30.0), 10.0);
    let dest: Rect = Rect::new(0.0, 0.0, 100.0, 50.0);

    let rects: Vec<Rect> = nine.dest_rects(dest).to_vec();
    let top_left: Rect = rects[0];
    assert_eq!(
        (top_left.get_width(), top_left.get_height()),
        (10.0, 10.0),
        "a corner never stretches, it keeps the border inset"
    );
    let middle: Rect = rects[4];
    assert_eq!(
        middle.get_width(),
        80.0,
        "while the middle column takes all the width the corners did not"
    );
    assert_eq!(
        middle.get_height(),
        30.0,
        "and the middle row the height they did not"
    );
    let first_row: f64 = rects[0..3].iter().map(|r: &Rect| r.get_width()).sum();
    assert!(
        (first_row - 100.0).abs() < 1e-9,
        "one row of the grid has to add up to the destination width, got {first_row}"
    );
    let first_column: f64 = rects.iter().step_by(3).map(|r: &Rect| r.get_height()).sum();
    assert!(
        (first_column - 50.0).abs() < 1e-9,
        "and one column of the grid to its height, got {first_column}"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "records a CanvasRenderingContext2d stand-in, which only exists under wasm"
)]
fn drawing_a_nine_slice_issues_one_draw_per_patch() {
    let (context, log): (CanvasRenderingContext2d, Object) = recording_context();
    let nine: NineSlice = NineSlice::from_image(image(30.0, 30.0), 10.0);

    nine.draw_into(&context, Rect::new(0.0, 0.0, 60.0, 60.0));

    assert_eq!(
        draw_calls(&log).len(),
        9,
        "one drawImage per patch, or the nine-slice silently renders as a single quad"
    );
}

fn sized_image(width: f64, height: f64) -> HtmlImageElement {
    let element: Object = Object::new();
    for (key, value) in [("naturalWidth", width), ("naturalHeight", height)] {
        let _: Result<bool, JsValue> = Reflect::set(
            element.as_ref(),
            &JsValue::from_str(key),
            &JsValue::from_f64(value),
        );
    }
    element.unchecked_into()
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "needs a stand-in HtmlImageElement to report an intrinsic size, which only \
              exists under wasm"
)]
fn an_atlas_uv_is_expressed_in_fractions_of_the_image_rather_than_pixels() {
    let atlas: SpriteAtlas = SpriteAtlas::create(sized_image(128.0, 64.0));

    let uv: UvRect = atlas.uv(Rect::new(32.0, 16.0, 64.0, 32.0));

    assert_eq!(
        (uv.get_u0(), uv.get_v0()),
        (0.25, 0.25),
        "the region's origin, divided by the image's intrinsic size — 32/128 and 16/64"
    );
    assert_eq!(
        (uv.get_u1(), uv.get_v1()),
        (0.75, 0.75),
        "and the far corner likewise, so a shader samples the region rather than the whole sheet"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "needs a stand-in HtmlImageElement to report an intrinsic size, which only \
              exists under wasm"
)]
fn an_atlas_whose_image_has_not_loaded_yields_an_all_zero_uv() {
    let atlas: SpriteAtlas = SpriteAtlas::create(sized_image(0.0, 0.0));

    let uv: UvRect = atlas.uv(Rect::new(0.0, 0.0, 32.0, 32.0));

    assert_eq!(
        (uv.get_u0(), uv.get_v0(), uv.get_u1(), uv.get_v1()),
        (0.0, 0.0, 0.0, 0.0),
        "dividing by a zero intrinsic size would produce NaN, and a NaN uv silently samples \
         nothing at all with no error anywhere"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "stands in an HtmlImageElement, which only exists under wasm"
)]
fn recording_a_nine_slice_queues_one_command_carrying_its_insets() {
    let nine: NineSlice = NineSlice::from_image(image(30.0, 30.0), 10.0);
    let mut list: DrawList = DrawList::create();

    nine.record(&mut list, Vector2D::new(5.0, 6.0), 40.0, 50.0);

    assert_eq!(list.len(), 1, "one nine-slice draw is one queued command");
    let recorded: &DrawCommand = list.get_commands().first().expect("just asserted one");
    assert!(
        matches!(recorded, DrawCommand::DrawNineSlice { .. }),
        "and it is the nine-slice variant, which replays as nine draws — queueing it as a plain \
         sprite would scale the borders along with the body"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "stands in an HtmlImageElement, which only exists under wasm"
)]
fn recording_a_named_region_from_an_atlas_queues_that_region() {
    let mut atlas: SpriteAtlas = SpriteAtlas::create(image(64.0, 64.0));
    atlas.insert("hero", Rect::new(0.0, 0.0, 16.0, 32.0));
    let mut list: DrawList = DrawList::create();

    atlas.record(&mut list, "hero", Vector2D::new(1.0, 2.0), 8.0, 16.0);

    assert_eq!(list.len(), 1, "a named region that exists is queued");
    let recorded: &DrawCommand = list.get_commands().first().expect("just asserted one");
    match recorded {
        DrawCommand::DrawAtlasRegion {
            source,
            dest_width,
            dest_height,
            ..
        } => {
            assert_eq!(
                source.get_width(),
                16.0,
                "the source rect is the named region"
            );
            assert_eq!(*dest_width, 8.0, "and the destination size is the caller's");
            assert_eq!(*dest_height, 16.0, "in both axes");
        }
        other => panic!("expected DrawAtlasRegion, got {other:?}"),
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "stands in an HtmlImageElement, which only exists under wasm"
)]
fn recording_a_name_the_atlas_does_not_have_queues_nothing() {
    let mut atlas: SpriteAtlas = SpriteAtlas::create(image(64.0, 64.0));
    atlas.insert("hero", Rect::new(0.0, 0.0, 16.0, 32.0));
    let mut list: DrawList = DrawList::create();

    atlas.record(&mut list, "villain", Vector2D::new(1.0, 2.0), 8.0, 16.0);

    assert_eq!(
        list.len(),
        0,
        "a missing name has to be skipped silently rather than queued with a zero source rect, \
         which would draw a degenerate quad and still count as a command"
    );
}
