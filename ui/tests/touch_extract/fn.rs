use super::*;

fn reflective(target: &JsValue, key: &str, value: &JsValue) {
    let installer: Function = Function::new_with_args(
        "target, key, value",
        "Object.defineProperty(target, key, { value: value, writable: true, configurable: true });",
    );
    let outcome: Result<JsValue, JsValue> = installer.call3(
        &JsValue::NULL,
        target.as_ref(),
        &JsValue::from_str(key),
        value,
    );
    assert!(
        outcome.is_ok(),
        "the stand-in has to accept the property the production code reads"
    );
}

fn touch_object(identifier: i32, client_x: i32, client_y: i32) -> Object {
    let touch: Object = Object::new();
    reflective(
        touch.as_ref(),
        "identifier",
        &JsValue::from_f64(f64::from(identifier)),
    );
    reflective(
        touch.as_ref(),
        "clientX",
        &JsValue::from_f64(f64::from(client_x)),
    );
    reflective(
        touch.as_ref(),
        "clientY",
        &JsValue::from_f64(f64::from(client_y)),
    );
    reflective(
        touch.as_ref(),
        "screenX",
        &JsValue::from_f64(f64::from(client_x + 100)),
    );
    reflective(
        touch.as_ref(),
        "screenY",
        &JsValue::from_f64(f64::from(client_y + 200)),
    );
    reflective(
        touch.as_ref(),
        "pageX",
        &JsValue::from_f64(f64::from(client_x)),
    );
    reflective(
        touch.as_ref(),
        "pageY",
        &JsValue::from_f64(f64::from(client_y)),
    );
    touch
}

fn target_at(left: f64, top: f64) -> Object {
    let rect: Object = Object::new();
    reflective(rect.as_ref(), "left", &JsValue::from_f64(left));
    reflective(rect.as_ref(), "top", &JsValue::from_f64(top));
    let element: Object = Object::new();
    reflective(element.as_ref(), "__rect", rect.as_ref());
    let bounds: Function = Function::new_with_args("ignored", "return this.__rect;");
    reflective(element.as_ref(), "getBoundingClientRect", bounds.as_ref());
    element
}

fn touch_event(active: &[Object], changed: &[Object], left: f64, top: f64) -> Event {
    let event: Event = Event::new("touchmove").expect("a touch event");
    let active_list: Array = Array::new();
    for touch in active {
        active_list.push(touch);
    }
    let changed_list: Array = Array::new();
    for touch in changed {
        changed_list.push(touch);
    }
    let element: Object = target_at(left, top);
    reflective(event.as_ref(), "touches", active_list.as_ref());
    reflective(event.as_ref(), "changedTouches", changed_list.as_ref());
    reflective(event.as_ref(), "target", element.as_ref());
    event
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "stands in a TouchList as a plain array on a real Event"
)]
fn active_touches_are_extracted_with_offsets_relative_to_the_target_rect() {
    let first: Object = touch_object(7, 15, 25);
    let second: Object = touch_object(8, 30, 45);
    let event: Event = touch_event(&[first, second], &[], 10.0, 20.0);

    let observed: Vec<NativeTouchPoint> = NativeTouchPoint::extract_all(&event);

    assert_eq!(
        observed.len(),
        2,
        "both active touch points must be reported"
    );
    let head: &NativeTouchPoint = &observed[0];
    assert_eq!(
        head.get_identifier(),
        7,
        "the identifier comes from the touch itself"
    );
    assert_eq!(
        head.get_client_x(),
        15,
        "clientX must be carried through untouched"
    );
    assert_eq!(
        head.get_client_y(),
        25,
        "clientY must be carried through untouched"
    );
    assert_eq!(
        head.get_screen_x(),
        115,
        "screenX is a distinct coordinate space and must not alias clientX"
    );
    assert_eq!(
        head.get_screen_y(),
        225,
        "screenY is a distinct coordinate space and must not alias clientY"
    );
    assert_eq!(
        head.get_page_x(),
        15,
        "pageX must be carried through from the touch"
    );
    assert_eq!(
        head.get_page_y(),
        25,
        "pageY must be carried through from the touch"
    );
    assert_eq!(
        head.get_offset_x(),
        5,
        "offsetX is clientX minus the target rect left edge"
    );
    assert_eq!(
        head.get_offset_y(),
        5,
        "offsetY is clientY minus the target rect top edge"
    );
    let tail: &NativeTouchPoint = &observed[1];
    assert_eq!(
        tail.get_offset_x(),
        20,
        "each point gets its own offset against the same rect"
    );
    assert_eq!(
        tail.get_offset_y(),
        25,
        "each point gets its own offset against the same rect"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "stands in a TouchList as a plain array on a real Event"
)]
fn an_event_without_active_touches_reports_no_points() {
    let event: Event = touch_event(&[], &[], 0.0, 0.0);
    let observed: Vec<NativeTouchPoint> = NativeTouchPoint::extract_all(&event);
    assert!(
        observed.is_empty(),
        "an empty active list must yield an empty vector rather than a placeholder point"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "stands in a TouchList as a plain array on a real Event"
)]
fn changed_touches_are_read_from_the_changed_list_and_not_the_active_one() {
    let active: Object = touch_object(1, 5, 5);
    let lifted: Object = touch_object(2, 90, 95);
    let moved: Object = touch_object(3, 40, 45);
    let event: Event = touch_event(&[active], &[lifted, moved], 10.0, 20.0);

    let observed: Vec<NativeTouchPoint> = NativeTouchPoint::extract_changed(&event);

    assert_eq!(
        observed.len(),
        2,
        "changedTouches must be reported even when the touched points are no longer active"
    );
    let identifiers: Vec<i32> = observed
        .iter()
        .map(NativeTouchPoint::get_identifier)
        .collect();
    assert_eq!(
        identifiers,
        vec![2, 3],
        "a lifted finger appears only in changedTouches, so reading touches instead would lose it"
    );
    assert_eq!(
        observed[0].get_offset_x(),
        80,
        "changed points get the same rect-relative offset as active ones"
    );
    assert_eq!(
        observed[1].get_offset_x(),
        30,
        "changed points get the same rect-relative offset as active ones"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "stands in a TouchList as a plain array on a real Event"
)]
fn offsets_are_computed_against_the_target_rect_rather_than_the_viewport() {
    let touch: Object = touch_object(4, 100, 100);
    let event: Event = touch_event(&[touch], &[], 64.0, 32.0);

    let observed: Vec<NativeTouchPoint> = NativeTouchPoint::extract_all(&event);

    let only: &NativeTouchPoint = &observed[0];
    assert_eq!(
        only.get_offset_x(),
        36,
        "offsetX must subtract the rect left, not the viewport origin"
    );
    assert_eq!(
        only.get_offset_y(),
        68,
        "offsetY must subtract the rect top, not the viewport origin"
    );
    assert_eq!(
        only.get_client_x(),
        100,
        "the raw viewport coordinate must stay untouched alongside the offset"
    );
}
