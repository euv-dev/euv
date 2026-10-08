use super::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn a_fresh_handle_hands_back_the_configuration_it_was_built_with() {
    let handle: EngineHandle = Engine::new_handle(EngineConfig::default());

    let config: &EngineConfig = handle.get_config();
    let render: RenderConfig = config.get_render().clone();
    assert_eq!(
        render.get_width(),
        800.0,
        "the default config names a concrete viewport, so the handle is usable without setup"
    );
    assert_eq!(render.get_height(), 600.0, "and a concrete height");
    assert!(
        !render.get_canvas_selector().is_empty(),
        "the default config names a canvas, or the engine has nothing to query"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(not(target_arch = "wasm32"), ignore = "needs a wasm realm")]
fn a_supersampled_canvas_lookup_yields_nothing_on_a_host_with_no_document() {
    let found: Option<SsaaCanvas> =
        SsaaCanvas::from_selector_with_scale("#euv-test-canvas", 64.0, 64.0, 2.0);

    assert!(
        found.is_none(),
        "there is no document on this host, so no canvas resolves and no context is acquired"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(not(target_arch = "wasm32"), ignore = "needs a wasm realm")]
fn a_default_scale_factor_still_yields_nothing_rather_than_a_broken_canvas() {
    let found: Option<SsaaCanvas> = SsaaCanvas::from_selector("#euv-test-canvas", 64.0, 64.0);

    assert!(
        found.is_none(),
        "the default-scale overload takes the same path and must fail the same way"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(not(target_arch = "wasm32"), ignore = "needs a wasm realm")]
fn registering_input_on_a_handle_with_no_document_yields_nothing() {
    let mut handle: EngineHandle = Engine::new_handle(EngineConfig::default());

    let cell: Option<InputStateCell> = handle.register_input();

    assert!(
        cell.is_none(),
        "with no document there is no element to attach listeners to, so nothing is registered"
    );
}
