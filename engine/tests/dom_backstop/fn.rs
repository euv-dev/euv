use super::*;

fn config() -> EngineConfig {
    EngineConfig::default()
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "the point is what happens on a host with no document, which only exists under \
              wasm"
)]
fn creating_an_image_element_yields_nothing_where_there_is_no_document() {
    let created: Option<HtmlImageElement> =
        AssetLoader::create_image_element("https://example.invalid/atlas.png");

    assert!(
        created.is_none(),
        "there is no document to create an <img> in, and the documented failure signal is \
         None - handing back an unchecked cast instead would produce an element the browser \
         rejects on first use"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "the point is what happens on a host with no document, which only exists under \
              wasm"
)]
fn a_load_that_cannot_make_an_element_leaves_the_loader_completely_untouched() {
    let mut subject: AssetLoader = AssetLoader::default();

    subject.load_image(String::from("https://example.invalid/atlas.png"));

    assert_eq!(
        subject.pending_count(),
        0,
        "the in-flight counter is the one number a caller polls to know when the loader is \
         idle; a load that could not even start must not increment it, or every wait loop built \
         on it hangs forever"
    );
    assert!(
        subject
            .get_image("https://example.invalid/atlas.png")
            .is_none(),
        "no half-built entry may be left behind for a later lookup to find as though the image \
         had arrived"
    );
    assert!(
        subject.is_all_loaded(),
        "which is what makes a loader with nothing in it report done, rather than a caller \
         waiting on a load that was never started"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    ignore = "the point is what happens on a host with no window, which only exists under wasm"
)]
fn asking_for_a_webgl_backend_with_no_window_names_the_missing_canvas() {
    let wanted: String = config().get_render().get_canvas_selector().clone();

    let outcome: Result<WebGl2Backend, WebGl2InitError> =
        Engine::webgl_renderer(&config().get_render());

    match outcome {
        Err(WebGl2InitError::CanvasNotFound(selector)) => assert_eq!(
            selector, wanted,
            "the error has to carry back the selector that was tried, otherwise a caller with \
             several canvases in the page cannot tell which one was missing"
        ),
        Err(other) => panic!("expected the missing-canvas error, got {other:?}"),
        Ok(_) => panic!("a host with no window cannot hand back a WebGL 2 context"),
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test]
async fn asking_for_a_webgpu_backend_with_no_window_reports_a_missing_navigator() {
    let outcome: Result<WebGpuRenderer, WebGpuInitError> =
        Engine::webgpu_renderer(&config().get_render()).await;

    match outcome {
        Ok(_) => panic!("a host with no navigator cannot hand back a GPU adapter"),
        Err(error) => assert_eq!(
            error.code(),
            "WEBGPU_NAVIGATOR_GPU_MISSING",
            "the navigator exists on this host but exposes no `gpu`, so the code a caller \
             branches on is the missing-GPU one - and it is distinct from every other \
             WebGPUInitError variant, which is what makes it usable as a branch at all"
        ),
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test]
async fn initialising_webgpu_on_a_handle_leaves_every_backend_slot_empty() {
    let mut handle: EngineHandle = Engine::new_handle(config());

    let outcome: Result<WebGpuRenderer, WebGpuInitError> = handle.init_webgpu().await;

    assert!(
        outcome.is_err(),
        "there is no GPU on this host, so initialization has to fail rather than half-succeed"
    );
    assert!(
        handle.get_webgpu_renderer().is_none(),
        "the webgpu slot stays empty: a renderer that failed to initialize must not be stored, \
         or the next frame reads a half-built renderer"
    );
    assert!(
        handle.get_canvas_renderer().is_none(),
        "and this entry point owns the webgpu slot alone, so it must not have produced a canvas \
         renderer as a side effect"
    );
    assert!(handle.get_webgl_renderer().is_none(), "nor a webgl one");
}
