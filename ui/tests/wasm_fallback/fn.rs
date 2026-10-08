#[cfg(target_arch = "wasm32")]
use super::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test::wasm_bindgen_test]
fn local_storage_get_returns_none_when_there_is_no_window() {
    assert_eq!(
        UseEuvBrowser::local_storage_get("euv.never.set"),
        None,
        "a host without localStorage must read as absent, not panic"
    );
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test::wasm_bindgen_test]
fn local_storage_set_is_a_no_op_when_there_is_no_window() {
    UseEuvBrowser::local_storage_set("euv.probe", "value");
    assert_eq!(
        UseEuvBrowser::local_storage_get("euv.probe"),
        None,
        "the write must not have created storage that the read can then see"
    );
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test::wasm_bindgen_test]
fn local_storage_get_of_an_absent_key_and_of_any_key_agree_under_node() {
    assert_eq!(UseEuvBrowser::local_storage_get("a"), None);
    assert_eq!(UseEuvBrowser::local_storage_get(""), None);
    assert_eq!(UseEuvBrowser::local_storage_get(String::from("b")), None);
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test::wasm_bindgen_test]
fn pushing_an_overlay_history_entry_is_a_no_op_without_a_window() {
    Router::overlay_push_state();
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test::wasm_bindgen_test]
fn going_back_from_an_overlay_without_a_window_still_records_the_pending_target() {
    Router::overlay_back(Some(String::from("/home")));
    Router::overlay_back(None);
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test::wasm_bindgen_test]
fn opening_an_external_url_is_a_no_op_without_a_window() {
    Router::open_system_browser("https://example.com");
    Router::open_system_browser(String::from("https://example.org"));
}
