use super::*;

fn reflective(target: &Object, key: &str, value: &JsValue) {
    let outcome: Result<bool, JsValue> =
        Reflect::set(target.as_ref(), &JsValue::from_str(key), value);
    assert!(
        outcome.expect("writing a plain data property must not throw"),
        "the stand-in has to accept the property the production code reads"
    );
}

fn decoded_buffer(channels: u32, duration: f64) -> AudioBuffer {
    let holder: Object = Object::new();
    reflective(
        &holder,
        "numberOfChannels",
        &JsValue::from_f64(f64::from(channels)),
    );
    reflective(&holder, "duration", &JsValue::from_f64(duration));
    let buffer: AudioBuffer = holder.unchecked_into();
    buffer
}

fn browser_context(sample_rate: f64, current_time: f64) -> AudioContext {
    let holder: Object = Object::new();
    reflective(&holder, "sampleRate", &JsValue::from_f64(sample_rate));
    reflective(&holder, "currentTime", &JsValue::from_f64(current_time));
    let context: AudioContext = holder.unchecked_into();
    context
}

fn silent_param() -> Object {
    Object::new()
}

fn recording_param(sink: &js_sys::Array) -> Object {
    let param: Object = Object::new();
    reflective(&param, "__sink", sink.as_ref());
    let arm: js_sys::Function = js_sys::Function::new_with_args(
        "target, key, value",
        "Object.defineProperty(target, 'value', { set(v) { target.__sink.push(v); } });",
    );
    let armed: JsValue = arm
        .call1(&JsValue::NULL, &param)
        .expect("arming the trap must not throw");
    let _: JsValue = armed;
    param
}

fn gain_node(param: &Object) -> GainNode {
    let holder: Object = Object::new();
    reflective(&holder, "gain", param.as_ref());
    let node: GainNode = holder.unchecked_into();
    node
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "stands in a decoded WebAudio buffer as a plain object"
)]
fn a_clip_reports_the_channel_count_of_the_buffer_it_was_built_from() {
    let stereo: AudioClip = AudioClip::create(decoded_buffer(2, 1.5), String::from("stereo"));
    assert_eq!(
        stereo.channel_count(),
        2,
        "the clip must read numberOfChannels off the buffer rather than assume mono"
    );
    let mono: AudioClip = AudioClip::create(decoded_buffer(1, 0.25), String::from("mono"));
    assert_eq!(
        mono.channel_count(),
        1,
        "a one channel buffer must not be reported as the stereo default"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "stands in a decoded WebAudio buffer as a plain object"
)]
fn a_clip_reports_the_duration_of_the_buffer_it_was_built_from() {
    let clip: AudioClip = AudioClip::create(decoded_buffer(2, 1.5), String::from("beep"));
    assert_eq!(
        clip.duration(),
        1.5,
        "duration is a property of the decoded buffer, not a constant of the clip"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "stands in the WebAudio context and gain graph as plain objects"
)]
fn a_context_reads_the_sample_rate_off_the_browser_supplied_context() {
    let context: GameAudioContext = GameAudioContext::new(
        browser_context(48000.0, 0.0),
        gain_node(&silent_param()),
        1.0,
    );
    assert_eq!(
        context.sample_rate(),
        48000.0,
        "the sample rate is a property of the live AudioContext, not a hardcoded default"
    );
    let other: GameAudioContext = GameAudioContext::new(
        browser_context(22050.0, 0.0),
        gain_node(&silent_param()),
        1.0,
    );
    assert_eq!(
        other.sample_rate(),
        22050.0,
        "a second context at a different rate must report its own rate"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "stands in the WebAudio context and gain graph as plain objects"
)]
fn a_context_reports_the_playback_time_off_the_browser_supplied_context() {
    let context: GameAudioContext = GameAudioContext::new(
        browser_context(48000.0, 12.5),
        gain_node(&silent_param()),
        1.0,
    );
    assert_eq!(
        context.current_time(),
        12.5,
        "currentTime is owned by the browser context and must be read, never cached"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "stands in the WebAudio gain node as a plain object and records every write"
)]
fn master_volume_is_clamped_into_the_unit_range_before_it_reaches_the_gain_node() {
    let sink: js_sys::Array = js_sys::Array::new();
    let context: GameAudioContext = GameAudioContext::new(
        browser_context(48000.0, 0.0),
        gain_node(&recording_param(&sink)),
        1.0,
    );

    context.apply_master_volume(2.0);
    context.apply_master_volume(-1.0);
    context.apply_master_volume(0.25);

    let written: u32 = sink.length();
    let observed: Vec<f64> = (0..written)
        .map(|index: u32| sink.get(index).as_f64().unwrap_or(-1.0))
        .collect();
    assert_eq!(
        observed,
        vec![1.0, 0.0, 0.25],
        "an out of range master volume must be clamped into 0..=1 before it reaches the gain node"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "stands in the WebAudio context as a plain object and records the call"
)]
fn suspending_a_context_is_routed_through_to_the_browser_context() {
    let holder: Object = Object::new();
    reflective(&holder, "sampleRate", &JsValue::from_f64(48000.0));
    reflective(&holder, "__suspended", &JsValue::FALSE);
    let suspend: js_sys::Function =
        js_sys::Function::new_no_args("this.__suspended = true; return this;");
    reflective(&holder, "suspend", suspend.as_ref());
    let browser: AudioContext = holder.clone().unchecked_into();
    let context: GameAudioContext = GameAudioContext::new(browser, gain_node(&silent_param()), 1.0);

    context.suspend();

    let observed: JsValue = Reflect::get(&holder, &JsValue::from_str("__suspended"))
        .expect("the recorded flag must be readable");
    assert!(
        observed.is_truthy(),
        "suspend must call through to the AudioContext rather than only flipping local state"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "asserts the documented fallback for a host that exposes no WebAudio"
)]
fn a_host_without_web_audio_yields_no_context_instead_of_panicking() {
    let global: Object = js_sys::global().unchecked_into();
    let constructor: Result<JsValue, JsValue> =
        Reflect::get(&global, &JsValue::from_str("AudioContext"));
    let exposes_web_audio: bool = constructor.is_ok_and(|found: JsValue| found.is_function());

    let created: Option<GameAudioContext> = GameAudioContext::create();

    if exposes_web_audio {
        assert!(
            created.is_some(),
            "a host that does expose an AudioContext must yield a usable GameAudioContext"
        );
        return;
    }
    assert!(
        created.is_none(),
        "without an AudioContext constructor there is nothing to build, so create must report None rather than panic"
    );
}
