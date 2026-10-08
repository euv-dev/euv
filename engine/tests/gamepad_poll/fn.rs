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

fn window_with_pads(pads: &Array) -> Window {
    let list: JsValue = pads.clone().into();
    let navigator: Object = Object::new();
    reflective(navigator.as_ref(), "__pads", &list);
    let method: js_sys::Function = js_sys::Function::new_no_args("return this.__pads;");
    reflective(navigator.as_ref(), "getGamepads", method.as_ref());
    let window: Object = Object::new();
    reflective(window.as_ref(), "navigator", navigator.as_ref());
    window.unchecked_into()
}

fn pad() -> Object {
    let axes: Array = Array::new();
    axes.push(&JsValue::from_f64(0.0));
    let button: Object = Object::new();
    reflective(button.as_ref(), "pressed", &JsValue::from_bool(true));
    reflective(button.as_ref(), "value", &JsValue::from_f64(1.0));
    let buttons: Array = Array::new();
    buttons.push(button.as_ref());
    let pad: Object = Object::new();
    reflective(pad.as_ref(), "index", &JsValue::from_f64(3.0));
    reflective(pad.as_ref(), "id", &JsValue::from_str("test pad"));
    reflective(pad.as_ref(), "axes", axes.as_ref());
    reflective(pad.as_ref(), "buttons", buttons.as_ref());
    pad
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "stands in a Window/Navigator chain as plain objects"
)]
fn a_poll_with_no_pads_attached_leaves_the_state_empty_rather_than_failing() {
    let pads: Array = Array::new();
    let window: Window = window_with_pads(&pads);
    let mut manager: GamepadManager = GamepadManager::default();

    manager.poll(&window);

    assert!(
        manager.get_states().is_empty(),
        "with no pad reporting in, the manager stays empty and the poll is a no-op"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "stands in a Window/Navigator chain as plain objects"
)]
fn an_empty_slot_in_the_pad_list_is_skipped_rather_than_unwrapped_into_a_pad() {
    let pads: Array = Array::new();
    pads.push(&JsValue::NULL);
    let window: Window = window_with_pads(&pads);
    let mut manager: GamepadManager = GamepadManager::default();

    manager.poll(&window);

    assert!(
        manager.get_states().is_empty(),
        "the browser leaves a gap for a pad that is not connected, and null is not a pad"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "stands in a Window/Navigator chain as plain objects"
)]
fn a_live_pad_is_recorded_in_the_connected_set() {
    let pads: Array = Array::new();
    pads.push(pad().as_ref());
    let window: Window = window_with_pads(&pads);
    let mut manager: GamepadManager = GamepadManager::default();

    manager.poll(&window);

    assert_eq!(
        manager.get_states().len(),
        1,
        "a pad the browser reports is observed, and the caller can find it by index"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "stands in a Window/Navigator chain as plain objects"
)]
fn a_pad_the_browser_stops_listing_is_dropped_from_the_connected_set() {
    let pads: Array = Array::new();
    pads.push(pad().as_ref());
    let window: Window = window_with_pads(&pads);
    let mut manager: GamepadManager = GamepadManager::default();
    manager.poll(&window);
    assert_eq!(manager.get_states().len(), 1, "it started connected");

    let empty: Array = Array::new();
    let window: Window = window_with_pads(&empty);
    manager.poll(&window);

    assert!(
        !manager.is_connected(3),
        "a pad the browser stops listing is marked disconnected, or a caller would read its          held buttons forever"
    );
    assert!(
        manager.get_states().len() == 1,
        "but its state entry is kept, so a later reconnect can be recognised as the same pad"
    );
}
