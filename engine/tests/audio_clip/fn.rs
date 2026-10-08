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

fn decoded_buffer(channels: u32, duration: f64) -> AudioBuffer {
    let element: Object = Object::new();
    reflective(
        element.as_ref(),
        "numberOfChannels",
        &JsValue::from_f64(f64::from(channels)),
    );
    reflective(element.as_ref(), "duration", &JsValue::from_f64(duration));
    element.unchecked_into()
}

fn clip() -> AudioClip {
    AudioClip::create(decoded_buffer(2, 1.5), String::from("beep"))
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "stands in a decoded WebAudio buffer as a plain object"
)]
fn a_fresh_clip_starts_stopped_at_the_documented_defaults() {
    let subject: AudioClip = clip();

    assert_eq!(
        subject.get_state(),
        AudioPlayState::Stopped,
        "a clip is stopped until play() hands it to a source"
    );
    assert!(
        !subject.get_looping(),
        "the default loop hint is off, or every clip would repeat forever"
    );
    assert_eq!(
        subject.get_volume(),
        1.0,
        "a clip is created at full volume, not at zero"
    );
    assert_eq!(
        subject.get_playback_rate(),
        1.0,
        "and at normal speed, so playback matches the authored timing"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "stands in a decoded WebAudio buffer as a plain object"
)]
fn the_looping_flag_is_remembered_even_with_no_source_node_attached() {
    let mut subject: AudioClip = clip();

    subject.update_looping(true);
    assert!(subject.get_looping(), "the flag is set on the clip itself");

    subject.update_looping(false);
    assert!(
        !subject.get_looping(),
        "and unset again, so it is not write-once"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "stands in a decoded WebAudio buffer as a plain object"
)]
fn the_volume_is_clamped_into_the_unit_range() {
    let mut subject: AudioClip = clip();

    subject.update_volume(2.0);
    assert_eq!(
        subject.get_volume(),
        1.0,
        "an amplifier past unity would distort rather than get louder"
    );

    subject.update_volume(-1.0);
    assert_eq!(
        subject.get_volume(),
        0.0,
        "a negative gain inverts the phase rather than silencing"
    );

    subject.update_volume(0.25);
    assert_eq!(
        subject.get_volume(),
        0.25,
        "a value inside the range passes through"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "stands in a decoded WebAudio buffer as a plain object"
)]
fn the_playback_rate_is_stored_verbatim_rather_than_clamped() {
    let mut subject: AudioClip = clip();

    subject.update_playback_rate(2.5);
    assert_eq!(
        subject.get_playback_rate(),
        2.5,
        "slow motion and fast forward are both legitimate, so this is not clamped"
    );

    subject.update_playback_rate(0.5);
    assert_eq!(
        subject.get_playback_rate(),
        0.5,
        "and half speed is a real use, not an out of range value"
    );
}
