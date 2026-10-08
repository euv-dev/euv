use super::*;

fn every_variant() -> Vec<(&'static str, WebGpuInitError)> {
    let payload: JsValue = JsValue::from_str("boom");
    vec![
        (
            "navigator_lookup",
            WebGpuInitError::NavigatorLookup(payload.clone()),
        ),
        (
            "navigator_gpu_missing",
            WebGpuInitError::NavigatorGpuMissing,
        ),
        (
            "request_adapter_lookup",
            WebGpuInitError::RequestAdapterLookup(payload.clone()),
        ),
        (
            "request_adapter_call",
            WebGpuInitError::RequestAdapterCall(payload.clone()),
        ),
        (
            "adapter_promise",
            WebGpuInitError::AdapterPromise(payload.clone()),
        ),
        ("adapter_unavailable", WebGpuInitError::AdapterUnavailable),
        (
            "request_device_lookup",
            WebGpuInitError::RequestDeviceLookup(payload.clone()),
        ),
        (
            "request_device_call",
            WebGpuInitError::RequestDeviceCall(payload.clone()),
        ),
        (
            "device_promise",
            WebGpuInitError::DevicePromise(payload.clone()),
        ),
        ("device_unavailable", WebGpuInitError::DeviceUnavailable),
        (
            "canvas_not_found",
            WebGpuInitError::CanvasNotFound(String::from("no canvas")),
        ),
        (
            "canvas_query",
            WebGpuInitError::CanvasQuery(payload.clone()),
        ),
        (
            "canvas_context_unavailable",
            WebGpuInitError::CanvasContextUnavailable,
        ),
        (
            "preferred_format_lookup",
            WebGpuInitError::PreferredFormatLookup(payload.clone()),
        ),
        (
            "preferred_format_call",
            WebGpuInitError::PreferredFormatCall(payload.clone()),
        ),
        (
            "preferred_format_type",
            WebGpuInitError::PreferredFormatType(payload.clone()),
        ),
        (
            "configure_lookup",
            WebGpuInitError::ConfigureLookup(payload.clone()),
        ),
        ("queue_lookup", WebGpuInitError::QueueLookup(payload)),
    ]
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "builds JsValue payloads, which only exist under wasm"
)]
fn every_variant_reports_a_distinct_stable_code() {
    let cases: Vec<(&'static str, WebGpuInitError)> = every_variant();
    let mut seen: Vec<&'static str> = Vec::new();
    for (name, error) in &cases {
        let code: &str = error.code();
        assert!(
            !code.is_empty(),
            "{name} must report a code, since logs and telemetry key off it"
        );
        assert!(
            !seen.contains(&code),
            "{name} reuses the code {code}, so two failures cannot be told apart"
        );
        seen.push(code);
    }
    assert_eq!(seen.len(), cases.len(), "one code per variant");
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "formats a JsValue, which only exists under wasm"
)]
fn a_code_names_its_own_variant_so_the_string_is_readable_in_a_log() {
    let cases: Vec<(&'static str, WebGpuInitError)> = every_variant();
    for (name, error) in &cases {
        let code: &str = error.code();
        assert!(
            code.starts_with("WEBGPU_"),
            "{name} reports {code}, which is not in the WEBGPU_ namespace"
        );
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "builds JsValue payloads, which only exist under wasm"
)]
fn the_variants_that_carry_a_js_value_hand_it_back_and_the_rest_return_none() {
    let carrying: Vec<WebGpuInitError> = vec![
        WebGpuInitError::NavigatorLookup(JsValue::from_str("a")),
        WebGpuInitError::RequestAdapterLookup(JsValue::from_str("a")),
        WebGpuInitError::RequestAdapterCall(JsValue::from_str("a")),
        WebGpuInitError::AdapterPromise(JsValue::from_str("a")),
        WebGpuInitError::RequestDeviceLookup(JsValue::from_str("a")),
        WebGpuInitError::RequestDeviceCall(JsValue::from_str("a")),
        WebGpuInitError::DevicePromise(JsValue::from_str("a")),
        WebGpuInitError::CanvasQuery(JsValue::from_str("a")),
        WebGpuInitError::PreferredFormatLookup(JsValue::from_str("a")),
        WebGpuInitError::PreferredFormatCall(JsValue::from_str("a")),
        WebGpuInitError::PreferredFormatType(JsValue::from_str("a")),
        WebGpuInitError::ConfigureLookup(JsValue::from_str("a")),
        WebGpuInitError::QueueLookup(JsValue::from_str("a")),
    ];
    for error in &carrying {
        let held: Option<&JsValue> = error.js_error();
        assert!(
            held.is_some(),
            "{:?} carries a JsValue and must hand it back for diagnostics",
            error.code()
        );
    }

    let bare: Vec<WebGpuInitError> = vec![
        WebGpuInitError::NavigatorGpuMissing,
        WebGpuInitError::AdapterUnavailable,
        WebGpuInitError::DeviceUnavailable,
        WebGpuInitError::CanvasContextUnavailable,
    ];
    for error in &bare {
        assert!(
            error.js_error().is_none(),
            "{:?} carries no JsValue, so it must not invent one",
            error.code()
        );
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "formats a JsValue, which only exists under wasm"
)]
fn the_display_form_leads_with_the_code_and_keeps_the_driver_text() {
    let error: WebGpuInitError =
        WebGpuInitError::RequestAdapterCall(JsValue::from_str("adapter exploded"));
    let rendered: String = error.to_string();
    assert!(
        rendered.contains(error.code()),
        "the formatted message is for end-user diagnostics, so it has to name the code: {rendered}"
    );
    assert!(
        rendered.contains("adapter exploded"),
        "the driver's own wording is the only actionable part: {rendered}"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "reads window().navigator(), which only exists under wasm"
)]
fn webgpu_is_reported_unavailable_on_a_host_with_no_window() {
    let observed: bool = WebGpuRenderer::is_available();

    assert!(
        !observed,
        "there is no navigator on this host, so the capability probe must answer false rather \
         than panic on the missing global"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "only exercises the cache key enum, which carries no JsValue"
)]
fn every_gpu_receiver_class_is_a_distinct_cache_key() {
    let classes: Vec<GpuReceiverClass> = vec![
        GpuReceiverClass::Device,
        GpuReceiverClass::Queue,
        GpuReceiverClass::Context,
        GpuReceiverClass::Texture,
        GpuReceiverClass::CommandEncoder,
    ];
    let mut distinct: Vec<GpuReceiverClass> = Vec::new();
    for class in classes {
        assert!(
            !distinct.contains(&class),
            "{class:?} appears twice, so the method cache would collide between two receivers"
        );
        distinct.push(class);
    }
    assert_eq!(
        distinct.len(),
        5,
        "five distinct receivers back the WebGPU surface"
    );
}
